//
//  RustEngineV2.swift
//  GoxViet
//
//  v2 API Singleton Wrapper - Feature Parity with v1 API
//  Maps high-level operations to v2 FFI calls with config changes
//

import Foundation

/// Singleton wrapper around v2 FFI API
/// Provides feature parity with v1 global API while using v2 per-engine design
final class RustEngineV2 {
    
    // MARK: - Singleton
    
    static let shared = RustEngineV2()
    
    // MARK: - Private Properties
    
    private var bridge: RustBridgeV2?
    private let lock = NSLock()
    private var currentConfig: FfiConfig_v2
    
    // MARK: - State (cached from config)
    
    private var isEnabled: Bool = true
    private var escRestoreEnabled: Bool = false
    private var freeToneEnabled: Bool = false
    private var instantRestoreEnabled: Bool = false
    
    // MARK: - Initialization
    
    private init() {
        // Default config
        self.currentConfig = FfiConfig_v2(
            input_method: .telex,
            tone_style: .modern,
            smart_mode: true,
            instant_restore_enabled: true,
            esc_restore_enabled: false,
            enable_shortcuts: true
        )
    }
    
    // MARK: - Lifecycle
    
    /// Initialize engine (equivalent to ime_init)
    func initialize() {
        lock.lock()
        defer { lock.unlock() }
        
        do {
            let tempBridge = RustBridgeV2.shared  // Use singleton!
            try tempBridge.initialize(config: currentConfig)
            bridge = tempBridge  // Only assign if initialization succeeds
            Log.info("RustEngineV2 initialized")
        } catch {
            Log.error("Failed to initialize RustEngineV2: \(error)")
            bridge = nil  // Ensure bridge is nil on failure
        }
    }
    
    /// Destroy engine
    func destroy() {
        lock.lock()
        defer { lock.unlock() }
        
        bridge?.destroyEngine()
        bridge = nil
        Log.info("RustEngineV2 destroyed")
    }
    
    // MARK: - Key Processing (equivalent to ime_key, ime_key_ext)
    
    /// Process key (equivalent to ime_key)
    /// - Parameters:
    ///   - key: ASCII key code (a-z, 0-9)
    ///   - caps: CapsLock state
    ///   - ctrl: Cmd/Ctrl/Alt pressed
    /// - Returns: (text, backspace, consumed)
    func processKey(_ key: UInt8, caps: Bool, ctrl: Bool) -> (text: String, backspace: Int, consumed: Bool) {
        // If ctrl pressed or engine disabled, pass through
        if ctrl {
            Log.info("processKey: ctrl pressed, passing through")
            return ("", 0, false)
        }
        
        if !isEnabled {
            Log.info("processKey: engine disabled, passing through")
            return ("", 0, false)
        }
        
        lock.lock()
        defer { lock.unlock() }
        
        guard let bridge = bridge else {
            Log.error("processKey: bridge is nil, engine not initialized!")
            return ("", 0, false)
        }
        
        do {
            let charStr = key >= 32 && key <= 126 ? String(Character(UnicodeScalar(key))) : "\(key)"
            Log.info("processKey: processing ASCII \(key) ('\(charStr)')")
            let result = try bridge.processKey(key)
            Log.info("processKey: result text='\(result.text)', bs=\(result.backspaceCount), consumed=\(result.consumed)")
            return (result.text, result.backspaceCount, result.consumed)
        } catch {
            Log.error("Failed to process key: \(error)")
            return ("", 0, false)
        }
    }
    
    /// Process key with shift parameter (equivalent to ime_key_ext)
    /// - Parameters:
    ///   - key: ASCII key code
    ///   - caps: CapsLock state
    ///   - ctrl: Cmd/Ctrl/Alt pressed
    ///   - shift: Shift pressed
    /// - Returns: (text, backspace, consumed)
    func processKeyExt(_ key: UInt8, caps: Bool, ctrl: Bool, shift: Bool) -> (text: String, backspace: Int, consumed: Bool) {
        // For VNI: if shift+number, pass through (user wants symbols @#$%^&*())
        if shift && currentConfig.input_method == .vni {
            let numberKeys: Set<UInt8> = [18, 19, 20, 21, 23, 22, 26, 28, 25, 29] // 1-9,0
            if numberKeys.contains(key) {
                return ("", 0, false)
            }
        }
        
        guard let bridge = bridge else {
            Log.error("processKeyExt: bridge is nil, engine not initialized!")
            return ("", 0, false)
        }
        
        do {
            let charStr = key >= 32 && key <= 126 ? String(Character(UnicodeScalar(key))) : "\(key)"
            Log.info("processKeyExt: processing ASCII \(key) ('\(charStr)') caps=\(caps) shift=\(shift) ctrl=\(ctrl)")
            let result = try bridge.processKeyExt(key, caps: caps, shift: shift, ctrl: ctrl)
            Log.info("processKeyExt: result text='\(result.text)', bs=\(result.backspaceCount), consumed=\(result.consumed)")
            return (result.text, result.backspaceCount, result.consumed)
        } catch {
            Log.error("Failed to process key ext: \(error)")
            return ("", 0, false)
        }
    }
    
    // MARK: - Configuration (equivalent to ime_method, ime_modern, etc.)
    
    /// Set input method (equivalent to ime_method)
    /// - Parameter method: 0 = Telex, 1 = VNI
    func setMethod(_ method: UInt8) {
        lock.lock()
        defer { lock.unlock() }
        
        currentConfig.input_method = (method == 0) ? .telex : .vni
        applyConfig()
        Log.info("Input method set to \(method == 0 ? "Telex" : "VNI")")
    }
    
    /// Set tone style (equivalent to ime_modern)
    /// - Parameter modern: true = modern, false = traditional
    func setToneStyle(_ modern: Bool) {
        lock.lock()
        defer { lock.unlock() }
        
        currentConfig.tone_style = modern ? .modern : .traditional
        applyConfig()
        Log.info("Tone style set to \(modern ? "modern" : "traditional")")
    }
    
    /// Enable/disable engine (equivalent to ime_enabled)
    /// - Parameter enabled: true to enable
    func setEnabled(_ enabled: Bool) {
        lock.lock()
        defer { lock.unlock() }
        
        isEnabled = enabled
        Log.info("Engine \(enabled ? "enabled" : "disabled")")
    }
    
    /// Set ESC restore (equivalent to ime_esc_restore)
    /// - Parameter enabled: true to enable ESC restore
    func setEscRestore(_ enabled: Bool) {
        lock.lock()
        defer { lock.unlock() }
        
        escRestoreEnabled = enabled
        currentConfig.esc_restore_enabled = enabled
        applyConfig()
        Log.info("ESC restore \(enabled ? "enabled" : "disabled")")
    }
    
    /// Set free tone (equivalent to ime_free_tone)
    func setFreeTone(_ enabled: Bool) {
        lock.lock()
        defer { lock.unlock() }
        
        freeToneEnabled = enabled
        Log.info("Free tone \(enabled ? "enabled" : "disabled")")
    }
    
    /// Set instant restore (equivalent to ime_instant_restore)
    /// - Parameter enabled: true to enable instant English restore
    func setInstantRestore(_ enabled: Bool) {
        lock.lock()
        defer { lock.unlock() }
        
        instantRestoreEnabled = enabled
        currentConfig.instant_restore_enabled = enabled
        applyConfig()
        Log.info("Instant restore \(enabled ? "enabled" : "disabled")")
    }
    
    // MARK: - Buffer Management
    
    /// Clear current buffer (equivalent to ime_clear)
    func clearBuffer() {
        lock.lock()
        defer { lock.unlock() }
        
        guard let bridge = bridge else {
            Log.error("Cannot clear buffer: Engine not initialized")
            return
        }
        
        do {
            try bridge.resetBuffer()
        } catch {
            Log.error("Failed to reset buffer: \(error)")
        }
    }
    
    /// Clear everything including word history (equivalent to ime_clear_all)
    func clearAll() {
        lock.lock()
        defer { lock.unlock() }
        
        guard let bridge = bridge else {
            Log.error("Cannot clear all: Engine not initialized")
            return
        }
        
        do {
            try bridge.resetAll()
        } catch {
            Log.error("Failed to reset all: \(error)")
        }
    }
    
    // MARK: - Shortcuts
    
    /// Add shortcut (equivalent to ime_add_shortcut)
    func addShortcut(_ key: String, _ value: String) -> Bool {
        guard let bridge = bridge else {
            Log.error("Engine not initialized")
            return false
        }
        
        do {
            try bridge.addShortcut(trigger: key, expansion: value)
            return true
        } catch {
            Log.error("Failed to add shortcut: \(error)")
            return false
        }
    }
    
    /// Remove shortcut (equivalent to ime_remove_shortcut)
    func removeShortcut(_ key: String) -> Bool {
        guard let bridge = bridge else {
            Log.error("Engine not initialized")
            return false
        }
        
        do {
            try bridge.removeShortcut(trigger: key)
            return true
        } catch {
            Log.error("Failed to remove shortcut: \(error)")
            return false
        }
    }
    
    /// Clear all shortcuts (equivalent to ime_clear_shortcuts)
    func clearShortcuts() {
        guard let bridge = bridge else {
            Log.error("Engine not initialized")
            return
        }
        
        do {
            try bridge.clearShortcuts()
        } catch {
            Log.error("Failed to clear shortcuts: \(error)")
        }
    }
    
    /// Get shortcut count
    func shortcutsCount() -> Int {
        guard let bridge = bridge else {
            return 0
        }
        
        return bridge.getShortcutsCount()
    }
    
    /// Set shortcuts enabled (equivalent to ime_set_shortcuts_enabled)
    func setShortcutsEnabled(_ enabled: Bool) -> Bool {
        guard let bridge = bridge else {
            Log.error("Engine not initialized")
            return false
        }
        
        do {
            try bridge.setShortcutsEnabled(enabled)
            return true
        } catch {
            Log.error("Failed to set shortcuts enabled: \(error)")
            return false
        }
    }
    
    /// Skip 'w' shortcut (equivalent to ime_skip_w_shortcut)
    func setSkipWShortcut(_ skip: Bool) {
        Log.warning("Skip w shortcut not supported in v2 API yet")
    }
    
    // MARK: - Text Restoration
    
    /// Restore word (equivalent to ime_restore_word)
    func restoreWord(_ buffer: String, _ position: Int) -> String? {
        // TODO: v2 API doesn't expose restore functions yet
        Log.warning("Restore word not supported in v2 API yet")
        return nil
    }
    
    // MARK: - Helpers
    
    /// Apply current config to engine
    private func applyConfig() {
        guard let bridge = bridge else { return }
        
        do {
            try bridge.setConfig(currentConfig)
        } catch {
            Log.error("Failed to apply config: \(error)")
        }
    }
    
    // MARK: - Version
    
    /// Get version info
    static func getVersion() -> (major: Int, minor: Int, patch: Int) {
        return RustBridgeV2.getVersion()
    }
}

// MARK: - Global v1-style Functions (Clean v2 API with Tuples)

/// Initialize engine
func ime_init_v2() {
    RustEngineV2.shared.initialize()
}

/// Keycode to ASCII mapping for macOS
private let keycodeToAscii: [UInt16: UInt8] = [
    // Letters
    0: 97,   // a
    1: 115,  // s
    2: 100,  // d
    3: 102,  // f
    4: 104,  // h
    5: 103,  // g
    6: 122,  // z
    7: 120,  // x
    8: 99,   // c
    9: 118,  // v
    11: 98,  // b
    12: 113, // q
    13: 119, // w
    14: 101, // e
    15: 114, // r
    16: 121, // y
    17: 116, // t
    31: 111, // o
    32: 117, // u
    34: 105, // i
    35: 112, // p
    37: 108, // l
    38: 106, // j
    40: 107, // k
    45: 110, // n
    46: 109, // m
    // Space & punctuation (word boundary keys — engine clears buffer on these)
    49: 32,  // space
    51: 127, // delete (backspace) → 0x7F DEL
    48: 9,   // tab
    36: 13,  // return
    53: 27,  // esc
    47: 46,  // .
    43: 44,  // ,
    44: 47,  // /
    41: 59,  // ;
    39: 39,  // '
    33: 91,  // [
    30: 93,  // ]
    42: 92,  // backslash
    27: 45,  // -
    24: 61,  // =
    50: 96,  // `
    // Numbers (VNI modifier keys / word boundary for Telex)
    18: 49,  // 1
    19: 50,  // 2
    20: 51,  // 3
    21: 52,  // 4
    23: 53,  // 5
    22: 54,  // 6
    26: 55,  // 7
    28: 56,  // 8
    25: 57,  // 9
    29: 48,  // 0
]

/// Convert macOS keycode to ASCII
private func keycodeToAsciiChar(_ keyCode: UInt16, caps: Bool) -> UInt8? {
    guard let ascii = keycodeToAscii[keyCode] else {
        return nil
    }
    // If caps is true and it's a letter, convert to uppercase
    if caps && ascii >= 97 && ascii <= 122 {
        return ascii - 32  // Convert lowercase to uppercase
    }
    return ascii
}

/// Process key - Returns tuple (text, backspace, consumed)
/// - Parameters:
///   - key: macOS keycode
///   - caps: CapsLock state
///   - ctrl: Cmd/Ctrl/Alt pressed
/// - Returns: (text: String, backspace: Int, consumed: Bool)
func ime_key_v2(_ key: UInt16, _ caps: Bool, _ ctrl: Bool) -> (text: String, backspace: Int, consumed: Bool) {
    guard let ascii = keycodeToAsciiChar(key, caps: caps) else {
        // Unmapped key → word boundary, clear engine buffer
        RustEngineV2.shared.clearAll()
        return ("", 0, false)
    }
    return RustEngineV2.shared.processKey(ascii, caps: caps, ctrl: ctrl)
}

/// Process key extended - Returns tuple (text, backspace, consumed)
/// - Parameters:
///   - key: macOS keycode
///   - caps: CapsLock state
///   - ctrl: Cmd/Ctrl/Alt pressed
///   - shift: Shift pressed
/// - Returns: (text: String, backspace: Int, consumed: Bool)
func ime_key_ext_v2(_ key: UInt16, _ caps: Bool, _ ctrl: Bool, _ shift: Bool) -> (text: String, backspace: Int, consumed: Bool) {
    guard let ascii = keycodeToAsciiChar(key, caps: caps) else {
        // Unmapped key → word boundary, clear engine buffer
        RustEngineV2.shared.clearAll()
        return ("", 0, false)
    }
    return RustEngineV2.shared.processKeyExt(ascii, caps: caps, ctrl: ctrl, shift: shift)
}

/// Set input method
func ime_method_v2(_ method: UInt8) {
    RustEngineV2.shared.setMethod(method)
}

/// Set enabled state
func ime_enabled_v2(_ enabled: Bool) {
    RustEngineV2.shared.setEnabled(enabled)
}

/// Set tone style
func ime_modern_v2(_ modern: Bool) {
    RustEngineV2.shared.setToneStyle(modern)
}

/// Set ESC restore
func ime_esc_restore_v2(_ enabled: Bool) {
    RustEngineV2.shared.setEscRestore(enabled)
}

/// Set free tone
func ime_free_tone_v2(_ enabled: Bool) {
    RustEngineV2.shared.setFreeTone(enabled)
}

/// Set instant restore
func ime_instant_restore_v2(_ enabled: Bool) {
    RustEngineV2.shared.setInstantRestore(enabled)
}

/// Clear buffer
func ime_clear_v2() {
    RustEngineV2.shared.clearBuffer()
}

/// Clear all
func ime_clear_all_v2() {
    RustEngineV2.shared.clearAll()
}

/// Clear shortcuts
func ime_clear_shortcuts_v2() {
    RustEngineV2.shared.clearShortcuts()
}

// MARK: - Shortcut Functions

/// Add shortcut
func ime_add_shortcut_v2(_ trigger: String, _ replacement: String) -> Bool {
    return RustEngineV2.shared.addShortcut(trigger, replacement)
}

/// Remove shortcut
func ime_remove_shortcut_v2(_ trigger: String) -> Bool {
    return RustEngineV2.shared.removeShortcut(trigger)
}

/// Get shortcut count
func ime_shortcuts_count_v2() -> Int {
    return RustEngineV2.shared.shortcutsCount()
}

/// Set shortcuts enabled
func ime_set_shortcuts_enabled_v2(_ enabled: Bool) -> Bool {
    return RustEngineV2.shared.setShortcutsEnabled(enabled)
}
