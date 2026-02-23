//
//  ShortcutRecorder.swift
//  GoxViet
//
//  Captures a single keyboard shortcut (including Fn) for toggle updates.
//  Uses high-priority CGEvent taps to ensure Control+Space and system shortcuts
//  can be captured without being overridden by the system or other apps.
//

import SwiftUI
import AppKit

struct ShortcutRecorder: NSViewRepresentable {
    @Binding var isRecording: Bool
    let onComplete: (KeyboardShortcut) -> Void
    let onCancel: () -> Void

    func makeCoordinator() -> Coordinator {
        Coordinator(self)
    }

    func makeNSView(context: Context) -> NSView {
        NSView(frame: .zero)
    }

    func updateNSView(_ nsView: NSView, context: Context) {
        if isRecording {
            context.coordinator.start()
        } else {
            context.coordinator.stop()
        }
    }

    final class Coordinator {
        private var eventCapture: HighPriorityKeyboardEventCapture?
        private var pendingModifierWorkItem: DispatchWorkItem?
        private var pendingModifiers: CGEventFlags = []
        private let parent: ShortcutRecorder

        init(_ parent: ShortcutRecorder) {
            self.parent = parent
        }

        func start() {
            guard eventCapture == nil else { return }

            eventCapture = HighPriorityKeyboardEventCapture(
                onKeyEvent: { [weak self] event in
                    self?.handleKeyboardEvent(event)
                },
                onCancel: { [weak self] in
                    self?.finish(cancel: true, shortcut: nil)
                }
            )
            
            if !eventCapture!.start() {
                print("[ShortcutRecorder] Failed to start high-priority event capture")
                finish(cancel: true, shortcut: nil)
            }
        }

        func stop() {
            pendingModifierWorkItem?.cancel()
            pendingModifierWorkItem = nil
            pendingModifiers = []
            eventCapture?.stop()
            eventCapture = nil
        }

        private func handleKeyboardEvent(_ event: NSEvent) {
            switch event.type {
            case .flagsChanged:
                handleFlagsChanged(event)
            case .keyDown:
                handleKeyDown(event)
            default:
                break
            }
        }

        private func handleFlagsChanged(_ event: NSEvent) {
            // Update current modifiers
            let cgFlags = event.cgEvent?.flags ?? CGEventFlags(rawValue: UInt64(event.modifierFlags.rawValue))
            pendingModifiers = cgFlags.intersection(KeyboardShortcut.allowedFlags)

            // Cancel any previous pending completion
            pendingModifierWorkItem?.cancel()

            // If no modifiers, do not schedule capture
            guard !pendingModifiers.isEmpty else { return }

            // Schedule a short delay to allow keyDown to arrive; if no keyDown, treat as modifier-only
            let workItem = DispatchWorkItem { [weak self] in
                guard let self = self else { return }
                if let shortcut = KeyboardShortcut.modifierOnly(from: self.pendingModifiers) {
                    self.finish(cancel: false, shortcut: shortcut)
                }
            }
            pendingModifierWorkItem = workItem
            DispatchQueue.main.asyncAfter(deadline: .now() + 0.35, execute: workItem)
        }

        private func handleKeyDown(_ event: NSEvent) {
            // Any keyDown cancels pending modifier-only capture
            pendingModifierWorkItem?.cancel()
            pendingModifierWorkItem = nil
            pendingModifiers = []

            if let shortcut = KeyboardShortcut.from(event: event) {
                self.finish(cancel: false, shortcut: shortcut)
            } else {
                print("[ShortcutRecorder] Invalid key combination, waiting for valid input")
            }
        }

        private func finish(cancel: Bool, shortcut: KeyboardShortcut?) {
            parent.isRecording = false
            if cancel {
                parent.onCancel()
            } else if let shortcut {
                parent.onComplete(shortcut)
            }
            stop()
        }
    }
}
