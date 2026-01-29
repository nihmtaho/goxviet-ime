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
    
    @Published private(set) var inputMethod: Int = 0 {
        didSet {
            syncToCore()
            postNotification(.inputMethodChanged, value: inputMethod)
        }
    }
    
    @Published private(set) var modernToneStyle: Bool = false {
        didSet {
            syncToCore()
            postNotification(.toneStyleChanged, value: modernToneStyle)
        }
    }
    
    @Published private(set) var escRestoreEnabled: Bool = true {
        didSet {
            syncToCore()
            postNotification(.escRestoreChanged, value: escRestoreEnabled)
        }
    }
    
    @Published private(set) var freeToneEnabled: Bool = false {
        didSet {
            syncToCore()
            postNotification(.freeToneChanged, value: freeToneEnabled)
        }
    }
    
    @Published private(set) var instantRestoreEnabled: Bool = true {
        didSet {
            syncToCore()
            postNotification(.instantRestoreChanged, value: instantRestoreEnabled)
        }
    }
    
    @Published private(set) var smartModeEnabled: Bool = true {
        didSet {
            syncToAppState()
            postNotification(.smartModeChanged, value: smartModeEnabled)
        }
    }
    
    @Published private(set) var autoDisableForNonLatin: Bool = true {
        didSet {
            syncToAppState()
        }
    }
    
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
        
        // Save to defaults
        saveAllToDefaults()
        
        Log.info("All settings reset to defaults")
    }
    
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
        
        saveAllToDefaults()
        syncToCore()
        syncToAppState()
        
        Log.info("Settings imported successfully")
    }
    
    // MARK: - Private Helpers
    
    private func loadFromDefaults() {
        inputMethod = userDefaults.integer(forKey: Keys.inputMethod)
        modernToneStyle = userDefaults.bool(forKey: Keys.modernToneStyle)
        escRestoreEnabled = userDefaults.bool(forKey: Keys.escRestoreEnabled)
        freeToneEnabled = userDefaults.bool(forKey: Keys.freeToneEnabled)
        instantRestoreEnabled = userDefaults.bool(forKey: Keys.instantRestoreEnabled)
        smartModeEnabled = userDefaults.bool(forKey: Keys.smartModeEnabled)
        autoDisableForNonLatin = userDefaults.bool(forKey: Keys.autoDisableForNonLatin)
        
        // Set defaults if never saved
        if !userDefaults.bool(forKey: "hasLaunchedBefore") {
            resetToDefaults()
            userDefaults.set(true, forKey: "hasLaunchedBefore")
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
    }
    
    private func syncToCore() {
        // Sync to Rust core via RustBridgeSafe
        let bridge = RustBridgeSafe.shared
        
        _ = bridge.setMethod(inputMethod)
        _ = bridge.setModernTone(modernToneStyle)
        _ = bridge.setEscRestore(escRestoreEnabled)
        _ = bridge.setFreeTone(freeToneEnabled)
        _ = bridge.setInstantRestore(instantRestoreEnabled)
    }
    
    private func syncToAppState() {
        // Sync to AppState for app-wide access (legacy compatibility)
        AppState.shared.isSmartModeEnabled = smartModeEnabled
        AppState.shared.autoDisableForNonLatinEnabled = autoDisableForNonLatin
    }
    
    private func postNotification(_ name: Notification.Name, value: Any) {
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
    }
}

// MARK: - Notification Names Extension

extension Notification.Name {
    static let settingsChanged = Notification.Name("com.goxviet.settingsChanged")
}
