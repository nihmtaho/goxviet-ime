//
//  InputManager.swift
//  GoxViet
//
//  Updated to use new Rust core API (ime_init, ime_key, etc.)
//

import Cocoa
import ApplicationServices

// MARK: - Break Key Detection

// OPTIMIZATION: Static Sets avoid reallocating on every call
private let standardBreakKeys: Set<CGKeyCode> = [
    31, 48, 36, 76, 53,  // space, tab, return, enter, esc
    123, 124, 125, 126,  // left, right, down, up arrows
    47, 43, 44, 41, 39, 33, 30, 42, 24, 27, 50  // punctuation: . , / ; ' [ ] \ = - `
]

private let  numberKeys: Set<CGKeyCode> = [29, 18, 19, 20, 21, 23, 22, 26, 28, 25]

/// Check if key is a break key (space, punctuation, arrows, etc.)
/// When shift=true, also treat number keys as break (they produce !@#$%^&*())
@inline(__always)
private func isBreakKey(_ keyCode: CGKeyCode, shift: Bool) -> Bool {
    if standardBreakKeys.contains(keyCode) { return true }
    
    // Shifted number keys produce symbols: !@#$%^&*()
    return shift && numberKeys.contains(keyCode)
}

// MARK: - Input Manager

class InputManager: LifecycleManaged {
    static let shared = InputManager()
    
    // OPTIMIZATION: String pool for common Vietnamese characters (reduces allocations)
    // Reduces 64B malloc overhead by reusing String objects for frequent chars
    private static let commonCharPool: [Character: String] = {
        var pool: [Character: String] = [:]
        let chars = "aăâeêioôơuưyáàảãạắằẳẵặấầẩẫậéèẻẽẹếềểễệíìỉĩịóòỏõọốồổỗộớờởỡợúùủũụứừửữựýỳỷỹỵ" +
                    "AĂÂEÊIOÔƠUƯYÁÀẢÃẠẮẰẲẴẶẤẦẨẪẬÉÈẺẼẸẾỀỂỄỆÍÌỈĨỊÓÒỎÕỌỐỒỔỖỘỚỜỞỠỢÚÙỦŨỤỨỪỬỮỰÝỲỶỸỴ" +
                    "bcdđghklmnpqrstvxzBCDĐGHKLMNPQRSTVXZ0123456789"
        for char in chars {
            pool[char] = String(char)
        }
        return pool
    }()
    
    private var eventTap: CFMachPort?
    private var runLoopSource: CFRunLoopSource?
    private var bridge: RustBridge
    private var mouseMonitor: Any?  // NSEvent monitor for mouse clicks
    
    // Running state for LifecycleManaged protocol
    private(set) var isRunning: Bool = false
    
    // Shortcut configuration
    private var currentShortcut: KeyboardShortcut
    
    // Previous key for detecting double-tap shortcuts
    private var previousKeyCode: UInt16?
    private var previousKeyTimestamp: TimeInterval = 0
    
    private init() {
        self.bridge = RustBridge.shared
        self.bridge.initialize()
        
        // Load shortcut configuration
        self.currentShortcut = KeyboardShortcut.load()
        Log.info("Toggle shortcut loaded: \(currentShortcut.displayString)")
        
        // Load and apply saved settings from SettingsManager
        loadSavedSettings()
        
        // Sync shortcuts to engine after initialization
        SettingsManager.shared.syncShortcutsToEngine()
        
        // Setup observers for configuration changes
        setupObservers()
    }
    
    deinit {
        stop()
        Log.info("InputManager deinitialized")
    }
    
    private func loadSavedSettings() {
        let settings = SettingsManager.shared
        
        // Apply saved input method
        ime_method(UInt8(settings.inputMethod))
        Log.info("Loaded input method: \(settings.inputMethod == 0 ? "Telex" : "VNI")")
        
        // Apply saved tone style
        ime_modern(settings.modernToneStyle)
        Log.info("Loaded tone style: \(settings.modernToneStyle ? "Modern" : "Traditional")")
        
        // Apply saved ESC restore setting
        ime_esc_restore(settings.escRestoreEnabled)
        Log.info("Loaded ESC restore: \(settings.escRestoreEnabled ? "enabled" : "disabled")")
        
        // Apply saved free tone setting
        ime_free_tone(settings.freeToneEnabled)
        Log.info("Loaded free tone: \(settings.freeToneEnabled ? "enabled" : "disabled")")
        
        // Apply saved instant restore setting
        ime_instant_restore(settings.instantRestoreEnabled)
        Log.info("Loaded instant restore: \(settings.instantRestoreEnabled ? "enabled" : "disabled")")
        
        // Apply saved text expansion enabled setting
        RustBridge.shared.setShortcutsEnabled(settings.textExpansionEnabled)
        Log.info("Loaded text expansion: \(settings.textExpansionEnabled ? "enabled" : "disabled")")
        
        // Set initial enabled state (will be overridden by per-app mode if enabled)
        ime_enabled(settings.isEnabled)
        Log.info("Initial Gõ Việt input state: \(settings.isEnabled ? "enabled" : "disabled")")
    }
    
    // MARK: - Lifecycle
    
    func start() {
        guard !isRunning else {
            Log.info("InputManager already running")
            return
        }
        
        // Note: Accessibility permission is checked in AppDelegate before calling start()
        // No need to check again here to avoid priority inversion issues
        
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
        
        isRunning = true
        Log.info("InputManager started")
        
        // Start NSEvent global monitor for mouse events
        startMouseMonitor()
        
        // Start per-app mode manager
        PerAppModeManagerEnhanced.shared.start()
        
        // Start input source monitor (auto-disable for non-Latin keyboards)
        InputSourceMonitor.shared.start()
    }
    
    func stop() {
        guard isRunning else {
            Log.info("InputManager already stopped")
            return
        }
        
        if let runLoopSource = self.runLoopSource {
            CFRunLoopRemoveSource(CFRunLoopGetCurrent(), runLoopSource, .commonModes)
            self.runLoopSource = nil
        }
        
        if let eventTap = self.eventTap {
            CGEvent.tapEnable(tap: eventTap, enable: false)
            self.eventTap = nil
        }
        
        // Unregister all observers via ResourceManager
        ResourceManager.shared.unregister(observerIdentifier: "InputManager.toggleObserver")
        ResourceManager.shared.unregister(observerIdentifier: "InputManager.shortcutObserver")
        
        // Stop mouse monitor
        if let monitor = mouseMonitor {
            NSEvent.removeMonitor(monitor)
            mouseMonitor = nil
        }
        
        PerAppModeManagerEnhanced.shared.stop()
        InputSourceMonitor.shared.stop()
        
        isRunning = false
        Log.info("InputManager stopped")
    }
    
    // MARK: - Mouse Monitoring
    
    /// Start NSEvent global monitor for mouse events
    /// This is more reliable than CGEventTap for detecting mouse clicks
    private func startMouseMonitor() {
        // Monitor both mouseDown and mouseUp to catch clicks and drag-selects
        mouseMonitor = NSEvent.addGlobalMonitorForEvents(matching: [.leftMouseDown, .leftMouseUp]) { [weak self] _ in
            // Clear all buffers on mouse click (user may be selecting/deleting text)
            ime_clear_all()
            TextInjector.shared.clearSessionBuffer()
            Log.info("Mouse click detected - cleared all buffers")
        }
    }
    
    // MARK: - Configuration
    
    private func setupObservers() {
        // Add observer for Vietnamese toggle
        let toggleObserver = NotificationCenter.default.addObserver(
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
        ResourceManager.shared.register(observer: toggleObserver, identifier: "InputManager.toggleObserver")
        
        // Add observer for shortcut changes
        let shortcutObserver = NotificationCenter.default.addObserver(
            forName: NSNotification.Name("shortcutChanged"),
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
        ResourceManager.shared.register(observer: shortcutObserver, identifier: "InputManager.shortcutObserver")
        
        // Add observer for text expansion enabled/disabled
        let textExpansionObserver = NotificationCenter.default.addObserver(
            forName: .textExpansionEnabledChanged,
            object: nil,
            queue: .main
        ) { [weak self] notification in
            if let enabled = notification.userInfo?["enabled"] as? Bool {
                RustBridge.shared.setShortcutsEnabled(enabled)
                Log.info("Text expansion \(enabled ? "enabled" : "disabled") via notification")
            }
        }
        ResourceManager.shared.register(observer: textExpansionObserver, identifier: "InputManager.textExpansionObserver")
    }
    
    func setEnabled(_ enabled: Bool) {
        let settings = SettingsManager.shared
        
        // Update SettingsManager
        settings.setEnabled(enabled)
        
        // Update Rust engine
        ime_enabled(enabled)
        
        // Clear buffer when toggling
        ime_clear()
        
        Log.info("IME \(enabled ? "enabled" : "disabled")")
        
        // Update per-app state if smart mode is enabled
        if settings.smartModeEnabled {
            PerAppModeManagerEnhanced.shared.setStateForCurrentApp(enabled)
        }
    }
    
    func toggleEnabled() {
        setEnabled(!SettingsManager.shared.isEnabled)
    }
    
    func setInputMethod(_ method: Int) {
        SettingsManager.shared.setInputMethod(method)
        ime_method(UInt8(method))
        Log.info("Input method changed: \(method == 0 ? "Telex" : "VNI")")
    }
    
    func setModernToneStyle(_ modern: Bool) {
        SettingsManager.shared.setModernToneStyle(modern)
        ime_modern(modern)
        Log.info("Modern tone style: \(modern ? "enabled" : "disabled")")
    }
    
    func setInstantRestore(_ enabled: Bool) {
        SettingsManager.shared.setInstantRestoreEnabled(enabled)
        ime_instant_restore(enabled)
        Log.info("Instant auto-restore: \(enabled ? "enabled" : "disabled")")
    }
    
    func reloadShortcuts() {
        // Load shortcuts from UserDefaults or other storage
        // For now, just clear
        ime_clear_shortcuts()
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
        guard SettingsManager.shared.isEnabled else {
            return Unmanaged.passUnretained(event)
        }
        
        // 5.1. Check if Vietnamese is temporarily disabled due to non-Latin input source
        if InputSourceMonitor.shared.shouldSkipVietnameseProcessing() {
            return Unmanaged.passUnretained(event)
        }
        
        // 6. Ignore events with command/control/option modifiers (except Shift)
        if flags.contains(.maskCommand) || flags.contains(.maskControl) || flags.contains(.maskAlternate) {
            // Clear ALL state on modifier shortcuts (selection-delete, Cmd+A, Cmd+V, etc.)
            // This prevents stale buffer content from appearing after selection operations
            ime_clear_all()
            return Unmanaged.passUnretained(event)
        }
        
        // 7. Handle ESC key for word restoration
        if keyCode == 53 { // ESC key
            let result = ime_key(keyCode, false, false)
            if let r = result {
                defer { ime_free(r) }
                if r.pointee.action == 2 { // Restore action
                    let backspaceCount = Int(r.pointee.backspace)
                    let chars = extractChars(from: r.pointee)
                    let (method, delays) = detectMethod()
                    TextInjector.shared.injectSync(
                        bs: backspaceCount,
                        text: Self.makeString(from: chars),
                        method: method,
                        delays: delays,
                        proxy: proxy
                    )
                    return nil
                }
            }
        }
        
        // 8. Handle navigation keys (clear composition and pass through)
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
            ime_clear_all()
            return Unmanaged.passUnretained(event)
        }
        
        // 9. Process with Rust engine
        return processKeyWithEngine(keyCode: keyCode, flags: flags, proxy: proxy, event: event)
    }
    
    private func handleFlagsChanged(event: CGEvent, proxy: CGEventTapProxy) -> Unmanaged<CGEvent>? {
        // Support modifier-only toggle shortcut (e.g., Command+Shift held together)
        let flags = event.flags
        let keyCode: UInt16 = 0xFFFF // Sentinel for modifier-only shortcuts

        // Ignore if no supported modifier is active
        if flags.intersection(KeyboardShortcut.allowedFlags).isEmpty {
            return Unmanaged.passUnretained(event)
        }

        // Trigger toggle when current shortcut is modifier-only and matches flags
        if currentShortcut.isModifierOnly && currentShortcut.matches(keyCode: keyCode, flags: flags) {
            toggleEnabled()
            Log.info("Toggle shortcut (modifier-only) triggered: \(currentShortcut.displayString)")
            return nil // Swallow event
        }

        return Unmanaged.passUnretained(event)
    }
    

    
    private func processKeyWithEngine(keyCode: UInt16, flags: CGEventFlags, proxy: CGEventTapProxy, event: CGEvent) -> Unmanaged<CGEvent>? {
        // IMPORTANT:
        // - `caps` here should represent "uppercase intent" for letters.
        // - On macOS, uppercase is typically (Shift XOR CapsLock).
        // - We still pass `shift` separately to Rust so it can decide when to skip modifiers (e.g., Shift+number).
        let capsLock = flags.contains(.maskAlphaShift)
        let shiftFlag = flags.contains(.maskShift)
        // Robust shift detection: sometimes non-printing key events (like backspace)
        // may not include the Shift flag. Also check physical key state for left/right shift.
        let leftShift: CGKeyCode = 56
        let rightShift: CGKeyCode = 60
        let leftDown = CGEventSource.keyState(.combinedSessionState, key: leftShift)
        let rightDown = CGEventSource.keyState(.combinedSessionState, key: rightShift)
        
        // FIX: The original code `shift = shiftFlag || leftDown || rightDown` caused
        // intermittent auto-capitalization issues because it overrode the OS event flags 
        // for normal typing if the global key state was stale (stuck key).
        // We now enforce that for normal keys, we trust the event flags.
        // We only fallback to physical key state for Backspace where robust Shift detection
        // is critical for the Shift+Backspace (Word Delete) feature.
        let isBackspace = (keyCode == 51)
        let physicalShift = leftDown || rightDown
        let shift = shiftFlag || (isBackspace && physicalShift)
        
        let caps = capsLock != shift
        
        let ctrl = flags.contains(.maskCommand) || flags.contains(.maskControl) || flags.contains(.maskAlternate)
        
        Log.key(keyCode, "Processing")
        
        // Special handling for backspace: coalesce rapid deletes to fix flicker
        if keyCode == 51 && !ctrl { // 51 = backspace
            handleDeleteKey(caps: caps, shift: shift, ctrl: ctrl, proxy: proxy, event: event)
            return nil // Swallow DELETE key - engine handles it
        }
        
        // Cancel any pending coalesced deletes when non-delete key is pressed
        // Clear engine buffer on non-DELETE keys
        
        // Old backspace handling (kept for reference, now replaced by coalescing)
        if false && keyCode == 51 && !ctrl { // 51 = backspace
            // First try Rust engine
            let result = ime_key(keyCode, caps, ctrl)
            if let r = result {
                defer { ime_free(r) }
                
                if r.pointee.action == 1 { // Send - replace text
                    let backspaceCount = Int(r.pointee.backspace)
                    let chars = extractChars(from: r.pointee)
                    
                    Log.transform(backspaceCount, Self.makeString(from: chars))
                    
                    let (method, delays) = detectMethod()
                    TextInjector.shared.injectSync(
                        bs: backspaceCount,
                        text: Self.makeString(from: chars),
                        method: method,
                        delays: delays,
                        proxy: proxy
                    )
                    return nil
                }
            }
            
            // Engine returned none - try to restore word from screen
            if let word = getWordToRestoreOnBackspace() {
                // TODO: Add ime_restore_word function to Rust bridge
                Log.info("Restored word from screen: \(word)")
                // For now, just log - will implement restoration in next iteration
            }
            
            // Pass through backspace to delete the character
            return Unmanaged.passUnretained(event)
        }
        
        // Call Rust engine for other keys (use extended API to preserve Shift state)
        // Respect SettingsManager: only treat Shift as special for engine when user enabled Shift+Backspace
        // Only let the Rust engine treat Shift specially when the user enabled Shift+Backspace.
        let useShiftForEngine = (shift && SettingsManager.shared.shiftBackspaceEnabled)
        let result = ime_key_ext(keyCode, caps, ctrl, useShiftForEngine)
        
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
            
            Log.transform(backspaceCount, Self.makeString(from: chars))
            
            // Inject replacement text using smart injection
            let (method, delays) = detectMethod()
            TextInjector.shared.injectSync(
                bs: backspaceCount,
                text: Self.makeString(from: chars),
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
            
            Log.info("Restore: bs=\(backspaceCount) text=\(Self.makeString(from: chars))")
            
            let (method, delays) = detectMethod()
            TextInjector.shared.injectSync(
                bs: backspaceCount,
                text: Self.makeString(from: chars),
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
    
    // MARK: - DELETE Key Handling
    
    /// Handle DELETE key - process immediately through engine (no batching)
    /// Each DELETE is processed individually to maintain correct state
    private func handleDeleteKey(caps: Bool, shift: Bool, ctrl: Bool, proxy: CGEventTapProxy, event: CGEvent) {
        // Special case: Shift+Backspace deletes entire word (configurable)
        // Use native macOS shortcut (Option+Backspace) for consistent behavior
        // Robust shift detection: also consider physical key state in case event flags omitted it.
        let leftShiftKey: CGKeyCode = 56
        let rightShiftKey: CGKeyCode = 60
        let leftIsDown = CGEventSource.keyState(.combinedSessionState, key: leftShiftKey)
        let rightIsDown = CGEventSource.keyState(.combinedSessionState, key: rightShiftKey)
        let shiftActive = shift || leftIsDown || rightIsDown

        if shiftActive && SettingsManager.shared.shiftBackspaceEnabled {
            guard let src = CGEventSource(stateID: .privateState) else { return }
            
            // Clear engine buffer since we're deleting the entire word
            ime_clear_all()
            
            // Send Option+Backspace (native macOS word delete)
            TextInjector.shared.postKey(51, source: src, flags: .maskAlternate, proxy: proxy)
            Log.info("Shift+DELETE: sent Option+Backspace to delete word")
            return
        }
        
        // Process DELETE through Rust engine (use extended API to preserve Shift state)
        // When processing DELETE, ensure engine sees Shift only if feature enabled
        let useShiftForEngine = (shiftActive && SettingsManager.shared.shiftBackspaceEnabled)
        let result = ime_key_ext(51, caps, ctrl, useShiftForEngine)
        
        if let r = result {
            defer { ime_free(r) }
            
            // Check action from engine
            if r.pointee.action == 1 { // Send - engine has content to replace
                let bs = Int(r.pointee.backspace)
                let chars = extractChars(from: r.pointee)
                let text = Self.makeString(from: chars)
                
                // Detect injection method
                let (method, delays) = detectMethod()
                
                // Inject transformation
                TextInjector.shared.injectSync(
                    bs: bs,
                    text: text,
                    method: method,
                    delays: delays,
                    proxy: proxy
                )
                
                Log.info("DELETE processed: bs=\(bs), text='\(text)'")
                return
            } else if r.pointee.action == 0 && r.pointee.backspace > 0 {
                // Engine wants to delete but has no replacement text
                // This happens when deleting the last character in buffer
                guard let src = CGEventSource(stateID: .privateState) else { return }
                for _ in 0..<r.pointee.backspace {
                    TextInjector.shared.postKey(51, source: src, proxy: proxy)
                }
                Log.info("DELETE: posted \(r.pointee.backspace) raw backspaces")
                return
            }
        }
        
        // Engine has no content - pass through single backspace
        guard let src = CGEventSource(stateID: .privateState) else { return }
        TextInjector.shared.postKey(51, source: src, proxy: proxy)
        Log.info("DELETE: passthrough (engine empty)")
    }

    
    /// Create String from chars array, using pool for common single chars
    @inline(__always)
    private static func makeString(from chars: [ Character]) -> String {
        // Fast path: empty array
        if chars.isEmpty { return "" }
        
        // Use pool for single common characters
        if chars.count == 1, let pooled = commonCharPool[chars[0]] {
            return pooled
        }
        
        // Fallback to normal String construction
        return String(chars)
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
    
    @inline(__always)
    private func extractChars(from result: ImeResult) -> [Character] {
        let count = Int(result.count)
        
        // Fast path: empty result
        guard count > 0, result.chars != nil else {
            return []
        }
        
        // OPTIMIZATION: Pre-allocate with exact capacity
        var chars = [Character]()
        chars.reserveCapacity(count)
        
        // Access heap-allocated chars via pointer
        for i in 0..<count {
            let codepoint = result.chars[i]
            if let scalar = UnicodeScalar(codepoint) {
                chars.append(Character(scalar))
            }
        }
        
        return chars
    }
}

// MARK: - Public API Extensions

extension InputManager {
    func getCurrentState() -> Bool {
        return SettingsManager.shared.isEnabled
    }
    
    func clearComposition() {
        bridge.clearBuffer()
    }
    
    func getCurrentShortcut() -> KeyboardShortcut {
        return currentShortcut
    }
    
    func setShortcut(_ shortcut: KeyboardShortcut) {
        currentShortcut = shortcut
        shortcut.save()
    }
    
    func setEscRestore(_ enabled: Bool) {
        SettingsManager.shared.setEscRestoreEnabled(enabled)
        bridge.setEscRestore(enabled)
    }
    
    func setFreeTone(_ enabled: Bool) {
        SettingsManager.shared.setFreeToneEnabled(enabled)
        bridge.setFreeTone(enabled)
    }
}
