# GoxViet Core - Migration Guide: v1 → v2 API

**Version:** 2.0.0  
**Date:** 2026-02-11  
**Status:** v1 API deprecated, will be removed in v3.0.0

---

## Table of Contents

1. [Overview](#overview)
2. [Why Migrate?](#why-migrate)
3. [Timeline](#timeline)
4. [Quick Start](#quick-start)
5. [API Comparison](#api-comparison)
6. [Migration by Language](#migration-by-language)
   - [C/C++](#cc)
   - [Swift (macOS)](#swift-macos)
   - [C# (Windows)](#c-windows)
7. [Common Patterns](#common-patterns)
8. [Troubleshooting](#troubleshooting)
9. [Testing Migration](#testing-migration)

---

## Overview

GoxViet Core v2.0.0 introduces a new FFI API (v2) that replaces the legacy v1 API. The v2 API uses **out parameter pattern** instead of struct-return, fixing ABI compatibility issues across all platforms.

**Key Changes:**
- ✅ Out parameter pattern (caller provides pointer)
- ✅ Explicit status codes for error handling
- ✅ No ABI issues in Swift standalone compilation
- ✅ Simpler configuration (removed shortcuts field)
- ✅ Consistent naming (`_v2` suffix)

---

## Why Migrate?

### Problems with v1 API

**❌ Swift ABI Issue:**
```swift
// v1 API - Broken in Swift standalone
let result = ime_process_key(engine, key)
// Result: text='', consumed=0 (corrupted!)
```

**❌ No Error Handling:**
```c
// v1 API - No way to detect errors
void* engine = ime_engine_new();
// Did it succeed? No way to know!
```

### Benefits of v2 API

**✅ Works Everywhere:**
```swift
// v2 API - Works in Swift standalone
var result = FfiProcessResult_v2()
let status = ime_process_key_v2(engine, key, &result)
// Result: text='a', consumed=1 (correct!)
```

**✅ Explicit Error Handling:**
```c
// v2 API - Clear status codes
void* engine = NULL;
int status = ime_create_engine_v2(&engine);
if (status != 0) {
    printf("Error: %d\n", status);
}
```

---

## Timeline

| Version | Date | Status |
|---------|------|--------|
| **v2.0.0** | 2026-02 | v1 deprecated, v2 introduced |
| **v2.1.0** | 2026-Q2 | Grace period (both APIs) |
| **v2.2.0** | 2026-Q3 | Grace period (both APIs) |
| **v3.0.0** | 2026-Q4 | **v1 API removed** |

**Recommendation:** Migrate to v2 API as soon as possible.

---

## Quick Start

### For New Projects

**Cargo.toml:**
```toml
[dependencies]
# Use v2-only (recommended)
goxviet-core = { version = "2.0", default-features = false }
```

### For Existing Projects

**Step 1:** Add v2 API alongside v1
```c
// Keep existing v1 code
ime_init();

// Add new v2 code
void* engine_v2 = NULL;
ime_create_engine_v2(&engine_v2);
```

**Step 2:** Test v2 implementation
```bash
# Compile both versions
gcc -o test_v2 test_v2.c -lgoxviet_core
./test_v2
```

**Step 3:** Replace v1 with v2
```c
// Remove all v1 API calls
// ime_init(); ❌

// Use only v2 API
void* engine = NULL;
ime_create_engine_v2(&engine); // ✅
```

**Step 4:** Test v2-only build (optional)
```toml
[dependencies]
goxviet-core = { version = "2.0", default-features = false }
```

---

## API Comparison

### Function Mapping

| v1 API (Deprecated) | v2 API (Recommended) | Notes |
|---------------------|----------------------|-------|
| `ime_engine_new()` | `ime_create_engine_v2()` | Out parameter |
| `ime_engine_new_with_config()` | `ime_create_engine_v2()` then `ime_set_config_v2()` | Split into 2 calls |
| `ime_engine_free()` | `ime_destroy_engine_v2()` | Renamed |
| `ime_process_key()` | `ime_process_key_v2()` | Out parameter, single char |
| `ime_get_config()` | `ime_get_config_v2()` | Out parameter |
| `ime_set_config()` | `ime_set_config_v2()` | In parameter |
| `ime_get_version()` | `ime_get_version_v2()` | Out parameter |
| `ime_free_string()` | `ime_free_string_v2()` | Renamed |

### Type Mapping

| v1 Type | v2 Type | Changes |
|---------|---------|---------|
| `FfiProcessResult` | `FfiProcessResult_v2` | - Removed `result` field<br>- `backspace_count`: `i32` → `u8` |
| `FfiConfig` | `FfiConfigInfo` | - Removed `enable_shortcuts` field<br>- Added padding for alignment |
| `FfiResult` | Status codes (`int32_t`) | - Replaced struct with status codes<br>- `0` = success, negative = error |

### Return Value Changes

**v1 API:** Returns struct by value or pointer
```c
FfiProcessResult result = ime_process_key(engine, key, action);
```

**v2 API:** Returns status code, writes to out parameter
```c
FfiProcessResult_v2 result;
int32_t status = ime_process_key_v2(engine, key, &result);
```

---

## Migration by Language

### C/C++

#### Example 1: Engine Lifecycle

**Before (v1):**
```c
#include "goxviet_core.h"

// Initialize
void* engine = ime_engine_new();

// Use engine...

// Clean up
ime_engine_free(engine);
```

**After (v2):**
```c
#include "goxviet_core.h"

// Create engine
void* engine = NULL;
int status = ime_create_engine_v2(&engine);
if (status != 0) {
    fprintf(stderr, "Failed to create engine: %d\n", status);
    return -1;
}

// Use engine...

// Destroy engine
ime_destroy_engine_v2(engine);
```

#### Example 2: Process Key

**Before (v1):**
```c
// Process keystroke
FfiProcessResult result = ime_process_key(engine, 'a', 0);

if (result.consumed) {
    // Delete result.backspace_count characters
    for (int i = 0; i < result.backspace_count; i++) {
        send_backspace();
    }
    
    // Insert text
    if (result.text != NULL) {
        insert_text(result.text);
        ime_free_string(result.text);
    }
}
```

**After (v2):**
```c
// Process keystroke
FfiProcessResult_v2 result;
int status = ime_process_key_v2(engine, 'a', &result);

if (status == 0 && result.consumed) {
    // Delete result.backspace_count characters
    for (int i = 0; i < result.backspace_count; i++) {
        send_backspace();
    }
    
    // Insert text
    if (result.text != NULL) {
        insert_text(result.text);
        ime_free_string_v2(result.text);
    }
}
```

#### Example 3: Configuration

**Before (v1):**
```c
// Get config
FfiConfig config = ime_get_config(engine);
printf("Method: %d\n", config.method);

// Modify
config.tone_style = 1;

// Set config
FfiResult result = ime_set_config(engine, config);
if (!result.success) {
    fprintf(stderr, "Failed to set config: %d\n", result.error_code);
}
```

**After (v2):**
```c
// Get config
FfiConfigInfo config;
int status = ime_get_config_v2(engine, &config);
if (status != 0) {
    fprintf(stderr, "Failed to get config: %d\n", status);
    return;
}

printf("Method: %d\n", config.method);

// Modify
config.tone_style = 1;

// Set config
status = ime_set_config_v2(engine, &config);
if (status != 0) {
    fprintf(stderr, "Failed to set config: %d\n", status);
}
```

---

### Swift (macOS)

#### Example 1: Engine Lifecycle

**Before (v1):**
```swift
import goxviet_core

class EngineWrapper {
    private var enginePtr: OpaquePointer?
    
    init() {
        enginePtr = ime_engine_new()
    }
    
    deinit {
        if let ptr = enginePtr {
            ime_engine_free(ptr)
        }
    }
}
```

**After (v2):**
```swift
import goxviet_core

class EngineWrapper {
    private var enginePtr: UnsafeMutableRawPointer?
    
    init() throws {
        var ptr: UnsafeMutableRawPointer?
        let status = ime_create_engine_v2(&ptr)
        
        guard status == 0, let engine = ptr else {
            throw EngineError.creationFailed(code: status)
        }
        
        self.enginePtr = engine
    }
    
    deinit {
        if let ptr = enginePtr {
            ime_destroy_engine_v2(ptr)
        }
    }
}

enum EngineError: Error {
    case creationFailed(code: Int32)
}
```

#### Example 2: Process Key (The Critical Fix!)

**Before (v1) - BROKEN:**
```swift
// ❌ This returns corrupted data in Swift standalone!
let result = ime_process_key(enginePtr, keyChar, 0)

// BUG: result.text = "", result.consumed = false
// Even when it should be "a", true
```

**After (v2) - FIXED:**
```swift
// ✅ This works correctly in Swift standalone!
var result = FfiProcessResult_v2()
let status = ime_process_key_v2(enginePtr, keyChar, &result)

guard status == 0 else {
    print("Process key failed: \(status)")
    return
}

if result.consumed {
    // Delete characters
    for _ in 0..<result.backspace_count {
        sendBackspace()
    }
    
    // Insert text
    if let text = result.text {
        let str = String(cString: text)
        insertText(str)
        ime_free_string_v2(text)
    }
}
```

#### Example 3: Configuration

**Before (v1):**
```swift
// Get config
let config = ime_get_config(enginePtr)
print("Method: \(config.method)")

// Set config
var newConfig = config
newConfig.tone_style = 1
let result = ime_set_config(enginePtr, newConfig)

if !result.success {
    print("Failed: \(result.error_code)")
}
```

**After (v2):**
```swift
// Get config
var config = FfiConfigInfo()
let status = ime_get_config_v2(enginePtr, &config)

guard status == 0 else {
    print("Failed to get config: \(status)")
    return
}

print("Method: \(config.method)")

// Set config
config.tone_style = 1
let setStatus = ime_set_config_v2(enginePtr, &config)

if setStatus != 0 {
    print("Failed to set config: \(setStatus)")
}
```

---

### C# (Windows)

#### Example 1: Engine Lifecycle

**Before (v1):**
```csharp
using System;
using System.Runtime.InteropServices;

class Engine {
    [DllImport("goxviet_core", CallingConvention = CallingConvention.Cdecl)]
    static extern IntPtr ime_engine_new();
    
    [DllImport("goxviet_core", CallingConvention = CallingConvention.Cdecl)]
    static extern void ime_engine_free(IntPtr engine);
    
    private IntPtr enginePtr;
    
    public Engine() {
        enginePtr = ime_engine_new();
    }
    
    public void Dispose() {
        if (enginePtr != IntPtr.Zero) {
            ime_engine_free(enginePtr);
            enginePtr = IntPtr.Zero;
        }
    }
}
```

**After (v2):**
```csharp
using System;
using System.Runtime.InteropServices;

class Engine : IDisposable {
    [DllImport("goxviet_core", CallingConvention = CallingConvention.Cdecl)]
    static extern int ime_create_engine_v2(out IntPtr engine);
    
    [DllImport("goxviet_core", CallingConvention = CallingConvention.Cdecl)]
    static extern void ime_destroy_engine_v2(IntPtr engine);
    
    private IntPtr enginePtr;
    
    public Engine() {
        int status = ime_create_engine_v2(out enginePtr);
        if (status != 0) {
            throw new Exception($"Failed to create engine: {status}");
        }
    }
    
    public void Dispose() {
        if (enginePtr != IntPtr.Zero) {
            ime_destroy_engine_v2(enginePtr);
            enginePtr = IntPtr.Zero;
        }
    }
}
```

#### Example 2: Process Key

**Before (v1):**
```csharp
[StructLayout(LayoutKind.Sequential)]
struct FfiProcessResult {
    public IntPtr text;
    public int backspace_count;
    public bool consumed;
    public FfiResult result;
}

[DllImport("goxviet_core", CallingConvention = CallingConvention.Cdecl)]
static extern FfiProcessResult ime_process_key(IntPtr engine, IntPtr keyChar, int action);

// Usage
FfiProcessResult result = ime_process_key(enginePtr, keyCharPtr, 0);
if (result.consumed) {
    // Process result...
    if (result.text != IntPtr.Zero) {
        string text = Marshal.PtrToStringAnsi(result.text);
        ime_free_string(result.text);
    }
}
```

**After (v2):**
```csharp
[StructLayout(LayoutKind.Sequential)]
struct FfiProcessResult_v2 {
    public IntPtr text;
    public byte backspace_count;
    public bool consumed;
}

[DllImport("goxviet_core", CallingConvention = CallingConvention.Cdecl)]
static extern int ime_process_key_v2(IntPtr engine, byte keyChar, out FfiProcessResult_v2 result);

// Usage
FfiProcessResult_v2 result;
int status = ime_process_key_v2(enginePtr, (byte)'a', out result);

if (status == 0 && result.consumed) {
    // Delete characters
    for (int i = 0; i < result.backspace_count; i++) {
        SendBackspace();
    }
    
    // Insert text
    if (result.text != IntPtr.Zero) {
        string text = Marshal.PtrToStringAnsi(result.text);
        InsertText(text);
        ime_free_string_v2(result.text);
    }
}
```

#### Example 3: Configuration

**Before (v1):**
```csharp
[StructLayout(LayoutKind.Sequential)]
struct FfiConfig {
    public byte method;
    public byte tone_style;
    public bool enable_shortcuts;
}

[DllImport("goxviet_core", CallingConvention = CallingConvention.Cdecl)]
static extern FfiConfig ime_get_config(IntPtr engine);

[DllImport("goxviet_core", CallingConvention = CallingConvention.Cdecl)]
static extern FfiResult ime_set_config(IntPtr engine, FfiConfig config);

// Usage
FfiConfig config = ime_get_config(enginePtr);
config.tone_style = 1;
FfiResult result = ime_set_config(enginePtr, config);
```

**After (v2):**
```csharp
[StructLayout(LayoutKind.Sequential)]
struct FfiConfigInfo {
    public byte method;
    public byte tone_style;
    // Note: enable_shortcuts removed
}

[DllImport("goxviet_core", CallingConvention = CallingConvention.Cdecl)]
static extern int ime_get_config_v2(IntPtr engine, out FfiConfigInfo config);

[DllImport("goxviet_core", CallingConvention = CallingConvention.Cdecl)]
static extern int ime_set_config_v2(IntPtr engine, ref FfiConfigInfo config);

// Usage
FfiConfigInfo config;
int status = ime_get_config_v2(enginePtr, out config);
if (status == 0) {
    config.tone_style = 1;
    status = ime_set_config_v2(enginePtr, ref config);
}
```

---

## Common Patterns

### Pattern 1: Error Handling

**v1 Pattern:**
```c
// Limited error info
FfiResult result = some_v1_function();
if (!result.success) {
    // Only have error_code
}
```

**v2 Pattern:**
```c
// Explicit status codes
int status = some_v2_function(&out_param);
switch (status) {
    case 0: // Success
        break;
    case -1: // Null engine
        break;
    case -2: // Null output
        break;
    case -3: // Processing failed
        break;
}
```

### Pattern 2: Memory Management

**v1 Pattern:**
```c
// Free with v1 function
char* str = ime_get_version();
// ... use str ...
ime_free_string(str);
```

**v2 Pattern:**
```c
// Free with v2 function
FfiVersionInfo version;
ime_get_version_v2(&version);
// ... use version.version_string ...
ime_free_string_v2(version.version_string);
```

**Important:** Always match free function with API version!
- v1 strings → `ime_free_string()`
- v2 strings → `ime_free_string_v2()`

### Pattern 3: Configuration Changes

**v1 Pattern:**
```c
// Get-modify-set
FfiConfig config = ime_get_config(engine);
config.method = 1;  // VNI
ime_set_config(engine, config);
```

**v2 Pattern:**
```c
// Get-modify-set with status checks
FfiConfigInfo config;
if (ime_get_config_v2(engine, &config) == 0) {
    config.method = 1;  // VNI
    ime_set_config_v2(engine, &config);
}
```

---

## Troubleshooting

### Issue 1: Linker Errors

**Symptom:**
```
undefined reference to `ime_create_engine_v2`
```

**Cause:** Using old library version or missing v2 symbols.

**Solution:**
```bash
# Update to v2.0.0+
cargo update goxviet-core

# Rebuild
cargo build --release
```

### Issue 2: Corrupted Data (v1 API)

**Symptom:**
```swift
// Swift: result.text is empty or garbage
let result = ime_process_key(engine, 'a', 0)
print(result.text) // ""  or garbage
```

**Cause:** Struct-return ABI mismatch in Swift standalone.

**Solution:** Use v2 API with out parameters:
```swift
var result = FfiProcessResult_v2()
ime_process_key_v2(engine, Character("a").asciiValue!, &result)
print(String(cString: result.text!)) // "a" ✅
```

### Issue 3: Compilation Errors with v2-only

**Symptom:**
```
error: cannot find function `ime_engine_new` in this scope
```

**Cause:** Building with `--no-default-features` but code still uses v1 API.

**Solution:** Either:
1. Migrate all code to v2 API, or
2. Enable legacy feature:
```toml
goxviet-core = { version = "2.0", features = ["legacy"] }
```

### Issue 4: Wrong Free Function

**Symptom:**
```
Segmentation fault / Access violation
```

**Cause:** Mixing v1 and v2 free functions.

**Solution:** Match API versions:
```c
// ❌ Wrong
char* str = ime_get_version();
ime_free_string_v2(str); // Wrong free function!

// ✅ Correct
char* str = ime_get_version();
ime_free_string(str); // Match v1

// Or use v2 consistently:
FfiVersionInfo info;
ime_get_version_v2(&info);
ime_free_string_v2(info.version_string);
```

### Issue 5: Status Code Confusion

**Symptom:**
```
// Treating status code as boolean
if (ime_process_key_v2(...)) { // Wrong!
```

**Cause:** v2 returns status codes, not booleans.

**Solution:**
```c
// ✅ Correct: Check status code
int status = ime_process_key_v2(...);
if (status == 0) { // 0 = success
    // Success
}
```

---

## Testing Migration

### Step 1: Test Both APIs Side-by-Side

```c
// test_migration.c
void test_both_apis() {
    // v1 API
    void* engine_v1 = ime_engine_new();
    // ... test v1 ...
    ime_engine_free(engine_v1);
    
    // v2 API
    void* engine_v2 = NULL;
    ime_create_engine_v2(&engine_v2);
    // ... test v2 ...
    ime_destroy_engine_v2(engine_v2);
}
```

### Step 2: Verify v2 Behavior

```c
// Ensure v2 produces same results as v1
void verify_equivalent() {
    // Setup both engines
    void* v1_engine = ime_engine_new();
    void* v2_engine = NULL;
    ime_create_engine_v2(&v2_engine);
    
    // Process same input
    FfiProcessResult v1_result = ime_process_key(v1_engine, 'a', 0);
    
    FfiProcessResult_v2 v2_result;
    ime_process_key_v2(v2_engine, 'a', &v2_result);
    
    // Compare results
    assert(strcmp(v1_result.text, v2_result.text) == 0);
    assert(v1_result.consumed == v2_result.consumed);
    
    // Cleanup
    ime_free_string(v1_result.text);
    ime_free_string_v2(v2_result.text);
    ime_engine_free(v1_engine);
    ime_destroy_engine_v2(v2_engine);
}
```

### Step 3: Test v2-only Build

```bash
# Build without v1 API
cargo build --no-default-features

# Link and run tests
gcc -o test_v2 test_v2.c -L./target/release -lgoxviet_core
./test_v2
```

### Step 4: Performance Comparison

```c
// Measure performance difference (should be minimal)
void benchmark() {
    clock_t start, end;
    
    // Benchmark v2 API
    void* engine = NULL;
    ime_create_engine_v2(&engine);
    
    start = clock();
    for (int i = 0; i < 100000; i++) {
        FfiProcessResult_v2 result;
        ime_process_key_v2(engine, 'a', &result);
        ime_free_string_v2(result.text);
    }
    end = clock();
    
    printf("v2 API: %f seconds\n", 
           (double)(end - start) / CLOCKS_PER_SEC);
    
    ime_destroy_engine_v2(engine);
}
```

---

## Summary

### Migration Checklist

- [ ] Read this guide completely
- [ ] Identify all v1 API usage in codebase
- [ ] Create v2 API implementation alongside v1
- [ ] Test v2 implementation thoroughly
- [ ] Replace v1 calls with v2 calls
- [ ] Update error handling for status codes
- [ ] Update memory management (free functions)
- [ ] Test with v2-only build (optional)
- [ ] Remove v1 API calls
- [ ] Update dependencies to v2-only (optional)

### Key Takeaways

1. **Out parameters fix ABI issues** - v2 works in Swift standalone
2. **Status codes for errors** - Explicit error handling
3. **Match API versions** - v1 strings → v1 free, v2 strings → v2 free
4. **Feature flags available** - Can disable v1 for smaller binary
5. **Timeline is clear** - v3.0.0 removes v1, migrate ASAP

### Getting Help

- **Documentation:** `core/PHASE_7_*` documents
- **Examples:** `platforms/macos/test_ffi_v2.*`
- **Issues:** File on GitHub with `migration` label

---

**Happy Migrating!** 🚀
