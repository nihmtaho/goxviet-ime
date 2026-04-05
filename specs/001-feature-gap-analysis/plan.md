# Implementation Plan: GoxViet Feature Gap (US1–US5)

**Branch**: `001-feature-gap-analysis` | **Date**: 2026-04-05 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `specs/001-feature-gap-analysis/spec.md`

---

## Summary

Five macOS-only features derived from gap analysis against the Gõ Nhanh reference
implementation. All changes are confined to the Rust core (`core/`) and the macOS Swift
layer (`platforms/macos/`). No FFI API surface is changed — only `FfiConfig_v2` gains four
new boolean fields. US1 is UI-only. US2/US3 are adapter-level engine changes. US4 introduces
a new static data file and a single state flag. US5 extends an already-existing ring buffer.

---

## Technical Context

**Language/Version**: Rust (stable) + Swift 6 (`SWIFT_DEFAULT_ACTOR_ISOLATION = MainActor`)
**Primary Dependencies**: SmallVec (hot-path stack allocation), AppKit/SwiftUI (macOS UI), Criterion (benchmarks)
**Storage**: `UserDefaults` (settings persistence); in-memory fixed-capacity ring buffer (`WordHistory`)
**Testing**: `cargo test` (integration tests in `core/tests/`); XCTest (macOS UI); Criterion benchmarks
**Target Platform**: macOS 11+ (Big Sur)
**Project Type**: IME — Rust core library + macOS desktop app
**Performance Goals**: < 3ms core processing; < 16ms end-to-end keystroke pipeline (per constitution)
**Constraints**: No heap allocation in `process_key`; O(1) lookups; `catch_unwind` at all FFI boundaries
**Scale/Scope**: Single-user; processes every keypress in all foreground applications

---

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design. ✅ PASSES.*

| Principle | Gate | Status |
|-----------|------|--------|
| I. Performance First | No heap allocation in `process_key` for any new path; O(1) or O(log n) on Space only | ✅ PASS — all new paths are O(1) except abbreviation binary-search on Space |
| II. Clean Architecture | New config in `application/dto/`; new data in `data/`; adapter changes in `infrastructure/adapters/`; FFI in `presentation/ffi/` | ✅ PASS — no layer violations |
| III. Regression-First Testing | Regression test written and confirmed failing before each story is implemented | ✅ REQUIRED — enforced per-story |
| IV. Zero FFI Panics | All new FFI-boundary code wrapped in `catch_unwind`; Swift `defer { free }` on all new pointer returns | ✅ PASS — no new pointer-returning FFI functions introduced |
| V. Branding Consistency | No `.uvasx/` names in any new code | ✅ PASS |

---

## Project Structure

### Documentation (this feature)

```text
specs/001-feature-gap-analysis/
├── plan.md              ← this file
├── research.md          ← Phase 0 findings
├── data-model.md        ← Phase 1 entities and state
├── quickstart.md        ← validation guide
├── contracts/
│   └── ffi-v2-config-extensions.md   ← FFI contract for new config fields
└── tasks.md             ← Phase 2 output (speckit-tasks)
```

### Source Code (affected paths)

```text
core/
├── src/
│   ├── application/dto/engine_config.rs        ← +4 new bool fields
│   ├── data/
│   │   └── auto_capitalise.rs                  ← NEW: static abbreviation list
│   ├── infrastructure/
│   │   ├── adapters/
│   │   │   ├── input/telex_adapter.rs           ← bracket shortcuts + foreign consonant w
│   │   │   └── validation/fsm/tables/mod.rs     ← foreign consonant bypass gate
│   │   └── engine/
│   │       ├── mod.rs                           ← auto-capitalise state; word history gate
│   │       └── state/history.rs                 ← bump capacity 3→10; add invalidation path
│   └── presentation/ffi/
│       └── types.rs                             ← +4 fields in FfiConfig_v2
└── tests/
    ├── bracket_shortcuts_test.rs               ← NEW regression tests
    ├── foreign_consonants_test.rs              ← NEW regression tests
    ├── auto_capitalise_test.rs                 ← NEW regression tests
    └── word_history_test.rs                    ← NEW regression tests

platforms/macos/goxviet/goxviet/
├── FFI/RustBridgeSafe.swift                    ← +4 config field mappings
├── Core/AppState.swift                         ← +4 UserDefaults-backed properties
└── UI/Settings/GeneralSettingsView.swift       ← +5 new toggles (incl. ESC Restore)
```

**Structure Decision**: Monorepo, single Rust crate + single macOS app. All changes follow
the existing Clean Architecture v3.0.0 layout. No new crates or targets created.

---

## Phase 0: Research Summary

All unknowns resolved. See [research.md](research.md) for full rationale.

| Story | Key Finding | Action |
|-------|-------------|--------|
| US1 | `esc_restore_enabled` already in `EngineConfig` + `FfiConfig_v2` | UI-only work |
| US2 | `[`/`]` completely absent from `telex_adapter.rs` | Add bracket handler + config field |
| US3 | F/J/W/Z flagged `PROP_INITIAL_INVALID` in FSM tables | Config-gated bypass + `w` position logic |
| US4 | No existing auto-capitalise infrastructure | New state flag + static abbreviation data |
| US5 | `WordHistory` exists with capacity 3; FR-009 invalidation path missing | Bump to 10 + add gate + add invalidation |

---

## Phase 1: Design Summary

Artifacts produced: [data-model.md](data-model.md), [contracts/ffi-v2-config-extensions.md](contracts/ffi-v2-config-extensions.md), [quickstart.md](quickstart.md).

### Key Design Decisions

**US2 Bracket Shortcuts**: Intercept `[`/`]` in `telex_adapter.rs` before `is_modifier()` is
called, guarded by `bracket_shortcuts_enabled`. Emit `ơ`/`ư` directly without FSM processing.
No-op when `input_method == VNI`.

**US3 Foreign Consonants**: Gate the `PROP_INITIAL_INVALID` check for `{z, j, f, w}` behind
`foreign_consonants_enabled`. For `w`: check buffer emptiness in `telex_adapter.rs` — empty
buffer = word-start = literal; non-empty with preceding vowel = horn modifier (existing path).

**US4 Auto-Capitalise**: Single `at_sentence_boundary: bool` flag in engine state. Set on
Space/Enter/`!`/`?` when the preceding context is not an abbreviation or decimal. Abbreviation
detection uses binary search over a sorted static list (`data/auto_capitalise.rs`). No heap
allocation.

**US5 Word History**: Bump `HISTORY_CAPACITY` from 3 to 10. Add `word_history_enabled` config
gate (when false, ring buffer is never written). Add "non-Backspace after Space invalidates
entry" path — track an `is_restorable: bool` on the most recent ring entry, set to `false`
when any non-Backspace key is processed while `at_word_start == true`.

### Constitution Check (Post-Design) ✅

All design decisions pass. The abbreviation binary search (O(log n)) runs only once per Space
key, not per keypress — acceptable per constitution (hot path is `process_key` for mid-word
keys, not Space). No heap allocations introduced on the hot path.

---

## Complexity Tracking

*No constitution violations. No complexity justification required.*
