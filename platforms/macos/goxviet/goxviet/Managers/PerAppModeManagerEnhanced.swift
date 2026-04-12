//
//  PerAppModeManagerEnhanced.swift
//  GoxViet
//
//  Enhanced per-app mode manager with caching, performance optimization,
//  and improved detection reliability
//

@preconcurrency import Foundation
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
    
    struct AppMetadata: @unchecked Sendable {
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
    
    nonisolated private init() {}
    
    deinit {}
    
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
            // Extract NSRunningApplication (NSObject, @unchecked Sendable) before crossing
            // the isolation boundary so the non-Sendable Notification never enters the Task.
            guard let app = notification.userInfo?[NSWorkspace.applicationUserInfoKey]
                    as? NSRunningApplication else { return }
            Task { @MainActor [weak self, app] in
                self?.handleAppActivation(app)
            }
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
    
    /// Entry point for notification-based app activation (internal and synthetic callers).
    private func handleActivationNotification(_ notification: Notification) {
        guard let app = notification.userInfo?[NSWorkspace.applicationUserInfoKey] as? NSRunningApplication else { return }
        handleAppActivation(app)
    }

    /// Core app-switch handler. Takes an `NSRunningApplication` directly so callers can
    /// extract it before crossing isolation boundaries (avoids sending `Notification`).
    private func handleAppActivation(_ app: NSRunningApplication) {
        let startTime = CFAbsoluteTimeGetCurrent()
        guard let bundleId = app.bundleIdentifier else { return }
        
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
        
        // Clear injection method detection cache on app switch
        clearDetectionCache()
        
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
        
        // Reset Spotlight detection cache so the next open is detected fresh
        resetSpotlightCache()
        
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
           frontmostApp.bundleIdentifier != nil {

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
        
        let timer = Timer.scheduledTimer(withTimeInterval: 5.0, repeats: true) { [weak self] _ in
            Task { @MainActor [weak self] in
                self?.checkForSpecialPanelApp()
            }
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
        let (appChanged, newBundleId, _) = SpecialPanelAppDetector.checkForAppChange()
        
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
    
    /// Lightweight check for panel apps (Spotlight, Raycast, …) dispatched to the
    /// main thread on every keystroke, gated by a short TTL to avoid querying AX on
    /// every single key press while still reacting within half a second of opening.
    ///
    /// TTL = 0.5 s:
    ///   • Worst-case detection lag: 0.5 s (down from 3 s)
    ///   • AX query rate during active typing: ≤ 2/sec (cheap with 50 ms cap)
    ///
    /// Cache is also reset immediately via `resetSpotlightCache()` whenever a
    /// modifier shortcut that typically opens a panel (Cmd/Opt + Space) is pressed,
    /// so the very first keystroke in the panel triggers detection.
    private var lastSpotlightCheckTime: Date = .distantPast
    private static let spotlightCheckTTL: TimeInterval = 0.5   // was 3.0 s

    /// - Parameter force: If true, bypass the TTL and run the AX check immediately.
    ///   Used by proactive checks scheduled after Cmd/Opt+Space to guarantee detection
    ///   even when the TTL was recently reset by an earlier (pre-panel-open) check.
    func checkSpotlightOnce(force: Bool = false) {
        // Throttle: skip if we checked within the last 0.5 seconds (unless forced)
        let now = Date()
        if !force {
            guard now.timeIntervalSince(lastSpotlightCheckTime) > Self.spotlightCheckTTL else { return }
        }
        lastSpotlightCheckTime = now

        // Quick check: is the focused element owned by a panel app?
        let systemWide = AXUIElementCreateSystemWide()
        AXUIElementSetMessagingTimeout(systemWide, 0.05) // 50 ms — prevents indefinite hang
        var focusedElement: CFTypeRef?

        guard AXUIElementCopyAttributeValue(systemWide, kAXFocusedUIElementAttribute as CFString, &focusedElement) == .success,
              let element = focusedElement else { return }

        var pid: pid_t = 0
        guard AXUIElementGetPid(element as! AXUIElement, &pid) == .success, pid > 0,
              let app = NSRunningApplication(processIdentifier: pid),
              let bundleId = app.bundleIdentifier,
              // Check against all known panel apps: Spotlight, Raycast, Emoji panel, …
              SpecialPanelAppDetector.isSpecialPanelApp(bundleId) else { return }

        // Panel app is active — handle app switch if not already tracked
        if bundleId != currentBundleId {
            Log.info("Panel app detected via checkSpotlightOnce(): \(bundleId)")
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

    /// Reset the panel-app detection cache — call on every app switch so the next
    /// Spotlight/Raycast open is detected fresh rather than being throttled by the TTL.
    /// Also called when Cmd/Opt+Space is pressed (likely opens a panel).
    func resetSpotlightCache() {
        lastSpotlightCheckTime = .distantPast
    }
}

// MARK: - Notification Names

extension Notification.Name {
    static let currentAppChanged = Notification.Name("com.goxviet.currentAppChanged")
    static let perAppModeChanged = Notification.Name("com.goxviet.perAppModeChanged")
}
