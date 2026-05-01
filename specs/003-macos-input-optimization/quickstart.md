# Quickstart: macOS Input Pipeline Bug Fixes & Optimization

**Feature**: 003-macos-input-optimization  
**Date**: 2026-04-12

## What This Feature Changes

10 targeted fixes to the macOS Swift platform layer. No Rust core changes, no new dependencies, no API changes. All changes are in 4 Swift files.

## Files to Modify

| File | What Changes |
|------|-------------|
| `platforms/macos/goxviet/goxviet/Managers/Input/InputManager.swift` | CF release in `stop()`, AX query dispatch, nonisolated hot path |
| `platforms/macos/goxviet/goxviet/Managers/Injection/TextInjectionHelper.swift` | Guard against semaphore on IOKit thread |
| `platforms/macos/goxviet/goxviet/Managers/PerAppModeManagerEnhanced.swift` | Spotlight TTL cache, polling timer 5s |
| `platforms/macos/goxviet/goxviet/Core/SettingsManager.swift` | Debounced UserDefaults writes, per-app LRU eviction |

## Fix-by-Fix Summary

### Fix 1: CF Memory Leak in `InputManager.stop()` (CRITICAL)
**File**: `InputManager.swift`  
**Lines affected**: 200–208 (the `stop()` method)  
Add `CFRelease(runLoopSource)` after `CFRunLoopRemoveSource` and `CFRelease(eventTap)` after `CGEvent.tapEnable(enable: false)`.

### Fix 2: AX Query Off Main Thread (HIGH)
**File**: `InputManager.swift`  
**Lines affected**: `checkFocusedElementIsTextInput()` (~line 238), `checkSpotlightOnce()` in PerAppModeManagerEnhanced  
- Add `AXUIElementSetMessagingTimeout(systemWide, 0.05)` before every AX system-wide query.
- Dispatch `checkFocusedElementIsTextInput()` to `DispatchQueue.main.async`; store result in `cachedIsFocusedOnTextInput`.
- Event tap callback reads the cached value only.

### Fix 3: `nonisolated` Hot Path (HIGH)
**File**: `InputManager.swift`  
**Affected**: `handleEvent`, `processVietKeydown`, and all methods called synchronously from them  
Mark with `nonisolated` to resolve Swift 6 actor-isolation violations in the event tap callback.

### Fix 4: Semaphore Guard on IOKit Thread (MEDIUM)
**File**: `TextInjectionHelper.swift`  
**Lines affected**: `injectSync()` (~line 85)  
Ensure `semaphore.wait()` is only reached for background-dispatched injection calls. Add a `precondition` that `syncProxy` path never enters the semaphore section.

### Fix 5: Spotlight Detection TTL Cache (MEDIUM)
**File**: `PerAppModeManagerEnhanced.swift`  
**Lines affected**: `checkSpotlightOnce()` (~line 414), `resetSpotlightCheck()` (~line 460)  
Replace `spotlightChecked: Bool` with `lastSpotlightCheckTime: Date` + 3-second TTL. Rename `resetSpotlightCheck()` to `resetSpotlightCache()` for clarity.

### Fix 6: Polling Timer Interval (LOW/MEDIUM)
**File**: `PerAppModeManagerEnhanced.swift`  
**Lines affected**: `startPollingTimer()` (~line 365)  
Change `withTimeInterval: 1.5` to `withTimeInterval: 5.0`.

### Fix 7: Debounced UserDefaults Writes (HIGH)
**File**: `SettingsManager.swift`  
**Lines affected**: `saveToDefaults(_:value:)` helper  
Introduce `DispatchWorkItem` debounce (300ms window). Cancel previous pending work item for same key before scheduling new one.

### Fix 8: Per-App LRU Eviction (LOW)
**File**: `SettingsManager.swift`  
**Lines affected**: `setPerAppMode(bundleId:enabled:)` (~line 415)  
Replace silent-drop at capacity with LRU eviction: track `lastAccessTime` per bundleId in a parallel `[String: Date]` dictionary; evict the entry with the oldest access time.

## Building & Testing

```bash
# Build the macOS app (requires Rust universal lib to be pre-built)
./scripts/rust_build_lib_universal_for_macos.sh
open platforms/macos/goxviet/goxviet.xcodeproj
# Build & run in Xcode (Cmd+R)
```

## Manual Test Protocol

For each fix, follow the regression test before applying the fix, confirm it reproduces the bug, then apply the fix and confirm the regression test passes.

| Fix | Regression Test |
|-----|----------------|
| Fix 1 (CF leak) | Enable/disable IME 50× in menu bar; check Activity Monitor memory delta |
| Fix 2 (AX thread) | Enable Accessibility Inspector; trigger `checkFocusedElementIsTextInput` from debugger; confirm no "AX on wrong thread" crash |
| Fix 3 (nonisolated) | Enable Swift 6 strict concurrency warnings; confirm 0 actor-isolation warnings in hot-path files |
| Fix 4 (semaphore) | Type 200 WPM in a terminal app (slow injection path); confirm no keyboard freeze |
| Fix 5 (Spotlight TTL) | Open Spotlight, close, open again 5×; confirm IME mode is correctly set each time |
| Fix 6 (polling interval) | Check CPU usage with no typing for 10 min before/after fix |
| Fix 7 (UserDefaults debounce) | Change input method 10× in 1 second; confirm no latency spike on keystrokes during change |
| Fix 8 (LRU eviction) | Fill per-app dict to capacity (200 entries via debug script); add one more; confirm oldest entry removed, no silent drop |
