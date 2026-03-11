//
//  RestoreShortcut.swift
//  GoxViet
//
//  Model for configurable restore-to-raw shortcut.
//  Supports multi-tap modifier keys (e.g., double Option, triple Command).
//

import Cocoa

/// A single hotkey entry in a restore shortcut sequence.
/// Each entry is a modifier flag set (no character key – purely modifier-level).
struct RestoreHotkey: Codable, Equatable {
    /// Raw value of `CGEventFlags` intersected with allowed modifiers.
    var flags: UInt64

    var cgFlags: CGEventFlags {
        CGEventFlags(rawValue: flags).intersection(Self.allowedModifiers)
    }

    nonisolated static let allowedModifiers: CGEventFlags = [
        .maskCommand,
        .maskAlternate,
        .maskShift,
        .maskControl,
    ]

    /// Human-readable symbol for the modifier set.
    var displaySymbol: String {
        let f = cgFlags
        var parts: [String] = []
        if f.contains(.maskControl)   { parts.append("⌃") }
        if f.contains(.maskAlternate) { parts.append("⌥") }
        if f.contains(.maskShift)     { parts.append("⇧") }
        if f.contains(.maskCommand)   { parts.append("⌘") }
        return parts.joined()
    }

    // Explicit nonisolated init so preset statics can be created from nonisolated context
    nonisolated init(flags: UInt64) {
        self.flags = flags
    }

    // Explicit nonisolated Decodable init so JSONDecoder can decode from nonisolated context
    nonisolated init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)
        flags = try container.decode(UInt64.self, forKey: .flags)
    }

    private enum CodingKeys: String, CodingKey {
        case flags
    }
}

/// Restore shortcut represented as an ordered sequence of modifier taps.
/// Examples:
///   – Double Option  → [⌥, ⌥]
///   – Triple Command → [⌘, ⌘, ⌘]
///   – Option then Shift → [⌥, ⇧]
struct RestoreShortcut: Codable, Equatable {
    /// Ordered hotkey taps (min 1, max 4).
    var keys: [RestoreHotkey]

    /// Maximum interval (seconds) between consecutive taps.
    var tapInterval: TimeInterval = 0.4

    // Explicit nonisolated init so preset statics can be created from nonisolated context
    nonisolated init(keys: [RestoreHotkey], tapInterval: TimeInterval = 0.4) {
        self.keys = keys
        self.tapInterval = tapInterval
    }

    // Explicit nonisolated Decodable init so JSONDecoder can decode from nonisolated context
    nonisolated init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)
        keys = try container.decode([RestoreHotkey].self, forKey: .keys)
        tapInterval = try container.decodeIfPresent(TimeInterval.self, forKey: .tapInterval) ?? 0.4
    }

    private enum CodingKeys: String, CodingKey {
        case keys
        case tapInterval
    }

    // MARK: - Validation

    var isValid: Bool {
        keys.count >= 1 && keys.count <= 4 && keys.allSatisfy({ $0.flags != 0 })
    }

    var tapCount: Int { keys.count }

    // MARK: - Display

    /// e.g. "⌥ × 2" or "⌘ ⌥"
    var displayString: String {
        guard !keys.isEmpty else { return "None" }

        // Check if all keys are the same modifier → "⌥ × N"
        if keys.count > 1, Set(keys.map(\.flags)).count == 1 {
            return "\(keys[0].displaySymbol) × \(keys.count)"
        }

        return keys.map(\.displaySymbol).joined(separator: " ")
    }

    /// Parts for badge-style display (each element rendered as a badge).
    var displayParts: [String] {
        guard !keys.isEmpty else { return ["None"] }

        if keys.count > 1, Set(keys.map(\.flags)).count == 1 {
            return ["\(keys[0].displaySymbol) × \(keys.count)"]
        }
        return keys.map(\.displaySymbol)
    }

    // MARK: - Presets

    nonisolated static let doubleOption = RestoreShortcut(keys: [
        RestoreHotkey(flags: CGEventFlags.maskAlternate.rawValue),
        RestoreHotkey(flags: CGEventFlags.maskAlternate.rawValue),
    ])

    nonisolated static let tripleOption = RestoreShortcut(keys: [
        RestoreHotkey(flags: CGEventFlags.maskAlternate.rawValue),
        RestoreHotkey(flags: CGEventFlags.maskAlternate.rawValue),
        RestoreHotkey(flags: CGEventFlags.maskAlternate.rawValue),
    ])

    nonisolated static let doubleCommand = RestoreShortcut(keys: [
        RestoreHotkey(flags: CGEventFlags.maskCommand.rawValue),
        RestoreHotkey(flags: CGEventFlags.maskCommand.rawValue),
    ])

    nonisolated static let doubleShift = RestoreShortcut(keys: [
        RestoreHotkey(flags: CGEventFlags.maskShift.rawValue),
        RestoreHotkey(flags: CGEventFlags.maskShift.rawValue),
    ])

    nonisolated static let `default` = doubleOption

    static var presets: [RestoreShortcut] {
        [doubleOption, tripleOption, doubleCommand, doubleShift]
    }

    // MARK: - Persistence

    @MainActor static func load() -> RestoreShortcut {
        guard let data = UserDefaults.standard.data(forKey: SettingsKey.restoreShortcut),
              let shortcut = try? JSONDecoder().decode(RestoreShortcut.self, from: data)
        else {
            return .default
        }
        return shortcut
    }

    func save() {
        if let data = try? JSONEncoder().encode(self) {
            UserDefaults.standard.set(data, forKey: SettingsKey.restoreShortcut)
        }
    }
}

