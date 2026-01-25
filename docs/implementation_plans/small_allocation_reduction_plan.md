# Kế hoạch triển khai: Giảm Small Allocation Overhead

## Mô tả
Instruments profiling cho thấy **52,563 small allocations** (32B/48B/64B/80B) chiếm **~4-5MB memory**. Đây là target optimization chính để đạt <10MB goal.

## Problem Issue

### Current Issues (From Instruments)
1. **Malloc 64 Bytes**: 1.77 MiB với **19,097 allocations**
   - Likely: String allocations từ Rust FFI (C string conversions)
   - Pattern: Mỗi keystroke tạo nhiều String objects

2. **Malloc 32 Bytes**: 706 KiB với **22,621 allocations**  
   - Likely: Swift Collection entries (Dictionary/Set)
   - Pattern: Per-app settings, cache entries, metadata

3. **Malloc 48 Bytes**: 517 KiB với **11,045 allocations** 🔴 HIGHLIGHTED
   - Likely: Buffer fragments hoặc intermediate objects
   - Pattern: Buffer reallocations không được reuse

4. **Malloc 80 Bytes**: 502 KiB với **6,435 allocations**
   - Likely: Larger struct allocations

### Root Causes
- **Rust FFI không reuse buffers**: Mỗi `ime_key()` call tạo mới `CString`
- **Không có String pooling**: Common characters ('a', 'e', 'i', ...) reallocate mỗi lần
- **Buffer không được clear**: Rust `Vec<char>` reallocate thay vì reuse capacity
- **Swift String overhead**: String conversions between Swift ↔ Rust ↔ C

## Các bước triển khai

### Step 1: Audit InputManager Event Loop
**File**: `platforms/macos/goxviet/goxviet/InputManager.swift`

**Check:**
1. How many String allocations per keystroke?
2. Is `CString` created multiple times?
3. Can we batch multiple keystrokes before FFI call?

**Code to review:**
```swift
// Tìm pattern này trong eventCallback
let char = String(...)  // String allocation
char.withCString { ... }  // CString allocation
```

### Step 2: Implement String Pooling (Swift Side)
**Goal**: Reuse String objects for common characters

```swift
// NEW: String pool for common Vietnamese characters
private static let commonChars: [Character: String] = {
    var pool: [Character: String] = [:]
    // Precompute common chars
    for char in "aăâeêioôơuưyáàảãạắằẳẵặấầẩẫậéèẻẽẹ..." {
        pool[char] = String(char)
    }
    return pool
}()

// USAGE in eventCallback:
let str = Self.commonChars[char] ?? String(char)
```

**Estimated savings**: Reduce ~5,000-10,000 allocations → ~500KB-1MB

### Step 3: Implement Buffer Reuse (Rust Side)
**Files**: 
- `core/src/engine/buffer.rs`
- `core/src/engine/raw_input_buffer.rs`

**Change pattern from:**
```rust
// BAD: Reallocates on every call
fn process(&mut self) {
    let mut temp = Vec::new();  // New allocation
    // ... use temp
}
```

**To:**
```rust
// GOOD: Reuse capacity
struct Engine {
    buffer_pool: Vec<char>,  // Reusable buffer
}

fn process(&mut self) {
    self.buffer_pool.clear();  // Keeps capacity
    // ... reuse buffer_pool
}
```

**Estimated savings**: Reduce ~10,000-15,000 allocations → ~1-2MB

### Step 4: Optimize CString Conversions
**File**: `platforms/macos/goxviet/goxviet/RustBridge.swift`

**Current pattern:**
```swift
func processKey(_ char: String) {
    char.withCString { ptr in
        ime_key(ptr)  // Temporary CString
    }
}
```

**Optimized pattern:**
```swift
// Reuse CString buffer
private var cstringBuffer: [CChar] = Array(repeating: 0, count: 16)

func processKey(_ char: String) {
    guard let utf8 = char.utf8CString.dropLast() else { return }
    if utf8.count < cstringBuffer.count {
        cstringBuffer.replaceSubrange(0..<utf8.count, with: utf8)
        cstringBuffer[utf8.count] = 0
        cstringBuffer.withUnsafeBufferPointer { ptr in
            ime_key(ptr.baseAddress!)
        }
    } else {
        // Fallback for long strings
        char.withCString { ime_key($0) }
    }
}
```

**Estimated savings**: Reduce ~19,000 allocations → ~1.5-2MB

### Step 5: Batch Keystroke Processing (Optional)
**Goal**: Reduce FFI call frequency

**Idea**: Collect multiple keystrokes in Swift, send batch to Rust

**Trade-off**: May increase latency by 1-3ms, need testing

**Skip if latency-sensitive**

## Proposed Changes Summary

| Change | Location | Savings | Risk |
|--------|----------|---------|------|
| String pooling | InputManager.swift | ~0.5-1MB | Low |
| Buffer reuse | engine/buffer.rs | ~1-2MB | Medium (needs testing) |
| CString optimization | RustBridge.swift | ~1.5-2MB | Low |
| **Total** | | **~3-5MB** | |

## Thời gian dự kiến
- Step 1 (Audit): 30 phút
- Step 2 (String pool): 20 phút  
- Step 3 (Buffer reuse): 45 phút (Rust changes + testing)
- Step 4 (CString opt): 30 phút
- Step 5 (Testing): 1 giờ
- **Total: ~3-3.5 giờ**

## Tài nguyên cần thiết
- Access to InputManager.swift, RustBridge.swift
- Access to core/src/engine/*.rs
- Instruments for validation profiling
- Test Vietnamese + English typing

## Implementation Order
1. Step 1 (Audit) - Understand current allocation pattern
2. Step 2 (String pool) - Quick win, low risk
3. Step 4 (CString opt) - Low risk, good savings
4. Step 3 (Buffer reuse) - Higher risk, test thoroughly
5. Re-profile with Instruments to verify savings

## Expected Results After All Steps

### Before:
```
Malloc 64B: 1.77 MiB (19,097 allocs)
Malloc 48B: 517 KiB  (11,045 allocs)
Malloc 32B: 706 KiB  (22,621 allocs)
Total:      ~3.0 MiB (52,763 allocs)
```

### After:
```
Malloc 64B: ~0.5 MiB  (~5,000 allocs)   ✅ -1.2MB
Malloc 48B: ~200 KiB  (~4,000 allocs)   ✅ -300KB
Malloc 32B: ~400 KiB  (~12,000 allocs)  ✅ -300KB
Total:      ~1.1 MiB  (~21,000 allocs)  ✅ -1.8MB saved
```

### Memory Target After This + Dictionary Lazy Load:
```
Current:           28.4 MB
- Small mallocs:   -1.8 MB
- Dictionary:      -1.4 MB (lazy load)
= Target:          ~25.2 MB
```

Still ~15MB above goal, but significant progress. Framework overhead (~18-20MB) is unavoidable.

## Validation Checklist
- [ ] Instruments shows reduction in 32B/48B/64B allocations
- [ ] Typing Vietnamese still <16ms latency
- [ ] No crashes or memory leaks
- [ ] Auto-restore still works
- [ ] Backspace behavior unchanged
- [ ] Memory after 1000 keystrokes < baseline + 1MB

## Notes
- String pooling is safe and quick win
- Buffer reuse needs careful testing for correctness
- CString optimization may have edge cases (long strings, special chars)
- Consider profiling again after each step to measure incremental gains
