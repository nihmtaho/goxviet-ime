# Phase 7: FFI API v2 Implementation Summary

**Date:** 2026-02-11  
**Status:** Implementation Complete, Ready for Testing  
**Progress:** 2/11 tasks (18%)

---

## 📊 Executive Summary

Phase 7 focuses on fixing the Swift FFI ABI issue discovered in Phase 6 by redesigning the FFI API to use out parameters instead of struct-return. This phase also prepares for v2.0.0 release by marking legacy code as deprecated while maintaining backward compatibility.

**Current Status:**
- ✅ API v2 Design Complete (18KB specification)
- ✅ API v2 Implementation Complete (ready for testing)
- ⏳ Testing in progress (next step)

---

## ✅ Completed Tasks (2/11)

### 1. ✅ Design New FFI API (`phase7-ffi-fix-design`)

**Deliverable:** `FFI_API_V2_DESIGN.md` (18KB comprehensive design)

#### Problem Statement

**The Issue:**
```
Swift standalone FFI test has ABI mismatch when returning struct by value:
- C test: text='a', consumed=1 ✅ (works perfectly)
- Swift standalone: text='', consumed=0 ❌ (corrupted data)
- Xcode app: works ✅ (user confirmed stable)
```

**Root Cause:**
- Rust `#[repr(C)]` struct-return uses System V AMD64 ABI
- Swift standalone compilation may use different calling convention
- Struct returned by value is passed differently (register vs memory)
- Result: field values get corrupted across FFI boundary

**Impact:** LOW for production (Xcode app works), but breaks development testing

#### Solution: Out Parameter Pattern

**Design Principle:**
```c
// BEFORE (v1): Return struct by value → ABI issues
FfiProcessResult ime_process_key(void* engine, FfiKeyEvent key);

// AFTER (v2): Return via out parameter → ABI compatible
int32_t ime_process_key_v2(void* engine, FfiKeyEvent key, FfiProcessResult_v2* out);
```

**Benefits:**
- ✅ ABI compatible across all platforms (C, Swift, C#)
- ✅ Explicit error handling with status codes
- ✅ Clear memory ownership semantics
- ✅ Future-proof design

#### API v2 Functions Designed

```c
// Engine lifecycle
int32_t ime_create_engine_v2(void** out_engine, const FfiConfig_v2* config);
int32_t ime_destroy_engine_v2(void* engine);

// Core processing
int32_t ime_process_key_v2(void* engine, FfiKeyEvent key, FfiProcessResult_v2* out);

// Configuration
int32_t ime_get_config_v2(void* engine, FfiConfig_v2* out);
int32_t ime_set_config_v2(void* engine, const FfiConfig_v2* config);

// Utilities
int32_t ime_get_version_v2(FfiVersionInfo* out);
void ime_free_string_v2(char* str);
```

#### Status Code Design

```rust
#[repr(C)]
pub enum FfiStatusCode {
    Success = 0,
    ErrorNullPointer = -1,
    ErrorInvalidEngine = -2,
    ErrorProcessing = -3,
    ErrorPanic = -99,
}
```

#### Type Changes

**FfiConfig_v2 (Simplified):**
```rust
#[repr(C)]
pub struct FfiConfig_v2 {
    pub input_method: u8,  // 0=Telex, 1=VNI
    pub tone_style: u8,    // 0=Modern, 1=Old
    pub smart_mode: u8,    // 0=Off, 1=On
    // Note: enable_shortcuts removed
}
```

**FfiProcessResult_v2 (Out Parameter):**
```rust
#[repr(C)]
pub struct FfiProcessResult_v2 {
    pub text: *mut c_char,      // Heap-allocated string
    pub consumed: u8,           // Was input consumed?
    pub requires_backspace: u8, // Should delete previous char?
}
```

#### Migration Strategy

**Dual API Approach:**
```
v2.0.0 (Phase 7):
- Both v1 and v2 APIs available
- v1 marked #[deprecated]
- Grace period: 2-3 releases (2-3 months)

v3.0.0 (Phase 8):
- Remove v1 API completely
- Only v2 API remains
```

**Documentation Created:**
- Complete API specification with examples
- C usage examples
- Swift usage examples
- C# usage examples
- Migration guide from v1 to v2
- Performance comparison

---

### 2. ✅ Implement FFI API v2 (`phase7-ffi-implement`)

**Files Modified:**
- `core/src/presentation/ffi/types.rs` (+119 lines)
- `core/src/presentation/ffi/api.rs` (+230 lines)

#### Types Implemented

**Status Code Enum:**
```rust
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FfiStatusCode {
    Success = 0,
    ErrorNullPointer = -1,
    ErrorInvalidEngine = -2,
    ErrorProcessing = -3,
    ErrorPanic = -99,
}
```

**Process Result v2:**
```rust
#[repr(C)]
pub struct FfiProcessResult_v2 {
    pub text: *mut c_char,
    pub consumed: u8,
    pub requires_backspace: u8,
}

impl Default for FfiProcessResult_v2 {
    fn default() -> Self {
        Self {
            text: ptr::null_mut(),
            consumed: 0,
            requires_backspace: 0,
        }
    }
}
```

**Config v2:**
```rust
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FfiConfig_v2 {
    pub input_method: u8,
    pub tone_style: u8,
    pub smart_mode: u8,
}

impl Default for FfiConfig_v2 {
    fn default() -> Self {
        Self {
            input_method: 0, // Telex
            tone_style: 0,   // Modern
            smart_mode: 1,   // On
        }
    }
}
```

**Version Info:**
```rust
#[repr(C)]
pub struct FfiVersionInfo {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}
```

#### Functions Implemented

**1. Engine Lifecycle:**
```rust
#[no_mangle]
pub extern "C" fn ime_create_engine_v2(
    out_engine: *mut *mut c_void,
    config: *const FfiConfig_v2,
) -> FfiStatusCode {
    // Null check
    // Convert config v2 → v1 → EngineConfig
    // Create Container
    // Store in out parameter
    // Return status code
}

#[no_mangle]
pub extern "C" fn ime_destroy_engine_v2(engine_ptr: *mut c_void) -> FfiStatusCode {
    // Null check
    // Drop Container
    // Return status
}
```

**2. Core Processing:**
```rust
#[no_mangle]
pub extern "C" fn ime_process_key_v2(
    engine_ptr: *mut c_void,
    key: FfiKeyEvent,
    out: *mut FfiProcessResult_v2,
) -> FfiStatusCode {
    // Null checks for engine and out
    // Get Container
    // Lock processor service
    // Process key
    // Write to out parameter
    // Return status
}
```

**3. Configuration:**
```rust
#[no_mangle]
pub extern "C" fn ime_get_config_v2(
    engine_ptr: *const c_void,
    out: *mut FfiConfig_v2,
) -> FfiStatusCode {
    // Null checks
    // Get EngineConfig
    // Convert to FfiConfig v1
    // Convert to FfiConfig_v2 (strip enable_shortcuts)
    // Write to out parameter
    // Return status
}

#[no_mangle]
pub extern "C" fn ime_set_config_v2(
    engine_ptr: *mut c_void,
    config: *const FfiConfig_v2,
) -> FfiStatusCode {
    // Null checks
    // Convert v2 → v1 (add enable_shortcuts=false)
    // Convert to EngineConfig
    // Update container config
    // Return status
}
```

**4. Utilities:**
```rust
#[no_mangle]
pub extern "C" fn ime_get_version_v2(out: *mut FfiVersionInfo) -> FfiStatusCode {
    // Null check
    // Parse CARGO_PKG_VERSION
    // Write to out parameter
    // Return status
}

#[no_mangle]
pub extern "C" fn ime_free_string_v2(ptr: *mut c_char) {
    // Drop CString
    // Same as v1 implementation
}
```

#### Implementation Details

**Panic Safety:**
```rust
let result = std::panic::catch_unwind(|| {
    // All FFI logic wrapped
    // Prevents unwinding across FFI boundary
});

match result {
    Ok(status) => status,
    Err(_) => FfiStatusCode::ErrorPanic,
}
```

**Container API Usage:**
```rust
// Get processor service (returns Arc<Mutex<ProcessorService>>)
let processor = container.processor_service();
let mut locked = processor.lock().unwrap();

// Process key through service
match locked.process_key(key_event) {
    Ok(transform_result) => { /* ... */ }
    Err(_) => { /* error handling */ }
}
```

**Config Conversions:**
```rust
// v2 → v1 → EngineConfig
let ffi_v1 = FfiConfig {
    input_method: v2.input_method,
    tone_style: v2.tone_style,
    smart_mode: v2.smart_mode,
    enable_shortcuts: false, // Default for v2
};
let engine_config = to_engine_config(ffi_v1);
container.update_config(engine_config);

// EngineConfig → v1 → v2
let engine_config = container.get_config();
let ffi_v1 = from_engine_config(&engine_config);
let ffi_v2 = FfiConfig_v2 {
    input_method: ffi_v1.input_method,
    tone_style: ffi_v1.tone_style,
    smart_mode: ffi_v1.smart_mode,
};
```

#### Compilation Fixes Applied

**Issue 1:** Container doesn't have direct `process_key()` method  
**Fix:** Use `container.processor_service().lock().unwrap()` pattern

**Issue 2:** Container.get_config() returns EngineConfig, not FfiConfig  
**Fix:** Use `from_engine_config(&container.get_config())` conversion

**Issue 3:** Container doesn't have `set_config(FfiConfig)` method  
**Fix:** Convert to EngineConfig, use `container.update_config(engine_config)`

**Result:** All compilation errors resolved ✅

---

## 🚧 Current Task: Testing (3/11)

### Next: Test New FFI API (`phase7-ffi-test`)

**Goal:** Verify Swift ABI issue is resolved

#### Test Plan

**1. Create C Test (`test_ffi_v2.c`):**
```c
// Test all v2 API functions
void test_engine_lifecycle_v2();
void test_process_key_v2();
void test_config_v2();
void test_version_v2();
void test_error_handling_v2();

// Compare with v1 API
void test_v1_vs_v2_compatibility();
```

**2. Create Swift Test (`test_ffi_v2.swift`):**
```swift
// CRITICAL: This should now work (v1 failed!)
func testProcessKeyV2Standalone() {
    // Compile standalone with swiftc
    // Verify out parameter receives correct data
    // Compare with C test results
}
```

**3. Build & Run:**
```bash
# Build universal library
./scripts/rust_build_lib_universal_for_macos.sh

# Run C test
gcc -o test_ffi_v2 test_ffi_v2.c -L. -lgoxviet_core
./test_ffi_v2

# Run Swift test (standalone compilation)
swiftc test_ffi_v2.swift -L. -lgoxviet_core -o test_ffi_v2_swift
./test_ffi_v2_swift

# Compare results
```

#### Success Criteria

- ✅ C test passes (baseline)
- ✅ **Swift standalone test passes** (proves ABI fix!)
- ✅ Results match between C and Swift
- ✅ No memory leaks
- ✅ Performance similar to v1

**If Swift test passes → PRIMARY GOAL ACHIEVED! 🎉**

---

## 📋 Remaining Tasks (9/11)

### Priority 2: Deprecation Marking (3 tasks)

4. ⏳ Mark legacy deprecated
   - Add `#[deprecated]` to all legacy modules
   - Add deprecation messages with migration hints
   - Update lib.rs to expose both v1 and v2

5. ⏳ Feature flags
   - Add "legacy" feature flag to Cargo.toml
   - Make legacy code conditional on feature
   - Default: both v1 and v2 enabled

6. ⏳ Update exports
   - Hide legacy from public API docs
   - Document v2 as primary API
   - Keep v1 accessible but deprecated

### Priority 3: Documentation & Release (5 tasks)

7. ⏳ Migration guide
   - Create MIGRATION_GUIDE.md
   - v1 → v2 code examples
   - Common pitfalls and solutions
   - Performance comparison

8. ⏳ Changelog
   - Update CHANGELOG.md for v2.0.0
   - Breaking changes section
   - New features section
   - Migration notes

9. ⏳ Release notes
   - Create release notes for v2.0.0
   - Highlight ABI fix
   - Migration timeline
   - Support policy

10. ⏳ Announcement
    - Blog post
    - GitHub release
    - Community channels

11. ⏳ Monitor issues
    - Track migration issues
    - Provide support for 2-3 releases
    - Fix any discovered bugs

---

## 🎯 Impact & Benefits

### Problem Solved

**Before (v1 API):**
```
Returning struct by value → ABI mismatch
├── C test: ✅ Works
├── Swift standalone: ❌ Corrupted (text='', consumed=0)
└── Xcode app: ✅ Works (but fragile)
```

**After (v2 API):**
```
Out parameter pattern → ABI compatible
├── C test: ✅ Should work
├── Swift standalone: ✅ Should work (proving the fix!)
└── Xcode app: ✅ Still works
```

### Benefits

1. **Stability:** ABI-safe across all platforms
2. **Compatibility:** Backward compatible (dual API)
3. **Error Handling:** Explicit status codes
4. **Simplicity:** Cleaner config (removed enable_shortcuts)
5. **Future-proof:** Out parameters are standard FFI pattern

### Migration Timeline

```
v2.0.0 (Phase 7 - Current):
├── Both v1 and v2 APIs available
├── v1 marked #[deprecated]
├── Documentation promotes v2
└── Grace period: 2-3 releases

v2.1.0, v2.2.0 (Grace Period):
├── Monitor migration progress
├── Fix any v2 issues
├── Provide migration support
└── Warn about v1 removal

v3.0.0 (Phase 8 - Future):
├── Remove v1 API completely
├── Only v2 remains
├── ~9,400 LOC deleted
└── 60% code reduction
```

---

## 📊 Phase 7 Progress Summary

| Category | Tasks | Status | Progress |
|----------|-------|--------|----------|
| **API Design** | 1/1 | ✅ Complete | 100% |
| **Implementation** | 1/1 | ✅ Complete | 100% |
| **Testing** | 0/1 | ⏳ Next | 0% |
| **Deprecation** | 0/3 | ⏳ Pending | 0% |
| **Documentation** | 0/5 | ⏳ Pending | 0% |
| **TOTAL** | **2/11** | 🚧 In Progress | **18%** |

---

## 🔍 Technical Deep Dive

### ABI Issue Explanation

**What is ABI?**
- Application Binary Interface
- Defines how functions are called at assembly level
- Includes: register usage, stack layout, struct passing

**The Problem:**
```
Rust #[repr(C)]:
- Follows System V AMD64 ABI (Linux/macOS standard)
- Small structs (<= 16 bytes) returned in registers (RAX, RDX)
- Larger structs returned via hidden pointer

Swift standalone:
- May use different convention for struct-return
- Could use stack instead of registers
- Result: field values end up in wrong locations
```

**Why Xcode App Works:**
- Xcode links everything together
- Optimizer may inline or use same calling convention
- No separate compilation units → no ABI boundary

**Why Out Parameters Work:**
```
Out parameter pattern:
- Caller allocates memory
- Callee writes to provided pointer
- No struct passing across boundary
- Universal ABI compatibility
```

### Implementation Patterns

**Pattern 1: Null Safety**
```rust
if out_engine.is_null() {
    return FfiStatusCode::ErrorNullPointer;
}
```

**Pattern 2: Panic Safety**
```rust
let result = std::panic::catch_unwind(|| {
    // All FFI logic here
});
match result {
    Ok(status) => status,
    Err(_) => FfiStatusCode::ErrorPanic,
}
```

**Pattern 3: Memory Management**
```rust
// Allocate in Rust
let text = CString::new(result.text).unwrap();
out.text = text.into_raw(); // Transfer ownership

// Free in Rust
#[no_mangle]
pub extern "C" fn ime_free_string_v2(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe { drop(CString::from_raw(ptr)); }
    }
}
```

**Pattern 4: Config Conversion**
```rust
// Chain conversions
v2 → v1 → EngineConfig  // Setting
EngineConfig → v1 → v2  // Getting
```

---

## 📚 Related Documentation

- `FFI_API_V2_DESIGN.md` - Complete API specification
- `PHASE_6_INTEGRATION_TEST_REPORT.md` - Phase 6 test results
- `PHASE_6_FFI_ABI_ISSUE.md` - Original ABI issue documentation
- `FFI_API.md` - v1 API reference (to be updated)
- Future: `MIGRATION_GUIDE.md` (task 7)
- Future: `CHANGELOG.md` for v2.0.0 (task 8)

---

## ✅ Acceptance Criteria Progress

**Phase 7 Goals:**
- ✅ Swift FFI ABI issue designed to be fixed
- ✅ New FFI API implemented
- ⏳ New FFI API tested (next step)
- ⏳ All legacy modules marked deprecated
- ⏳ Feature flag allows legacy usage
- ⏳ Migration guide complete
- ⏳ v2.0.0 prepared for release
- ⏳ Grace period planned (2-3 releases)

---

**Status:** Implementation complete, ready to test and verify ABI fix! 🚀
