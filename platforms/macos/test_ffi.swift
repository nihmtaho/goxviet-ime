//
//  test_ffi.swift
//  Simple test to verify Rust FFI is working
//
//  Compile and run:
//  swiftc -I ../../../core/target/release -L ../../../core/target/release -lvietnamese_ime_core test_ffi.swift -o test_ffi
//  ./test_ffi
//

import Foundation

// Import C types
#if canImport(Darwin)
import Darwin
#else
import Glibc
#endif

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

// FFI function declarations
@_silgen_name("ime_init")
func ime_init()

@_silgen_name("ime_key")
func ime_key(_ key: UInt16, _ caps: Bool, _ ctrl: Bool) -> UnsafeMutablePointer<ImeResult>?

@_silgen_name("ime_free")
func ime_free(_ result: UnsafeMutablePointer<ImeResult>?)

@_silgen_name("ime_method")
func ime_method(_ method: UInt8)

@_silgen_name("ime_clear")
func ime_clear()

// Helper function to extract characters from result
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

// Main test
print("=== Vietnamese IME FFI Test ===\n")

// Initialize engine
print("1. Initializing engine...")
ime_init()
print("   ✓ Engine initialized")

// Enable engine explicitly
print("2. Enabling engine...")
@_silgen_name("ime_enabled")
func ime_enabled(_ enabled: Bool)
ime_enabled(true)
print("   ✓ Engine enabled")

// Set method to Telex
print("3. Setting method to Telex (0)...")
ime_method(0)
print("   ✓ Method set to Telex\n")

// Test 1: Type 'a' (keycode 0)
print("4. Testing keystroke: 'a' (keycode 0)")
if let result1 = ime_key(0, false, false) {
    defer { ime_free(result1) }
    let r = result1.pointee
    print("   Action: \(r.action), Backspace: \(r.backspace), Count: \(r.count)")
    let output = extractChars(from: r)
    print("   Output: '\(output)'")
    
    if r.action == 0 {
        print("   ✓ Engine buffering (action=0 is expected for first letter)")
    } else if r.action == 1 && output == "a" {
        print("   ✓ Test passed\n")
    } else {
        print("   ⚠ Unexpected: action=\(r.action), output='\(output)'\n")
    }
} else {
    print("   ✗ Test failed - ime_key returned null\n")
}

// Test 2: Type 's' to add sắc mark (keycode 1)
print("\n5. Testing keystroke: 's' (keycode 1) - should add sắc mark")
if let result2 = ime_key(1, false, false) {
    defer { ime_free(result2) }
    let r = result2.pointee
    print("   Action: \(r.action), Backspace: \(r.backspace), Count: \(r.count)")
    let output = extractChars(from: r)
    print("   Output: '\(output)'")
    
    if r.action == 1 && output == "á" {
        print("   ✓ Test passed - Got 'á'\n")
    } else if r.action == 1 {
        print("   ⚠ Got transformation but wrong output: '\(output)' (expected 'á')\n")
    } else {
        print("   ✗ Test failed - Expected action=1, output='á', got action=\(r.action), '\(output)'\n")
    }
} else {
    print("   ✗ Test failed - ime_key returned null\n")
}

// Clear buffer
print("6. Clearing buffer...")
ime_clear()
print("   ✓ Buffer cleared\n")

// Test 3: Type 'v' 'i' 'e' 't'
print("7. Testing sequence: v-i-e-t")
let keyCodes: [(UInt16, String)] = [
    (9, "v"),   // v
    (34, "i"),  // i
    (14, "e"),  // e
    (17, "t")   // t
]

for (keyCode, letter) in keyCodes {
    print("   Typing '\(letter)' (keycode \(keyCode))...")
    if let result = ime_key(keyCode, false, false) {
        defer { ime_free(result) }
        let r = result.pointee
        let output = extractChars(from: r)
        print("   -> Output: '\(output)', Action: \(r.action), BS: \(r.backspace)")
    } else {
        print("   -> Error: null result")
    }
}
print("   ✓ Sequence completed\n")

// Test 4: Add tone mark
print("\n8. Testing tone mark: 's' for sắc")
if let result = ime_key(1, false, false) {
    defer { ime_free(result) }
    let r = result.pointee
    let output = extractChars(from: r)
    print("   Output: '\(output)' (should be 'viết')")
    print("   Action: \(r.action), Backspace: \(r.backspace), Count: \(r.count)")
    
    if output.contains("ế") || output.contains("viết") {
        print("   ✓ Test passed - Tone mark applied\n")
    } else {
        print("   ⚠ Unexpected output: '\(output)'\n")
    }
} else {
    print("   ✗ Test failed - null result\n")
}

print("\n=== Test Complete ===")
print("\nNote: Engine uses buffered mode - action=0 means letter is buffered.")
print("Swift app must inject the original character when action=0.")
print("\nIf you see errors about 'Unknown attribute kind', that's from nm tool - ignore it.")