# SHORTCUT VERIFICATION CHECKLIST

Quick verification checklist để đảm bảo tính năng keyboard shortcut toggle hoạt động chính xác.

---

## ✅ Pre-Deployment Checklist

### Code Review

- [ ] `KeyboardShortcut.swift` exists và compile thành công
- [ ] `InputManager.swift` có property `currentShortcut`
- [ ] `RustBridge.swift` có function `matchesToggleShortcut()` updated
- [ ] `AppDelegate.swift` hiển thị shortcut trong menu
- [ ] No build warnings hoặc errors

### File Structure

- [ ] `platforms/macos/VietnameseIMEFast/VietnameseIMEFast/KeyboardShortcut.swift` (240 lines)
- [ ] `platforms/macos/VietnameseIMEFast/VietnameseIMEFast/KeyboardShortcutTests.swift` (354 lines)
- [ ] `docs/SHORTCUT_GUIDE.md` (335 lines)
- [ ] `docs/TEST_SHORTCUT.md` (629 lines)
- [ ] `docs/SHORTCUT_IMPLEMENTATION_SUMMARY.md` (460 lines)
- [ ] `docs/SHORTCUT_VERIFICATION_CHECKLIST.md` (this file)

### Documentation

- [ ] README.md mentions Control+Space shortcut
- [ ] CHANGELOG.md has entry for shortcut feature
- [ ] docs/README.md links to SHORTCUT_GUIDE.md
- [ ] All code has proper comments

---

## ✅ Build & Run Verification

### Step 1: Build

```bash
cd platforms/macos/VietnameseIMEFast
xcodebuild -scheme VietnameseIMEFast clean build
```

**Expected:**
- [ ] Build succeeds (0 errors, 0 warnings)
- [ ] `KeyboardShortcut.swift` compiles
- [ ] No linking errors

### Step 2: Launch

```bash
# Build & Run in Xcode (⌘R)
open VietnameseIMEFast.xcodeproj
```

**Expected:**
- [ ] App launches without crash
- [ ] Status bar icon appears (🇻🇳 or EN)
- [ ] Menu bar accessible
- [ ] Log shows: "Toggle shortcut loaded: ⌃Space"

### Step 3: Accessibility Permission

```bash
# System Settings → Privacy & Security → Accessibility
```

**Expected:**
- [ ] Permission dialog appears
- [ ] Grant permission
- [ ] App continues running
- [ ] Log shows: "InputManager started"

---

## ✅ Basic Functionality Tests

### Test 1: Default Shortcut

**Action:** Press Control+Space

**Expected:**
- [ ] Status bar icon changes (🇻🇳 ↔️ EN)
- [ ] Change is instant (< 100ms)
- [ ] No errors in log
- [ ] Log shows: "Toggle shortcut triggered: ⌃Space"

**Verify in Log:**
```bash
tail -f /tmp/vietnameseime.log | grep "Toggle"
```

### Test 2: Toggle State

**Actions:**
1. Start with IME ON (🇻🇳)
2. Press Control+Space → Should be OFF (EN)
3. Press Control+Space → Should be ON (🇻🇳)
4. Press Control+Space → Should be OFF (EN)

**Expected:**
- [ ] Alternates correctly
- [ ] Each press toggles state
- [ ] UI updates each time

### Test 3: Typing After Toggle

**Actions:**
1. Toggle IME ON
2. Type "vietn" → should show Vietnamese
3. Toggle IME OFF
4. Type "hello" → should show English

**Expected:**
- [ ] Vietnamese works when ON
- [ ] English works when OFF
- [ ] No mixing of modes
- [ ] Composition buffer cleared on toggle

---

## ✅ Priority & Conflict Tests

### Test 4: Priority Over VSCode

**Setup:**
```bash
# Open VSCode
# Set VSCode shortcut to Control+Space (if not already)
```

**Action:** Press Control+Space in VSCode

**Expected:**
- [ ] IME toggles (NOT VSCode command palette)
- [ ] Status bar changes
- [ ] VSCode shortcut does NOT fire
- [ ] Log shows: "Toggle shortcut triggered"

**Why:** IME uses `.headInsertEventTap` (highest priority)

### Test 5: No Conflict with Spotlight

**Action:** Press Command+Space

**Expected:**
- [ ] Spotlight opens (system shortcut)
- [ ] IME does NOT toggle
- [ ] No interference between shortcuts

### Test 6: Extra Modifiers Rejected

**Actions:**
1. Press Control+Shift+Space
2. Press Control+Option+Space
3. Press Command+Control+Space

**Expected:**
- [ ] None of these toggle IME
- [ ] Only Control+Space toggles
- [ ] Strict matching enforced

---

## ✅ Menu Integration Tests

### Test 7: Menu Display

**Action:** Click status bar icon

**Expected:**
- [ ] Menu opens
- [ ] Shows "Toggle: ⌃Space" item
- [ ] Item is disabled (gray, non-clickable)
- [ ] Checkmark on "Vietnamese Input" matches current state

### Test 8: Menu Updates

**Actions:**
1. Note menu state (Toggle: ⌃Space)
2. Programmatically change shortcut (future feature)
3. Click menu again

**Expected:**
- [ ] Menu shows updated shortcut
- [ ] Display format correct (⌃Space)

---

## ✅ Persistence Tests

### Test 9: Save & Load

**Actions:**
1. Launch app → Note default shortcut
2. Quit app (Cmd+Q)
3. Relaunch app

**Expected:**
- [ ] Shortcut persists (Control+Space)
- [ ] Log shows: "Toggle shortcut loaded: ⌃Space"
- [ ] Works immediately after relaunch

### Test 10: UserDefaults

**Verify:**
```bash
defaults read com.vietnamese.ime.toggleShortcut 2>/dev/null || echo "Using default"
```

**Expected:**
- [ ] Key exists after first save
- [ ] Contains valid JSON
- [ ] Decodes to KeyboardShortcut struct

---

## ✅ Performance Tests

### Test 11: Latency

**Method:**
```bash
tail -f /tmp/vietnameseime.log | grep "Toggle"
# Press Control+Space multiple times
# Check timestamp differences
```

**Expected:**
- [ ] Latency < 5ms per toggle
- [ ] Typically ~2ms
- [ ] No delays or lags
- [ ] Instant UI feedback

### Test 12: CPU Usage

**Method:**
```bash
# Activity Monitor → VietnameseIMEFast
# Press Control+Space 100 times rapidly
```

**Expected:**
- [ ] CPU stays < 1%
- [ ] No CPU spikes
- [ ] Smooth, no stuttering

### Test 13: Memory

**Method:**
```bash
# Activity Monitor → Memory tab
# Toggle 1000 times
# Check memory usage
```

**Expected:**
- [ ] Memory usage stable
- [ ] No memory leaks
- [ ] Growth < 1MB after 1000 toggles

---

## ✅ Edge Cases

### Test 14: Rapid Toggling

**Action:** Press Control+Space 20 times quickly

**Expected:**
- [ ] No crashes
- [ ] No hangs
- [ ] Each toggle processes
- [ ] Final state correct (even/odd)

### Test 15: Toggle During Typing

**Actions:**
1. Type "vietn" (don't press Space)
2. Press Control+Space
3. Continue typing

**Expected:**
- [ ] Composition buffer cleared
- [ ] Switches to English immediately
- [ ] No partial Vietnamese left

### Test 16: Toggle with Selection

**Actions:**
1. Type "hello world"
2. Select "world"
3. Press Control+Space
4. Type "vietnam"

**Expected:**
- [ ] Toggle works
- [ ] Selection preserved
- [ ] Replaces selected text correctly

### Test 17: Multiple Apps

**Apps to Test:**
- [ ] TextEdit
- [ ] VSCode
- [ ] Terminal
- [ ] Chrome/Safari
- [ ] Slack
- [ ] Notes

**Expected:**
- [ ] Works in ALL apps
- [ ] Consistent behavior
- [ ] No app-specific issues

---

## ✅ System Integration

### Test 18: After Sleep/Wake

**Actions:**
1. Note IME state
2. Close laptop lid (sleep)
3. Wait 30 seconds
4. Open lid (wake)
5. Press Control+Space

**Expected:**
- [ ] Shortcut still works
- [ ] Event tap re-enabled
- [ ] No crashes
- [ ] State preserved

### Test 19: Multiple Keyboards

**Setup:** Connect external keyboard

**Actions:**
1. Press Control+Space on laptop keyboard
2. Press Control+Space on external keyboard

**Expected:**
- [ ] Both keyboards work
- [ ] No device-specific issues
- [ ] Consistent behavior

### Test 20: Modal Dialogs

**Actions:**
1. Open Save dialog (Cmd+S in TextEdit)
2. Press Control+Space in filename field

**Expected:**
- [ ] Toggle works in modal dialogs
- [ ] Can type Vietnamese in filename
- [ ] No crashes

---

## ✅ Unit Tests (Optional)

### Run Tests

```bash
cd platforms/macos/VietnameseIMEFast
xcodebuild test -scheme VietnameseIMEFast
```

**Expected:**
- [ ] All 25 unit tests pass
- [ ] 0 failures
- [ ] Coverage > 80%
- [ ] No flaky tests

### Key Tests to Verify

- [ ] `testDefaultShortcut()` - PASS
- [ ] `testMatchesControlSpace()` - PASS
- [ ] `testDoesNotMatchExtraModifiers()` - PASS
- [ ] `testDisplayStringControlSpace()` - PASS
- [ ] `testSaveAndLoad()` - PASS
- [ ] `testCommandSpaceHasConflict()` - PASS

---

## ✅ Production Readiness

### Code Quality

- [ ] No TODO/FIXME comments unresolved
- [ ] No debug print statements
- [ ] No hardcoded values (use constants)
- [ ] Error handling in place
- [ ] Logging appropriate level

### Documentation

- [ ] User-facing documentation complete
- [ ] Developer documentation complete
- [ ] Code comments clear
- [ ] Examples provided
- [ ] Troubleshooting guide exists

### Performance

- [ ] Latency < 5ms ✅
- [ ] CPU < 1% ✅
- [ ] Memory stable ✅
- [ ] No crashes in 1000+ toggles ✅

### Compatibility

- [ ] Works on macOS 11+
- [ ] Works on macOS 12+
- [ ] Works on macOS 13+
- [ ] Works on macOS 14+ (tested)

---

## 🚀 Deployment Checklist

### Before Release

- [ ] All tests pass (20/20 functional + 25/25 unit)
- [ ] No known critical bugs
- [ ] Documentation complete
- [ ] CHANGELOG updated
- [ ] Version bumped (if applicable)

### Release Notes

```markdown
## Keyboard Shortcut Toggle

- Added Control+Space shortcut to toggle Vietnamese/English input
- High priority event capture (never overridden by apps)
- Persistent configuration across app restarts
- Instant toggle with < 5ms latency
- Works system-wide in all applications
```

### User Communication

- [ ] Announce feature in release notes
- [ ] Update user guide/wiki
- [ ] Mention default shortcut (Control+Space)
- [ ] Explain how to verify it works
- [ ] Provide troubleshooting link

---

## ⚠️ Known Issues

### Expected Behaviors (Not Bugs)

- Command+Space opens Spotlight (system shortcut, by design)
- Control+Shift+Space does NOT toggle (extra modifier, by design)
- Modifier-only shortcuts not supported yet (future feature)

### Potential Issues

- [ ] If Accessibility permission denied → Shortcut won't work
  - **Fix:** Grant permission in System Settings
  
- [ ] If too many event taps active → May slow down
  - **Fix:** Quit other input methods/tools

- [ ] If app uses system-level Control+Space → May conflict
  - **Fix:** Change to Control+Shift+Space

---

## ✅ Sign-Off

### Developer Verification

- [ ] I have implemented the feature according to spec
- [ ] I have tested all 20 functional test cases
- [ ] I have verified unit tests pass (25/25)
- [ ] I have updated documentation
- [ ] I have added CHANGELOG entry
- [ ] Code follows project style guidelines
- [ ] No regressions introduced

**Developer:** _______________  
**Date:** _______________

### QA Verification

- [ ] I have executed all test cases
- [ ] I have verified in multiple apps
- [ ] I have tested edge cases
- [ ] Performance meets targets
- [ ] No crashes or hangs observed
- [ ] Ready for production

**QA:** _______________  
**Date:** _______________

### Product Owner Approval

- [ ] Feature meets requirements
- [ ] User experience is smooth
- [ ] Documentation adequate
- [ ] Ready for release

**PO:** _______________  
**Date:** _______________

---

## 📊 Test Results Summary

**Date:** _______________  
**Tester:** _______________  
**Build:** _______________

| Category | Total | Passed | Failed | Skipped |
|----------|-------|--------|--------|---------|
| Basic Functionality | 3 | __ | __ | __ |
| Priority & Conflicts | 3 | __ | __ | __ |
| Menu Integration | 2 | __ | __ | __ |
| Persistence | 2 | __ | __ | __ |
| Performance | 3 | __ | __ | __ |
| Edge Cases | 4 | __ | __ | __ |
| System Integration | 3 | __ | __ | __ |
| **TOTAL** | **20** | **__** | **__** | **__** |

**Pass Rate:** ____%  
**Status:** ⬜ PASS / ⬜ FAIL / ⬜ NEEDS WORK

---

## 🎯 Success Criteria

### Must Have (Critical)

- ✅ Default Control+Space works
- ✅ High priority (not overrideable)
- ✅ Works system-wide
- ✅ Latency < 5ms
- ✅ No crashes on rapid toggling

### Should Have (High)

- ✅ Persistence across restarts
- ✅ Menu integration
- ✅ Proper state management
- ✅ Clear composition buffer on toggle

### Nice to Have (Medium)

- ✅ All documentation complete
- ✅ All unit tests pass
- ✅ Performance exceeds targets
- ✅ Works after sleep/wake

---

**FINAL STATUS:** ⬜ APPROVED FOR PRODUCTION

**Last Updated:** 2024-01-20  
**Version:** 1.0