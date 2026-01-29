# Milestone 2.5: Xcode Project Integration - Task Checklist

## Pre-Integration
- [x] Backup current project state
- [x] Create implementation guide
- [ ] Open Xcode project

## Add Main App Files (13 files)

### UI Components (7 files)
- [ ] Add `GlassBackground.swift` to UI/Shared/
- [ ] Add `SettingRow.swift` to UI/Settings/Components/
- [ ] Add `MetricsChartView.swift` to UI/Settings/Components/
- [ ] Add `GeneralSettingsView.swift` to UI/Settings/
- [ ] Add `PerAppSettingsView.swift` to UI/Settings/
- [ ] Add `AdvancedSettingsView.swift` to UI/Settings/
- [ ] Add `AboutSettingsView.swift` to UI/Settings/

### Core Components (4 files)
- [ ] Add `RustBridgeError.swift` to Core/
- [ ] Add `RustBridgeSafe.swift` to Core/
- [ ] Add `SettingsManager.swift` to Core/
- [ ] Add `TypedNotifications.swift` to Core/

### Manager & MenuBar (2 files)
- [ ] Add `PerAppModeManagerEnhanced.swift` to Managers/
- [ ] Add `SmartModeIndicator.swift` to UI/MenuBar/

## Add Test Files (3 files)
- [ ] Add `RustBridgeSafeTests.swift` to goxvietTests/
- [ ] Add `SettingsManagerTests.swift` to goxvietTests/
- [ ] Add `PerAppModeManagerEnhancedTests.swift` to goxvietTests/

## Verification
- [ ] Check all files appear in Project Navigator
- [ ] Verify no red file references (broken links)
- [ ] Verify target membership:
  - [ ] Main app files → goxviet target
  - [ ] Test files → goxvietTests target
- [ ] Check deployment target (macOS 13+ for Charts)

## Build & Test
- [ ] Clean Build Folder (⌘+Shift+K)
- [ ] Build project (⌘+B)
- [ ] Fix any build errors
- [ ] Review and document warnings
- [ ] Run test suite (⌘+U)
- [ ] Verify 66+ tests can run

## Documentation
- [ ] Document any build issues encountered
- [ ] Document warnings and their resolution
- [ ] Update STATE.md with milestone completion
- [ ] Commit all changes

## Success Criteria
- [ ] Zero build errors
- [ ] All files added with correct target membership
- [ ] All tests executable (pass/fail doesn't matter yet)
- [ ] Project structure clean and organized

---

**Status:** In Progress  
**Started:** 2026-01-15  
**Expected Completion:** 2026-01-15
