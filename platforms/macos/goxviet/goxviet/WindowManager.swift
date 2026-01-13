//
//  WindowManager.swift
//  GoxViet
//
//  Manually manages NSWindow lifecycle to ensure complete memory cleanup
//  when windows are closed.
//

import Cocoa
import SwiftUI

class WindowManager: NSObject, NSWindowDelegate {
    static let shared = WindowManager()
    
    // Check if the windows are currently open
    var isSettingsWindowOpen: Bool { return settingsWindow != nil }
    var isUpdateWindowOpen: Bool { return updateWindow != nil }
    
    private var settingsWindow: NSWindow?
    private var updateWindow: NSWindow?
    
    private override init() {
        super.init()
    }
    
    // MARK: - Settings Window
    
    func showSettingsWindow() {
        // If window exists, bring to front
        if let window = settingsWindow {
            setActivationPolicy(.regular)
            DispatchQueue.main.async {
                window.makeKeyAndOrderFront(nil)
                NSApp.activate(ignoringOtherApps: true)
            }
            return
        }
        
        // Create new window
        let window = NSWindow(
            contentRect: NSRect(x: 0, y: 0, width: 820, height: 520),
            styleMask: [.titled, .closable, .miniaturizable, .resizable, .fullSizeContentView],
            backing: .buffered,
            defer: false
        )
        
        window.center()
        window.title = "GoxViet Settings"
        window.titlebarAppearsTransparent = true
        window.titleVisibility = .hidden
        window.isReleasedWhenClosed = false // We handle cleanup manually
        window.delegate = self
        window.identifier = NSUserInterfaceItemIdentifier("settings")
        window.minSize = NSSize(width: 820, height: 520)
        window.isRestorable = false  // Disable window restoration
        
        // Set content view using NSHostingView
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
        
        Log.info("Created direct Settings window")
    }
    
    func closeSettingsWindow() {
        settingsWindow?.close()
        settingsWindow = nil
        handleLastWindowClosed()
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
        window.isReleasedWhenClosed = false
        window.delegate = self
        window.identifier = NSUserInterfaceItemIdentifier("update")
        window.isRestorable = false  // Disable window restoration
        
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
        
        Log.info("Created direct Update window")
    }
    
    func closeUpdateWindow() {
        updateWindow?.close()
        updateWindow = nil
        handleLastWindowClosed()
    }
    
    // MARK: - Helper Logic
    
    private func handleLastWindowClosed() {
        // If no windows are open, restore background mode policy
        if settingsWindow == nil && updateWindow == nil {
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
        
        if window === settingsWindow {
            Log.info("Settings window closing - releasing memory")
            settingsWindow = nil
        } else if window === updateWindow {
            Log.info("Update window closing - releasing memory")
            updateWindow = nil
        }
        
        // Ensure policy is updated after window is gone
        DispatchQueue.main.async { [weak self] in
            self?.handleLastWindowClosed()
        }
    }
}
