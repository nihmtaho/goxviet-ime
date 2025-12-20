//
//  InputManager.swift
//  VietnameseIMEFast
//
//  Updated to use new Rust core API (ime_init, ime_key, etc.)
//

import Cocoa
import ApplicationServices

// MARK: - Input Manager

class InputManager {
    static let shared = InputManager()
    
    private var eventTap: CFMachPort?
    private var runLoopSource: CFRunLoopSource?
    private var bridge: RustBridge
    
    // IME state
    private var isEnabled: Bool = true
    
    // Shortcut configuration
    private var currentShortcut: KeyboardShortcut
    
    // Previous key for detecting double-tap shortcuts
    private var previousKeyCode: UInt16?
    private var previousKeyTimestamp: TimeInterval = 0
    
    private init() {
        self.bridge = RustBridge()
        self.bridge.initialize()
        
        // Load shortcut configuration
        self.currentShortcut = KeyboardShortcut.load()
        Log.info("Toggle shortcut loaded: \(currentShortcut.displayString)")
        
        // Setup observers for configuration changes
        setupObservers()
    }
    
    deinit {
        // Rust engine uses global singleton, no need to destroy
    }
    
    // MARK: - Lifecycle
    
    func start() {
        let options = [kAXTrustedCheckOptionPrompt.takeUnretainedValue() as String: true]
        let accessEnabled = AXIsProcessTrustedWithOptions(options as CFDictionary)
        
        if !accessEnabled {
            Log.info("Accessibility permission not granted")
            KeyboardHookManager.shared.showAccessibilityAlert()
            return
        }
        
        let eventMask = (1 << CGEventType.keyDown.rawValue) | 
                        (1 << CGEventType.keyUp.rawValue) |
                        (1 << CGEventType.flagsChanged.rawValue)
        
        guard let tap = CGEvent.tapCreate(
            tap: .cghidEventTap,
            place: .headInsertEventTap,
            options: .defaultTap,
            eventsOfInterest: CGEventMask(eventMask),
            callback: { (proxy, type, event, refcon) -> Unmanaged<CGEvent>? in
                if let observer = refcon {
                    let mySelf = Unmanaged<InputManager>.fromOpaque(observer).takeUnretainedValue()
                    return mySelf.handleEvent(event: event, type: type, proxy: proxy)
                }
                return Unmanaged.passUnretained(event)
            },
            userInfo: Unmanaged.passUnretained(self).toOpaque()
        ) else {
            Log.info("Failed to create event tap")
            return
        }
        
        self.eventTap = tap
        self.runLoopSource = CFMachPortCreateRunLoopSource(kCFAllocatorDefault, tap, 0)
        CFRunLoopAddSource(CFRunLoopGetCurrent(), self.runLoopSource, .commonModes)
        CGEvent.tapEnable(tap: tap, enable: true)
        
        Log.info("InputManager started")
        
        // Start per-app mode manager
        PerAppModeManager.shared.start()
    }
    
    func stop() {
        if let runLoopSource = self.runLoopSource {
            CFRunLoopRemoveSource(CFRunLoopGetCurrent(), runLoopSource, .commonModes)
            self.runLoopSource = nil
        }
        
        if let eventTap = self.eventTap {
            CGEvent.tapEnable(tap: eventTap, enable: false)
            self.eventTap = nil
        }
        
        PerAppModeManager.shared.stop()
        Log.info("InputManager stopped")
    }
    
    // MARK: - Configuration
    
    private func setupObservers() {
        NotificationCenter.default.addObserver(
            forName: .toggleVietnamese,
            object: nil,
            queue: .main
        ) { [weak self] notification in
            if let enabled = notification.object as? Bool {
                self?.setEnabled(enabled)
            } else {
                self?.toggleEnabled()
            }
        }
        
        NotificationCenter.default.addObserver(
            forName: .shortcutChanged,
            object: nil,
            queue: .main
        ) { [weak self] notification in
            // Reload toggle shortcut configuration
            if let shortcut = notification.object as? KeyboardShortcut {
                self?.currentShortcut = shortcut
                Log.info("Toggle shortcut updated: \(shortcut.displayString)")
            } else {
                self?.currentShortcut = KeyboardShortcut.load()
                Log.info("Toggle shortcut reloaded: \(self?.currentShortcut.displayString ?? "unknown")")
            }
            
            // Also reload text expansion shortcuts
            self?.reloadShortcuts()
        }
    }
    
    func setEnabled(_ enabled: Bool) {
        isEnabled = enabled
        bridge.setEnabled(enabled)
        
        // Clear buffer when toggling
        ime_clear()
        
        Log.info("IME \(enabled ? "enabled" : "disabled")")
        
        // Update per-app state
        PerAppModeManager.shared.setStateForCurrentApp(enabled)
        
        // Post notification for UI update
        NotificationCenter.default.post(name: .updateStateChanged, object: enabled)
    }
    
    func toggleEnabled() {
        setEnabled(!isEnabled)
    }
    
    func setInputMethod(_ method: Int) {
        bridge.setMethod(method)
        ime_method(UInt8(method))
    }
    
    func setModernToneStyle(_ modern: Bool) {
        bridge.setModernTone(modern)
        ime_modern(modern)
    }
    
    func reloadShortcuts() {
        // Load shortcuts from UserDefaults or other storage
        // For now, just clear
        ime_clear_shortcuts()
        bridge.clearShortcuts()
        Log.info("Shortcuts reloaded")
    }
    
    // MARK: - Event Handling
    
    private func handleEvent(event: CGEvent, type: CGEventType, proxy: CGEventTapProxy) -> Unmanaged<CGEvent>? {
        // 1. Ignore our own injected events
        let marker = event.getIntegerValueField(.eventSourceUserData)
        if marker == 0x564E5F494D45 { // "VN_IME"
            return Unmanaged.passUnretained(event)
        }
        
        // 2. Handle flags changed (for modifier-only shortcuts)
        if type == .flagsChanged {
            return handleFlagsChanged(event: event, proxy: proxy)
        }
        
        // 3. Only process key down events
        guard type == .keyDown else {
            return Unmanaged.passUnretained(event)
        }
        
        let keyCode = UInt16(event.getIntegerValueField(.keyboardEventKeycode))
        let flags = event.flags
        
        // 4. Check for toggle shortcut (default: Control+Space)
        if currentShortcut.matches(keyCode: keyCode, flags: flags) {
            toggleEnabled()
            Log.info("Toggle shortcut triggered: \(currentShortcut.displayString)")
            return nil // Swallow event
        }
        
        // 5. If IME is disabled, pass through
        guard isEnabled else {
            return Unmanaged.passUnretained(event)
        }
        
        // 6. Ignore events with command/control/option modifiers (except Shift)
        if flags.contains(.maskCommand) || flags.contains(.maskControl) || flags.contains(.maskAlternate) {
            // Clear buffer on modifier shortcuts
            ime_clear()
            return Unmanaged.passUnretained(event)
        }
        
        // 7. Handle special keys
        if handleSpecialKey(keyCode: keyCode, flags: flags, event: event, proxy: proxy) {
            return nil // Special key was handled
        }
        
        // 8. Process with Rust engine
        return processKeyWithEngine(keyCode: keyCode, flags: flags, proxy: proxy, event: event)
    }
    
    private func handleFlagsChanged(event: CGEvent, proxy: CGEventTapProxy) -> Unmanaged<CGEvent>? {
        // This can be used for modifier-only shortcuts (e.g., double-tap Shift)
        // For now, just pass through
        return Unmanaged.passUnretained(event)
    }
    
    private func handleSpecialKey(keyCode: UInt16, flags: CGEventFlags, event: CGEvent, proxy: CGEventTapProxy) -> Bool {
        // Handle ESC key for word restoration
        if keyCode == KeyCode.escape {
            // ESC restore is handled internally by Rust engine
            let result = ime_key(keyCode, false, false)
            if let r = result {
                defer { ime_free(r) }
                if r.pointee.action == 2 { // Restore action
                    let backspaceCount = Int(r.pointee.backspace)
                    let chars = extractChars(from: r.pointee)
                    let (method, delays) = detectMethod()
                    TextInjector.shared.injectSync(
                        bs: backspaceCount,
                        text: String(chars),
                        method: method,
                        delays: delays,
                        proxy: proxy
                    )
                    return true
                }
            }
        }
        
        // Handle navigation keys (clear composition and pass through)
        let navigationKeys: Set<UInt16> = [
            36,  // Return
            76,  // Enter
            48,  // Tab
            123, // Left arrow
            124, // Right arrow
            125, // Down arrow
            126  // Up arrow
        ]
        
        if navigationKeys.contains(keyCode) {
            ime_clear()
            return false // Don't swallow, let system handle
        }
        
        // Backspace is handled in processKeyWithEngine
        // No special treatment needed here
        
        return false
    }
    
    private func processKeyWithEngine(keyCode: UInt16, flags: CGEventFlags, proxy: CGEventTapProxy, event: CGEvent) -> Unmanaged<CGEvent>? {
        let caps = flags.contains(.maskAlphaShift)
        let ctrl = flags.contains(.maskCommand) || flags.contains(.maskControl) || flags.contains(.maskAlternate)
        
        Log.key(keyCode, "Processing")
        
        // Call Rust engine
        let result = ime_key(keyCode, caps, ctrl)
        
        guard let r = result else {
            Log.skip()
            return Unmanaged.passUnretained(event) // Engine not initialized, pass through
        }
        
        defer { ime_free(r) }
        
        // Check action
        if r.pointee.action == 0 { // None - pass through original event
            // Engine is not transforming this key (e.g., arrow keys, non-Vietnamese input)
            // Just pass through and let system handle naturally
            Log.skip()
            return Unmanaged.passUnretained(event)
        }
        
        if r.pointee.action == 1 { // Send - replace text
            let backspaceCount = Int(r.pointee.backspace)
            let chars = extractChars(from: r.pointee)
            
            if chars.isEmpty && backspaceCount == 0 {
                Log.skip()
                return Unmanaged.passUnretained(event)
            }
            
            Log.transform(backspaceCount, String(chars))
            
            // Inject replacement text using smart injection
            let (method, delays) = detectMethod()
            TextInjector.shared.injectSync(
                bs: backspaceCount,
                text: String(chars),
                method: method,
                delays: delays,
                proxy: proxy
            )
            
            // Swallow the original event
            return nil
        }
        
        if r.pointee.action == 2 { // Restore - used by ESC key
            let backspaceCount = Int(r.pointee.backspace)
            let chars = extractChars(from: r.pointee)
            
            Log.info("Restore: bs=\(backspaceCount) text=\(String(chars))")
            
            let (method, delays) = detectMethod()
            TextInjector.shared.injectSync(
                bs: backspaceCount,
                text: String(chars),
                method: method,
                delays: delays,
                proxy: proxy
            )
            
            return nil
        }
        
        // Unknown action - pass through
        Log.skip()
        return Unmanaged.passUnretained(event)
    }
    
    private func getCharFromEvent(event: CGEvent, keyCode: UInt16, caps: Bool) -> Character? {
        // Try to get the character from the event
        var length = 0
        event.keyboardGetUnicodeString(maxStringLength: 1, actualStringLength: &length, unicodeString: nil)
        
        if length > 0 {
            var chars = [UniChar](repeating: 0, count: length)
            event.keyboardGetUnicodeString(maxStringLength: length, actualStringLength: &length, unicodeString: &chars)
            if let string = String(utf16CodeUnits: chars, count: length).first {
                return string
            }
        }
        
        // Fallback: map keycode to character
        return keycodeToChar(keyCode: keyCode, caps: caps)
    }
    
    private func keycodeToChar(keyCode: UInt16, caps: Bool) -> Character? {
        let lowerMap: [UInt16: Character] = [
            0: "a", 1: "s", 2: "d", 3: "f", 4: "h", 5: "g", 6: "z", 7: "x", 8: "c", 9: "v",
            11: "b", 12: "q", 13: "w", 14: "e", 15: "r", 16: "y", 17: "t",
            31: "o", 32: "u", 34: "i", 35: "p", 37: "l", 38: "j", 40: "k", 45: "n", 46: "m",
            18: "1", 19: "2", 20: "3", 21: "4", 23: "5", 22: "6", 26: "7", 28: "8", 25: "9", 29: "0"
        ]
        
        if let char = lowerMap[keyCode] {
            return caps ? Character(char.uppercased()) : char
        }
        
        return nil
    }
    
    private func extractChars(from result: ImeResult) -> [Character] {
        var chars: [Character] = []
        let count = Int(result.count)
        
        // Access tuple elements using Mirror for reflection
        let mirror = Mirror(reflecting: result.chars)
        var i = 0
        for child in mirror.children {
            if i >= count { break }
            if let codepoint = child.value as? UInt32,
               let scalar = UnicodeScalar(codepoint) {
                chars.append(Character(scalar))
            }
            i += 1
        }
        
        return chars
    }
}

// MARK: - Public API Extensions

extension InputManager {
    func getCurrentState() -> Bool {
        return isEnabled
    }
    
    func clearComposition() {
        ime_clear()
    }
    
    func getCurrentShortcut() -> KeyboardShortcut {
        return currentShortcut
    }
    
    func setShortcut(_ shortcut: KeyboardShortcut) {
        currentShortcut = shortcut
        shortcut.save()
    }
}