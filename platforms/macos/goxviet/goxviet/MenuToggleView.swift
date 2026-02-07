//
//  MenuToggleView.swift
//  GoxViet
//
//  Simple native NSSwitch for menu items
//

import Cocoa

/// Simple toggle view using native NSSwitch for menu items
class MenuToggleView: NSView {
    
    private let toggleSwitch: NSSwitch
    private let label: NSTextField
    private var onToggle: ((Bool) -> Void)?
    
    var isOn: Bool {
        get { toggleSwitch.state == .on }
        set { toggleSwitch.state = newValue ? .on : .off }
    }
    
    init(labelText: String, isOn: Bool, onToggle: @escaping (Bool) -> Void) {
        self.onToggle = onToggle
        
        // Create label
        self.label = NSTextField(labelWithString: labelText)
        self.label.font = NSFont.systemFont(ofSize: 13)
        self.label.textColor = .labelColor
        self.label.backgroundColor = .clear
        self.label.isBordered = false
        self.label.isEditable = false
        self.label.isSelectable = false
        self.label.frame = NSRect(x: 14, y: 6, width: 145, height: 20)
        
        // Create switch
        self.toggleSwitch = NSSwitch()
        self.toggleSwitch.state = isOn ? .on : .off
        self.toggleSwitch.frame = NSRect(x: 170, y: 4, width: 40, height: 24)
        
        super.init(frame: NSRect(x: 0, y: 0, width: 220, height: 32))
        
        setupView()
    }
    
    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }
    
    private func setupView() {
        wantsLayer = true
        layer?.backgroundColor = NSColor.clear.cgColor
        
        // Add subviews
        addSubview(label)
        addSubview(toggleSwitch)
        
        // Setup target/action for switch
        toggleSwitch.target = self
        toggleSwitch.action = #selector(switchChanged(_:))
    }
    
    @objc private func switchChanged(_ sender: NSSwitch) {
        let newState = sender.state == .on
        onToggle?(newState)
    }
    
    func updateState(_ newState: Bool) {
        toggleSwitch.state = newState ? .on : .off
    }
    
    func cleanup() {
        toggleSwitch.target = nil
        toggleSwitch.action = nil
        onToggle = nil
    }
}
