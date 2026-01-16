//
//  AppState.swift
//  GoxViet
//
//  Manages application state and per-app Vietnamese input mode settings
//  Default: English input mode (Vietnamese disabled)
//  Per-app: Only stores apps with Vietnamese ENABLED (max 100 apps)
//

import Foundation
import Cocoa
import Combine

/// Maximum number of per-app settings to store (prevents unbounded memory growth)
/// This limit is reasonable for most users while preventing memory bloat
private let MAX_PER_APP_ENTRIES = 100

/// Global application state manager
class AppState: ObservableObject {
    static let shared = AppState()

    // MARK: - Properties

    /// Whether Vietnamese input is currently enabled
    private(set) var isEnabled: Bool = false

    /// Whether smart per-app mode is enabled
    var isSmartModeEnabled: Bool {
        get {
            return UserDefaults.standard.bool(forKey: Keys.smartModeEnabled)
        }
        set {
            UserDefaults.standard.set(newValue, forKey: Keys.smartModeEnabled)
            
            // Post notification for UI synchronization
            NotificationCenter.default.post(
                name: .smartModeChanged,
                object: newValue
            )
            
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
            
            // Post notification for UI synchronization
            NotificationCenter.default.post(
                name: .inputMethodChanged,
                object: newValue
            )
        }
    }

    /// Modern tone placement style
    var modernToneStyle: Bool {
        get {
            return UserDefaults.standard.bool(forKey: Keys.modernToneStyle)
        }
        set {
            UserDefaults.standard.set(newValue, forKey: Keys.modernToneStyle)
            
            // Post notification for UI synchronization
            NotificationCenter.default.post(
                name: .toneStyleChanged,
                object: newValue
            )
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

    /// Automatically check for application updates in the background
    var autoUpdateCheckEnabled: Bool {
        get {
            return UserDefaults.standard.bool(forKey: Keys.autoUpdateCheck)
        }
        set {
            UserDefaults.standard.set(newValue, forKey: Keys.autoUpdateCheck)
            Log.info("Auto update check: \(newValue ? "enabled" : "disabled")")
        }
    }

    /// Hide app from Dock (menubar-only mode)
    var hideFromDock: Bool {
        get {
            return UserDefaults.standard.bool(forKey: Keys.hideFromDock)
        }
        set {
            objectWillChange.send()
            UserDefaults.standard.set(newValue, forKey: Keys.hideFromDock)
            Log.info("Hide from Dock: \(newValue ? "enabled" : "disabled")")
        }
    }

    /// Automatically install updates via Homebrew when available
    var autoUpdateInstallEnabled: Bool {
        get {
            return UserDefaults.standard.bool(forKey: Keys.autoUpdateInstall)
        }
        set {
            UserDefaults.standard.set(newValue, forKey: Keys.autoUpdateInstall)
            Log.info("Auto install updates (Homebrew): \(newValue ? "enabled" : "disabled")")
        }
    }

    /// Timestamp of the last update check
    var lastUpdateCheckDate: Date? {
        get {
            let timestamp = UserDefaults.standard.double(forKey: Keys.lastUpdateCheck)
            return timestamp > 0 ? Date(timeIntervalSince1970: timestamp) : nil
        }
        set {
            if let date = newValue {
                UserDefaults.standard.set(date.timeIntervalSince1970, forKey: Keys.lastUpdateCheck)
            } else {
                UserDefaults.standard.removeObject(forKey: Keys.lastUpdateCheck)
            }
        }
    }

    /// The last version we already notified the user about (prevents repeated alerts)
    var lastNotifiedUpdateVersion: String? {
        get {
            let version = UserDefaults.standard.string(forKey: Keys.lastNotifiedUpdateVersion) ?? ""
            return version.isEmpty ? nil : version
        }
        set {
            if let value = newValue, !value.isEmpty {
                UserDefaults.standard.set(value, forKey: Keys.lastNotifiedUpdateVersion)
            } else {
                UserDefaults.standard.removeObject(forKey: Keys.lastNotifiedUpdateVersion)
            }
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
        static let knownApps = "com.goxviet.ime.knownApps"
        static let autoDisableNonLatin = "com.goxviet.ime.autoDisableNonLatin"
        static let hasLaunchedBefore = "com.goxviet.ime.hasLaunchedBefore"
        static let autoUpdateCheck = "com.goxviet.ime.autoUpdateCheck"
        static let autoUpdateInstall = "com.goxviet.ime.autoUpdateInstall"
        static let lastUpdateCheck = "com.goxviet.ime.lastUpdateCheck"
        static let lastNotifiedUpdateVersion = "com.goxviet.ime.lastNotifiedUpdateVersion"
        static let hideFromDock = "com.goxviet.ime.hideFromDock"
    }

    // MARK: - Initialization

    private init() {
        // Set defaults if not previously configured
        registerDefaults()
        
        // On first launch, Vietnamese input is disabled by default
        // This ensures users explicitly enable it rather than being surprised by transforms
        if !UserDefaults.standard.bool(forKey: Keys.hasLaunchedBefore) {
            UserDefaults.standard.set(true, forKey: Keys.hasLaunchedBefore)
            isEnabled = false
            Log.info("First launch detected - Vietnamese input disabled by default")
        }
    }

    private func registerDefaults() {
        let defaults: [String: Any] = [
            Keys.smartModeEnabled: true,
            Keys.inputMethod: 0,  // Telex
            Keys.modernToneStyle: false,
            Keys.escRestore: true,
            Keys.freeTone: false,
            Keys.autoDisableNonLatin: true,  // Default: enabled for better UX with multilingual users
            Keys.autoUpdateCheck: true,
            Keys.autoUpdateInstall: false,
            Keys.hideFromDock: true  // Default: hide from dock (menubar-only)
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
    /// Returns false (disabled/English) by default if no specific setting exists
    /// Only apps with Vietnamese ENABLED are stored
    func getPerAppMode(bundleId: String) -> Bool {
        guard let dict = UserDefaults.standard.dictionary(forKey: Keys.perAppModes) as? [String: Bool] else {
            return false  // Default: disabled (English mode)
        }

        // Return stored value, or false if not found (default: English)
        return dict[bundleId] ?? false
    }

    /// Save the mode for a specific app
    /// Only stores apps with Vietnamese ENABLED (default is disabled/English)
    /// Enforces MAX_PER_APP_ENTRIES limit to prevent unbounded memory growth
    func setPerAppMode(bundleId: String, enabled: Bool) {
        var dict = UserDefaults.standard.dictionary(forKey: Keys.perAppModes) as? [String: Bool] ?? [:]

        if !enabled {
            // Remove from dictionary if disabled (default state = English)
            dict.removeValue(forKey: bundleId)
            // Also remove from known apps list
            removeKnownApp(bundleId: bundleId)
        } else {
            // Check capacity limit before adding new entry
            if dict[bundleId] == nil && dict.count >= MAX_PER_APP_ENTRIES {
                Log.warning("Per-app settings at capacity (\(MAX_PER_APP_ENTRIES)). Not saving new entry for: \(bundleId)")
                Log.warning("Consider clearing old per-app settings from Preferences.")
                return
            }

            // Store only Vietnamese-enabled apps
            dict[bundleId] = true
            // Auto-record as known app for Saved Applications UI
            recordKnownApp(bundleId: bundleId)
        }

        UserDefaults.standard.set(dict, forKey: Keys.perAppModes)

        Log.info("Per-app mode saved: \(bundleId) = \(enabled ? "Vietnamese" : "English")")
    }

    /// Clear per-app settings for a specific app (reset to default)
    func clearPerAppMode(bundleId: String) {
        var dict = UserDefaults.standard.dictionary(forKey: Keys.perAppModes) as? [String: Bool] ?? [:]
        dict.removeValue(forKey: bundleId)
        UserDefaults.standard.set(dict, forKey: Keys.perAppModes)

        // Also remove from known apps list so UI no longer shows it as "Saved"
        removeKnownApp(bundleId: bundleId)

        Log.info("Per-app mode cleared: \(bundleId)")
    }

    /// Get all per-app settings
    func getAllPerAppModes() -> [String: Bool] {
        return UserDefaults.standard.dictionary(forKey: Keys.perAppModes) as? [String: Bool] ?? [:]
    }

    /// Clear all per-app settings
    func clearAllPerAppModes() {
        UserDefaults.standard.removeObject(forKey: Keys.perAppModes)
        UserDefaults.standard.removeObject(forKey: Keys.knownApps)
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

    // MARK: - Known Apps Tracking (for UI)

    /// Record an app as "known" so the UI can show it in Saved Applications.
    /// Only called when Vietnamese is ENABLED for an app.
    ///
    /// This is bounded by MAX_PER_APP_ENTRIES to avoid unbounded storage growth.
    func recordKnownApp(bundleId: String) {
        guard !bundleId.isEmpty else { return }

        var known = UserDefaults.standard.array(forKey: Keys.knownApps) as? [String] ?? []

        // Already tracked
        if known.contains(bundleId) { return }

        // Enforce capacity
        if known.count >= MAX_PER_APP_ENTRIES {
            Log.warning("Known apps at capacity (\(MAX_PER_APP_ENTRIES)). Not recording: \(bundleId)")
            return
        }

        known.append(bundleId)
        UserDefaults.standard.set(known, forKey: Keys.knownApps)
    }

    /// Remove an app from the known list (UI will no longer show it as saved)
    func removeKnownApp(bundleId: String) {
        var known = UserDefaults.standard.array(forKey: Keys.knownApps) as? [String] ?? []
        guard let idx = known.firstIndex(of: bundleId) else { return }
        known.remove(at: idx)
        UserDefaults.standard.set(known, forKey: Keys.knownApps)
    }

    /// Get all known apps (bundle IDs) for Saved Applications UI
    func getKnownApps() -> [String] {
        return UserDefaults.standard.array(forKey: Keys.knownApps) as? [String] ?? []
    }

    /// Get a map of known apps -> effective enabled state (true/false)
    /// Effective state is computed from per-app overrides (default: disabled/English).
    func getKnownAppsWithStates() -> [String: Bool] {
        let known = getKnownApps()
        guard !known.isEmpty else { return [:] }

        var result: [String: Bool] = [:]
        for bundleId in known {
            result[bundleId] = getPerAppMode(bundleId: bundleId)
        }
        return result
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
    static let smartModeChanged = Notification.Name("smartModeChanged")
    static let inputMethodChanged = Notification.Name("inputMethodChanged")
    static let toneStyleChanged = Notification.Name("toneStyleChanged")
    static let showUpdateWindow = Notification.Name("showUpdateWindow")
    static let openUpdateWindow = Notification.Name("openUpdateWindow")
    static let openSettingsWindow = Notification.Name("openSettingsWindow")
}
