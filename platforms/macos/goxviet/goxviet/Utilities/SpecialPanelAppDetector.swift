//
//  SpecialPanelAppDetector.swift
//  GoxViet
//
//  Detects special panel apps (Spotlight, Raycast) that don't trigger
//  NSWorkspaceDidActivateApplicationNotification
//
//  PERFORMANCE: Uses caching and fast-path detection to avoid expensive
//  CGWindowListCopyWindowInfo and AX queries on every keystroke.
//
//  Based on reference implementation
//

import Cocoa
import ApplicationServices

/// Detects special panel/overlay apps like Spotlight and Raycast
class SpecialPanelAppDetector {

    // MARK: - Properties

    /// List of special panel app bundle identifiers
    static let specialPanelApps: [String] = [
        "com.apple.Spotlight",
        "com.raycast.macos",
        "com.runningwithcrayons.Alfred",           // Alfred launcher
        "com.apple.inputmethod.EmojiFunctionRowItem",
    ]

    /// Last detected frontmost app (for tracking changes)
    private static var lastFrontMostApp: String = ""

    // MARK: - Cache

    /// PERFORMANCE: Uses CFAbsoluteTimeGetCurrent() instead of Date() for faster timestamp
    /// TTL is 4.5s: polling fires every 5s, so cache expires between polls but not during.
    /// Cache is also explicitly invalidated on app switch via invalidateCache().
    private enum Cache {
        static var result: String?
        static var timestamp: CFAbsoluteTime = 0
        static let ttl: CFAbsoluteTime = 4.5  // 4500ms — matches 5s polling interval

        static func get() -> String?? {  // Double optional: nil = miss, .some(nil) = cached nil
            CFAbsoluteTimeGetCurrent() - timestamp < ttl ? .some(result) : nil
        }

        static func set(_ value: String?) {
            result = value
            timestamp = CFAbsoluteTimeGetCurrent()
        }

        static func clear() {
            result = nil
            timestamp = 0
        }
    }

    // MARK: - Detection Methods

    /// Check if a bundle ID is a special panel app
    static func isSpecialPanelApp(_ bundleId: String?) -> Bool {
        guard let bundleId else { return false }
        guard !SettingsManager.shared.disablePanelDetection else { return false }
        return specialPanelApps.contains { bundleId.hasPrefix($0) || bundleId == $0 }
    }

    /// Fast path: check focused element only (cheapest AX query)
    /// Returns:
    ///   - `.some(bundleId)` — focused element belongs to a special panel app
    ///   - `.some(nil)`     — AX query succeeded, focused element is NOT a special panel app
    ///   - `.none`          — AX query failed (permission denied, server timeout, etc.)
    private static func getFocusedSpecialPanelApp() -> String?? {
        let systemWide = AXUIElementCreateSystemWide()
        AXUIElementSetMessagingTimeout(systemWide, 0.05) // 50ms cap — prevents hang
        var focusedElement: CFTypeRef?

        let axResult = AXUIElementCopyAttributeValue(systemWide, kAXFocusedUIElementAttribute as CFString, &focusedElement)
        guard axResult == .success, let element = focusedElement else {
            // AX failed — caller should try slow path as fallback
            return nil
        }

        var pid: pid_t = 0
        guard AXUIElementGetPid(element as! AXUIElement, &pid) == .success, pid > 0,
              let app = NSRunningApplication(processIdentifier: pid),
              let bundleId = app.bundleIdentifier else {
            // AX succeeded but couldn't resolve app — treat as "no panel"
            return .some(nil)
        }

        return .some(isSpecialPanelApp(bundleId) ? bundleId : nil)
    }

    /// Get the currently active special panel app (if any)
    /// Uses caching and fast-path to avoid expensive operations on every call.
    ///
    /// Slow path (CGWindowListCopyWindowInfo) is only used when the AX query itself
    /// fails — if AX succeeds and the focused element isn't a panel app, we skip
    /// the window scan entirely (no panel is active).
    static func getActiveSpecialPanelApp() -> String? {
        // Check cache first (4.5s TTL, invalidated on app switch)
        if let cached = Cache.get() { return cached }

        switch getFocusedSpecialPanelApp() {
        case .some(let bundleId):
            // AX succeeded: cache whatever it found (panel app or nil "no panel")
            Cache.set(bundleId)
            return bundleId
        case nil:
            // AX failed: fall through to slow path as a last resort
            break
        }

        // Slow path: only reached when AX is unavailable (permission denied / server error)
        let result = getActiveSpecialPanelAppFullScan()
        Cache.set(result)
        return result
    }

    /// Full scan: expensive operation, only called when fast path fails
    private static func getActiveSpecialPanelAppFullScan() -> String? {
        // Method 1: Use CGWindowListCopyWindowInfo to find on-screen windows
        if let windowList = CGWindowListCopyWindowInfo([.optionOnScreenOnly, .excludeDesktopElements], kCGNullWindowID) as? [[String: Any]] {
            for window in windowList {
                guard let ownerPID = window[kCGWindowOwnerPID as String] as? pid_t,
                      let windowLayer = window[kCGWindowLayer as String] as? Int else {
                    continue
                }

                // Spotlight and Raycast typically use high window layers (above normal windows)
                if windowLayer > 0 {
                    if let app = NSRunningApplication(processIdentifier: ownerPID),
                       let bundleId = app.bundleIdentifier,
                       isSpecialPanelApp(bundleId) {
                        return bundleId
                    }
                }
            }
        }

        // Method 2: Check each special panel app directly
        for panelAppId in specialPanelApps {
            let runningApps = NSRunningApplication.runningApplications(withBundleIdentifier: panelAppId)

            for app in runningApps where app.isActive {
                return panelAppId
            }
        }

        return nil
    }

    /// Invalidate cache (call when app switch is detected)
    static func invalidateCache() {
        Cache.clear()
    }

    /// Clear cache (called on memory pressure)
    static func clearCache() {
        Cache.clear()
        Log.info("SpecialPanelAppDetector cache cleared")
    }

    // MARK: - Smart Switch Integration
    
    /// Check if a special panel app has become active or inactive
    /// Returns: (appChanged: Bool, newBundleId: String?, isSpecialPanelApp: Bool)
    static func checkForAppChange() -> (appChanged: Bool, newBundleId: String?, isSpecialPanelApp: Bool) {
        // Check if a special panel app is currently active
        let activePanelApp = getActiveSpecialPanelApp()
        
        if let panelApp = activePanelApp {
            // A special panel app is active
            if panelApp != lastFrontMostApp {
                lastFrontMostApp = panelApp
                return (true, panelApp, true)
            }
            return (false, panelApp, true)
        }
        
        // No special panel app is active
        // If we were previously in a special panel app, we've returned to a normal app
        if isSpecialPanelApp(lastFrontMostApp) {
            let workspaceApp = NSWorkspace.shared.frontmostApplication?.bundleIdentifier
            if let app = workspaceApp {
                lastFrontMostApp = app
                return (true, app, false)
            }
        }
        
        return (false, nil, false)
    }
    
    /// Update the last frontmost app (call this when NSWorkspaceDidActivateApplicationNotification fires)
    static func updateLastFrontMostApp(_ bundleId: String) {
        lastFrontMostApp = bundleId
    }
    
    /// Get the last known frontmost app
    static func getLastFrontMostApp() -> String {
        lastFrontMostApp
    }
}
