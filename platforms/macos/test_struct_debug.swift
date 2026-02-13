import Foundation

@_silgen_name("ime_engine_new")
func ime_engine_new() -> UnsafeMutableRawPointer?

@_silgen_name("ime_process_key")
func ime_process_key(_ handle: UnsafeMutableRawPointer?, _ key_char: UnsafePointer<CChar>?, _ action: Int32) -> FfiProcessResult

@_silgen_name("ime_engine_free")
func ime_engine_free(_ handle: UnsafeMutableRawPointer?)

@_silgen_name("ime_free_string")
func ime_free_string(_ str: UnsafeMutablePointer<CChar>?)

struct FfiResult {
    var success: Bool
    var error_code: Int32
}

struct FfiProcessResult {
    var text: UnsafeMutablePointer<CChar>?      // 0: *mut c_char (8 bytes)
    var backspace_count: Int32                   // 8: i32 (4 bytes)
    var consumed: Bool                           // 12: bool (1 byte + 3 padding)
    var result: FfiResult                        // 16: FfiResult (5 bytes -> 8 with padding)
}

let handle = ime_engine_new()!
print("Engine created")

let keyA = "a"
var resultA = keyA.withCString { ptr in
    ime_process_key(handle, ptr, 0)
}

print("=== FfiProcessResult Debug ===")
print("Size: \(MemoryLayout<FfiProcessResult>.size) bytes")
print("Stride: \(MemoryLayout<FfiProcessResult>.stride) bytes")
print()

withUnsafeBytes(of: &resultA) { bytes in
    print("Raw bytes (first 32):")
    for i in 0..<min(32, bytes.count) {
        print(String(format: "%02d: 0x%02X (%3d)", i, bytes[i], bytes[i]))
    }
}

print()
print("Field interpretation:")
print("  text ptr (0-7):        0x\(String(format: "%016llX", UInt(bitPattern: resultA.text)))")
print("  backspace_count (8-11): \(resultA.backspace_count)")
print("  consumed (12):          \(resultA.consumed)")
print("  result.success (16):    \(resultA.result.success)")
print("  result.error_code (17-20): \(resultA.result.error_code)")

if let text = resultA.text {
    let output = String(cString: text)
    print()
    print("Text output: \"\(output)\"")
    ime_free_string(text)
}

ime_engine_free(handle)
