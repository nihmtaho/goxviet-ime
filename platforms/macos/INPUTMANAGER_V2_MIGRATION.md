# InputManager v2 API Full Migration Plan

## Goal
Rewrite InputManager.swift to use v2 API directly with tuple returns, eliminating all v1 pointer-based calls.

## Affected Code Sections

### 1. Initialization (DONE)
- ✅ Line 66: `RustBridgeSafe.shared.initialize()` → `ime_init_v2()`

### 2. Settings Loading (DONE)
- ✅ Line 91-116: All `ime_*` calls → `ime_*_v2()` calls

### 3. Key Processing - CRITICAL CHANGES NEEDED

#### A. ESC Key Handler (Line 365-383)
**Current (v1):**
```swift
let result = ime_key(keyCode, false, false)
if let r = result {
    defer { ime_free(r) }
    if r.pointee.action == 2 { // Restore
        let backspaceCount = Int(r.pointee.backspace)
        let chars = extractChars(from: r.pointee)
        TextInjector.shared.injectSync(bs: backspaceCount, text: makeString(from: chars), ...)
    }
}
```

**New (v2):**
```swift
let (text, backspace, consumed) = ime_key_v2(keyCode, false, false)
if consumed {
    let (method, delays) = detectMethod()
    TextInjector.shared.injectSync(bs: backspace, text: text, method: method, delays: delays, proxy: proxy)
    return nil
}
```

#### B. Backspace Handler (Line 469)
**Pattern:** Same as above - replace pointer with tuple destructuring.

#### C. Main Processing (Line 506, 600)
**Pattern:** `ime_key_ext()` → tuple destructuring.

### 4. Utility Changes

#### A. Remove Helper Functions (No longer needed)
- `extractChars(from:)` - v2 returns String directly
- `makeString(from:)` - v2 returns String directly

#### B. Update Return Types
- Functions returning processed results need to adapt to tuple format

## Implementation Steps

### Step 1: Update Global Functions (in RustEngineV2.swift)
Remove shim layer, keep only clean tuple-returning functions:
```swift
func ime_key_v2(_ key: UInt16, _ caps: Bool, _ ctrl: Bool) -> (text: String, backspace: Int, consumed: Bool)
func ime_key_ext_v2(_ key: UInt16, _ caps: Bool, _ ctrl: Bool, _ shift: Bool) -> (text: String, backspace: Int, consumed: Bool)
```

### Step 2: Update ESC Key Handler
Replace pointer-based code with tuple destructuring.

### Step 3: Update Backspace Handler (Old code, false block)
Replace pointer-based code with tuple destructuring.

### Step 4: Update processKeyWithEngine
Main engine processing - replace all `ime_key_ext()` calls.

### Step 5: Remove Dead Code
- Remove `extractChars()` helper
- Remove `makeString(from:)` helper
- Remove any v1-specific types

### Step 6: Test Build
- Add RustEngineV2.swift to Xcode
- Build and verify no linker errors
- Test keyboard input

## File Changes

**Files to Modify:**
1. `RustEngineV2.swift` - Remove shim, keep clean API
2. `InputManager.swift` - Full rewrite of key processing
3. Xcode project - Add RustEngineV2.swift

**Files to Remove (later):**
1. `RustBridgeSafe.swift` - Old v1 bridge
2. `CleanArchitectureFFIBridge.swift` - Unused

## Risk Assessment

**High Risk Areas:**
- ESC key restoration logic
- Backspace coalescing logic
- Text injection timing

**Mitigation:**
- Test each function independently
- Verify backspace counts match
- Test multi-character Vietnamese words
- Test tone positioning

## Testing Checklist

- [ ] Basic typing: a → á → ấ
- [ ] Backspace: ấ → á → a → (empty)
- [ ] ESC key restoration
- [ ] Shortcuts toggle
- [ ] Input method switch (Telex ↔ VNI)
- [ ] Tone style switch (Modern ↔ Traditional)
- [ ] Per-app mode
- [ ] Rapid typing (stress test)

## Timeline

**Estimated: 2-3 hours**
- Step 1-2: 30 min (rewrite helpers)
- Step 3-4: 60 min (rewrite key processing)
- Step 5: 15 min (cleanup)
- Step 6: 30 min (testing)

## Next Actions

1. ✅ Remove shim layer from RustEngineV2.swift
2. ⏳ Rewrite ESC handler
3. ⏳ Rewrite backspace handler  
4. ⏳ Rewrite processKeyWithEngine
5. ⏳ Remove dead code
6. ⏳ Add to Xcode & test

---

*Created: 2026-02-12*  
*Status: In Progress*
