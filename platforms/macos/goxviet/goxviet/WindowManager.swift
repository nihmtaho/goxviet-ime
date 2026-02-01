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
    
    // Check if settings window is open
    var isSettingsWindowOpen: Bool { return settingsWindow != nil }
    
    // Use weak reference to allow automatic deallocation
    private weak var settingsWindow: NSWindow?
    
    private override init() {
        super.init()
    }
    
    deinit {
        cleanup()
        Log.info("WindowManager deinitialized")
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
        window.isReleasedWhenClosed = true // Auto-release when closed to save memory
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
    
    /// Closes the Settings window independently without affecting Update window.
    /// This ensures that closing one window doesn't close the other.
    func closeSettingsWindow() {
        settingsWindow?.close()
        // cleanup delegated to windowWillClose via notifications/delegates
        // but explicit nulling is safe here as backup
        if settingsWindow == nil { handleLastWindowClosed() }
    }
    
    // MARK: - Helper Logic
    
    /// Called after a window closes to check if we need to switch back to background mode.
    /// IMPORTANT: Uses delayed policy change to prevent race conditions with InputManager's event tap.
    private func handleLastWindowClosed() {
        // If settings window is closed, restore background mode policy
        if settingsWindow == nil {
            // DOUBLE CHECK: Ensure no windows are actually visible/key
            if NSApp.windows.contains(where: { $0.isVisible && $0.canBecomeKey }) {
                Log.info("Window closed but others visible, skipping policy change")
                return
            }
            
            let hideFromDock = SettingsManager.shared.hideFromDock
            let policy: NSApplication.ActivationPolicy = hideFromDock ? .accessory : .regular
            
            // Delay policy change to ensure window closing is complete
            DispatchQueue.main.asyncAfter(deadline: .now() + 0.2) { [weak self] in
                guard let self = self, self.settingsWindow == nil else {
                    return
                }
                
                self.setActivationPolicy(policy)
                Log.info("Settings window closed. Policy set to: \(hideFromDock ? ".accessory" : ".regular")")
            }
        }
    }
    
    private func setActivationPolicy(_ policy: NSApplication.ActivationPolicy) {
        // Only change if different to avoid redundant layout triggers
        guard NSApp.activationPolicy() != policy else { return }

        // Delegate to coordinator to apply outside current layout pass
        ActivationPolicyCoordinator.shared.request(policy)
    }
    
    // MARK: - NSWindowDelegate
    
    /// Called BEFORE the window starts closing
    func windowShouldClose(_ sender: NSWindow) -> Bool {
        // Allow all windows to close normally
        return true
    }
    
    /// Called when a window is about to close
    func windowWillClose(_ notification: Notification) {
        guard let window = notification.object as? NSWindow else { return }
        
        // IMPORTANT: Must run on main thread to ensure thread safety
        guard Thread.isMainThread else {
            DispatchQueue.main.async { [weak self] in
                self?.windowWillClose(notification)
            }
            return
        }
        
        // Handle settings window close
        if window === settingsWindow {
            Log.info("✅ Settings window will close")
            settingsWindow = nil
            handleLastWindowClosed()
        }
    }
    
    // MARK: - Cleanup
    
    private func cleanup() {
        settingsWindow?.delegate = nil
        settingsWindow?.close()
        settingsWindow = nil
        
        Log.info("WindowManager cleaned up")
    }
}
