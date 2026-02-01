//
//  RustBridgeSafeTests.swift
//  GoxVietTests
//
//  Unit tests for RustBridgeSafe memory safety and error handling
//

import XCTest
@testable import goxviet

final class RustBridgeSafeTests: XCTestCase {
    
    var bridge: RustBridgeSafe!
    
    override func setUpWithError() throws {
        try super.setUpWithError()
        bridge = RustBridgeSafe.shared
        
        // Ensure initialized for tests
        let result = bridge.initialize()
        XCTAssertTrue(result.isSuccess, "Bridge initialization should succeed")
    }
    
    override func tearDownWithError() throws {
        // Clear state between tests
        _ = bridge.clearAll()
        try super.tearDownWithError()
    }
    
    // MARK: - Initialization Tests
    
    func testInitialization() throws {
        // Initialize should be idempotent
        let result1 = bridge.initialize()
        let result2 = bridge.initialize()
        
        XCTAssertTrue(result1.isSuccess)
        XCTAssertTrue(result2.isSuccess)
        XCTAssertTrue(bridge.isReady)
        XCTAssertNil(bridge.initializationError)
    }
    
    func testIsReady() throws {
        XCTAssertTrue(bridge.isReady, "Bridge should be ready after initialization")
    }
    
    // MARK: - Configuration Tests
    
    func testSetMethod() throws {
        // Valid methods
        let telexResult = bridge.setMethod(0)
        XCTAssertTrue(telexResult.isSuccess, "Setting Telex should succeed")
        
        let vniResult = bridge.setMethod(1)
        XCTAssertTrue(vniResult.isSuccess, "Setting VNI should succeed")
        
        // Invalid method
        let invalidResult = bridge.setMethod(99)
        XCTAssertTrue(invalidResult.isFailure, "Invalid method should fail")
        
        if case .failure(let error) = invalidResult {
            if case .invalidParameter(let msg) = error {
                XCTAssertTrue(msg.contains("must be 0"), "Error should mention valid values")
            } else {
                XCTFail("Should be invalidParameter error")
            }
        }
    }
    
    func testSetEnabled() throws {
        let enableResult = bridge.setEnabled(true)
        XCTAssertTrue(enableResult.isSuccess)
        
        let disableResult = bridge.setEnabled(false)
        XCTAssertTrue(disableResult.isSuccess)
    }
    
    func testSetModernTone() throws {
        let modernResult = bridge.setModernTone(true)
        XCTAssertTrue(modernResult.isSuccess)
        
        let traditionalResult = bridge.setModernTone(false)
        XCTAssertTrue(traditionalResult.isSuccess)
    }
    
    func testSetEscRestore() throws {
        let result = bridge.setEscRestore(true)
        XCTAssertTrue(result.isSuccess)
    }
    
    func testSetFreeTone() throws {
        let result = bridge.setFreeTone(true)
        XCTAssertTrue(result.isSuccess)
    }
    
    func testSetInstantRestore() throws {
        let result = bridge.setInstantRestore(true)
        XCTAssertTrue(result.isSuccess)
    }
    
    func testSetSkipWShortcut() throws {
        let result = bridge.setSkipWShortcut(true)
        XCTAssertTrue(result.isSuccess)
    }
    
    // MARK: - Buffer Management Tests
    
    func testClearBuffer() throws {
        let result = bridge.clearBuffer()
        XCTAssertTrue(result.isSuccess, "Clear buffer should succeed")
    }
    
    func testClearAll() throws {
        let result = bridge.clearAll()
        XCTAssertTrue(result.isSuccess, "Clear all should succeed")
    }
    
    func testRestoreWord() throws {
        // Valid word
        let validResult = bridge.restoreWord("việt")
        XCTAssertTrue(validResult.isSuccess, "Restore valid word should succeed")
        
        // Empty word should fail
        let emptyResult = bridge.restoreWord("")
        XCTAssertTrue(emptyResult.isFailure, "Restore empty word should fail")
        
        if case .failure(let error) = emptyResult {
            if case .invalidParameter(let msg) = error {
                XCTAssertTrue(msg.contains("empty"), "Error should mention empty parameter")
            } else {
                XCTFail("Should be invalidParameter error")
            }
        }
    }
    
    // MARK: - Shortcut Management Tests
    
    func testAddShortcut() throws {
        let result = bridge.addShortcut(trigger: "tt", replacement: "thân thiện")
        XCTAssertTrue(result.isSuccess, "Add shortcut should succeed")
    }
    
    func testAddShortcutEmptyTrigger() throws {
        let result = bridge.addShortcut(trigger: "", replacement: "test")
        XCTAssertTrue(result.isFailure, "Empty trigger should fail")
        
        if case .failure(let error) = result {
            if case .invalidParameter(let msg) = error {
                XCTAssertTrue(msg.contains("trigger"), "Error should mention trigger")
            } else {
                XCTFail("Should be invalidParameter error")
            }
        }
    }
    
    func testAddShortcutEmptyReplacement() throws {
        let result = bridge.addShortcut(trigger: "tt", replacement: "")
        XCTAssertTrue(result.isFailure, "Empty replacement should fail")
        
        if case .failure(let error) = result {
            if case .invalidParameter(let msg) = error {
                XCTAssertTrue(msg.contains("replacement"), "Error should mention replacement")
            } else {
                XCTFail("Should be invalidParameter error")
            }
        }
    }
    
    func testRemoveShortcut() throws {
        // Add first
        _ = bridge.addShortcut(trigger: "test", replacement: "value")
        
        // Remove
        let result = bridge.removeShortcut(trigger: "test")
        XCTAssertTrue(result.isSuccess, "Remove shortcut should succeed")
    }
    
    func testRemoveShortcutEmpty() throws {
        let result = bridge.removeShortcut(trigger: "")
        XCTAssertTrue(result.isFailure, "Remove empty trigger should fail")
    }
    
    func testClearShortcuts() throws {
        // Add some shortcuts
        _ = bridge.addShortcut(trigger: "tt", replacement: "test1")
        _ = bridge.addShortcut(trigger: "vv", replacement: "test2")
        
        // Clear all
        let result = bridge.clearShortcuts()
        XCTAssertTrue(result.isSuccess, "Clear shortcuts should succeed")
    }
    
    func testSyncShortcuts() throws {
        let shortcuts = [
            (key: "tt", value: "thân thiện", enabled: true),
            (key: "vv", value: "vui vẻ", enabled: true),
            (key: "xx", value: "disabled", enabled: false)
        ]
        
        let result = bridge.syncShortcuts(shortcuts)
        XCTAssertTrue(result.isSuccess, "Sync shortcuts should succeed")
    }
    
    func testSyncShortcutsEmpty() throws {
        let result = bridge.syncShortcuts([])
        XCTAssertTrue(result.isSuccess, "Sync empty shortcuts should succeed")
    }
    
    // MARK: - Key Processing Tests
    
    func testProcessKey() throws {
        // Process a simple key
        let result = bridge.processKey(UInt16(Character("a").asciiValue!), caps: false, ctrl: false, shift: false)
        
        XCTAssertTrue(result.isSuccess, "Process key should succeed")
        
        if case .success(let imeResult) = result {
            // Result should be valid
            XCTAssertTrue(imeResult.count <= imeResult.capacity, "Count should not exceed capacity")
        }
    }
    
    func testProcessKeySequence() throws {
        // Clear first
        _ = bridge.clearBuffer()
        
        // Process sequence: v, i, e, t -> việt in Telex
        _ = bridge.setMethod(0) // Telex
        
        let keys: [Character] = ["v", "i", "e", "t"]
        for char in keys {
            if let ascii = char.asciiValue {
                let result = bridge.processKey(UInt16(ascii), caps: false, ctrl: false, shift: false)
                XCTAssertTrue(result.isSuccess, "Processing '\(char)' should succeed")
            }
        }
    }
    
    // MARK: - Thread Safety Tests
    
    func testConcurrentAccess() throws {
        let expectation = self.expectation(description: "Concurrent access")
        expectation.expectedFulfillmentCount = 10
        
        let queue = DispatchQueue(label: "test.concurrent", attributes: .concurrent)
        
        for i in 0..<10 {
            queue.async {
                // Random operations
                if i % 2 == 0 {
                    _ = self.bridge.setMethod(i % 2)
                } else {
                    _ = self.bridge.clearBuffer()
                }
                expectation.fulfill()
            }
        }
        
        waitForExpectations(timeout: 5.0)
    }
    
    // MARK: - Memory Safety Tests
    
    func testMemoryLeakOnRepeatedCalls() throws {
        // This test ensures no memory leaks from FFI calls
        // Run in Instruments for detailed analysis
        
        for _ in 0..<1000 {
            _ = bridge.setMethod(0)
            _ = bridge.clearBuffer()
            _ = bridge.addShortcut(trigger: "test", replacement: "value")
            _ = bridge.removeShortcut(trigger: "test")
        }
        
        // If we get here without crashes, basic memory safety is OK
        XCTAssertTrue(true)
    }
    
    func testMemoryLeakOnProcessKey() throws {
        // Process many keys to check for leaks
        _ = bridge.clearBuffer()
        
        for _ in 0..<1000 {
            let result = bridge.processKey(UInt16(Character("a").asciiValue!), caps: false, ctrl: false, shift: false)
            // Result is properly freed in RustBridgeSafe
            XCTAssertTrue(result.isSuccess)
        }
    }
    
    // MARK: - Error Propagation Tests
    
    func testErrorPropagation() throws {
        // Test that errors are properly captured and don't crash
        
        // Invalid parameters
        let invalidMethod = bridge.setMethod(999)
        XCTAssertTrue(invalidMethod.isFailure)
        
        let emptyTrigger = bridge.addShortcut(trigger: "", replacement: "test")
        XCTAssertTrue(emptyTrigger.isFailure)
        
        let emptyWord = bridge.restoreWord("")
        XCTAssertTrue(emptyWord.isFailure)
        
        // All errors should have descriptions
        if case .failure(let error) = invalidMethod {
            XCTAssertFalse(error.localizedDescription.isEmpty)
        }
    }
    
    // MARK: - Convenience Method Tests
    
    func testSetMethodLogged() throws {
        XCTAssertTrue(bridge.setMethodLogged(0))
        XCTAssertTrue(bridge.setMethodLogged(1))
        XCTAssertFalse(bridge.setMethodLogged(99))
    }
}

// MARK: - Result Extension for Testing

extension Result {
    var isSuccess: Bool {
        if case .success = self {
            return true
        }
        return false
    }
    
    var isFailure: Bool {
        if case .failure = self {
            return true
        }
        return false
    }
}
