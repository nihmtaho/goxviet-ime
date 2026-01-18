//
//  PerAppModeManager.swift
//  GoxViet
//
//  Manages per-app Gõ Việt input mode settings
//  Default: English input (Vietnamese disabled)
//  Only tracks apps with Vietnamese ENABLED (max 100 apps)
//  Includes special panel app detection (Spotlight, Raycast)
//

import Foundation
import Cocoa

/// Manages per-application Vietnamese input mode
/// Default: English input (Vietnamese disabled) for all apps
/// Only stores apps where user explicitly enabled Vietnamese (max 100 apps)
class PerAppModeManager: LifecycleManaged {
    static let shared = PerAppModeManager()
    
    // MARK: - Properties
    
    /// Currently active application bundle ID
    private(set) var currentBundleId: String?
    
    /// Timer for polling special panel apps (Spotlight, Raycast)
    private var pollingTimer: Timer?
    
    /// Whether the manager is currently active
    private(set) var isRunning: Bool = false
    
    // MARK: - Initialization
    
    private init() {}
    
    deinit {
        stop()
        Log.info("PerAppModeManager deinitialized")
    }
    
    // MARK: - Lifecycle
    
    /// Start observing app switches
    func start() {
        guard !isRunning else {
            Log.info("PerAppModeManager already running")
            return
        }
        
        // IMPORTANT: NSWorkspace notifications must be registered with
        // NSWorkspace.shared.notificationCenter, NOT NotificationCenter.default!
        let observer = NSWorkspace.shared.notificationCenter.addObserver(
            forName: NSWorkspace.didActivateApplicationNotification,
            object: nil,
            queue: .main
        ) { [weak self] notification in
            self?.handleActivationNotification(notification)
        }
        
        // Register with ResourceManager for automatic cleanup
        ResourceManager.shared.register(
            observer: observer,
            identifier: "PerAppModeManager.workspaceObserver",
            center: NSWorkspace.shared.notificationCenter
        )
        
        isRunning = true
        
        // Get the current frontmost app
        if let frontmostApp = NSWorkspace.shared.frontmostApplication,
           let bundleId = frontmostApp.bundleIdentifier {
            currentBundleId = bundleId
            SpecialPanelAppDetector.updateLastFrontMostApp(bundleId)
            Log.info("PerAppModeManager started (current app: \(bundleId))")
            
            // Note: We no longer record all apps as "known" on start
            // Only apps with Vietnamese ENABLED are tracked (via setPerAppMode)
            
            // Apply saved mode for current app if smart mode is enabled
            if AppState.shared.isSmartModeEnabled {
                restoreModeForCurrentApp()
            }
        } else {
            Log.info("PerAppModeManager started")
        }
        
        // Start polling timer for special panel apps (Spotlight, Raycast)
        // These apps don't trigger NSWorkspaceDidActivateApplicationNotification
        startPollingTimer()
    }
    
    /// Stop observing app switches
    func stop() {
        guard isRunning else {
            return
        }
        
        // Unregister observer via ResourceManager
        ResourceManager.shared.unregister(
            observerIdentifier: "PerAppModeManager.workspaceObserver",
            center: NSWorkspace.shared.notificationCenter
        )
        
        // Stop polling timer
        stopPollingTimer()
        
        isRunning = false
        currentBundleId = nil
        
        Log.info("PerAppModeManager stopped")
    }
    
    // MARK: - Notification Handling
    
    private func handleActivationNotification(_ notification: Notification) {
        // Extract the activated application
        guard let app = notification.userInfo?[NSWorkspace.applicationUserInfoKey] as? NSRunningApplication,
              let bundleId = app.bundleIdentifier else {
            Log.info("App activation notification with no bundle ID")
            return
        }
        
        // Ignore if it's the same app (prevents redundant processing)
        guard bundleId != currentBundleId else {
            return
        }
        
        let appName = app.localizedName ?? bundleId
        Log.info("App switched: \(appName) (\(bundleId))")
        
        // Invalidate special panel app cache on app switch
        SpecialPanelAppDetector.invalidateCache()
        SpecialPanelAppDetector.updateLastFrontMostApp(bundleId)
        
        // Update current bundle ID
        let previousBundleId = currentBundleId
        currentBundleId = bundleId
        
        // Note: We no longer record all apps as "known" on switch
        // Only apps with Vietnamese ENABLED are tracked (via setPerAppMode)
        // This reduces resource usage and aligns with default-English behavior
        
        // Save the current mode for the previous app (if smart mode is enabled)
        // Only Vietnamese-enabled apps will be stored
        if let previousId = previousBundleId,
           AppState.shared.isSmartModeEnabled {
            let currentMode = AppState.shared.isEnabled
            AppState.shared.setPerAppMode(bundleId: previousId, enabled: currentMode)
        }
        
        // Clear composition buffer to avoid inconsistencies
        ime_clear()
        
        // Restore mode for new app if smart mode is enabled
        if AppState.shared.isSmartModeEnabled {
            restoreModeForCurrentApp()
        }
    }
    
    // MARK: - Mode Management
    
    /// Restore the saved mode for the current app
    private func restoreModeForCurrentApp() {
        guard let bundleId = currentBundleId else {
            return
        }
        
        // Get saved mode (default: disabled/English)
        let savedMode = AppState.shared.getPerAppMode(bundleId: bundleId)
        
        // Apply the saved mode
        AppState.shared.setEnabledSilently(savedMode)
        ime_enabled(savedMode)
        
        // Update UI - defer to avoid layout recursion during app switch
        DispatchQueue.main.async {
            NotificationCenter.default.post(
                name: .updateStateChanged,
                object: savedMode
            )
        }
        
        let appName = AppState.shared.getAppName(bundleId: bundleId)
        Log.info("Mode restored for \(appName): \(savedMode ? "Vietnamese" : "English")")
    }
    
    /// Explicitly set the state for the current app and save it
    func setStateForCurrentApp(_ enabled: Bool) {
        guard let bundleId = currentBundleId else {
            Log.info("No current app to save state for")
            return
        }
        
        // Only save if smart mode is enabled
        guard AppState.shared.isSmartModeEnabled else {
            return
        }
        
        // Save the mode for this app
        // Note: setPerAppMode automatically handles recordKnownApp when enabled=true
        AppState.shared.setPerAppMode(bundleId: bundleId, enabled: enabled)
        
        let appName = AppState.shared.getAppName(bundleId: bundleId)
        Log.info("State saved for \(appName): \(enabled ? "Vietnamese" : "English")")
    }
    
    /// Get the current app's bundle ID
    func getCurrentBundleId() -> String? {
        return currentBundleId
    }
    
    /// Get the current app's display name
    func getCurrentAppName() -> String? {
        guard let bundleId = currentBundleId else {
            return nil
        }
        return AppState.shared.getAppName(bundleId: bundleId)
    }
    
    /// Force refresh the current app state
    func refresh() {
        if let frontmostApp = NSWorkspace.shared.frontmostApplication,
           let _ = frontmostApp.bundleIdentifier {
            
            // Simulate app switch
            let previousId = currentBundleId
            currentBundleId = nil  // Reset to force update
            
            // Create a synthetic notification
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
    
    // MARK: - Special Panel App Detection
    
    /// Start polling timer for special panel apps (Spotlight, Raycast)
    /// PERFORMANCE: 1.5s polling interval with cached results (100ms TTL) for minimal overhead
    private func startPollingTimer() {
        // Stop existing timer if any
        stopPollingTimer()
        
        // Create new timer with 1.5s interval for reduced memory/CPU overhead
        let timer = Timer.scheduledTimer(withTimeInterval: 1.5, repeats: true) { [weak self] _ in
            self?.checkForSpecialPanelApp()
        }
        
        // Register with ResourceManager for automatic cleanup
        ResourceManager.shared.register(timer: timer, identifier: "PerAppModeManager.pollingTimer")
        pollingTimer = timer
        
        // Ensure timer runs even when UI is scrolling/tracking
        if let timer = pollingTimer {
            RunLoop.current.add(timer, forMode: .common)
        }
        
        Log.info("Special panel polling timer started (1.5s interval)")
    }
    
    /// Stop polling timer
    private func stopPollingTimer() {
        ResourceManager.shared.unregister(timerIdentifier: "PerAppModeManager.pollingTimer")
        pollingTimer = nil
    }
    
    /// Check for special panel app activation/deactivation
    /// Called by polling timer (200ms interval)
    private func checkForSpecialPanelApp() {
        let (appChanged, newBundleId, isSpecialPanel) = SpecialPanelAppDetector.checkForAppChange()
        
        guard appChanged else {
            return
        }
        
        // Handle app change
        if let bundleId = newBundleId {
            handleAppSwitch(to: bundleId, isSpecialPanel: isSpecialPanel)
        }
    }
    
    /// Handle app switch (both normal and special panel apps)
    private func handleAppSwitch(to bundleId: String, isSpecialPanel: Bool) {
        // Ignore if it's the same app
        guard bundleId != currentBundleId else {
            return
        }
        
        let appType = isSpecialPanel ? "special panel" : "normal"
        Log.info("App switched (\(appType)): \(bundleId)")
        
        // Save mode for previous app if smart mode is enabled
        if let previousId = currentBundleId,
           AppState.shared.isSmartModeEnabled {
            let currentMode = AppState.shared.isEnabled
            AppState.shared.setPerAppMode(bundleId: previousId, enabled: currentMode)
        }
        
        // Update current bundle ID
        currentBundleId = bundleId
        
        // Clear composition buffer to avoid inconsistencies
        ime_clear()
        
        // Restore mode for new app if smart mode is enabled
        if AppState.shared.isSmartModeEnabled {
            restoreModeForCurrentApp()
        }
    }
}