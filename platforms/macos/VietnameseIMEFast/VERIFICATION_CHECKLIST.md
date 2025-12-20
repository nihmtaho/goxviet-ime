# Verification Checklist - GoNhanh Mechanisms Integration

## ✅ Tổng quan

Checklist này giúp verify rằng tất cả các cơ chế từ GoNhanh đã được tích hợp đúng và hoạt động như mong đợi.

---

## 🏗️ PHASE 1: Build & Setup

### Rust Core
- [ ] `cargo build --release` thành công
- [ ] Library file tồn tại: `core/target/release/libvietnamese_ime_core.a`
- [ ] Không có error trong build output
- [ ] Chỉ có warnings về unused variables (stub implementations)

### Xcode Project
- [ ] Project mở được trong Xcode
- [ ] Bridging Header có đầy đủ 15 FFI functions
- [ ] Library Search Path trỏ đúng: `$(PROJECT_DIR)/../../../core/target/release`
- [ ] Build thành công (Cmd+B)
- [ ] Không có Swift compilation errors

### Files Added/Modified
- [x] `RustBridge.swift` - Created (728 lines)
- [x] `InputManager.swift` - Refactored to use RustBridge
- [x] `AppDelegate.swift` - Enhanced with menu & state management
- [x] `VietnameseIMEFast-Bridging-Header.h` - Extended with 12 new functions
- [x] `core/src/lib.rs` - Added 8 FFI stub implementations

---

## 🔍 PHASE 2: Components Verification

### 1. Log System (`Log`)
- [ ] Log.logPath trỏ đến `/tmp/vietnameseime.log`
- [ ] Log.isEnabled = true trong DEBUG mode
- [ ] Log.write() tạo/append file thành công
- [ ] Log file readable và có format đúng: `[timestamp] message`

**Test:**
```swift
Log.isEnabled = true
Log.info("Test message")
// Check: cat /tmp/vietnameseime.log
```

### 2. KeyCode Constants
- [ ] KeyCode.backspace = 51
- [ ] KeyCode.forwardDelete = 117
- [ ] KeyCode.leftArrow = 123
- [ ] KeyCode.escape = 53

### 3. Event Marker System
- [ ] kEventMarker = 0x564E5F494D45 ("VN_IME")
- [ ] Injected events có marker set
- [ ] Event handler kiểm tra marker và skip nếu match
- [ ] Không có infinite loop khi gõ

**Test:** Type "aa" nhanh, check log không có duplicate processing

### 4. TextInjector
- [ ] TextInjector.shared singleton tồn tại
- [ ] Semaphore prevents concurrent injection
- [ ] injectViaBackspace() hoạt động
- [ ] injectViaSelection() hoạt động
- [ ] injectViaAutocomplete() hoạt động

**Test per method:**
- Fast: Type in TextEdit
- Selection: Type in Safari address bar
- Autocomplete: Type in Spotlight
- Slow: Type in Terminal

### 5. App Detection (`detectMethod()`)
- [ ] Phát hiện AXComboBox → selection
- [ ] Phát hiện AXSearchField → selection
- [ ] Spotlight → autocomplete
- [ ] Chrome/Safari address bar → selection
- [ ] VSCode → slow
- [ ] Terminal → slow
- [ ] Microsoft Office → slow
- [ ] Default apps → fast

**Test:** Enable logging, switch apps, verify Log.method() output

### 6. RustBridge Class
- [ ] RustBridge() initializes
- [ ] initialize() chạy thành công
- [ ] setMethod() calls (stub)
- [ ] setEnabled() calls (stub)
- [ ] clearBuffer() calls
- [ ] All shortcut methods defined

**Test:**
```swift
let bridge = RustBridge()
bridge.initialize()
bridge.setMethod(0)
bridge.setEnabled(true)
// No crashes
```

### 7. KeyboardHookManager
- [ ] KeyboardHookManager.shared singleton
- [ ] start() tạo event tap
- [ ] Accessibility permission check hoạt động
- [ ] showAccessibilityAlert() hiển thị đúng dialog
- [ ] stop() cleanup event tap
- [ ] Không leak CFMachPort

**Test:** Launch app, check permission prompt, grant permission, verify event tap active

### 8. PerAppModeManager
- [ ] PerAppModeManager.shared singleton
- [ ] start() registers workspace observer
- [ ] handleAppSwitch() được gọi khi switch app
- [ ] appStates dictionary lưu trạng thái
- [ ] setStateForCurrentApp() hoạt động
- [ ] stop() removes observer

**Test:** Enable IME, switch to another app, disable IME, switch back, verify state restored

### 9. Word Restoration
- [ ] getWordToRestoreOnBackspace() định nghĩa
- [ ] Lấy được focused element
- [ ] Lấy được selected text range
- [ ] Trích xuất được last word
- [ ] Return nil khi không có word

**Test:** Type "hello", press ESC (stub returns false, but function exists)

### 10. CGEventFlags Extension
- [ ] flags.modifierCount property tồn tại
- [ ] Đếm đúng số modifiers
- [ ] Cmd = 1, Cmd+Shift = 2, etc.

**Test:**
```swift
let flags: CGEventFlags = [.maskCommand, .maskShift]
XCTAssertEqual(flags.modifierCount, 2)
```

### 11. Shortcut Recording
- [ ] startShortcutRecording() defined
- [ ] stopShortcutRecording() defined
- [ ] setupShortcutObserver() defined
- [ ] matchesToggleShortcut() defined
- [ ] matchesModifierOnlyShortcut() defined

### 12. Custom Notifications
- [ ] .toggleVietnamese defined
- [ ] .showUpdateWindow defined
- [ ] .shortcutChanged defined
- [ ] .updateStateChanged defined
- [ ] .shortcutRecorded defined
- [ ] .shortcutRecordingCancelled defined

**Test:** Post notification, verify observer receives it

---

## 🎯 PHASE 3: InputManager Integration

### Event Handling
- [ ] handleEvent() checks event marker first
- [ ] Ignores injected events (marker match)
- [ ] Processes keyDown events
- [ ] Handles flagsChanged events
- [ ] Detects toggle shortcut
- [ ] Respects isEnabled state

### Composition Tracking
- [ ] currentCompositionLength initialized to 0
- [ ] Increments on Vietnamese output
- [ ] Decrements on backspace
- [ ] Resets on navigation keys
- [ ] Resets on Cmd/Ctrl/Opt shortcuts

### Special Key Handling
- [ ] ESC triggers word restoration (interface)
- [ ] Navigation keys (arrows, Enter, Tab) clear buffer
- [ ] Backspace handled correctly
- [ ] Forward Delete clears composition

### State Management
- [ ] setEnabled() updates isEnabled
- [ ] toggleEnabled() flips state
- [ ] setInputMethod() calls ime_set_method
- [ ] setModernToneStyle() calls ime_set_modern_tone
- [ ] reloadShortcuts() calls ime_clear_shortcuts

### Rust Engine Integration
- [ ] engine created with ime_create()
- [ ] engine destroyed in deinit with ime_destroy()
- [ ] processKeyWithEngine() calls ime_process_key
- [ ] Result string decoded correctly
- [ ] Memory freed with ime_free_string

---

## 🖥️ PHASE 4: AppDelegate Features

### Menu Bar
- [ ] Status item created
- [ ] Icon shows 🇻🇳 when enabled
- [ ] Icon shows EN when disabled
- [ ] Tooltip displays current state

### Menu Items
- [ ] "Vietnamese Input" toggle with checkmark
- [ ] "Input Method" submenu (Telex/VNI)
- [ ] "Tone Style" submenu (Modern/Traditional)
- [ ] "Settings..." placeholder
- [ ] "View Log..." (DEBUG only)
- [ ] "About VietnameseIMEFast"
- [ ] "Quit" with Cmd+Q

### Actions
- [ ] toggleVietnamese() flips state
- [ ] selectTelex() sets method to 0
- [ ] selectVNI() sets method to 1
- [ ] selectModernTone() sets modern to true
- [ ] selectOldTone() sets modern to false
- [ ] showSettings() displays alert
- [ ] viewLog() opens log file
- [ ] showAbout() displays info

### Observers
- [ ] .updateStateChanged updates UI
- [ ] .toggleVietnamese updates state
- [ ] Menu checkmarks update correctly

---

## 🧪 PHASE 5: Runtime Testing

### Basic Input
- [ ] Launch app successfully
- [ ] Type English: passes through
- [ ] Type "a" "a": produces "â"
- [ ] Type "a" "w": produces "ă"
- [ ] Type "o" "o": produces "ô"
- [ ] Type "o" "w": produces "ơ"
- [ ] Type "u" "w": produces "ư"
- [ ] Type "d" "d": produces "đ"

### Tone Marks (Telex)
- [ ] "s" adds sắc
- [ ] "f" adds huyền
- [ ] "r" adds hỏi
- [ ] "x" adds ngã
- [ ] "j" adds nặng
- [ ] "z" removes tone

### Toggle Functionality
- [ ] Click menu item toggles state
- [ ] Status icon updates immediately
- [ ] Input disabled when state = OFF
- [ ] Input enabled when state = ON

### Method Switching
- [ ] Switch to VNI in menu
- [ ] Type "a" "1": produces "á" (VNI style)
- [ ] Switch back to Telex
- [ ] Type "a" "s": produces "á" (Telex style)

### App Detection
- [ ] Open TextEdit: Log shows "fast"
- [ ] Open Safari address bar: Log shows "selection"
- [ ] Open Spotlight: Log shows "autocomplete"
- [ ] Open Terminal: Log shows "slow"
- [ ] Open VSCode: Log shows "slow"

### Per-App State
- [ ] Enable IME in TextEdit
- [ ] Switch to Chrome, disable IME
- [ ] Switch back to TextEdit: IME still enabled
- [ ] Switch back to Chrome: IME still disabled

### Composition Management
- [ ] Type "h" "o" "a": composition length = 3
- [ ] Press backspace: composition length = 2
- [ ] Press arrow key: composition length = 0
- [ ] Press Cmd+C: composition length = 0

### Edge Cases
- [ ] Rapid typing: no lag
- [ ] Paste (Cmd+V): no interference
- [ ] Undo (Cmd+Z): works normally
- [ ] Switch app mid-composition: no crash
- [ ] Multiple backspaces: no negative length

---

## 📊 PHASE 6: Performance Verification

### Latency
- [ ] Key to display < 20ms (subjective test)
- [ ] No visible lag in TextEdit
- [ ] Acceptable lag in Terminal/VSCode
- [ ] Smooth in browser address bars

### Memory
- [ ] Launch memory < 20MB
- [ ] Memory stable after 1000 keystrokes
- [ ] No memory leaks (Instruments)
- [ ] Log file size reasonable

### CPU
- [ ] Idle CPU < 1%
- [ ] Typing CPU spike < 10%
- [ ] No CPU spin

---

## 🐛 PHASE 7: Error Handling

### Accessibility Permission
- [ ] Denied: Shows alert with link to settings
- [ ] Granted: Event tap starts successfully
- [ ] Revoked at runtime: App detects and prompts

### Rust Engine
- [ ] NULL engine pointer: Returns NULL (no crash)
- [ ] Invalid UTF-32: Returns NULL (no crash)
- [ ] Panic in Rust: Caught, returns NULL (no crash)

### Text Injection
- [ ] NULL CGEventSource: Returns early (no crash)
- [ ] Empty text: No-op (no crash)
- [ ] Very long text: Chunks correctly

### App Detection
- [ ] Unknown app: Falls back to fast method
- [ ] NULL bundle ID: Falls back to fast method
- [ ] Nil focused element: Falls back to fast method

---

## 📝 PHASE 8: Code Quality

### Swift Code
- [ ] No force unwraps (!)
- [ ] Guard/if let for optionals
- [ ] Proper memory management (weak self in closures)
- [ ] No retain cycles
- [ ] Descriptive variable names

### Rust Code
- [ ] All FFI functions marked unsafe extern "C"
- [ ] Null pointer checks
- [ ] Panic boundaries with catch_unwind
- [ ] Proper CString handling
- [ ] Memory freed correctly

### Documentation
- [ ] INTEGRATION_NOTES.md comprehensive
- [ ] Code comments for complex logic
- [ ] FFI functions documented
- [ ] Bridging header has descriptions

---

## 🎓 PHASE 9: Comparison with GoNhanh

### Feature Parity
- [x] Log system
- [x] Event marker
- [x] TextInjector with 4 strategies
- [x] App detection logic
- [x] RustBridge interface
- [x] KeyboardHookManager
- [x] PerAppModeManager
- [x] Word restoration (interface)
- [x] Shortcut management (interface)
- [x] Custom notifications
- [x] Menu bar integration
- [x] Toggle functionality

### Improvements Over GoNhanh
- [ ] Better error handling
- [ ] More descriptive logging
- [ ] Cleaner separation of concerns
- [ ] Extended FFI interface
- [ ] Type-safe Swift code

---

## ✅ Final Sign-Off

### Pre-Release Checklist
- [ ] All PHASE 1-7 items checked
- [ ] No outstanding crashes
- [ ] Performance acceptable
- [ ] Memory leaks resolved
- [ ] Code reviewed
- [ ] Documentation complete

### Known Limitations
- [ ] Rust engine config functions are stubs (to be implemented)
- [ ] Settings UI is placeholder
- [ ] Shortcut recording not implemented
- [ ] Update checker not implemented
- [ ] No candidate window

### Next Steps
1. Implement config functions in Rust core
2. Add Settings window (SwiftUI)
3. Implement shortcut recording
4. Add candidate window for ambiguous input
5. Add update checker
6. Beta testing with users
7. Performance profiling
8. Release v1.0.0

---

## 📅 Sign-Off

**Date:** _________________

**Tested By:** _________________

**Verified By:** _________________

**Status:** 
- [ ] ✅ Ready for Beta
- [ ] ⚠️ Needs Work
- [ ] ❌ Blocked

**Notes:**
_____________________________________________________________
_____________________________________________________________
_____________________________________________________________