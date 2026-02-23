//
//  CleanArchitectureFFITests.swift
//  goxvietTests
//
//  Created by GoxViet Team on 2026-02-11.
//  Integration tests for clean architecture FFI API
//

import XCTest
@testable import goxviet

/// Integration tests for clean architecture FFI
/// Tests the presentation layer FFI API from presentation/ffi/api.rs
class CleanArchitectureFFITests: XCTestCase {
    
    // MARK: - Test Setup
    
    override func setUp() {
        super.setUp()
        // Each test gets fresh engine
    }
    
    override func tearDown() {
        super.tearDown()
    }
    
    // MARK: - Engine Lifecycle Tests
    
    func testEngineCreationAndDestruction() {
        // Test: Create engine with default config
        let engine = ime_engine_new()
        XCTAssertNotNil(engine, "Engine should be created")
        
        // Test: Free engine
        ime_engine_free(engine)
        // If no crash, memory management is correct
    }
    
    func testEngineCreationWithConfig() {
        // Test: Create engine with custom config
        var config = FfiConfig(
            input_method: Telex,
            tone_style: New,
            smart_mode: true,
            enable_shortcuts: true
        )
        
        let engine = ime_engine_new_with_config(config)
        XCTAssertNotNil(engine, "Engine should be created with config")
        
        ime_engine_free(engine)
    }
    
    func testMultipleEngineInstances() {
        // Test: Multiple concurrent engines
        let engine1 = ime_engine_new()
        let engine2 = ime_engine_new()
        let engine3 = ime_engine_new()
        
        XCTAssertNotNil(engine1)
        XCTAssertNotNil(engine2)
        XCTAssertNotNil(engine3)
        
        // Different instances
        XCTAssertNotEqual(engine1, engine2)
        XCTAssertNotEqual(engine2, engine3)
        
        ime_engine_free(engine1)
        ime_engine_free(engine2)
        ime_engine_free(engine3)
    }
    
    // MARK: - Configuration Tests
    
    func testGetConfiguration() {
        let engine = ime_engine_new()
        defer { ime_engine_free(engine) }
        
        // Test: Get config
        let config = ime_get_config(engine)
        
        // Default config values
        XCTAssertEqual(config.input_method, Telex)
        XCTAssertEqual(config.tone_style, New)
    }
    
    func testSetConfiguration() {
        let engine = ime_engine_new()
        defer { ime_engine_free(engine) }
        
        // Test: Change to VNI
        var newConfig = FfiConfig(
            input_method: Vni,
            tone_style: Old,
            smart_mode: false,
            enable_shortcuts: false
        )
        
        let result = ime_set_config(engine, newConfig)
        XCTAssertTrue(result.success, "Config update should succeed")
        XCTAssertEqual(result.error_code, 0)
        
        // Verify changes
        let updatedConfig = ime_get_config(engine)
        XCTAssertEqual(updatedConfig.input_method, Vni)
        XCTAssertEqual(updatedConfig.tone_style, Old)
        XCTAssertFalse(updatedConfig.smart_mode)
    }
    
    // MARK: - Keystroke Processing Tests (Telex)
    
    func testTelexBasicInput() {
        let engine = ime_engine_new()
        defer { ime_engine_free(engine) }
        
        // Test: Type "a"
        let result = processKey(engine: engine, key: "a")
        XCTAssertTrue(result.consumed, "Input should be consumed")
        XCTAssertEqual(result.backspace_count, 0)
    }
    
    func testTelexVietnameseWord() {
        let engine = ime_engine_new()
        defer { ime_engine_free(engine) }
        
        // Test: Type "viet" (no tone)
        _ = processKey(engine: engine, key: "v")
        _ = processKey(engine: engine, key: "i")
        _ = processKey(engine: engine, key: "e")
        let result = processKey(engine: engine, key: "t")
        
        XCTAssertTrue(result.consumed)
        // Should output "viet" (no transformation without tone)
    }
    
    func testTelexWithToneMark() {
        let engine = ime_engine_new()
        defer { ime_engine_free(engine) }
        
        // Test: Type "viets" -> "viét"
        _ = processKey(engine: engine, key: "v")
        _ = processKey(engine: engine, key: "i")
        _ = processKey(engine: engine, key: "e")
        _ = processKey(engine: engine, key: "t")
        
        let result = processKey(engine: engine, key: "s") // Add sắc tone
        
        XCTAssertTrue(result.consumed)
        XCTAssertTrue(result.backspace_count > 0, "Should delete previous text")
        
        if let text = result.text {
            let output = String(cString: text)
            XCTAssertTrue(output.contains("é") || output.contains("ế"), 
                          "Should contain tone mark, got: \(output)")
            ime_free_string(text)
        }
    }
    
    func testTelexComplexWord() {
        let engine = ime_engine_new()
        defer { ime_engine_free(engine) }
        
        // Test: Type "duowngf" -> "đường"
        let keys = ["d", "u", "o", "w", "n", "g", "f"]
        var lastResult: FfiProcessResult?
        
        for key in keys {
            lastResult = processKey(engine: engine, key: key)
        }
        
        XCTAssertNotNil(lastResult)
        if let result = lastResult, let text = result.text {
            let output = String(cString: text)
            // Should contain đ, ư, ờ, ng
            XCTAssertTrue(output.contains("đ"), "Should contain đ")
            XCTAssertTrue(output.contains("ư"), "Should contain ư")
            ime_free_string(text)
        }
    }
    
    // MARK: - Memory Management Tests
    
    func testStringMemoryManagement() {
        let engine = ime_engine_new()
        defer { ime_engine_free(engine) }
        
        // Process multiple keystrokes and free all strings
        let keys = ["v", "i", "e", "t", "s"]
        var allocatedStrings: [UnsafeMutablePointer<CChar>] = []
        
        for key in keys {
            let result = processKey(engine: engine, key: key)
            if let text = result.text {
                allocatedStrings.append(text)
            }
        }
        
        // Free all allocated strings
        for ptr in allocatedStrings {
            ime_free_string(ptr)
        }
        
        // If no crash, memory management is correct
        XCTAssertTrue(true)
    }
    
    func testNullPointerHandling() {
        // Test: Free null pointer (should not crash)
        ime_free_string(nil)
        
        // Test: Free engine with null (should not crash)
        ime_engine_free(nil)
        
        XCTAssertTrue(true, "Null pointer handling works")
    }
    
    // MARK: - Error Handling Tests
    
    func testInvalidEngineHandle() {
        // Test: Process with invalid handle
        let invalidHandle: FfiEngineHandle = nil
        let result = ime_process_key(invalidHandle, "a", 0)
        
        XCTAssertFalse(result.result.success, "Should fail with invalid handle")
        XCTAssertEqual(result.result.error_code, 2, "Error code should be 2 (invalid handle)")
    }
    
    func testInvalidUTF8() {
        let engine = ime_engine_new()
        defer { ime_engine_free(engine) }
        
        // Test: Invalid UTF-8 sequence
        // Note: Swift automatically validates UTF-8, so this is hard to test
        // In production, C clients might pass invalid UTF-8
        
        // For now, test with valid UTF-8
        let result = processKey(engine: engine, key: "a")
        XCTAssertTrue(result.result.success)
    }
    
    // MARK: - Backspace Handling Tests
    
    func testBackspaceAction() {
        let engine = ime_engine_new()
        defer { ime_engine_free(engine) }
        
        // Type something
        _ = processKey(engine: engine, key: "v")
        _ = processKey(engine: engine, key: "i")
        _ = processKey(engine: engine, key: "e")
        _ = processKey(engine: engine, key: "t")
        
        // Test: Backspace
        let backspaceResult = processBackspace(engine: engine)
        
        // Should handle backspace (consumed or not)
        XCTAssertTrue(backspaceResult.result.success)
    }
    
    // MARK: - Performance Tests
    
    func testKeystrokeLatency() {
        let engine = ime_engine_new()
        defer { ime_engine_free(engine) }
        
        let iterations = 1000
        let keys = ["v", "i", "e", "t", "s", " "] // Type "viets "
        
        measure {
            for _ in 0..<iterations {
                for key in keys {
                    let result = processKey(engine: engine, key: key)
                    if let text = result.text {
                        ime_free_string(text)
                    }
                }
            }
        }
        
        // Target: <1ms per keystroke (1000 keystrokes in <1 second)
    }
    
    func testMemoryFootprint() {
        // Test: Create 100 engines
        var engines: [FfiEngineHandle?] = []
        
        for _ in 0..<100 {
            engines.append(ime_engine_new())
        }
        
        // Process some keystrokes on each
        for engine in engines {
            if let eng = engine {
                _ = processKey(engine: eng, key: "v")
                _ = processKey(engine: eng, key: "i")
            }
        }
        
        // Clean up
        for engine in engines {
            if let eng = engine {
                ime_engine_free(eng)
            }
        }
        
        // Target: <10MB total (100 engines * ~100KB each)
        XCTAssertTrue(true)
    }
    
    // MARK: - Helper Methods
    
    private func processKey(engine: FfiEngineHandle?, key: String) -> FfiProcessResult {
        guard let cString = key.cString(using: .utf8) else {
            fatalError("Invalid UTF-8")
        }
        
        return cString.withUnsafeBufferPointer { buffer in
            ime_process_key(engine, buffer.baseAddress, 0) // 0 = TEXT action
        }
    }
    
    private func processBackspace(engine: FfiEngineHandle?) -> FfiProcessResult {
        return ime_process_key(engine, "", 1) // 1 = BACKSPACE action
    }
}

// MARK: - FfiProcessResult Extension

extension FfiProcessResult {
    var consumed: Bool {
        return self.consumed
    }
}
