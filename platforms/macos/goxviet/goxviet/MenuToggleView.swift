//
//  MenuToggleView.swift
//  GoxViet
//
//  Custom view for menu item with SwiftUI Toggle
//  Memory-optimized: proper cleanup of NSHostingView resources
//
//  Fixed: Layout recursion warning by using ObservableObject instead of replacing rootView
//

import Cocoa
import SwiftUI
import Combine

class MenuToggleView: NSView {
    
    // ViewModel to handle state changes without rebuilding view hierarchy
    class ViewModel: ObservableObject {
        @Published var isOn: Bool
        var onToggle: ((Bool) -> Void)?
        
        init(isOn: Bool) {
            self.isOn = isOn
        }
    }
    
    private var hostingView: NSHostingView<AnyView>?
    private let label: NSTextField
    private let viewModel: ViewModel
    
    private var currentState: Bool
    
    var isOn: Bool {
        get { currentState }
        set {
            if currentState != newValue {
                currentState = newValue
                // Update ViewModel instead of replacing RootView
                viewModel.isOn = newValue
            }
        }
    }
    
    init(labelText: String, isOn: Bool, onToggle: @escaping (Bool) -> Void) {
        self.currentState = isOn
        self.viewModel = ViewModel(isOn: isOn)
        self.viewModel.onToggle = onToggle
        
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
    
    // Internal SwiftUI View that observes the ViewModel
    struct ToggleWrapper: View {
        @ObservedObject var viewModel: ViewModel
        
        var body: some View {
            Toggle("", isOn: Binding(
                get: { viewModel.isOn },
                set: { newValue in
                    viewModel.isOn = newValue
                    viewModel.onToggle?(newValue)
                }
            ))
            .toggleStyle(.switch)
            .labelsHidden()
            .scaleEffect(0.8)
        }
    }
    
    private func createToggleView() {
        let rootView = ToggleWrapper(viewModel: viewModel)
        let hosting = NSHostingView(rootView: AnyView(rootView))
        hosting.frame = NSRect(x: 162, y: 2, width: 50, height: 28)
        
        hostingView = hosting
        addSubview(hosting)
    }
    
    private func releaseHostingView() {
        hostingView?.removeFromSuperview()
        hostingView = nil
    }
    
    func cleanup() {
        viewModel.onToggle = nil
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
        // No need to rebuild view here anymore
    }
}