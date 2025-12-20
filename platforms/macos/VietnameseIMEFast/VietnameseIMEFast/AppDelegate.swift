//
//  AppDelegate.swift
//  VietnameseIMEFast
//
//  Enhanced with toggle functionality and state management from GoNhanh
//

import Cocoa

class AppDelegate: NSObject, NSApplicationDelegate {
    
    var statusItem: NSStatusItem!
    var isEnabled: Bool = true
    var toggleView: MenuToggleView?
    
    func applicationDidFinishLaunching(_ aNotification: Notification) {
        // Enable logging in debug mode
        #if DEBUG
        Log.isEnabled = true
        Log.info("VietnameseIMEFast starting in DEBUG mode")
        #endif
        
        // Create Status Bar Item
        statusItem = NSStatusBar.system.statusItem(withLength: NSStatusItem.variableLength)
        
        updateStatusIcon()
        
        setupMenu()
        setupObservers()
        
        // Start the Input Manager
        InputManager.shared.start()
        
        Log.info("Application launched successfully")
    }
    
    func setupMenu() {
        let menu = NSMenu()
        
        // Toggle Item with NSSwitch using custom view
        let toggleMenuItem = NSMenuItem()
        toggleMenuItem.tag = 100
        
        // Create custom toggle view
        toggleView = MenuToggleView(labelText: "Vietnamese Input", isOn: isEnabled) { [weak self] newState in
            self?.handleToggleChanged(newState)
        }
        
        toggleMenuItem.view = toggleView
        menu.addItem(toggleMenuItem)
        
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
        telexItem.state = .on // Default
        methodMenu.addItem(telexItem)
        
        let vniItem = NSMenuItem(title: "VNI", action: #selector(selectVNI), keyEquivalent: "")
        vniItem.tag = 1
        methodMenu.addItem(vniItem)
        
        let methodMenuItem = NSMenuItem(title: "Input Method", action: nil, keyEquivalent: "")
        methodMenuItem.submenu = methodMenu
        menu.addItem(methodMenuItem)
        
        // Tone Style submenu
        let toneMenu = NSMenu()
        let modernToneItem = NSMenuItem(title: "Modern (hoà, thuỷ)", action: #selector(selectModernTone), keyEquivalent: "")
        modernToneItem.tag = 1
        toneMenu.addItem(modernToneItem)
        
        let oldToneItem = NSMenuItem(title: "Traditional (hòa, thủy)", action: #selector(selectOldTone), keyEquivalent: "")
        oldToneItem.tag = 0
        oldToneItem.state = .on // Default
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
            title: "About VietnameseIMEFast",
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
            if let enabled = notification.object as? Bool {
                self?.isEnabled = enabled
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
            if let enabled = notification.object as? Bool {
                self?.isEnabled = enabled
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
    }
    
    func updateStatusIcon() {
        if let button = statusItem.button {
            button.title = isEnabled ? "🇻🇳" : "EN"
            button.toolTip = isEnabled ? "Vietnamese Input (Enabled)" : "Vietnamese Input (Disabled)"
        }
    }
    
    func updateToggleMenuItem() {
        toggleView?.updateState(isEnabled)
    }
    
    // MARK: - Toggle Handler
    
    func handleToggleChanged(_ newState: Bool) {
        isEnabled = newState
        InputManager.shared.setEnabled(isEnabled)
        updateStatusIcon()
        
        Log.info("Toggle Vietnamese: \(isEnabled ? "ON" : "OFF")")
    }
    
    // MARK: - Menu Actions
    
    @objc func toggleVietnamese(_ sender: Any?) {
        // Toggle state
        isEnabled.toggle()
        handleToggleChanged(isEnabled)
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
        alert.informativeText = "Settings panel coming soon!\n\nFor now, use the menu to:\n• Toggle Vietnamese input\n• Switch input methods (Telex/VNI)\n• Change tone style"
        alert.alertStyle = .informational
        alert.addButton(withTitle: "OK")
        alert.runModal()
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
        alert.messageText = "VietnameseIMEFast"
        alert.informativeText = """
        A high-performance Vietnamese IME powered by Rust.
        
        Version: 1.0.0
        
        Features:
        • Native macOS integration via Accessibility API
        • Ultra-low latency input processing
        • Smart text injection (app-aware)
        • Telex and VNI input methods
        • Modern and traditional tone styles
        
        Toggle Shortcut: \(shortcut.displayString)
        (Use \(shortcut.displayString) to switch between Vietnamese and English)
        
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
