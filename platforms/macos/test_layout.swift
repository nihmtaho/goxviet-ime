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

print("FfiResult size: \(MemoryLayout<FfiResult>.size), stride: \(MemoryLayout<FfiResult>.stride)")
print("FfiProcessResult size: \(MemoryLayout<FfiProcessResult>.size), stride: \(MemoryLayout<FfiProcessResult>.stride)")
print("Bool size: \(MemoryLayout<Bool>.size)")
print("Int32 size: \(MemoryLayout<Int32>.size)")
print("OpaquePointer size: \(MemoryLayout<OpaquePointer?>.size)")
