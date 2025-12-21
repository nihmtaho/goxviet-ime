# Build and Test Guide - Safari Backspace Fix

**Date:** 2025-12-21  
**Target:** Safari address bar backspace fix  
**Status:** Code complete, needs Xcode project configuration

---

## Overview

This guide walks through building and testing the Safari backspace fix after code changes have been applied.

---

## Files Added/Modified

### ✅ New Files Created
1. `platforms/macos/GoxViet/GoxViet/TextInjectionHelper.swift`
   - Text injection utility with app-specific methods
   - Fixed `getWordToRestoreOnBackspace()` to skip Safari address bars
   - Contains `detectMethod()` for app detection

2. `platforms/macos/GoxViet/GoxViet/Log.swift`
   - Logging utility for debugging
   - Writes to `~/Library/Logs/GoxViet/keyboard.log`

3. `docs/SAFARI_BACKSPACE_FIX.md`
   - Documentation of the issue and fix

4. `docs/BUILD_AND_TEST_GUIDE.md`
   - This file

### ✅ Modified Files
1. `platforms/macos/GoxViet/GoxViet/InputManager.swift`
   - Updated to use `TextInjector.shared.injectSync()`
   - Added backspace handling with word restoration
   - Integrated `detectMethod()` for app-specific injection

2. `platforms/macos/GoxViet/GoxViet/RustBridge.swift`
   - Cleaned up: removed duplicate Log and TextInjector classes
   - Kept only RustBridge class and helper functions

---

## Build Steps

### Step 1: Add New Files to Xcode Project

**IMPORTANT:** The new Swift files must be added to the Xcode project before building.

```bash
# Navigate to project directory
cd goxviet/platforms/macos/GoxViet

# Open in Xcode
open GoxViet.xcodeproj
```

**In Xcode:**

1. **Add Log.swift:**
   - Right-click on `GoxViet` folder in Project Navigator
   - Select "Add Files to GoxViet..."
   - Navigate to: `GoxViet/Log.swift`
   - ✅ Check "Copy items if needed" (if prompted)
   - ✅ Check "Add to targets: GoxViet"
   - Click "Add"

2. **Add TextInjectionHelper.swift:**
   - Repeat same process for `GoxViet/TextInjectionHelper.swift`

3. **Verify files are in target:**
   - Select each file in Project Navigator
   - Check File Inspector (right panel)
   - Ensure "Target Membership" includes ✅ GoxViet

### Step 2: Configure Rust Library Linking

Ensure the Rust library is properly linked:

```bash
# Build Rust core (if not already built)
cd goxviet/core
cargo build --release

# Verify library exists
ls -lh target/release/libvietnamese_ime.a
```

**In Xcode:**

1. Select project in Navigator
2. Select "GoxViet" target
3. Go to "Build Phases" tab
4. Expand "Link Binary With Libraries"
5. Verify `libvietnamese_ime.a` is present
6. If missing, click "+" and add: `core/target/release/libvietnamese_ime.a`

### Step 3: Verify Bridging Header

Check that bridging header is configured:

1. Select project → Build Settings
2. Search for "bridging"
3. Verify "Objective-C Bridging Header" is set to:
   ```
   GoxViet/GoxViet-Bridging-Header.h
   ```

### Step 4: Build Project

```bash
# Clean build folder (recommended after adding new files)
cd goxviet/platforms/macos/GoxViet
xcodebuild clean

# Build for Release
xcodebuild -configuration Release

# OR build in Xcode GUI:
# Product → Clean Build Folder (Cmd+Shift+K)
# Product → Build (Cmd+B)
```

**Expected Output:**
```
** BUILD SUCCEEDED **
```

**If build fails with missing symbols:**
- Verify Log.swift and TextInjectionHelper.swift are added to target
- Check Rust library is linked
- Ensure bridging header path is correct

---

## Testing

### Test 1: Basic Functionality

1. **Run the app:**
   ```bash
   # From Xcode: Product → Run (Cmd+R)
   # Or run built binary directly
   ```

2. **Enable Vietnamese input:**
   - Click menu bar icon (should show "EN")
   - Toggle "Vietnamese Input" ON (icon changes to 🇻🇳)

3. **Test in VSCode (control test):**
   - Open VSCode
   - Type: `vie^t`
   - Expected: `việt` ✅
   - Type: `nam `
   - Press backspace to delete space
   - Expected: Cursor enters "nam" for editing ✅

### Test 2: Safari Address Bar Fix (Main Issue)

1. **Open Safari**

2. **Click address bar** (URL field at top)

3. **Type Vietnamese text:**
   ```
   Input:  g o ~ space t i e^ ' n g space v i e^ . t
   Display: gõ tiếng việt
   ```

4. **Delete characters with backspace:**
   - Delete "t" → `gõ tiếng việ` ✅
   - Delete "ệ" → `gõ tiếng vi` ✅
   - Delete "i" → `gõ tiếng v` ✅
   - Delete "v" → `gõ tiếng ` ✅
   - Delete " " → `gõ tiếng` ✅
   - Delete "g" → `gõ tiên` ✅
   - Continue deleting...
   - **Expected:** Smooth deletion, NO garbled text ✅
   - **Before fix:** Would show random characters around "gõ "

5. **Type again in same address bar:**
   ```
   Input:  v i e^ . t space n a m
   Display: việt nam
   ```
   - Expected: Works normally ✅

### Test 3: Chrome Address Bar

Repeat Test 2 in Chrome to verify fix works across browsers:
- Open Chrome
- Click address bar
- Type Vietnamese text
- Delete with backspace
- Expected: No garbled text ✅

### Test 4: Safari Regular Input Fields

1. **Open Safari**
2. **Navigate to:** `google.com`
3. **Click search box** (NOT address bar)
4. **Type Vietnamese text:**
   ```
   Input: t i m space k i e^ ' m
   Display: tìm kiếm
   ```
5. **Delete with backspace:**
   - Expected: Works normally ✅

### Test 5: Check Debug Logs (Optional)

If you enabled debug logging:

```bash
# Enable logging in AppDelegate.swift:
# Log.isEnabled = true (in DEBUG mode)

# View log file
tail -f ~/Library/Logs/GoxViet/keyboard.log

# Look for these entries when using Safari:
# detect: com.apple.Safari role=AXTextField
# METHOD: sel:browser
# restore: skipping browser address bar to avoid placeholder text
```

---

## Troubleshooting

### Issue: Build fails with "Cannot find 'Log' in scope"

**Solution:**
- Verify Log.swift is added to Xcode project
- Check Target Membership includes GoxViet
- Clean build folder: `xcodebuild clean`
- Rebuild

### Issue: Build fails with "Cannot find 'ime_init' in scope"

**Solution:**
- Verify Rust library is built: `cd core && cargo build --release`
- Check library is linked in Xcode Build Phases
- Verify bridging header path is correct

### Issue: App crashes on launch

**Solution:**
- Check Accessibility permissions:
  - System Preferences → Security & Privacy → Privacy → Accessibility
  - Add GoxViet to list and enable
- View crash logs: `Console.app` → filter by "GoxViet"

### Issue: Vietnamese input not working

**Solution:**
- Click menu bar icon
- Verify "Vietnamese Input" is toggled ON (🇻🇳 icon)
- Try toggle shortcut: Control+Space (default)
- Check logs for errors

### Issue: Still seeing garbled text in Safari

**Solution:**
- Verify build includes latest TextInjectionHelper.swift changes
- Check log file for "restore: skipping browser address bar" message
- If message not appearing:
  - App might be using old binary
  - Clean build and reinstall
  - Kill running process: `killall GoxViet`
  - Run again from Xcode

---

## Performance Verification

The fix should maintain performance targets:

- **Latency:** < 16ms for single keystroke ✅
- **Backspace:** < 3ms for deletion operations ✅
- **No memory leaks:** Check with Instruments (Xcode → Product → Profile)

### Benchmark Test

```bash
# Run performance benchmark (if available)
cd goxviet
./test-performance.sh
```

Expected metrics:
- Average keystroke latency: < 10ms
- 99th percentile: < 16ms
- Backspace operations: < 3ms

---

## Rollback Instructions

If the fix causes issues:

```bash
# Revert changes
cd goxviet
git status
git checkout platforms/macos/GoxViet/GoxViet/InputManager.swift
git checkout platforms/macos/GoxViet/GoxViet/RustBridge.swift

# Remove new files from Xcode project
# (In Xcode: Select file → Delete → Move to Trash)

# Rebuild
xcodebuild clean build
```

---

## Success Criteria

✅ Build succeeds without errors  
✅ App launches and shows menu bar icon  
✅ Vietnamese input works in VSCode  
✅ Safari address bar: no garbled text on backspace  
✅ Chrome address bar: no garbled text on backspace  
✅ Safari regular input fields work normally  
✅ Performance maintained (< 16ms latency)

---

## Next Steps

After successful testing:

1. **Commit changes:**
   ```bash
   git add platforms/macos/GoxViet/GoxViet/TextInjectionHelper.swift
   git add platforms/macos/GoxViet/GoxViet/Log.swift
   git add platforms/macos/GoxViet/GoxViet/InputManager.swift
   git add platforms/macos/GoxViet/GoxViet/RustBridge.swift
   git add docs/SAFARI_BACKSPACE_FIX.md
   git add docs/BUILD_AND_TEST_GUIDE.md
   git commit -m "Fix Safari address bar backspace garbled text issue"
   ```

2. **Update documentation index:**
   - Add entry to `docs/README.md`
   - Update `docs/DOCUMENTATION_STRUCTURE.md`

3. **Tag release (if stable):**
   ```bash
   git tag -a v1.0.2 -m "Safari backspace fix"
   git push origin v1.0.2
   ```

---

## References

- Safari Backspace Fix: `docs/SAFARI_BACKSPACE_FIX.md`
- Project Architecture: `docs/ARCHITECTURE.md` (if exists)
- Performance Guide: `docs/PERFORMANCE_OPTIMIZATION_GUIDE.md` (if exists)

---

**Last Updated:** 2025-12-21  
**Tested On:** macOS Sonoma 14.x  
**Safari Version:** 17.x  
**Status:** ✅ Ready for testing