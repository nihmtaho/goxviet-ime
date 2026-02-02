# Task: Text Expansion UI Tab Implementation

## Metadata
- **Title**: Text Expansion Settings Tab with Full CRUD
- **Author**: AI Assistant (Claude)
- **Created**: 2026-02-01
- **Estimated Effort**: 6-8 hours
- **Milestone**: 2.9.2 (Text Expansion UI)
- **Branch**: `feature/text-expansion-ui-tab`
- **Status**: In Progress

## Goal
Create a dedicated "Text Expansion" tab in GoxViet Settings that allows users to:
- View all shortcuts in a searchable list
- Add/Edit/Delete shortcuts with validation
- Import/Export shortcuts as JSON
- Enable/disable text expansion feature globally
- Persist settings across app restarts

## Background
- **Phase 1 (Milestone 1.1)**: ✅ Core engine text expansion implemented
- **FFI Functions**: ✅ All required FFI functions exported (ime_export_shortcuts_json, ime_import_shortcuts_json, etc.)
- **Current Gap**: ❌ NO UI for users to manage shortcuts
- **Current State**: Phase 2 Supplement (Architecture Migration complete, awaiting Xcode integration)

## Acceptance Criteria

### Functional Requirements
- [ ] **Add Shortcut**: Users can create new shortcuts with trigger and replacement text
- [ ] **Edit Shortcut**: Users can modify existing shortcuts inline or via modal
- [ ] **Delete Shortcut**: Users can remove shortcuts via swipe gesture or button
- [ ] **Search/Filter**: Users can search shortcuts by trigger or replacement text
- [ ] **Import JSON**: Users can import shortcuts from a JSON file (NSOpenPanel)
- [ ] **Export JSON**: Users can export all shortcuts to a JSON file (NSSavePanel)
- [ ] **Enable/Disable**: Global toggle for text expansion feature
- [ ] **Persistence**: All shortcuts and settings persist across app restart
- [ ] **Validation**: Prevent empty triggers, duplicates, and invalid characters
- [ ] **Preview**: Show live preview of shortcut expansion

### Non-Functional Requirements
- [ ] **UI Style**: Matches existing settings views (GeneralSettingsView pattern)
- [ ] **HIG Compliance**: Follows macOS Human Interface Guidelines
- [ ] **Performance**: List renders smoothly with 100+ shortcuts (< 16ms)
- [ ] **Accessibility**: Full VoiceOver support
- [ ] **Memory**: No leaks when adding/removing shortcuts

## Files to Change

### New Files
1. `platforms/macos/goxviet/goxviet/UI/Settings/TextExpansionSettingsView.swift` (~500 lines)
   - Main settings view with list, search, toolbar
   - Integrates with SettingsManager
   
2. `platforms/macos/goxviet/goxviet/UI/Settings/Components/ShortcutEditorSheet.swift` (~350 lines)
   - Modal sheet for add/edit operations
   - Validation logic and preview

3. `platforms/macos/goxviet/goxvietTests/TextExpansionUITests.swift` (~200 lines)
   - Unit tests for CRUD operations
   - Validation tests
   - Import/Export tests

### Modified Files
1. `platforms/macos/goxviet/goxviet/UI/Settings/SettingsWindowCoordinator.swift` (+30 lines)
   - Add TextExpansionSettingsTab to TabView
   
2. `platforms/macos/goxviet/goxviet/Core/SettingsManager.swift` (+50 lines if needed)
   - Ensure textExpansionEnabled property exists
   - Wire UserDefaults persistence
   
3. `platforms/macos/goxviet/goxviet/InputManager.swift` (+20 lines if needed)
   - Ensure observation of textExpansionEnabled
   - Call RustBridge.setShortcutsEnabled()

4. `platforms/macos/goxviet/goxviet/RustBridge.swift` (verify existing)
   - Check if text expansion FFI functions already wrapped

## Architecture

### Data Flow
```
User Action (Add/Edit/Delete)
  ↓
TextExpansionSettingsView
  ↓
SettingsManager.textExpansionEnabled (Published)
  ↓
UserDefaults.standard (Persistence)
  ↓
NotificationCenter (textExpansionChanged)
  ↓
InputManager (Observer)
  ↓
RustBridge.setShortcutsEnabled(bool)
  ↓
FFI: ime_set_shortcuts_enabled()
  ↓
Rust Core Engine
```

### Component Structure
```
TextExpansionSettingsView
├── Header (Title + Description)
├── Toggle (Enable/Disable)
├── Toolbar
│   ├── Search Field
│   ├── Add Button
│   ├── Import Button
│   └── Export Button
├── Shortcut List (ScrollView)
│   └── ShortcutRow (trigger → replacement)
│       ├── Edit button
│       └── Delete button (swipe gesture)
└── Footer (Count + Capacity info)

ShortcutEditorSheet (Modal)
├── Trigger TextField (validation)
├── Replacement TextField
├── Preview Area
├── Cancel Button
└── Save Button
```

## Implementation Steps

### Step 1: Create TextExpansionSettingsView.swift
- Copy pattern from GeneralSettingsView/AdvancedSettingsView
- Header with title and description
- GroupBox with Enable toggle
- Search field in toolbar
- List with ForEach over shortcuts
- Add/Import/Export buttons
- Wire to SettingsManager

### Step 2: Create ShortcutEditorSheet.swift
- Modal sheet with @State for trigger/replacement
- TextFields with validation
- Preview area showing expansion result
- Cancel/Save buttons
- Error alerts for validation failures

### Step 3: Implement Import/Export
- NSOpenPanel for import (filter: .json)
- Parse JSON and call RustBridge.importShortcuts()
- NSSavePanel for export (default: shortcuts.json)
- Call RustBridge.exportShortcuts() and write to file

### Step 4: Integrate into SettingsWindowCoordinator
- Add TextExpansionSettingsTab struct
- Wire to TabView after AboutSettingsTab
- Tab icon: "text.badge.plus" or "character.textbox"

### Step 5: Wire SettingsManager
- Check if textExpansionEnabled property exists
- If not, add @Published var with UserDefaults backing
- Sync with RustBridge on change

### Step 6: Update InputManager
- Verify observation of textExpansionChanged notification
- Call RustBridge.setShortcutsEnabled() on change

### Step 7: Add Unit Tests
- Test add/edit/delete operations
- Test validation (empty, duplicates, invalid chars)
- Test import/export JSON
- Mock RustBridge if needed

### Step 8: Manual Testing
- Add 10 shortcuts via UI
- Test search/filter
- Export JSON, verify format
- Import JSON, verify shortcuts loaded
- Type trigger + space, verify expansion
- Restart app, verify persistence
- Test with 100+ shortcuts (performance)

## Performance Targets
- List rendering: < 16ms for 100 shortcuts
- Search/filter: < 50ms for 1000 shortcuts
- Import/Export: < 100ms for 1000 shortcuts
- Memory: < 5MB for shortcut management
- No leaks when adding/removing shortcuts repeatedly

## Testing Checklist

### Unit Tests (goxvietTests)
- [ ] Test shortcut creation with valid data
- [ ] Test shortcut creation with invalid data (empty, duplicates)
- [ ] Test shortcut editing
- [ ] Test shortcut deletion
- [ ] Test search/filter functionality
- [ ] Test JSON import (valid data)
- [ ] Test JSON import (invalid data)
- [ ] Test JSON export
- [ ] Test enable/disable toggle
- [ ] Test persistence to UserDefaults

### Integration Tests
- [ ] Test RustBridge FFI calls (mock if needed)
- [ ] Test SettingsManager → InputManager flow
- [ ] Test InputManager → RustBridge flow

### Manual Tests
- [ ] Add shortcut via UI → appears in list
- [ ] Edit shortcut → changes reflected
- [ ] Delete shortcut → removed from list
- [ ] Search shortcut → filters correctly
- [ ] Import JSON → shortcuts loaded
- [ ] Export JSON → file created with valid JSON
- [ ] Type "tt" + space → expands to "thân thiện"
- [ ] Disable text expansion → shortcuts don't expand
- [ ] Enable text expansion → shortcuts expand again
- [ ] Restart app → shortcuts persisted
- [ ] Add 100 shortcuts → list performs well
- [ ] VoiceOver navigation → accessible

### Performance Tests
- [ ] Instruments memory profiling (no leaks)
- [ ] List scroll with 100+ shortcuts (< 16ms frame time)
- [ ] Search with 1000 shortcuts (< 50ms)

## Dependencies / Blockers
- None (Rust core and FFI already complete)
- Milestone 2.5 (Xcode integration) must be complete before building in Xcode

## Rollback Plan
If issues arise:
1. Remove new tab from SettingsWindowCoordinator
2. Keep new files but don't integrate
3. Revert SettingsManager/InputManager changes if needed
4. Core engine unaffected (feature already exists)

## Documentation Updates
- [ ] Update `.docs/features/platform/macos/settings_features.md`
  - Add "Text Expansion" section with screenshots
- [ ] Create user guide `.docs/guides/text-expansion-guide.md`
  - How to add shortcuts
  - How to import/export
  - Common use cases
  - Troubleshooting
- [ ] Update `docs/SHORTCUTS.md` if needed

## Review Requirements
- **Reviewer**: macOS platform developer with SwiftUI experience
- **Focus Areas**:
  - UI/UX follows macOS HIG
  - Code follows existing patterns (SettingsManager, TypedNotifications)
  - Memory safety (no leaks)
  - Accessibility (VoiceOver)
  - Performance (list rendering)

## Success Metrics
- ✅ All acceptance criteria met
- ✅ All unit tests passing
- ✅ Manual testing checklist complete
- ✅ Performance targets met
- ✅ Documentation updated
- ✅ Zero crashes/memory leaks
- ✅ Code review approved

## Timeline
- **Day 1 (4-5 hours)**:
  - Create TextExpansionSettingsView (2h)
  - Create ShortcutEditorSheet (1.5h)
  - Integrate into Settings (0.5h)
  
- **Day 2 (2-3 hours)**:
  - Implement Import/Export (1h)
  - Wire SettingsManager/InputManager (0.5h)
  - Add unit tests (1h)
  - Manual testing (0.5-1h)
  
- **Day 3 (1-2 hours)**:
  - Documentation (1h)
  - Review summary (0.5h)
  - Bug fixes from testing (0.5h)

## Notes
- Follow existing UI patterns from GeneralSettingsView for consistency
- Use GroupBox for sections like other settings views
- Reuse Components like SettingRow, ToggleRow if applicable
- Consider adding "Common Shortcuts" preset templates (optional enhancement)
- Phase 2 pattern: Use SettingsManager instead of @AppStorage for state
- Use TypedNotificationCenter for notifications
- Follow Swift 6 concurrency rules (MainActor for UI updates)

## References
- [MILESTONE_1.1.md](.planning/milestones/MILESTONE_1.1.md) - Core implementation
- [PHASE2.9_FEATURE_INTEGRATION.md](.planning/phases/PHASE2.9_FEATURE_INTEGRATION.md) - Integration plan
- [macos-development skill](.claude/skills/macos-development/SKILL.md) - SwiftUI best practices
- [rust-core skill](.claude/skills/rust-core/SKILL.md) - FFI guidance
- Existing settings views for UI patterns
