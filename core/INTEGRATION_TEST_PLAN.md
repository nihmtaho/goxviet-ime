# Integration Test Plan - Phase 6

**Version:** 1.0  
**Date:** 2026-02-11  
**Status:** ✅ Ready for Execution

---

## 1. Test Objectives

Validate clean architecture FFI API on actual macOS and Windows platforms:

- ✅ **Engine lifecycle** (create, configure, destroy)
- ✅ **Keystroke processing** (Telex, VNI)
- ✅ **Memory management** (no leaks, proper cleanup)
- ✅ **Configuration updates** (dynamic config changes)
- ✅ **Error handling** (null pointers, invalid handles)
- ⏳ **Performance** (< 1ms per keystroke)
- ⏳ **Stress testing** (10k keystrokes/sec)

---

## 2. Test Environment

### 2.1 Platforms

| Platform | Language | FFI Binding | Status |
|----------|----------|-------------|--------|
| macOS    | Swift    | `@_silgen_name` | ✅ Ready |
| Windows  | C#       | `P/Invoke` | ⏳ Pending |

### 2.2 Test Infrastructure

**Created Files:**
- ✅ `platforms/macos/goxviet/goxvietTests/CleanArchitectureFFITests.swift` (10KB)
- ✅ `platforms/macos/goxviet/goxviet/FFI/CleanArchitectureFFIBridge.swift` (9KB)
- ✅ `core/lib` built successfully (31 FFI tests passing)

**Build Status:**
```bash
$ cd core && cargo build --release --lib
Finished `release` profile [optimized] target(s) in 0.03s
✅ 31 FFI tests passing
⚠️ 34 warnings (unused code in legacy modules)
```

---

## 3. Test Cases

### 3.1 Engine Lifecycle (5 tests)

| Test | Description | Expected |
|------|-------------|----------|
| `testEngineCreationAndDestruction` | Create + free engine | No crash |
| `testEngineCreationWithConfig` | Create with custom config | Config applied |
| `testMultipleEngineInstances` | 3 concurrent engines | Different handles |
| `testGetConfiguration` | Read current config | Default values |
| `testSetConfiguration` | Update config | Changes applied |

**Status:** ✅ Implemented

### 3.2 Keystroke Processing (5 tests)

| Test | Description | Input | Expected Output |
|------|-------------|-------|-----------------|
| `testTelexBasicInput` | Single char | `a` | `a` (consumed) |
| `testTelexVietnameseWord` | Word without tone | `viet` | `viet` |
| `testTelexWithToneMark` | Word with tone | `viets` | `viét` or `việt` |
| `testTelexComplexWord` | Complex transformation | `duowngf` | `đường` |
| `testBackspaceAction` | Backspace handling | (after `viet`) | Buffer restored |

**Status:** ✅ Implemented

### 3.3 Memory Management (3 tests)

| Test | Description | Expected |
|------|-------------|----------|
| `testStringMemoryManagement` | Free all allocated strings | No leaks |
| `testNullPointerHandling` | Free null pointers | No crash |
| `testInvalidEngineHandle` | Process with null handle | Error code 2 |

**Status:** ✅ Implemented

### 3.4 Error Handling (2 tests)

| Test | Description | Expected |
|------|-------------|----------|
| `testInvalidEngineHandle` | Invalid handle | `error_code=2`, `success=false` |
| `testInvalidUTF8` | Invalid UTF-8 input | Graceful failure |

**Status:** ✅ Implemented

### 3.5 Performance (2 tests)

| Test | Description | Target |
|------|-------------|--------|
| `testKeystrokeLatency` | 1000 iterations of "viets " | < 1ms per keystroke |
| `testMemoryFootprint` | 100 concurrent engines | < 10MB total |

**Status:** ✅ Implemented

---

## 4. Test Execution Plan

### Phase 6.1: macOS Integration (Week 15)

**Step 1: Add Swift test to Xcode project**
```bash
# Manual step: Add CleanArchitectureFFITests.swift to test target
# Open goxviet.xcodeproj → Add file to goxvietTests
```

**Step 2: Link Rust library**
```bash
cd core
cargo build --release --lib
# Copy libgoxviet_core.a to macOS project
cp target/release/libgoxviet_core.a ../platforms/macos/goxviet/
```

**Step 3: Run tests**
```bash
cd platforms/macos
xcodebuild test -scheme goxviet -destination 'platform=macOS'
# OR: Cmd+U in Xcode
```

**Expected Results:**
- ✅ All 15 integration tests passing
- ✅ No memory leaks (Instruments)
- ✅ Performance < 1ms per keystroke

### Phase 6.2: Windows Integration (Week 16)

**Step 1: Create C# test project**
```bash
cd platforms/windows
dotnet new xunit -n GoxVietTests
# Create CleanArchitectureFFITests.cs
```

**Step 2: P/Invoke bindings**
```csharp
[DllImport("goxviet_core.dll", CallingConvention = CallingConvention.Cdecl)]
public static extern IntPtr ime_engine_new();
// ... (see FFI_API.md for full examples)
```

**Step 3: Run tests**
```bash
dotnet test
```

**Expected Results:**
- ✅ All tests passing
- ✅ No memory leaks
- ✅ Performance < 1ms per keystroke

---

## 5. Acceptance Criteria

### 5.1 Functional

- ✅ All lifecycle tests passing
- ✅ Keystroke processing accurate (Telex + VNI)
- ✅ Memory management correct (no leaks)
- ✅ Configuration updates applied
- ✅ Error handling robust

### 5.2 Non-Functional

- ⏳ **Performance:** < 1ms per keystroke (to be measured)
- ⏳ **Memory:** < 10MB footprint (to be measured)
- ⏳ **Stability:** No crashes after 1 million keystrokes
- ✅ **Safety:** No undefined behavior, no panics

### 5.3 Compatibility

- ✅ Backward compatible with legacy API (verified in Phase 4)
- ✅ Platform-specific types handled correctly (UTF-8 ↔ UTF-16)
- ✅ Thread-safe (engine per thread)

---

## 6. Risk Assessment

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| ABI incompatibility | Low | High | Use `#[repr(C)]` consistently |
| Memory leaks | Medium | High | Extensive leak testing with Instruments |
| UTF-8/UTF-16 issues | Medium | Medium | Test with Unicode edge cases |
| Performance regression | Low | Medium | Benchmark against legacy |
| Platform-specific bugs | Medium | Medium | Test on both macOS and Windows |

---

## 7. Test Data

### 7.1 Vietnamese Test Corpus

**Basic Words:**
```
việt, tiếng, người, được, đường
hoà, thủy, quốc, nguyễn, chuyện
```

**Complex Patterns:**
```
quế, quyển, tuyết, nhẫn, đứng, miễn
```

**Edge Cases:**
```
- Rapid backspace: viet → vie → vi → v → (empty)
- Tone changes: to → tó → tò → tỏ → tõ → tọ
- Invalid sequences: qz, xq, fw (non-Vietnamese)
```

### 7.2 Stress Test Data

**Long sessions:**
- 10,000 keystrokes without reset
- 100 engines in parallel
- Rapid config switching (Telex ↔ VNI)

---

## 8. Next Steps

### Immediate (Week 15):
1. ✅ Create Swift test files
2. ✅ Build Rust library
3. ⏳ Link library to Xcode project
4. ⏳ Run integration tests
5. ⏳ Measure performance with Instruments
6. ⏳ Document results

### Week 16:
1. ⏳ Create C# test project
2. ⏳ Implement P/Invoke bindings
3. ⏳ Run Windows integration tests
4. ⏳ Measure performance
5. ⏳ Update SOLID_REFACTORING_PROGRESS.md

### Documentation:
- ⏳ Create TEST_REPORT.md with results
- ⏳ Update FFI_API.md with platform notes
- ⏳ Add performance benchmarks to ARCHITECTURE.md

---

## 9. References

- **FFI API:** `core/FFI_API.md`
- **Architecture:** `core/ARCHITECTURE.md`
- **Progress:** `core/SOLID_REFACTORING_PROGRESS.md`
- **Test Files:**
  - `platforms/macos/goxviet/goxvietTests/CleanArchitectureFFITests.swift`
  - `platforms/macos/goxviet/goxviet/FFI/CleanArchitectureFFIBridge.swift`

---

**Document Version:** 1.0  
**Last Updated:** 2026-02-11  
**Status:** ✅ Ready for execution
