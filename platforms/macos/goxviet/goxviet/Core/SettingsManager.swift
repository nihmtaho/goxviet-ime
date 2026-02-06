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
            RustBridge.shared.setShortcutsEnabled(textExpansionEnabled)
            postNotification(.textExpansionEnabledChanged, value: textExpansionEnabled)
            Log.info("Text Expansion \(textExpansionEnabled ? "enabled" : "disabled")")
        }
    }
    
    // MARK: - Shortcuts (Text Expansion)
    
    /// Source of truth for shortcuts - stored in UserDefaults and synced to Rust engine
    @Published var shortcuts: [TextShortcutItem] = []
    
    /// Track if shortcuts have been loaded from storage
    @Published var shortcutsLoaded: Bool = false
    
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
        
        // Shortcuts key
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
    
    // MARK: - Shortcuts Management
    
    /// Add a new shortcut
    func addShortcut(trigger: String, replacement: String) -> Bool {
        lock.lock()
        defer { lock.unlock() }
        
        let trimmedTrigger = trigger.trimmingCharacters(in: .whitespaces)
        guard !trimmedTrigger.isEmpty && !replacement.isEmpty else { return false }
        
        // Check for duplicates
        if shortcuts.contains(where: { $0.trigger == trimmedTrigger }) {
            Log.warning("Shortcut '\(trimmedTrigger)' already exists")
            return false
        }
        
        let newShortcut = TextShortcutItem(trigger: trimmedTrigger, replacement: replacement, isEnabled: true)
        shortcuts.append(newShortcut)
        Log.info("Added shortcut: \(trimmedTrigger) → \(replacement)")
        return true
    }
    
    /// Update an existing shortcut
    func updateShortcut(oldTrigger: String, newTrigger: String, replacement: String) -> Bool {
        lock.lock()
        defer { lock.unlock() }
        
        let trimmedNewTrigger = newTrigger.trimmingCharacters(in: .whitespaces)
        guard !trimmedNewTrigger.isEmpty && !replacement.isEmpty else { return false }
        
        // Remove old
        shortcuts.removeAll { $0.trigger == oldTrigger }
        
        // Add new
        let updatedShortcut = TextShortcutItem(trigger: trimmedNewTrigger, replacement: replacement, isEnabled: true)
        shortcuts.append(updatedShortcut)
        Log.info("Updated shortcut: \(oldTrigger) → \(trimmedNewTrigger): \(replacement)")
        return true
    }
    
    /// Remove a shortcut by trigger
    func removeShortcut(trigger: String) {
        lock.lock()
        defer { lock.unlock() }
        
        shortcuts.removeAll { $0.trigger == trigger }
        Log.info("Removed shortcut: \(trigger)")
    }
    
    /// Get all shortcuts as array of tuples for RustBridge
    func getShortcutsForEngine() -> [(key: String, value: String, enabled: Bool)] {
        lock.lock()
        defer { lock.unlock() }
        
        return shortcuts.map { ($0.trigger, $0.replacement, $0.isEnabled) }
    }
    
    /// Sync all enabled shortcuts to Rust engine
    func syncShortcutsToEngine() {
        let enabledShortcuts = getShortcutsForEngine().filter { $0.enabled }
        RustBridge.shared.syncShortcuts(enabledShortcuts)
        Log.info("Synced \(enabledShortcuts.count) enabled shortcuts to engine")
    }
    
    /// Import shortcuts from custom format
    /// Format: "trigger:replacement" per line, or ";comment" for comments
    func importShortcuts(from content: String) -> Int {
        lock.lock()
        defer { lock.unlock() }
        
        let lines = content.components(separatedBy: .newlines)
        var imported = 0
        
        for line in lines {
            let trimmed = line.trimmingCharacters(in: .whitespaces)
            guard !trimmed.isEmpty, !trimmed.hasPrefix(";"),
                  let colonIndex = trimmed.firstIndex(of: ":") else { continue }
            
            let trigger = String(trimmed[..<colonIndex]).trimmingCharacters(in: .whitespaces)
            let replacement = String(trimmed[trimmed.index(after: colonIndex)...]).trimmingCharacters(in: .whitespaces)
            
            guard !trigger.isEmpty else { continue }
            
            // Update existing or add new
            if let idx = shortcuts.firstIndex(where: { $0.trigger == trigger }) {
                shortcuts[idx].replacement = replacement
                shortcuts[idx].isEnabled = true
            } else {
                shortcuts.append(TextShortcutItem(trigger: trigger, replacement: replacement, isEnabled: true))
            }
            imported += 1
        }
        
        Log.info("Imported \(imported) shortcuts")
        return imported
    }
    
    /// Export shortcuts to custom format
    func exportShortcutsToString() -> String {
        lock.lock()
        defer { lock.unlock() }
        
        var lines = [";Gõ Việt - Bảng gõ tắt"]
        for shortcut in shortcuts where !shortcut.trigger.isEmpty {
            lines.append("\(shortcut.trigger):\(shortcut.replacement)")
        }
        return lines.joined(separator: "\n")
    }
    
    // MARK: - Private Shortcuts Helpers
    
    private func loadShortcutsFromDefaults() {
        if let data = userDefaults.data(forKey: Keys.shortcuts),
           let saved = try? JSONDecoder().decode([TextShortcutItem].self, from: data) {
            shortcuts = saved
            Log.info("Loaded \(shortcuts.count) shortcuts from UserDefaults")
        } else {
            // No saved shortcuts - start with empty array
            shortcuts = []
            Log.info("No saved shortcuts found, starting fresh")
        }
        
        // Sync to engine (will be called again when engine is ready via InputManager)
        syncShortcutsToEngine()
        
        // Mark as loaded
        DispatchQueue.main.async {
            self.shortcutsLoaded = true
        }
    }
    
    private func setupShortcutsObserver() {
        // Auto-save and sync to engine when shortcuts change
        $shortcuts
            .dropFirst() // Skip initial value
            .debounce(for: .milliseconds(300), scheduler: RunLoop.main)
            .sink { [weak self] _ in
                guard let self = self else { return }
                
                // Save to UserDefaults
                if let data = try? JSONEncoder().encode(self.shortcuts) {
                    self.userDefaults.set(data, forKey: Keys.shortcuts)
                }
                
                // Sync to Rust engine
                self.syncShortcutsToEngine()
                
                // Post notification
                DispatchQueue.main.async {
                    NotificationCenter.default.post(name: .didSaveShortcuts, object: Date())
                }
            }
            .store(in: &cancellables)
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
        // Cancel any existing subscriptions to prevent memory leaks
        // This is critical because SettingsManager is a singleton
        cancellables.removeAll()
        
        // Setup shortcuts auto-save and sync
        setupShortcutsObserver()
        
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
    }
    
    /// Clean up all observers and subscriptions
    /// Call this when app is terminating to prevent memory leaks
    func cleanup() {
        lock.lock()
        defer { lock.unlock() }
        
        // Cancel all Combine subscriptions
        cancellables.removeAll()
        
        // Cancel pending debounce work
        setEnabledDebounceWork?.cancel()
        setEnabledDebounceWork = nil
        
        Log.info("SettingsManager cleaned up")
    }
}

// MARK: - Text Shortcut Item Model

/// Represents a single text expansion shortcut
struct TextShortcutItem: Identifiable, Codable, Equatable {
    let id: UUID
    var trigger: String
    var replacement: String
    var isEnabled: Bool
    
    init(id: UUID = UUID(), trigger: String, replacement: String, isEnabled: Bool = true) {
        self.id = id
        self.trigger = trigger
        self.replacement = replacement
        self.isEnabled = isEnabled
    }
}
