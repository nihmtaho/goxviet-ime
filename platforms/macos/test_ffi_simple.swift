#!/usr/bin/env swift

// Simple FFI Test Runner for Clean Architecture
// Compile: swiftc -o test_ffi test_ffi_simple.swift -L./goxviet -lgoxviet_core
// Run: ./test_ffi

import Foundation

typealias FfiEngineHandle = OpaquePointer?

struct FfiResult {
    var success: UInt8  // Use UInt8 instead of Bool for C interop
    var _padding: (UInt8, UInt8, UInt8)  // Explicit padding to 4 bytes
    var error_code: Int32
}

enum FfiInputMethod: Int32 {
    case telex = 0
    case vni = 1
}

enum FfiToneStyle: Int32 {
    case old = 0
    case new_ = 1
}

struct FfiConfig {
    var input_method: FfiInputMethod
    var tone_style: FfiToneStyle
    var smart_mode: UInt8  // Use UInt8 instead of Bool
    var enable_shortcuts: UInt8  // Use UInt8 instead of Bool
}

struct FfiProcessResult {
    var text: UnsafeMutablePointer<CChar>?
    var backspace_count: Int32
    var consumed: UInt8  // Use UInt8 instead of Bool
    var _padding: (UInt8, UInt8, UInt8)  // Explicit padding to 4 bytes
    var result: FfiResult
}

// FFI Function Declarations
@_silgen_name("ime_engine_new")
func ime_engine_new() -> FfiEngineHandle

@_silgen_name("ime_engine_free")
func ime_engine_free(_ handle: FfiEngineHandle)

@_silgen_name("ime_get_config")
func ime_get_config(_ handle: FfiEngineHandle) -> FfiConfig

@_silgen_name("ime_set_config")
func ime_set_config(_ handle: FfiEngineHandle, _ config: FfiConfig) -> FfiResult

@_silgen_name("ime_process_key")
func ime_process_key(_ handle: FfiEngineHandle, _ key: UnsafePointer<CChar>?, _ action: Int32) -> FfiProcessResult

@_silgen_name("ime_free_string")
func ime_free_string(_ ptr: UnsafeMutablePointer<CChar>?)

@_silgen_name("ime_get_version")
func ime_get_version() -> UnsafeMutablePointer<CChar>?

// Test Functions
func testEngineLifecycle() -> Bool {
    print("Test 1: Engine Lifecycle...")
    
    let handle = ime_engine_new()
    guard handle != nil else {
        print("  ❌ Failed: engine creation returned null")
        return false
    }
    print("  ✅ Engine created")
    
    ime_engine_free(handle)
    print("  ✅ Engine freed")
    
    return true
}

func testConfigGetSet() -> Bool {
    print("\nTest 2: Config Get/Set...")
    
    let handle = ime_engine_new()
    guard handle != nil else {
        print("  ❌ Failed: engine creation returned null")
        return false
    }
    defer { ime_engine_free(handle) }
    
    // Get default config
    let defaultConfig = ime_get_config(handle)
    print("  ✅ Got default config: method=\(defaultConfig.input_method.rawValue)")
    
    // Set new config
    var newConfig = FfiConfig(
        input_method: .telex,
        tone_style: .new_,
        smart_mode: 1,
        enable_shortcuts: 1
    )
    
    let result = ime_set_config(handle, newConfig)
    guard result.success != 0 else {
        print("  ❌ Failed: set_config returned error \(result.error_code)")
        return false
    }
    print("  ✅ Config set successfully")
    
    // Verify config was set
    let verifyConfig = ime_get_config(handle)
    guard verifyConfig.input_method.rawValue == FfiInputMethod.telex.rawValue else {
        print("  ❌ Failed: config not persisted")
        return false
    }
    print("  ✅ Config verified")
    
    return true
}

func testProcessKey() -> Bool {
    print("\nTest 3: Process Key (Telex 'a' + 's')...")
    
    let handle = ime_engine_new()
    guard handle != nil else {
        print("  ❌ Failed: engine creation returned null")
        return false
    }
    defer { ime_engine_free(handle) }
    
    // Process 'a'
    let keyA = "a"
    let resultA = keyA.withCString { ptr in
        ime_process_key(handle, ptr, 0) // 0 = text action
    }
    
    guard resultA.result.success != 0 else {
        print("  ❌ Failed: process 'a' returned error code \(resultA.result.error_code)")
        print("  Details: consumed=\(resultA.consumed != 0), backspace_count=\(resultA.backspace_count)")
        print("  Raw result.success: \(resultA.result.success != 0)")
        print("  Result struct: success=\(resultA.result.success != 0), error_code=\(resultA.result.error_code)")
        
        // Try to read text anyway
        if let text = resultA.text {
            let output = String(cString: text)
            print("  Unexpected text output: \"\(output)\"")
            ime_free_string(resultA.text)
        }
        return false
    }
    
    if let text = resultA.text {
        let output = String(cString: text)
        print("  ✅ Process 'a' → \"\(output)\"")
        ime_free_string(resultA.text)
    } else {
        print("  ❌ Failed: no output text for 'a'")
        return false
    }
    
    // Process 's' (sắc tone)
    let keyS = "s"
    let resultS = keyS.withCString { ptr in
        ime_process_key(handle, ptr, 0)
    }
    
    guard resultS.result.error_code == 0 else {
        print("  ❌ Failed: process 's' returned error")
        return false
    }
    
    if let text = resultS.text {
        let output = String(cString: text)
        print("  ✅ Process 's' → \"\(output)\" (expected: á)")
        ime_free_string(resultS.text)
        
        // Check if we got the expected tone mark
        if output.contains("á") {
            print("  ✅ Tone mark applied correctly!")
            return true
        } else {
            print("  ⚠️  Output doesn't contain expected 'á'")
            return false
        }
    } else {
        print("  ❌ Failed: no output text for 's'")
        return false
    }
}

func testVersion() -> Bool {
    print("\nTest 4: Get Version...")
    
    let versionPtr = ime_get_version()
    guard let ptr = versionPtr else {
        print("  ❌ Failed: version string is null")
        return false
    }
    defer { ime_free_string(versionPtr) }
    
    let version = String(cString: ptr)
    print("  ✅ Version: \(version)")
    
    return !version.isEmpty
}

// Run all tests
print("========================================")
print("GoxViet Clean Architecture FFI Tests")
print("========================================")

var passCount = 0
var failCount = 0

let tests: [(String, () -> Bool)] = [
    ("Engine Lifecycle", testEngineLifecycle),
    ("Config Get/Set", testConfigGetSet),
    ("Process Key", testProcessKey),
    ("Version", testVersion)
]

for (name, test) in tests {
    if test() {
        passCount += 1
    } else {
        failCount += 1
    }
}

print("\n========================================")
print("Results: \(passCount) passed, \(failCount) failed")
print("========================================")

exit(failCount > 0 ? 1 : 0)
