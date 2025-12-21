//
//  AppDelegate.swift
//  GoxViet
//
//  Enhanced with toggle functionality and state management
//

import Cocoa

class AppDelegate: NSObject, NSApplicationDelegate {
    
    var statusItem: NSStatusItem!
    var toggleView: MenuToggleView?
    var smartModeToggleView: MenuToggleView?
    
    var isEnabled: Bool {
        return AppState.shared.isEnabled
    }
    
    func applicationDidFinishLaunching(_ aNotification: Notification) {
        // Enable logging in debug mode
        #if DEBUG
        Log.isEnabled = true
        Log.info("GoxViet starting in DEBUG mode")
        #endif
        
        // Create Status Bar Item first (before permission check)
        statusItem = NSStatusBar.system.statusItem(withLength: NSStatusItem.variableLength)
        
        updateStatusIcon()
        
        setupMenu()
        setupObservers()
        
        // Check and request Accessibility Permission
        // InputManager will only start if permission is granted
        checkAccessibilityPermission()
        
        Log.info("Application launched successfully")
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
            
            // Start InputManager only after permission is confirmed
            InputManager.shared.start()
        }
    }
    
    func showAccessibilityAlert() {
        let alert = NSAlert()
        alert.messageText = "🔐 Accessibility Permission Required"
        alert.informativeText = """
        GoxViet needs Accessibility permissions to capture keyboard input for Vietnamese typing.
        
        📝 Please follow these steps:
        
        1️⃣ Click "Open System Preferences" below
        2️⃣ Find "GoxViet" in the list
        3️⃣ Check the box next to "GoxViet"
        4️⃣ Close System Preferences
        5️⃣ Click "Check Again" or restart GoxViet
        
        ⚠️ Without this permission, Vietnamese input will NOT work.
        
        💡 Tip: If you don't see GoxViet in the list, try:
           • Restart GoxViet
           • Or manually add it using the + button
        """
        alert.alertStyle = .warning
        alert.addButton(withTitle: "Open System Preferences")
        alert.addButton(withTitle: "Check Again")
        alert.addButton(withTitle: "Quit")
        
        let response = alert.runModal()
        
        switch response {
        case .alertFirstButtonReturn:
            // Open System Preferences
            let prefpaneUrl = URL(string: "x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")!
            NSWorkspace.shared.open(prefpaneUrl)
            
            // Wait longer for user to grant permission
            DispatchQueue.main.asyncAfter(deadline: .now() + 3.0) { [weak self] in
                self?.recheckAccessibilityPermission()
            }
            
        case .alertSecondButtonReturn:
            // Check again immediately
            recheckAccessibilityPermission()
            
        case .alertThirdButtonReturn:
            // Quit
            NSApplication.shared.terminate(self)
            
        default:
            break
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
            Log.info("Accessibility permission now granted")
            
            // Start InputManager now that permission is granted
            InputManager.shared.start()
            
            let successAlert = NSAlert()
            successAlert.messageText = "Permission Granted! ✅"
            successAlert.informativeText = "GoxViet can now function properly.\n\nYou may need to restart the app if input doesn't work immediately."
            successAlert.alertStyle = .informational
            successAlert.addButton(withTitle: "OK")
            successAlert.addButton(withTitle: "Restart Now")
            
            let response = successAlert.runModal()
            if response == .alertSecondButtonReturn {
                // Restart app
                let url = URL(fileURLWithPath: Bundle.main.resourcePath!)
                let path = url.deletingLastPathComponent().deletingLastPathComponent().absoluteString
                let task = Process()
                task.launchPath = "/usr/bin/open"
                task.arguments = [path]
                task.launch()
                NSApplication.shared.terminate(self)
            }
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
            action: #selector(showSettings),
            keyEquivalent: ","
        ))
        
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
        // Listen for state changes
        NotificationCenter.default.addObserver(
            forName: .updateStateChanged,
            object: nil,
            queue: .main
        ) { [weak self] notification in
            if notification.object as? Bool != nil {
                self?.updateStatusIcon()
                self?.updateToggleMenuItem()
            }
        }
        
        // Listen for toggle requests
        NotificationCenter.default.addObserver(
            forName: .toggleVietnamese,
            object: nil,
            queue: .main
        ) { [weak self] notification in
            if notification.object as? Bool != nil {
                self?.updateStatusIcon()
                self?.updateToggleMenuItem()
            }
        }
        
        // Listen for shortcut changes
        NotificationCenter.default.addObserver(
            forName: .shortcutChanged,
            object: nil,
            queue: .main
        ) { [weak self] _ in
            self?.setupMenu()  // Rebuild menu to show new shortcut
        }
        
        // Listen for app becoming active (detect permission changes)
        NotificationCenter.default.addObserver(
            forName: NSApplication.didBecomeActiveNotification,
            object: nil,
            queue: .main
        ) { [weak self] _ in
            self?.checkPermissionOnActivate()
        }
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
        let alert = NSAlert()
        alert.messageText = "Settings"
        
        let currentApp = PerAppModeManager.shared.getCurrentAppName() ?? "Unknown"
        let smartModeStatus = AppState.shared.isSmartModeEnabled ? "Enabled" : "Disabled"
        let perAppCount = AppState.shared.getAllPerAppModes().count
        
        alert.informativeText = """
        Current Configuration:
        
        • Input Method: \(AppState.shared.inputMethod == 0 ? "Telex" : "VNI")
        • Tone Style: \(AppState.shared.modernToneStyle ? "Modern" : "Traditional")
        • ESC Restore: \(AppState.shared.escRestoreEnabled ? "Enabled" : "Disabled")
        • Free Tone: \(AppState.shared.freeToneEnabled ? "Enabled" : "Disabled")
        
        Smart Per-App Mode: \(smartModeStatus)
        • Current App: \(currentApp)
        • Apps with custom settings: \(perAppCount)
        
        Use the menu to configure settings.
        """
        
        alert.alertStyle = .informational
        alert.addButton(withTitle: "OK")
        
        if perAppCount > 0 {
            alert.addButton(withTitle: "Clear Per-App Settings")
            let response = alert.runModal()
            if response == .alertSecondButtonReturn {
                clearPerAppSettings()
            }
        } else {
            alert.runModal()
        }
    }
    
    func clearPerAppSettings() {
        let confirmAlert = NSAlert()
        confirmAlert.messageText = "Clear All Per-App Settings?"
        confirmAlert.informativeText = "This will reset Gõ Việt input mode to default (enabled) for all applications."
        confirmAlert.alertStyle = .warning
        confirmAlert.addButton(withTitle: "Clear")
        confirmAlert.addButton(withTitle: "Cancel")
        
        if confirmAlert.runModal() == .alertFirstButtonReturn {
            AppState.shared.clearAllPerAppModes()
            
            let successAlert = NSAlert()
            successAlert.messageText = "Settings Cleared"
            successAlert.informativeText = "All per-app settings have been reset."
            successAlert.alertStyle = .informational
            successAlert.addButton(withTitle: "OK")
            successAlert.runModal()
        }
    }
    
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
        alert.informativeText = """
        A high-performance Vietnamese IME powered by Rust.
        
        Version: 1.0.2
        
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
        NSApplication.shared.terminate(self)
    }
    
    func applicationWillTerminate(_ aNotification: Notification) {
        Log.info("Application terminating")
        InputManager.shared.stop()
    }
    
    // MARK: - Application Lifecycle
    
    func applicationShouldHandleReopen(_ sender: NSApplication, hasVisibleWindows flag: Bool) -> Bool {
        // When user clicks app icon again, show about dialog
        if !flag {
            showAbout()
        }
        return true
    }
}
