# macOS Platform Architecture

The macOS platform implementation of GoxViet is a **hybrid application** combining a high-performance Rust core with a native Swift/Cocoa frontend.

## High-Level Architecture

```
User Keyboard Input
        │
        ▼
CGEventTap (system-wide intercept)
        │
        ▼
┌─────────────────────────────────────────────────────────┐
│  Swift Platform Layer                                   │
│                                                         │
│  InputManager (singleton)                               │
│    │                                                     │
│    ├── PerAppModeManagerEnhanced  (Smart Mode check)    │
│    ├── RustBridgeV2 / RustEngineV2  (FFI v2 wrapper)   │
│    └── TextInjectionHelper  (text output)               │
│                                                         │
│  AppState / SettingsManager  (settings, UserDefaults)   │
│  NotificationCenter  (reactive settings broadcast)      │
└─────────────────────────────────────────────────────────┘
        │  FFI v2 (out-parameter pattern)
        ▼
┌─────────────────────────────────────────────────────────┐
│  Rust Core Engine (libgoxviet_core.a)                   │
│  presentation/ffi/api.rs                                │
│    → application / domain / infrastructure              │
└─────────────────────────────────────────────────────────┘
        │
        ▼
Target Application (text field)
```

## Key Components

### App Entry (`App/`)

| File | Role |
|---|---|
| `AppDelegate.swift` | Status-bar app lifecycle, Accessibility permission check (`AXIsProcessTrusted`), `NSStatusItem` setup |
| `GoxVietApp.swift` | SwiftUI `@main` App definition |

### Core Layer (`Core/`)

| File | Role |
|---|---|
| `RustBridgeV2.swift` | Declares FFI v2 types (`FfiConfig_v2`, `FfiProcessResult_v2`, `FfiStatusCode`) and `@_silgen_name` function bindings. **The only place raw FFI symbols are declared.** |
| `RustEngineV2.swift` | Thread-safe Swift wrapper around `RustBridgeV2`. Owns the engine pointer, applies config, exposes `processKey(_:) -> ProcessResult` |
| `SettingsManager.swift` | Single source of truth for `UserDefaults`-backed settings. Syncs config changes to the Rust engine via `RustEngineV2` |
| `OutputEncoding.swift` | `OutputEncoding` enum (Unicode / VIQR / TCVN3) for encoding conversion |
| `TypedNotifications.swift` | Type-safe `NotificationCenter` wrappers for settings changes |
| `RustBridgeError.swift` | `RustBridgeError` enum for FFI error propagation |

### Managers (`Managers/`)

| File | Role |
|---|---|
| `Input/InputManager.swift` | **Singleton CGEventTap event loop.** Highest-risk file — intercepts all keystrokes, dispatches to `RustEngineV2`, applies backspaces + text injection. |
| `Injection/TextInjectionHelper.swift` | Injects backspace events + text via `CGEvent` posts |
| `PerAppModeManagerEnhanced.swift` | Per-application Smart Mode config stored in `UserDefaults` |
| `ResourceManager.swift` | Asset / resource loading helpers |
| `Update/UpdateManager.swift` | Auto-update coordinator |
| `Update/UpdateChecker.swift` | Version check against GitHub releases |
| `WindowManager.swift` | Settings window presentation (`NSWindow`) |

### Models (`Models/`)

| File | Role |
|---|---|
| `KeyboardShortcut.swift` | Toggle shortcut model (key + modifiers) |
| `RestoreShortcut.swift` | Restore-word shortcut model |
| `LRUCache.swift` | Generic LRU cache (used for character pool) |

### Services (`Services/`)

| File | Role |
|---|---|
| `InputSourceMonitor.swift` | Monitors macOS input source changes |
| `Log.swift` | Structured logging to `~/Library/Logs/GoxViet/` |

### UI (`UI/`)

| Path | Role |
|---|---|
| `UI/Settings/SettingsRootView.swift` | Root SwiftUI settings window (Glass style) |
| `UI/Settings/GeneralSettingsView.swift` | Input method, tone style settings |
| `UI/Settings/AdvancedSettingsView.swift` | Smart Mode, ESC restore, etc. |
| `UI/Settings/PerAppSettingsView.swift` | Per-app enable/disable table |
| `UI/Settings/TextExpansionSettingsView.swift` | Shortcut CRUD UI |
| `UI/MenuBar/MenuToggleView.swift` | Menu bar toggle item |
| `UI/MenuBar/SmartModeIndicator.swift` | Smart Mode status indicator |

### Utilities (`Utilities/`)

| File | Role |
|---|---|
| `HighPriorityKeyboardEventCapture.swift` | CGEventTap priority setup |
| `ActivationPolicyCoordinator.swift` | Switches between `accessory` and `regular` activation policy |
| `SpecialPanelAppDetector.swift` | Detects apps that need special handling (password fields, etc.) |
| `LifecycleManaged.swift` | Protocol for `start()`/`stop()` lifecycle |
| `KeyCodes.swift` | macOS virtual keycode constants and sets |

## Data Flow: Keystroke → Text Output

1. `CGEventTap` fires in `InputManager.handleEvent(_:)`
2. Self-generated events filtered by marker `0x564E5F494D45`
3. Break keys (space, arrows, punctuation) → commit word, pass through
4. Toggle shortcut? → toggle IME on/off
5. Smart Mode disabled for this app? → pass through
6. `RustEngineV2.processKey(char)` → `ime_process_key_v2(engine, key, &result)`
7. If `result.consumed`:
   - Post `backspace_count` backspace `CGEvent`s
   - Post synthetic key events for `result.text`
8. Free `result.text` via `ime_free_string_v2`

## Settings Sync Flow

```
SettingsManager.shared.inputMethod = .vni
    → builds FfiConfig_v2
    → RustEngineV2.applyConfig(config)
    → ime_set_config_v2(enginePtr, &config)
    → posts TypedNotification (SwiftUI reacts)
```

## Thread Safety

- `InputManager` runs on the CGEventTap callback thread (high priority).
- `RustEngineV2` serializes all calls under a `NSLock`.
- UI updates must be dispatched to `DispatchQueue.main`.
- Settings changes from UI call `SettingsManager` on main thread, which posts notification and calls `RustEngineV2.applyConfig` (thread-safe).
