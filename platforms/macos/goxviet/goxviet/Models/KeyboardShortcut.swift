//
//  KeyboardShortcut.swift
//  GoxViet
//
//  Manages keyboard shortcuts for toggling Gõ Việt input
//  Based on reference implementation
//

import Cocoa

// MARK: - Keyboard Shortcut Structure

struct KeyboardShortcut: Codable, Equatable {
    var keyCode: UInt16
    var modifiers: UInt64  // CGEventFlags raw value

    // Supported modifier flags for matching and display
    static let allowedFlags: CGEventFlags = [
        .maskControl,
        .maskAlternate,
        .maskShift,
        .maskCommand,
        .maskSecondaryFn
    ]
    
    // Default: Control+Space
    static let `default` = KeyboardShortcut(
        keyCode: 0x31,  // Space key
        modifiers: CGEventFlags.maskControl.rawValue
    )
    
    // Alternative shortcuts
    static let commandSpace = KeyboardShortcut(
        keyCode: 0x31,
        modifiers: CGEventFlags.maskCommand.rawValue
    )
    
    static let controlShiftSpace = KeyboardShortcut(
        keyCode: 0x31,
        modifiers: (CGEventFlags.maskControl.rawValue | CGEventFlags.maskShift.rawValue)
    )
    
    // MARK: - Display
    
    var displayParts: [String] {
        var parts: [String] = []
        let flags = normalizedFlags

        // Add modifiers in standard order: Fn, Control, Option, Shift, Command
        if flags.contains(.maskSecondaryFn) { parts.append("fn") }
        if flags.contains(.maskControl) { parts.append("⌃") }
        if flags.contains(.maskAlternate) { parts.append("⌥") }
        if flags.contains(.maskShift) { parts.append("⇧") }
        if flags.contains(.maskCommand) { parts.append("⌘") }
        
        // Add key name
        let keyStr = keyCodeToString(keyCode)
        if !keyStr.isEmpty { parts.append(keyStr) }
        
        return parts
    }
    
    var displayString: String {
        return displayParts.joined()
    }
    
    // MARK: - Key Code Mapping
    
    private func keyCodeToString(_ code: UInt16) -> String {
        switch code {
        // Special keys
        case 0x31: return "Space"
        case 0x24: return "↩"      // Return
        case 0x30: return "⇥"      // Tab
        case 0x33: return "⌫"      // Backspace
        case 0x35: return "⎋"      // Escape
        case 0x7B: return "←"      // Left arrow
        case 0x7C: return "→"      // Right arrow
        case 0x7D: return "↓"      // Down arrow
        case 0x7E: return "↑"      // Up arrow
        
        // Letters
        case 0x00: return "A"
        case 0x01: return "S"
        case 0x02: return "D"
        case 0x03: return "F"
        case 0x04: return "H"
        case 0x05: return "G"
        case 0x06: return "Z"
        case 0x07: return "X"
        case 0x08: return "C"
        case 0x09: return "V"
        case 0x0B: return "B"
        case 0x0C: return "Q"
        case 0x0D: return "W"
        case 0x0E: return "E"
        case 0x0F: return "R"
        case 0x10: return "Y"
        case 0x11: return "T"
        case 0x1F: return "O"
        case 0x20: return "U"
        case 0x22: return "I"
        case 0x23: return "P"
        case 0x25: return "L"
        case 0x26: return "J"
        case 0x28: return "K"
        case 0x2D: return "N"
        case 0x2E: return "M"
        
        // Numbers
        case 0x12: return "1"
        case 0x13: return "2"
        case 0x14: return "3"
        case 0x15: return "4"
        case 0x17: return "5"
        case 0x16: return "6"
        case 0x1A: return "7"
        case 0x1C: return "8"
        case 0x19: return "9"
        case 0x1D: return "0"
        
        // Punctuation
        case 0x18: return "="
        case 0x1B: return "-"
        case 0x1E: return "]"
        case 0x21: return "["
        case 0x27: return "'"
        case 0x29: return ";"
        case 0x2A: return "\\"
        case 0x2B: return ","
        case 0x2C: return "/"
        case 0x2F: return "."
        case 0x32: return "`"
        
        // Modifier-only shortcut (no key)
        case 0xFFFF: return ""
        
        default: return "?"
        }
    }
    
    // MARK: - Persistence
    
    private static let storageKey = "com.vietnamese.ime.toggleShortcut"
    
    static func load() -> KeyboardShortcut {
        guard let data = UserDefaults.standard.data(forKey: storageKey),
              let shortcut = try? JSONDecoder().decode(KeyboardShortcut.self, from: data) else {
            return .default
        }
        return shortcut
    }
    
    func save() {
        if let data = try? JSONEncoder().encode(self) {
            UserDefaults.standard.set(data, forKey: KeyboardShortcut.storageKey)
            
            // Notify observers that shortcut changed
            NotificationCenter.default.post(name: NSNotification.Name("shortcutChanged"), object: self)
            
            Log.info("Shortcut saved: \(displayString)")
        }
    }
    
    // MARK: - Validation
    
    /// Check if this shortcut is modifier-only (no character key)
    var isModifierOnly: Bool {
        return keyCode == 0xFFFF
    }
    
    /// Check if this shortcut is valid
    var isValid: Bool {
        // Must have at least one supported modifier
        let flags = normalizedFlags
        let hasModifier = !flags.intersection(Self.allowedFlags).isEmpty
        
        // Either has a key or is modifier-only
        return hasModifier && (keyCode != 0 || isModifierOnly)
    }
    
    /// Get conflict status and severity (warning, not blocking)
    var conflictInfo: (hasConflict: Bool, level: String, message: String)? {
        let flags = normalizedFlags
        
        // Cmd+Space is Spotlight (can be overridden but warning)
        if keyCode == 0x31 && flags.contains(.maskCommand) && !flags.contains(.maskShift) {
            return (true, "spotlight", "Command+Space conflicts with macOS Spotlight. You can override it, but Spotlight will not work unless disabled in System Settings.")
        }
        
        // Cmd+Tab is app switcher (can be overridden but warning)
        if keyCode == 0x30 && flags.contains(.maskCommand) {
            return (true, "switcher", "Command+Tab conflicts with app switcher. You can override it, but app switching may be affected.")
        }
        
        // Control+Space may conflict with input source switching on some systems
        if keyCode == 0x31 && flags.contains(.maskControl) && !flags.contains(.maskShift) {
            return (true, "inputSource", "Control+Space may conflict with Input Source switching (depending on system settings). Check System Settings → Keyboard → Keyboard Shortcuts → Input Sources if needed.")
        }
        
        return nil
    }
    
    /// Deprecated: use conflictInfo instead
    var hasSystemConflict: Bool {
        return conflictInfo != nil
    }
}

// MARK: - Shortcut Matching

extension KeyboardShortcut {
    /// Match this shortcut against a key event
    func matches(keyCode: UInt16, flags: CGEventFlags) -> Bool {
        let incoming = flags.intersection(Self.allowedFlags)
        let savedFlags = normalizedFlags

        if isModifierOnly {
            // Modifier-only shortcuts should only match on the sentinel keyCode (flagsChanged)
            return keyCode == 0xFFFF && incoming == savedFlags
        }

        // For key-bound shortcuts, keyCode must match and modifiers must match
        guard keyCode == self.keyCode else { return false }
        return incoming == savedFlags
    }
}

// MARK: - Preset Shortcuts

extension KeyboardShortcut {
    static var presets: [KeyboardShortcut] {
        return [
            .default,                           // Control+Space
            commandSpace,                       // Command+Space (may conflict with Spotlight)
            controlShiftSpace,                  // Control+Shift+Space
            KeyboardShortcut(                   // Control+Shift+V
                keyCode: 0x09,
                modifiers: (CGEventFlags.maskControl.rawValue | CGEventFlags.maskShift.rawValue)
            ),
            KeyboardShortcut(                   // Control+Option+Space
                keyCode: 0x31,
                modifiers: (CGEventFlags.maskControl.rawValue | CGEventFlags.maskAlternate.rawValue)
            )
        ]
    }
}

// MARK: - Helpers

extension KeyboardShortcut {
    /// Normalized flags limited to supported modifiers
    var normalizedFlags: CGEventFlags {
        return CGEventFlags(rawValue: modifiers).intersection(Self.allowedFlags)
    }

    /// Build a shortcut from a keyDown NSEvent.
    static func from(event: NSEvent) -> KeyboardShortcut? {
        guard event.type == .keyDown else { return nil }
        let cgFlags = event.cgEvent?.flags ?? CGEventFlags(rawValue: UInt64(event.modifierFlags.rawValue))
        let normalized = cgFlags.intersection(Self.allowedFlags)
        guard !normalized.isEmpty else { return nil }
        return KeyboardShortcut(keyCode: event.keyCode, modifiers: normalized.rawValue)
    }

    /// Build a modifier-only shortcut from flags (no key code).
    static func modifierOnly(from flags: CGEventFlags) -> KeyboardShortcut? {
        let normalized = flags.intersection(Self.allowedFlags)
        guard !normalized.isEmpty else { return nil }
        return KeyboardShortcut(keyCode: 0xFFFF, modifiers: normalized.rawValue)
    }
}
