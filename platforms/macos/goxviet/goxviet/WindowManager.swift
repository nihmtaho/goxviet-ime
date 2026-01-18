//
//  WindowManager.swift
//  GoxViet
//
//  Manages application windows (Settings and Update).
//

import Cocoa
import SwiftUI

class WindowManager: NSObject, NSWindowDelegate {
    static let shared = WindowManager()
    
    // Check if windows are currently open
    var isUpdateWindowOpen: Bool { return updateWindow != nil }
    var isSettingsWindowOpen: Bool { return settingsWindow != nil }
    
    private var updateWindow: NSWindow?
    private var settingsWindow: NSWindow?
    
    private override init() {
        super.init()
    }
    
    // MARK: - Update Window
    
    func showUpdateWindow() {
        if let window = updateWindow {
            setActivationPolicy(.regular)
            DispatchQueue.main.async {
                window.makeKeyAndOrderFront(nil)
                NSApp.activate(ignoringOtherApps: true)
            }
            return
        }
        
        let window = NSWindow(
            contentRect: NSRect(x: 0, y: 0, width: 480, height: 520),
            styleMask: [.titled, .closable, .miniaturizable, .fullSizeContentView],
            backing: .buffered,
            defer: false
        )
        
        window.center()
        window.title = "Check for Updates"
        window.titlebarAppearsTransparent = true
        window.titleVisibility = .hidden
        window.isReleasedWhenClosed = false // Auto-release when closed to save memory
        window.delegate = self
        window.identifier = NSUserInterfaceItemIdentifier("update")
        window.isRestorable = false
        
        let contentView = UpdateWindowView()
        let hostingView = NSHostingView(rootView: contentView)
        hostingView.autoresizingMask = [.width, .height]
        window.contentView = hostingView
        
        self.updateWindow = window
        
        // Request activation policy change first
        setActivationPolicy(.regular)
        
        // Defer window show to allow policy change to complete
        DispatchQueue.main.async {
            window.makeKeyAndOrderFront(nil)
            NSApp.activate(ignoringOtherApps: true)
        }
        
        Log.info("Created Update window")
    }
    
    func closeUpdateWindow() {
        updateWindow?.close()
        updateWindow = nil
        handleLastWindowClosed()
    }
    
    // MARK: - Settings Window
    
    func showSettingsWindow() {
        if let window = settingsWindow {
            setActivationPolicy(.regular)
            DispatchQueue.main.async {
                window.makeKeyAndOrderFront(nil)
                NSApp.activate(ignoringOtherApps: true)
            }
            return
        }
        
        let window = NSWindow(
            contentRect: NSRect(x: 0, y: 0, width: 800, height: 600),
            styleMask: [.titled, .closable, .miniaturizable, .resizable, .fullSizeContentView],
            backing: .buffered,
            defer: false
        )
        
        window.center()
        window.title = "Settings"
        window.titlebarAppearsTransparent = true
        window.titleVisibility = .hidden
        window.isReleasedWhenClosed = false
        window.delegate = self
        window.identifier = NSUserInterfaceItemIdentifier("settings")
        window.isRestorable = false
        
        let contentView = SettingsRootView()
        let hostingView = NSHostingView(rootView: contentView)
        hostingView.autoresizingMask = [.width, .height]
        window.contentView = hostingView
        
        self.settingsWindow = window
        
        // Request activation policy change first
        setActivationPolicy(.regular)
        
        // Defer window show to allow policy change to complete
        DispatchQueue.main.async {
            window.makeKeyAndOrderFront(nil)
            NSApp.activate(ignoringOtherApps: true)
        }
        
        Log.info("Created Settings window")
    }
    
    func closeSettingsWindow() {
        settingsWindow?.close()
        settingsWindow = nil
        handleLastWindowClosed()
    }
    
    // MARK: - Helper Logic
    
    private func handleLastWindowClosed() {
        // If no windows are open, restore background mode policy
        if updateWindow == nil && settingsWindow == nil {
            let hideFromDock = AppState.shared.hideFromDock
            let policy: NSApplication.ActivationPolicy = hideFromDock ? .accessory : .regular
            
            setActivationPolicy(policy)
            Log.info("All windows closed. Policy set to: \(hideFromDock ? ".accessory" : ".regular")")
        }
    }
    
    private func setActivationPolicy(_ policy: NSApplication.ActivationPolicy) {
        // Only change if different to avoid redundant layout triggers
        guard NSApp.activationPolicy() != policy else { return }

        // Delegate to coordinator to apply outside current layout pass
        ActivationPolicyCoordinator.shared.request(policy)
    }
    
    // MARK: - NSWindowDelegate
    
    func windowWillClose(_ notification: Notification) {
        guard let window = notification.object as? NSWindow else { return }
        
        if window === updateWindow {
            Log.info("✅ Update window will close - releasing strong reference")
            updateWindow = nil
        } else if window === settingsWindow {
            Log.info("✅ Settings window will close - releasing strong reference")
            settingsWindow = nil
        }
        
        // Update policy after window is completely released
        DispatchQueue.main.async { [weak self] in
            self?.handleLastWindowClosed()
        }
    }
}
