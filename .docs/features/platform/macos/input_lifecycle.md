# Input Handling & Lifecycle

## Lifecycle Management (`Managers/Input/InputManager.swift`)

`InputManager` is a singleton that manages the active IME session. It implements the `LifecycleManaged` protocol (`start()` / `stop()`).

### Initialization

```swift
private init() {
    ime_init_v2()                          // Creates Rust engine (FFI v2)
    self.currentShortcut = KeyboardShortcut.load()
    loadSavedSettings()
    SettingsManager.shared.syncShortcutsToEngine()
    setupObservers()
}
```

### Start Sequence

1. `AppDelegate` checks Accessibility permissions (`AXIsProcessTrusted`).
2. `InputManager.shared.start()` is called.
3. Creates `CGEventTap` at `.cghidEventTap` / `.headInsertEventTap`.
4. Adds tap source to `CFRunLoop.current` (`.commonModes`).
5. Sets up `NSEvent` mouse monitor to reset engine state on click.

### Stop Sequence

1. Removes `CGEventTap` from run loop.
2. Disables event tap.
3. Removes mouse monitor.

---

## Event Loop (`handleEvent`)

Every system keystroke passes through `handleEvent`. Processing order:

1. **Self-filter**: Events injected by GoxViet carry marker `0x564E5F494D45` — ignored immediately.
2. **Modifier-only keys**: Pure Cmd/Ctrl/Opt pass through unchanged.
3. **Toggle shortcut**: Check against `currentShortcut` (default `Ctrl+Space`) → toggle IME on/off.
4. **Smart Mode check**: `PerAppModeManagerEnhanced` checks current frontmost app — if disabled, pass through.
5. **Break key check**: Space, Tab, Enter, arrows, punctuation, Shift+numbers → call `RustEngineV2.resetBuffer()`, pass through.
6. **Rust processing**: `RustEngineV2.processKey(char)` → `ime_process_key_v2(engine, key, &result)`.
7. **Apply result**:
   - If `consumed == false`: return event unmodified.
   - If `consumed == true`: suppress original event, apply backspaces + inject text.

---

## Text Injection (`Managers/Injection/TextInjectionHelper.swift`)

When the Rust core returns a transformation, `TextInjectionHelper` updates the target application.

### Action Types

| `consumed` | `backspace_count` | `text` | Action |
|---|---|---|---|
| `false` | — | — | Pass original event through |
| `true` | 0 | non-null | Insert text only (first char of word) |
| `true` | > 0 | non-null | Delete N chars, insert new text (mid-word transform) |
| `true` | > 0 | null/empty | Backspace only (soft delete) |

### Injection Mechanism

```swift
// 1. Post synthetic Backspace key events
for _ in 0..<result.backspaceCount {
    postBackspaceEvent()
}

// 2. Post synthetic key events for each Unicode scalar in result.text
for scalar in text.unicodeScalars {
    postUnicodeEvent(scalar)
}
```

Events are posted at `.cghidEventTap` with the self-marker to avoid re-processing.

### Backspace Handling

`Keycode 51` (Backspace) is intercepted:
- If engine buffer is empty → pass through to OS.
- If engine has content → call `RustEngineV2.processKey(backspace)` → get new buffer state → inject minimal diff.
- Rapid backspaces are coalesced to reduce flicker.

### Mouse Click Reset

An `NSEvent` monitor watches for `leftMouseDown` and `rightMouseDown`:
```swift
NSEvent.addGlobalMonitorForEvents(matching: [.leftMouseDown, .rightMouseDown]) { _ in
    RustEngineV2.shared.resetBuffer()  // clears engine state on focus change
}
```

---

## Per-App Smart Mode (`Managers/PerAppModeManagerEnhanced.swift`)

```swift
// Check before processing each key
let bundleID = NSWorkspace.shared.frontmostApplication?.bundleIdentifier ?? ""
if !perAppManager.isEnabled(for: bundleID) {
    return passThrough(event)
}
```

State stored in `UserDefaults` as a dictionary `[bundleID: Bool]`. Updated via `PerAppSettingsView`.

---

## App Lifecycle (`App/AppDelegate.swift`)

### Launch
- Registers `UserDefaults` defaults.
- Starts Accessibility permission polling timer (every 2s until granted).
- Initializes `NSStatusItem` (menu bar icon).
- Calls `InputManager.shared.start()` when permissions are available.

### Activation Policy
- `ActivationPolicyCoordinator` switches between `.accessory` (background, no Dock icon) and `.regular` (when Settings window is open).

### Termination
- `InputManager.shared.stop()` (removes CGEventTap).
- `NSStatusItem` is removed.

---

## Restore Shortcut

A configurable double-tap shortcut (default: double `Shift`) restores the last auto-transformed word to its original ASCII keystrokes.

```swift
// RestoreShortcut detection in handleEvent
if restoreShortcutEnabled {
    if detectDoubleTap(key: event.keyCode, flags: event.modifierFlags) {
        RustEngineV2.shared.triggerEscRestore()
        return nil  // consume
    }
}
```
