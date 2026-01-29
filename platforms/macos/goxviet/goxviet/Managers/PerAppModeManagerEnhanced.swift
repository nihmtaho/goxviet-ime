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
    
    /// Performance metrics
    private var switchCount: Int = 0
    private var cacheHitCount: Int = 0
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
    
    struct PerformanceMetrics {
        let totalSwitches: Int
        let cacheHitRate: Double
        let averageSwitchTime: TimeInterval?
        let recentAppsCount: Int
        let cachedAppsCount: Int
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
            
            if AppState.shared.isSmartModeEnabled {
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
        
        // Update metrics
        switchCount += 1
        
        // Cache metadata
        cacheAppMetadata(bundleId, app: app)
        
        // Add to recent apps
        addToRecentApps(bundleId)
        
        Log.info("App switched: \(getAppName(bundleId)) (\(bundleId))")
        
        // Invalidate special panel cache
        SpecialPanelAppDetector.invalidateCache()
        SpecialPanelAppDetector.updateLastFrontMostApp(bundleId)
        
        // Save previous app state
        if let previousId = currentBundleId,
           AppState.shared.isSmartModeEnabled {
            let currentMode = AppState.shared.isEnabled
            AppState.shared.setPerAppMode(bundleId: previousId, enabled: currentMode)
        }
        
        // Update current
        currentBundleId = bundleId
        
        // Clear buffer
        ime_clear()
        
        // Restore mode for new app
        if AppState.shared.isSmartModeEnabled {
            restoreModeForCurrentApp()
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
    
    private func restoreModeForCurrentApp() {
        guard let bundleId = currentBundleId else { return }
        
        let savedMode = AppState.shared.getPerAppMode(bundleId: bundleId)
        
        AppState.shared.setEnabledSilently(savedMode)
        ime_enabled(savedMode)
        
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
        guard AppState.shared.isSmartModeEnabled else { return }
        
        AppState.shared.setPerAppMode(bundleId: bundleId, enabled: enabled)
        
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
        if let cached = appMetadataCache.get(bundleId) {
            cacheHitCount += 1
            return
        }
        
        // Create and cache metadata
        let metadata = AppMetadata(bundleId: bundleId, app: app)
        appMetadataCache.set(bundleId, value: metadata)
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
        return AppState.shared.getAppName(bundleId: bundleId)
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
    
    func getPerformanceMetrics() -> PerformanceMetrics {
        let totalQueries = switchCount
        let hitRate = totalQueries > 0 ? Double(cacheHitCount) / Double(totalQueries) : 0.0
        
        return PerformanceMetrics(
            totalSwitches: switchCount,
            cacheHitRate: hitRate,
            averageSwitchTime: nil,  // Could implement if needed
            recentAppsCount: recentlyUsedApps.count,
            cachedAppsCount: appMetadataCache.count
        )
    }
    
    func clearCache() {
        appMetadataCache.clear()
        recentlyUsedApps.removeAll()
        switchCount = 0
        cacheHitCount = 0
        Log.info("Cache cleared")
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
