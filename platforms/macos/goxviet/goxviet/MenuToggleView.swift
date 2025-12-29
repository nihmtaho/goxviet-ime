//
//  MenuToggleView.swift
//  GoxViet
//
//  Custom view for menu item with SwiftUI Toggle
//  Memory-optimized: proper cleanup of NSHostingView resources
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
                updateToggleView()
            }
        }
    }
    
    init(labelText: String, isOn: Bool, onToggle: @escaping (Bool) -> Void) {
        self.onToggle = onToggle
        self.currentState = isOn
        
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
    
    deinit {
        cleanup()
    }
    
    private func setupView() {
        wantsLayer = true
        layer?.backgroundColor = NSColor.clear.cgColor
        
        label.frame = NSRect(x: 16, y: 6, width: 140, height: 20)
        addSubview(label)
        
        createToggleView()
    }
    
    private func createToggleView() {
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
        
        let hosting = NSHostingView(rootView: AnyView(toggleView))
        hosting.frame = NSRect(x: 162, y: 2, width: 50, height: 28)
        
        hostingView = hosting
        addSubview(hosting)
    }
    
    private func updateToggleView() {
        releaseHostingView()
        createToggleView()
    }
    
    private func releaseHostingView() {
        hostingView?.removeFromSuperview()
        hostingView = nil
    }
    
    func cleanup() {
        releaseHostingView()
    }
    
    override func draw(_ dirtyRect: NSRect) {
        NSColor.clear.setFill()
        dirtyRect.fill()
    }
    
    override var acceptsFirstResponder: Bool {
        return false
    }
    
    override func menu(for event: NSEvent) -> NSMenu? {
        return nil
    }
    
    func updateState(_ newState: Bool) {
        isOn = newState
    }
    
    override func viewDidChangeEffectiveAppearance() {
        super.viewDidChangeEffectiveAppearance()
        label.textColor = .labelColor
        
        DispatchQueue.main.async { [weak self] in
            self?.updateToggleView()
        }
    }
}