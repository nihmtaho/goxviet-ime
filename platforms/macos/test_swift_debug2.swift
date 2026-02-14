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
    var success: UInt8
    var _padding: (UInt8, UInt8, UInt8)
    var error_code: Int32
}

struct FfiProcessResult {
    var text: UnsafeMutablePointer<CChar>?
    var backspace_count: Int32
    var consumed: UInt8
    var _padding: (UInt8, UInt8, UInt8)
    var result: FfiResult
}

let handle = ime_engine_new()!
print("Engine created")

var resultA = "a".withCString { ptr in
    ime_process_key(handle, ptr, 0)
}

print("\n=== Swift Result ===")
print("text ptr: 0x\(String(format: "%016llX", UInt(bitPattern: resultA.text)))")
print("backspace_count: \(resultA.backspace_count)")
print("consumed: \(resultA.consumed)")
print("result.success: \(resultA.result.success)")
print("result.error_code: \(resultA.result.error_code)")

withUnsafeBytes(of: &resultA) { bytes in
    print("\nRaw bytes:")
    for i in 0..<min(24, bytes.count) {
        print(String(format: "%02d: 0x%02X", i, bytes[i]))
    }
}

if let text = resultA.text {
    let output = String(cString: text)
    print("\nText: '\(output)'")
    // Don't free - will crash
}

ime_engine_free(handle)
