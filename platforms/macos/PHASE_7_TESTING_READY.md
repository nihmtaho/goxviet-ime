# Phase 7 Task 3: Test FFI API v2 - Ready to Execute

**Created:** 2026-02-11  
**Status:** 🎯 Ready to Test  
**Critical Task:** Verify Swift ABI issue is resolved

---

## ✅ What's Been Prepared

### 1. Test Suite Created (3 files)

**C Test Suite** (`test_ffi_v2.c` - 13.6KB)
- 9 comprehensive tests
- Validates all v2 API functions
- Compares v1 vs v2 results
- Memory leak checks
- Null safety validation

**Swift Standalone Test** (`test_ffi_v2.swift` - 13.6KB)
- 7 critical tests
- **Most important test:** `test_v2_process_key_simple()`
  - v1 failed here (text='', consumed=0)
  - v2 should work (text='a', consumed=1)
  - Proves out parameter fixes ABI issue
- Tests all v2 API in Swift standalone compilation
- Demonstrates v1 vs v2 ABI difference

**Build & Test Script** (`build_and_test_v2.sh` - 5.3KB)
- Automated build pipeline
- Compiles Rust library
- Compiles both C and Swift tests
- Runs both tests
- Provides summary report

### 2. Test Documentation

**Test Report Template** (`PHASE_7_FFI_V2_TEST_REPORT.md` - 8.3KB)
- Test plan and objectives
- Success criteria
- Expected results
- Manual testing instructions
- Troubleshooting guide

---

## 🎯 Critical Test: Swift ABI Validation

### The Problem (Phase 6)
```
v1 API returning struct by value:
  C test:           ✅ Works (text='a', consumed=1)
  Swift standalone: ❌ Corrupted (text='', consumed=0)
  Xcode app:        ✅ Works (user confirmed)
  
Root cause: ABI mismatch between Rust and Swift standalone
```

### The Solution (Phase 7)
```
v2 API using out parameters:
  C test:           ✅ Should work
  Swift standalone: ✅ Should work ← CRITICAL TEST
  Xcode app:        ✅ Should still work
  
Fix: Out parameters are ABI-safe across all platforms
```

### The Critical Test
```swift
// test_ffi_v2.swift - test_v2_process_key_simple()

var result = FfiProcessResult_v2(text: nil, consumed: 0, requires_backspace: 0)
ime_process_key_v2(engine, key, &result)

// CRITICAL ASSERTIONS:
assert(result.text != nil)              // Not nil
assert(String(cString: result.text!) == "a")  // 'a' not ''
assert(result.consumed == 1)            // 1 not 0

// If this passes → ABI ISSUE FIXED! 🎉
```

---

## 🚀 How to Execute

### Automated (Recommended)
```bash
cd /Users/nihmtaho/developer/personal-projects/cmlia/goxviet/platforms/macos
chmod +x build_and_test_v2.sh
./build_and_test_v2.sh
```

This will:
1. Build Rust core (`cargo build --release`)
2. Compile C test
3. Compile Swift standalone test
4. Run C test (baseline)
5. **Run Swift test (critical test!)**
6. Report results

### Manual (If Automated Fails)

**Step 1: Build Library**
```bash
cd /Users/nihmtaho/developer/personal-projects/cmlia/goxviet/core
cargo build --release
cp target/release/libgoxviet_core.a ../platforms/macos/
```

**Step 2: Test C**
```bash
cd /Users/nihmtaho/developer/personal-projects/cmlia/goxviet/platforms/macos
gcc -o test_ffi_v2 test_ffi_v2.c -L. -lgoxviet_core
./test_ffi_v2
```

**Step 3: Test Swift (CRITICAL)**
```bash
swiftc test_ffi_v2.swift -L. -lgoxviet_core -Xlinker -rpath -Xlinker @loader_path -o test_ffi_v2_swift
./test_ffi_v2_swift
```

---

## 📊 Expected Results

### C Test Output (Baseline)
```
╔════════════════════════════════════════════════════════════╗
║           GoxViet FFI API v2 Test Suite (C)               ║
╚════════════════════════════════════════════════════════════╝

[TEST 1] v2 Get Version
  📌 Version: 2.0.0
  ✅ PASS: Version info retrieved

[TEST 2] v2 Engine Lifecycle
  ✅ PASS: Lifecycle complete

[TEST 3] v2 Engine with Custom Config
  ✅ PASS: Config roundtrip successful

[TEST 4] v2 Process Key - Simple Character
  📌 Input: 'a' -> Output: 'a', consumed: 1
  ✅ PASS: Simple key processing works (ABI SAFE!)

[TEST 5] v2 Process Key - Tone Mark (Telex)
  📌 Step 1: 'a' -> 'a'
  📌 Step 2: 's' -> 'á'
  ✅ PASS: Tone mark processing works

[TEST 6] v2 Config Get/Set
  ✅ PASS: Config get/set roundtrip works

[TEST 7] v2 Null Pointer Safety
  ✅ PASS: Null pointer checks work

[TEST 8] v2 Memory Cleanup
  ✅ PASS: 100 keys processed and cleaned up

[TEST 9] v1 vs v2 Result Comparison
  📌 v1 result: text='a', consumed=1
  📌 v2 result: text='a', consumed=1
  ✅ PASS: v1 and v2 produce same results

╔════════════════════════════════════════════════════════════╗
║                      TEST SUMMARY                          ║
╠════════════════════════════════════════════════════════════╣
║  Total Tests: 9                                            ║
║  Passed:      9 ✅                                         ║
║  Failed:      0 ❌                                         ║
╚════════════════════════════════════════════════════════════╝

🎉 ALL TESTS PASSED! FFI API v2 is working correctly.
```

### Swift Test Output (CRITICAL)
```
╔════════════════════════════════════════════════════════════╗
║      GoxViet FFI API v2 Test Suite (Swift Standalone)     ║
║                                                            ║
║  ⚠️  CRITICAL: This should NOW WORK (v1 failed!)           ║
╚════════════════════════════════════════════════════════════╝

[TEST 1] v2 Get Version
  📌 Version: 2.0.0
  ✅ PASS: Version info retrieved via out parameter

[TEST 2] v2 Engine Lifecycle
  ✅ PASS: Lifecycle complete with status codes

[TEST 3] v2 Process Key - Simple Character (CRITICAL TEST!)
  📌 Input: 'a' -> Output: 'a', consumed: 1
  ✅ PASS: ✨ OUT PARAMETER PATTERN WORKS! ABI ISSUE FIXED! ✨

[TEST 4] v2 Process Key - Vietnamese Tone
  📌 Step 1: 'a' -> 'a'
  📌 Step 2: 's' -> 'á'
  ✅ PASS: Vietnamese tone processing works with out parameters

[TEST 5] v2 Config Get/Set via Out Parameters
  ✅ PASS: Config get/set via out parameters works

[TEST 6] v2 Null Pointer Safety
  ✅ PASS: Null safety checks work

[TEST 7] v1 vs v2 Comparison (ABI Issue Demonstration)
  📌 v1 result: text='', consumed=0  (ABI issue expected)
  📌 v2 result: text='a', consumed=1 (ABI safe!)
  ✅ PASS: v2 works correctly, v1 corrupted (proves ABI fix)

╔════════════════════════════════════════════════════════════╗
║                      TEST SUMMARY                          ║
╠════════════════════════════════════════════════════════════╣
║  Total Tests: 7                                            ║
║  Passed:      7 ✅                                         ║
║  Failed:      0 ❌                                         ║
╚════════════════════════════════════════════════════════════╝

🎉 ALL TESTS PASSED!
✨ Out parameter pattern fixes Swift ABI issue!
✨ FFI API v2 is production ready!
```

---

## ✅ Success Criteria

**Must achieve all:**
- ✅ C test: 9/9 tests passing
- ✅ Swift test: 7/7 tests passing
- ✅ **Critical:** Swift `test_v2_process_key_simple()` returns 'a' not ''
- ✅ **Critical:** Swift `test_v2_process_key_simple()` consumed=1 not 0
- ✅ No compilation errors
- ✅ No runtime crashes
- ✅ No memory leaks

**If ALL pass:**
- 🎉 **PRIMARY GOAL ACHIEVED**
- ✅ Swift ABI issue is resolved
- ✅ Out parameter pattern proven effective
- ✅ v2 API ready for production
- ✅ Can proceed to Phase 7 remaining tasks

---

## 🚧 If Tests Fail

### Compilation Errors
**Check:**
1. Library exports: `nm -g libgoxviet_core.a | grep ime_`
2. Should see all v2 functions with `_v2` suffix
3. Missing symbols → rebuild core library

### Runtime Errors
**Check:**
1. Null pointer errors → check FFI null safety
2. Panics → check catch_unwind wrapping
3. Memory errors → run with Address Sanitizer

### Incorrect Results
**Debug:**
1. Compare C vs Swift results
2. Add debug logging to Rust functions
3. Verify struct layout with `#[repr(C)]`
4. Check pointer ownership and lifetimes

---

## 📈 After Testing

### If Tests Pass (Expected)
1. ✅ Update `PHASE_7_FFI_V2_TEST_REPORT.md` with actual results
2. ✅ Mark `phase7-ffi-test` as done in database
3. ✅ Update progress in `SOLID_REFACTORING_PROGRESS.md`
4. ✅ Update `plan.md` with Phase 7 status (3/11 complete)
5. ✅ Proceed to next task: Mark legacy deprecated

### If Tests Fail (Unexpected)
1. ❌ Document failure mode
2. ❌ Debug and fix implementation
3. ❌ Re-test until passing
4. ❌ Update issue documentation

---

## 🎯 The Big Picture

**Phase 6 Discovery:**
- Found Swift ABI struct-return issue
- Documented in `PHASE_6_FFI_ABI_ISSUE.md`
- Deferred fix to Phase 7

**Phase 7 Solution:**
- Designed v2 API with out parameters
- Implemented all v2 functions
- Created comprehensive test suite

**This Test Phase:**
- **Validates the fix works**
- Proves out parameters solve ABI issue
- Confirms v2 API is production ready
- Enables deprecation of v1 API

**After This:**
- Mark v1 as deprecated
- Add feature flags
- Create migration guide
- Release v2.0.0
- Eventually remove v1 (Phase 8)

---

## 📝 Test Checklist

- [ ] Make script executable: `chmod +x build_and_test_v2.sh`
- [ ] Run automated test: `./build_and_test_v2.sh`
- [ ] Verify C test passes (9/9)
- [ ] **Verify Swift test passes (7/7)** ← CRITICAL
- [ ] Check Swift output: text='a', consumed=1
- [ ] Document results in test report
- [ ] Update task status in database
- [ ] Update progress document
- [ ] Create checkpoint if successful

---

**Ready to execute:** All test infrastructure is in place. Run the script to validate the v2 API and prove the ABI fix works! 🚀
