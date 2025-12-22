# SETTINGS UI IMPLEMENTATION

**Status:** ✅ Completed  
**Date:** 2025-01-XX  
**Version:** 1.0.0

---

## Overview

This document describes the implementation of the Settings UI for GoxViet IME on macOS. The new Settings window replaces the previous alert-based configuration interface with a modern SwiftUI-based solution.

---

## Architecture

### Components

1. **SettingsView.swift** - Main SwiftUI view with tabbed interface
2. **SettingsWindowController.swift** - NSWindowController managing window lifecycle
3. **AppDelegate.swift** - Updated to use new Settings window

### Design Pattern

- **Singleton Pattern:** `SettingsWindowController.shared` ensures single settings window instance
- **Observer Pattern:** Settings changes propagate through `AppState` and `NotificationCenter`
- **MVVM-like:** SwiftUI views observe `@StateObject` for reactive updates

---

## Features

### 1. General Settings Tab

**Icon:** `gearshape.fill`

**Sections:**
- **Input Method**
  - Telex / VNI selector (segmented control)
  - Real-time switching via `InputManager.shared.setInputMethod()`
  - Descriptive help text for each method

- **Tone Placement Style**
  - Traditional (hòa, thủy) / Modern (hoà, thuỷ)
  - Radio group picker
  - Updates via `InputManager.shared.setModernToneStyle()`

- **Smart Features**
  - ESC key restores original word (toggle)
  - Free tone placement (toggle)
  - Each with descriptive help text

### 2. Per-App Mode Tab

**Icon:** `app.badge.fill`

**Sections:**
- **Smart Mode Toggle**
  - Enable/disable per-app mode
  - Automatically remembers Vietnamese input state per application

- **Current Application Info**
  - Shows active app name and bundle ID
  - Visual indicator (green/gray dot) for enabled/disabled state

- **Saved Applications List**
  - Scrollable list of apps with custom settings
  - Shows app name, bundle ID, and state
  - Remove button (X) for each entry
  - Clear All button with confirmation dialog
  - Shows capacity usage: "X / 100 apps"

### 3. Advanced Settings Tab

**Icon:** `slider.horizontal.3`

**Sections:**
- **Keyboard Shortcuts**
  - Display current toggle shortcut (read-only)
  - Instructions to change via System Settings

- **Performance Metrics** (TODO: FFI implementation needed)
  - Total Keystrokes
  - Backspace Count
  - Average Buffer Length
  - Reset Statistics button

- **Debug** (DEBUG builds only)
  - Open Log File button
  - Shows log file path

### 4. About Tab

**Icon:** `info.circle.fill`

**Content:**
- App icon (system icon placeholder)
- App name: "GoxViet" / "Gõ Việt"
- Version and build number
- Description
- Feature highlights with icons
- Links to GitHub and Issue tracker
- Copyright notice

---

## Implementation Details

### Window Configuration

```swift
// Window properties
- Size: 600x500 (default)
- Min Size: 500x400
- Max Size: 800x700
- Style: titled, closable, miniaturizable, resizable
- Level: floating
- Behavior: moveToActiveSpace, fullScreenAuxiliary
- Released when closed: false (singleton pattern)
```

### State Management

All settings are persisted via `AppState.shared` using `UserDefaults`:

```swift
// UserDefaults Keys
Keys.inputMethod          // Int: 0=Telex, 1=VNI
Keys.modernToneStyle      // Bool
Keys.escRestore           // Bool
Keys.freeTone             // Bool
Keys.smartModeEnabled     // Bool
Keys.perAppModes          // [String: Bool] dictionary
```

### Integration Points

1. **AppDelegate.swift**
   - Removed old `showSettings()` alert-based implementation
   - Removed `clearPerAppSettings()` (moved to SettingsView)
   - New: `SettingsWindowController.shared.show()`

2. **InputManager.swift**
   - Already has required methods:
     - `setInputMethod(_:)`
     - `setModernToneStyle(_:)`
     - `setEscRestore(_:)`
     - `setFreeTone(_:)`

3. **AppState.swift**
   - Already implements all required state management
   - Per-app mode dictionary management
   - Capacity limits (MAX_PER_APP_ENTRIES = 100)

---

## TODO: Rust FFI Integration

### Metrics API (High Priority)

Need to expose metrics from Rust core:

```rust
// core/src/engine/metrics.rs (new file)
pub struct EngineMetrics {
    pub total_keystrokes: u64,
    pub backspace_count: u64,
    pub simple_backspace_count: u64,
    pub complex_backspace_count: u64,
    pub avg_buffer_length: f64,
}

// FFI functions needed
#[no_mangle]
pub extern "C" fn ime_get_metrics() -> EngineMetrics { ... }

#[no_mangle]
pub extern "C" fn ime_reset_metrics() { ... }
```

**Swift side:**
```swift
// Add to RustBridge or InputManager
func getMetrics() -> EngineMetrics {
    let metrics = ime_get_metrics()
    return EngineMetrics(
        totalKeystrokes: metrics.total_keystrokes,
        backspaceCount: metrics.backspace_count,
        avgBufferLength: metrics.avg_buffer_length
    )
}
```

---

## File Structure

```
goxviet/platforms/macos/goxviet/goxviet/
├── SettingsView.swift              ✅ NEW (627 lines)
├── SettingsWindowController.swift   ✅ NEW (100 lines)
├── AppDelegate.swift               ✅ MODIFIED (-61 lines)
└── (other existing files...)
```

---

## How to Add Files to Xcode Project

**IMPORTANT:** New Swift files must be added to Xcode project manually.

### Steps:

1. **Open Xcode Project:**
   ```bash
   cd platforms/macos/goxviet
   open goxviet.xcodeproj
   ```

2. **Add SettingsView.swift:**
   - Right-click on `goxviet` group in Project Navigator
   - Select "Add Files to 'goxviet'..."
   - Navigate to `goxviet/SettingsView.swift`
   - ✅ Check "Copy items if needed" (should be unchecked - already in folder)
   - ✅ Check "Add to targets: goxviet"
   - Click "Add"

3. **Add SettingsWindowController.swift:**
   - Repeat above steps for `goxviet/SettingsWindowController.swift`

4. **Verify:**
   - Select `goxviet` target
   - Go to "Build Phases" → "Compile Sources"
   - Confirm both files are listed

5. **Build:**
   ```bash
   # Clean build
   xcodebuild clean
   xcodebuild -configuration Debug
   ```

---

## Testing Checklist

### General Settings
- [ ] Input Method switcher (Telex ↔ VNI) works in real-time
- [ ] Tone Style switcher works and persists
- [ ] ESC Restore toggle works
- [ ] Free Tone toggle works
- [ ] All changes persist after relaunch

### Per-App Mode
- [ ] Smart Mode toggle works
- [ ] Current app info displays correctly
- [ ] Saved apps list updates dynamically
- [ ] Remove individual app setting works
- [ ] Clear All confirmation dialog works
- [ ] Capacity counter displays correctly

### Advanced Settings
- [ ] Keyboard shortcut displays correctly
- [ ] Performance metrics display (when FFI implemented)
- [ ] Reset statistics button works (when FFI implemented)
- [ ] Open Log File button works (DEBUG)

### About
- [ ] Version/build number displays correctly
- [ ] Links open in browser
- [ ] All text displays properly

### Window Behavior
- [ ] Window can be opened/closed multiple times
- [ ] Window size persists between sessions
- [ ] Window can be minimized/restored
- [ ] Only one settings window can be open at a time
- [ ] Window floats above other windows
- [ ] Window moves to active space

---

## Known Issues & Limitations

1. **Metrics API Not Implemented**
   - Performance tab shows placeholder data (0, 0, 0.0)
   - Need to implement Rust FFI for `ime_get_metrics()` and `ime_reset_metrics()`

2. **App Icon**
   - Currently using system icon (`keyboard.fill`)
   - Should replace with actual app icon when available

3. **GitHub Links**
   - Placeholder URLs: `https://github.com/yourusername/goxviet`
   - Update with actual repository URL

---

## Future Enhancements

### Short Term
1. Implement Metrics FFI (see TODO section above)
2. Add real app icon
3. Add keyboard shortcut customization UI
4. Add import/export settings

### Long Term (from RUST_CORE_ROADMAP.md)
1. **Text Expansion (Gõ tắt)**
   - Add new tab for shortcut management
   - Import/export shortcuts
   - Enable/disable per app

2. **English Word Handling**
   - Settings toggle for smart English detection
   - Whitelist editor

3. **Shift+Backspace**
   - Settings toggle for quick word deletion

---

## References

### Related Documentation
- `docs/project/RUST_CORE_ROADMAP.md` - Roadmap including future settings features
- `.github/copilot-instructions.md` - Project naming conventions and architecture rules
- `.github/instructions/00_master_rules.md` - Git workflow and commit standards

### Code References
- `AppState.swift` - State management and UserDefaults
- `InputManager.swift` - Core input processing and configuration
- `RustBridge.swift` - FFI interface to Rust core
- `MenuToggleView.swift` - Reference for custom SwiftUI menu views

---

## Commit Information

### Branch
```bash
git checkout -b feat/settings-ui-swiftui
```

### Commit Message
```
feat(macos): add SwiftUI-based Settings window

- Add SettingsView with 4 tabs (General, Per-App, Advanced, About)
- Add SettingsWindowController for window lifecycle management
- Replace alert-based settings with modern UI
- General tab: input method, tone style, smart features
- Per-App tab: smart mode, app list with remove/clear all
- Advanced tab: shortcuts, performance metrics (FFI TODO), debug
- About tab: app info, features, links

BREAKING CHANGE: Removed AppDelegate.clearPerAppSettings() method
(functionality moved to SettingsView with improved UX)

TODO: Implement Rust FFI for performance metrics
- ime_get_metrics() -> EngineMetrics
- ime_reset_metrics()

Refs: docs/project/RUST_CORE_ROADMAP.md
```

---

## Manual Steps After Commit

1. **Add files to Xcode** (see "How to Add Files to Xcode Project" above)
2. **Test build:**
   ```bash
   cd platforms/macos/goxviet
   xcodebuild clean
   xcodebuild -configuration Debug
   ```
3. **Test Settings window:**
   - Run app
   - Click menu bar icon → "Settings..."
   - Verify all tabs load
   - Test all toggles and controls

4. **Commit Xcode project changes:**
   ```bash
   git add goxviet.xcodeproj/project.pbxproj
   git commit -m "build(macos): add SettingsView files to Xcode project"
   ```

---

## Success Criteria

✅ Settings window opens without crashes  
✅ All tabs are accessible and render correctly  
✅ Settings changes persist via AppState/UserDefaults  
✅ Input method changes take effect immediately  
✅ Per-app mode list updates dynamically  
✅ Window behavior follows macOS standards  
⏳ Performance metrics (pending FFI implementation)  

---

*Last updated: 2025-01-XX*
*Author: GoxViet Development Team*