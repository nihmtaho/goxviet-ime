//
//  RustBridge.swift
//  VietnameseIMEFast
//
//  Swift bridge to Rust core IME engine
//  Provides FFI interface and helper functions
//

import Cocoa
import ApplicationServices

// MARK: - Rust Bridge Class

class RustBridge {
    private var isInitialized = false
    
    func initialize() {
        guard !isInitialized else { return }
        ime_init()
        
        // Set default configuration
        ime_method(0)  // 0 = Telex, 1 = VNI
        ime_enabled(true)  // Enable by default
        ime_modern(false)  // Use traditional tone placement
        ime_esc_restore(true)  // Enable ESC restore
        
        isInitialized = true
        Log.info("RustBridge initialized with Telex mode enabled")
    }
    
    func setMethod(_ method: Int) {
        Log.method("Setting input method to \(method)")
        ime_method(UInt8(method))
    }
    
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
    
    func addShortcut(trigger: String, replacement: String) {
        guard let triggerC = trigger.cString(using: .utf8),
              let replacementC = replacement.cString(using: .utf8) else { return }
        Log.info("Add shortcut: \(trigger) → \(replacement)")
        triggerC.withUnsafeBufferPointer { triggerPtr in
            replacementC.withUnsafeBufferPointer { replacementPtr in
                ime_add_shortcut(triggerPtr.baseAddress!, replacementPtr.baseAddress!)
            }
        }
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