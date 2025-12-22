# REBRANDING TO GOXVIET - COMPLETE GUIDE

**Date:** 2025-12-21  
**Status:** ✅ Partially Complete - Xcode Project Update Required  
**New Brand:** Gõ Việt (GoxViet)

---

## 📋 Summary of Changes

### New Branding
- **Brand Name:** Gõ Việt
- **Display Name:** GoxViet
- **Repository:** goxviet
- **Xcode Project:** goxviet
- **Bundle ID:** com.goxviet.ime
- **Log Path:** ~/Library/Logs/GoxViet/

---

## ✅ Completed Changes

### 1. File System & Directories
- [x] Renamed root directory: `vietnamese-ime` → `goxviet`
- [x] Renamed Xcode project directory: `platforms/macos/VietnameseIMEFast` → `platforms/macos/goxviet`
- [x] Renamed Xcode project file: `VietnameseIMEFast.xcodeproj` → `goxviet.xcodeproj`
- [x] Renamed entitlements: `VietnameseIMEFast.entitlements` → `goxviet.entitlements`

### 2. Swift Source Code
- [x] **AppDelegate.swift**
  - Updated header comment to GoxViet
  - Changed log messages: "VietnameseIMEFast" → "GoxViet"
  - Updated menu item: "About VietnameseIMEFast" → "About GoxViet"
  - Changed tooltip: "Vietnamese Input" → "Gõ Việt"
  - Updated alert message text to "GoxViet - Gõ Việt"
  - Version updated to 1.0.2

- [x] **AppState.swift**
  - Updated header comment to GoxViet
  - Changed all UserDefaults keys: `com.vietnamese.ime.*` → `com.goxviet.ime.*`
  - Updated log messages: "Vietnamese input" → "Gõ Việt input"

- [x] **InputManager.swift**
  - Updated header comment to GoxViet
  - Changed accessibility alert: "VietnameseIMEFast" → "GoxViet"
  - Updated log messages to "Gõ Việt"

- [x] **RustBridge.swift**
  - Updated header comment to GoxViet

- [x] **KeyboardShortcut.swift**
  - Updated header comment to GoxViet
  - Changed description: "toggling Vietnamese input" → "toggling Gõ Việt input"

- [x] **Log.swift**
  - Updated header comment to GoxViet
  - Changed log path: `~/Library/Logs/VietnameseIME/` → `~/Library/Logs/GoxViet/`

- [x] **MenuToggleView.swift**
  - Updated header comment to GoxViet

- [x] **PerAppModeManager.swift**
  - Updated header comment to GoxViet
  - Changed description: "Vietnamese input mode" → "Gõ Việt input mode"

- [x] **TextInjectionHelper.swift**
  - Updated header comment to GoxViet

### 3. Rust Core
- [x] **core/Cargo.toml**
  - Package name: `vietnamese-ime-core` → `goxviet-core`
  - Library name: `vietnamese_ime_core` → `goxviet_core`
  - Version updated to 1.0.2
  - Authors: "Vietnamese IME Contributors" → "GoxViet Contributors"
  - Description: "Gõ Việt - Vietnamese input method core engine"

---

## 🔧 Remaining Tasks - CRITICAL

### 1. Xcode Project Configuration (Must Do in Xcode GUI)

#### A. Open Project in Xcode
```bash
cd goxviet/platforms/macos/goxviet
open goxviet.xcodeproj
```

#### B. Update Project Settings
1. **Select project** in navigator (blue icon at top)
2. **General Tab:**
   - Display Name: `GoxViet`
   - Bundle Identifier: `com.goxviet.ime`
   - Version: `1.0.2`
   - Build: `1`

3. **Build Settings Tab:**
   - Search for "Product Name"
   - Change to: `goxviet`
   - Search for "Product Bundle Identifier"
   - Verify: `com.goxviet.ime`

4. **Signing & Capabilities Tab:**
   - Update Team (if needed)
   - Update signing certificate
   - Verify entitlements file: `goxviet.entitlements`

#### C. Update Target
1. **Select target** "VietnameseIMEFast" in project navigator
2. **Rename target** to: `goxviet`
   - Right-click target → Rename → `goxviet`

#### D. Update Scheme
1. **Product → Scheme → Manage Schemes**
2. Rename scheme: `VietnameseIMEFast` → `goxviet`
3. Ensure "Shared" is checked

#### E. Update Info.plist (if needed)
- Verify CFBundleDisplayName: `GoxViet`
- Verify CFBundleIdentifier: `com.goxviet.ime`

### 2. Rebuild Rust Core
```bash
cd goxviet/core
cargo clean
cargo build --release

# Verify new library name
ls -la target/release/libgoxviet_core.*
```

### 3. Update Xcode Library Reference
In Xcode:
1. Remove old library reference: `libvietnamese_ime_core.dylib`
2. Add new library:
   - Right-click on project → Add Files
   - Navigate to: `goxviet/core/target/release/`
   - Add: `libgoxviet_core.a` or `libgoxviet_core.dylib`

### 4. Update Build Settings
In Xcode Build Settings:
1. Search for "Library Search Paths"
2. Update path to: `$(PROJECT_DIR)/../../core/target/release`
3. Search for "Other Linker Flags"
4. Verify: `-lgoxviet_core`

### 5. Clean & Rebuild
```bash
# In Xcode
Product → Clean Build Folder (Cmd+Shift+K)
Product → Build (Cmd+B)

# Or via command line
cd goxviet/platforms/macos/goxviet
xcodebuild clean
xcodebuild -scheme goxviet -configuration Release build
```

---

## 📝 Documentation Updates Needed

### Files to Update (Search & Replace)
Update all references in documentation:

```bash
cd goxviet

# Find all occurrences of old names
grep -r "Vietnamese IME" docs/ README.md CHANGELOG.md
grep -r "VietnameseIMEFast" docs/ README.md
grep -r "vietnamese-ime" docs/ README.md .github/
grep -r "com.vietnamese.ime" docs/
```

### Key Files to Update:
- [ ] `README.md` - Update project name, structure, bundle ID
- [ ] `CHANGELOG.md` - Update all references
- [ ] `docs/README.md` - Update documentation index
- [ ] `docs/**/*.md` - Update all documentation files
- [ ] `.github/copilot-instructions.md` - Update project structure
- [ ] Build scripts (if any)
- [ ] CI/CD configuration (if any)

---

## 🧪 Testing Checklist

### After Xcode Updates:
- [ ] Build succeeds without errors
- [ ] App launches correctly
- [ ] Menu bar icon appears as "🇻🇳" or "EN"
- [ ] About dialog shows "GoxViet - Gõ Việt"
- [ ] Version displays as "1.0.2"
- [ ] Bundle ID is `com.goxviet.ime`
- [ ] Logs appear in `~/Library/Logs/GoxViet/keyboard.log`
- [ ] Accessibility permission prompt shows "GoxViet"
- [ ] UserDefaults keys use `com.goxviet.ime.*`
- [ ] Toggle shortcut works (Control+Space)
- [ ] Vietnamese input works correctly
- [ ] Per-app mode saves settings correctly

### Verify Settings:
```bash
# Check UserDefaults
defaults read com.goxviet.ime

# Check logs
tail -f ~/Library/Logs/GoxViet/keyboard.log

# Check bundle ID
mdls -name kMDItemCFBundleIdentifier /path/to/GoxViet.app
```

---

## 🔄 Git Commit & Push

After completing all changes:

```bash
cd goxviet

# Add all changes
git add .

# Commit with clear message
git commit -m "Rebrand to Gõ Việt (GoxViet)

- Rename project from VietnameseIMEFast to goxviet
- Update bundle ID to com.goxviet.ime
- Update all branding: Gõ Việt / GoxViet
- Update Rust core package name to goxviet-core
- Update log path to ~/Library/Logs/GoxViet/
- Version bump to 1.0.2
- All Swift files updated with new branding
- Xcode project and targets renamed"

# Push to origin
git push origin main
```

---

## 📊 Summary of Naming Conventions

| Context | Name | Example |
|---------|------|---------|
| **Brand Name** | Gõ Việt | "Gõ Việt - Vietnamese IME" |
| **Display Name** | GoxViet | App name in Finder, menu bar |
| **Code/Technical** | goxviet | File names, variables, functions |
| **Bundle ID** | com.goxviet.ime | Reverse domain notation |
| **Rust Package** | goxviet-core | Cargo package name |
| **Rust Library** | goxviet_core | Rust crate name (snake_case) |
| **UserDefaults** | com.goxviet.ime.* | Settings keys |
| **Log Path** | GoxViet | ~/Library/Logs/GoxViet/ |
| **Git Repo** | goxviet | github.com/username/goxviet |

---

## 🎯 Priority Order

1. **CRITICAL - Must do first:**
   - [ ] Complete Xcode project configuration
   - [ ] Rebuild Rust core with new name
   - [ ] Update library references in Xcode
   - [ ] Test build and basic functionality

2. **HIGH - Do next:**
   - [ ] Update all documentation files
   - [ ] Update README and CHANGELOG
   - [ ] Update project rules in .github/

3. **MEDIUM - Can do later:**
   - [ ] Update example scripts
   - [ ] Update build automation
   - [ ] Create new icons/assets with GoxViet branding

4. **LOW - Optional:**
   - [ ] Update release notes templates
   - [ ] Update contribution guidelines

---

## 🚨 Common Issues & Solutions

### Issue 1: Build fails with "library not found"
**Solution:**
```bash
# Rebuild Rust core
cd core && cargo clean && cargo build --release

# Update library search path in Xcode Build Settings
# Verify linker flags: -lgoxviet_core
```

### Issue 2: App crashes on launch
**Solution:**
- Verify bundle ID matches in all places
- Check Info.plist has correct values
- Verify entitlements file is set correctly
- Check signing certificate

### Issue 3: UserDefaults not migrating
**Solution:**
Users will need to reconfigure settings. Consider adding migration code:
```swift
// In AppState.init()
if let oldValue = UserDefaults.standard.bool(forKey: "com.vietnamese.ime.smartMode") {
    UserDefaults.standard.set(oldValue, forKey: "com.goxviet.ime.smartMode")
}
```

### Issue 4: Logs not appearing
**Solution:**
```bash
# Create log directory manually
mkdir -p ~/Library/Logs/GoxViet
chmod 755 ~/Library/Logs/GoxViet
```

---

## 📞 Support

If you encounter issues:
1. Check Xcode build logs: Product → Show Build Log
2. Check runtime logs: `~/Library/Logs/GoxViet/keyboard.log`
3. Verify all naming is consistent (case-sensitive!)
4. Clean build folder and rebuild

---

## ✅ Final Verification

Before considering rebranding complete:
- [ ] All files renamed and updated
- [ ] Xcode project builds successfully
- [ ] App runs and functions correctly
- [ ] All branding shows "Gõ Việt" or "GoxViet"
- [ ] No references to old names in code
- [ ] Documentation updated
- [ ] Git committed and pushed
- [ ] README reflects new branding

---

**Status:** 🟡 IN PROGRESS - Xcode configuration pending  
**Next Step:** Complete Xcode project updates (see section "Remaining Tasks")  
**Estimated Time:** 30-45 minutes for Xcode updates + testing  

---

**Prepared by:** GoxViet Development Team  
**Date:** 2025-12-21  
**Version:** 1.0.2