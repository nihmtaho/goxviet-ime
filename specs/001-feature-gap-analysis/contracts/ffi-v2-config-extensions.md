# Contract: FFI v2 Config Extensions

**Feature**: GoxViet Feature Gap US1–US5
**Layer**: `presentation/ffi/` (Rust) ↔ `FFI/RustBridgeSafe.swift` (Swift)
**Date**: 2026-04-05

---

## Overview

All 5 user stories require new boolean configuration fields in `FfiConfig_v2`. No new FFI
functions are introduced — the existing `ime_create_engine_v2(config)` and the config update
path handle all new fields. This contract defines the additions to `FfiConfig_v2` and the
corresponding Swift `RustBridgeSafe` mapping.

---

## FfiConfig_v2 Additions (Rust, `types.rs`)

```c
// Existing fields (unchanged):
//   input_method: FfiInputMethod
//   tone_style: FfiToneStyle
//   smart_mode: bool
//   instant_restore_enabled: bool
//   esc_restore_enabled: bool          ← already present, just needs Swift UI
//   enable_shortcuts: bool

// NEW fields (appended, maintaining C-layout order):
bool bracket_shortcuts_enabled;    // US2 — default: false
bool foreign_consonants_enabled;   // US3 — default: false
bool auto_capitalise_enabled;      // US4 — default: false
bool word_history_enabled;         // US5 — default: false
```

**Layout rule**: New fields are appended after existing fields to maintain ABI compatibility
with any already-compiled callers. The `Default` impl must set all new fields to `false`.

---

## Swift Mapping (RustBridgeSafe.swift)

The Swift struct that mirrors `FfiConfig_v2` must be extended with the same 4 fields:

```swift
// Additions to the Swift config struct in RustBridgeSafe:
var bracketShortcutsEnabled: Bool = false
var foreignConsonantsEnabled: Bool = false
var autoCapitaliseEnabled: Bool = false
var wordHistoryEnabled: Bool = false
```

The `toFfi()` conversion method must map these fields to the C struct:

```swift
config.bracket_shortcuts_enabled = bracketShortcutsEnabled
config.foreign_consonants_enabled = foreignConsonantsEnabled
config.auto_capitalise_enabled = autoCapitaliseEnabled
config.word_history_enabled = wordHistoryEnabled
```

---

## Settings → Engine Config Flow

```
UserDefaults (SettingsManager)
    → GeneralSettingsView (SwiftUI binding)
    → AppState (source of truth)
    → RustBridgeSafe.updateConfig()
    → ime_create_engine_v2(FfiConfig_v2) or config reload path
    → EngineConfig (Rust domain)
```

Each new toggle in `GeneralSettingsView` binds to a `Bool` property in `AppState`, which is
persisted to `UserDefaults` under a namespaced key and used to build `FfiConfig_v2` on engine
construction or config update.

---

## Error Handling

No new error codes are needed. All new config fields are boolean and cannot fail validation.
The existing `FFI_STATUS_OK` / `FFI_STATUS_ERROR` codes remain unchanged.

---

## Invariants

- All new fields default to `false` (FR-010: all new toggles off by default).
- `bracket_shortcuts_enabled` is logically scoped to Telex mode; the engine ignores it when
  `input_method == VNI`.
- `foreign_consonants_enabled` affects both Telex and VNI `w`/`z`/`j`/`f` initial handling.
- `auto_capitalise_enabled` state (`at_sentence_boundary`) is reset when `ime_reset_v2` is
  called (app focus change, IME toggle).
- `word_history_enabled: false` MUST result in the `WordHistory` ring buffer never being
  written to or read from (zero overhead when disabled).
