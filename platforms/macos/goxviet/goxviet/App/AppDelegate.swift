//
//  AppDelegate.swift
//  GoxViet
//
//  Enhanced with toggle functionality and state management
//

import Cocoa
import SwiftUI

class AppDelegate: NSObject, NSApplicationDelegate {
    
    var statusItem: NSStatusItem!
    
    // Termination flag to prevent new operations during shutdown
    private var isTerminating = false
    
    // Timer for auto-polling accessibility permission
    private var accessibilityPollTimer: Timer?

    // Flag to track if permission was granted while modal was showing
    private var permissionGrantedWhileModalActive = false
    private var isModalAlertActive = false

    // Flag: app was launched right after an auto-update
    private var isPostUpdateLaunch: Bool = false
    private let notificationCenter = NotificationCenter.default

    private enum ObserverKey {
        static let updateState = "AppDelegate.updateStateObserver"
        static let toggleVietnamese = "AppDelegate.toggleObserver"
        static let shortcutChanged = "AppDelegate.shortcutObserver"
        static let appActivation = "AppDelegate.activationObserver"
        static let inputMethod = "AppDelegate.inputMethodObserver"
        static let settingsClose = "AppDelegate.settingsCloseObserver"
        static let settingsCleanup = "AppDelegate.settingsCleanupObserver"
        static let accessibilityRevoked = "AppDelegate.accessibilityRevokedObserver"
    }
    
    var isEnabled: Bool {
        return SettingsManager.shared.isEnabled
    }

    private func applyActivationPolicyFromPreference() {
        // Use SettingsManager instead of direct UserDefaults access
        let hide = SettingsManager.shared.hideFromDock
        let policy: NSApplication.ActivationPolicy = hide ? .accessory : .regular

        // Delegate to coordinator to coalesce and apply outside layout passes
        ActivationPolicyCoordinator.shared.request(policy)
    }
    
    func applicationDidFinishLaunching(_ aNotification: Notification) {
        // Logging is disabled by default - can be enabled in Advanced Settings
        // This reduces memory footprint and improves performance
        #if DEBUG
        if Log.isEnabled {
            Log.info("GoxViet starting in DEBUG mode (logging enabled)")
        }
        #endif

        // Start checking for /tmp/goxviet_debug.log trigger file (every 5s)
        Log.startDebugTriggerCheck()
        
        // Disable automatic window restoration to avoid className errors
        UserDefaults.standard.register(defaults: ["NSQuitAlwaysKeepsWindows": false])

        // Initialize settings — must happen before InputManager is created
        SettingsManager.shared.initialize()

        // Initialize service singletons that depend on MainActor
        SoundFeedbackService.shared = SoundFeedbackService()
        PerAppInjectionManager.shared = PerAppInjectionManager()
        DebugLogger.shared = DebugLogger()

        // Create InputManager singleton on main actor (init has @MainActor dependencies)
        InputManager.shared = InputManager()

        // Apply Dock visibility from user preference
        applyActivationPolicyFromPreference()
        
        // Create Status Bar Item first (before permission check)
        statusItem = NSStatusBar.system.statusItem(withLength: NSStatusItem.variableLength)

        // Install hidden SwiftUI host to capture openSettingsAction
        SettingsActionBridge.shared.installIfNeeded()
        
        updateStatusIcon()
        
        setupMenu()
        setupObservers()
        
        // Phase 2: Initialize Smart Mode Menu Bar Item (separate from main status item)
        // TODO: Uncomment after verifying all dependencies are added to Xcode project
        // smartModeMenuBarItem = SmartModeMenuBarItem()
        // Log.info("Smart Mode menu bar indicator initialized")
        
        // Detect post-update launch: explicit flag OR version change since last run
        isPostUpdateLaunch = CommandLine.arguments.contains("--post-update")
        if !isPostUpdateLaunch {
            let currentVersion = Bundle.main.object(forInfoDictionaryKey: "CFBundleShortVersionString") as? String ?? ""
            let lastVersion = UserDefaults.standard.string(forKey: SettingsKey.lastKnownVersion) ?? ""
            if !lastVersion.isEmpty && lastVersion != currentVersion {
                isPostUpdateLaunch = true
                Log.info("Version changed \(lastVersion) → \(currentVersion): treating as post-update launch")
            }
            UserDefaults.standard.set(currentVersion, forKey: SettingsKey.lastKnownVersion)
        }
        if isPostUpdateLaunch {
            Log.info("Post-update launch detected")
        }

        // Check and request Accessibility Permission
        // InputManager will only start if permission is granted
        // Delay slightly on post-update launches to let macOS TCC settle
        let delay: TimeInterval = (isPostUpdateLaunch || UserDefaults.standard.bool(forKey: SettingsKey.permissionGranted)) ? 1.5 : 0.0
        DispatchQueue.main.asyncAfter(deadline: .now() + delay) { [weak self] in
            self?.checkAccessibilityPermission()
        }

        // Start background update checks
        UpdateManager.shared.start()
        
        Log.info("Application launched successfully")
    }
    
    // Settings window is now managed by macOS Settings scene
    // Accessed via Cmd+, or "Settings..." menu item
    
    // MARK: - Accessibility Permission

    func checkAccessibilityPermission(retryCount: Int = 0) {
        // Check WITHOUT showing system prompt (no duplicate dialogs)
        let accessEnabled = AXIsProcessTrusted()

        if !accessEnabled {
            // On post-update launches macOS TCC may need a moment to recognise the
            // new binary as the previously-trusted app. Also retry patiently if the
            // user previously had permission (hadPermissionBefore) — covers manual
            // revocation/re-grant and update-triggered TCC resets.
            let hadPermissionBefore = UserDefaults.standard.bool(forKey: SettingsKey.permissionGranted)
            let shouldRetryPatiently = isPostUpdateLaunch || hadPermissionBefore
            let maxRetries = shouldRetryPatiently ? 8 : 0
            let retryInterval: TimeInterval = 0.75
            if retryCount < maxRetries {
                Log.info("Accessibility not yet granted, retrying (\(retryCount + 1)/\(maxRetries))…")
                DispatchQueue.main.asyncAfter(deadline: .now() + retryInterval) { [weak self] in
                    self?.checkAccessibilityPermission(retryCount: retryCount + 1)
                }
                return
            }

            Log.warning("Accessibility permission not granted")

            // Show only our custom alert (not system prompt)
            DispatchQueue.main.async { [weak self] in
                self?.showAccessibilityAlert()
            }
        } else {
            Log.info("Accessibility permission granted")
            stopAccessibilityPollTimer()

            UserDefaults.standard.set(true, forKey: SettingsKey.permissionGranted)

            // Start InputManager only after permission is confirmed
            InputManager.shared.start()
        }
    }
    
    // MARK: - Auto-Polling Timer
    
    func startAccessibilityPollTimer() {
        // Ensure we're on main thread for Timer scheduling
        guard Thread.isMainThread else {
            DispatchQueue.main.async { [weak self] in
                self?.startAccessibilityPollTimer()
            }
            return
        }
        
        // Stop existing timer if any
        stopAccessibilityPollTimer()
        
        // Poll every 1 second to check if permission was granted
        let timer = Timer.scheduledTimer(withTimeInterval: 1.0, repeats: true) { [weak self] _ in
            let accessEnabled = AXIsProcessTrusted()
            if accessEnabled {
                Task { @MainActor [weak self] in
                    guard let self = self else { return }
                    Log.info("Accessibility permission detected via auto-polling")
                    ResourceManager.shared.unregister(timerIdentifier: "AppDelegate.accessibilityPollTimer")
                    self.accessibilityPollTimer = nil

                    // If modal is active, just set the flag - don't try to manipulate UI
                    if self.isModalAlertActive {
                        self.permissionGrantedWhileModalActive = true
                        Log.info("Permission granted while modal active - will handle after modal closes")
                    } else {
                        self.onAccessibilityGranted()
                    }
                }
            }
        }
        ResourceManager.shared.register(timer: timer, identifier: "AppDelegate.accessibilityPollTimer")
        accessibilityPollTimer = timer
        Log.info("Started accessibility permission auto-polling")
    }
    
    func stopAccessibilityPollTimer() {
        // Ensure we're on main thread for Timer invalidation
        guard Thread.isMainThread else {
            DispatchQueue.main.async { [weak self] in
                self?.stopAccessibilityPollTimer()
            }
            return
        }
        
        ResourceManager.shared.unregister(timerIdentifier: "AppDelegate.accessibilityPollTimer")
        accessibilityPollTimer = nil
    }
    
    func onAccessibilityGranted() {
        // Ensure we're on main thread
        guard Thread.isMainThread else {
            DispatchQueue.main.async { [weak self] in
                self?.onAccessibilityGranted()
            }
            return
        }
        
        stopAccessibilityPollTimer()

        Log.info("Accessibility permission granted - starting InputManager")
        UserDefaults.standard.set(true, forKey: SettingsKey.permissionGranted)
        InputManager.shared.start()
    }
    
    func showAccessibilityAlert() {
        // Reset flag
        permissionGrantedWhileModalActive = false
        
        // Start auto-polling when showing the alert
        startAccessibilityPollTimer()
        
        isModalAlertActive = true
        
        let alert = NSAlert()
        if isPostUpdateLaunch {
            alert.messageText = "🔄 Re-enable Accessibility After Update"
            alert.informativeText = """
            GoxViet was just updated. macOS requires you to re-enable Accessibility permission for the new version.

            📝 Quick re-enable (takes ~10 seconds):

            1️⃣ Click "Open System Settings" below
            2️⃣ Find "GoxViet" in the list — toggle it OFF, then back ON
            3️⃣ That's it! Permission will be auto-detected

            ⚠️ If GoxViet is not in the list, click + to add it.
            """
        } else {
            alert.messageText = "🔐 Accessibility Permission Required"
            alert.informativeText = """
            GoxViet needs Accessibility permission to capture keyboard input for Vietnamese typing.
            📝 Quick Setup (one-time only):

            1️⃣ Click "Open System Settings" below
            2️⃣ Find "GoxViet" in the list and toggle it ON
            3️⃣ That's it! Permission will be auto-detected

            💡 The permission is remembered - you won't need to do this again after rebuilding the app.

            ⚠️ If GoxViet is not in the list:
               • Click the + button to add it manually
               • Or drag GoxViet.app into the list
            """
        }
        alert.alertStyle = .warning
        alert.addButton(withTitle: "Open System Settings")
        alert.addButton(withTitle: "Quit")
        
        // Add accessory view with status indicator
        let statusLabel = NSTextField(labelWithString: "⏳ Waiting for permission... (auto-detecting)")
        statusLabel.font = NSFont.systemFont(ofSize: 11)
        statusLabel.textColor = .secondaryLabelColor
        alert.accessoryView = statusLabel
        
        let response = alert.runModal()
        
        isModalAlertActive = false
        
        // Check if permission was granted while modal was showing
        if permissionGrantedWhileModalActive {
            Log.info("Permission was granted while modal was active - starting InputManager")
            onAccessibilityGranted()
            return
        }
        
        switch response {
        case .alertFirstButtonReturn:
            // Open System Settings - Privacy & Security > Accessibility
            let prefpaneUrl = URL(string: "x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")!
            NSWorkspace.shared.open(prefpaneUrl)
            
            // Continue polling in background - will auto-detect when granted
            Log.info("Opened System Settings, waiting for user to grant permission...")
            
            // Check again after a delay in case user already granted permission
            DispatchQueue.main.asyncAfter(deadline: .now() + 2.0) { [weak self] in
                self?.checkAndShowAlertIfNeeded()
            }
            
        case .alertSecondButtonReturn:
            // Quit
            stopAccessibilityPollTimer()
            NSApplication.shared.terminate(self)
            
        default:
            stopAccessibilityPollTimer()
            break
        }
    }
    
    func checkAndShowAlertIfNeeded() {
        let accessEnabled = AXIsProcessTrusted()
        if accessEnabled {
            onAccessibilityGranted()
        } else {
            // Show alert again if permission still not granted
            showAccessibilityAlert()
        }
    }
    
    func recheckAccessibilityPermission() {
        let accessEnabled = AXIsProcessTrusted()
        
        if !accessEnabled {
            Log.warning("Accessibility permission still not granted - showing alert again")
            
            // Delay before showing alert again to give user time
            DispatchQueue.main.asyncAfter(deadline: .now() + 0.5) { [weak self] in
                self?.showAccessibilityAlert()
            }
        } else {
            onAccessibilityGranted()
        }
    }
    
    func setupMenu() {
        NSLog("[GoxViet] setupMenu() called")
        let menu = NSMenu()
        
        // Vietnamese Input Toggle - using menu item with checkmark
        let toggleItem = NSMenuItem(
            title: "Vietnamese Input",
            action: #selector(toggleVietnamese),
            keyEquivalent: ""
        )
        toggleItem.tag = 100
        toggleItem.target = self
        menu.addItem(toggleItem)
        
        menu.addItem(NSMenuItem.separator())

        // Input Method Selection
        let telexItem = NSMenuItem(title: "Input Method: Telex", action: #selector(selectTelex), keyEquivalent: "")
        telexItem.tag = 200
        menu.addItem(telexItem)

        let vniItem = NSMenuItem(title: "Input Method: VNI", action: #selector(selectVNI), keyEquivalent: "")
        vniItem.tag = 201
        menu.addItem(vniItem)
        
        menu.addItem(NSMenuItem.separator())
        
        // Settings - opens macOS standard Settings window
        let settingsMenuItem = NSMenuItem(
            title: "Settings...",
            action: #selector(AppDelegate.openSettings),
            keyEquivalent: ","
        )
        settingsMenuItem.target = self
        NSLog("[GoxViet] Added Settings menu item")
        menu.addItem(settingsMenuItem)

        // Quit
        menu.addItem(NSMenuItem.separator())
        menu.addItem(NSMenuItem(
            title: "Quit",
            action: #selector(quitApp),
            keyEquivalent: "q"
        ))
        
        statusItem.menu = menu
        
        // Update all menu states
        updateMenuStates()
    }
    
    func updateMenuStates() {
        DispatchQueue.main.async { [weak self] in
            guard let self = self, let menu = self.statusItem.menu else { return }
            
            // Update Vietnamese Input state
            if let toggleItem = menu.item(withTag: 100) {
                toggleItem.state = SettingsManager.shared.isEnabled ? .on : .off
            }
            
            // Update Input Method state
            let currentMethod = SettingsManager.shared.inputMethod
            if let telexItem = menu.item(withTag: 200) {
                telexItem.state = (currentMethod == 0) ? .on : .off
            }
            if let vniItem = menu.item(withTag: 201) {
                vniItem.state = (currentMethod == 1) ? .on : .off
            }
        }
    }


    
    // MARK: - Settings Window
    
    @objc func openSettings() {
        // Prevent opening settings during termination
        guard !isTerminating else {
            Log.warning("Cannot open settings - application is terminating")
            return
        }
        
        NSLog("[GoxViet] openSettings() called")

        // Always elevate to regular to show Settings and bring it forward
        ActivationPolicyCoordinator.shared.request(.regular)
        NSApp.activate(ignoringOtherApps: true)

        // Prefer SwiftUI openSettingsAction when available
        if SettingsActionBridge.shared.open() {
            NSLog("[GoxViet] openSettingsAction handled")
            registerSettingsCloseObserverForSystemSettings()
            focusSettingsWindow()
            return
        }

        NSLog("[GoxViet] openSettingsAction unavailable, fallback to WindowManager")
        WindowManager.shared.showSettingsWindow()

        // Ensure app is visible
        ActivationPolicyCoordinator.shared.request(.regular)
        NSApp.activate(ignoringOtherApps: true)

        focusSettingsWindow()

        NSLog("[GoxViet] Settings window should now be visible (fallback)")
    }

    /// Ensure Settings window is key and visible after being opened.
    private func focusSettingsWindow() {
        // Multiple attempts with increasing delays to catch Settings window
        for delay in [0.1, 0.2, 0.3] {
            DispatchQueue.main.asyncAfter(deadline: .now() + delay) {
                NSApp.setActivationPolicy(.regular)
                NSApp.activate(ignoringOtherApps: true)
                
                if let window = NSApplication.shared.windows.first(where: { window in
                    window.title == "Settings" || 
                    window.className.contains("Settings") ||
                    window.identifier?.rawValue.contains("settings") ?? false
                }) {
                    NSLog("[GoxViet] Found Settings window at delay \(delay), bringing to front")
                    window.level = .floating
                    window.makeKeyAndOrderFront(nil)
                    window.orderFrontRegardless()
                    NSApp.activate(ignoringOtherApps: true)
                    window.level = .normal
                }
            }
        }
    }

    /// Restore Dock visibility to user preference when system Settings window closes.
    private func registerSettingsCloseObserverForSystemSettings() {
        ResourceManager.shared.unregister(observerIdentifier: ObserverKey.settingsClose, center: notificationCenter)

        let observer = notificationCenter.addObserver(
            forName: NSWindow.willCloseNotification,
            object: nil,
            queue: .main
        ) { [weak self] notification in
            Task { @MainActor [weak self] in
                guard let self = self else { return }
                // Restore Dock policy whenever any window closes; we will check if a Settings window remains.
                self.restoreDockPolicyIfNoSettingsWindow()
                ResourceManager.shared.unregister(observerIdentifier: ObserverKey.settingsClose, center: self.notificationCenter)
            }
        }

        ResourceManager.shared.register(observer: observer, identifier: ObserverKey.settingsClose, center: notificationCenter)
    }

    /// Apply user preference for Dock visibility when no Settings window remains.
    private func restoreDockPolicyIfNoSettingsWindow() {
        // Delay check to allow window to fully close
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.2) {
            let hasSettingsWindow = NSApplication.shared.windows.contains { window in
                if window.isVisible == false { return false }
                let identifierMatch = window.identifier?.rawValue.lowercased().contains("settings") ?? false
                return identifierMatch || window.className.contains("Settings") || window.title == "Settings"
            }

            guard !hasSettingsWindow else { return }

            // Read current user preference (may have been changed in Settings UI)
            let hideFromDock = SettingsManager.shared.hideFromDock
            let policy: NSApplication.ActivationPolicy = hideFromDock ? .accessory : .regular
            
            NSLog("[GoxViet] Restoring Dock policy: hideFromDock=\(hideFromDock), policy=\(policy == .accessory ? "accessory" : "regular")")
            
            // Force immediate application
            NSApp.setActivationPolicy(policy)
        }
    }
    
    func setupObservers() {
        // Clear any existing observers first to prevent duplicates
        cleanupObservers()
        
        // Listen for state changes
        let stateToken = notificationCenter.addObserver(
            forName: .updateStateChanged,
            object: nil,
            queue: .main
        ) { [weak self] notification in
            guard notification.object as? Bool != nil else { return }
            Task { @MainActor [weak self] in
                self?.updateStatusIcon()
                self?.updateMenuStates()
            }
        }
        ResourceManager.shared.register(observer: stateToken, identifier: ObserverKey.updateState, center: notificationCenter)
        
        // Listen for toggle requests
        let toggleToken = notificationCenter.addObserver(
            forName: .toggleVietnamese,
            object: nil,
            queue: .main
        ) { [weak self] notification in
            guard notification.object as? Bool != nil else { return }
            Task { @MainActor [weak self] in
                self?.updateStatusIcon()
                self?.updateMenuStates()
            }
        }
        ResourceManager.shared.register(observer: toggleToken, identifier: ObserverKey.toggleVietnamese, center: notificationCenter)
        
        // Listen for shortcut changes
        let shortcutToken = notificationCenter.addObserver(
            forName: NSNotification.Name("shortcutChanged"),
            object: nil,
            queue: .main
        ) { _ in
            Task { @MainActor in
                // Shortcut display is only in Settings, no menu update needed
                Log.info("Shortcut changed")
            }
        }
        ResourceManager.shared.register(observer: shortcutToken, identifier: ObserverKey.shortcutChanged, center: notificationCenter)

        // Listen for input method changes
        let inputMethodToken = notificationCenter.addObserver(
            forName: .inputMethodChanged,
            object: nil,
            queue: .main
        ) { [weak self] _ in
            Task { @MainActor [weak self] in
                self?.updateMenuStates()
            }
        }
        ResourceManager.shared.register(observer: inputMethodToken, identifier: ObserverKey.inputMethod, center: notificationCenter)
        
        // Listen for app becoming active (detect permission changes)
        let activateToken = notificationCenter.addObserver(
            forName: NSApplication.didBecomeActiveNotification,
            object: nil,
            queue: .main
        ) { [weak self] _ in
            Task { @MainActor [weak self] in
                self?.checkPermissionOnActivate()
            }
        }
        ResourceManager.shared.register(observer: activateToken, identifier: ObserverKey.appActivation, center: notificationCenter)
        
        // Listen for settings window cleanup notification
        let cleanupToken = notificationCenter.addObserver(
            forName: NSNotification.Name("settingsWindowCleanup"),
            object: nil,
            queue: .main
        ) { [weak self] _ in
            Task { @MainActor [weak self] in
                self?.handleSettingsWindowCleanup()
            }
        }
        ResourceManager.shared.register(observer: cleanupToken, identifier: ObserverKey.settingsCleanup, center: notificationCenter)

        // Listen for CGEventTap being disabled by accessibility revocation
        let revokedToken = notificationCenter.addObserver(
            forName: .accessibilityPermissionRevoked,
            object: nil,
            queue: .main
        ) { [weak self] _ in
            Task { @MainActor [weak self] in
                guard let self = self else { return }
                Log.warning("Accessibility permission revoked — prompting user to re-grant")
                // Mark as post-update so the alert message instructs toggle OFF/ON
                self.isPostUpdateLaunch = true
                self.checkAccessibilityPermission()
            }
        }
        ResourceManager.shared.register(observer: revokedToken, identifier: ObserverKey.accessibilityRevoked, center: notificationCenter)

    }
    
    private func cleanupObservers() {
        let identifiers = [
            ObserverKey.updateState,
            ObserverKey.toggleVietnamese,
            ObserverKey.shortcutChanged,
            ObserverKey.appActivation,
            ObserverKey.inputMethod,
            ObserverKey.settingsClose,
            ObserverKey.settingsCleanup,
            ObserverKey.accessibilityRevoked
        ]
        identifiers.forEach { identifier in
            ResourceManager.shared.unregister(observerIdentifier: identifier, center: notificationCenter)
        }
    }
    
    deinit {}
    
    func checkPermissionOnActivate() {
        let accessEnabled = AXIsProcessTrusted()
        
        // If permission is now granted and InputManager isn't running, start it
        if accessEnabled && !InputManager.shared.isRunning {
            Log.info("Accessibility permission detected on app activation - starting InputManager")
            InputManager.shared.start()
        }
    }
    
    func updateStatusIcon() {
        if let button = statusItem.button {
            button.title = isEnabled ? "VI" : "EN"
            button.toolTip = isEnabled ? "Gõ Việt (Enabled)" : "Gõ Việt (Disabled)"
        }
    }
    
    // MARK: - Toggle Handlers
    
    func handleToggleChanged(_ newState: Bool) {
        InputManager.shared.setEnabled(newState)
        updateStatusIcon()
        updateMenuStates()
        
        Log.info("Toggle Vietnamese: \(newState ? "ON" : "OFF")")
    }
    
    // MARK: - Menu Actions
    
    @objc func toggleVietnamese(_ sender: Any?) {
        // Prevent actions during termination
        guard !isTerminating else { return }
        
        // Toggle state
        let newState = !SettingsManager.shared.isEnabled
        handleToggleChanged(newState)
    }
    
    @objc func selectTelex() {
        guard !isTerminating else { return }
        
        SettingsManager.shared.setInputMethod(0)
        InputManager.shared.setInputMethod(0)
        updateMenuStates()
        Log.info("Input method: Telex (selected from Menu)")
    }
    
    @objc func selectVNI() {
        guard !isTerminating else { return }
        
        SettingsManager.shared.setInputMethod(1)
        InputManager.shared.setInputMethod(1)
        updateMenuStates()
        Log.info("Input method: VNI (selected from Menu)")
    }
    
    @objc func selectModernTone() {
        InputManager.shared.setModernToneStyle(true)
        Log.info("Tone style: Modern (changed in Settings)")
    }
    
    @objc func selectOldTone() {
        InputManager.shared.setModernToneStyle(false)
        Log.info("Tone style: Traditional (changed in Settings)")
    }
    
//    @objc func checkForUpdates() {
//        // Open Settings window to About tab where update UI is now located
//        WindowManager.shared.showSettingsWindow()
//        // Update check is auto-triggered when About tab appears
//    }
    
//    @objc func showAbout() {
//        let shortcut = InputManager.shared.getCurrentShortcut()
//        let alert = NSAlert()
//        alert.messageText = "GoxViet - Gõ Việt"
//        let version = Bundle.main.object(forInfoDictionaryKey: "CFBundleShortVersionString") as? String ?? "?"
//        let build = Bundle.main.object(forInfoDictionaryKey: "CFBundleVersion") as? String ?? "?"
//        alert.informativeText = """
//        A high-performance Vietnamese IME powered by Rust.
//
//        Version: \(version) (Build \(build))
//
//        Features:
//        • Native macOS integration via Accessibility API
//        • Ultra-low latency input processing (< 5ms)
//        • Smart text injection (app-aware)
//        • Per-app Vietnamese mode memory
//        • Telex and VNI input methods
//        • Modern and traditional tone styles
//
//        Toggle Shortcut: \(shortcut.displayString)
//        (Use \(shortcut.displayString) to switch between Gõ Việt and English)
//
//        Built with ❤️ using Rust + Swift
//        """
//        alert.alertStyle = .informational
//        alert.addButton(withTitle: "OK")
//        alert.runModal()
//    }
    
    @objc func quitApp() {
        // Prevent double termination
        guard !isTerminating else { return }
        
        isTerminating = true
        Log.info("Application quitting via menu")
        
        // Stop input processing immediately
        InputManager.shared.stop()
        
        // Remove menu to prevent further interactions
        statusItem.menu = nil
        
        // Request termination
        NSApplication.shared.terminate(self)
    }
    
    func applicationShouldTerminate(_ sender: NSApplication) -> NSApplication.TerminateReply {
        Log.info("Termination approved")
        return .terminateNow
    }
    
    func applicationWillTerminate(_ aNotification: Notification) {
        Log.info("Application terminating - starting cleanup...")
        
        // Set termination flag to prevent new operations
        isTerminating = true
        
        // Stop event tap immediately to prevent new keyboard events
        InputManager.shared.stop()

        // Flush any debounced UserDefaults writes so no settings are lost on quit
        SettingsManager.shared.flushPendingSaves()

        // Cancel all pending operations
        NSObject.cancelPreviousPerformRequests(withTarget: self)
        
        // Cleanup all timers
        stopAccessibilityPollTimer()
        
        // Close all windows
        NSApp.windows.forEach { window in
            window.close()
        }
        
        // Remove status item immediately
        if let item = statusItem {
            NSStatusBar.system.removeStatusItem(item)
            statusItem = nil
        }
        
        // Stop all managers
        UpdateManager.shared.stop()
        
        // Cleanup observers
        cleanupObservers()
        
        // Cleanup ResourceManager
        ResourceManager.shared.cleanup()
        
        // Cleanup SettingsManager
        SettingsManager.shared.cleanup()
        
        // Force release of any remaining objects
        autoreleasepool {
            // Additional cleanup in autorelease pool
        }
        
        Log.info("Application cleanup completed - ready to terminate")
    }
    
    // MARK: - Application Lifecycle
    
    func applicationShouldHandleReopen(_ sender: NSApplication, hasVisibleWindows flag: Bool) -> Bool {
        // When user clicks app icon, open Settings window
        openSettings()
        return false
    }
    
    // MARK: - Memory Cleanup Handler
    
    @objc private func handleSettingsWindowCleanup() {
        // Clear SettingsManager caches when settings window closes
        SettingsManager.shared.clearCaches()
        
        // Force autorelease pool
        autoreleasepool {
            // Additional cleanup
        }
        
        // Log memory after cleanup
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.5) { [weak self] in
            _ = self // suppress unused warning
            #if DEBUG
            NSLog("[GoxViet] Settings cleanup complete")
            #endif
        }
    }
}

