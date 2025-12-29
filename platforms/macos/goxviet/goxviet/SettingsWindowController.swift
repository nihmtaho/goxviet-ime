//
//  SettingsWindowController.swift
//  GoxViet
//
//  Manages the Settings window lifecycle and presentation
//  Memory-optimized: releases all SwiftUI resources when window closes
//

import Cocoa
import SwiftUI

final class SettingsWindowController: NSWindowController {
    
    // MARK: - Static State
    
    private static var current: SettingsWindowController?
    private static var isClosing = false
    
    // MARK: - Instance Properties
    
    private var hostingController: NSHostingController<AnyView>?
    
    // MARK: - Class Methods
    
    static func showSettings() {
        guard !isClosing else { return }
        
        if let existing = current, existing.window?.isVisible == true {
            existing.window?.makeKeyAndOrderFront(nil)
            NSApp.activate(ignoringOtherApps: true)
            return
        }
        
        releaseCurrentInstance()
        
        let controller = SettingsWindowController()
        current = controller
        
        NSApp.setActivationPolicy(.regular)
        
        controller.window?.center()
        controller.showWindow(nil)
        controller.window?.makeKeyAndOrderFront(nil)
        NSApp.activate(ignoringOtherApps: true)
    }
    
    static var isVisible: Bool {
        return current?.window?.isVisible ?? false
    }
    
    static func closeSettings() {
        current?.window?.close()
    }
    
    private static func releaseCurrentInstance() {
        guard let instance = current else { return }
        instance.releaseContent()
        current = nil
    }
    
    // MARK: - Initialization
    
    private init() {
        let window = NSWindow(
            contentRect: NSRect(x: 0, y: 0, width: 840, height: 580),
            styleMask: [.titled, .closable, .miniaturizable, .resizable, .fullSizeContentView],
            backing: .buffered,
            defer: true
        )
        
        window.title = "GoxViet Settings"
        window.titlebarAppearsTransparent = true
        window.titleVisibility = .hidden
        window.isMovableByWindowBackground = true
        window.isOpaque = false
        window.backgroundColor = .clear
        window.isReleasedWhenClosed = false
        window.level = .normal
        window.collectionBehavior = [.moveToActiveSpace]
        window.minSize = NSSize(width: 760, height: 520)
        window.maxSize = NSSize(width: 1200, height: 820)
        
        if #available(macOS 11.0, *) {
            window.toolbarStyle = .unified
        }
        
        super.init(window: window)
        window.delegate = self
        
        createContent()
    }
    
    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }
    
    deinit {
        releaseContent()
    }
    
    // MARK: - Content Management
    
    private func createContent() {
        let rootView = AnyView(SettingsRootView())
        let hosting = NSHostingController(rootView: rootView)
        hosting.view.wantsLayer = true
        hosting.view.layer?.backgroundColor = .clear
        
        hostingController = hosting
        window?.contentViewController = hosting
    }
    
    private func releaseContent() {
        window?.contentViewController = nil
        
        if let hosting = hostingController {
            hosting.rootView = AnyView(EmptyView())
            hosting.view.removeFromSuperview()
        }
        hostingController = nil
        
        window?.contentView?.subviews.forEach { $0.removeFromSuperview() }
        window?.contentView = nil
    }
}

// MARK: - NSWindowDelegate

extension SettingsWindowController: NSWindowDelegate {
    
    func windowWillClose(_ notification: Notification) {
        guard !SettingsWindowController.isClosing else { return }
        SettingsWindowController.isClosing = true
        
        NSApp.setActivationPolicy(.accessory)
        
        releaseContent()
        
        DispatchQueue.main.async {
            SettingsWindowController.current = nil
            SettingsWindowController.isClosing = false
        }
    }
}