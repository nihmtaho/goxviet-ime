# SHORTCUT CUSTOMIZATION ROADMAP

## Overview

Roadmap chi tiết cho việc phát triển tính năng customization keyboard shortcuts trong Vietnamese IME.

**Current Status:** Phase 1 Complete ✅  
**Next Phase:** Phase 2 - Settings UI & Customization 🎯  
**Timeline:** 3-6 months for complete implementation

---

## Phase 1: Core Shortcut Toggle ✅ COMPLETE

**Status:** ✅ Shipped (2024-01-20)

### Achievements

- ✅ Default Control+Space shortcut
- ✅ High-priority event capture (`.headInsertEventTap`)
- ✅ Persistent configuration (UserDefaults)
- ✅ System-wide operation (all apps)
- ✅ Performance: ~2ms latency, < 0.05% CPU
- ✅ Comprehensive documentation (2,900+ lines)
- ✅ Menu bar integration
- ✅ Notification system for updates

### Technical Foundation

```swift
struct KeyboardShortcut: Codable {
    var keyCode: UInt16
    var modifiers: UInt64
    
    func matches(keyCode: UInt16, flags: CGEventFlags) -> Bool
    func save()
    static func load() -> KeyboardShortcut
}
```

### Files Delivered

- `KeyboardShortcut.swift` (240 lines)
- `InputManager.swift` (updated)
- `AppDelegate.swift` (updated)
- 7 documentation files (2,900+ lines)

---

## Phase 2: Settings UI & Customization 🎯 NEXT

**Timeline:** 2-3 months  
**Priority:** High  
**Complexity:** Medium-High

### 2.1. Settings Window UI (4 weeks)

#### Week 1-2: Basic Settings Window

**Deliverables:**
- [ ] `SettingsWindow.swift` - Main settings window controller
- [ ] `SettingsViewController.swift` - Tab view controller
- [ ] `ShortcutSettingsView.swift` - Shortcut tab content
- [ ] Window design in Interface Builder or SwiftUI

**Features:**
- Tabbed interface (Shortcuts, Input Methods, Appearance, About)
- 600×400 window size, centered on screen
- Modal window or preferences-style
- Persistent window position

**UI Components:**
```swift
class SettingsWindow: NSWindow {
    var tabView: NSTabView
    var shortcutTab: ShortcutSettingsView
    var inputMethodTab: InputMethodSettingsView
    var appearanceTab: AppearanceSettingsView
    var aboutTab: AboutView
}
```

**Menu Integration:**
```swift
// In AppDelegate.swift
@objc func showSettings() {
    SettingsWindow.shared.show()
    SettingsWindow.shared.selectTab(.shortcuts)
}
```

---

#### Week 3-4: Shortcut Tab Layout

**Layout Design:**

```
┌─────────────────────────────────────────────────────┐
│  Settings                                      ⊗    │
├─────────────────────────────────────────────────────┤
│  Shortcuts  Input  Appearance  About                │
├─────────────────────────────────────────────────────┤
│                                                      │
│  Toggle Shortcut                                     │
│  ┌──────────────────────────────┐  [Record]         │
│  │  ⌃Space                      │  [Test]           │
│  └──────────────────────────────┘  [Reset]          │
│                                                      │
│  ⚠️  No conflicts detected                          │
│                                                      │
│  Preset Shortcuts:                                   │
│  ○  Control+Space (Default)                         │
│  ○  Control+Shift+Space                             │
│  ○  Control+Option+Space                            │
│  ○  Control+Shift+V                                 │
│  ●  Custom                                          │
│                                                      │
│  ☐  Enable secondary shortcut                      │
│  ☐  Enable modifier-only shortcuts                 │
│                                                      │
└─────────────────────────────────────────────────────┘
```

**Components:**
- Current shortcut display (read-only text field)
- Record button (start shortcut recording)
- Test button (verify shortcut works)
- Reset button (back to default)
- Conflict indicator (⚠️ or ✅)
- Preset radio buttons
- Advanced options checkboxes

---

### 2.2. Visual Shortcut Recorder (2 weeks)

#### Week 5-6: Recording Implementation

**Deliverables:**
- [ ] `ShortcutRecorder.swift` - Recording component
- [ ] Recording state management
- [ ] Visual feedback during recording
- [ ] Key combination parsing

**User Flow:**

```
1. User clicks "Record" button
   → Button text changes to "Recording..."
   → TextField shows "Press shortcut..."
   → Background color changes (recording state)

2. User presses key combination (e.g., Control+Shift+V)
   → TextField shows "⌃⇧V"
   → System validates combination
   → Conflict check runs

3. User confirms or cancels
   → If valid: Save and apply
   → If conflict: Show warning dialog
   → If invalid: Show error message
```

**Implementation:**

```swift
class ShortcutRecorder: NSView {
    var isRecording: Bool = false
    var recordedShortcut: KeyboardShortcut?
    var onRecordingComplete: ((KeyboardShortcut) -> Void)?
    
    func startRecording() {
        isRecording = true
        // Start listening for key events
        NSEvent.addLocalMonitorForEvents(matching: .keyDown) { event in
            self.handleRecordedEvent(event)
            return nil // Consume event
        }
    }
    
    func handleRecordedEvent(_ event: NSEvent) {
        let keyCode = UInt16(event.keyCode)
        let modifiers = event.modifierFlags.rawValue
        
        // Validate
        guard validateShortcut(keyCode: keyCode, modifiers: modifiers) else {
            showError("Invalid shortcut")
            return
        }
        
        let shortcut = KeyboardShortcut(keyCode: keyCode, modifiers: modifiers)
        recordedShortcut = shortcut
        
        // Check conflicts
        let conflicts = detectConflicts(shortcut)
        if !conflicts.isEmpty {
            showConflictWarning(conflicts)
        }
        
        stopRecording()
        onRecordingComplete?(shortcut)
    }
    
    func stopRecording() {
        isRecording = false
        NSEvent.removeMonitor(eventMonitor!)
    }
}
```

**Validation Rules:**
- Must have at least one modifier (Control, Option, Shift, Command)
- Cannot be system-only shortcuts (Cmd+Tab, Cmd+Q, etc.)
- Should not conflict with critical system shortcuts
- Key code must be valid (0-127 for standard keys)

---

### 2.3. Conflict Detection (2 weeks)

#### Week 7-8: Conflict Detection System

**Deliverables:**
- [ ] `ConflictDetector.swift` - Conflict detection engine
- [ ] System shortcuts database
- [ ] App shortcuts database
- [ ] Conflict resolution UI

**System Shortcuts Database:**

```swift
struct SystemShortcutDatabase {
    static let shortcuts: [SystemShortcut] = [
        SystemShortcut(
            keyCode: 0x31,
            modifiers: .maskCommand,
            name: "Spotlight",
            description: "Open Spotlight search",
            severity: .critical
        ),
        SystemShortcut(
            keyCode: 0x30,
            modifiers: .maskCommand,
            name: "App Switcher",
            description: "Switch between applications",
            severity: .critical
        ),
        SystemShortcut(
            keyCode: 0x0C,
            modifiers: .maskCommand,
            name: "Quit",
            description: "Quit application",
            severity: .critical
        ),
        // ... more system shortcuts
    ]
}
```

**App Shortcuts Database:**

```swift
struct AppShortcutDatabase {
    static let vscode: [AppShortcut] = [
        AppShortcut(
            keyCode: 0x31,
            modifiers: [.maskControl],
            name: "Show All Commands",
            app: "Visual Studio Code",
            severity: .medium
        ),
        // ... more VSCode shortcuts
    ]
    
    static let terminal: [AppShortcut] = [
        AppShortcut(
            keyCode: 0x31,
            modifiers: [.maskControl],
            name: "Mark",
            app: "Terminal",
            severity: .low
        ),
    ]
}
```

**Conflict Detection Logic:**

```swift
class ConflictDetector {
    func detectConflicts(_ shortcut: KeyboardShortcut) -> [Conflict] {
        var conflicts: [Conflict] = []
        
        // Check system shortcuts
        for systemShortcut in SystemShortcutDatabase.shortcuts {
            if systemShortcut.matches(shortcut) {
                conflicts.append(.system(systemShortcut))
            }
        }
        
        // Check app shortcuts
        let installedApps = detectInstalledApps()
        for app in installedApps {
            if let appShortcuts = AppShortcutDatabase.shortcuts(for: app) {
                for appShortcut in appShortcuts {
                    if appShortcut.matches(shortcut) {
                        conflicts.append(.app(appShortcut))
                    }
                }
            }
        }
        
        return conflicts
    }
    
    func suggestAlternatives(_ shortcut: KeyboardShortcut) -> [KeyboardShortcut] {
        // Generate alternative shortcuts
        var alternatives: [KeyboardShortcut] = []
        
        // Try different modifiers
        let modifierCombinations: [CGEventFlags] = [
            .maskControl,
            [.maskControl, .maskShift],
            [.maskControl, .maskAlternate],
            [.maskControl, .maskShift, .maskAlternate],
        ]
        
        for modifiers in modifierCombinations {
            let alternative = KeyboardShortcut(
                keyCode: shortcut.keyCode,
                modifiers: modifiers.rawValue
            )
            
            if detectConflicts(alternative).isEmpty {
                alternatives.append(alternative)
            }
        }
        
        return alternatives
    }
}
```

**Conflict Warning UI:**

```
┌─────────────────────────────────────────────────────┐
│  ⚠️  Shortcut Conflict Detected                    │
├─────────────────────────────────────────────────────┤
│                                                      │
│  The shortcut ⌘Space conflicts with:                │
│                                                      │
│  • Spotlight (System) - Critical                    │
│                                                      │
│  Using this shortcut may prevent Spotlight from     │
│  working. Vietnamese IME will capture the shortcut  │
│  first, but this is not recommended.                │
│                                                      │
│  Suggested alternatives:                             │
│  • ⌃Space (No conflicts)                            │
│  • ⌃⇧Space (No conflicts)                           │
│  • ⌃⌥Space (No conflicts)                           │
│                                                      │
│           [Use Anyway]  [Choose Alternative]        │
└─────────────────────────────────────────────────────┘
```

---

### 2.4. Test & Reset Features (1 week)

#### Week 9: Test and Reset Implementation

**Test Shortcut Button:**

```swift
@objc func testShortcut() {
    let currentShortcut = InputManager.shared.getCurrentShortcut()
    
    // Show instruction dialog
    let alert = NSAlert()
    alert.messageText = "Test Shortcut"
    alert.informativeText = "Press \(currentShortcut.displayString) now to test.\n\nThe IME should toggle ON/OFF."
    alert.alertStyle = .informational
    alert.addButton(withTitle: "OK")
    alert.runModal()
    
    // Monitor for shortcut within 10 seconds
    startShortcutMonitoring { detected in
        if detected {
            self.showSuccess("✅ Shortcut works!")
        } else {
            self.showError("❌ Shortcut not detected. Try again.")
        }
    }
}
```

**Reset to Default:**

```swift
@objc func resetToDefault() {
    let alert = NSAlert()
    alert.messageText = "Reset Shortcut?"
    alert.informativeText = "This will reset the toggle shortcut to Control+Space (⌃Space)."
    alert.alertStyle = .warning
    alert.addButton(withTitle: "Reset")
    alert.addButton(withTitle: "Cancel")
    
    if alert.runModal() == .alertFirstButtonReturn {
        let defaultShortcut = KeyboardShortcut.default
        defaultShortcut.save()
        
        updateUI(shortcut: defaultShortcut)
        showSuccess("Shortcut reset to default (⌃Space)")
    }
}
```

---

### 2.5. Testing & Polish (1 week)

#### Week 10: Integration Testing

**Test Checklist:**
- [ ] Settings window opens/closes correctly
- [ ] Shortcut recorder captures all key combinations
- [ ] Conflict detection works for system shortcuts
- [ ] Conflict detection works for app shortcuts
- [ ] Test button verifies shortcut works
- [ ] Reset button returns to default
- [ ] Changes persist across app restarts
- [ ] UI updates in real-time
- [ ] No memory leaks
- [ ] No crashes

**Performance Testing:**
- Settings window opens in < 100ms
- Conflict detection runs in < 50ms
- UI remains responsive during recording
- No lag when switching tabs

---

## Phase 3: Advanced Features 🔮 FUTURE

**Timeline:** 2-3 months  
**Priority:** Medium  
**Complexity:** High

### 3.1. Multiple Shortcuts (2 weeks)

**Feature:**
- Primary shortcut (Control+Space)
- Secondary shortcut (backup, e.g., Control+Shift+V)
- Both work independently
- Priority: Primary > Secondary

**Implementation:**

```swift
struct ShortcutConfiguration: Codable {
    var primary: KeyboardShortcut
    var secondary: KeyboardShortcut?
    var isSecondaryEnabled: Bool
}

// In InputManager
func handleEvent(event: CGEvent) -> Unmanaged<CGEvent>? {
    let keyCode = UInt16(event.getIntegerValueField(.keyboardEventKeycode))
    let flags = event.flags
    
    // Check primary first
    if config.primary.matches(keyCode: keyCode, flags: flags) {
        toggleEnabled()
        return nil
    }
    
    // Check secondary
    if config.isSecondaryEnabled,
       let secondary = config.secondary,
       secondary.matches(keyCode: keyCode, flags: flags) {
        toggleEnabled()
        return nil
    }
    
    // Continue normal processing...
}
```

**UI:**

```
Primary Shortcut:
┌──────────────────────────────┐  [Record]  [Test]
│  ⌃Space                      │
└──────────────────────────────┘

☑️  Enable secondary shortcut

Secondary Shortcut:
┌──────────────────────────────┐  [Record]  [Test]
│  ⌃⇧V                         │
└──────────────────────────────┘
```

---

### 3.2. Modifier-Only Shortcuts (3 weeks)

**Feature:**
- Double-tap Shift to toggle
- Double-tap Control to toggle
- Configurable timing threshold (200-500ms)

**Implementation Challenges:**
- Detect modifier key press/release
- Track timing between presses
- Avoid false positives
- Handle held modifiers

**Logic:**

```swift
class ModifierOnlyDetector {
    var lastModifierPress: Date?
    var lastModifierKey: UInt16?
    var threshold: TimeInterval = 0.3 // 300ms
    
    func handleFlagsChanged(event: CGEvent) -> Bool {
        let flags = event.flags
        let timestamp = Date()
        
        // Detect Shift key
        if flags.contains(.maskShift) {
            if let lastPress = lastModifierPress,
               lastModifierKey == KeyCode.shift,
               timestamp.timeIntervalSince(lastPress) < threshold {
                // Double-tap detected!
                return true
            }
            
            lastModifierPress = timestamp
            lastModifierKey = KeyCode.shift
        }
        
        return false
    }
}
```

**UI:**

```
☑️  Enable modifier-only shortcuts

Modifier-Only Shortcut:
[Double-tap Shift ▼]

Timing Threshold:
[────●────────] 300ms
Fast    Normal    Slow
```

---

### 3.3. Per-App Shortcuts (3 weeks)

**Feature:**
- Different shortcut for different apps
- VSCode: Control+Shift+Space
- Terminal: Control+Space
- Others: Control+Space (default)

**Implementation:**

```swift
struct AppShortcutOverride: Codable {
    var bundleIdentifier: String
    var appName: String
    var shortcut: KeyboardShortcut
    var isEnabled: Bool
}

class PerAppShortcutManager {
    var overrides: [AppShortcutOverride] = []
    var defaultShortcut: KeyboardShortcut
    
    func getShortcut(for app: String) -> KeyboardShortcut {
        if let override = overrides.first(where: { $0.bundleIdentifier == app && $0.isEnabled }) {
            return override.shortcut
        }
        return defaultShortcut
    }
    
    func getCurrentAppShortcut() -> KeyboardShortcut {
        let frontmostApp = NSWorkspace.shared.frontmostApplication?.bundleIdentifier ?? ""
        return getShortcut(for: frontmostApp)
    }
}

// In InputManager
func handleEvent(event: CGEvent) -> Unmanaged<CGEvent>? {
    let currentShortcut = PerAppShortcutManager.shared.getCurrentAppShortcut()
    
    if currentShortcut.matches(keyCode: keyCode, flags: flags) {
        toggleEnabled()
        return nil
    }
    
    // Continue...
}
```

**UI:**

```
Per-App Shortcut Overrides:

┌────────────────────────────────────────────────────┐
│ App                  │ Shortcut    │ Enabled      │
├────────────────────────────────────────────────────┤
│ Visual Studio Code   │ ⌃⇧Space     │ ☑️           │
│ Terminal             │ ⌃Space      │ ☑️           │
│ Slack                │ ⌃⌥Space     │ ☐            │
└────────────────────────────────────────────────────┘

[+ Add Override]  [- Remove]  [Edit]
```

---

### 3.4. Shortcut Profiles (2 weeks)

**Feature:**
- Profile 1: Developer (Control+Shift+Space, per-app overrides)
- Profile 2: Writer (Control+Space, simple)
- Profile 3: Custom

**Implementation:**

```swift
struct ShortcutProfile: Codable {
    var id: UUID
    var name: String
    var icon: String
    var shortcut: KeyboardShortcut
    var secondaryShortcut: KeyboardShortcut?
    var appOverrides: [AppShortcutOverride]
    var modifierOnlyEnabled: Bool
}

class ProfileManager {
    var profiles: [ShortcutProfile] = []
    var activeProfile: ShortcutProfile
    
    func switchProfile(_ profile: ShortcutProfile) {
        activeProfile = profile
        InputManager.shared.setShortcut(profile.shortcut)
        // Apply all settings from profile
        NotificationCenter.default.post(name: .profileChanged, object: profile)
    }
}
```

**UI:**

```
Active Profile: [Developer ▼]

Profiles:
● Developer
  - Primary: ⌃⇧Space
  - VSCode: ⌃⇧V
  - Terminal: ⌃Space

○ Writer
  - Primary: ⌃Space
  - Simple, no overrides

○ Custom
  - Primary: ⌃⌥Space
  - ...

[+ New Profile]  [Edit]  [Delete]
```

---

### 3.5. Import/Export (1 week)

**Feature:**
- Export shortcut configuration to JSON
- Import from JSON file
- Share between devices

**Implementation:**

```swift
struct ShortcutConfiguration: Codable {
    var version: String = "1.0"
    var primary: KeyboardShortcut
    var secondary: KeyboardShortcut?
    var profiles: [ShortcutProfile]
    var appOverrides: [AppShortcutOverride]
    var modifierOnly: ModifierOnlyConfig?
}

class ConfigurationManager {
    func exportConfiguration() -> Data? {
        let config = ShortcutConfiguration(
            primary: InputManager.shared.getCurrentShortcut(),
            // ... other settings
        )
        
        return try? JSONEncoder().encode(config)
    }
    
    func importConfiguration(from data: Data) throws {
        let config = try JSONDecoder().decode(ShortcutConfiguration.self, from: data)
        
        // Validate version compatibility
        guard config.version == "1.0" else {
            throw ImportError.incompatibleVersion
        }
        
        // Apply configuration
        config.primary.save()
        // ... apply other settings
    }
}
```

**UI:**

```
Import/Export Settings:

[Export to File...]
Export current shortcut configuration to JSON file

[Import from File...]
Import shortcut configuration from JSON file

⚠️  Importing will replace all current settings
```

---

## Phase 4: Polish & Optimization 🌟 POLISH

**Timeline:** 1-2 months  
**Priority:** Medium  
**Complexity:** Low-Medium

### 4.1. UI/UX Polish

- [ ] Animations (fade in/out, smooth transitions)
- [ ] Better visual feedback during recording
- [ ] Improved conflict warning dialogs
- [ ] Tooltips and help text
- [ ] Keyboard navigation support
- [ ] Dark mode support
- [ ] Accessibility improvements (VoiceOver)

### 4.2. Documentation

- [ ] User guide for Settings UI
- [ ] Video tutorials
- [ ] FAQ section
- [ ] Troubleshooting guide updates
- [ ] In-app help system

### 4.3. Performance

- [ ] Optimize conflict detection (< 50ms)
- [ ] Reduce memory footprint
- [ ] Lazy loading for databases
- [ ] Cache frequently used data

---

## Technical Specifications

### Architecture

```
SettingsWindow
    ├── ShortcutSettingsView
    │   ├── ShortcutRecorder
    │   ├── ConflictDetector
    │   ├── PresetSelector
    │   └── AdvancedOptions
    │
    ├── InputMethodSettingsView
    ├── AppearanceSettingsView
    └── AboutView

Supporting Classes:
    ├── ShortcutConfiguration
    ├── ConflictDetector
    ├── SystemShortcutDatabase
    ├── AppShortcutDatabase
    ├── PerAppShortcutManager
    ├── ProfileManager
    └── ConfigurationManager
```

### Data Models

```swift
// Core
struct KeyboardShortcut: Codable, Equatable
struct ShortcutConfiguration: Codable

// Conflicts
struct SystemShortcut: Codable
struct AppShortcut: Codable
enum Conflict
enum ConflictSeverity

// Advanced
struct AppShortcutOverride: Codable
struct ShortcutProfile: Codable
struct ModifierOnlyConfig: Codable
```

### Storage

```swift
// UserDefaults Keys
"com.vietnamese.ime.shortcutConfiguration"    // Main config
"com.vietnamese.ime.shortcutProfiles"         // Profiles
"com.vietnamese.ime.appOverrides"             // Per-app
"com.vietnamese.ime.activeProfile"            // Current profile
```

---

## Testing Strategy

### Unit Tests
- [ ] ShortcutRecorder captures keys correctly
- [ ] ConflictDetector identifies all conflicts
- [ ] ShortcutConfiguration saves/loads correctly
- [ ] ProfileManager switches profiles
- [ ] Import/Export preserves all data

### Integration Tests
- [ ] Settings window integrates with InputManager
- [ ] Shortcut changes apply immediately
- [ ] Per-app overrides work correctly
- [ ] Profiles apply all settings

### UI Tests
- [ ] Record shortcut flow works end-to-end
- [ ] Conflict warnings appear correctly
- [ ] Test button verifies shortcut
- [ ] Reset button works

### Performance Tests
- [ ] Settings window opens in < 100ms
- [ ] Conflict detection < 50ms
- [ ] Shortcut matching < 1ms
- [ ] No memory leaks after 1000 operations

---

## Success Metrics

### User Experience
- Users can customize shortcuts in < 2 minutes
- 95% of users find their preferred shortcut
- Conflict detection prevents 90% of issues
- Settings are intuitive (no support requests)

### Technical
- Zero crashes in Settings UI
- < 100ms latency for all operations
- Memory usage < 50MB for Settings
- 100% data persistence reliability

### Adoption
- 60%+ users customize default shortcut
- 30%+ users use per-app overrides
- 20%+ users create custom profiles

---

## Dependencies

### Internal
- KeyboardShortcut.swift (already exists)
- InputManager.swift (already exists)
- NotificationCenter integration

### External
- SwiftUI (optional, for modern UI)
- Combine (for reactive updates)
- AppKit (for window management)

### System
- macOS 11.0+ (Accessibility API)
- Xcode 14+ (for building)

---

## Risk Assessment

### High Risk
- **Conflict Detection Accuracy:** May miss some app shortcuts
  - Mitigation: Community-maintained database, user reports
  
- **Modifier-Only Detection:** False positives/negatives
  - Mitigation: Configurable threshold, disable option

### Medium Risk
- **UI Complexity:** Too many options overwhelm users
  - Mitigation: Progressive disclosure, good defaults
  
- **Performance:** Conflict detection may be slow
  - Mitigation: Caching, lazy loading, background processing

### Low Risk
- **Import/Export:** File format changes
  - Mitigation: Versioning, backward compatibility

---

## Timeline Summary

| Phase | Duration | Priority | Status |
|-------|----------|----------|--------|
| Phase 1: Core Toggle | 4 weeks | Critical | ✅ Complete |
| Phase 2: Settings UI | 10 weeks | High | 🎯 Next |
| Phase 3: Advanced Features | 11 weeks | Medium | 🔮 Future |
| Phase 4: Polish | 6 weeks | Medium | 🌟 Future |
| **TOTAL** | **31 weeks (~7 months)** | | |

---

## Conclusion

Roadmap này cung cấp lộ trình chi tiết và thực tế để phát triển tính năng customization keyboard shortcuts. 

**Key Principles:**
- Start simple (Settings UI first)
- Iterate based on user feedback
- Maintain performance (< 100ms operations)
- Keep defaults sensible (most users won't customize)
- Comprehensive testing at each phase

**Next Steps:**
1. Review and approve this roadmap
2. Start Phase 2 (Settings UI)
3. Create Xcode project structure
4. Begin Week 1 implementation

---

**Status:** 📋 Roadmap Complete  
**Version:** 1.0  
**Last Updated:** 2024-01-20  
**Owner:** Vietnamese IME Team