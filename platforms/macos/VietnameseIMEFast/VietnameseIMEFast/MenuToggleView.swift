//
//  MenuToggleView.swift
//  VietnameseIMEFast
//
//  Custom view for menu item with SwiftUI Toggle
//  Simple implementation following reference architecture
//

import Cocoa
import SwiftUI

class MenuToggleView: NSView {
    
    private var hostingView: NSHostingView<AnyView>?
    private let label: NSTextField
    private let onToggle: (Bool) -> Void
    private var currentState: Bool
    
    var isOn: Bool {
        get { currentState }
        set {
            if currentState != newValue {
                currentState = newValue
                recreateToggleView()
            }
        }
    }
    
    init(labelText: String, isOn: Bool, onToggle: @escaping (Bool) -> Void) {
        self.onToggle = onToggle
        self.currentState = isOn
        
        // Create label
        label = NSTextField(labelWithString: labelText)
        label.font = NSFont.systemFont(ofSize: 13, weight: .medium)
        label.textColor = .labelColor
        label.backgroundColor = .clear
        label.isBordered = false
        label.isEditable = false
        label.isSelectable = false
        
        super.init(frame: NSRect(x: 0, y: 0, width: 220, height: 32))
        
        setupView()
    }
    
    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }
    
    private func setupView() {
        // Configure view to prevent selection highlighting
        self.wantsLayer = true
        self.layer?.backgroundColor = NSColor.clear.cgColor
        
        // Layout label (left side)
        label.frame = NSRect(x: 16, y: 6, width: 140, height: 20)
        addSubview(label)
        
        // Create SwiftUI Toggle with NSHostingView (right side)
        recreateToggleView()
    }
    
    private func recreateToggleView() {
        // Remove old hosting view if exists
        hostingView?.removeFromSuperview()
        
        // Create SwiftUI Toggle (simple, like reference implementation)
        let toggleView = Toggle("", isOn: Binding(
            get: { [weak self] in self?.currentState ?? false },
            set: { [weak self] newValue in
                self?.currentState = newValue
                self?.onToggle(newValue)
            }
        ))
        .toggleStyle(.switch)
        .labelsHidden()
        .scaleEffect(0.8)
        
        // Create hosting view
        hostingView = NSHostingView(rootView: AnyView(toggleView))
        hostingView?.frame = NSRect(x: 162, y: 2, width: 50, height: 28)
        
        if let hostingView = hostingView {
            addSubview(hostingView)
        }
    }
    
    // Override to prevent selection highlighting
    override func draw(_ dirtyRect: NSRect) {
        // Draw clear background to prevent blue selection highlight
        NSColor.clear.setFill()
        dirtyRect.fill()
    }
    
    // Prevent the view from accepting first responder status
    override var acceptsFirstResponder: Bool {
        return false
    }
    
    // Override to prevent context menu
    override func menu(for event: NSEvent) -> NSMenu? {
        return nil
    }
    
    // Update toggle state programmatically
    func updateState(_ newState: Bool) {
        isOn = newState
    }
    
    // Update appearance for light/dark mode
    override func viewDidChangeEffectiveAppearance() {
        super.viewDidChangeEffectiveAppearance()
        label.textColor = .labelColor
        
        // Defer to next runloop to avoid layout recursion
        DispatchQueue.main.async { [weak self] in
            self?.recreateToggleView()
        }
    }
}