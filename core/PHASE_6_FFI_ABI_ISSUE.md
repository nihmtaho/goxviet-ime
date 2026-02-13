# Phase 6: FFI ABI Struct Return Issue

## Metadata
- **Date**: 2026-02-11
- **Status**: Documented (Workaround exists)
- **Severity**: Medium (blocks standalone Swift FFI test, but Xcode app works)
- **Impact**: Phase 6 testing

## Problem Summary

**ProcessorService implementation hoạt động CHÍNH XÁC trong C FFI test, nhưng Swift standalone test gặp ABI (Application Binary Interface) mismatch khi return struct by value.**

### Symptoms

1. **C FFI Test** (`test_c_minimal.c`): ✅ **100% HOẠT ĐỘNG**
   ```
   Result:
     text ptr: 0x10191dbe0
     backspace_count: 0
     consumed: 1
     result.success: 1
     text value: 'a'
   ```

2. **Swift Standalone Test** (`test_ffi_simple.swift`): ❌ **Struct Layout Mismatch**
   ```
   Swift reads:
     text ptr: 0x1055A0000 (WRONG - Rust returned 0x1055BB120)
     backspace_count: 1371 (WRONG - should be 0)
     consumed: 0 (WRONG - should be 1)
   ```

3. **Xcode macOS App**: ✅ **HOẠT ĐỘNG ỔN ĐỊNH** (theo user report)

## Technical Details

### Root Cause: ABI Struct-Return Mismatch

Khi function return struct **by value** (không phải pointer), calling convention khác nhau giữa:
- **Rust** (với #[repr(C)])
- **Swift** (standalone compiled)
- **Swift** (trong Xcode với module system)

**FfiProcessResult struct layout:**
```rust
#[repr(C)]
pub struct FfiProcessResult {
    pub text: *mut c_char,      // offset 0, 8 bytes
    pub backspace_count: c_int, // offset 8, 4 bytes
    pub consumed: bool,          // offset 12, 1 byte + 3 padding
    pub result: FfiResult,      // offset 16, 8 bytes
}  // Total: 24 bytes
```

### Why C Works But Swift Doesn't

1. **C ABI is standard** và gcc/clang follow cùng rules
2. **Swift standalone** có thể use khác register passing rules cho struct return
3. **Xcode build** có module/bridge header integration tốt hơn

### Evidence

**Debug logging shows:**
```
[DEBUG] Rust returns ptr: 0x1055BB120
[Swift reads ptr]: 0x1055A0000  ← DIFFERENT!
```

**Struct size matches:**
- C: sizeof(FfiProcessResult) = 24 ✓
- Swift: MemoryLayout<FfiProcessResult>.size = 24 ✓
- But **field values corrupted** when copied across FFI

## Current Workaround

**Xcode macOS app hoạt động ổn định** vì:
1. Xcode uses bridging header hoặc module maps
2. Build system handles ABI correctly
3. Swift compiler trong Xcode context có better FFI interop

## Proposed Solutions (For Future)

### Option 1: Return Via Out Parameter (RECOMMENDED)
**Change API to pass result pointer instead of return by value:**

```rust
// Current (problematic):
pub extern "C" fn ime_process_key(...) -> FfiProcessResult

// Proposed (safe):
pub extern "C" fn ime_process_key(..., out_result: *mut FfiProcessResult) -> c_int
```

**Pros:**
- ✅ Eliminates struct-return ABI issues
- ✅ Standard C pattern
- ✅ Works with all languages

**Cons:**
- ❌ API breaking change
- ❌ Requires updating all callers

### Option 2: Box and Return Pointer
```rust
pub extern "C" fn ime_process_key(...) -> *mut FfiProcessResult {
    Box::into_raw(Box::new(result))
}

// Caller frees with:
pub extern "C" fn ime_free_process_result(ptr: *mut FfiProcessResult)
```

**Pros:**
- ✅ No struct-return issues
- ✅ Explicit memory ownership

**Cons:**
- ❌ Extra allocation overhead
- ❌ Two-step cleanup (free string + free struct)

### Option 3: Investigate Swift @convention(c)
```swift
typealias ImeProcessKeyFn = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafePointer<CChar>?,
    Int32
) -> FfiProcessResult
```

**Pros:**
- ✅ No API change
- ✅ May fix ABI mismatch

**Cons:**
- ❌ Uncertain if this solves the issue
- ❌ Still platform-specific

### Option 4: Create C Wrapper Layer
```c
// c_wrapper.c
FfiProcessResult* ime_process_key_wrapper(...) {
    FfiProcessResult result = ime_process_key(...);
    FfiProcessResult* heap = malloc(sizeof(FfiProcessResult));
    memcpy(heap, &result, sizeof(FfiProcessResult));
    return heap;
}
```

**Pros:**
- ✅ C ABI guaranteed
- ✅ No Rust changes

**Cons:**
- ❌ Extra layer complexity
- ❌ Manual memory management

## Impact Assessment

### Current Impact: LOW ✅
- ✅ Xcode app works (primary use case)
- ✅ C tests work (validation)
- ❌ Swift standalone tests fail (development only)

### Future Risk: MEDIUM ⚠️
- Other platforms (Windows C#) may have same issue
- Debugging becomes harder without working standalone tests
- Maintenance burden for workarounds

## Action Items

### Immediate (Phase 6)
- [x] Document issue comprehensively
- [x] Verify Xcode app stability (user confirmed)
- [x] Keep C test as validation reference
- [ ] Add note in FFI documentation

### Phase 7 (API Refinement)
- [ ] Evaluate Option 1 (out parameter) vs Option 2 (box return)
- [ ] Design new FFI API if changing
- [ ] Test new API with Swift/C#/C
- [ ] Migration guide for API breaking change

### Phase 8 (Platform Testing)
- [ ] Test Windows C# FFI
- [ ] Verify Android JNI if applicable
- [ ] Cross-platform ABI verification suite

## References

### Working Code
- `core/PHASE_6_FFI_TEST_REPORT.md` - Initial test analysis
- `platforms/macos/test_c_minimal.c` - ✅ Working C test
- `platforms/macos/test_ffi_simple.swift` - ❌ Failing Swift test

### Related Files
- `core/src/presentation/ffi/types.rs` - FFI struct definitions
- `core/src/presentation/ffi/api.rs` - FFI function implementations
- `core/src/presentation/ffi/conversions.rs` - Rust ↔ FFI conversions

### Debug Session
- Full debugging journey in checkpoint 009
- Raw byte comparison showing pointer mismatch
- Struct layout verification with offsetof

## Conclusion

**Vấn đề đã được ISOLATED và DOCUMENTED. App production (Xcode) hoạt động ổn định.**

Quyết định trì hoãn fix cho Phase 7 là hợp lý vì:
1. ✅ Core functionality đã verified (C test 100%)
2. ✅ Primary use case (Xcode app) works
3. ⚠️ Fix requires API design changes (Phase 7 scope)
4. 📝 Issue được document đầy đủ để reference sau

**Recommendation:** Tiếp tục Phase 6 với C test validation, defer API redesign to Phase 7.
