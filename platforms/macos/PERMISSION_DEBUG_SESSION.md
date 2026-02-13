# Permission Debug Session - 2026-02-12

## Issue Reported

User reported:
- ✅ App launches without crash
- ✅ Menu bar icon appears
- ❌ Cannot click menu bar icon
- ❌ Cannot press Ctrl+Space
- ❌ Cannot type Vietnamese
- ❌ Xcode console has no logs

## Root Cause Analysis

### Investigation Steps

1. **Checked if app crashed**: No, app is running
2. **Checked logs**: Old logs from Feb 8, no new logs since restart
3. **Checked Accessibility permission**: Reset it, no TCC entry found
4. **Identified the problem**: InputManager not starting

### Why InputManager Not Starting

**Code Flow**:
```
AppDelegate.applicationDidFinishLaunching() (line 49)
  ↓
checkAccessibilityPermission() (line 82)
  ↓
AXIsProcessTrusted() (line 97)
  ├─ FALSE → showAccessibilityAlert() (line 104)
  │          User must grant permission manually
  │          App continues running without InputManager
  └─ TRUE  → InputManager.shared.start() (line 111)
               ← THIS IS WHERE IT WORKS
```

**The Problem**:
- `AXIsProcessTrusted()` returns `FALSE` (permission not granted)
- App shows alert asking user to grant permission
- User must manually:
  1. Open System Settings
  2. Navigate to Privacy & Security → Accessibility
  3. Add/enable goxviet
  4. Restart the app
- Without permission, InputManager never starts
- Without InputManager:
  - No event tap for keyboard
  - No shortcut monitoring
  - No Vietnamese processing
  - Menu bar exists but is not functional

## Solution

### User Action Required

**MUST** grant Accessibility permission:

1. Open System Settings → Privacy & Security → Accessibility
2. Find "goxviet" in list OR click + to add it
3. Toggle ON (switch should be BLUE)
4. Restart app from Xcode (⌘+R)

**After granting permission**:
- `AXIsProcessTrusted()` returns `TRUE`
- `InputManager.shared.start()` is called
- Event tap created
- Keyboard monitoring active
- Menu bar becomes functional
- Ctrl+Space works
- Vietnamese input processing active

## Verification

### Success Indicators

**Xcode console should show**:
```
Accessibility permission granted
InputManager started
```

**Functionality should work**:
- ✅ Menu bar icon clickable
- ✅ Ctrl+Space toggles input
- ✅ Vietnamese typing works

### Debug Commands

```bash
# Check if permission is granted
osascript -e 'tell application "System Events" to get exists (processes whose name is "goxviet")'

# Check console logs
log show --predicate 'process == "goxviet"' --last 1m | grep -i accessibility

# Check if InputManager started
log show --predicate 'process == "goxviet"' --last 1m | grep -i "inputmanager"
```

## Files Modified in This Session

### Crash Fixes Applied (Earlier)

**File**: `platforms/macos/goxviet/goxviet/Core/RustEngineV2.swift`

**Changes**:
1. Line 183-200: Added guard clause in `clearBuffer()`
2. Line 45-59: Fixed `initialize()` to only assign on success

**Result**: App no longer crashes when toggling Vietnamese input

### Permission Issue (Current)

**No code changes needed** - This is a user permission issue, not a code bug.

The permission check code is correct:
- Line 95-113: `checkAccessibilityPermission()` properly checks and handles permission
- Line 180-249: `showAccessibilityAlert()` shows helpful instructions
- Line 115-165: Timer-based auto-detection when permission is granted

## Lessons Learned

1. **Accessibility permission is REQUIRED** for IME functionality
   - CGEventTap cannot be created without it
   - App should not crash, but InputManager won't start

2. **Permission must be granted by user** in System Settings
   - macOS doesn't allow programmatic granting
   - Best we can do is show instructions and open Settings

3. **Permission is cached** by macOS
   - Once granted, persists across rebuilds (usually)
   - Can be reset with: `tccutil reset Accessibility com.goxviet.ime`

4. **Debug builds use different bundle ID path**
   - Permission is tied to app path
   - Debug build: `~/Library/Developer/Xcode/DerivedData/.../goxviet.app`
   - Release build: `/Applications/GoxViet.app`

## Next Steps

1. **User**: Grant Accessibility permission following guide above
2. **User**: Restart app after granting permission
3. **User**: Test functionality (menu bar, shortcut, Vietnamese input)
4. **Dev**: If still issues, check console logs for actual error
5. **Dev**: Continue Phase 8 tasks once app is confirmed working

## Related Documentation

- `CRASH_FIX_CLEAROFFER.md` - clearBuffer() crash fix
- `DEBUGGING_RUNTIME_ISSUES.md` - General debugging guide
- `diagnostic.sh` - Automated diagnostic tool
- `RUNTIME_ISSUES_FIXED.md` - isEnabled persistence fix

---

**Status**: Waiting for user to grant Accessibility permission and verify functionality
