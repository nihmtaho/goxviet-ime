# Phase 2.9: Feature Integration (Text Expansion, Encoding, Shift+Backspace)

## Overview

Phase này tập trung vào việc **tích hợp 3 tính năng core** (đã implement ở Phase 1) vào macOS UI/Settings để người dùng có thể sử dụng:

1. **Text Expansion (Gõ tắt)** - Định nghĩa shortcuts, import/export JSON
2. **Multi-Encoding Output** - Chọn bảng mã: Unicode, TCVN3, VNI, CP1258
3. **Shift+Backspace** - Xóa nhanh một từ

## Duration
Estimated: 8-9 giờ làm việc (2-3 work sessions)

## Objectives

1. **Expose Core Features to UI** - Tạo UI Settings để configure 3 tính năng
2. **RustBridge Extension** - Thêm FFI calls cho Text Expansion & Encoding
3. **SettingsManager Enhancement** - Quản lý state cho 3 tính năng mới
4. **InputManager Integration** - Apply settings khi user gõ
5. **Zero Regression** - Tất cả existing features vẫn hoạt động bình thường

## Scope

### In Scope
- UI Settings cho Text Expansion (list, add, edit, delete, import/export)
- UI Settings cho Output Encoding (picker với 4 options)
- UI Settings cho Shift+Backspace (toggle enable/disable)
- RustBridge FFI calls mới
- SettingsManager properties & persistence
- InputManager apply settings
- Unit tests cho new code
- User documentation

### Out of Scope
- Advanced Text Expansion features (variables, conditions) - defer to Phase 3
- Real-time encoding preview (nice-to-have) - defer if time constraint
- Shortcut conflict detection - defer to Phase 3
- Cloud sync for shortcuts - defer to Phase 4

## Background

### Phase 1 Status (Core Engine)
- ✅ **Text Expansion**: Core logic implemented, FFI exported
  - `ime_set_shortcuts_enabled(bool)`
  - `ime_export_shortcuts_json() -> *char`
  - `ime_shortcuts_count() -> u32`
  - `ime_shortcuts_capacity() -> u32`
  - `ime_clear_shortcuts()`
- ✅ **Multi-Encoding**: Core converter implemented
  - `ime_set_encoding(u8)` - 0=Unicode, 1=TCVN3, 2=VNI, 3=CP1258
  - `ime_get_encoding() -> u8`
- ✅ **Shift+Backspace**: Handled in `ime_key_ext(key, caps, ctrl, shift)`

### Phase 2 Status (UI Components)
- ✅ UI components refactored (GeneralSettingsView, AdvancedSettingsView, etc.)
- ✅ SettingsManager for unified state management
- ✅ TypedNotifications system
- ✅ RustBridgeSafe with error handling

### Gap
- ❌ **NO UI** for users to enable/configure these 3 features
- ❌ Swift code not calling new FFI functions
- ❌ Settings not persisted in UserDefaults

## Milestones

### Milestone 2.9.1: RustBridge & SettingsManager Extension
**Goal**: Expose FFI functions in Swift, add SettingsManager properties

**Tasks**:
1. Extend `RustBridge.swift`:
   - Add Text Expansion functions (enable, export, count, clear)
   - Add Encoding functions (set, get)
   - Document Shift+Backspace in `ime_key_ext`
2. Extend `SettingsManager.swift`:
   - Add `@Published var textExpansionEnabled: Bool`
   - Add `@Published var outputEncoding: OutputEncoding` (enum)
   - Add `@Published var shiftBackspaceEnabled: Bool`
   - Add UserDefaults persistence
   - Sync with RustBridge on change
3. Unit tests:
   - RustBridge functions (mock FFI if needed)
   - SettingsManager properties & persistence

**Deliverables**:
- RustBridge with 3 new feature groups
- SettingsManager with 3 new properties
- 10+ unit tests passing

**Estimated Time**: 2 hours

---

### Milestone 2.9.2: Text Expansion UI
**Goal**: Full-featured Text Expansion settings view

**Tasks**:
1. Create `TextExpansionSettingsView.swift`:
   - Header with description
   - Toggle "Enable Text Expansion"
   - List view showing shortcuts (trigger → replacement)
   - Add button → Sheet to create new shortcut
   - Edit inline or sheet
   - Delete with swipe or button
   - Import JSON button (file picker)
   - Export JSON button (save panel)
   - Search/filter shortcuts
2. Create `ShortcutEditorSheet.swift` (if needed):
   - TextField for trigger
   - TextField for replacement text
   - Preview area
   - Validation (no empty, no duplicates)
3. Integrate into `SettingsRootView`:
   - Add new tab "Text Expansion" or
   - Add as section in GeneralSettingsView
4. Wire up to SettingsManager & RustBridge

**Deliverables**:
- TextExpansionSettingsView with full CRUD
- Import/Export JSON working
- UI matches macOS HIG & existing style
- Settings persist correctly

**Estimated Time**: 3 hours

---

### Milestone 2.9.3: Encoding & Shift+Backspace UI
**Goal**: Simple settings for Encoding & Shift+Backspace

**Tasks**:
1. **Encoding UI** (in AdvancedSettingsView):
   - Add GroupBox "Output Encoding"
   - Picker with 4 options:
     - Unicode (Default) ✓
     - TCVN3 (Legacy)
     - VNI Windows (Legacy)
     - CP1258 (Windows-1258)
   - Description text for each option
   - Warning banner when legacy encoding selected
   - Optional: Preview textfield showing encoding result
2. **Shift+Backspace UI** (in GeneralSettingsView):
   - Add toggle "Enable Shift+Backspace to delete word"
   - Helper text: "Quickly delete entire word with Shift+Backspace"
   - Wire to SettingsManager
3. Wire both to SettingsManager & RustBridge

**Deliverables**:
- Encoding picker in AdvancedSettingsView
- Shift+Backspace toggle in GeneralSettingsView
- Settings apply immediately
- Persist across app restart

**Estimated Time**: 1.5 hours

---

### Milestone 2.9.4: InputManager Integration & Testing
**Goal**: Apply settings in real typing, comprehensive testing

**Tasks**:
1. Update `InputManager.swift`:
   - Read `textExpansionEnabled` from SettingsManager
   - Read `outputEncoding` from SettingsManager
   - Read `shiftBackspaceEnabled` from SettingsManager
   - Call RustBridge to sync settings on app launch & settings change
   - Process Text Expansion in keystroke handling
   - Apply encoding to output
   - Handle Shift+Backspace event (delete word)
2. Comprehensive testing:
   - Unit tests for InputManager logic
   - Manual testing in Xcode:
     - Type "tt" → expands to "thân thiện" (if shortcut defined)
     - Select TCVN3 → output in TCVN3 encoding
     - Press Shift+Backspace → deletes word
   - Integration testing: Settings → InputManager → Core → Output
   - Memory profiling (Instruments)
3. Bug fixes & polish

**Deliverables**:
- InputManager applying all 3 settings
- All features working end-to-end
- 15+ tests passing (unit + manual)
- Zero crashes, zero memory leaks
- Performance: <1ms overhead for settings check

**Estimated Time**: 2 hours

---

### Milestone 2.9.5: Documentation & Final Review
**Goal**: Complete documentation, ready for Phase 3

**Tasks**:
1. Update `.docs/features/platform/macos/settings_features.md`:
   - Add Text Expansion section
   - Add Encoding section
   - Add Shift+Backspace section
   - Screenshots for each
2. Create user guides:
   - How to create shortcuts
   - How to import/export shortcuts JSON
   - When to use legacy encodings
   - Shift+Backspace usage tips
3. Update `.planning/STATE.md`:
   - Mark Phase 2.9 complete
   - Update progress percentages
4. Final review:
   - Code review checklist
   - Manual testing checklist
   - Performance verification
   - Accessibility check (VoiceOver)

**Deliverables**:
- Complete documentation
- User guides with examples
- STATE.md updated
- All checklists complete
- Ready for Phase 3 (Quality & Testing)

**Estimated Time**: 1.5 hours

---

## Technical Approach

### Architecture Integration

```
┌─────────────────────────────────────────────────────────────┐
│                         USER ACTION                         │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                      SETTINGS UI                            │
│  • TextExpansionSettingsView                                │
│  • AdvancedSettingsView (Encoding)                          │
│  • GeneralSettingsView (Shift+Backspace)                    │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼ @Published property change
┌─────────────────────────────────────────────────────────────┐
│                    SETTINGS MANAGER                         │
│  • textExpansionEnabled: Bool                               │
│  • outputEncoding: OutputEncoding                           │
│  • shiftBackspaceEnabled: Bool                              │
│  • Persist to UserDefaults                                  │
│  • Post TypedNotification                                   │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼ Notification listener
┌─────────────────────────────────────────────────────────────┐
│                     INPUT MANAGER                           │
│  • Observe SettingsManager changes                          │
│  • Call RustBridge.setShortcutsEnabled()                    │
│  • Call RustBridge.setEncoding()                            │
│  • Handle Shift+Backspace in key event                      │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼ FFI call
┌─────────────────────────────────────────────────────────────┐
│                      RUST BRIDGE                            │
│  • setShortcutsEnabled(bool) → ime_set_shortcuts_enabled    │
│  • setEncoding(u8) → ime_set_encoding                       │
│  • processKey(shift: true) → ime_key_ext                    │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼ Rust FFI
┌─────────────────────────────────────────────────────────────┐
│                       CORE ENGINE                           │
│  • Text Expansion logic (shortcut matching)                 │
│  • Encoding converter (Unicode → TCVN3/VNI/CP1258)          │
│  • Shift+Backspace handler (delete word)                    │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼ Result
┌─────────────────────────────────────────────────────────────┐
│                          OUTPUT                             │
│  • Expanded text (if shortcut matched)                      │
│  • Encoded text (if legacy encoding)                        │
│  • Word deleted (if Shift+Backspace)                        │
└─────────────────────────────────────────────────────────────┘
```

### Data Flow Examples

#### 1. Text Expansion Flow
```
User types "tt" + Space
  → InputManager.processKey('t')
  → RustBridge.processKey()
  → Core matches "tt" shortcut → "thân thiện"
  → Result.action=Send, chars="thân thiện"
  → InputManager sends text to OS
```

#### 2. Encoding Change Flow
```
User selects "TCVN3" in Settings
  → SettingsManager.outputEncoding = .tcvn3
  → UserDefaults saves
  → TypedNotification posted
  → InputManager receives notification
  → RustBridge.setEncoding(1) // 1=TCVN3
  → Core engine updates encoder
  → Next keystroke uses TCVN3 encoding
```

#### 3. Shift+Backspace Flow
```
User presses Shift+Backspace
  → InputManager detects shift=true, key=delete
  → Checks SettingsManager.shiftBackspaceEnabled
  → RustBridge.processKey(shift: true)
  → Core detects Shift+Delete → action=DeleteWord
  → Result.backspace=5 (word length)
  → InputManager sends 5 backspaces to OS
```

### UI Components Structure

```
SettingsRootView.swift
├── TabView
│   ├── GeneralSettingsView
│   │   ├── Input Method section
│   │   ├── Keyboard Shortcuts section
│   │   └── **NEW: Shift+Backspace toggle**
│   ├── PerAppSettingsView (unchanged)
│   ├── **NEW: TextExpansionSettingsView**
│   │   ├── Enable toggle
│   │   ├── Shortcuts List
│   │   │   ├── Row: trigger → replacement
│   │   │   ├── Add button
│   │   │   ├── Edit inline
│   │   │   └── Delete swipe
│   │   ├── Import JSON button
│   │   └── Export JSON button
│   ├── AdvancedSettingsView
│   │   ├── **NEW: Output Encoding section**
│   │   │   ├── Picker (Unicode/TCVN3/VNI/CP1258)
│   │   │   ├── Description text
│   │   │   └── Warning for legacy
│   │   ├── Engine Metrics (unchanged)
│   │   └── Diagnostics (unchanged)
│   └── AboutSettingsView (unchanged)
```

### UserDefaults Keys

```swift
// New keys for Phase 2.9
static let textExpansionEnabled = "textExpansionEnabled" // Bool, default: true
static let outputEncoding = "outputEncoding" // Int (0-3), default: 0 (Unicode)
static let shiftBackspaceEnabled = "shiftBackspaceEnabled" // Bool, default: false
```

### Enum Definition

```swift
enum OutputEncoding: Int, CaseIterable, Identifiable {
    case unicode = 0
    case tcvn3 = 1
    case vni = 2
    case cp1258 = 3
    
    var id: Int { rawValue }
    
    var displayName: String {
        switch self {
        case .unicode: return "Unicode (Default)"
        case .tcvn3: return "TCVN3 (Legacy)"
        case .vni: return "VNI Windows (Legacy)"
        case .cp1258: return "CP1258 (Windows-1258)"
        }
    }
    
    var description: String {
        switch self {
        case .unicode:
            return "Modern Unicode standard, compatible with all applications."
        case .tcvn3:
            return "Legacy TCVN3 encoding for older Vietnamese applications."
        case .vni:
            return "VNI Windows encoding for legacy software compatibility."
        case .cp1258:
            return "Windows-1258 code page for Windows applications."
        }
    }
}
```

## Testing Strategy

### Unit Tests (15+ tests)
1. **RustBridge tests**:
   - `testSetShortcutsEnabled()`
   - `testSetEncoding()`
   - `testGetEncoding()`
   - `testExportShortcutsJSON()`
2. **SettingsManager tests**:
   - `testTextExpansionEnabledPersistence()`
   - `testOutputEncodingPersistence()`
   - `testShiftBackspacePersistence()`
   - `testSettingsNotification()`
3. **InputManager tests**:
   - `testShortcutExpansion()`
   - `testEncodingApplied()`
   - `testShiftBackspaceDeletesWord()`

### Manual Testing Checklist
- [ ] Open Settings → Text Expansion tab
- [ ] Toggle enable, verify persists across restart
- [ ] Add shortcut "tt" → "thân thiện", test in app
- [ ] Edit shortcut, verify changes apply
- [ ] Delete shortcut, verify removed
- [ ] Export JSON, verify file format
- [ ] Import JSON, verify shortcuts loaded
- [ ] Select TCVN3 encoding, type Vietnamese, verify output
- [ ] Enable Shift+Backspace, press it, verify word deleted
- [ ] Disable Shift+Backspace, verify normal backspace behavior
- [ ] Test all features with different apps (per-app mode)
- [ ] Check memory usage (Instruments)
- [ ] Check performance (<1ms overhead)

### Integration Testing
- Settings UI → SettingsManager → RustBridge → Core → Output
- Verify each layer correctly propagates changes
- Test error cases (invalid JSON, encoding failure)

## Risk Mitigation

### Risks
1. **FFI calls crash or fail**
   - Mitigation: Comprehensive error handling in RustBridgeSafe
   - Test FFI boundaries thoroughly
   - Graceful fallback if core fails

2. **UI complexity overwhelms users**
   - Mitigation: Follow macOS HIG
   - Simple, clear labels and descriptions
   - Tooltips and helper text
   - Hide advanced options by default

3. **Performance degradation**
   - Mitigation: Benchmark before/after
   - Optimize settings lookup (cache)
   - Lazy load shortcuts list

4. **Settings don't persist**
   - Mitigation: Test UserDefaults thoroughly
   - Add logging for debug
   - Handle migration from old settings

5. **Encoding breaks existing apps**
   - Mitigation: Default to Unicode (safe)
   - Clear warnings for legacy encodings
   - Test with common apps (Word, Chrome, etc.)

### Rollback Plan
- Keep Phase 2.8 code functional
- Git branch allows easy rollback
- Can disable features via SettingsManager if bugs found
- UserDefaults backward compatible (old keys still work)

## Success Criteria

### Must Have ✅
- [ ] User can enable/disable Text Expansion in Settings
- [ ] User can add/edit/delete shortcuts
- [ ] User can import/export shortcuts JSON
- [ ] User can select output encoding (4 options)
- [ ] User can enable/disable Shift+Backspace
- [ ] All settings persist across app restart
- [ ] All features work end-to-end (Settings → Core → Output)
- [ ] Zero crashes, zero memory leaks
- [ ] All 15+ tests passing
- [ ] Performance: <1ms overhead for settings check
- [ ] Documentation complete

### Nice to Have ⭐
- Real-time preview of encoding
- Shortcut conflict detection
- Advanced Text Expansion features (variables, multi-line)
- Cloud sync for shortcuts (defer to Phase 4)

### Out of Scope 🚫
- Advanced animations/transitions (defer to Phase 3)
- NavigationSplitView refactor (defer to Phase 3)
- iOS/Windows implementation (separate phases)

## Dependencies

### Must Complete First
- ✅ Phase 1: Core Engine (Text Expansion, Encoding, Shift+Backspace)
- ✅ Phase 2 Core: UI Components (SettingsManager, RustBridgeSafe)

### External Dependencies
- Xcode 15+ for SwiftUI features
- macOS 13+ for some APIs
- Rust core with FFI functions (already done)

## Documentation Updates

### During Implementation
- Add inline code comments
- Update RustBridge function documentation
- Document SettingsManager new properties

### After Completion
- Update `.docs/features/platform/macos/settings_features.md`
- Create user guide for Text Expansion
- Create user guide for Encoding selection
- Screenshot all UI changes
- Update README with new features
- Update `.planning/STATE.md` to Phase 2.9 complete

## Rollout Plan

### Week 1: Core Integration (Milestones 2.9.1-2.9.2)
- Day 1: RustBridge & SettingsManager extension
- Day 2-3: Text Expansion UI (full-featured)
- Deliverable: Text Expansion working end-to-end

### Week 2: Remaining Features & Testing (Milestones 2.9.3-2.9.5)
- Day 4: Encoding & Shift+Backspace UI
- Day 5: InputManager integration & testing
- Day 6: Documentation & final review
- Deliverable: All 3 features complete, documented, tested

## Review & Sign-off

After Milestone 2.9.5:
- [ ] All 5 milestones complete
- [ ] All tests passing (15+ unit + manual)
- [ ] Manual testing checklist complete
- [ ] Instruments profiling done (memory, performance)
- [ ] Documentation updated
- [ ] Code review passed
- [ ] STATE.md updated to Phase 2 Supplement 100%
- [ ] Ready for Phase 3 (Quality & Testing)

---

**Next Phase**: Phase 3 - Quality & Testing (comprehensive E2E testing, performance tuning)
