//
//  PerAppModeManagerEnhanced.swift
//  GoxViet
//
//  Enhanced per-app mode manager with caching, performance optimization,
//  and improved detection reliability
//

import Foundation
import Cocoa

/// Enhanced per-app mode manager with caching and performance optimizations
final class PerAppModeManagerEnhanced: LifecycleManaged {
    
    static let shared = PerAppModeManagerEnhanced()
    
    // MARK: - Properties
    
    private(set) var currentBundleId: String?
    private(set) var isRunning: Bool = false
    
    // Polling timer for special panel apps
    private var pollingTimer: Timer?
    
    // MARK: - Caching
    
    /// LRU cache for app metadata (icon, name, etc.)
    private let appMetadataCache = LRUCache<String, AppMetadata>(capacity: 50)
    
    /// Recently used apps (for quick access)
    private var recentlyUsedApps: [String] = []
    private let maxRecentApps = 10
    
    private var lastSwitchTime: Date?
    
    // MARK: - Structures
    
    struct AppMetadata {
        let bundleId: String
        let name: String
        let icon: NSImage?
        let version: String?
        let lastUsed: Date
        
        init(bundleId: String, app: NSRunningApplication? = nil) {
            self.bundleId = bundleId
            self.lastUsed = Date()
            
            if let app = app {
                self.name = app.localizedName ?? bundleId
                self.icon = app.icon
                self.version = app.bundleURL?.path
            } else if let url = NSWorkspace.shared.urlForApplication(withBundleIdentifier: bundleId) {
                let bundle = Bundle(url: url)
                self.name = bundle?.object(forInfoDictionaryKey: "CFBundleName") as? String ?? bundleId
                self.icon = NSWorkspace.shared.icon(forFile: url.path)
                self.version = bundle?.object(forInfoDictionaryKey: "CFBundleShortVersionString") as? String
            } else {
                self.name = bundleId
                self.icon = nil
                self.version = nil
            }
        }
    }
    

    
    // MARK: - Initialization
    
    private init() {}
    
    deinit {
        stop()
        Log.info("PerAppModeManagerEnhanced deinitialized")
    }
    
    // MARK: - Lifecycle
    
    func start() {
        guard !isRunning else {
            Log.info("PerAppModeManagerEnhanced already running")
            return
        }
        
        // Register workspace observer
        let observer = NSWorkspace.shared.notificationCenter.addObserver(
            forName: NSWorkspace.didActivateApplicationNotification,
            object: nil,
            queue: .main
        ) { [weak self] notification in
            self?.handleActivationNotification(notification)
        }
        
        ResourceManager.shared.register(
            observer: observer,
            identifier: "PerAppModeManagerEnhanced.workspaceObserver",
            center: NSWorkspace.shared.notificationCenter
        )
        
        isRunning = true
        
        // Initialize with current app
        if let frontmostApp = NSWorkspace.shared.frontmostApplication,
           let bundleId = frontmostApp.bundleIdentifier {
            currentBundleId = bundleId
            cacheAppMetadata(bundleId, app: frontmostApp)
            addToRecentApps(bundleId)
            
            if SettingsManager.shared.smartModeEnabled {
                restoreModeForCurrentApp()
            }
            
            Log.info("PerAppModeManagerEnhanced started (current: \(bundleId))")
        } else {
            Log.info("PerAppModeManagerEnhanced started")
        }
        
        // Start polling for special panel apps
        startPollingTimer()
    }
    
    func stop() {
        guard isRunning else { return }
        
        ResourceManager.shared.unregister(
            observerIdentifier: "PerAppModeManagerEnhanced.workspaceObserver",
            center: NSWorkspace.shared.notificationCenter
        )
        
        stopPollingTimer()
        
        isRunning = false
        currentBundleId = nil
        
        Log.info("PerAppModeManagerEnhanced stopped")
    }
    
    // MARK: - Notification Handling
    
    private func handleActivationNotification(_ notification: Notification) {
        let startTime = CFAbsoluteTimeGetCurrent()
        
        guard let app = notification.userInfo?[NSWorkspace.applicationUserInfoKey] as? NSRunningApplication,
              let bundleId = app.bundleIdentifier else {
            return
        }
        
        // Ignore same app
        guard bundleId != currentBundleId else { return }
        
        // Cache metadata
        cacheAppMetadata(bundleId, app: app)
        
        // Add to recent apps
        addToRecentApps(bundleId)
        
        Log.info("App switched: \(getAppName(bundleId)) (\(bundleId))")
        
        // Invalidate special panel cache
        SpecialPanelAppDetector.invalidateCache()
        SpecialPanelAppDetector.updateLastFrontMostApp(bundleId)
        
        // Save previous app state - must capture before updating currentBundleId
        let previousId = currentBundleId
        if let previousId = previousId,
           SettingsManager.shared.smartModeEnabled,
           previousId != bundleId {
            let currentMode = SettingsManager.shared.isEnabled
            SettingsManager.shared.setPerAppMode(bundleId: previousId, enabled: currentMode)
        }
        
        // Update current
        currentBundleId = bundleId
        
        // Clear buffer
        ime_clear_v2()
        
        // Restore mode for new app - pass bundleId directly to avoid race condition
        if SettingsManager.shared.smartModeEnabled {
            restoreModeForCurrentApp(bundleId: bundleId)
        }
        
        // Post notification for UI updates
        NotificationCenter.default.post(
            name: .currentAppChanged,
            object: bundleId,
            userInfo: ["appName": getAppName(bundleId)]
        )
        
        // Record switch time
        let elapsed = CFAbsoluteTimeGetCurrent() - startTime
        lastSwitchTime = Date()
        
        if elapsed > 0.01 {  // Log if > 10ms
            Log.warning("Slow app switch: \(Int(elapsed * 1000))ms")
        }
    }
    
    // MARK: - Mode Management
    
    private func restoreModeForCurrentApp(bundleId: String? = nil) {
        let targetBundleId = bundleId ?? currentBundleId
        guard let bundleId = targetBundleId else { return }
        
        let savedMode = SettingsManager.shared.getPerAppMode(bundleId: bundleId)
        
        SettingsManager.shared.setEnabledSilently(savedMode)
        ime_enabled_v2(savedMode)
        
        DispatchQueue.main.async {
            NotificationCenter.default.post(
                name: .updateStateChanged,
                object: savedMode
            )
        }
        
        Log.info("Mode restored: \(getAppName(bundleId)) → \(savedMode ? "Vietnamese" : "English")")
    }
    
    func setStateForCurrentApp(_ enabled: Bool) {
        guard let bundleId = currentBundleId else { return }
        guard SettingsManager.shared.smartModeEnabled else { return }
        
        SettingsManager.shared.setPerAppMode(bundleId: bundleId, enabled: enabled)
        
        Log.info("State saved: \(getAppName(bundleId)) → \(enabled ? "Vietnamese" : "English")")
        
        // Post notification
        NotificationCenter.default.post(
            name: .perAppModeChanged,
            object: bundleId,
            userInfo: ["enabled": enabled]
        )
    }
    
    // MARK: - Caching
    
    private func cacheAppMetadata(_ bundleId: String, app: NSRunningApplication? = nil) {
        // Check cache first
        if appMetadataCache.get(bundleId) != nil {
            return
        }
        
        // Create and cache metadata
        let metadata = AppMetadata(bundleId: bundleId, app: app)
        appMetadataCache.set(bundleId, metadata)
    }
    
    private func addToRecentApps(_ bundleId: String) {
        // Remove if already exists
        recentlyUsedApps.removeAll { $0 == bundleId }
        
        // Add to front
        recentlyUsedApps.insert(bundleId, at: 0)
        
        // Trim to max size
        if recentlyUsedApps.count > maxRecentApps {
            recentlyUsedApps = Array(recentlyUsedApps.prefix(maxRecentApps))
        }
    }
    
    // MARK: - Public API
    
    func getCurrentBundleId() -> String? {
        return currentBundleId
    }
    
    func getCurrentAppName() -> String? {
        guard let bundleId = currentBundleId else { return nil }
        return getAppName(bundleId)
    }
    
    func getCurrentAppIcon() -> NSImage? {
        guard let bundleId = currentBundleId else { return nil }
        return appMetadataCache.get(bundleId)?.icon
    }
    
    func getAppName(_ bundleId: String) -> String {
        if let cached = appMetadataCache.get(bundleId) {
            return cached.name
        }
        return SettingsManager.shared.getAppName(bundleId: bundleId)
    }
    
    func getAppIcon(_ bundleId: String) -> NSImage? {
        if let cached = appMetadataCache.get(bundleId) {
            return cached.icon
        }
        
        // Cache miss - load and cache
        cacheAppMetadata(bundleId)
        return appMetadataCache.get(bundleId)?.icon
    }
    
    func getRecentlyUsedApps() -> [String] {
        return recentlyUsedApps
    }
    
    func clearCache() {
        appMetadataCache.clear()
        recentlyUsedApps.removeAll()
        Log.info("Cache cleared")
    }
    
    /// Get all known apps with their Vietnamese input states
    /// - Returns: Dictionary mapping bundle IDs to enabled states
    func getKnownAppsWithStates() -> [String: Bool] {
        return SettingsManager.shared.getKnownAppsWithStates()
    }
    
    /// Set per-app mode for a specific app
    /// - Parameters:
    ///   - bundleId: Application bundle identifier
    ///   - enabled: Whether Vietnamese input should be enabled
    func setPerAppMode(bundleId: String, enabled: Bool) {
        SettingsManager.shared.setPerAppMode(bundleId: bundleId, enabled: enabled)
        
        // Post notification
        NotificationCenter.default.post(
            name: .perAppModeChanged,
            object: bundleId,
            userInfo: ["enabled": enabled]
        )
        
        Log.info("Per-app mode set: \(getAppName(bundleId)) → \(enabled ? "Vietnamese" : "English")")
    }
    
    /// Clear all per-app settings
    func clearAllPerAppModes() {
        SettingsManager.shared.clearAllPerAppModes()
        Log.info("All per-app modes cleared")
    }
    
    func refresh() {
        if let frontmostApp = NSWorkspace.shared.frontmostApplication,
           let bundleId = frontmostApp.bundleIdentifier {
            
            let previousId = currentBundleId
            currentBundleId = nil
            
            let userInfo: [AnyHashable: Any] = [
                NSWorkspace.applicationUserInfoKey: frontmostApp
            ]
            let notification = Notification(
                name: NSWorkspace.didActivateApplicationNotification,
                object: NSWorkspace.shared,
                userInfo: userInfo
            )
            
            currentBundleId = previousId
            handleActivationNotification(notification)
        }
    }
    
    // MARK: - Special Panel Detection
    
    private func startPollingTimer() {
        stopPollingTimer()
        
        let timer = Timer.scheduledTimer(withTimeInterval: 1.5, repeats: true) { [weak self] _ in
            self?.checkForSpecialPanelApp()
        }
        
        ResourceManager.shared.register(timer: timer, identifier: "PerAppModeManagerEnhanced.pollingTimer")
        pollingTimer = timer
        
        if let timer = pollingTimer {
            RunLoop.current.add(timer, forMode: .common)
        }
    }
    
    private func stopPollingTimer() {
        ResourceManager.shared.unregister(timerIdentifier: "PerAppModeManagerEnhanced.pollingTimer")
        pollingTimer = nil
    }
    
    private func checkForSpecialPanelApp() {
        let (appChanged, newBundleId, isSpecialPanel) = SpecialPanelAppDetector.checkForAppChange()
        
        guard appChanged, let bundleId = newBundleId else { return }
        
        // Simulate app switch
        if bundleId != currentBundleId {
            Log.info("Special panel detected: \(bundleId)")
            
            // Create synthetic notification
            if let app = NSRunningApplication.runningApplications(withBundleIdentifier: bundleId).first {
                let userInfo: [AnyHashable: Any] = [
                    NSWorkspace.applicationUserInfoKey: app
                ]
                let notification = Notification(
                    name: NSWorkspace.didActivateApplicationNotification,
                    object: NSWorkspace.shared,
                    userInfo: userInfo
                )
                handleActivationNotification(notification)
            }
        }
    }
}

// MARK: - Notification Names

extension Notification.Name {
    static let currentAppChanged = Notification.Name("com.goxviet.currentAppChanged")
    static let perAppModeChanged = Notification.Name("com.goxviet.perAppModeChanged")
}
