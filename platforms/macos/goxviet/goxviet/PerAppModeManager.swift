//
//  PerAppModeManager.swift
//  GoxViet
//
//  Manages per-app Gõ Việt input mode settings
//  Default: English input (Vietnamese disabled)
//  Only tracks apps with Vietnamese ENABLED (max 100 apps)
//

import Foundation
import Cocoa

/// Manages per-application Vietnamese input mode
/// Default: English input (Vietnamese disabled) for all apps
/// Only stores apps where user explicitly enabled Vietnamese (max 100 apps)
class PerAppModeManager {
    static let shared = PerAppModeManager()
    
    // MARK: - Properties
    
    /// Currently active application bundle ID
    private(set) var currentBundleId: String?
    
    /// Notification observer token
    private var observer: NSObjectProtocol?
    
    /// Whether the manager is currently active
    private(set) var isRunning: Bool = false
    
    // MARK: - Initialization
    
    private init() {}
    
    // MARK: - Lifecycle
    
    /// Start observing app switches
    func start() {
        guard !isRunning else {
            Log.info("PerAppModeManager already running")
            return
        }
        
        // IMPORTANT: NSWorkspace notifications must be registered with
        // NSWorkspace.shared.notificationCenter, NOT NotificationCenter.default!
        observer = NSWorkspace.shared.notificationCenter.addObserver(
            forName: NSWorkspace.didActivateApplicationNotification,
            object: nil,
            queue: .main
        ) { [weak self] notification in
            self?.handleActivationNotification(notification)
        }
        
        isRunning = true
        
        // Get the current frontmost app
        if let frontmostApp = NSWorkspace.shared.frontmostApplication,
           let bundleId = frontmostApp.bundleIdentifier {
            currentBundleId = bundleId
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
    }
    
    /// Stop observing app switches
    func stop() {
        guard isRunning else {
            return
        }
        
        if let observer = observer {
            NSWorkspace.shared.notificationCenter.removeObserver(observer)
            self.observer = nil
        }
        
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
        
        // Update UI
        NotificationCenter.default.post(
            name: .updateStateChanged,
            object: savedMode
        )
        
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
}