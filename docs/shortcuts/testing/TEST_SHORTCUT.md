# TEST SHORTCUT - Keyboard Toggle Testing Guide

## Overview

Hướng dẫn test tính năng keyboard shortcut để chuyển đổi giữa chế độ gõ tiếng Việt và tiếng Anh.

**Default Shortcut:** Control+Space (⌃Space)

---

## Pre-Test Checklist

### ✅ Requirements

- [ ] App đã build và chạy thành công
- [ ] Accessibility permission đã được cấp (System Settings → Privacy & Security → Accessibility)
- [ ] Status bar icon hiển thị (🇻🇳 hoặc EN)
- [ ] Menu bar có item "Toggle: ⌃Space"

### ✅ Setup

```bash
# 1. Build and run
cd platforms/macos/VietnameseIMEFast
open VietnameseIMEFast.xcodeproj
# Press ⌘R to build & run

# 2. Check status bar
# Look for 🇻🇳 or EN icon in top-right corner

# 3. Enable logging (for debugging)
tail -f /tmp/vietnameseime.log
```

---

## Test Cases

### Test 1: Basic Toggle (Control+Space)

**Objective:** Verify shortcut toggles IME ON/OFF

**Steps:**
1. Open TextEdit
2. Ensure IME is ON (status bar shows 🇻🇳)
3. Press **Control+Space**
4. Status bar should change to **EN**
5. Press **Control+Space** again
6. Status bar should change to **🇻🇳**

**Expected:**
- ✅ Status bar updates immediately
- ✅ No delay or lag
- ✅ Menu item checkmark updates

**Log Output:**
```
[HH:MM:SS] Toggle shortcut triggered: ⌃Space
[HH:MM:SS] IME disabled
[HH:MM:SS] Toggle shortcut triggered: ⌃Space
[HH:MM:SS] IME enabled
```

---

### Test 2: Toggle During Typing

**Objective:** Verify composition buffer is cleared on toggle

**Steps:**
1. Type "vietn" (don't press Space)
2. Buffer should have partial text
3. Press **Control+Space** to toggle OFF
4. Try typing - should type English normally

**Expected:**
- ✅ Composition buffer cleared
- ✅ No Vietnamese characters appear after toggle
- ✅ English typing works immediately

---

### Test 3: Toggle in Different Apps

**Objective:** Verify shortcut works system-wide

**Test Apps:**
- [ ] TextEdit
- [ ] VSCode
- [ ] Terminal
- [ ] Slack
- [ ] Chrome/Safari address bar
- [ ] Notes
- [ ] Messages

**Steps for each app:**
1. Focus on the app
2. Press **Control+Space**
3. Verify status bar changes
4. Type to confirm state

**Expected:**
- ✅ Works in ALL apps
- ✅ No app can override the shortcut
- ✅ Consistent behavior everywhere

---

### Test 4: Priority Over App Shortcuts

**Objective:** Verify IME shortcut has highest priority

**Known Apps with Control+Space:**
- VSCode: "Show all commands" (can be set to Control+Space)
- Terminal: Some terminal emulators use Control+Space

**Steps:**
1. Open VSCode
2. Set VSCode shortcut to Control+Space (if not already)
3. Press **Control+Space** in VSCode
4. IME should toggle (not VSCode command palette)

**Expected:**
- ✅ IME captures shortcut FIRST
- ✅ VSCode command palette does NOT open
- ✅ IME toggle works correctly

**Explanation:**
- IME uses `.headInsertEventTap` - highest priority
- Event is swallowed (returns nil) when matched
- App-level shortcuts never receive the event

---

### Test 5: Extra Modifiers Do Not Match

**Objective:** Verify strict modifier matching

**Steps:**
1. Press **Control+Shift+Space** (extra Shift)
2. Should NOT toggle IME
3. Press **Command+Space** (wrong modifier)
4. Should open Spotlight, NOT toggle IME
5. Press **Control+Space** (correct)
6. Should toggle IME

**Expected:**
- ✅ Control+Shift+Space → No toggle (extra modifier)
- ✅ Command+Space → Spotlight opens (system shortcut)
- ✅ Control+Space → Toggle works

---

### Test 6: Rapid Toggling

**Objective:** Verify stability under rapid input

**Steps:**
1. Press **Control+Space** 10 times quickly
2. Each press should toggle state
3. Final state should be opposite of initial

**Expected:**
- ✅ No crashes or hangs
- ✅ Each toggle updates UI
- ✅ Log shows 10 toggle events
- ✅ Final state is correct (odd = toggled, even = same)

**Example Log:**
```
[12:00:00] Toggle shortcut triggered: ⌃Space
[12:00:00] IME disabled
[12:00:00] Toggle shortcut triggered: ⌃Space
[12:00:00] IME enabled
[12:00:00] Toggle shortcut triggered: ⌃Space
[12:00:00] IME disabled
... (7 more times)
```

---

### Test 7: Toggle with CapsLock

**Objective:** Verify CapsLock doesn't interfere

**Steps:**
1. Enable CapsLock
2. Press **Control+Space**
3. IME should toggle
4. Type Vietnamese → should be UPPERCASE
5. Toggle again with CapsLock still on

**Expected:**
- ✅ CapsLock doesn't prevent toggle
- ✅ Toggle works normally
- ✅ Vietnamese uppercase works (Á, Ă, Â, etc.)

---

### Test 8: Toggle State Persistence

**Objective:** Verify state persists across focus changes

**Steps:**
1. Toggle IME OFF in TextEdit
2. Switch to Terminal (Cmd+Tab)
3. Toggle IME ON
4. Switch back to TextEdit
5. State should be OFF (per-app state)
6. Switch to Terminal again
7. State should be ON

**Expected:**
- ✅ Each app remembers its own state
- ✅ Toggle affects current app only
- ✅ State persists when switching back

---

### Test 9: Menu Bar Integration

**Objective:** Verify menu displays shortcut correctly

**Steps:**
1. Click status bar icon
2. Menu opens
3. Look for "Toggle: ⌃Space" item
4. Item should be disabled (gray, not clickable)

**Expected:**
- ✅ Menu shows current shortcut
- ✅ Item is informational only
- ✅ Display format is correct (⌃Space)

---

### Test 10: Toggle with Text Selected

**Objective:** Verify toggle works with selection

**Steps:**
1. Type "hello world"
2. Select "world"
3. Press **Control+Space** to toggle
4. Selection should remain
5. Type to replace selection

**Expected:**
- ✅ Toggle works with selection
- ✅ Selection is preserved
- ✅ Can type normally after toggle

---

## Performance Tests

### Test P1: Toggle Latency

**Objective:** Measure time from keypress to UI update

**Method:**
```bash
# Check log timestamps
tail -f /tmp/vietnameseime.log | grep "Toggle shortcut"
```

**Steps:**
1. Enable DEBUG logging
2. Press Control+Space multiple times
3. Measure time between keypress and log entry

**Expected:**
- ✅ Latency < 5ms (target: ~2ms)
- ✅ No noticeable delay
- ✅ Instant UI feedback

---

### Test P2: CPU Overhead

**Objective:** Verify negligible CPU usage

**Method:**
```bash
# Use Activity Monitor
# Search for "VietnameseIMEFast"
```

**Steps:**
1. Open Activity Monitor
2. Find VietnameseIMEFast process
3. Type continuously for 30 seconds
4. Toggle ON/OFF 20 times
5. Check CPU %

**Expected:**
- ✅ CPU < 1% during normal typing
- ✅ No CPU spikes on toggle
- ✅ Idle CPU ~0%

---

### Test P3: Memory Usage

**Objective:** Verify no memory leaks on toggle

**Method:**
1. Open Activity Monitor
2. Note starting memory
3. Toggle 1000 times
4. Check memory usage

**Expected:**
- ✅ Memory stays constant
- ✅ No memory leaks
- ✅ Struct-based (zero heap allocation)

---

## Edge Cases

### Edge Case 1: Toggle During Modal Dialog

**Steps:**
1. Open Save dialog (Cmd+S in TextEdit)
2. Press Control+Space
3. IME should toggle

**Expected:**
- ✅ Toggle works in modal dialogs
- ✅ Can type Vietnamese in file names

---

### Edge Case 2: Toggle During Spotlight

**Steps:**
1. Open Spotlight (Cmd+Space)
2. Press Control+Space
3. IME should toggle

**Expected:**
- ✅ Toggle works in Spotlight
- ✅ Can search Vietnamese terms

---

### Edge Case 3: Toggle with Multiple Keyboards

**Steps:**
1. Connect external keyboard
2. Use external keyboard: Control+Space
3. Use laptop keyboard: Control+Space
4. Both should toggle

**Expected:**
- ✅ Works on all keyboards
- ✅ No device-specific issues

---

### Edge Case 4: Toggle After Sleep/Wake

**Steps:**
1. Toggle IME to specific state
2. Put Mac to sleep (close lid)
3. Wake up Mac
4. Press Control+Space

**Expected:**
- ✅ Shortcut still works after wake
- ✅ Event tap is re-enabled if needed
- ✅ State is preserved

---

## Troubleshooting

### Problem: Shortcut Not Working

**Symptoms:**
- Press Control+Space
- Nothing happens
- Status bar doesn't change

**Solutions:**

1. **Check Accessibility Permission:**
   ```
   System Settings → Privacy & Security → Accessibility
   → Ensure "VietnameseIMEFast" is checked
   ```

2. **Check Event Tap:**
   ```bash
   tail -f /tmp/vietnameseime.log | grep "InputManager started"
   # Should see: "InputManager started"
   ```

3. **Restart App:**
   - Quit app (menu bar → Quit)
   - Relaunch

4. **Check for Conflicts:**
   - Other apps might capture Control+Space
   - Try Control+Shift+Space instead

---

### Problem: Toggle Works But UI Doesn't Update

**Symptoms:**
- Control+Space toggles state (log shows toggle)
- Status bar icon doesn't change

**Solutions:**

1. **Check Notification Observers:**
   ```bash
   tail -f /tmp/vietnameseime.log | grep "updateStateChanged"
   ```

2. **Force UI Update:**
   - Click menu bar icon
   - Menu should rebuild

3. **Restart App**

---

### Problem: Shortcut Conflicts with App

**Symptoms:**
- App's shortcut fires instead of IME toggle
- Only happens in specific app

**Solutions:**

1. **Verify Priority:**
   - IME uses `.headInsertEventTap` - should be highest
   - If app still captures, it's using system-level shortcut

2. **Change Shortcut:**
   - Use Control+Shift+Space (no conflicts)
   - Or Control+Option+Space

3. **Disable App Shortcut:**
   - Go to app's settings
   - Disable or change conflicting shortcut

---

### Problem: Toggle is Slow

**Symptoms:**
- Noticeable delay between keypress and toggle
- > 100ms latency

**Solutions:**

1. **Check CPU Usage:**
   - Activity Monitor → VietnameseIMEFast
   - Should be < 1%

2. **Check Log Performance:**
   ```bash
   tail -f /tmp/vietnameseime.log | grep "Toggle"
   # Check timestamp gaps
   ```

3. **Disable Other Apps:**
   - Too many event taps can slow down system
   - Quit other input methods/tools

---

## Regression Testing

### After Code Changes

**Quick Smoke Test (5 minutes):**
- [ ] Test 1: Basic Toggle
- [ ] Test 3: Toggle in VSCode
- [ ] Test 5: Extra Modifiers
- [ ] Test 9: Menu Display

**Full Regression (20 minutes):**
- [ ] All 10 main tests
- [ ] All 3 performance tests
- [ ] All 4 edge cases

---

## Success Criteria

### Must Pass

- ✅ Basic toggle works (Test 1)
- ✅ Works system-wide (Test 3)
- ✅ High priority (Test 4)
- ✅ Strict matching (Test 5)
- ✅ Stable under rapid input (Test 6)
- ✅ Latency < 5ms (Test P1)

### Should Pass

- ✅ Toggle during typing (Test 2)
- ✅ State persistence (Test 8)
- ✅ Menu integration (Test 9)
- ✅ CPU < 1% (Test P2)
- ✅ No memory leaks (Test P3)

### Nice to Have

- ✅ All edge cases pass
- ✅ Works after sleep/wake
- ✅ Works with external keyboards

---

## Report Template

```markdown
# Shortcut Toggle Test Report

**Date:** YYYY-MM-DD
**Tester:** [Name]
**Build:** [Version/Commit]
**macOS:** [Version]

## Summary
- Total Tests: 10
- Passed: X
- Failed: Y
- Skipped: Z

## Test Results

### Basic Tests
- [ ] Test 1: Basic Toggle - PASS/FAIL
- [ ] Test 2: Toggle During Typing - PASS/FAIL
- [ ] Test 3: Toggle in Different Apps - PASS/FAIL
- [ ] Test 4: Priority Over Apps - PASS/FAIL
- [ ] Test 5: Extra Modifiers - PASS/FAIL
- [ ] Test 6: Rapid Toggling - PASS/FAIL
- [ ] Test 7: Toggle with CapsLock - PASS/FAIL
- [ ] Test 8: State Persistence - PASS/FAIL
- [ ] Test 9: Menu Integration - PASS/FAIL
- [ ] Test 10: Toggle with Selection - PASS/FAIL

### Performance Tests
- [ ] Test P1: Latency - XXms (target < 5ms)
- [ ] Test P2: CPU - XX% (target < 1%)
- [ ] Test P3: Memory - No leaks detected

### Edge Cases
- [ ] Modal Dialog - PASS/FAIL
- [ ] Spotlight - PASS/FAIL
- [ ] Multiple Keyboards - PASS/FAIL
- [ ] Sleep/Wake - PASS/FAIL

## Issues Found

### Issue 1: [Title]
**Severity:** Critical/High/Medium/Low
**Steps to Reproduce:**
1. ...
2. ...

**Expected:** ...
**Actual:** ...

## Overall Assessment

- [ ] Ready for production
- [ ] Needs minor fixes
- [ ] Needs major fixes

**Notes:** [Additional comments]
```

---

## Automated Testing (Future)

### UI Tests (XCTest)

```swift
func testToggleShortcut() {
    // Simulate Control+Space
    let event = CGEvent(keyboardEventSource: nil, 
                       virtualKey: 0x31, 
                       keyDown: true)
    event?.flags = .maskControl
    
    // Verify toggle
    XCTAssertEqual(InputManager.shared.getCurrentState(), false)
}
```

### Performance Tests

```swift
func testToggleLatency() {
    measure {
        // Toggle 100 times
        for _ in 0..<100 {
            InputManager.shared.toggleEnabled()
        }
    }
    
    // Baseline: < 5ms per toggle
}
```

---

## Conclusion

Tính năng keyboard shortcut toggle là **CRITICAL** cho user experience. Control+Space phải hoạt động **instant, reliable, và system-wide**.

**Priority:** 🔴 HIGHEST  
**Impact:** Affects every user, every typing session  
**Testing Frequency:** Before every release

---

**Last Updated:** 2024-01-20  
**Next Review:** Before v0.2.0 release