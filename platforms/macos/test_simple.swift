//
//  test_simple.swift
//  Simplest test case to debug Telex
//
//  Compile and run:
//  swiftc -I ../../core/target/release -L ../../core/target/release -lvietnamese_ime_core test_simple.swift -o test_simple
//  ./test_simple
//

import Foundation

// Define FFI structures and functions
struct ImeResult {
    var chars: (UInt32, UInt32, UInt32, UInt32, UInt32, UInt32, UInt32, UInt32,
                UInt32, UInt32, UInt32, UInt32, UInt32, UInt32, UInt32, UInt32,
                UInt32, UInt32, UInt32, UInt32, UInt32, UInt32, UInt32, UInt32,
                UInt32, UInt32, UInt32, UInt32, UInt32, UInt32, UInt32, UInt32)
    var action: UInt8
    var backspace: UInt8
    var count: UInt8
    var _pad: UInt8
}

@_silgen_name("ime_init")
func ime_init()

@_silgen_name("ime_key")
func ime_key(_ key: UInt16, _ caps: Bool, _ ctrl: Bool) -> UnsafeMutablePointer<ImeResult>?

@_silgen_name("ime_free")
func ime_free(_ result: UnsafeMutablePointer<ImeResult>?)

@_silgen_name("ime_method")
func ime_method(_ method: UInt8)

@_silgen_name("ime_enabled")
func ime_enabled(_ enabled: Bool)

@_silgen_name("ime_clear")
func ime_clear()

// Helper to extract chars
func extractChars(from result: ImeResult) -> String {
    var chars: [Character] = []
    let count = Int(result.count)
    let mirror = Mirror(reflecting: result.chars)
    
    var i = 0
    for child in mirror.children {
        if i >= count { break }
        if let codepoint = child.value as? UInt32,
           let scalar = UnicodeScalar(codepoint) {
            chars.append(Character(scalar))
        }
        i += 1
    }
    
    return String(chars)
}

print("=== SIMPLEST TEST: a + s → á ===\n")

// Initialize
print("Step 1: Initialize engine")
ime_init()
ime_method(0)  // Telex
ime_enabled(true)
print("✓ Engine ready\n")

// Type 'a' (keycode 0)
print("Step 2: Press 'a' (keycode 0)")
if let r1 = ime_key(0, false, false) {
    defer { ime_free(r1) }
    let result = r1.pointee
    print("  → action=\(result.action), backspace=\(result.backspace), count=\(result.count)")
    let output = extractChars(from: result)
    if !output.isEmpty {
        print("  → output: '\(output)'")
    }
    if result.action == 0 {
        print("  ✓ Buffered (action=0 expected)")
    }
} else {
    print("  ✗ NULL result!")
}

print()

// Type 's' (keycode 1) - sắc mark
print("Step 3: Press 's' (keycode 1) - should apply sắc mark")
if let r2 = ime_key(1, false, false) {
    defer { ime_free(r2) }
    let result = r2.pointee
    print("  → action=\(result.action), backspace=\(result.backspace), count=\(result.count)")
    let output = extractChars(from: result)
    if !output.isEmpty {
        print("  → output: '\(output)'")
    }
    
    if result.action == 1 {
        if output == "á" {
            print("  ✅ SUCCESS! Got 'á' as expected")
        } else {
            print("  ⚠ Action=1 but wrong output: '\(output)' (expected 'á')")
        }
    } else {
        print("  ✗ FAILED! Expected action=1 (Send), got action=\(result.action)")
        print("     This means engine did NOT recognize 's' as tone mark")
        print("     Possible causes:")
        print("     - Buffer validation failed")
        print("     - Foreign word detection blocked it")
        print("     - Mark not applied to vowel")
    }
} else {
    print("  ✗ NULL result!")
}

print("\n=== Test Complete ===\n")