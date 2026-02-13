# Debugging Runtime Issues - GoxViet

**Date:** 2026-02-12  
**Issues:** Menu bar not clickable, keyboard shortcut not working, Vietnamese input not working

## Current Situation

### What We Know

1. ✅ **Build succeeds** - No compilation errors
2. ✅ **App launches** - Process running
3. ✅ **Menu bar created** - Logs show `[GoxViet] setupMenu() called`
4. ❌ **Menu bar not clickable** - User reports cannot click
5. ❌ **Keyboard shortcut not working** - Ctrl+Space not toggling
6. ❌ **Vietnamese input not working** - Keys not being processed

### Root Cause Analysis

**The problem:** `InputManager.start()` is likely not being called because Accessibility permission check is failing.

**Evidence:**
```swift
// AppDelegate.swift (Line 95-112)
func checkAccessibilityPermission() {
    let accessEnabled = AXIsProcessTrusted()
    
    if !accessEnabled {
        Log.warning("Accessibility permission not granted")
        showAccessibilityAlert()  // ← User sees alert
    } else {
        Log.info("Accessibility permission granted")
        InputManager.shared.start()  // ← This needs to run!
    }
}
```

**Without InputManager running:**
- No keyboard event tap
- No shortcut monitoring
- No Vietnamese processing
- Menu bar exists but might not respond properly

## Immediate Fix Steps

### Step 1: Grant Accessibility Permission

I've already opened System Settings → Privacy & Security → Accessibility for you.

**What you need to do:**

1. In System Settings → Accessibility panel:
   - Look for **"goxviet"** in the list
   - If it exists: **Toggle it ON** (enable the checkbox)
   - If it doesn't exist: **Click the '+' button** and add:
     ```
     /Users/nihmtaho/Library/Developer/Xcode/DerivedData/goxviet-*/Build/Products/Debug/goxviet.app
     ```

2. **Quit and relaunch** the app after granting permission

### Step 2: Verify Permission is Working

After relaunching, check system logs:

```bash
log stream --predicate 'process == "goxviet"' --level info | grep -E "(InputManager|Toggle|Permission)"
```

**Expected output:**
```
[INFO] Accessibility permission granted
[INFO] InputManager started
[INFO] PerAppModeManagerEnhanced started
```

**If you see:**
```
[WARNING] Accessibility permission not granted
```
→ Permission was not granted correctly, try again.

### Step 3: Test Functionality

Once InputManager starts:

1. **Test keyboard shortcut:**
   ```
   Press: Ctrl + Space
   Expected: Status icon toggles between 🇻🇳 and ✏️
   ```

2. **Test Vietnamese typing:**
   ```
   Type: v i e t
   Expected: việt
   ```

3. **Test menu bar:**
   ```
   Click: Menu bar icon (🇻🇳 or ✏️)
   Expected: Menu appears with options
   ```

## Common Issues & Solutions

### Issue 1: "App not in Accessibility list"

**Symptom:** Cannot find "goxviet" in System Settings

**Cause:** Debug build path changes with each Xcode version

**Solutions:**

A. **Use the alert** - When you launch the app, it should show an alert:
   ```
   🔐 Accessibility Permission Required
   
   Click "Open System Settings" → Find "GoxViet" → Toggle ON
   ```

B. **Add manually:**
   1. Click '+' button in Accessibility panel
   2. Press Cmd+Shift+G to open "Go to Folder"
   3. Paste this path:
      ```
      /Users/nihmtaho/Library/Developer/Xcode/DerivedData
      ```
   4. Find `goxviet-*` folder → `Build/Products/Debug/goxviet.app`
   5. Select it and click "Open"

### Issue 2: "Permission granted but still not working"

**Symptom:** Checkbox is ON but app still not working

**Causes & Fixes:**

A. **Stale permission cache:**
   ```bash
   tccutil reset Accessibility com.goxviet.ime
   killall goxviet
   # Relaunch app and grant permission again
   ```

B. **Event tap disabled by system:**
   ```bash
   # Check if event tap creation succeeds
   log stream --predicate 'process == "goxviet"' | grep "event tap"
   ```
   
   If you see `"Failed to create event tap"` → macOS blocked it
   
   **Fix:** Reboot macOS (sometimes required after permission changes)

C. **Wrong bundle identifier:**
   ```bash
   # Check what identifier the app is using
   defaults read ~/Library/Developer/Xcode/DerivedData/goxviet-*/Build/Products/Debug/goxviet.app/Contents/Info.plist CFBundleIdentifier
   ```
   
   Should be: `com.goxviet.ime`

### Issue 3: "Menu bar appears but not clickable"

**Symptom:** Icon shows up but click does nothing

**Causes & Fixes:**

A. **App not active:**
   ```swift
   // Try activating the app
   NSApp.activate(ignoringOtherApps: true)
   ```

B. **Status item not properly configured:**
   ```bash
   # Check if menu was attached
   log show --predicate 'process == "goxviet"' --last 5m | grep setupMenu
   ```
   
   Should see: `[GoxViet] setupMenu() called`

C. **Dock hidden + App not activating:**
   - Go to Settings → Advanced → **Disable** "Hide from Dock"
   - Relaunch app
   - Try clicking menu bar again

### Issue 4: "First launch shows alert, but subsequent launches don't work"

**Symptom:** Alert appears, user grants permission, but app still broken after relaunch

**Cause:** Permission granted while app was running, but app didn't detect it

**Fix:**
1. Quit app completely (Cmd+Q)
2. Verify permission is ON in System Settings
3. Launch app again from Xcode
4. Should work now without alert

## Debugging Tools

### Enable Logging

The app has logging disabled by default. To enable:

```bash
defaults write com.goxviet.ime loggingEnabled -bool true
killall goxviet
# Relaunch app
```

Then check logs:
```bash
tail -f ~/Library/Logs/GoxViet/keyboard.log
```

### Check InputManager Status

Run this while app is running:
```bash
log stream --predicate 'process == "goxviet"' --level info | grep -E "(InputManager|start|stop|Toggle)"
```

**Healthy output:**
```
[INFO] InputManager started
[INFO] Toggle shortcut loaded: ⌃Space
[INFO] PerAppModeManagerEnhanced started
```

**Problem output:**
```
(No InputManager logs) ← Manager never started!
```

### Check CGEventTap

```bash
log stream --predicate 'process == "goxviet"' | grep -i "event tap"
```

**Success:**
```
(No "Failed" messages)
```

**Failure:**
```
[INFO] Failed to create event tap ← PROBLEM!
```

If event tap fails → Accessibility permission issue or macOS blocking it

### System Logs

Full debug output:
```bash
log show --predicate 'process == "goxviet"' --last 5m --info --debug | less
```

Look for:
- `Accessibility`
- `InputManager`
- `event tap`
- `Toggle`
- `Failed`
- `Error`

## Expected Flow (Correct Behavior)

### App Launch Sequence

1. **AppDelegate.applicationDidFinishLaunching:**
   ```
   → setupMenu()
   → checkAccessibilityPermission()
   ```

2. **If permission NOT granted:**
   ```
   → showAccessibilityAlert()
   → startAccessibilityPollTimer() (checks every 1s)
   → (User grants permission in System Settings)
   → onAccessibilityGranted()
   → InputManager.shared.start() ← KEY MOMENT
   ```

3. **If permission granted:**
   ```
   → InputManager.shared.start() ← KEY MOMENT
   → Start event tap
   → Start shortcut monitoring
   → Start per-app mode
   ```

### InputManager.start() Checklist

When this function runs successfully:

```swift
✅ Create CGEventTap for keyboard events
✅ Add to run loop
✅ Enable tap
✅ Start mouse monitor
✅ Start PerAppModeManagerEnhanced
✅ Start InputSourceMonitor
✅ Set isRunning = true
✅ Log "InputManager started"
```

If ANY of these fail → App won't work

## Quick Diagnostic Script

Run this to check everything:

```bash
#!/bin/bash
echo "=== GoxViet Diagnostic ==="
echo ""

echo "1. Process running?"
ps aux | grep -i goxviet | grep -v grep || echo "❌ NOT RUNNING"
echo ""

echo "2. Bundle ID?"
defaults read ~/Library/Developer/Xcode/DerivedData/goxviet-*/Build/Products/Debug/goxviet.app/Contents/Info.plist CFBundleIdentifier 2>/dev/null || echo "❌ Cannot find app"
echo ""

echo "3. Settings?"
defaults read com.goxviet.ime isEnabled 2>/dev/null && echo "✅ isEnabled = true" || echo "⚠️  isEnabled not set"
echo ""

echo "4. Logging enabled?"
defaults read com.goxviet.ime loggingEnabled 2>/dev/null && echo "✅ Logging ON" || echo "⚠️  Logging OFF (enable with: defaults write com.goxviet.ime loggingEnabled -bool true)"
echo ""

echo "5. Recent logs (last 10 lines):"
tail -10 ~/Library/Logs/GoxViet/keyboard.log 2>/dev/null || echo "⚠️  No log file (enable logging)"
echo ""

echo "6. System logs (last 2 min):"
log show --predicate 'process == "goxviet"' --last 2m --style compact 2>/dev/null | grep -E "(InputManager|Permission|Toggle|Error)" | tail -5 || echo "⚠️  No system logs"
echo ""

echo "=== End Diagnostic ==="
```

Save as `diagnostic.sh`, run with: `bash diagnostic.sh`

## Next Steps After Permission Grant

Once Accessibility permission is working:

1. **Verify all features:**
   - [ ] Keyboard shortcut (Ctrl+Space)
   - [ ] Vietnamese input (viet → việt)
   - [ ] Menu bar clickable
   - [ ] Settings window opens
   - [ ] Toggle from menu works
   - [ ] Per-app mode works

2. **Test edge cases:**
   - [ ] Restart app → state persists
   - [ ] Switch apps → per-app mode works
   - [ ] Type in different apps
   - [ ] Change input method (Telex ↔ VNI)

3. **Performance check:**
   - [ ] No lag when typing
   - [ ] No memory leaks (use Activity Monitor)
   - [ ] CPU usage reasonable (<5% when idle)

## Still Not Working?

If after granting permission it still doesn't work:

1. **Reboot macOS** - Sometimes required after permission changes
2. **Clean build:**
   ```bash
   cd platforms/macos
   xcodebuild clean
   rm -rf ~/Library/Developer/Xcode/DerivedData/goxviet-*
   xcodebuild build
   ```

3. **Check for crashes:**
   ```bash
   log show --predicate 'eventMessage CONTAINS "goxviet" AND messageType == fault' --last 5m
   ```

4. **Contact for help** - Share:
   - System logs (last 5 minutes)
   - `diagnostic.sh` output
   - Accessibility settings screenshot

## Files to Check

If you need to debug the code:

1. **`AppDelegate.swift` (Line 95-112):** checkAccessibilityPermission()
2. **`AppDelegate.swift` (Line 165-178):** onAccessibilityGranted()
3. **`InputManager.swift` (Line 121-168):** start()
4. **`InputManager.swift` (Line 319-344):** handleEvent() - Toggle shortcut
5. **`SettingsManager.swift` (Line 123):** isEnabled default value

## Summary

**The core issue:** InputManager not starting because Accessibility permission not properly granted.

**The fix:** Grant permission in System Settings, quit app, relaunch.

**After fix:** Everything should work - shortcut, Vietnamese typing, menu bar.

**If still broken:** Run diagnostic script and check system logs.
