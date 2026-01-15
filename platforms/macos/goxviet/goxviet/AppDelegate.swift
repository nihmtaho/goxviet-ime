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
    var toggleView: MenuToggleView?
    var smartModeToggleView: MenuToggleView?
    
    // NotificationCenter observer tokens for proper cleanup
    private var observerTokens: [NSObjectProtocol] = []
    
    // Timer for auto-polling accessibility permission
    private var accessibilityPollTimer: Timer?
    
    // Flag to track if permission was granted while modal was showing
    private var permissionGrantedWhileModalActive = false
    private var isModalAlertActive = false
    
    var isEnabled: Bool {
        return AppState.shared.isEnabled
    }

    private func applyActivationPolicyFromPreference() {
        let hide = UserDefaults.standard.bool(forKey: "com.goxviet.ime.hideFromDock")
        let policy: NSApplication.ActivationPolicy = hide ? .accessory : .regular

        // Delegate to coordinator to coalesce and apply outside layout passes
        ActivationPolicyCoordinator.shared.request(policy)
    }
    
    func applicationDidFinishLaunching(_ aNotification: Notification) {
        // Enable logging in debug mode
        #if DEBUG
        Log.isEnabled = true
        Log.info("GoxViet starting in DEBUG mode")
        #endif
        
        // Disable automatic window restoration to avoid className errors
        UserDefaults.standard.register(defaults: ["NSQuitAlwaysKeepsWindows": false])
        
        // Apply Dock visibility from user preference
        applyActivationPolicyFromPreference()
        
        // Create Status Bar Item first (before permission check)
        statusItem = NSStatusBar.system.statusItem(withLength: NSStatusItem.variableLength)
        
        updateStatusIcon()
        
        setupMenu()
        setupObservers()
        
        // Check and request Accessibility Permission
        // InputManager will only start if permission is granted
        checkAccessibilityPermission()

        // Start background update checks
        UpdateManager.shared.start()
        
        Log.info("Application launched successfully")
    }
    
    // MARK: - Settings Window (SwiftUI)
    
    @objc func showSettingsWindow() {
        WindowManager.shared.showSettingsWindow()
    }
    
    // MARK: - Accessibility Permission
    
    func checkAccessibilityPermission() {
        // Check WITHOUT showing system prompt (no duplicate dialogs)
        let accessEnabled = AXIsProcessTrusted()
        
        if !accessEnabled {
            Log.warning("Accessibility permission not granted")
            
            // Show only our custom alert (not system prompt)
            DispatchQueue.main.async { [weak self] in
                self?.showAccessibilityAlert()
            }
        } else {
            Log.info("Accessibility permission granted")
            stopAccessibilityPollTimer()
            
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
        accessibilityPollTimer = Timer.scheduledTimer(withTimeInterval: 1.0, repeats: true) { [weak self] _ in
            guard let self = self else { return }
            let accessEnabled = AXIsProcessTrusted()
            if accessEnabled {
                Log.info("Accessibility permission detected via auto-polling")
                self.stopAccessibilityPollTimer()
                
                // If modal is active, just set the flag - don't try to manipulate UI
                if self.isModalAlertActive {
                    self.permissionGrantedWhileModalActive = true
                    Log.info("Permission granted while modal active - will handle after modal closes")
                } else {
                    self.onAccessibilityGranted()
                }
            }
        }
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
        
        accessibilityPollTimer?.invalidate()
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
        InputManager.shared.start()
    }
    
    func showAccessibilityAlert() {
        // Reset flag
        permissionGrantedWhileModalActive = false
        
        // Start auto-polling when showing the alert
        startAccessibilityPollTimer()
        
        isModalAlertActive = true
        
        let alert = NSAlert()
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
        let menu = NSMenu()
        
        // Toggle Item with NSSwitch using custom view
        let toggleMenuItem = NSMenuItem()
        toggleMenuItem.tag = 100
        
        // Create custom toggle view
        toggleView = MenuToggleView(labelText: "Vietnamese Input", isOn: AppState.shared.isEnabled) { [weak self] newState in
            self?.handleToggleChanged(newState)
        }
        
        toggleMenuItem.view = toggleView
        menu.addItem(toggleMenuItem)
        
        menu.addItem(NSMenuItem.separator())
        
        // Smart Per-App Mode Toggle
        let smartModeMenuItem = NSMenuItem()
        smartModeMenuItem.tag = 101
        
        smartModeToggleView = MenuToggleView(
            labelText: "Smart Per-App Mode",
            isOn: AppState.shared.isSmartModeEnabled
        ) { [weak self] newState in
            self?.handleSmartModeChanged(newState)
        }
        
        smartModeMenuItem.view = smartModeToggleView
        menu.addItem(smartModeMenuItem)
        
        menu.addItem(NSMenuItem.separator())
        
        // Shortcut info (non-clickable)
        let shortcutInfo = NSMenuItem(
            title: "Toggle: \(InputManager.shared.getCurrentShortcut().displayString)",
            action: nil,
            keyEquivalent: ""
        )
        shortcutInfo.isEnabled = false
        menu.addItem(shortcutInfo)
        
        menu.addItem(NSMenuItem.separator())
        
        // Input Method submenu
        let methodMenu = NSMenu()
        let telexItem = NSMenuItem(title: "Telex", action: #selector(selectTelex), keyEquivalent: "")
        telexItem.tag = 0
        telexItem.state = (AppState.shared.inputMethod == 0) ? .on : .off
        methodMenu.addItem(telexItem)
        
        let vniItem = NSMenuItem(title: "VNI", action: #selector(selectVNI), keyEquivalent: "")
        vniItem.tag = 1
        vniItem.state = (AppState.shared.inputMethod == 1) ? .on : .off
        methodMenu.addItem(vniItem)
        
        let methodMenuItem = NSMenuItem(title: "Input Method", action: nil, keyEquivalent: "")
        methodMenuItem.submenu = methodMenu
        menu.addItem(methodMenuItem)
        
        // Tone Style submenu
        let toneMenu = NSMenu()
        let modernToneItem = NSMenuItem(title: "Modern (hoà, thuỷ)", action: #selector(selectModernTone), keyEquivalent: "")
        modernToneItem.tag = 1
        modernToneItem.state = AppState.shared.modernToneStyle ? .on : .off
        toneMenu.addItem(modernToneItem)
        
        let oldToneItem = NSMenuItem(title: "Traditional (hòa, thủy)", action: #selector(selectOldTone), keyEquivalent: "")
        oldToneItem.tag = 0
        oldToneItem.state = AppState.shared.modernToneStyle ? .off : .on
        toneMenu.addItem(oldToneItem)
        
        let toneMenuItem = NSMenuItem(title: "Tone Style", action: nil, keyEquivalent: "")
        toneMenuItem.submenu = toneMenu
        menu.addItem(toneMenuItem)
        
        menu.addItem(NSMenuItem.separator())
        
        // Settings
        menu.addItem(NSMenuItem(
            title: "Settings...",
            action: #selector(showSettingsWindow),
            keyEquivalent: ","
        ))

        let updateMenuItem = NSMenuItem(
            title: "Check for Updates...",
            action: #selector(checkForUpdates),
            keyEquivalent: ""
        )
        updateMenuItem.target = self
        menu.addItem(updateMenuItem)
        
        // View Log (Debug)
        #if DEBUG
        menu.addItem(NSMenuItem(
            title: "View Log...",
            action: #selector(viewLog),
            keyEquivalent: ""
        ))
        #endif
        
        menu.addItem(NSMenuItem.separator())
        
        // About
        menu.addItem(NSMenuItem(
            title: "About GoxViet",
            action: #selector(showAbout),
            keyEquivalent: ""
        ))
        
        // Quit
        menu.addItem(NSMenuItem.separator())
        menu.addItem(NSMenuItem(
            title: "Quit",
            action: #selector(quitApp),
            keyEquivalent: "q"
        ))
        
        statusItem.menu = menu
    }
    
    func setupObservers() {
        // Clear any existing observers first to prevent duplicates
        cleanupObservers()
        
        // Listen for state changes
        let stateToken = NotificationCenter.default.addObserver(
            forName: .updateStateChanged,
            object: nil,
            queue: .main
        ) { [weak self] notification in
            if notification.object as? Bool != nil {
                self?.updateStatusIcon()
                self?.updateToggleMenuItem()
            }
        }
        observerTokens.append(stateToken)
        
        // Listen for toggle requests
        let toggleToken = NotificationCenter.default.addObserver(
            forName: .toggleVietnamese,
            object: nil,
            queue: .main
        ) { [weak self] notification in
            if notification.object as? Bool != nil {
                self?.updateStatusIcon()
                self?.updateToggleMenuItem()
            }
        }
        observerTokens.append(toggleToken)
        
        // Listen for shortcut changes
        let shortcutToken = NotificationCenter.default.addObserver(
            forName: .shortcutChanged,
            object: nil,
            queue: .main
        ) { [weak self] _ in
            self?.setupMenu()  // Rebuild menu to show new shortcut
        }
        observerTokens.append(shortcutToken)
        
        // Listen for app becoming active (detect permission changes)
        let activateToken = NotificationCenter.default.addObserver(
            forName: NSApplication.didBecomeActiveNotification,
            object: nil,
            queue: .main
        ) { [weak self] _ in
            self?.checkPermissionOnActivate()
        }
        observerTokens.append(activateToken)
        
        // Listen for internal open window requests (from Settings UI buttons etc)
        let openUpdateToken = NotificationCenter.default.addObserver(
            forName: .openUpdateWindow,
            object: nil,
            queue: .main
        ) { _ in
            WindowManager.shared.showUpdateWindow()
        }
        observerTokens.append(openUpdateToken)
    }
    
    private func cleanupObservers() {
        for token in observerTokens {
            NotificationCenter.default.removeObserver(token)
        }
        observerTokens.removeAll()
    }
    
    deinit {
        cleanupObservers()
        cleanupMenuViews()
        stopAccessibilityPollTimer()
    }
    
    private func cleanupMenuViews() {
        toggleView?.cleanup()
        toggleView = nil
        smartModeToggleView?.cleanup()
        smartModeToggleView = nil
    }
    
    func checkPermissionOnActivate() {
        let accessEnabled = AXIsProcessTrusted()
        
        // If permission is now granted and InputManager isn't running, start it
        if accessEnabled && !InputManager.shared.isRunning() {
            Log.info("Accessibility permission detected on app activation - starting InputManager")
            InputManager.shared.start()
        }
    }
    
    func updateStatusIcon() {
        if let button = statusItem.button {
            button.title = isEnabled ? "🇻🇳" : "EN"
            button.toolTip = isEnabled ? "Gõ Việt (Enabled)" : "Gõ Việt (Disabled)"
        }
    }
    
    func updateToggleMenuItem() {
        toggleView?.updateState(isEnabled)
    }
    
    // MARK: - Toggle Handlers
    
    func handleToggleChanged(_ newState: Bool) {
        InputManager.shared.setEnabled(newState)
        updateStatusIcon()
        
        Log.info("Toggle Vietnamese: \(newState ? "ON" : "OFF")")
    }
    
    func handleSmartModeChanged(_ newState: Bool) {
        AppState.shared.isSmartModeEnabled = newState
        
        if newState {
            // Refresh to apply saved state for current app
            PerAppModeManager.shared.refresh()
        }
        
        Log.info("Smart Per-App Mode: \(newState ? "ON" : "OFF")")
    }
    
    // MARK: - Menu Actions
    
    @objc func toggleVietnamese(_ sender: Any?) {
        // Toggle state
        let newState = !AppState.shared.isEnabled
        handleToggleChanged(newState)
        updateToggleMenuItem()
    }
    
    @objc func selectTelex() {
        InputManager.shared.setInputMethod(0)
        updateMethodMenuSelection(selectedTag: 0)
        Log.info("Input method: Telex")
    }
    
    @objc func selectVNI() {
        InputManager.shared.setInputMethod(1)
        updateMethodMenuSelection(selectedTag: 1)
        Log.info("Input method: VNI")
    }
    
    func updateMethodMenuSelection(selectedTag: Int) {
        guard let menu = statusItem.menu,
              let methodMenuItem = menu.item(withTitle: "Input Method"),
              let methodMenu = methodMenuItem.submenu else { return }
        
        for item in methodMenu.items {
            item.state = (item.tag == selectedTag) ? .on : .off
        }
    }
    
    @objc func selectModernTone() {
        InputManager.shared.setModernToneStyle(true)
        updateToneMenuSelection(selectedTag: 1)
        Log.info("Tone style: Modern")
    }
    
    @objc func selectOldTone() {
        InputManager.shared.setModernToneStyle(false)
        updateToneMenuSelection(selectedTag: 0)
        Log.info("Tone style: Traditional")
    }
    
    func updateToneMenuSelection(selectedTag: Int) {
        guard let menu = statusItem.menu,
              let toneMenuItem = menu.item(withTitle: "Tone Style"),
              let toneMenu = toneMenuItem.submenu else { return }
        
        for item in toneMenu.items {
            item.state = (item.tag == selectedTag) ? .on : .off
        }
    }
    
    @objc func showSettings() {
        // Show settings window using GoxVietApp
        showSettingsWindow()
    }

    @objc func checkForUpdates() {
        WindowManager.shared.showUpdateWindow()
        // Trigger update check
        UpdateManager.shared.checkForUpdates(userInitiated: true)
    }
    
    // Removed clearPerAppSettings() - now handled in SettingsView
    
    @objc func viewLog() {
        if FileManager.default.fileExists(atPath: Log.logPath.path) {
            NSWorkspace.shared.open(Log.logPath)
        } else {
            let alert = NSAlert()
            alert.messageText = "Log File Not Found"
            alert.informativeText = "No log file exists yet. Enable logging in debug mode and perform some typing to generate logs."
            alert.alertStyle = .informational
            alert.addButton(withTitle: "OK")
            alert.runModal()
        }
    }
    
    @objc func showAbout() {
        let shortcut = InputManager.shared.getCurrentShortcut()
        let alert = NSAlert()
        alert.messageText = "GoxViet - Gõ Việt"
        let version = Bundle.main.object(forInfoDictionaryKey: "CFBundleShortVersionString") as? String ?? "?"
        let build = Bundle.main.object(forInfoDictionaryKey: "CFBundleVersion") as? String ?? "?"
        alert.informativeText = """
        A high-performance Vietnamese IME powered by Rust.

        Version: \(version) (Build \(build))

        Features:
        • Native macOS integration via Accessibility API
        • Ultra-low latency input processing (< 5ms)
        • Smart text injection (app-aware)
        • Per-app Vietnamese mode memory
        • Telex and VNI input methods
        • Modern and traditional tone styles

        Toggle Shortcut: \(shortcut.displayString)
        (Use \(shortcut.displayString) to switch between Gõ Việt and English)

        Built with ❤️ using Rust + Swift
        """
        alert.alertStyle = .informational
        alert.addButton(withTitle: "OK")

        #if DEBUG
        alert.addButton(withTitle: "View Log")
        let response = alert.runModal()
        if response == .alertSecondButtonReturn {
            viewLog()
        }
        #else
        alert.runModal()
        #endif
    }
    
    @objc func quitApp() {
        Log.info("Application quitting")
        InputManager.shared.stop()
        cleanupMenuViews()
        NSApplication.shared.terminate(self)
    }
    
    func applicationWillTerminate(_ aNotification: Notification) {
        Log.info("Application terminating")
        UpdateManager.shared.stop()
        InputManager.shared.stop()
        cleanupMenuViews()
    }
    
    // MARK: - Application Lifecycle
    
    func applicationShouldHandleReopen(_ sender: NSApplication, hasVisibleWindows flag: Bool) -> Bool {
        // When user clicks app icon, always show Settings window
        showSettingsWindow()
        return false // prevent default About popup
    }
}

