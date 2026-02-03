//
//  RustBridge.swift
//  GoxViet
//
//  Swift bridge to Rust core IME engine
//  Provides FFI interface and helper functions
//

import Cocoa
import ApplicationServices

// MARK: - Rust Bridge Class

class RustBridge {
    static let shared = RustBridge()
    
    private var isInitialized = false
    
    private init() {}
    
    func initialize() {
        guard !isInitialized else { return }
        ime_init()
        
        // Set default configuration
        ime_method(0)  // 0 = Telex, 1 = VNI
        ime_enabled(true)  // Enable by default
        ime_modern(false)  // Use traditional tone placement
        ime_esc_restore(true)  // Enable ESC restore
        ime_instant_restore(true)  // Enable instant auto-restore by default
        
        isInitialized = true
        Log.info("RustBridge initialized with Telex mode enabled")
    }
    
    @inline(__always)
    func setMethod(_ method: Int) {
        Log.method("Setting input method to \(method)")
        ime_method(UInt8(method))
    }
    
    @inline(__always)
    func setEnabled(_ enabled: Bool) {
        Log.info("IME \(enabled ? "enabled" : "disabled")")
        ime_enabled(enabled)
    }
    
    func setSkipWShortcut(_ skip: Bool) {
        Log.info("Skip W in shortcuts: \(skip)")
        ime_skip_w_shortcut(skip)
    }
    
    func setEscRestore(_ enabled: Bool) {
        Log.info("ESC restore: \(enabled)")
        ime_esc_restore(enabled)
    }
    
    func setFreeTone(_ enabled: Bool) {
        Log.info("Free tone: \(enabled)")
        ime_free_tone(enabled)
    }
    
    func setModernTone(_ modern: Bool) {
        Log.info("Modern tone style: \(modern)")
        ime_modern(modern)
    }
    
    func setInstantRestore(_ enabled: Bool) {
        Log.info("Instant auto-restore: \(enabled)")
        ime_instant_restore(enabled)
    }
    
    @inline(__always)
    func clearBuffer() {
        ime_clear()
    }
    
    func restoreWord(_ word: String) {
        guard let cString = word.cString(using: .utf8) else { return }
        Log.info("Restore word: \(word)")
        cString.withUnsafeBufferPointer { ptr in
            ime_restore_word(ptr.baseAddress!)
        }
    }
    
    // MARK: - Shortcut Management
    
    @discardableResult
    func addShortcut(trigger: String, replacement: String) -> Bool {
        guard let triggerC = trigger.cString(using: .utf8),
              let replacementC = replacement.cString(using: .utf8) else { 
            Log.error("Failed to convert strings to C format")
            return false 
        }
        Log.info("Add shortcut: \(trigger) → \(replacement)")
        var result = false
        triggerC.withUnsafeBufferPointer { triggerPtr in
            replacementC.withUnsafeBufferPointer { replacementPtr in
                result = ime_add_shortcut(triggerPtr.baseAddress!, replacementPtr.baseAddress!)
            }
        }
        return result
    }
    
    func removeShortcut(trigger: String) {
        guard let triggerC = trigger.cString(using: .utf8) else { return }
        Log.info("Remove shortcut: \(trigger)")
        triggerC.withUnsafeBufferPointer { ptr in
            ime_remove_shortcut(ptr.baseAddress!)
        }
    }
    
    func clearShortcuts() {
        Log.info("Clear all shortcuts")
        ime_clear_shortcuts()
    }
    
    func syncShortcuts(_ shortcuts: [(key: String, value: String, enabled: Bool)]) {
        clearShortcuts()
        for shortcut in shortcuts where shortcut.enabled {
            addShortcut(trigger: shortcut.key, replacement: shortcut.value)
        }
    }
    
    // MARK: - Text Expansion Extended Methods
    
    /// Export all shortcuts as JSON string
    /// - Returns: JSON string containing all shortcuts, or nil if export fails
    func exportShortcutsJSON() -> String? {
        guard let jsonPtr = ime_export_shortcuts_json() else {
            Log.error("Failed to export shortcuts JSON")
            return nil
        }
        
        defer {
            ime_free_string(jsonPtr)
        }
        
        let jsonString = String(cString: jsonPtr)
        Log.info("Exported shortcuts JSON (\(jsonString.count) bytes)")
        return jsonString
    }
    
    /// Import shortcuts from JSON string
    /// - Parameter json: JSON string containing shortcuts
    /// - Returns: Number of imported shortcuts, or -1 on error
    func importShortcutsJSON(_ json: String) -> Int {
        guard let jsonC = json.cString(using: .utf8) else {
            Log.error("Failed to convert JSON string to C string")
            return -1
        }
        
        let count = jsonC.withUnsafeBufferPointer { ptr in
            Int(ime_import_shortcuts_json(ptr.baseAddress!))
        }
        
        if count >= 0 {
            Log.info("Imported \(count) shortcuts from JSON")
        } else {
            Log.error("Failed to import shortcuts from JSON")
        }
        
        return count
    }
    
    /// Enable or disable text expansion globally
    /// - Parameter enabled: true to enable, false to disable
    func setShortcutsEnabled(_ enabled: Bool) {
        ime_set_shortcuts_enabled(enabled)
        Log.info("Text expansion \(enabled ? "enabled" : "disabled")")
    }
    
    /// Get current number of shortcuts
    /// - Returns: Number of shortcuts currently stored
    func getShortcutsCount() -> Int {
        Int(ime_shortcuts_count())
    }
    
    /// Get maximum capacity for shortcuts
    /// - Returns: Maximum number of shortcuts that can be stored
    func getShortcutsCapacity() -> Int {
        Int(ime_shortcuts_capacity())
    }
}



// MARK: - Helper Extensions

extension CGEventFlags {
    var modifierCount: Int {
        var count = 0
        if contains(.maskShift) { count += 1 }
        if contains(.maskControl) { count += 1 }
        if contains(.maskAlternate) { count += 1 }
        if contains(.maskCommand) { count += 1 }
        return count
    }
}