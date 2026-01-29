# RustBridge FFI Memory Management Documentation

## Overview

This document describes the memory management rules and ownership semantics for the FFI boundary between Swift (macOS platform) and Rust (core engine).

## Memory Ownership Rules

### Golden Rule
**"Who allocates, deallocates"** - The component that allocates memory is responsible for freeing it.

### Rust-Allocated Memory

#### ImeResult Struct
```c
typedef struct {
    uint32_t *chars;   // Heap-allocated by Rust
    size_t capacity;   // Allocated capacity
    uint8_t action;    // Action type
    uint8_t backspace; // Number of backspaces
    uint8_t count;     // Valid character count
    uint8_t _pad;      // Padding
} ImeResult;
```

**Ownership Flow:**
1. Rust allocates `ImeResult` on heap via `ime_key()` or `ime_key_ext()`
2. Returns raw pointer to Swift
3. Swift **MUST** call `ime_free(result)` immediately after copying data
4. Failure to free causes memory leak

**Correct Pattern:**
```swift
let resultPtr = ime_key_ext(key, caps, ctrl, shift)
guard let result = resultPtr else { return }

// Copy data immediately
let imeResult = result.pointee

// CRITICAL: Free Rust memory
ime_free(result)

// Now safe to use copied data
return imeResult
```

**❌ INCORRECT - Memory Leak:**
```swift
let resultPtr = ime_key_ext(key, caps, ctrl, shift)
// Forgot to call ime_free!
return resultPtr?.pointee
```

### String Passing (Swift → Rust)

#### C Strings (const char*)
When passing strings from Swift to Rust:

**Ownership:**
- Swift allocates temporary C string
- Rust **reads only**, does not take ownership
- String automatically freed when Swift scope ends

**Pattern:**
```swift
func addShortcut(trigger: String, replacement: String) {
    guard let triggerC = trigger.cString(using: .utf8),
          let replacementC = replacement.cString(using: .utf8) else { return }
    
    // Use withUnsafeBufferPointer to ensure pointer validity
    triggerC.withUnsafeBufferPointer { triggerPtr in
        replacementC.withUnsafeBufferPointer { replacementPtr in
            guard let triggerAddr = triggerPtr.baseAddress,
                  let replacementAddr = replacementPtr.baseAddress else { return }
            
            // Rust copies the string, doesn't take ownership
            ime_add_shortcut(triggerAddr, replacementAddr)
        }
    }
    // Strings automatically freed here
}
```

### Global State (Rust-Owned)

The Rust engine maintains internal global state:
- Input buffer
- Configuration
- Shortcuts map
- Word history

**Ownership:**
- Rust owns all internal state
- Swift never accesses internal structures directly
- All access via FFI functions

**No Explicit Cleanup:**
```swift
deinit {
    // No need to free global Rust state
    // It persists for application lifetime
}
```

## Thread Safety

### Locking Strategy

**Swift Side (RustBridgeSafe):**
```swift
private let ffiLock = NSRecursiveLock()

private func performFFICall<T>(_ block: () throws -> T) -> Result<T, Error> {
    ffiLock.lock()
    defer { ffiLock.unlock() }
    return try block()
}
```

**Rust Side:**
- Currently single-threaded
- All state access from Swift must be serialized
- No concurrent FFI calls allowed

### Future Considerations
For multi-threaded support:
1. Add `Mutex<T>` wrappers in Rust
2. Return error codes for lock contention
3. Implement async FFI interface

## Error Handling

### FFI Errors

**Rust Never Panics Across FFI:**
- All panics caught at FFI boundary
- NULL returned on error
- Swift validates all results

**Pattern:**
```swift
func processKey() -> Result<ImeResult, RustBridgeError> {
    let resultPtr = ime_key_ext(key, caps, ctrl, shift)
    
    // Check for NULL (Rust error)
    guard let result = resultPtr else {
        return .failure(.resultIsNull)
    }
    
    // Validate result structure
    guard result.pointee.count <= result.pointee.capacity else {
        ime_free(result)
        return .failure(.invalidResult)
    }
    
    // Safe to use
    let imeResult = result.pointee
    ime_free(result)
    return .success(imeResult)
}
```

### Error Types

```swift
enum RustBridgeError: Error {
    case notInitialized        // Bridge not initialized
    case invalidParameter(String)  // Bad input
    case ffiCallFailed(String)     // FFI error
    case memoryAllocationFailed    // Out of memory
    case stringEncodingFailed      // UTF-8 conversion failed
    case resultIsNull              // Rust returned NULL
    case invalidResult             // Result structure invalid
}
```

## Lifetime Management

### ImeResult Lifetime

**Valid Until:**
- `ime_free(result)` called
- Another FFI call that modifies state

**Safe Pattern:**
```swift
// 1. Get result
let resultPtr = ime_key_ext(...)
guard let result = resultPtr else { return }

// 2. IMMEDIATELY copy to Swift-owned memory
let action = result.pointee.action
let backspace = result.pointee.backspace
let count = result.pointee.count

// 3. Free Rust memory RIGHT AWAY
ime_free(result)

// 4. Now safe to use copied data
if action == 1 {
    processAction(backspace: backspace, count: count)
}
```

**❌ Unsafe - Dangling Pointer:**
```swift
let resultPtr = ime_key_ext(...)
guard let result = resultPtr else { return }

// DO SOMETHING ELSE
doOtherWork()

// Pointer may be invalid now!
let action = result.pointee.action  // ❌ DANGER
ime_free(result)
```

### String Lifetime (Swift → Rust)

**Scope Rule:**
- C string valid only within `withUnsafeBufferPointer` scope
- Rust must copy if needs to keep data

**Safe:**
```swift
cString.withUnsafeBufferPointer { ptr in
    ime_add_shortcut(ptr.baseAddress!, otherPtr.baseAddress!)
    // Rust copies strings, safe
}
// cString freed here, but Rust has its own copy
```

**❌ Unsafe:**
```swift
let ptr = cString.withUnsafeBufferPointer { $0.baseAddress! }
// ptr is now INVALID - dangling pointer!
ime_add_shortcut(ptr, otherPtr)  // ❌ CRASH
```

## Best Practices

### 1. Always Use RustBridgeSafe
```swift
// ✅ Good
let result = RustBridgeSafe.shared.setMethod(0)

// ❌ Bad - direct FFI call
ime_method(0)  // No error handling, no thread safety
```

### 2. Check Initialization
```swift
guard bridge.isReady else {
    Log.error("Bridge not ready")
    return
}
```

### 3. Handle All Errors
```swift
switch bridge.addShortcut(trigger: "tt", replacement: "test") {
case .success:
    Log.info("Shortcut added")
case .failure(let error):
    Log.error("Failed: \(error.localizedDescription)")
    // Show user-facing error if needed
}
```

### 4. Free Memory Immediately
```swift
let resultPtr = ime_key_ext(...)
guard let result = resultPtr else { return }

let data = result.pointee  // Copy
ime_free(result)          // Free immediately

// Use copied data
process(data)
```

### 5. Validate All Inputs
```swift
func restoreWord(_ word: String) -> Result<Void, RustBridgeError> {
    guard !word.isEmpty else {
        return .failure(.invalidParameter("word cannot be empty"))
    }
    
    guard let cString = word.cString(using: .utf8) else {
        return .failure(.stringEncodingFailed)
    }
    
    // Safe to proceed
    ...
}
```

## Testing

### Memory Leak Detection

**In Xcode:**
1. Run tests with Memory Graph Debugger
2. Check for leaked ImeResult allocations
3. Use Instruments → Leaks

**In Tests:**
```swift
func testMemoryLeakOnRepeatedCalls() throws {
    for _ in 0..<1000 {
        _ = bridge.processKey(...)  // Should not leak
    }
}
```

### Thread Safety Testing
```swift
func testConcurrentAccess() throws {
    let queue = DispatchQueue(label: "test", attributes: .concurrent)
    
    for i in 0..<100 {
        queue.async {
            _ = self.bridge.setMethod(i % 2)
        }
    }
    
    // Should not crash
}
```

## Debugging

### Enable FFI Logging
```swift
// In RustBridgeSafe
Log.debug("FFI call: \(functionName)")
Log.debug("Result pointer: \(resultPtr)")
```

### Check for NULL
```swift
if resultPtr == nil {
    Log.error("Rust returned NULL - check core engine logs")
}
```

### Validate Results
```swift
guard result.pointee.count <= result.pointee.capacity else {
    Log.error("Invalid result: count=\(result.pointee.count), capacity=\(result.pointee.capacity)")
    ime_free(result)
    return .failure(.invalidResult)
}
```

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                     Swift Layer (macOS)                     │
│  ┌────────────────────────────────────────────────────────┐ │
│  │              RustBridgeSafe (Safe API)                 │ │
│  │  - Error handling                                      │ │
│  │  - Thread safety (NSRecursiveLock)                     │ │
│  │  - Memory management                                   │ │
│  │  - Result validation                                   │ │
│  └────────────────────────────────────────────────────────┘ │
│                            │                                │
│                            ▼                                │
│  ┌────────────────────────────────────────────────────────┐ │
│  │              FFI Boundary (C ABI)                      │ │
│  │  - ime_init(), ime_key_ext(), ime_free()              │ │
│  │  - Raw pointers                                        │ │
│  │  - No exceptions                                       │ │
│  └────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────┐
│                     Rust Layer (Core)                       │
│  ┌────────────────────────────────────────────────────────┐ │
│  │                 #[no_mangle] pub extern "C"            │ │
│  │  - Panic handlers                                      │ │
│  │  - Memory allocation (Box, Vec)                        │ │
│  │  - State management                                    │ │
│  └────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

## Summary

**Key Takeaways:**
1. **Always free Rust-allocated memory** with `ime_free()`
2. **Copy data immediately** before freeing
3. **Use RustBridgeSafe** for all FFI calls
4. **Validate all results** before use
5. **Handle all errors** gracefully
6. **Test for memory leaks** regularly
7. **Serialize FFI calls** for thread safety

Following these rules ensures memory safety, prevents crashes, and maintains performance.
