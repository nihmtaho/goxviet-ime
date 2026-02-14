import Foundation

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

print("FfiResult size: \(MemoryLayout<FfiResult>.size), stride: \(MemoryLayout<FfiResult>.stride)")
print("FfiProcessResult size: \(MemoryLayout<FfiProcessResult>.size), stride: \(MemoryLayout<FfiProcessResult>.stride)")
