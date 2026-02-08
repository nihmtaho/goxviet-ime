//
//  SettingsManager.swift
//  GoxViet
//
//  Unified settings manager for centralized state management
//  Synchronizes between AppStorage, AppState, and Rust core
//

import Foundation
import SwiftUI
import Combine

/// Centralized settings management with validation and synchronization
final class SettingsManager: ObservableObject {
    
    // MARK: - Singleton
    
    static let shared = SettingsManager()
    
    // MARK: - Published Settings
    
    @Published var inputMethod: Int = 0 {
        didSet {
            saveToDefaults(Keys.inputMethod, value: inputMethod)
            syncToCore()
            postNotification(.inputMethodChanged, value: inputMethod)
        }
    }
    
    @Published var modernToneStyle: Bool = false {
        didSet {
            saveToDefaults(Keys.modernToneStyle, value: modernToneStyle)
            syncToCore()
            postNotification(.toneStyleChanged, value: modernToneStyle)
        }
    }
    
    @Published var escRestoreEnabled: Bool = true {
        didSet {
            saveToDefaults(Keys.escRestoreEnabled, value: escRestoreEnabled)
            syncToCore()
            postNotification(.escRestoreChanged, value: escRestoreEnabled)
        }
    }
    
    @Published var freeToneEnabled: Bool = false {
        didSet {
            saveToDefaults(Keys.freeToneEnabled, value: freeToneEnabled)
            syncToCore()
            postNotification(.freeToneChanged, value: freeToneEnabled)
        }
    }
    
    @Published var instantRestoreEnabled: Bool = true {
        didSet {
            saveToDefaults(Keys.instantRestoreEnabled, value: instantRestoreEnabled)
            syncToCore()
            postNotification(.instantRestoreChanged, value: instantRestoreEnabled)
        }
    }
    
    @Published var smartModeEnabled: Bool = true {
        didSet {
            saveToDefaults(Keys.smartModeEnabled, value: smartModeEnabled)
            postNotification(.smartModeChanged, value: smartModeEnabled)
        }
    }
    
    @Published var autoDisableForNonLatin: Bool = true {
        didSet {
            saveToDefaults(Keys.autoDisableForNonLatin, value: autoDisableForNonLatin)
        }
    }
    
    @Published var hideFromDock: Bool = false {
        didSet {
            saveToDefaults(Keys.hideFromDock, value: hideFromDock)
            // Use legacy notification for now (hideFromDockChanged not in TypedNotifications yet)
            NotificationCenter.default.post(name: NSNotification.Name("hideFromDockChanged"), object: hideFromDock)
        }
    }
    
    // MARK: - Phase 2.9 Settings
    
    @Published var outputEncoding: OutputEncoding = .unicode {
        didSet {
            saveToDefaults(Keys.outputEncoding, value: outputEncoding.rawValue)
            syncToCore()
            postNotification(.outputEncodingChanged, value: outputEncoding)
            Log.info("Output encoding changed to: \(outputEncoding.shortName)")
        }
    }
    
    @Published var shiftBackspaceEnabled: Bool = false {
        didSet {
            saveToDefaults(Keys.shiftBackspaceEnabled, value: shiftBackspaceEnabled)
            postNotification(.shiftBackspaceEnabledChanged, value: shiftBackspaceEnabled)
            Log.info("Shift+Backspace \(shiftBackspaceEnabled ? "enabled" : "disabled")")
        }
    }
    
    @Published var textExpansionEnabled: Bool = true {
        didSet {
            saveToDefaults(Keys.textExpansionEnabled, value: textExpansionEnabled)
            postNotification(.textExpansionEnabledChanged, value: textExpansionEnabled)
            Log.info("Text Expansion \(textExpansionEnabled ? "enabled" : "disabled")")
        }
    }
    
    @Published var shortcutsLoaded: Bool = false
    
    /// Text expansion shortcuts - UserDefaults is source of truth
    /// Auto-synced to Rust engine via Combine
    @Published var shortcuts: [TextShortcutItem] = [] {
        didSet {
            // Auto-save to UserDefaults when shortcuts change
            if let data = try? JSONEncoder().encode(shortcuts) {
                userDefaults.set(data, forKey: Keys.shortcuts)
            }
        }
    }
    
    // MARK: - Runtime State (Not Persisted)
    
    /// Whether Vietnamese input is currently enabled (runtime state, not persisted)
    @Published private(set) var isEnabled: Bool = false
    
    /// Debounce work item for setEnabled notifications
    private var setEnabledDebounceWork: DispatchWorkItem?
    
    // MARK: - Private Properties
    
    private let userDefaults = UserDefaults.standard
    private let lock = NSRecursiveLock()
    private var cancellables = Set<AnyCancellable>()
    
    // Keys for UserDefaults
    private enum Keys {
        static let inputMethod = "inputMethod"
        static let modernToneStyle = "modernToneStyle"
        static let escRestoreEnabled = "escRestoreEnabled"
        static let freeToneEnabled = "freeToneEnabled"
        static let instantRestoreEnabled = "instantRestoreEnabled"
        static let smartModeEnabled = "smartModeEnabled"
        static let autoDisableForNonLatin = "com.goxviet.ime.autoDisableNonLatin"
        static let hideFromDock = "com.goxviet.ime.hideFromDock"
        
        // Phase 2.9 keys
        static let outputEncoding = "com.goxviet.ime.outputEncoding"
        static let shiftBackspaceEnabled = "com.goxviet.ime.shiftBackspaceEnabled"
        static let textExpansionEnabled = "com.goxviet.ime.textExpansionEnabled"
        static let shortcuts = "com.goxviet.ime.shortcuts"
    }
    
    // MARK: - Initialization
    
    private init() {
        loadFromDefaults()
        setupObservers()
        Log.info("SettingsManager initialized")
    }
    
    // MARK: - Public API
    
    /// Update input method (0 = Telex, 1 = VNI)
    func setInputMethod(_ method: Int) {
        lock.lock()
        defer { lock.unlock() }
        
        guard method == 0 || method == 1 else {
            Log.error("Invalid input method: \(method)")
            return
        }
        
        guard method != inputMethod else { return }
        
        inputMethod = method
        saveToDefaults(Keys.inputMethod, value: method)
        Log.info("Input method changed to: \(method == 0 ? "Telex" : "VNI")")
    }
    
    /// Toggle modern tone style
    func setModernToneStyle(_ modern: Bool) {
        lock.lock()
        defer { lock.unlock() }
        
        guard modern != modernToneStyle else { return }
        
        modernToneStyle = modern
        saveToDefaults(Keys.modernToneStyle, value: modern)
        Log.info("Tone style changed to: \(modern ? "Modern" : "Traditional")")
    }
    
    /// Toggle ESC restore
    func setEscRestoreEnabled(_ enabled: Bool) {
        lock.lock()
        defer { lock.unlock() }
        
        guard enabled != escRestoreEnabled else { return }
        
        escRestoreEnabled = enabled
        saveToDefaults(Keys.escRestoreEnabled, value: enabled)
    }
    
    /// Toggle free tone marking
    func setFreeToneEnabled(_ enabled: Bool) {
        lock.lock()
        defer { lock.unlock() }
        
        guard enabled != freeToneEnabled else { return }
        
        freeToneEnabled = enabled
        saveToDefaults(Keys.freeToneEnabled, value: enabled)
    }
    
    /// Toggle instant auto-restore
    func setInstantRestoreEnabled(_ enabled: Bool) {
        lock.lock()
        defer { lock.unlock() }
        
        guard enabled != instantRestoreEnabled else { return }
        
        instantRestoreEnabled = enabled
        saveToDefaults(Keys.instantRestoreEnabled, value: enabled)
    }
    
    /// Toggle smart mode
    func setSmartModeEnabled(_ enabled: Bool) {
        lock.lock()
        defer { lock.unlock() }
        
        guard enabled != smartModeEnabled else { return }
        
        smartModeEnabled = enabled
        saveToDefaults(Keys.smartModeEnabled, value: enabled)
    }
    
    /// Toggle auto-disable for non-Latin apps
    func setAutoDisableForNonLatin(_ enabled: Bool) {
        lock.lock()
        defer { lock.unlock() }
        
        guard enabled != autoDisableForNonLatin else { return }
        
        autoDisableForNonLatin = enabled
        saveToDefaults(Keys.autoDisableForNonLatin, value: enabled)
    }
    
    /// Toggle hide from dock
    func setHideFromDock(_ hide: Bool) {
        lock.lock()
        defer { lock.unlock() }
        
        guard hide != hideFromDock else { return }
        
        hideFromDock = hide
        saveToDefaults(Keys.hideFromDock, value: hide)
        Log.info("Hide from dock: \(hide)")
    }
    
    /// Reset all settings to defaults
    func resetToDefaults() {
        lock.lock()
        defer { lock.unlock() }
        
        inputMethod = 0
        modernToneStyle = false
        escRestoreEnabled = true
        freeToneEnabled = false
        instantRestoreEnabled = true
        smartModeEnabled = true
        autoDisableForNonLatin = true
        hideFromDock = true  // Default: hide from dock (menubar-only)
        outputEncoding = .unicode
        shiftBackspaceEnabled = false
        textExpansionEnabled = true
        
        // Save to defaults
        saveAllToDefaults()
        
        Log.info("All settings reset to defaults")
    }
    
    // MARK: - Runtime State Management
    
    /// Set enabled state and notify observers
    /// Debounced to reduce overhead during rapid toggles
    func setEnabled(_ enabled: Bool) {
        lock.lock()
        defer { lock.unlock() }
        
        guard enabled != isEnabled else { return }
        
        isEnabled = enabled
        
        // Cancel pending debounce work
        setEnabledDebounceWork?.cancel()
        
        // Create new debounced notification (50ms delay)
        let work = DispatchWorkItem { [weak self] in
            guard let self = self else { return }
            
            // Post notification for UI update
            NotificationCenter.default.post(
                name: .updateStateChanged,
                object: enabled
            )
            
            Log.info("Gõ Việt input: \(enabled ? "enabled" : "disabled")")
        }
        
        setEnabledDebounceWork = work
        DispatchQueue.main.asyncAfter(deadline: .now() + 0.05, execute: work)
    }
    
    /// Set enabled state without posting notification (used during app switching)
    func setEnabledSilently(_ enabled: Bool) {
        lock.lock()
        defer { lock.unlock() }
        
        isEnabled = enabled
    }
    
    // MARK: - Per-App Mode Management
    
    /// Maximum number of per-app settings to store (prevents unbounded memory growth)
    private static let MAX_PER_APP_ENTRIES = 100
    private static let perAppModesKey = "com.goxviet.ime.perAppModes"
    private static let knownAppsKey = "com.goxviet.ime.knownApps"
    
    /// Get the saved mode for a specific app
    /// Returns false (disabled/English) by default if no specific setting exists
    func getPerAppMode(bundleId: String) -> Bool {
        lock.lock()
        defer { lock.unlock() }
        
        guard let dict = userDefaults.dictionary(forKey: Self.perAppModesKey) as? [String: Bool] else {
            return false  // Default: disabled (English mode)
        }
        
        return dict[bundleId] ?? false
    }
    
    /// Save the mode for a specific app
    func setPerAppMode(bundleId: String, enabled: Bool) {
        lock.lock()
        defer { lock.unlock() }
        
        var dict = userDefaults.dictionary(forKey: Self.perAppModesKey) as? [String: Bool] ?? [:]
        
        let isNewApp = (dict[bundleId] == nil)
        
        // If this is a new app and the user has not enabled Vietnamese yet,
        // do NOT create a tracking entry. Keep default = disabled without persisting.
        if isNewApp && enabled == false {
            Log.info("Per-app skip save (new app, still English): \(bundleId)")
            return
        }
        
        // For a new app with enabled == true, enforce capacity before saving
        if isNewApp {
            if dict.count >= Self.MAX_PER_APP_ENTRIES {
                Log.warning("Per-app settings at capacity (\(Self.MAX_PER_APP_ENTRIES)). Not saving new entry for: \(bundleId)")
                return
            }
        }
        
        // Store/update the state
        dict[bundleId] = enabled
        
        // Record as known app only when Vietnamese is enabled at least once
        if enabled {
            recordKnownApp(bundleId: bundleId)
        }
        
        userDefaults.set(dict, forKey: Self.perAppModesKey)
        
        Log.info("Per-app mode saved: \(bundleId) = \(enabled ? "Vietnamese" : "English")")
        postNotification(.perAppModesChanged, value: nil)
    }
    
    /// Clear per-app settings for a specific app
    func clearPerAppMode(bundleId: String) {
        lock.lock()
        defer { lock.unlock() }
        
        var dict = userDefaults.dictionary(forKey: Self.perAppModesKey) as? [String: Bool] ?? [:]
        dict.removeValue(forKey: bundleId)
        userDefaults.set(dict, forKey: Self.perAppModesKey)
        
        removeKnownApp(bundleId: bundleId)
        
        Log.info("Per-app mode cleared: \(bundleId)")
        postNotification(.perAppModesChanged, value: nil)
    }
    
    /// Get all per-app settings
    func getAllPerAppModes() -> [String: Bool] {
        lock.lock()
        defer { lock.unlock() }
        
        return userDefaults.dictionary(forKey: Self.perAppModesKey) as? [String: Bool] ?? [:]
    }
    
    /// Clear all per-app settings
    func clearAllPerAppModes() {
        lock.lock()
        defer { lock.unlock() }
        
        userDefaults.removeObject(forKey: Self.perAppModesKey)
        userDefaults.removeObject(forKey: Self.knownAppsKey)
        Log.info("All per-app modes cleared")
        postNotification(.perAppModesChanged, value: nil)
    }
    
    /// Get count of stored per-app settings
    func getPerAppModesCount() -> Int {
        lock.lock()
        defer { lock.unlock() }
        
        let dict = userDefaults.dictionary(forKey: Self.perAppModesKey) as? [String: Bool] ?? [:]
        return dict.count
    }
    
    /// Check if per-app settings is at capacity
    func isPerAppModesAtCapacity() -> Bool {
        return getPerAppModesCount() >= Self.MAX_PER_APP_ENTRIES
    }
    
    // MARK: - Known Apps Tracking
    
    private func recordKnownApp(bundleId: String) {
        guard !bundleId.isEmpty else { return }
        
        var known = userDefaults.array(forKey: Self.knownAppsKey) as? [String] ?? []
        
        if known.contains(bundleId) { return }
        
        if known.count >= Self.MAX_PER_APP_ENTRIES {
            Log.warning("Known apps at capacity (\(Self.MAX_PER_APP_ENTRIES)). Not recording: \(bundleId)")
            return
        }
        
        known.append(bundleId)
        userDefaults.set(known, forKey: Self.knownAppsKey)
        postNotification(.perAppModesChanged, value: nil)
    }
    
    private func removeKnownApp(bundleId: String) {
        var known = userDefaults.array(forKey: Self.knownAppsKey) as? [String] ?? []
        guard let idx = known.firstIndex(of: bundleId) else { return }
        known.remove(at: idx)
        userDefaults.set(known, forKey: Self.knownAppsKey)
        postNotification(.perAppModesChanged, value: nil)
    }
    
    /// Get all known apps (bundle IDs)
    func getKnownApps() -> [String] {
        lock.lock()
        defer { lock.unlock() }
        
        return userDefaults.array(forKey: Self.knownAppsKey) as? [String] ?? []
    }
    
    /// Get a map of known apps -> effective enabled state
    func getKnownAppsWithStates() -> [String: Bool] {
        lock.lock()
        defer { lock.unlock() }
        
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
    
    // MARK: - Shortcut Persistence
    
    private var shortcutsFileURL: URL? {
        guard let appSupportURL = FileManager.default.urls(for: .applicationSupportDirectory, in: .userDomainMask).first else {
            return nil
        }
        let dir = appSupportURL.appendingPathComponent("GoxViet")
        return dir.appendingPathComponent("shortcuts.json")
    }

    /// Legacy method - now auto-saved via didSet
    /// Kept for API compatibility
    public func saveShortcuts() {
        // Shortcuts are auto-saved via didSet on shortcuts property
        // and auto-synced to engine via Combine
        Log.info("saveShortcuts() called - now auto-saved")
    }

    /// Load shortcuts from UserDefaults (source of truth)
    /// Initializes with defaults if no saved data exists
    private func loadShortcutsFromDefaults() {
        if let data = userDefaults.data(forKey: Keys.shortcuts),
           let saved = try? JSONDecoder().decode([TextShortcutItem].self, from: data) {
            shortcuts = saved
            Log.info("Loaded \(shortcuts.count) shortcuts from UserDefaults")
        } else {
            // Initialize with default shortcuts
            shortcuts = [
                TextShortcutItem(key: "vn", value: "Việt Nam", isEnabled: false),
                TextShortcutItem(key: "hn", value: "Hà Nội", isEnabled: false),
                TextShortcutItem(key: "hcm", value: "Hồ Chí Minh", isEnabled: false),
                TextShortcutItem(key: "tphcm", value: "Thành phố Hồ Chí Minh", isEnabled: false),
            ]
            Log.info("Initialized default shortcuts")
        }
        
        // Sync to engine after loading
        DispatchQueue.main.async { [weak self] in
            self?.syncShortcutsToEngine()
            self?.shortcutsLoaded = true
        }
    }

    // MARK: - Settings Import/Export
    
    /// Export settings to dictionary
    func exportSettings() -> [String: Any] {
        lock.lock()
        defer { lock.unlock() }
        
        return [
            "inputMethod": inputMethod,
            "modernToneStyle": modernToneStyle,
            "escRestoreEnabled": escRestoreEnabled,
            "freeToneEnabled": freeToneEnabled,
            "instantRestoreEnabled": instantRestoreEnabled,
            "smartModeEnabled": smartModeEnabled,
            "autoDisableForNonLatin": autoDisableForNonLatin,
            "hideFromDock": hideFromDock,
            "outputEncoding": outputEncoding.rawValue,
            "shiftBackspaceEnabled": shiftBackspaceEnabled,
            "exportedAt": ISO8601DateFormatter().string(from: Date())
        ]
    }
    
    /// Import settings from dictionary
    func importSettings(_ settings: [String: Any]) {
        lock.lock()
        defer { lock.unlock() }
        
        if let method = settings["inputMethod"] as? Int {
            inputMethod = method
        }
        if let modern = settings["modernToneStyle"] as? Bool {
            modernToneStyle = modern
        }
        if let enabled = settings["escRestoreEnabled"] as? Bool {
            escRestoreEnabled = enabled
        }
        if let enabled = settings["freeToneEnabled"] as? Bool {
            freeToneEnabled = enabled
        }
        if let enabled = settings["instantRestoreEnabled"] as? Bool {
            instantRestoreEnabled = enabled
        }
        if let enabled = settings["smartModeEnabled"] as? Bool {
            smartModeEnabled = enabled
        }
        if let enabled = settings["autoDisableForNonLatin"] as? Bool {
            autoDisableForNonLatin = enabled
        }
        if let hide = settings["hideFromDock"] as? Bool {
            hideFromDock = hide
        }
        if let encodingValue = settings["outputEncoding"] as? Int,
           let encoding = OutputEncoding(rawValue: encodingValue) {
            outputEncoding = encoding
        }
        if let enabled = settings["shiftBackspaceEnabled"] as? Bool {
            shiftBackspaceEnabled = enabled
        }
        
        saveAllToDefaults()
        syncToCore()
        
        Log.info("Settings imported successfully")
    }
    
    // MARK: - Private Helpers
    
    private func loadFromDefaults() {
        // Check if this is first launch
        let hasLaunchedBefore = userDefaults.bool(forKey: "hasLaunchedBefore")
        
        if !hasLaunchedBefore {
            // First launch: Register default values without triggering didSet
            let defaults: [String: Any] = [
                Keys.inputMethod: 0,
                Keys.modernToneStyle: false,
                Keys.escRestoreEnabled: true,
                Keys.freeToneEnabled: false,
                Keys.instantRestoreEnabled: true,
                Keys.smartModeEnabled: true,
                Keys.autoDisableForNonLatin: true,
                Keys.hideFromDock: true,
                Keys.outputEncoding: OutputEncoding.unicode.rawValue,
                Keys.shiftBackspaceEnabled: false,
                Keys.textExpansionEnabled: true,
                Keys.shortcuts: try? JSONEncoder().encode([
                    TextShortcutItem(key: "vn", value: "Việt Nam", isEnabled: false),
                    TextShortcutItem(key: "hn", value: "Hà Nội", isEnabled: false),
                    TextShortcutItem(key: "hcm", value: "Hồ Chí Minh", isEnabled: false),
                    TextShortcutItem(key: "tphcm", value: "Thành phố Hồ Chí Minh", isEnabled: false)
                ]),
                "hasLaunchedBefore": true
            ]
            userDefaults.register(defaults: defaults)
            
            // Set values directly without triggering didSet (avoiding double save)
            // This ensures proper initialization on first launch
            Log.info("First launch: registering default settings")
        }
        
        // Load from UserDefaults (will use registered defaults if keys don't exist)
        inputMethod = userDefaults.integer(forKey: Keys.inputMethod)
        modernToneStyle = userDefaults.bool(forKey: Keys.modernToneStyle)
        escRestoreEnabled = userDefaults.bool(forKey: Keys.escRestoreEnabled)
        freeToneEnabled = userDefaults.bool(forKey: Keys.freeToneEnabled)
        instantRestoreEnabled = userDefaults.bool(forKey: Keys.instantRestoreEnabled)
        smartModeEnabled = userDefaults.bool(forKey: Keys.smartModeEnabled)
        autoDisableForNonLatin = userDefaults.bool(forKey: Keys.autoDisableForNonLatin)
        hideFromDock = userDefaults.bool(forKey: Keys.hideFromDock)
        
        // Phase 2.9 settings
        if let encoding = OutputEncoding(rawValue: userDefaults.integer(forKey: Keys.outputEncoding)) {
            outputEncoding = encoding
        }
        shiftBackspaceEnabled = userDefaults.bool(forKey: Keys.shiftBackspaceEnabled)
        textExpansionEnabled = userDefaults.bool(forKey: Keys.textExpansionEnabled)
        
        // Load shortcuts from UserDefaults (source of truth)
        loadShortcutsFromDefaults()

        // Mark as launched after loading
        if !hasLaunchedBefore {
            userDefaults.set(true, forKey: "hasLaunchedBefore")
            // Explicitly save all loaded defaults to ensure persistence
            saveAllToDefaults()
            Log.info("First launch defaults saved to UserDefaults")
        }
    }
    
    private func saveToDefaults(_ key: String, value: Any) {
        userDefaults.set(value, forKey: key)
    }
    
    private func saveAllToDefaults() {
        saveToDefaults(Keys.inputMethod, value: inputMethod)
        saveToDefaults(Keys.modernToneStyle, value: modernToneStyle)
        saveToDefaults(Keys.escRestoreEnabled, value: escRestoreEnabled)
        saveToDefaults(Keys.freeToneEnabled, value: freeToneEnabled)
        saveToDefaults(Keys.instantRestoreEnabled, value: instantRestoreEnabled)
        saveToDefaults(Keys.smartModeEnabled, value: smartModeEnabled)
        saveToDefaults(Keys.autoDisableForNonLatin, value: autoDisableForNonLatin)
        saveToDefaults(Keys.hideFromDock, value: hideFromDock)
        saveToDefaults(Keys.outputEncoding, value: outputEncoding.rawValue)
        saveToDefaults(Keys.shiftBackspaceEnabled, value: shiftBackspaceEnabled)
    }
    
    private func syncToCore() {
        // Sync to Rust core via RustBridgeSafe
        let bridge = RustBridgeSafe.shared
        
        // Ensure bridge is initialized
        _ = bridge.initialize()
        
        // Sync all settings to core
        let results = [
            bridge.setMethod(inputMethod),
            bridge.setModernTone(modernToneStyle),
            bridge.setEscRestore(escRestoreEnabled),
            bridge.setFreeTone(freeToneEnabled),
            bridge.setInstantRestore(instantRestoreEnabled)
        ]
        
        // Log any failures
        for (index, result) in results.enumerated() {
            if case .failure(let error) = result {
                Log.error("Failed to sync setting \(index) to core: \(error.localizedDescription)")
            }
        }
    }
    
    private func postNotification(_ name: Notification.Name, value: Any?) {
        DispatchQueue.main.async {
            NotificationCenter.default.post(name: name, object: value)
        }
    }
    
    private func setupObservers() {
        // Observe external changes (from settings window, menu, etc.)
        NotificationCenter.default.publisher(for: .inputMethodChanged)
            .compactMap { $0.object as? Int }
            .sink { [weak self] method in
                self?.setInputMethod(method)
            }
            .store(in: &cancellables)
        
        NotificationCenter.default.publisher(for: .toneStyleChanged)
            .compactMap { $0.object as? Bool }
            .sink { [weak self] modern in
                self?.setModernToneStyle(modern)
            }
            .store(in: &cancellables)
        
        NotificationCenter.default.publisher(for: .smartModeChanged)
            .compactMap { $0.object as? Bool }
            .sink { [weak self] enabled in
                self?.setSmartModeEnabled(enabled)
            }
            .store(in: &cancellables)
        
        // Auto-sync shortcuts to engine with 300ms debounce
        $shortcuts
            .dropFirst()  // Skip initial empty array
            .debounce(for: .milliseconds(300), scheduler: RunLoop.main)
            .sink { [weak self] _ in
                self?.syncShortcutsToEngine()
            }
            .store(in: &cancellables)
    }
    
    /// Sync shortcuts from UI to Rust engine
    /// UI is source of truth, engine receives updates
    func syncShortcutsToEngine() {
        // Clear and re-add all enabled shortcuts
        RustBridge.shared.clearShortcuts()
        for shortcut in shortcuts where shortcut.isEnabled && !shortcut.key.isEmpty {
            RustBridge.shared.addShortcut(trigger: shortcut.key, replacement: shortcut.value)
        }
        Log.info("Synced \(shortcuts.filter { $0.isEnabled }.count) shortcuts to engine")
    }
}

// MARK: - Text Shortcut Model

/// Text expansion shortcut item
/// Stored in UserDefaults as source of truth
struct TextShortcutItem: Identifiable, Codable, Equatable {
    var id = UUID()
    var key: String
    var value: String
    var isEnabled: Bool = true
    
    static func == (lhs: TextShortcutItem, rhs: TextShortcutItem) -> Bool {
        lhs.id == rhs.id && lhs.key == rhs.key && lhs.value == rhs.value && lhs.isEnabled == rhs.isEnabled
    }
}

