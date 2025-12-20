# Adding RustBridge.swift to Xcode Project

## Problem
The new `RustBridge.swift` file has been created but is not part of the Xcode project, causing compilation errors.

## Solution

### Method 1: Using Xcode GUI (Recommended)

1. **Open Xcode Project**
   ```
   cd vietnamese-ime/platforms/macos/VietnameseIMEFast
   open VietnameseIMEFast.xcodeproj
   ```

2. **Add RustBridge.swift to Project**
   - In Xcode, right-click on the `VietnameseIMEFast` group (the folder icon)
   - Select "Add Files to VietnameseIMEFast..."
   - Navigate to: `VietnameseIMEFast/RustBridge.swift`
   - Make sure these options are checked:
     - ✅ "Copy items if needed" (leave unchecked since file is already in place)
     - ✅ "Create groups"
     - ✅ Target: "VietnameseIMEFast"
   - Click "Add"

3. **Verify File is Added**
   - You should see `RustBridge.swift` in the Project Navigator
   - It should have a checkmark next to "VietnameseIMEFast" target in File Inspector (right panel)

4. **Rebuild Project**
   - Press Cmd+B to build
   - All errors should be resolved

### Method 2: Using Command Line

If you prefer command line, you can use `xcodeproj` tool:

```bash
# Install xcodeproj if needed
gem install xcodeproj

# Add file to project
cd vietnamese-ime/platforms/macos/VietnameseIMEFast

# Run Ruby script to add file
ruby << 'EOF'
require 'xcodeproj'

project_path = 'VietnameseIMEFast.xcodeproj'
project = Xcodeproj::Project.open(project_path)

# Find the main group
main_group = project.main_group['VietnameseIMEFast']

# Add file
file_ref = main_group.new_reference('VietnameseIMEFast/RustBridge.swift')

# Add to target
target = project.targets.first
target.add_file_references([file_ref])

# Save
project.save
EOF
```

### Method 3: Manual Project.pbxproj Edit (Advanced)

If both methods above fail, you can manually edit the project file, but this is **not recommended** unless you're familiar with Xcode project structure.

## Verification Checklist

After adding the file, verify:

- [ ] `RustBridge.swift` appears in Project Navigator
- [ ] File is checked for target membership (File Inspector → Target Membership)
- [ ] Project builds without "Cannot find type" errors
- [ ] All 728 lines of RustBridge.swift are recognized
- [ ] Log, TextInjector, KeyboardHookManager symbols are found

## Troubleshooting

### Error: "Cannot find type 'EnginePtr'"
**Cause**: Bridging header not properly configured or RustBridge not in project

**Fix**:
1. Verify bridging header path in Build Settings
2. Should be: `$(SRCROOT)/VietnameseIMEFast/VietnameseIMEFast-Bridging-Header.h`
3. Clean build folder (Cmd+Shift+K)
4. Rebuild (Cmd+B)

### Error: "Cannot find 'Log', 'TextInjector', etc."
**Cause**: RustBridge.swift not added to project

**Fix**:
1. Follow Method 1 above to add file
2. Make sure file is checked for target

### Error: "Duplicate symbol '_ime_create'"
**Cause**: Multiple bridging headers or duplicate implementations

**Fix**:
1. Verify only ONE bridging header is configured
2. Check that Rust library is only linked once
3. Clean derived data: `rm -rf ~/Library/Developer/Xcode/DerivedData`

### Build Still Fails After Adding File
**Fix**:
1. Clean Build Folder: Product → Clean Build Folder (Cmd+Shift+K)
2. Close Xcode completely
3. Delete DerivedData:
   ```bash
   rm -rf ~/Library/Developer/Xcode/DerivedData
   ```
4. Reopen Xcode
5. Build (Cmd+B)

## Expected Build Output

After successfully adding the file and rebuilding:

```
Build target VietnameseIMEFast
  CompileSwift RustBridge.swift
  CompileSwift InputManager.swift
  CompileSwift AppDelegate.swift
  CompileSwift main.swift
  Link VietnameseIMEFast
  Generate VietnameseIMEFast.app
Build succeeded
```

## File Location Verification

Ensure the file is in the correct location:

```bash
cd vietnamese-ime/platforms/macos/VietnameseIMEFast
ls -la VietnameseIMEFast/RustBridge.swift
# Should show: -rw-r--r-- ... RustBridge.swift (728 lines)
```

## Next Steps

Once RustBridge.swift is added and project builds:

1. [ ] Rebuild Rust library: `cd core && cargo build --release`
2. [ ] Rebuild Xcode project: Cmd+B
3. [ ] Run app: Cmd+R
4. [ ] Grant Accessibility permission
5. [ ] Test basic typing: "a" "a" → "â"
6. [ ] Enable logging in DEBUG mode
7. [ ] Check log file: `cat /tmp/vietnameseime.log`

## Success Indicator

You'll know everything is working when:
- ✅ Project builds with 0 errors
- ✅ App launches and shows 🇻🇳 in menu bar
- ✅ Accessibility permission prompt appears
- ✅ After granting permission, typing "aa" produces "â"
- ✅ Log file is created (if logging enabled)

## Contact

If issues persist, check:
- INTEGRATION_NOTES.md for architecture details
- VERIFICATION_CHECKLIST.md for testing steps
- GONHANH_INTEGRATION_SUMMARY.md for overview