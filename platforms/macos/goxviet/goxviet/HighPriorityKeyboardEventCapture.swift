//
//  HighPriorityKeyboardEventCapture.swift
//  GoxViet
//
//  Captures keyboard events with highest priority using CGEvent taps.
//  Ensures Control+Space and other system shortcuts can be captured.
//  Unlike NSEvent monitors, CGEvent taps intercept at the lowest level.
//

import Cocoa

/// Manages high-priority keyboard event capture using CGEvent taps.
/// This provides the highest possible priority and cannot be overridden by
/// system shortcuts or other applications.
class HighPriorityKeyboardEventCapture {
    private var eventTap: CFMachPort?
    private var runLoopSource: CFRunLoopSource?
    private let onKeyEvent: (NSEvent) -> Void
    private let onCancel: () -> Void
    
    init(onKeyEvent: @escaping (NSEvent) -> Void, onCancel: @escaping () -> Void) {
        self.onKeyEvent = onKeyEvent
        self.onCancel = onCancel
    }
    
    deinit {
        stop()
    }
    
    /// Start capturing keyboard events with highest priority
    func start() -> Bool {
        guard eventTap == nil else { return true }
        
        // Check if we have accessibility permission
        guard isAccessibilityEnabled() else {
            DispatchQueue.main.async {
                self.promptForAccessibilityPermission()
            }
            return false
        }
        
        // Create the event tap at HID level (highest priority)
        // kCGHIDEventTap: Hardware input device level (highest priority - cannot be overridden)
        let eventMask = CGEventMask(
            (1 << CGEventType.keyDown.rawValue) |
            (1 << CGEventType.flagsChanged.rawValue)
        )
        
        guard let tap = CGEvent.tapCreate(
            tap: .cghidEventTap,                // Highest priority - cannot be overridden by system
            place: .headInsertEventTap,         // Insert at head of queue
            options: .defaultTap,
            eventsOfInterest: eventMask,
            callback: Self.eventTapCallback,
            userInfo: Unmanaged.passUnretained(self).toOpaque()
        ) else {
            print("[HighPriorityKeyboardEventCapture] Failed to create event tap")
            return false
        }
        
        let runLoopSource = CFMachPortCreateRunLoopSource(
            kCFAllocatorDefault,
            tap,
            0
        )
        
        CFRunLoopAddSource(
            CFRunLoopGetCurrent(),
            runLoopSource,
            .commonModes
        )
        
        CGEvent.tapEnable(tap: tap, enable: true)
        
        self.eventTap = tap
        self.runLoopSource = runLoopSource
        
        print("[HighPriorityKeyboardEventCapture] Started capturing events at HID level")
        return true
    }
    
    /// Stop capturing keyboard events
    func stop() {
        if let tap = eventTap {
            CGEvent.tapEnable(tap: tap, enable: false)
            if let runLoopSource = runLoopSource {
                CFRunLoopRemoveSource(
                    CFRunLoopGetCurrent(),
                    runLoopSource,
                    .commonModes
                )
            }
            self.eventTap = nil
            self.runLoopSource = nil
        }
        print("[HighPriorityKeyboardEventCapture] Stopped capturing events")
    }
    
    // MARK: - Accessibility Permission
    
    private func isAccessibilityEnabled() -> Bool {
        let options = [
            kAXTrustedCheckOptionPrompt.takeRetainedValue() as String: true
        ] as [String: Any]
        
        return AXIsProcessTrustedWithOptions(options as CFDictionary)
    }
    
    private func promptForAccessibilityPermission() {
        let alert = NSAlert()
        alert.messageText = "Accessibility Permission Required"
        alert.informativeText = "GoxViet needs accessibility permission to capture keyboard shortcuts including system shortcuts like Control+Space.\n\nPlease click 'Open System Settings' to grant permission."
        alert.addButton(withTitle: "Open System Settings")
        alert.addButton(withTitle: "Cancel")
        alert.alertStyle = .warning
        
        if alert.runModal() == .alertFirstButtonReturn {
            // Open System Settings - Security & Privacy - Accessibility
            if let url = URL(string: "x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility") {
                NSWorkspace.shared.open(url)
            }
        }
    }
    
    // MARK: - Event Tap Callback
    
    private static let eventTapCallback: CGEventTapCallBack = { proxy, type, event, userInfo in
        // If userInfo is missing, allow the event to proceed untouched
        guard let userInfo = userInfo else { return Unmanaged.passUnretained(event) }
        
        let capture = Unmanaged<HighPriorityKeyboardEventCapture>.fromOpaque(userInfo)
            .takeUnretainedValue()
        
        // Convert CGEvent to NSEvent
        guard let nsEvent = NSEvent(cgEvent: event) else {
            return Unmanaged.passUnretained(event)
        }
        
        // Handle the event
        capture.handleKeyboardEvent(nsEvent)
        
        // Return event to allow normal processing while still capturing
        return Unmanaged.passUnretained(event)
    }
    
    private func handleKeyboardEvent(_ event: NSEvent) {
        // Ignore non-keyboard events
        guard event.type == .keyDown || event.type == .flagsChanged else {
            return
        }
        
        // ESC key cancels recording
        if event.type == .keyDown && event.keyCode == 53 {
            DispatchQueue.main.async {
                self.onCancel()
            }
            return
        }
        
        // Forward to handler
        DispatchQueue.main.async {
            self.onKeyEvent(event)
        }
    }
}
