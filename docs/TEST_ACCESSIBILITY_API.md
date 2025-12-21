# TEST ACCESSIBILITY API

> **Version:** 1.0.0  
> **Last Updated:** December 21, 2025  
> **Status:** ✅ Ready for Testing

## Overview

This document provides comprehensive testing procedures for Accessibility API features, including Spotlight support, browser address bars, and automatic app detection.

---

## Table of Contents

- [Prerequisites](#prerequisites)
- [Test Environment Setup](#test-environment-setup)
- [Test Cases](#test-cases)
  - [Spotlight Tests](#spotlight-tests)
  - [Browser Tests](#browser-tests)
  - [Modern Editor Tests](#modern-editor-tests)
  - [Terminal Tests](#terminal-tests)
  - [Detection Tests](#detection-tests)
- [Performance Validation](#performance-validation)
- [Troubleshooting](#troubleshooting)
- [Test Results Template](#test-results-template)

---

## Prerequisites

### System Requirements

- ✅ macOS 12.0 or later
- ✅ Vietnamese IME installed and running
- ✅ Accessibility permission granted
- ✅ Test applications installed (see list below)

### Granting Accessibility Permission

```bash
# Check if permission is granted
tccutil list Accessibility

# If not granted, Vietnamese IME will show alert on first run
# Go to: System Settings > Privacy & Security > Accessibility
# Enable Vietnamese IME
```

### Enable Debug Logging

```bash
# Enable logging
defaults write com.vietnamese.ime enableLogging -bool true

# View logs
tail -f ~/Library/Logs/VietnameseIME/keyboard.log
```

---

## Test Environment Setup

### Required Applications

**Install these browsers for comprehensive testing:**

```bash
# Chromium browsers
brew install --cask google-chrome brave-browser microsoft-edge

# Firefox
brew install --cask firefox

# Safari (pre-installed)

# Arc browser
brew install --cask arc

# Modern editors
brew install --cask visual-studio-code zed

# Terminals
brew install --cask iterm2
```

### Test Data

Create a text file with Vietnamese test inputs:

```
hoa → hoà
thuy → thuỷ
truong → trường
tiec → tiệc
khong → không
vietnam → việt nam
```

---

## Test Cases

### Spotlight Tests

#### TC-SPOT-001: Basic Vietnamese Input

**Objective:** Verify Vietnamese input works in Spotlight

**Steps:**
1. Press `Cmd+Space` to open Spotlight
2. Ensure Vietnamese IME is enabled (`Ctrl+Space` if needed)
3. Type: `hoa`
4. Observe transformation to `hoà`

**Expected Result:**
- ✅ Text transforms correctly to `hoà`
- ✅ No character loss
- ✅ Latency < 20ms (feels instant)

**Log Validation:**
```
INFO: detect: com.apple.Spotlight role=AXSearchField
METHOD: auto:spotlight
SEND[autocomplete] bs=3 chars=hoà
```

---

#### TC-SPOT-002: Forward Delete Clears Suggestions

**Objective:** Verify Forward Delete clears auto-selected suggestions

**Steps:**
1. Open Spotlight (`Cmd+Space`)
2. Type: `vsc` (triggers VSCode suggestion)
3. Without selecting suggestion, type: `o` (for "vsco")
4. Enable Vietnamese IME
5. Type: `d` (should trigger Forward Delete first)

**Expected Result:**
- ✅ Suggestion is cleared before text injection
- ✅ Vietnamese text appears correctly
- ✅ No suggestion text mixed with typed text

---

#### TC-SPOT-003: Backspace Behavior

**Objective:** Verify backspace works correctly in Spotlight

**Steps:**
1. Open Spotlight
2. Type: `truong` → should become `trường`
3. Press backspace once
4. Observe result

**Expected Result:**
- ✅ Backspace removes entire syllable or character correctly
- ✅ Buffer state is consistent
- ✅ Can continue typing after backspace

---

### Browser Tests

#### TC-BROWSER-001: Chrome Address Bar

**Objective:** Verify Vietnamese input in Chrome address bar

**Steps:**
1. Open Google Chrome
2. Click address bar (Cmd+L)
3. Type: `hoa`
4. Observe transformation

**Expected Result:**
- ✅ Transforms to `hoà`
- ✅ Selection method used (Shift+Left)
- ✅ Latency < 8ms
- ✅ Autocomplete suggestions not affected

**Log Validation:**
```
INFO: detect: com.google.Chrome role=AXTextField
METHOD: sel:browser
SEND[selection] bs=3 chars=hoà
```

---

#### TC-BROWSER-002: Arc Browser Address Bar

**Objective:** Verify Arc browser support (special test for Arc)

**Steps:**
1. Open Arc browser
2. Press `Cmd+T` (new tab)
3. Address bar should be focused
4. Type: `thuy`
5. Observe transformation to `thuỷ`

**Expected Result:**
- ✅ Arc detected: `company.thebrowser.Arc`
- ✅ Role: `AXTextField`
- ✅ Selection method applied
- ✅ Text transforms correctly
- ✅ No interference with Arc's autocomplete

**Log Validation:**
```
INFO: detect: company.thebrowser.Arc role=AXTextField
METHOD: sel:browser
SEND[selection] bs=4 chars=thuỷ
```

---

#### TC-BROWSER-003: Firefox Address Bar

**Objective:** Verify Firefox support

**Steps:**
1. Open Firefox
2. Focus address bar (Cmd+L)
3. Type: `vietnam`
4. Observe transformation

**Expected Result:**
- ✅ Transforms to `việt nam` correctly
- ✅ Firefox detected: `org.mozilla.firefox`
- ✅ Selection method used

---

#### TC-BROWSER-004: Safari Address Bar

**Objective:** Verify Safari support

**Steps:**
1. Open Safari
2. Focus address bar (Cmd+L)
3. Type: `khong`
4. Observe transformation to `không`

**Expected Result:**
- ✅ Safari detected: `com.apple.Safari`
- ✅ Selection method applied
- ✅ Works with Safari's autocomplete

---

#### TC-BROWSER-005: Browser Content Area

**Objective:** Verify Vietnamese input in web page text fields

**Steps:**
1. Open any browser
2. Navigate to: `https://www.google.com`
3. Click search box
4. Type: `tiec`
5. Observe transformation

**Expected Result:**
- ✅ Transforms to `tiệc`
- ✅ Fast method used (not selection)
- ✅ Latency < 15ms

**Log Validation:**
```
INFO: detect: com.google.Chrome role=AXTextField
METHOD: default (or fast)
SEND[fast] bs=4 chars=tiệc
```

---

### Modern Editor Tests

#### TC-EDITOR-001: VSCode Instant Method

**Objective:** Verify zero-delay instant method in VSCode

**Steps:**
1. Open Visual Studio Code
2. Create new file (Cmd+N)
3. Type rapidly: `hoa thuy truong`
4. Observe transformations

**Expected Result:**
- ✅ All syllables transform correctly
- ✅ Instant method detected
- ✅ Zero delays between events
- ✅ Total latency < 10ms per syllable

**Log Validation:**
```
INFO: detect: com.microsoft.VSCode role=nil
METHOD: instant:editor
SEND[instant] bs=3 chars=hoà
SEND[instant] bs=4 chars=thuỷ
SEND[instant] bs=7 chars=trường
```

---

#### TC-EDITOR-002: Zed Editor Performance

**Objective:** Verify Zed editor uses instant method

**Steps:**
1. Open Zed
2. Create new buffer
3. Type: `vietnam khong tiec`
4. Measure responsiveness

**Expected Result:**
- ✅ Zed detected: `dev.zed.Zed`
- ✅ Instant method applied
- ✅ Feels instantaneous
- ✅ No visible lag

---

#### TC-EDITOR-003: Sublime Text

**Objective:** Verify Sublime Text support

**Steps:**
1. Open Sublime Text
2. New file
3. Type Vietnamese text

**Expected Result:**
- ✅ Instant method used
- ✅ Bundle: `com.sublimetext.4` or `com.sublimetext.3`
- ✅ Performance matches VSCode

---

### Terminal Tests

#### TC-TERM-001: iTerm2 Slow Method

**Objective:** Verify terminals use slow method with delays

**Steps:**
1. Open iTerm2
2. Type: `hoa`
3. Observe rendering

**Expected Result:**
- ✅ iTerm2 detected: `com.googlecode.iterm2`
- ✅ Slow method applied
- ✅ Delays: (3000, 8000, 3000) microseconds
- ✅ No character loss

**Log Validation:**
```
INFO: detect: com.googlecode.iterm2 role=nil
METHOD: slow:term
SEND[slow] bs=3 chars=hoà
```

---

#### TC-TERM-002: Terminal.app

**Objective:** Verify macOS Terminal support

**Steps:**
1. Open Terminal.app
2. Type: `thuy`
3. Verify transformation

**Expected Result:**
- ✅ Terminal detected: `com.apple.Terminal`
- ✅ Slow method used
- ✅ Stable rendering

---

### Detection Tests

#### TC-DETECT-001: App Switching

**Objective:** Verify detection changes when switching apps

**Steps:**
1. Open VSCode (should use instant)
2. Type: `hoa` → verify instant method
3. Switch to iTerm2 (Cmd+Tab)
4. Type: `hoa` → verify slow method
5. Switch back to VSCode
6. Type: `hoa` → verify instant method again

**Expected Result:**
- ✅ Method changes correctly per app
- ✅ No cached detection errors
- ✅ Each app gets optimal method

---

#### TC-DETECT-002: Role Detection

**Objective:** Verify UI role detection works

**Steps:**
1. Open browser
2. Focus address bar → should detect `AXTextField` + selection
3. Focus search box in page → should detect different method
4. Check logs for role information

**Expected Result:**
- ✅ Address bar: `role=AXTextField` + selection method
- ✅ Content: different role or nil + fast/default method

---

#### TC-DETECT-003: Overlay Detection

**Objective:** Verify overlays like Spotlight are detected

**Steps:**
1. Open Spotlight (overlay, not frontmost app)
2. Type Vietnamese
3. Verify detection works even though Spotlight isn't frontmost

**Expected Result:**
- ✅ Spotlight detected via focused element
- ✅ Bundle ID: `com.apple.Spotlight`
- ✅ Works correctly despite being overlay

---

## Performance Validation

### Latency Measurement

Use logs to measure latency:

```bash
# Filter for send operations
grep "SEND\[" ~/Library/Logs/VietnameseIME/keyboard.log

# Expected patterns:
# Instant: < 10ms total
# Selection: < 8ms total  
# Fast: < 15ms total
# Slow: < 30ms total
# Autocomplete: < 20ms total
```

### Performance Test Matrix

| App Type | Method | Expected Latency | Test Input | Pass/Fail |
|----------|--------|------------------|------------|-----------|
| VSCode | instant | < 10ms | `truong` | ☐ |
| Chrome (address) | selection | < 8ms | `hoa` | ☐ |
| Spotlight | autocomplete | < 20ms | `thuy` | ☐ |
| iTerm2 | slow | < 30ms | `vietnam` | ☐ |
| Safari | selection | < 8ms | `khong` | ☐ |
| Arc | selection | < 8ms | `tiec` | ☐ |

---

## Troubleshooting

### Issue: Detection Not Working

**Symptoms:** Wrong method applied, or default method used everywhere

**Debug Steps:**
```bash
# Check logs
tail -f ~/Library/Logs/VietnameseIME/keyboard.log | grep "detect:"

# Should show bundle ID and role
# Example: "detect: com.google.Chrome role=AXTextField"
```

**Solutions:**
1. Verify Accessibility permission granted
2. Restart Vietnamese IME
3. Check bundle ID matches in code

---

### Issue: Spotlight Not Transforming

**Symptoms:** Spotlight shows raw input, no Vietnamese

**Debug Steps:**
1. Check bundle ID detection
2. Verify autocomplete method is selected
3. Check Forward Delete is sent first

**Solutions:**
```bash
# Check logs for:
METHOD: auto:spotlight

# If shows different method, detection failed
```

---

### Issue: Browser Address Bar Issues

**Symptoms:** Text doesn't replace, autocomplete interferes

**Debug Steps:**
1. Check role detection: should be `AXTextField`
2. Verify selection method is used
3. Check browser is in supported list

**Solutions:**
- Add browser bundle ID if missing
- Verify `AXTextField` role detection works
- Check Shift+Left events are sent

---

## Test Results Template

### Test Session Information

```
Date: _______________
Tester: _______________
Vietnamese IME Version: _______________
macOS Version: _______________
```

### Test Results

| Test ID | Test Name | Status | Notes |
|---------|-----------|--------|-------|
| TC-SPOT-001 | Spotlight Basic Input | ☐ Pass ☐ Fail | |
| TC-SPOT-002 | Forward Delete | ☐ Pass ☐ Fail | |
| TC-SPOT-003 | Backspace | ☐ Pass ☐ Fail | |
| TC-BROWSER-001 | Chrome Address Bar | ☐ Pass ☐ Fail | |
| TC-BROWSER-002 | Arc Browser | ☐ Pass ☐ Fail | |
| TC-BROWSER-003 | Firefox | ☐ Pass ☐ Fail | |
| TC-BROWSER-004 | Safari | ☐ Pass ☐ Fail | |
| TC-BROWSER-005 | Content Area | ☐ Pass ☐ Fail | |
| TC-EDITOR-001 | VSCode Instant | ☐ Pass ☐ Fail | |
| TC-EDITOR-002 | Zed Editor | ☐ Pass ☐ Fail | |
| TC-EDITOR-003 | Sublime Text | ☐ Pass ☐ Fail | |
| TC-TERM-001 | iTerm2 Slow | ☐ Pass ☐ Fail | |
| TC-TERM-002 | Terminal.app | ☐ Pass ☐ Fail | |
| TC-DETECT-001 | App Switching | ☐ Pass ☐ Fail | |
| TC-DETECT-002 | Role Detection | ☐ Pass ☐ Fail | |
| TC-DETECT-003 | Overlay Detection | ☐ Pass ☐ Fail | |

### Performance Results

| Metric | Target | Actual | Pass/Fail |
|--------|--------|--------|-----------|
| Spotlight latency | < 20ms | _____ ms | ☐ |
| Chrome address bar | < 8ms | _____ ms | ☐ |
| Arc address bar | < 8ms | _____ ms | ☐ |
| VSCode latency | < 10ms | _____ ms | ☐ |
| iTerm2 latency | < 30ms | _____ ms | ☐ |

### Overall Assessment

- **Total Tests:** 16
- **Passed:** _____
- **Failed:** _____
- **Pass Rate:** _____%

**Critical Issues Found:**
1. _______________________________
2. _______________________________

**Recommendations:**
1. _______________________________
2. _______________________________

---

## Appendix: Quick Test Script

```bash
#!/bin/bash
# Quick validation script

echo "=== Vietnamese IME Accessibility API Test ==="
echo ""

# Check Accessibility permission
echo "1. Checking Accessibility permission..."
if [[ $(tccutil list Accessibility | grep -c "vietnamese") -gt 0 ]]; then
    echo "   ✅ Permission granted"
else
    echo "   ❌ Permission NOT granted"
fi

# Check if app is running
echo "2. Checking if Vietnamese IME is running..."
if pgrep -f "VietnameseIMEFast" > /dev/null; then
    echo "   ✅ App is running"
else
    echo "   ❌ App is NOT running"
fi

# Check log file exists
echo "3. Checking log file..."
if [[ -f ~/Library/Logs/VietnameseIME/keyboard.log ]]; then
    echo "   ✅ Log file exists"
    echo "   Recent entries:"
    tail -5 ~/Library/Logs/VietnameseIME/keyboard.log | sed 's/^/      /'
else
    echo "   ❌ Log file NOT found"
fi

echo ""
echo "=== Ready to test! ==="
echo "Open Spotlight (Cmd+Space) and type: hoa"
echo "Then check logs: tail -f ~/Library/Logs/VietnameseIME/keyboard.log"
```

---

## Related Documentation

- **[ACCESSIBILITY_API_SUPPORT.md](./ACCESSIBILITY_API_SUPPORT.md)** - Technical implementation details
- **[BROWSER_SUPPORT.md](./BROWSER_SUPPORT.md)** - Complete browser support list
- **[getting-started/TESTING_GUIDE.md](./getting-started/TESTING_GUIDE.md)** - General testing guide

---

## License

This testing documentation is part of Vietnamese IME project.  
Copyright © 2025 Vietnamese IME Contributors.  
Licensed under MIT License.