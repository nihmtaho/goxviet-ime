# XCODE SETUP CHECKLIST - Settings UI

**Status:** ⏳ Pending Manual Steps  
**Commit:** 75ecad9

---

## Quick Steps

### 1. Open Xcode Project
```bash
cd platforms/macos/goxviet
open goxviet.xcodeproj
```

### 2. Add New Files (2 files)

#### Add SettingsView.swift
- Right-click `goxviet` folder in Project Navigator
- Select **"Add Files to 'goxviet'..."**
- Navigate to `goxviet/SettingsView.swift`
- ✅ Check **"Add to targets: goxviet"**
- ❌ Uncheck "Copy items if needed" (already in correct location)
- Click **"Add"**

#### Add SettingsWindowController.swift
- Repeat above steps for `goxviet/SettingsWindowController.swift`

### 3. Verify Files Added
- Select `goxviet` target
- Go to **Build Phases** → **Compile Sources**
- Confirm both files are listed:
  - ✅ SettingsView.swift
  - ✅ SettingsWindowController.swift

### 4. Clean Build
```bash
xcodebuild clean
xcodebuild -configuration Debug
```

### 5. Run & Test
- Build and run (⌘R)
- Click menu bar icon → **"Settings..."**
- Verify:
  - ✅ Window opens
  - ✅ All 4 tabs visible (General, Per-App, Advanced, About)
  - ✅ Controls are responsive
  - ✅ Settings persist after app restart

### 6. Commit Project File
```bash
git status  # Should show goxviet.xcodeproj/project.pbxproj modified
git add goxviet.xcodeproj/project.pbxproj
git commit -m "build(macos): add SettingsView files to Xcode project"
```

---

## Troubleshooting

### Build Error: "No such module 'SwiftUI'"
- Ensure deployment target is macOS 11.0+ (in project settings)

### Files not appearing in Navigator
- Check that files are physically in `platforms/macos/goxviet/goxviet/` directory
- Use Finder to verify file location

### Window doesn't open
- Check Console.app for errors
- Look for log: "Settings window opened"
- Verify `SettingsWindowController.shared.show()` is called

---

## Success Indicators

✅ Clean build succeeds  
✅ Settings window opens on menu click  
✅ All tabs are accessible  
✅ No crashes or errors in Console  
✅ Settings persist after relaunch  

---

**Next:** See `docs/SETTINGS_UI_IMPLEMENTATION.md` for full testing checklist
