import Foundation

struct FfiResult {
    var success: Bool
    var error_code: Int32
}

struct FfiProcessResult {
    var text: UnsafeMutablePointer<CChar>?
    var backspace_count: Int32
    var consumed: Bool
    var result: FfiResult
}

print("=== Swift Struct Layout ===")
print("FfiResult:")
print("  size: \(MemoryLayout<FfiResult>.size)")
print("  stride: \(MemoryLayout<FfiResult>.stride)")
print()
print("FfiProcessResult:")
print("  size: \(MemoryLayout<FfiProcessResult>.size)")
print("  stride: \(MemoryLayout<FfiProcessResult>.stride)")
print()

// Test offsets using withUnsafePointer
var testResult = FfiProcessResult(
    text: nil,
    backspace_count: 0,
    consumed: false,
    result: FfiResult(success: false, error_code: 0)
)

withUnsafePointer(to: &testResult) { ptr in
    let baseAddr = UInt(bitPattern: ptr)
    
    withUnsafePointer(to: &testResult.text) { fieldPtr in
        print("text offset: \(UInt(bitPattern: fieldPtr) - baseAddr)")
    }
    withUnsafePointer(to: &testResult.backspace_count) { fieldPtr in
        print("backspace_count offset: \(UInt(bitPattern: fieldPtr) - baseAddr)")
    }
    withUnsafePointer(to: &testResult.consumed) { fieldPtr in
        print("consumed offset: \(UInt(bitPattern: fieldPtr) - baseAddr)")
    }
    withUnsafePointer(to: &testResult.result) { fieldPtr in
        print("result offset: \(UInt(bitPattern: fieldPtr) - baseAddr)")
    }
}
