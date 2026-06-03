# FFI API v2 Design - Out Parameter Pattern

**Version:** 2.0.0  
**Date:** 2026-02-11  
**Status:** Implemented (v2) — v1 API removed in v3.0.0  
**Priority:** CRITICAL - Fixes Swift FFI ABI issue

---

## Executive Summary

**Problem:** Current FFI API returns structs by value, causing ABI incompatibility with Swift standalone compilation.

**Solution:** Redesign FFI API to return status codes and write results via out parameters.

**Impact:** Breaking change — v1 API was removed in v3.0.0. All callers must use v2.

---

## Design Principles

1. **Out Parameters:** All complex results passed via mutable pointers
2. **Status Codes:** Return `c_int` for success/error status
3. **C89 Compatible:** Works with C, Swift, C#, and all FFI consumers
4. **v2 Only:** v1 API removed in v3.0.0 — v2 is the sole API
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

```rust
// ffi/types.rs
#[repr(C)]
pub enum FfiStatusCode {
    Success = 0,

    // Input errors
    ErrorNullEngine = -1,
    ErrorNullOutput = -2,
    ErrorNullConfig = -3,
    ErrorInvalidKey = -4,
    ErrorInvalidArgument = -5,

    // Processing errors
    ErrorProcessingFailed = -10,
    ErrorInvalidUtf8 = -11,
    ErrorParseError = -12,

    // Shortcut errors
    ErrorAlreadyExists = -30,
    ErrorNotFound = -31,

    // System errors
    ErrorOutOfMemory = -20,
    ErrorUnknown = -98,
    ErrorPanic = -99,
}

impl FfiStatusCode {
    pub const fn to_c_int(self) -> c_int {
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
    /// UTF-8 text to insert (caller must free with ime_free_string_v2)
    pub text: *mut c_char,
    /// Number of backspaces to perform
    pub backspace_count: u8,
    /// Whether the input was consumed (IME processed the key)
    pub consumed: bool,
    /// Whether the triggering key was consumed by a shortcut.
    /// When true, the platform layer must NOT re-insert the triggering character.
    pub key_consumed: bool,
}

/// Config structure (for get_config_v2 / set_config_v2)
#[repr(C)]
pub struct FfiConfig_v2 {
    pub input_method: FfiInputMethod,
    pub tone_style: FfiToneStyle,
    pub smart_mode: bool,
    pub instant_restore_enabled: bool,
    pub esc_restore_enabled: bool,
    pub enable_shortcuts: bool,
    pub bracket_shortcuts_enabled: bool,
    pub foreign_consonants_enabled: bool,
    pub auto_capitalise_enabled: bool,
    pub word_history_enabled: bool,
    pub free_tone_enabled: bool,
    pub skip_w_shortcut: bool,
}

/// Extended shortcut descriptor for ime_add_shortcut_ext_v2
/// All pointer fields are owned by the caller — not freed by Rust.
#[repr(C)]
pub struct FfiShortcutExt_v2 {
    pub trigger: *const c_char,      // Null-terminated UTF-8 (caller owns)
    pub replacement: *const c_char,  // Null-terminated UTF-8 (caller owns)
    pub trigger_condition: u8,       // 0 = OnWordBoundary, 1 = Immediate
    pub case_mode: u8,               // 0 = MatchCase, 1 = Exact
    pub enabled: bool,
    pub input_method: u8,            // 0 = All, 1 = TelexOnly, 2 = VniOnly
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

```rust
// ffi/api.rs

/// Create engine instance
/// @param config  Initial configuration (NULL for defaults)
/// @return Engine pointer on success, NULL on failure
#[no_mangle]
pub extern "C" fn ime_create_engine_v2(config: *const FfiConfig_v2) -> *mut c_void

/// Destroy engine instance (safe to pass NULL)
#[no_mangle]
pub extern "C" fn ime_destroy_engine_v2(engine_ptr: *mut c_void)

/// Reset buffer state without destroying engine (preserves shortcuts and config)
#[no_mangle]
pub extern "C" fn ime_reset_buffer_v2(engine_ptr: *mut c_void) -> FfiStatusCode

/// Reset all state including word history (preserves shortcuts and config)
/// Use when cursor moves or app switches.
#[no_mangle]
pub extern "C" fn ime_reset_all_v2(engine_ptr: *mut c_void) -> FfiStatusCode

/// Process single keystroke
/// @param engine_ptr  Engine (must not be NULL)
/// @param key_char    ASCII character
/// @param out         Output result (must not be NULL)
/// @return Status code (0 = success, <0 = error)
#[no_mangle]
pub extern "C" fn ime_process_key_v2(
    engine_ptr: *mut c_void,
    key_char: c_char,
    out: *mut FfiProcessResult_v2,
) -> c_int

/// Process key with extended modifiers (caps, shift, ctrl)
/// Required for Shift+Backspace, correct casing, modifier-aware processing.
#[no_mangle]
pub extern "C" fn ime_process_key_ext_v2(
    engine_ptr: *mut c_void,
    key_char: c_char,
    caps: bool,
    shift: bool,
    ctrl: bool,
    out: *mut FfiProcessResult_v2,
) -> c_int

/// Process key with an optional Unicode codepoint (for Option-modified keys on macOS)
/// @param char_code  Actual Unicode codepoint (0 = use keycode mapping only)
#[no_mangle]
pub extern "C" fn ime_key_with_char_v2(
    engine_ptr: *mut c_void,
    key_char: c_char,
    caps: bool,
    shift: bool,
    ctrl: bool,
    char_code: u32,
    out: *mut FfiProcessResult_v2,
) -> c_int

/// Get current configuration
#[no_mangle]
pub extern "C" fn ime_get_config_v2(engine_ptr: *mut c_void, out: *mut FfiConfig_v2) -> c_int

/// Set configuration
#[no_mangle]
pub extern "C" fn ime_set_config_v2(engine_ptr: *mut c_void, config: *const FfiConfig_v2) -> c_int

/// Get version information
#[no_mangle]
pub extern "C" fn ime_get_version_v2(out: *mut FfiVersionInfo) -> c_int

/// Free string allocated by Rust (safe to pass NULL)
#[no_mangle]
pub extern "C" fn ime_free_string_v2(ptr: *mut c_char)

/// Restore current buffer to raw ASCII input (undo all Vietnamese transforms)
#[no_mangle]
pub extern "C" fn ime_restore_to_raw_v2(engine_ptr: *mut c_void, out: *mut FfiProcessResult_v2) -> c_int

/// Export current displayed buffer as UTF-32 codepoints.
/// Returns codepoints written, or -1 on error.
#[no_mangle]
pub extern "C" fn ime_get_buffer_v2(engine: *mut c_void, out_buf: *mut u32, capacity: i64) -> i64

/// Parse a Vietnamese word back into the engine buffer (for backspace-into-word).
#[no_mangle]
pub extern "C" fn ime_restore_word_v2(engine: *mut c_void, word: *const c_char) -> c_int
```

### 4. Shortcut Management API (v2)

```rust
// ffi/api.rs

/// Add shortcut (trigger → expansion)
#[no_mangle]
pub extern "C" fn ime_add_shortcut_v2(
    engine: *mut c_void,
    trigger: *const c_char,
    expansion: *const c_char,
) -> FfiStatusCode

/// Add shortcut with extended fields (smart case, trigger condition, input method filter)
#[no_mangle]
pub extern "C" fn ime_add_shortcut_ext_v2(
    engine: *mut c_void,
    ext: *const FfiShortcutExt_v2,
) -> FfiStatusCode

/// Remove shortcut by trigger
#[no_mangle]
pub extern "C" fn ime_remove_shortcut_v2(engine: *mut c_void, trigger: *const c_char) -> FfiStatusCode

/// Clear all shortcuts
#[no_mangle]
pub extern "C" fn ime_clear_shortcuts_v2(engine: *mut c_void) -> FfiStatusCode

/// Get shortcut count
#[no_mangle]
pub extern "C" fn ime_shortcuts_count_v2(engine: *mut c_void) -> c_int

/// Enable/disable shortcuts globally
#[no_mangle]
pub extern "C" fn ime_set_shortcuts_enabled_v2(engine: *mut c_void, enabled: bool) -> FfiStatusCode
```

### 5. Input Method Config API (v2)

```rust
/// Load a data-driven InputMethodConfig from JSON bytes.
///
/// JSON format:
///   {"name":"telex","mappings":{"s":"tone_sac","f":"tone_huyen","dd":"stroke_d"}}
///
/// Does NOT require NUL terminator — use raw pointer + length.
#[no_mangle]
pub extern "C" fn ime_load_input_config_v2(
    engine_ptr: *mut c_void,
    config_json: *const u8,
    len: usize,
) -> FfiStatusCode
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
| macOS | clang | v2 | ✅ Passing | C reference |
| macOS | swiftc standalone | v2 | ✅ Passing | ABI issue resolved |
| macOS | Xcode | v2 | ✅ Passing | Production |
| Windows | MSVC | v2 | ✅ Target | C# interop |
| Windows | MinGW | v2 | ✅ Target | C interop |

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

**Document Status:** ✅ IMPLEMENTED — v2 API is live; v1 removed in v3.0.0
