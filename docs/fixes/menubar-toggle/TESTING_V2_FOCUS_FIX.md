# Testing Instructions: v2.0.0 Focus Dimming Fix

**Date:** 2025-12-20  
**Version:** 2.0.0  
**Component:** MenuToggleView (Custom NSControl)  
**Issue:** Toggle dims when app loses focus  
**Status:** Ready for testing

---

## 🎯 Test Objective

Verify that the menu bar toggle maintains **100% vibrant colors** regardless of app focus state.

---

## 📋 Pre-Test Setup

### 1. Build the Application

```bash
cd platforms/macos/VietnameseIMEFast
xcodebuild -project VietnameseIMEFast.xcodeproj \
           -scheme VietnameseIMEFast \
           -configuration Debug \
           clean build
```

**Expected:** `** BUILD SUCCEEDED **`

### 2. Launch the Application

```bash
open /Users/$(whoami)/Library/Developer/Xcode/DerivedData/VietnameseIMEFast-*/Build/Products/Debug/VietnameseIMEFast.app
```

**Expected:** App icon appears in menu bar

---

## 🧪 Test Cases

### Test Case 1: Basic Focus State Changes ⭐️ CRITICAL

**Purpose:** Verify toggle maintains color when app loses focus

**Steps:**
1. Click menu bar icon to open menu
2. Observe toggle state (should be ON = green, or OFF = gray)
3. Note the color intensity
4. Click outside the menu to close it
5. Click on Desktop or another app (app loses focus)
6. Click menu bar icon again to reopen menu
7. Observe toggle state again

**Expected Result:**
- ✅ Toggle shows **SAME VIBRANT COLOR** in steps 2 and 7
- ✅ NO dimming when app lost focus
- ✅ Green remains bright green (if ON)
- ✅ Gray remains light gray (if OFF)

**Pass Criteria:** Colors are identical in focused and unfocused states

---

### Test Case 2: Rapid Focus Changes

**Purpose:** Verify stability during rapid focus switching

**Steps:**
1. Open menu → Observe toggle color
2. Click Desktop → Click menu bar → Observe color (repeat 5 times)
3. Switch to another app → Switch back → Open menu
4. Minimize app → Restore → Open menu
5. Use Mission Control → Return → Open menu

**Expected Result:**
- ✅ Toggle **ALWAYS** shows vibrant colors
- ✅ NO dimming at any point
- ✅ NO flashing or color transitions
- ✅ Consistent appearance across all focus changes

**Pass Criteria:** 100% color consistency across all 5+ iterations

---

### Test Case 3: Toggle State Changes During Focus Loss

**Purpose:** Verify state changes work correctly when app is unfocused

**Steps:**
1. Open menu with toggle ON (green)
2. Click toggle → Changes to OFF (gray)
3. Click outside menu to close
4. Click on Desktop (app loses focus)
5. Use keyboard shortcut to toggle Vietnamese input
6. Reopen menu

**Expected Result:**
- ✅ Toggle shows correct state (OFF after keyboard toggle)
- ✅ Color is vibrant gray (not dimmed)
- ✅ Animation was smooth when clicking in step 2
- ✅ State synchronized with keyboard shortcut

**Pass Criteria:** State correct, colors vibrant, animation smooth

---

### Test Case 4: Animation Quality

**Purpose:** Verify 0.25s animation works correctly

**Steps:**
1. Open menu
2. Click toggle 10 times rapidly
3. Observe each transition ON→OFF and OFF→ON
4. Check for smooth color change (green ↔ gray)
5. Check for smooth thumb sliding animation

**Expected Result:**
- ✅ All transitions complete smoothly
- ✅ No stutter or lag
- ✅ Thumb slides left/right (0.25s duration)
- ✅ Track color fades green ↔ gray (0.25s duration)
- ✅ Can interrupt mid-animation (click during animation)

**Pass Criteria:** Smooth 60fps animations, no visual glitches

---

### Test Case 5: Dark Mode Compatibility

**Purpose:** Verify toggle works in both light and dark modes

**Steps:**
1. System Preferences → General → Appearance → Light
2. Open menu → Observe toggle colors
3. System Preferences → General → Appearance → Dark
4. Open menu → Observe toggle colors
5. Toggle between light/dark 3 times
6. Test focus changes in both modes

**Expected Result:**
- ✅ Toggle visible and colored in light mode
- ✅ Toggle visible and colored in dark mode
- ✅ Colors appropriate for each mode
- ✅ NO dimming in either mode when focus lost
- ✅ Smooth transition when switching modes

**Pass Criteria:** Works perfectly in both appearance modes

---

### Test Case 6: Menu Integration

**Purpose:** Verify toggle doesn't interfere with menu behavior

**Steps:**
1. Open menu
2. Hover over other menu items
3. Click toggle multiple times
4. Hover over toggle
5. Right-click on toggle
6. Press Escape to close menu

**Expected Result:**
- ✅ NO blue highlight appears on toggle
- ✅ Toggle responds to clicks
- ✅ Menu stays open when clicking toggle
- ✅ NO context menu appears on right-click
- ✅ Menu closes properly with Escape
- ✅ Toggle doesn't steal focus from other items

**Pass Criteria:** Perfect menu integration, no conflicts

---

### Test Case 7: Extended Focus Loss

**Purpose:** Verify toggle maintains color over extended periods

**Steps:**
1. Open menu → Note toggle color
2. Click Desktop
3. Work in other apps for 5 minutes (browse, edit, etc.)
4. Do NOT open VietnameseIMEFast menu during this time
5. After 5 minutes, open menu again
6. Observe toggle color

**Expected Result:**
- ✅ Toggle shows **SAME VIBRANT COLOR** as step 1
- ✅ NO dimming after extended unfocused period
- ✅ State correctly preserved

**Pass Criteria:** Zero color degradation over time

---

### Test Case 8: System Events

**Purpose:** Verify toggle survives system state changes

**Steps:**
1. Open menu → Note toggle state and color
2. Lock screen (Cmd+Ctrl+Q)
3. Unlock screen
4. Open menu → Verify toggle
5. Put Mac to sleep
6. Wake Mac
7. Open menu → Verify toggle
8. Log out and log back in
9. Launch app and open menu

**Expected Result:**
- ✅ Toggle maintains color after each system event
- ✅ State persists correctly
- ✅ NO visual artifacts
- ✅ App launches correctly after logout

**Pass Criteria:** Survives all system state changes

---

## 🔍 Visual Inspection Checklist

When testing, verify these visual aspects:

### Color Comparison

**When Toggle is ON (Green):**
- [ ] In focus: Bright green (#33C759 / RGB 51,199,89)
- [ ] Out of focus: **SAME** bright green
- [ ] Colors are identical (hold a ruler/card to compare)

**When Toggle is OFF (Gray):**
- [ ] In focus: Light gray (#D9D9D9 / RGB 217,217,217)
- [ ] Out of focus: **SAME** light gray
- [ ] Colors are identical

### Animation Check

- [ ] Thumb slides smoothly (no jumps)
- [ ] Track color fades smoothly (no flashing)
- [ ] Duration feels natural (~0.25s)
- [ ] Can click during animation (interrupts properly)

### Layout Check

- [ ] Toggle aligned to right side of menu
- [ ] Label aligned to left side
- [ ] Proper spacing between elements
- [ ] Toggle size consistent (44x24 points)

---

## 📊 Results Template

### Test Summary

| Test Case | Status | Notes |
|-----------|--------|-------|
| 1. Basic Focus Changes | ⬜ Pass / ⬜ Fail | |
| 2. Rapid Focus Changes | ⬜ Pass / ⬜ Fail | |
| 3. State Changes | ⬜ Pass / ⬜ Fail | |
| 4. Animation Quality | ⬜ Pass / ⬜ Fail | |
| 5. Dark Mode | ⬜ Pass / ⬜ Fail | |
| 6. Menu Integration | ⬜ Pass / ⬜ Fail | |
| 7. Extended Focus Loss | ⬜ Pass / ⬜ Fail | |
| 8. System Events | ⬜ Pass / ⬜ Fail | |

### Overall Result

- **Version Tested:** v2.0.0
- **Date:** ___________
- **Tester:** ___________
- **macOS Version:** ___________
- **Pass/Fail:** ⬜ Pass / ⬜ Fail
- **Ready for Production:** ⬜ Yes / ⬜ No

### Issues Found (if any)

```
Issue #1:
Description: 
Steps to reproduce:
Expected:
Actual:

Issue #2:
...
```

---

## ✅ Success Criteria

For v2.0.0 to pass testing:

- [ ] **ALL** 8 test cases pass
- [ ] **ZERO** instances of dimming observed
- [ ] Colors are 100% consistent (focused vs unfocused)
- [ ] Animations are smooth (no lag/stutter)
- [ ] Dark mode works perfectly
- [ ] No visual glitches or artifacts
- [ ] No crashes or errors
- [ ] Performance is smooth (no delays)

**If ANY test fails:** Report to development team with details

---

## 🐛 Common Issues to Watch For

### Known Non-Issues (Expected Behavior)

These are NOT bugs:
- ✅ Toggle stays in place when clicking it (menu doesn't close)
- ✅ No hover effect on toggle
- ✅ No focus ring around toggle

### Potential Issues (Report if Found)

These WOULD be bugs:
- ❌ Any dimming when app loses focus
- ❌ Color flashing or flickering
- ❌ Animation stutter or lag
- ❌ Toggle becomes invisible
- ❌ State doesn't match keyboard shortcut
- ❌ Crashes when clicking toggle

---

## 📸 Screenshot Comparison

### How to Verify Colors Objectively

1. **Take screenshots:**
   - Open menu with app in focus → Screenshot
   - Click Desktop (app loses focus)
   - Reopen menu → Screenshot

2. **Compare in Preview:**
   - Open both screenshots in Preview
   - Use Digital Color Meter (Applications → Utilities)
   - Hover over toggle in each screenshot
   - Compare RGB values

3. **Expected:**
   - RGB values should be **IDENTICAL** or within ±2 points
   - No visible difference to human eye

---

## 🎓 Understanding the Fix

### What Changed in v2.0.0

**Before (v1.0.x - FAILED):**
- Used SwiftUI Toggle
- Tried to override dimming with modifiers
- System still dimmed the control

**After (v2.0.0 - SUCCESS):**
- Custom NSControl with CALayer
- Manual color management
- Complete control over appearance
- System cannot interfere

### Why This Matters

Menu bar apps should remain **visible and usable** even when not in focus. Dimmed controls look disabled and unprofessional. v2.0.0 achieves always-vibrant appearance.

---

## 📞 Support

If you encounter issues during testing:

1. **Check logs:**
   ```bash
   log stream --predicate 'subsystem == "com.vietnamese.ime"'
   ```

2. **Report bugs with:**
   - macOS version
   - Exact steps to reproduce
   - Screenshots (both focused and unfocused)
   - Console logs

3. **Reference documentation:**
   - `docs/MENUBAR_TOGGLE_CUSTOM_CONTROL.md` - Technical details
   - `docs/TOGGLE_V2_SUMMARY.md` - Quick summary
   - `docs/CHANGELOG_TOGGLE_FIX.md` - Complete history

---

## 🚀 After Testing

### If All Tests Pass

1. Update status in `CHANGELOG_TOGGLE_FIX.md`
2. Mark as production-ready
3. Proceed with deployment

### If Any Test Fails

1. Document the failure in detail
2. Take screenshots/videos
3. Report to development team
4. Do NOT deploy to production

---

**Version:** 2.0.0  
**Last Updated:** 2025-12-20  
**Status:** Ready for User Acceptance Testing

---

*"Test thoroughly, deploy confidently."*