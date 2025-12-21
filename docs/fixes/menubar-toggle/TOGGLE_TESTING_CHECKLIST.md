# TOGGLE TESTING CHECKLIST

**Date:** 2025-12-20  
**Version:** 1.0.2 (SwiftUI Implementation)  
**Purpose:** Verify toggle control stability after SwiftUI migration

---

## 🎯 TESTING OBJECTIVE

Verify that the SwiftUI Toggle implementation completely resolves:
1. ❌ Intermittent color loss issues
2. ❌ Blue selection highlight conflicts
3. ✅ Stable, consistent toggle appearance

---

## ⚙️ SETUP

### Before Testing:
1. Build the app in Debug mode:
   ```bash
   cd platforms/macos/VietnameseIMEFast
   xcodebuild -project VietnameseIMEFast.xcodeproj \
              -scheme VietnameseIMEFast \
              -configuration Debug build
   ```

2. Launch the app:
   ```bash
   open build/Debug/VietnameseIMEFast.app
   ```

3. Grant Accessibility permissions if prompted

---

## 📋 TEST CASES

### Test 1: Basic Toggle Functionality
**Objective:** Verify toggle changes state correctly

- [ ] Click menu bar icon to open menu
- [ ] Observe toggle control on first menu item
- [ ] **VERIFY:** Toggle shows with proper color (green ON or gray OFF)
- [ ] Click toggle to change state
- [ ] **VERIFY:** Toggle animates smoothly
- [ ] **VERIFY:** Status bar icon changes (🇻🇳 ↔ EN)
- [ ] Click toggle again
- [ ] **VERIFY:** State changes back correctly

**Expected:** ✅ Toggle always has color, smooth state changes

---

### Test 2: Menu Close/Reopen Stability
**Objective:** Verify toggle maintains color after menu interactions

- [ ] Open menu, observe toggle color (should be visible)
- [ ] Close menu by clicking outside
- [ ] **Wait 2 seconds**
- [ ] Reopen menu
- [ ] **VERIFY:** Toggle STILL has proper color (not gray/faded)
- [ ] Repeat 10 times
- [ ] **VERIFY:** Color remains consistent 10/10 times

**Expected:** ✅ Toggle color NEVER disappears

**Previously:** ❌ Color would randomly disappear

---

### Test 3: Hover Behavior
**Objective:** Verify no blue selection highlight

- [ ] Open menu
- [ ] Hover mouse over toggle area
- [ ] **VERIFY:** NO blue highlight background
- [ ] Move mouse around toggle
- [ ] **VERIFY:** NO visual glitches or flashing
- [ ] **VERIFY:** Toggle maintains color while hovering

**Expected:** ✅ Clean hover, no highlights

**Previously:** ❌ Blue highlight when toggle had color

---

### Test 4: Click Without Highlight
**Objective:** Verify clicking doesn't trigger menu highlight

- [ ] Open menu
- [ ] Click directly on toggle switch part
- [ ] **VERIFY:** State changes
- [ ] **VERIFY:** NO blue background flash
- [ ] Click on label area (left side)
- [ ] **VERIFY:** Nothing happens (expected - no action)
- [ ] **VERIFY:** Still no blue highlight

**Expected:** ✅ Clean interaction, no menu selection visual

---

### Test 5: Keyboard Shortcut Sync
**Objective:** Verify toggle updates when keyboard shortcut used

- [ ] Open menu, observe toggle state (e.g., ON)
- [ ] Close menu
- [ ] Press keyboard shortcut (default: Cmd+Shift+V)
- [ ] Reopen menu
- [ ] **VERIFY:** Toggle state changed correctly
- [ ] **VERIFY:** Toggle still has proper color
- [ ] Press shortcut again
- [ ] Reopen menu
- [ ] **VERIFY:** Toggle state toggled back

**Expected:** ✅ Toggle syncs perfectly with shortcut

---

### Test 6: Dark Mode Compatibility
**Objective:** Verify toggle works in both light and dark mode

**In Light Mode:**
- [ ] System Preferences → Appearance → Light
- [ ] Open menu
- [ ] **VERIFY:** Toggle visible with appropriate light mode colors
- [ ] Click toggle ON
- [ ] **VERIFY:** Green color shows correctly
- [ ] Click toggle OFF
- [ ] **VERIFY:** Gray color shows correctly

**In Dark Mode:**
- [ ] System Preferences → Appearance → Dark
- [ ] Open menu
- [ ] **VERIFY:** Toggle visible with appropriate dark mode colors
- [ ] Click toggle ON
- [ ] **VERIFY:** Green color shows correctly (brighter in dark mode)
- [ ] Click toggle OFF
- [ ] **VERIFY:** Gray color shows correctly
- [ ] Close/reopen menu multiple times
- [ ] **VERIFY:** Colors remain stable

**Expected:** ✅ Automatic color adaptation, stable in both modes

---

### Test 7: Rapid Interactions
**Objective:** Verify stability under stress

- [ ] Open menu
- [ ] Click toggle rapidly 10 times
- [ ] **VERIFY:** State changes correctly each time
- [ ] **VERIFY:** No rendering glitches
- [ ] **VERIFY:** No color loss
- [ ] Close and reopen menu rapidly 10 times
- [ ] **VERIFY:** Toggle color stable after rapid reopen

**Expected:** ✅ Stable under rapid interactions

---

### Test 8: Long-term Stability
**Objective:** Verify toggle remains stable over extended use

- [ ] Open menu, close menu 50 times
- [ ] Every 10th time, check toggle color
- [ ] **VERIFY:** Color maintained 50/50 times
- [ ] Toggle state 20 times throughout
- [ ] **VERIFY:** State changes work every time
- [ ] Leave app running for 10 minutes
- [ ] Open menu
- [ ] **VERIFY:** Toggle still has color

**Expected:** ✅ 100% stability over time

**Previously:** ❌ Would lose color randomly

---

### Test 9: Menu Item Layout
**Objective:** Verify visual alignment and spacing

- [ ] Open menu
- [ ] **VERIFY:** Label text: "Vietnamese Input"
- [ ] **VERIFY:** Label positioned on left side (~16px margin)
- [ ] **VERIFY:** Toggle positioned on right side
- [ ] **VERIFY:** Proper spacing between label and toggle
- [ ] **VERIFY:** Toggle size looks proportional (not too large/small)
- [ ] **VERIFY:** Menu item height looks correct (~32px)

**Expected:** ✅ Clean, professional layout

---

### Test 10: Edge Cases
**Objective:** Test unusual scenarios

**App Restart:**
- [ ] Toggle to ON state
- [ ] Quit app completely
- [ ] Restart app
- [ ] Open menu
- [ ] **VERIFY:** Toggle shows last known state
- [ ] **VERIFY:** Toggle has proper color

**System Sleep/Wake:**
- [ ] Open menu, note toggle state
- [ ] Close menu
- [ ] Put Mac to sleep (Apple menu → Sleep)
- [ ] Wake Mac
- [ ] Open menu
- [ ] **VERIFY:** Toggle color still visible
- [ ] **VERIFY:** Toggle state preserved

**Multiple Monitor:**
- [ ] If you have multiple monitors, move between them
- [ ] Open menu on each monitor
- [ ] **VERIFY:** Toggle renders correctly on all monitors

**Expected:** ✅ Stable in all edge cases

---

## 📊 RESULTS SUMMARY

### Color Stability:
- [ ] Toggle color visible: __/10 times (should be 10/10)
- [ ] No color loss after rapid interactions: YES / NO
- [ ] Stable in dark mode: YES / NO
- [ ] Stable in light mode: YES / NO

### Highlight Issues:
- [ ] Blue highlight observed: YES / NO (should be NO)
- [ ] Any visual glitches: YES / NO (should be NO)
- [ ] Clean hover behavior: YES / NO (should be YES)

### Functionality:
- [ ] State changes correctly: YES / NO
- [ ] Syncs with keyboard shortcut: YES / NO
- [ ] Status bar icon syncs: YES / NO

---

## ✅ ACCEPTANCE CRITERIA

**MUST PASS ALL:**
- ✅ Toggle ALWAYS has color (no gray/invisible state)
- ✅ NO blue highlight in any scenario
- ✅ State changes work 100% of the time
- ✅ Stable after 50+ menu open/close cycles
- ✅ Works perfectly in light AND dark mode
- ✅ Syncs with keyboard shortcut
- ✅ No rendering glitches or animation issues

**If ANY test fails:** Issue is NOT resolved, needs more investigation.

---

## 🐛 IF TESTS FAIL

### Debugging Steps:
1. Check Console.app for errors:
   - Filter by "VietnameseIMEFast"
   - Look for SwiftUI or rendering errors

2. Verify SwiftUI import:
   ```bash
   cd platforms/macos/VietnameseIMEFast/VietnameseIMEFast
   grep "import SwiftUI" MenuToggleView.swift
   ```
   Should show: `import SwiftUI`

3. Verify NSHostingView usage:
   ```bash
   grep "NSHostingView" MenuToggleView.swift
   ```
   Should find multiple references

4. Check build warnings:
   ```bash
   xcodebuild -project VietnameseIMEFast.xcodeproj \
              -scheme VietnameseIMEFast build 2>&1 | grep warning
   ```

5. Report issue with:
   - Which test case failed
   - Screenshot or video of issue
   - Console logs
   - macOS version
   - Light or dark mode

---

## 📸 VISUAL REFERENCE

### Expected Appearance:

**Toggle ON (Vietnamese Input Enabled):**
```
┌─────────────────────────────────┐
│ Vietnamese Input        [●──]   │  ← Green toggle (right side)
└─────────────────────────────────┘
```

**Toggle OFF (Vietnamese Input Disabled):**
```
┌─────────────────────────────────┐
│ Vietnamese Input        [○──]   │  ← Gray toggle (right side)
└─────────────────────────────────┘
```

**NO blue highlight at any time!**

---

## 📝 NOTES FOR TESTERS

### What Changed:
- **Before:** NSSwitch control (AppKit)
- **After:** SwiftUI Toggle with NSHostingView
- **Why:** NSSwitch had rendering conflicts with menu selection state

### Expected Improvements:
1. ✅ **100% color stability** - Toggle NEVER loses color
2. ✅ **Zero highlight issues** - No blue background ever
3. ✅ **Better performance** - SwiftUI optimized rendering
4. ✅ **Automatic dark mode** - No manual color management

### Common Questions:
**Q: Why does it look slightly different?**  
A: SwiftUI Toggle has slightly different proportions. We use `.scaleEffect(0.8)` to match native size.

**Q: Is it slower?**  
A: No, SwiftUI rendering is highly optimized. Should be same or faster.

**Q: Will it work on older macOS?**  
A: Yes, NSHostingView available since macOS 10.15 (Catalina).

---

## ✨ SUCCESS CRITERIA

**Test Passed If:**
- ✅ All 10 test cases pass completely
- ✅ No color loss in 50+ interactions
- ✅ Zero blue highlight sightings
- ✅ Works perfectly in both light/dark mode
- ✅ Stable under stress testing

**Test Result:** PASS / FAIL

**Tester Signature:** ________________  
**Date Tested:** ________________  
**macOS Version:** ________________

---

**Last Updated:** 2025-12-20  
**Document Version:** 1.0  
**Status:** Ready for Testing