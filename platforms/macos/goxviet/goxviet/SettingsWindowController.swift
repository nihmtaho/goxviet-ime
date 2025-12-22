//
//  SettingsWindowController.swift
//  GoxViet
//
//  Manages the Settings window lifecycle and presentation
//

import Cocoa
import SwiftUI

class SettingsWindowController: NSWindowController {
    
    // MARK: - Singleton
    
    static let shared = SettingsWindowController()
    
    // MARK: - Initialization
    
    private init() {
        // Create the SwiftUI settings view
        let settingsView = SettingsRootView()
        
        // Wrap in NSHostingController
        let hostingController = NSHostingController(rootView: settingsView)
        
        // Create window with modern SwiftUI styling
        let window = NSWindow(contentViewController: hostingController)
        window.title = "GoxViet Settings"
        window.styleMask = [.titled, .closable, .miniaturizable, .resizable, .fullSizeContentView]
        window.titlebarAppearsTransparent = true
        window.titleVisibility = .hidden
        window.isMovableByWindowBackground = true
        window.isOpaque = false
        window.backgroundColor = .clear
        window.setContentSize(NSSize(width: 840, height: 580))
        window.center()
        
        // Configure window behavior
        window.isReleasedWhenClosed = false
        window.level = .normal
        window.collectionBehavior = [.moveToActiveSpace]
        
        // Minimum size (allow resizing for sidebar)
        window.minSize = NSSize(width: 760, height: 520)
        window.maxSize = NSSize(width: 1200, height: 820)
        
        // Modern appearance with unified toolbar
        if #available(macOS 11.0, *) {
            window.toolbarStyle = .unified
        }
        
        super.init(window: window)
        window.delegate = self
    }
    
    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }
    
    // MARK: - Public Methods
    
    /// Show the settings window and bring it to front
    func show() {
        guard let window = window else { return }
        
        // If window is already visible, just bring to front
        if window.isVisible {
            window.makeKeyAndOrderFront(nil)
            NSApp.activate(ignoringOtherApps: true)
            return
        }
        
        // Center window before showing
        window.center()
        
        // Show window
        showWindow(nil)
        window.makeKeyAndOrderFront(nil)
        NSApp.activate(ignoringOtherApps: true)
        
        Log.info("Settings window opened")
    }
    
    /// Close the settings window
    func closeWindow() {
        window?.close()
        Log.info("Settings window closed")
    }
    
    /// Check if settings window is currently visible
    var isVisible: Bool {
        return window?.isVisible ?? false
    }
}

// MARK: - NSWindowDelegate

extension SettingsWindowController: NSWindowDelegate {
    
    func windowWillClose(_ notification: Notification) {
        Log.info("Settings window will close")
    }
    
    func windowDidBecomeKey(_ notification: Notification) {
        Log.info("Settings window became key")
    }
    
    func windowDidResignKey(_ notification: Notification) {
        Log.info("Settings window resigned key")
    }
}
