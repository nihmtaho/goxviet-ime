# Data Model: GoxViet Feature Gap (US1–US5)

**Phase**: 1 | **Date**: 2026-04-05 | **Plan**: [plan.md](plan.md)

---

## EngineConfig Extensions

**File**: `core/src/application/dto/engine_config.rs`

Four new boolean fields added to the existing `EngineConfig` struct:

| Field | Type | Default | Governs |
|-------|------|---------|---------|
| `bracket_shortcuts_enabled` | `bool` | `false` | US2: `[`→ơ, `]`→ư in Telex |
| `foreign_consonants_enabled` | `bool` | `false` | US3: z/w/j/f as valid initials |
| `auto_capitalise_enabled` | `bool` | `false` | US4: capitalise after sentence end |
| `word_history_enabled` | `bool` | `false` | US5: backspace-after-space restore |

Note: `esc_restore_enabled` already exists (line 70); no change needed.

---

## FfiConfig_v2 Extensions

**File**: `core/src/presentation/ffi/types.rs`

Mirror of EngineConfig additions at the FFI boundary:

```c
// Additions to FfiConfig_v2 struct (C-compatible layout, repr(C))
bool bracket_shortcuts_enabled;   // default: false
bool foreign_consonants_enabled;  // default: false
bool auto_capitalise_enabled;     // default: false
bool word_history_enabled;        // default: false
```

All fields map 1:1 to `EngineConfig` fields via the existing config conversion path in
`presentation/ffi/api.rs`.

---

## Engine State Extensions

**File**: `core/src/infrastructure/engine/state/` (existing module)

### SentenceBoundaryState (new, for US4)

A single `bool` field added to the existing engine state struct:

| Field | Type | Set to true when | Set to false when |
|-------|------|-----------------|------------------|
| `at_sentence_boundary` | `bool` | Space/Enter/`!`/`?` processed AND last word is not abbreviation/decimal | Next key processed AND capitalised (or non-letter key) |

No separate struct needed; integrates directly into the engine's existing state tracking.

### WordHistory (existing, modified for US5)

**File**: `core/src/infrastructure/engine/state/history.rs`

| Attribute | Current | Required |
|-----------|---------|----------|
| `HISTORY_CAPACITY` | 3 | 10 |
| Entry shape | `(Buffer, RawInputBuffer)` | No change |
| Push trigger | Space key | No change |
| Pop trigger | Backspace at word-start | No change |
| Invalidation trigger | Missing | Add: any non-Backspace key after Space |
| Gate | Always active | Gate behind `word_history_enabled` config |

State transitions:

```
IDLE ──[Space]──► COMMITTED(word) ──[Backspace]──► RESTORED(word) ──[any key]──► IDLE
                       │
                       └──[non-Backspace key]──► INVALIDATED ──[any key]──► IDLE
```

Clearing rules (FR-009):
- IME toggled off → `history.clear()`
- Focused app changes → `history.clear()` (called from Swift via `ime_reset_v2` or equivalent)
- Buffer position invalidated → mark last entry as non-restorable (not full clear)

---

## Static Data: Abbreviation List (US4)

**File**: `core/src/data/auto_capitalise.rs` (new)

Stored as a sorted `&'static [&'static str]` for binary search O(log n):

```rust
pub static ABBREVIATION_LIST: &[&str] = &[
    "bs.", "đ.", "gs.", "no.", "pgs.", "pgsts.", "ths.", "tp.", "tr.", "ts.", "v.d.", "v.v.",
];
```

All entries lowercase. Detection: lowercase the last confirmed word fragment before the period,
then binary-search in `ABBREVIATION_LIST`. Decimal detection: check if the character
immediately before `.` is `b'0'..=b'9'`.

---

## BracketShortcutConfig (US2)

No separate struct required. The mapping is a constant lookup:

| Input Key | Output Character | Unicode |
|-----------|-----------------|---------|
| `[` (keycode 33) | `ơ` | U+01A1 |
| `]` (keycode 30) | `ư` | U+01B0 |

Scoped to Telex mode only; VNI adapter is unmodified.

---

## ForeignConsonantSet (US3)

No separate struct required. The set is the constant `{z, w, j, f}` (ASCII), checked inline.

`w` position rule:
- Buffer empty (word-start) → treat `w` as literal foreign consonant
- Buffer non-empty and last char is a vowel → treat `w` as Telex horn modifier (existing path)
- Buffer non-empty and last char is not a vowel → treat `w` as literal (pass-through)
