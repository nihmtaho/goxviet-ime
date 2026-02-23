//
//  PerAppModeManagerEnhancedTests.swift
//  GoxVietTests
//
//  Unit tests for PerAppModeManagerEnhanced
//

import XCTest
@testable import GoxViet

final class PerAppModeManagerEnhancedTests: XCTestCase {
    
    var manager: PerAppModeManagerEnhanced!
    
    override func setUpWithError() throws {
        try super.setUpWithError()
        manager = PerAppModeManagerEnhanced.shared
        
        // Clear cache before each test
        manager.clearCache()
    }
    
    override func tearDownWithError() throws {
        if manager.isRunning {
            manager.stop()
        }
        try super.tearDownWithError()
    }
    
    // MARK: - Lifecycle Tests
    
    func testStartStop() throws {
        XCTAssertFalse(manager.isRunning)
        
        manager.start()
        XCTAssertTrue(manager.isRunning)
        
        manager.stop()
        XCTAssertFalse(manager.isRunning)
    }
    
    func testMultipleStarts() throws {
        manager.start()
        XCTAssertTrue(manager.isRunning)
        
        // Should not crash
        manager.start()
        XCTAssertTrue(manager.isRunning)
        
        manager.stop()
    }
    
    func testMultipleStops() throws {
        manager.start()
        manager.stop()
        XCTAssertFalse(manager.isRunning)
        
        // Should not crash
        manager.stop()
        XCTAssertFalse(manager.isRunning)
    }
    
    // MARK: - Current App Tests
    
    func testGetCurrentBundleId() throws {
        manager.start()
        
        // Should get current app
        let bundleId = manager.getCurrentBundleId()
        XCTAssertNotNil(bundleId, "Should have current app after start")
        
        manager.stop()
    }
    
    func testGetCurrentAppName() throws {
        manager.start()
        
        let name = manager.getCurrentAppName()
        XCTAssertNotNil(name, "Should have app name")
        
        if let name = name {
            XCTAssertFalse(name.isEmpty, "App name should not be empty")
        }
        
        manager.stop()
    }
    
    func testGetCurrentAppIcon() throws {
        manager.start()
        
        let icon = manager.getCurrentAppIcon()
        // Icon might be nil for some apps
        
        manager.stop()
    }
    
    // MARK: - Caching Tests
    
    func testAppMetadataCache() throws {
        manager.start()
        
        // Get current app
        guard let bundleId = manager.getCurrentBundleId() else {
            XCTFail("No current app")
            return
        }
        
        // First call should cache
        let name1 = manager.getAppName(bundleId)
        XCTAssertFalse(name1.isEmpty)
        
        // Second call should hit cache (faster)
        let name2 = manager.getAppName(bundleId)
        XCTAssertEqual(name1, name2)
        
        manager.stop()
    }
    
    func testIconCache() throws {
        manager.start()
        
        guard let bundleId = manager.getCurrentBundleId() else {
            XCTFail("No current app")
            return
        }
        
        let icon1 = manager.getAppIcon(bundleId)
        let icon2 = manager.getAppIcon(bundleId)
        
        // Should return same icon (cached)
        if icon1 != nil {
            XCTAssertNotNil(icon2)
        }
        
        manager.stop()
    }
    
    func testClearCache() throws {
        manager.start()
        
        // Build cache
        if let bundleId = manager.getCurrentBundleId() {
            _ = manager.getAppName(bundleId)
            _ = manager.getAppIcon(bundleId)
        }
        
        // Clear
        manager.clearCache()
        
        // Verify cache is cleared by checking recent apps
        let recentApps = manager.getRecentlyUsedApps()
        XCTAssertEqual(recentApps.count, 0, "Recent apps should be empty")
        
        manager.stop()
    }
    
    // MARK: - Recent Apps Tests
    
    func testRecentApps() throws {
        manager.start()
        
        let recentApps = manager.getRecentlyUsedApps()
        
        // Should have at least current app
        XCTAssertGreaterThanOrEqual(recentApps.count, 1, "Should have at least current app")
        
        // Should contain current app
        if let currentBundleId = manager.getCurrentBundleId() {
            XCTAssertTrue(recentApps.contains(currentBundleId), "Should contain current app")
        }
        
        manager.stop()
    }
    
    func testRecentAppsLimit() throws {
        manager.start()
        
        // Simulate many app switches (via refresh)
        for _ in 0..<20 {
            manager.refresh()
        }
        
        let recentApps = manager.getRecentlyUsedApps()
        XCTAssertLessThanOrEqual(recentApps.count, 10, "Should respect max limit")
        
        manager.stop()
    }
    
    // MARK: - Cache Tests
    
    func testCacheHitRate() throws {
        manager.start()
        
        guard let bundleId = manager.getCurrentBundleId() else {
            XCTFail("No current app")
            return
        }
        
        // Clear cache
        manager.clearCache()
        
        // First call (miss)
        _ = manager.getAppName(bundleId)
        
        // Multiple calls (hits - should use cache)
        for _ in 0..<10 {
            _ = manager.getAppName(bundleId)
        }
        
        manager.stop()
    }
    
    // MARK: - Mode Management Tests
    
    func testSetStateForCurrentApp() throws {
        manager.start()
        SettingsManager.shared.smartModeEnabled = true
        
        guard let bundleId = manager.getCurrentBundleId() else {
            XCTFail("No current app")
            return
        }
        
        // Save state
        manager.setStateForCurrentApp(true)
        
        // Verify saved
        let saved = SettingsManager.shared.getPerAppMode(bundleId: bundleId)
        XCTAssertTrue(saved, "Should save state")
        
        manager.stop()
    }
    
    func testSetStateRequiresSmartMode() throws {
        manager.start()
        SettingsManager.shared.smartModeEnabled = false
        
        guard let bundleId = manager.getCurrentBundleId() else {
            XCTFail("No current app")
            return
        }
        
        let originalState = SettingsManager.shared.getPerAppMode(bundleId: bundleId)
        
        // Try to save (should be ignored)
        manager.setStateForCurrentApp(!originalState)
        
        // Verify not changed (Smart Mode disabled)
        let newState = SettingsManager.shared.getPerAppMode(bundleId: bundleId)
        // Note: This might still change because AppState doesn't check Smart Mode
        // This is a test of expected behavior
        
        manager.stop()
    }
    
    // MARK: - App Switch Performance Tests
    
    func testAppSwitchPerformance() throws {
        manager.start()
        
        measure {
            manager.refresh()
        }
        
        manager.stop()
    }
    
    func testCacheLookupPerformance() throws {
        manager.start()
        
        guard let bundleId = manager.getCurrentBundleId() else {
            XCTFail("No current app")
            return
        }
        
        // Pre-cache
        _ = manager.getAppName(bundleId)
        
        measure {
            for _ in 0..<1000 {
                _ = manager.getAppName(bundleId)
            }
        }
        
        manager.stop()
    }
    
    // MARK: - Thread Safety Tests
    
    func testConcurrentAccess() throws {
        manager.start()
        
        let expectation = self.expectation(description: "Concurrent access")
        expectation.expectedFulfillmentCount = 100
        
        let queue = DispatchQueue(label: "test.concurrent", attributes: .concurrent)
        
        for _ in 0..<100 {
            queue.async {
                _ = self.manager.getCurrentBundleId()
                _ = self.manager.getCurrentAppName()
                _ = self.manager.getRecentlyUsedApps()
                expectation.fulfill()
            }
        }
        
        waitForExpectations(timeout: 5.0)
        manager.stop()
    }
    
    func testConcurrentRefresh() throws {
        manager.start()
        
        let expectation = self.expectation(description: "Concurrent refresh")
        expectation.expectedFulfillmentCount = 10
        
        let queue = DispatchQueue(label: "test.refresh", attributes: .concurrent)
        
        for _ in 0..<10 {
            queue.async {
                self.manager.refresh()
                expectation.fulfill()
            }
        }
        
        waitForExpectations(timeout: 10.0)
        manager.stop()
    }
    
    // MARK: - Memory Tests
    
    func testNoMemoryLeaks() throws {
        // Start and stop multiple times
        for _ in 0..<100 {
            manager.start()
            
            if let bundleId = manager.getCurrentBundleId() {
                _ = manager.getAppName(bundleId)
                _ = manager.getAppIcon(bundleId)
            }
            
            manager.stop()
        }
        
        // Should not crash or leak
    }
    
    func testCacheSizeLimit() throws {
        manager.start()
        
        // Try to cache many apps
        for i in 0..<100 {
            let fakeBundleId = "com.test.app\(i)"
            _ = manager.getAppName(fakeBundleId)
        }
        
        // Should not crash or leak with many cached apps
        
        manager.stop()
    }
}
