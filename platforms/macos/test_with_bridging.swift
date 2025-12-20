//
//  test_with_bridging.swift
//  Test using actual bridging header from VietnameseIMEFast
//
//  Compile and run:
//  swiftc -import-objc-header ../../VietnameseIMEFast/VietnameseIMEFast/VietnameseIMEFast-Bridging-Header.h \
//         -I ../../../core/target/release -L ../../../core/target/release -lvietnamese_ime_core \
//         test_with_bridging.swift -o test_with_bridging
//  ./test_with_bridging
//

import Foundation

print("=== Test with Real Bridging Header ===\n")

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
    
    if result.action == 0 {
        print("  ✓ Buffered (action=0 expected)")
    }
}

print()

// Type 's' (keycode 1) - sắc mark
print("Step 3: Press 's' (keycode 1) - should apply sắc mark")
if let r2 = ime_key(1, false, false) {
    defer { ime_free(r2) }
    let result = r2.pointee
    print("  → action=\(result.action), backspace=\(result.backspace), count=\(result.count)")
    
    // Extract chars
    var output = ""
    if result.count > 0 {
        let charArray = withUnsafeBytes(of: result.chars) { bytes -> [UInt32] in
            let buffer = bytes.bindMemory(to: UInt32.self)
            return Array(buffer.prefix(Int(result.count)))
        }
        
        for codepoint in charArray {
            if let scalar = UnicodeScalar(codepoint) {
                output.append(Character(scalar))
            }
        }
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
    }
}

print("\n=== Test Complete ===\n")