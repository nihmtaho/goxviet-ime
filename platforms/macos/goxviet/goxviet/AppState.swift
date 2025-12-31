//
//  AppState.swift
//  GoxViet
//
//  Manages application state and per-app Vietnamese input mode settings
//

import Foundation
import Cocoa

/// Maximum number of per-app settings to store (prevents unbounded memory growth)
/// This limit is reasonable for most users while preventing memory bloat
private let MAX_PER_APP_ENTRIES = 100

/// Global application state manager
class AppState {
    static let shared = AppState()

    // MARK: - Properties

    /// Whether Vietnamese input is currently enabled
    private(set) var isEnabled: Bool = true

    /// Whether smart per-app mode is enabled
    var isSmartModeEnabled: Bool {
        get {
            return UserDefaults.standard.bool(forKey: Keys.smartModeEnabled)
        }
        set {
            UserDefaults.standard.set(newValue, forKey: Keys.smartModeEnabled)
            Log.info("Smart per-app mode: \(newValue ? "enabled" : "disabled")")
        }
    }

    /// Input method (0=Telex, 1=VNI)
    var inputMethod: Int {
        get {
            return UserDefaults.standard.integer(forKey: Keys.inputMethod)
        }
        set {
            UserDefaults.standard.set(newValue, forKey: Keys.inputMethod)
        }
    }

    /// Modern tone placement style
    var modernToneStyle: Bool {
        get {
            return UserDefaults.standard.bool(forKey: Keys.modernToneStyle)
        }
        set {
            UserDefaults.standard.set(newValue, forKey: Keys.modernToneStyle)
        }
    }

    /// ESC key restores original word
    var escRestoreEnabled: Bool {
        get {
            return UserDefaults.standard.bool(forKey: Keys.escRestore)
        }
        set {
            UserDefaults.standard.set(newValue, forKey: Keys.escRestore)
        }
    }

    /// Free tone placement (allow tone marks before completing vowel)
    var freeToneEnabled: Bool {
        get {
            return UserDefaults.standard.bool(forKey: Keys.freeTone)
        }
        set {
            UserDefaults.standard.set(newValue, forKey: Keys.freeTone)
        }
    }

    /// Auto-disable Vietnamese when non-Latin input sources are active
    /// (e.g., Japanese, Korean, Chinese keyboards)
    var autoDisableForNonLatinEnabled: Bool {
        get {
            return UserDefaults.standard.bool(forKey: Keys.autoDisableNonLatin)
        }
        set {
            UserDefaults.standard.set(newValue, forKey: Keys.autoDisableNonLatin)
            Log.info("Auto-disable for non-Latin input: \(newValue ? "enabled" : "disabled")")
        }
    }

    // MARK: - Storage Keys

    private enum Keys {
        static let smartModeEnabled = "com.goxviet.ime.smartMode"
        static let inputMethod = "com.goxviet.ime.method"
        static let modernToneStyle = "com.goxviet.ime.modernTone"
        static let escRestore = "com.goxviet.ime.escRestore"
        static let freeTone = "com.goxviet.ime.freeTone"
        static let perAppModes = "com.goxviet.ime.perAppModes"
        static let autoDisableNonLatin = "com.goxviet.ime.autoDisableNonLatin"
    }

    // MARK: - Initialization

    private init() {
        // Set defaults if not previously configured
        registerDefaults()
    }

    private func registerDefaults() {
        let defaults: [String: Any] = [
            Keys.smartModeEnabled: true,
            Keys.inputMethod: 0,  // Telex
            Keys.modernToneStyle: false,
            Keys.escRestore: true,
            Keys.freeTone: false,
            Keys.autoDisableNonLatin: true  // Default: enabled for better UX with multilingual users
        ]

        UserDefaults.standard.register(defaults: defaults)
    }

    // MARK: - Global State Management

    /// Set enabled state and notify observers
    func setEnabled(_ enabled: Bool) {
        isEnabled = enabled

        // Post notification for UI update
        NotificationCenter.default.post(
            name: .updateStateChanged,
            object: enabled
        )

        Log.info("Gõ Việt input: \(enabled ? "enabled" : "disabled")")
    }

    /// Set enabled state without posting notification (used during app switching)
    func setEnabledSilently(_ enabled: Bool) {
        isEnabled = enabled
    }

    // MARK: - Per-App Mode Management

    /// Get the saved mode for a specific app
    /// Returns true (enabled) by default if no specific setting exists
    func getPerAppMode(bundleId: String) -> Bool {
        guard let dict = UserDefaults.standard.dictionary(forKey: Keys.perAppModes) as? [String: Bool] else {
            return true  // Default: enabled
        }

        // Return stored value, or true if not found
        return dict[bundleId] ?? true
    }

    /// Save the mode for a specific app
    /// Only stores apps that are disabled (to save space)
    /// Enforces MAX_PER_APP_ENTRIES limit to prevent unbounded memory growth
    func setPerAppMode(bundleId: String, enabled: Bool) {
        var dict = UserDefaults.standard.dictionary(forKey: Keys.perAppModes) as? [String: Bool] ?? [:]

        if enabled {
            // Remove from dictionary if enabled (default state)
            dict.removeValue(forKey: bundleId)
        } else {
            // Check capacity limit before adding new entry
            if dict[bundleId] == nil && dict.count >= MAX_PER_APP_ENTRIES {
                Log.warning("Per-app settings at capacity (\(MAX_PER_APP_ENTRIES)). Not saving new entry for: \(bundleId)")
                Log.warning("Consider clearing old per-app settings from Preferences.")
                return
            }

            // Store only disabled apps
            dict[bundleId] = false
        }

        UserDefaults.standard.set(dict, forKey: Keys.perAppModes)

        Log.info("Per-app mode saved: \(bundleId) = \(enabled ? "enabled" : "disabled")")
    }

    /// Clear per-app settings for a specific app (reset to default)
    func clearPerAppMode(bundleId: String) {
        var dict = UserDefaults.standard.dictionary(forKey: Keys.perAppModes) as? [String: Bool] ?? [:]
        dict.removeValue(forKey: bundleId)
        UserDefaults.standard.set(dict, forKey: Keys.perAppModes)

        Log.info("Per-app mode cleared: \(bundleId)")
    }

    /// Get all per-app settings
    func getAllPerAppModes() -> [String: Bool] {
        return UserDefaults.standard.dictionary(forKey: Keys.perAppModes) as? [String: Bool] ?? [:]
    }

    /// Clear all per-app settings
    func clearAllPerAppModes() {
        UserDefaults.standard.removeObject(forKey: Keys.perAppModes)
        Log.info("All per-app modes cleared")
    }

    /// Get count of stored per-app settings
    func getPerAppModesCount() -> Int {
        let dict = UserDefaults.standard.dictionary(forKey: Keys.perAppModes) as? [String: Bool] ?? [:]
        return dict.count
    }

    /// Check if per-app settings is at capacity
    func isPerAppModesAtCapacity() -> Bool {
        return getPerAppModesCount() >= MAX_PER_APP_ENTRIES
    }

    /// Get maximum capacity for per-app settings
    func getPerAppModesCapacity() -> Int {
        return MAX_PER_APP_ENTRIES
    }

    /// Get human-readable app name from bundle ID
    func getAppName(bundleId: String) -> String {
        // Try to get the app name from the running application
        if let app = NSWorkspace.shared.runningApplications.first(where: { $0.bundleIdentifier == bundleId }) {
            return app.localizedName ?? bundleId
        }

        // Fallback: try to extract from bundle ID
        let components = bundleId.components(separatedBy: ".")
        if let lastComponent = components.last {
            return lastComponent.capitalized
        }

        return bundleId
    }
}

// MARK: - Notification Names

extension Notification.Name {
    static let toggleVietnamese = Notification.Name("toggleVietnamese")
    static let updateStateChanged = Notification.Name("updateStateChanged")
    static let shortcutChanged = Notification.Name("shortcutChanged")
    static let shortcutRecorded = Notification.Name("shortcutRecorded")
    static let shortcutRecordingCancelled = Notification.Name("shortcutRecordingCancelled")
    static let showUpdateWindow = Notification.Name("showUpdateWindow")
}
