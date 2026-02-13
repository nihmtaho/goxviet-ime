# FFI API v2 Design - Out Parameter Pattern

**Version:** 2.0.0  
**Date:** 2026-02-11  
**Status:** Design Phase  
**Priority:** CRITICAL - Fixes Swift FFI ABI issue

---

## Executive Summary

**Problem:** Current FFI API returns structs by value, causing ABI incompatibility with Swift standalone compilation.

**Solution:** Redesign FFI API to return status codes and write results via out parameters.

**Impact:** Breaking change requiring version bump (v1 → v2), but maintains backward compatibility via dual API.

---

## Design Principles

1. **Out Parameters:** All complex results passed via mutable pointers
2. **Status Codes:** Return `c_int` for success/error status
3. **C89 Compatible:** Works with C, Swift, C#, and all FFI consumers
4. **Backward Compatible:** Keep v1 API, add v2 alongside
5. **Memory Safety:** Clear ownership rules, documented lifecycle

---

## API v1 vs v2 Comparison

### Current API (v1) - Struct Return ❌

```c
// PROBLEMATIC: Returns struct by value
typedef struct {
    char* text;                // UTF-8 string (caller must free)
    uint8_t backspace_count;   // Number of backspaces
    bool consumed;             // Whether key was consumed
    bool success;              // Operation success
} FfiProcessResult;

// Function signature
FfiProcessResult ime_process_key(void* engine_ptr, char key_char);

// Usage (C works, Swift standalone FAILS)
FfiProcessResult result = ime_process_key(engine, 'a');
if (result.success) {
    printf("text: %s\n", result.text);  // ✅ C works
    ime_free_string(result.text);       // ❌ Swift reads garbage
}
```

**Issues:**
- Different struct-return calling conventions between Rust and Swift
- Registers vs stack passing varies by ABI
- Swift standalone gets corrupted struct fields

---

### New API (v2) - Out Parameters ✅

```c
// SUCCESS: Use out parameter
typedef struct {
    char* text;                // UTF-8 string (caller must free)
    uint8_t backspace_count;   // Number of backspaces
    bool consumed;             // Whether key was consumed
} FfiProcessResult_v2;

// Return codes
typedef enum {
    FFI_SUCCESS = 0,
    FFI_ERROR_NULL_ENGINE = -1,
    FFI_ERROR_NULL_OUTPUT = -2,
    FFI_ERROR_INVALID_KEY = -3,
    FFI_ERROR_PANIC = -99
} FfiStatusCode;

// Function signature (NEW)
int32_t ime_process_key_v2(
    void* engine_ptr,          // IN: Engine instance
    char key_char,             // IN: Key to process
    FfiProcessResult_v2* out   // OUT: Result written here
);

// Usage (Works everywhere! ✅)
FfiProcessResult_v2 result;
int32_t status = ime_process_key_v2(engine, 'a', &result);

if (status == FFI_SUCCESS) {
    printf("text: %s\n", result.text);  // ✅ Works in C, Swift, C#
    ime_free_string(result.text);
} else {
    fprintf(stderr, "Error: %d\n", status);
}
```

**Advantages:**
- ✅ ABI-safe across all platforms
- ✅ Clear error handling (status codes)
- ✅ No struct-return ABI issues
- ✅ Works with C, Swift, C#, JavaScript FFI
- ✅ Explicit null checks

---

## Complete API Design

### 1. Status Codes

```c
// ffi/types.rs
#[repr(C)]
pub enum FfiStatusCode {
    Success = 0,
    
    // Input errors
    ErrorNullEngine = -1,
    ErrorNullOutput = -2,
    ErrorNullConfig = -3,
    ErrorInvalidKey = -4,
    
    // Processing errors
    ErrorProcessingFailed = -10,
    ErrorInvalidUtf8 = -11,
    
    // System errors
    ErrorOutOfMemory = -20,
    ErrorPanic = -99,
}

impl FfiStatusCode {
    pub fn to_c_int(self) -> c_int {
        self as c_int
    }
}
```

### 2. Result Structs

```c
// ffi/types.rs

/// Process key result (v2) - OUT PARAMETER
#[repr(C)]
pub struct FfiProcessResult_v2 {
    /// UTF-8 text to insert (caller must free with ime_free_string)
    pub text: *mut c_char,
    
    /// Number of backspaces to perform
    pub backspace_count: u8,
    
    /// Whether key was consumed by IME
    pub consumed: bool,
}

/// Config structure (for get_config_v2)
#[repr(C)]
pub struct FfiConfig_v2 {
    pub input_method: FfiInputMethod,
    pub tone_style: FfiToneStyle,
    pub smart_mode: bool,
}

/// Version info
#[repr(C)]
pub struct FfiVersionInfo {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub api_version: u32,  // 2 for v2 API
}
```

### 3. Core API Functions (v2)

```c
// ffi/api.rs

/// Create engine instance
/// 
/// @param config Initial configuration (can be NULL for defaults)
/// @return Engine pointer or NULL on failure
#[no_mangle]
pub extern "C" fn ime_create_engine_v2(config: *const FfiConfig_v2) -> *mut c_void {
    // Implementation (no change needed, already returns pointer)
}

/// Destroy engine instance
/// 
/// @param engine_ptr Engine to destroy (safe to pass NULL)
#[no_mangle]
pub extern "C" fn ime_destroy_engine_v2(engine_ptr: *mut c_void) {
    // Implementation (no change needed)
}

/// Process single keystroke
/// 
/// @param engine_ptr Engine instance (must not be NULL)
/// @param key_char  Character to process
/// @param out       Output result (must not be NULL)
/// @return Status code (0 = success, <0 = error)
#[no_mangle]
pub extern "C" fn ime_process_key_v2(
    engine_ptr: *mut c_void,
    key_char: c_char,
    out: *mut FfiProcessResult_v2,
) -> c_int {
    use std::panic::catch_unwind;
    use std::panic::AssertUnwindSafe;
    
    // Null checks
    if engine_ptr.is_null() {
        return FfiStatusCode::ErrorNullEngine.to_c_int();
    }
    if out.is_null() {
        return FfiStatusCode::ErrorNullOutput.to_c_int();
    }
    
    // Panic safety
    let result = catch_unwind(AssertUnwindSafe(|| {
        // Cast to engine
        let engine = unsafe { &mut *(engine_ptr as *mut Engine) };
        
        // Process key
        let key_event = KeyEvent::Character(key_char as u8 as char);
        let result = engine.processor_service.process_key(&mut engine.buffer_manager, key_event);
        
        // Convert to FFI
        let ffi_result = FfiProcessResult_v2 {
            text: result.text.into_raw_c_string(),
            backspace_count: result.backspace_count,
            consumed: result.consumed,
        };
        
        // Write to out parameter
        unsafe { *out = ffi_result; }
        
        FfiStatusCode::Success
    }));
    
    match result {
        Ok(status) => status.to_c_int(),
        Err(_) => FfiStatusCode::ErrorPanic.to_c_int(),
    }
}

/// Get current configuration
/// 
/// @param engine_ptr Engine instance (must not be NULL)
/// @param out        Output config (must not be NULL)
/// @return Status code
#[no_mangle]
pub extern "C" fn ime_get_config_v2(
    engine_ptr: *mut c_void,
    out: *mut FfiConfig_v2,
) -> c_int {
    if engine_ptr.is_null() || out.is_null() {
        return FfiStatusCode::ErrorNullEngine.to_c_int();
    }
    
    let engine = unsafe { &*(engine_ptr as *const Engine) };
    let config = engine.config_service.get_config();
    
    unsafe {
        *out = FfiConfig_v2 {
            input_method: config.input_method.into(),
            tone_style: config.tone_style.into(),
            smart_mode: config.smart_mode,
        };
    }
    
    FfiStatusCode::Success.to_c_int()
}

/// Set configuration
/// 
/// @param engine_ptr Engine instance (must not be NULL)
/// @param config     New configuration (must not be NULL)
/// @return Status code
#[no_mangle]
pub extern "C" fn ime_set_config_v2(
    engine_ptr: *mut c_void,
    config: *const FfiConfig_v2,
) -> c_int {
    if engine_ptr.is_null() || config.is_null() {
        return FfiStatusCode::ErrorNullEngine.to_c_int();
    }
    
    let engine = unsafe { &mut *(engine_ptr as *mut Engine) };
    let ffi_config = unsafe { &*config };
    
    let rust_config = EngineConfig {
        input_method: ffi_config.input_method.into(),
        tone_style: ffi_config.tone_style.into(),
        smart_mode: ffi_config.smart_mode,
    };
    
    engine.config_service.set_config(rust_config);
    FfiStatusCode::Success.to_c_int()
}

/// Get version information
/// 
/// @param out Output version info (must not be NULL)
/// @return Status code
#[no_mangle]
pub extern "C" fn ime_get_version_v2(out: *mut FfiVersionInfo) -> c_int {
    if out.is_null() {
        return FfiStatusCode::ErrorNullOutput.to_c_int();
    }
    
    unsafe {
        *out = FfiVersionInfo {
            major: 2,
            minor: 0,
            patch: 0,
            api_version: 2,
        };
    }
    
    FfiStatusCode::Success.to_c_int()
}

/// Free string allocated by Rust
/// 
/// @param ptr String pointer (safe to pass NULL)
#[no_mangle]
pub extern "C" fn ime_free_string_v2(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe { drop(CString::from_raw(ptr)); }
    }
}
```

---

## Usage Examples

### C Client

```c
#include <stdio.h>
#include <stdlib.h>

int main() {
    // Create engine
    void* engine = ime_create_engine_v2(NULL);
    if (!engine) {
        fprintf(stderr, "Failed to create engine\n");
        return 1;
    }
    
    // Process key
    FfiProcessResult_v2 result;
    int status = ime_process_key_v2(engine, 'a', &result);
    
    if (status == FFI_SUCCESS) {
        if (result.text) {
            printf("Output: %s\n", result.text);
            printf("Backspace: %d\n", result.backspace_count);
            printf("Consumed: %s\n", result.consumed ? "true" : "false");
            
            // Free string
            ime_free_string_v2(result.text);
        }
    } else {
        fprintf(stderr, "Error processing key: %d\n", status);
    }
    
    // Cleanup
    ime_destroy_engine_v2(engine);
    return 0;
}
```

### Swift Client

```swift
import Foundation

class GoxVietEngine {
    private var enginePtr: UnsafeMutableRawPointer?
    
    init() {
        self.enginePtr = ime_create_engine_v2(nil)
        guard enginePtr != nil else {
            fatalError("Failed to create engine")
        }
    }
    
    deinit {
        if let ptr = enginePtr {
            ime_destroy_engine_v2(ptr)
        }
    }
    
    func processKey(_ char: Character) -> ProcessResult? {
        guard let engine = enginePtr else { return nil }
        
        // Allocate result on stack
        var result = FfiProcessResult_v2(
            text: nil,
            backspace_count: 0,
            consumed: false
        )
        
        // Call FFI (pass by reference)
        let status = ime_process_key_v2(
            engine,
            Int8(char.asciiValue ?? 0),
            &result
        )
        
        guard status == FFI_SUCCESS else {
            print("Error: \(status)")
            return nil
        }
        
        defer {
            if let text = result.text {
                ime_free_string_v2(text)
            }
        }
        
        // Convert to Swift
        let text = result.text != nil 
            ? String(cString: result.text!) 
            : ""
        
        return ProcessResult(
            text: text,
            backspaceCount: result.backspace_count,
            consumed: result.consumed
        )
    }
}

struct ProcessResult {
    let text: String
    let backspaceCount: UInt8
    let consumed: Bool
}
```

### C# Client (Windows)

```csharp
using System;
using System.Runtime.InteropServices;

public class GoxVietEngine : IDisposable
{
    [StructLayout(LayoutKind.Sequential)]
    public struct FfiProcessResult_v2
    {
        public IntPtr text;
        public byte backspace_count;
        [MarshalAs(UnmanagedType.I1)]
        public bool consumed;
    }
    
    [DllImport("goxviet_core", CallingConvention = CallingConvention.Cdecl)]
    private static extern IntPtr ime_create_engine_v2(IntPtr config);
    
    [DllImport("goxviet_core", CallingConvention = CallingConvention.Cdecl)]
    private static extern void ime_destroy_engine_v2(IntPtr engine);
    
    [DllImport("goxviet_core", CallingConvention = CallingConvention.Cdecl)]
    private static extern int ime_process_key_v2(
        IntPtr engine,
        sbyte keyChar,
        ref FfiProcessResult_v2 outResult
    );
    
    [DllImport("goxviet_core", CallingConvention = CallingConvention.Cdecl)]
    private static extern void ime_free_string_v2(IntPtr ptr);
    
    private IntPtr enginePtr;
    
    public GoxVietEngine()
    {
        enginePtr = ime_create_engine_v2(IntPtr.Zero);
        if (enginePtr == IntPtr.Zero)
            throw new Exception("Failed to create engine");
    }
    
    public ProcessResult? ProcessKey(char keyChar)
    {
        var result = new FfiProcessResult_v2();
        int status = ime_process_key_v2(
            enginePtr,
            (sbyte)keyChar,
            ref result
        );
        
        if (status != 0)
        {
            Console.WriteLine($"Error: {status}");
            return null;
        }
        
        string text = result.text != IntPtr.Zero
            ? Marshal.PtrToStringUTF8(result.text)
            : "";
        
        if (result.text != IntPtr.Zero)
            ime_free_string_v2(result.text);
        
        return new ProcessResult
        {
            Text = text,
            BackspaceCount = result.backspace_count,
            Consumed = result.consumed
        };
    }
    
    public void Dispose()
    {
        if (enginePtr != IntPtr.Zero)
        {
            ime_destroy_engine_v2(enginePtr);
            enginePtr = IntPtr.Zero;
        }
    }
}
```

---

## Backward Compatibility Strategy

### Dual API Approach

Keep both v1 and v2 APIs for 2-3 releases:

```rust
// OLD API (v1) - Marked deprecated
#[deprecated(since = "2.0.0", note = "Use ime_process_key_v2 instead")]
#[no_mangle]
pub extern "C" fn ime_process_key(
    engine_ptr: *mut c_void,
    key_char: c_char,
) -> FfiProcessResult {
    // Keep implementation, mark deprecated
    // ...
}

// NEW API (v2) - Recommended
#[no_mangle]
pub extern "C" fn ime_process_key_v2(
    engine_ptr: *mut c_void,
    key_char: c_char,
    out: *mut FfiProcessResult_v2,
) -> c_int {
    // New implementation
    // ...
}
```

### Migration Timeline

```
v2.0.0 (Now)     → Introduce v2 API, deprecate v1
v2.1.0 (+1 month) → Warning if using v1
v2.2.0 (+2 months) → Final warning
v3.0.0 (+3 months) → Remove v1 API (breaking change)
```

---

## Testing Strategy

### 1. C Test (Reference)

```c
// test_ffi_v2.c
void test_process_key_v2() {
    void* engine = ime_create_engine_v2(NULL);
    assert(engine != NULL);
    
    FfiProcessResult_v2 result;
    int status = ime_process_key_v2(engine, 'a', &result);
    
    assert(status == FFI_SUCCESS);
    assert(result.text != NULL);
    assert(strcmp(result.text, "a") == 0);
    assert(result.consumed == true);
    
    ime_free_string_v2(result.text);
    ime_destroy_engine_v2(engine);
}
```

### 2. Swift Standalone Test

```swift
// test_ffi_v2.swift
func testProcessKeyV2() {
    let engine = ime_create_engine_v2(nil)
    XCTAssertNotNil(engine)
    
    var result = FfiProcessResult_v2(text: nil, backspace_count: 0, consumed: false)
    let status = ime_process_key_v2(engine!, 97, &result)  // 'a'
    
    XCTAssertEqual(status, FFI_SUCCESS)
    XCTAssertNotNil(result.text)
    XCTAssertEqual(String(cString: result.text!), "a")
    XCTAssertTrue(result.consumed)
    
    ime_free_string_v2(result.text)
    ime_destroy_engine_v2(engine!)
}
```

### 3. Integration Test Matrix

| Platform | Compiler | API | Status | Notes |
|----------|----------|-----|--------|-------|
| macOS | clang | v2 | ✅ Must pass | C reference |
| macOS | swiftc standalone | v2 | ✅ Must pass | Fixes ABI issue |
| macOS | Xcode | v2 | ✅ Must pass | Production |
| Windows | MSVC | v2 | ✅ Must pass | C# interop |
| Windows | MinGW | v2 | ✅ Must pass | C interop |

---

## Performance Impact

**Expected:** Minimal to zero performance impact

**Rationale:**
- Out parameters are just pointers (register passing)
- No additional allocations
- Same internal logic
- Compiler optimizes both patterns identically

**Measurement:**
- Benchmark v1 vs v2 (should be ~equal)
- Target: <1% difference

---

## Security Considerations

### Null Pointer Safety

```c
// SAFE: All public APIs validate pointers
int status = ime_process_key_v2(NULL, 'a', &result);
// Returns FFI_ERROR_NULL_ENGINE, does not crash

int status = ime_process_key_v2(engine, 'a', NULL);
// Returns FFI_ERROR_NULL_OUTPUT, does not crash
```

### Panic Safety

```rust
// All FFI boundaries use catch_unwind
let result = catch_unwind(AssertUnwindSafe(|| {
    // ... processing logic
}));

match result {
    Ok(status) => status.to_c_int(),
    Err(_) => FfiStatusCode::ErrorPanic.to_c_int(),
}
```

### Memory Safety

```c
// Clear ownership: Caller owns strings returned by Rust
FfiProcessResult_v2 result;
ime_process_key_v2(engine, 'a', &result);

// result.text is owned by caller
printf("%s", result.text);

// Caller MUST free
ime_free_string_v2(result.text);
```

---

## Implementation Checklist

### Phase 1: Core Implementation
- [ ] Define FfiStatusCode enum
- [ ] Define FfiProcessResult_v2 struct
- [ ] Define FfiConfig_v2 struct
- [ ] Define FfiVersionInfo struct
- [ ] Implement ime_process_key_v2
- [ ] Implement ime_get_config_v2
- [ ] Implement ime_set_config_v2
- [ ] Implement ime_get_version_v2
- [ ] Implement ime_free_string_v2

### Phase 2: Testing
- [ ] Create test_ffi_v2.c (C reference test)
- [ ] Create test_ffi_v2.swift (Swift standalone test)
- [ ] Run all tests and verify pass
- [ ] Verify Swift ABI issue is fixed
- [ ] Benchmark v1 vs v2 performance

### Phase 3: Integration
- [ ] Update Xcode project to use v2 API
- [ ] Update Windows client to use v2 API
- [ ] Mark v1 API as deprecated
- [ ] Update documentation
- [ ] Create migration guide

### Phase 4: Release
- [ ] Version bump to 2.0.0
- [ ] Update CHANGELOG.md
- [ ] Create release notes
- [ ] Tag release
- [ ] Monitor for issues

---

## Success Criteria

✅ **Primary Goal:** Swift standalone test passes with v2 API  
✅ **Compatibility:** C, Swift, C# clients all work  
✅ **Performance:** <1% overhead vs v1  
✅ **Safety:** All null/panic cases handled  
✅ **Documentation:** Complete migration guide  

---

## References

- **Issue Report:** `core/PHASE_6_FFI_ABI_ISSUE.md`
- **Test Results:** `core/PHASE_6_INTEGRATION_TEST_REPORT.md`
- **C ABI Standard:** System V AMD64 ABI
- **Rust FFI Guide:** https://doc.rust-lang.org/nomicon/ffi.html
- **Swift Interop:** https://developer.apple.com/documentation/swift/c_interoperability

---

**Next Steps:**
1. Review and approve design
2. Implement Phase 1 (core API)
3. Test with C and Swift
4. Verify ABI issue resolved
5. Update documentation
6. Release v2.0.0

---

**Document Status:** ✅ READY FOR REVIEW  
**Estimated Implementation Time:** 4-6 hours  
**Risk Level:** LOW (backward compatible, well-tested pattern)
