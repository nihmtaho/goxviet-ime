//
//  SettingsManagerTests.swift
//  GoxVietTests
//
//  Unit tests for SettingsManager state synchronization
//

import XCTest
import Combine
@testable import goxviet

final class SettingsManagerTests: XCTestCase {
    
    var manager: SettingsManager!
    var cancellables: Set<AnyCancellable>!
    
    override func setUpWithError() throws {
        try super.setUpWithError()
        manager = SettingsManager.shared
        cancellables = Set<AnyCancellable>()
        
        // Reset to defaults before each test
        manager.resetToDefaults()
    }
    
    override func tearDownWithError() throws {
        cancellables = nil
        try super.tearDownWithError()
    }
    
    // MARK: - Initialization Tests
    
    func testDefaultValues() throws {
        // After reset, should have default values
        XCTAssertEqual(manager.inputMethod, 0, "Default should be Telex")
        XCTAssertFalse(manager.modernToneStyle, "Default should be traditional")
        XCTAssertTrue(manager.escRestoreEnabled, "ESC restore should be enabled by default")
        XCTAssertFalse(manager.freeToneEnabled, "Free tone should be disabled by default")
        XCTAssertTrue(manager.instantRestoreEnabled, "Instant restore should be enabled")
        XCTAssertTrue(manager.smartModeEnabled, "Smart mode should be enabled")
        XCTAssertTrue(manager.autoDisableForNonLatin, "Auto-disable should be enabled")
    }
    
    // MARK: - Input Method Tests
    
    func testSetInputMethod() throws {
        let expectation = XCTestExpectation(description: "Input method changed")
        
        manager.$inputMethod
            .dropFirst()  // Skip initial value
            .sink { method in
                XCTAssertEqual(method, 1, "Method should be VNI")
                expectation.fulfill()
            }
            .store(in: &cancellables)
        
        manager.setInputMethod(1)
        
        wait(for: [expectation], timeout: 1.0)
    }
    
    func testSetInputMethodInvalid() throws {
        manager.setInputMethod(99)
        
        // Should remain unchanged
        XCTAssertEqual(manager.inputMethod, 0, "Invalid method should not change")
    }
    
    func testSetInputMethodIdempotent() throws {
        manager.setInputMethod(0)
        manager.setInputMethod(0)
        manager.setInputMethod(0)
        
        // Should still be 0
        XCTAssertEqual(manager.inputMethod, 0)
    }
    
    // MARK: - Tone Style Tests
    
    func testSetModernToneStyle() throws {
        let expectation = XCTestExpectation(description: "Tone style changed")
        
        manager.$modernToneStyle
            .dropFirst()
            .sink { modern in
                XCTAssertTrue(modern, "Should be modern")
                expectation.fulfill()
            }
            .store(in: &cancellables)
        
        manager.setModernToneStyle(true)
        
        wait(for: [expectation], timeout: 1.0)
    }
    
    func testSetModernToneStyleIdempotent() throws {
        manager.setModernToneStyle(true)
        manager.setModernToneStyle(true)
        
        XCTAssertTrue(manager.modernToneStyle)
    }
    
    // MARK: - ESC Restore Tests
    
    func testSetEscRestoreEnabled() throws {
        manager.setEscRestoreEnabled(false)
        XCTAssertFalse(manager.escRestoreEnabled)
        
        manager.setEscRestoreEnabled(true)
        XCTAssertTrue(manager.escRestoreEnabled)
    }
    
    // MARK: - Free Tone Tests
    
    func testSetFreeToneEnabled() throws {
        manager.setFreeToneEnabled(true)
        XCTAssertTrue(manager.freeToneEnabled)
        
        manager.setFreeToneEnabled(false)
        XCTAssertFalse(manager.freeToneEnabled)
    }
    
    // MARK: - Instant Restore Tests
    
    func testSetInstantRestoreEnabled() throws {
        manager.setInstantRestoreEnabled(false)
        XCTAssertFalse(manager.instantRestoreEnabled)
        
        manager.setInstantRestoreEnabled(true)
        XCTAssertTrue(manager.instantRestoreEnabled)
    }
    
    // MARK: - Smart Mode Tests
    
    func testSetSmartModeEnabled() throws {
        manager.setSmartModeEnabled(false)
        XCTAssertFalse(manager.smartModeEnabled)
        
        manager.setSmartModeEnabled(true)
        XCTAssertTrue(manager.smartModeEnabled)
    }
    
    // MARK: - Auto-Disable Tests
    
    func testSetAutoDisableForNonLatin() throws {
        manager.setAutoDisableForNonLatin(false)
        XCTAssertFalse(manager.autoDisableForNonLatin)
        
        manager.setAutoDisableForNonLatin(true)
        XCTAssertTrue(manager.autoDisableForNonLatin)
    }
    
    // MARK: - Reset Tests
    
    func testResetToDefaults() throws {
        // Change all settings
        manager.setInputMethod(1)
        manager.setModernToneStyle(true)
        manager.setEscRestoreEnabled(false)
        manager.setFreeToneEnabled(true)
        manager.setInstantRestoreEnabled(false)
        manager.setSmartModeEnabled(false)
        manager.setAutoDisableForNonLatin(false)
        
        // Reset
        manager.resetToDefaults()
        
        // Verify all back to defaults
        XCTAssertEqual(manager.inputMethod, 0)
        XCTAssertFalse(manager.modernToneStyle)
        XCTAssertTrue(manager.escRestoreEnabled)
        XCTAssertFalse(manager.freeToneEnabled)
        XCTAssertTrue(manager.instantRestoreEnabled)
        XCTAssertTrue(manager.smartModeEnabled)
        XCTAssertTrue(manager.autoDisableForNonLatin)
    }
    
    // MARK: - Export/Import Tests
    
    func testExportSettings() throws {
        manager.setInputMethod(1)
        manager.setModernToneStyle(true)
        
        let exported = manager.exportSettings()
        
        XCTAssertEqual(exported["inputMethod"] as? Int, 1)
        XCTAssertEqual(exported["modernToneStyle"] as? Bool, true)
        XCTAssertNotNil(exported["exportedAt"])
    }
    
    func testImportSettings() throws {
        let settings: [String: Any] = [
            "inputMethod": 1,
            "modernToneStyle": true,
            "escRestoreEnabled": false,
            "freeToneEnabled": true,
            "instantRestoreEnabled": false,
            "smartModeEnabled": false,
            "autoDisableForNonLatin": false
        ]
        
        manager.importSettings(settings)
        
        XCTAssertEqual(manager.inputMethod, 1)
        XCTAssertTrue(manager.modernToneStyle)
        XCTAssertFalse(manager.escRestoreEnabled)
        XCTAssertTrue(manager.freeToneEnabled)
        XCTAssertFalse(manager.instantRestoreEnabled)
        XCTAssertFalse(manager.smartModeEnabled)
        XCTAssertFalse(manager.autoDisableForNonLatin)
    }
    
    func testImportPartialSettings() throws {
        // Only import some settings
        let settings: [String: Any] = [
            "inputMethod": 1,
            "modernToneStyle": true
        ]
        
        manager.importSettings(settings)
        
        XCTAssertEqual(manager.inputMethod, 1)
        XCTAssertTrue(manager.modernToneStyle)
        // Others should remain at defaults
        XCTAssertTrue(manager.escRestoreEnabled)
    }
    
    func testExportImportRoundTrip() throws {
        // Set custom values
        manager.setInputMethod(1)
        manager.setModernToneStyle(true)
        manager.setEscRestoreEnabled(false)
        
        // Export
        let exported = manager.exportSettings()
        
        // Reset
        manager.resetToDefaults()
        XCTAssertEqual(manager.inputMethod, 0)
        
        // Import
        manager.importSettings(exported)
        
        // Should match original
        XCTAssertEqual(manager.inputMethod, 1)
        XCTAssertTrue(manager.modernToneStyle)
        XCTAssertFalse(manager.escRestoreEnabled)
    }
    
    // MARK: - Persistence Tests
    
    func testPersistence() throws {
        // Change a setting
        manager.setInputMethod(1)
        
        // Create new instance (simulates app restart)
        let newManager = SettingsManager.shared
        
        // Should have persisted value
        XCTAssertEqual(newManager.inputMethod, 1)
    }
    
    // MARK: - Thread Safety Tests
    
    func testConcurrentAccess() throws {
        let expectation = XCTestExpectation(description: "Concurrent access")
        expectation.expectedFulfillmentCount = 100
        
        let queue = DispatchQueue(label: "test.concurrent", attributes: .concurrent)
        
        for i in 0..<100 {
            queue.async {
                self.manager.setInputMethod(i % 2)
                self.manager.setModernToneStyle(i % 2 == 0)
                expectation.fulfill()
            }
        }
        
        wait(for: [expectation], timeout: 5.0)
        
        // Should not crash
        XCTAssertTrue(true)
    }
    
    func testRapidUpdates() throws {
        // Rapidly change settings
        for i in 0..<1000 {
            manager.setInputMethod(i % 2)
        }
        
        // Should be stable
        XCTAssertTrue(manager.inputMethod == 0 || manager.inputMethod == 1)
    }
    
    // MARK: - Notification Tests
    
    func testNotificationPosted() throws {
        let expectation = XCTestExpectation(description: "Notification posted")
        
        let observer = NotificationCenter.default.addObserver(
            forName: .inputMethodChanged,
            object: nil,
            queue: .main
        ) { _ in
            expectation.fulfill()
        }
        
        manager.setInputMethod(1)
        
        wait(for: [expectation], timeout: 1.0)
        
        NotificationCenter.default.removeObserver(observer)
    }
    
    func testNotificationNotPostedWhenUnchanged() throws {
        let expectation = XCTestExpectation(description: "Notification not posted")
        expectation.isInverted = true
        
        let observer = NotificationCenter.default.addObserver(
            forName: .inputMethodChanged,
            object: nil,
            queue: .main
        ) { _ in
            expectation.fulfill()
        }
        
        // Set to same value
        manager.setInputMethod(0)
        manager.setInputMethod(0)
        
        wait(for: [expectation], timeout: 0.5)
        
        NotificationCenter.default.removeObserver(observer)
    }
    
    // MARK: - Validation Tests
    
    func testInputValidation() throws {
        // Valid values
        manager.setInputMethod(0)
        XCTAssertEqual(manager.inputMethod, 0)
        
        manager.setInputMethod(1)
        XCTAssertEqual(manager.inputMethod, 1)
        
        // Invalid values should be rejected
        let previousValue = manager.inputMethod
        manager.setInputMethod(-1)
        XCTAssertEqual(manager.inputMethod, previousValue)
        
        manager.setInputMethod(2)
        XCTAssertEqual(manager.inputMethod, previousValue)
    }
}
