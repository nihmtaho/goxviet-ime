---

description: "Task list for GoxViet Feature Gap (US1–US5)"
---

# Tasks: GoxViet Feature Gap (US1–US5)

**Input**: Design documents from `specs/001-feature-gap-analysis/`
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, contracts/ ✅

**Tests**: Regression test tasks are included per Constitution Principle III (Regression-First
Testing). Each test task MUST be written and confirmed failing before its corresponding
implementation task begins.

**Organization**: Tasks are grouped by user story to enable independent implementation and
testing of each story.

## Format: `[ID] [P?] [Story?] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1–US5)
- File paths are absolute from repo root

---

## Phase 1: Setup

**Purpose**: Create test file stubs and confirm existing infrastructure is in place.

- [X] T001 Verify `core/tests/` integration test runner works: run `cd core && cargo test` and confirm existing tests pass; note any existing `esc_restore`, `bracket`, `foreign_consonant`, `auto_capitalise`, `word_history` test files

---

## Phase 2: Foundational (Shared Config — Blocking US2–US5)

**Purpose**: Add 4 new boolean config fields to the shared engine config and Swift bridge.
US1 does not require these; US2–US5 all do. No user story work for US2–US5 can begin until
this phase is complete.

**⚠️ CRITICAL**: US2–US5 implementation tasks MUST NOT begin until T002–T005 are complete.

- [X] T002 [P] Add `bracket_shortcuts_enabled: bool`, `foreign_consonants_enabled: bool`, `auto_capitalise_enabled: bool`, `word_history_enabled: bool` fields (all defaulting to `false`) to `EngineConfig` struct in `core/src/application/dto/engine_config.rs`
- [X] T003 [P] Add matching `bracket_shortcuts_enabled: bool`, `foreign_consonants_enabled: bool`, `auto_capitalise_enabled: bool`, `word_history_enabled: bool` fields to `FfiConfig_v2` struct in `core/src/presentation/ffi/types.rs`; update `Default` impl to set all to `false`
- [X] T004 Add 4 new `@AppStorage`-backed `Bool` properties to `AppState.swift` in `platforms/macos/goxviet/goxviet/Core/AppState.swift` for each new config field, mirroring naming convention of existing fields (depends on T002, T003)
- [X] T005 Add 4 config field assignments in `RustBridgeSafe.swift` (`platforms/macos/goxviet/goxviet/FFI/RustBridgeSafe.swift`) inside the `toFfi()` / config-building method, mapping Swift properties to C struct fields (depends on T004)

**Checkpoint**: Rust core compiles (`cargo build`); macOS app compiles in Xcode. Foundation ready for US1–US5.

---

## Phase 3: User Story 1 – ESC Key Restore Toggle (Priority: P1) 🎯 MVP

**Goal**: Expose the already-wired `esc_restore_enabled` engine config field via a Settings UI toggle.

**Independent Test**: Enable toggle in Settings → General; type a transformed Vietnamese word;
press ESC; verify raw ASCII is restored. Disable toggle; verify ESC passes through.

### Regression Test for US1

> **NOTE: Write this test FIRST. Confirm it FAILS before implementing T007.**

- [X] T006 [US1] Write integration test in `core/tests/esc_restore_test.rs` verifying that: (a) with `esc_restore_enabled: true`, an ESC key call after a Vietnamese transformation reverts the buffer to raw ASCII; (b) with `esc_restore_enabled: false`, ESC is not consumed by the engine

### Implementation for US1

- [X] T007 [US1] Add an "ESC Key Restore" `Toggle` binding in `GeneralSettingsView.swift` (`platforms/macos/goxviet/goxviet/UI/Settings/GeneralSettingsView.swift`) inside the Smart Features section, bound to `AppState.escRestoreEnabled`; confirm the toggle persists across app restarts

**Checkpoint**: US1 fully functional. T006 regression test now passes. Validate with quickstart.md § US1.

---

## Phase 4: User Story 2 – Bracket Shortcuts ơ/ư in Telex (Priority: P2)

**Goal**: In Telex mode, `[` inserts `ơ` and `]` inserts `ư` when the bracket shortcuts feature is enabled.

**Independent Test**: Enable toggle; switch to Telex; type `[` → confirm `ơ`; type `]` → confirm `ư`.
Disable toggle → confirm literal brackets. Switch to VNI → confirm no effect.

### Regression Test for US2

> **NOTE: Write this test FIRST. Confirm it FAILS before implementing T009.**

- [X] T008 [US2] Write integration test in `core/tests/bracket_shortcuts_test.rs` covering: (a) `bracket_shortcuts_enabled: true` + Telex mode: `[` → `ơ`, `]` → `ư`; (b) `bracket_shortcuts_enabled: false`: `[`/`]` pass through as literals; (c) VNI mode: `[`/`]` always pass through regardless of flag (depends on T002 compile passing)

### Implementation for US2

- [X] T009 [US2] In `core/src/infrastructure/adapters/input/telex_adapter.rs`, add bracket key interception before the `is_modifier()` check: when `config.bracket_shortcuts_enabled` is `true` and the input keycode is `LBRACKET (33)` or `RBRACKET (30)`, emit `ơ` (U+01A1) or `ư` (U+01B0) respectively and return early without FSM processing
- [X] T010 [P] [US2] Add a "Bracket Shortcuts (Telex only)" `Toggle` in `GeneralSettingsView.swift` inside the Tone Settings or Editing section, bound to `AppState.bracketShortcutsEnabled`; label must note it is Telex-only

**Checkpoint**: US2 fully functional and independently testable. T008 regression test passes. Validate with quickstart.md § US2.

---

## Phase 5: User Story 3 – Foreign Consonants z/w/j/f (Priority: P2)

**Goal**: When enabled, z/w/j/f are valid Vietnamese word-initial consonants. `w` is
context-sensitive: literal at word-start, horn modifier after a vowel.

**Independent Test**: Enable toggle; Telex mode; type `wifi` → `wifi` (not `ưifi`); type `zoom` → `zoom`;
type `hoaw` → `hoă` (horn modifier intact mid-word). Disable → English auto-restore fires.

### Regression Test for US3

> **NOTE: Write this test FIRST. Confirm it FAILS before implementing T012.**

- [X] T011 [US3] Write integration test in `core/tests/foreign_consonants_test.rs` covering: (a) `foreign_consonants_enabled: true`: `wifi` → `wifi`, `zoom` → `zoom`, `jazz` → `jazz`; (b) `w` post-vowel still acts as horn: `how` → `hơ`; (c) `foreign_consonants_enabled: false`: existing English auto-restore behaviour unchanged for z/w/j/f initials

### Implementation for US3

- [X] T012 [US3] In the FSM validation path (trace from `core/src/infrastructure/adapters/validation/fsm/tables/mod.rs` through the caller), add a config-gated bypass: when `foreign_consonants_enabled` is `true` and the initial character is one of `{z, j, f}`, skip the `PROP_INITIAL_INVALID` rejection and allow the character as a word initial
- [X] T013 [US3] In `core/src/infrastructure/adapters/input/telex_adapter.rs`, add position-aware `w` logic: when `foreign_consonants_enabled` is `true` and `w` is pressed, check if the current buffer is empty (word-start) → treat `w` as a literal consonant and bypass the horn-modifier path; if buffer is non-empty with a preceding vowel → use existing Telex horn-modifier behaviour
- [X] T014 [P] [US3] Add a "Foreign Consonants (z, w, j, f)" `Toggle` in `GeneralSettingsView.swift` in the Input Method section, bound to `AppState.foreignConsonantsEnabled`; add a brief descriptive subtitle explaining loanword use case

**Checkpoint**: US3 fully functional and independently testable. T011 regression test passes. Validate with quickstart.md § US3.

---

## Phase 6: User Story 4 – Auto-Capitalise After Sentence End (Priority: P3)

**Goal**: When enabled, the first letter after `.`, `!`, `?`, or Enter is automatically
capitalised, except after numeric decimals or tokens in the built-in abbreviation list.

**Independent Test**: Enable toggle; type `xin chào. t` → `T` is auto-capitalised;
type `3.14 t` → `t` unchanged; type `v.v. t` → `t` unchanged.

### Regression Test for US4

> **NOTE: Write this test FIRST. Confirm it FAILS before implementing T016.**

- [X] T015 [US4] Write integration test in `core/tests/auto_capitalise_test.rs` covering: (a) sentence-end `.` + space + lowercase → capitalised; (b) `!` and `?` triggers; (c) Enter trigger; (d) decimal exclusion `3.14 t` → `t` unchanged; (e) abbreviation exclusions: `v.v.`, `v.d.`, `tr.`, `tp.`, `PGS.`, `TS.` → no capitalisation; (f) `auto_capitalise_enabled: false` → no effect

### Implementation for US4

- [X] T016 [US4] Create `core/src/data/auto_capitalise.rs` with a sorted static `&[&str]` array `ABBREVIATION_LIST` containing at minimum: `["bs.", "đ.", "gs.", "no.", "pgs.", "pgsts.", "ths.", "tp.", "tr.", "ts.", "v.d.", "v.v."]`; register the new module in `core/src/data/mod.rs`
- [X] T017 [US4] Add `at_sentence_boundary: bool` field to the engine state struct in `core/src/infrastructure/engine/mod.rs`; set it to `true` when a Space/`!`/`?` is processed AND the preceding token is not in `ABBREVIATION_LIST` and does not end a decimal number; set it to `false` after each non-Space key is processed (depends on T016)
- [X] T018 [US4] In the engine's key processing path (`core/src/infrastructure/engine/mod.rs`), when `auto_capitalise_enabled` is `true` and `at_sentence_boundary` is `true` and the incoming key is a lowercase ASCII letter, capitalise the letter (output uppercase form) and set `at_sentence_boundary = false` (depends on T017)
- [X] T019 [P] [US4] Add an "Auto-Capitalise After Sentence End" `Toggle` in `GeneralSettingsView.swift` in the Smart Features section, bound to `AppState.autoCapitaliseEnabled`

**Checkpoint**: US4 fully functional and independently testable. T015 regression test passes. Validate with quickstart.md § US4.

---

## Phase 7: User Story 5 – Word History / Backspace-After-Space (Priority: P3)

**Goal**: Pressing Backspace immediately after confirming a word with Space restores the
previous word's buffer for editing. Ring buffer capacity is 10. Non-Backspace after Space
invalidates the restore opportunity for that position.

**Independent Test**: Type `xin` + Space + Backspace → buffer restores to `xin`;
type `hello` + Space + `w` + Backspace → only `w` deleted (history not invoked).

### Regression Test for US5

> **NOTE: Write this test FIRST. Confirm it FAILS before implementing T021.**

- [X] T020 [US5] Write integration test in `core/tests/word_history_test.rs` covering: (a) Space then immediate Backspace → previous word buffer restored; (b) Space then non-Backspace key then Backspace → non-Backspace key deleted, history NOT invoked; (c) 10 consecutive words with spaces, then Backspace → steps back through all 10; (d) `word_history_enabled: false` → Backspace-after-Space behaves as plain Backspace with no restore

### Implementation for US5

- [X] T021 [US5] In `core/src/infrastructure/engine/state/history.rs`, change `HISTORY_CAPACITY` from `3` to `10`; add an `is_restorable: bool` field to the ring buffer entry type to track whether the most recent entry can still be restored
- [X] T022 [US5] In `core/src/infrastructure/engine/mod.rs`, gate the entire `WordHistory` push/pop code paths behind `config.word_history_enabled`; when `false`, the ring buffer MUST NOT be written to or read from (depends on T021)
- [X] T023 [US5] In `core/src/infrastructure/engine/mod.rs`, add the FR-009 invalidation path: when any non-Backspace key is processed while the engine is at word-start position (buffer empty, most recent ring entry is restorable), set `is_restorable = false` on that entry (depends on T022)
- [X] T024 [P] [US5] Add a "Backspace-After-Space Restore" `Toggle` in `GeneralSettingsView.swift` in the Editing section, bound to `AppState.wordHistoryEnabled`

**Checkpoint**: US5 fully functional and independently testable. T020 regression test passes. Validate with quickstart.md § US5.

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Final quality pass after all user stories are complete.

- [X] T025 [P] Run `cd core && cargo fmt && cargo clippy -- -D warnings` on all changed files; fix all warnings in `telex_adapter.rs`, `mod.rs`, `history.rs`, `auto_capitalise.rs`, `engine_config.rs`, `types.rs`
- [X] T026 Run `cd core && cargo test` — confirm all existing and new regression tests pass; zero regressions allowed
- [X] T027 Run `cd core && cargo bench` — confirm `process_key` benchmark shows no regression vs. baseline (< 3ms per constitution Principle I)
- [X] T028 [P] Validate all 5 features against `quickstart.md` acceptance criteria in a real macOS build
- [X] T029 [P] Update `CHANGELOG.md` with entries for each new feature under the next version section

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Phase 1 — **BLOCKS all US2–US5 user stories**
- **US1 (Phase 3)**: Depends on Phase 1 only (no Rust config changes needed); can run in parallel with Phase 2
- **US2 (Phase 4)**: Depends on Foundational completion
- **US3 (Phase 5)**: Depends on Foundational completion
- **US4 (Phase 6)**: Depends on Foundational completion
- **US5 (Phase 7)**: Depends on Foundational completion
- **Polish (Phase 8)**: Depends on all user story phases complete

### User Story Dependencies

- **US1 (P1)**: Independent of US2–US5 — starts after Phase 1
- **US2 (P2)**: Independent of US1/US3/US4/US5 — starts after Foundational
- **US3 (P2)**: Independent of US1/US2/US4/US5 — starts after Foundational; touches same `telex_adapter.rs` as US2 so coordinate if working in parallel
- **US4 (P3)**: Independent of all other stories
- **US5 (P3)**: Independent of all other stories

### Within Each User Story

- Regression test task MUST be written and confirmed failing before implementation tasks
- Static data / struct additions before logic tasks
- Engine logic before UI toggle
- [P] tasks within a story can run in parallel

### Parallel Opportunities

- T002 and T003 can run in parallel (different files)
- T006 (US1 test) and T002–T003 (Foundational) can run in parallel
- T007 (US1 UI) does not depend on Foundational — can run concurrently with T002–T005
- Once Foundational is done: T008, T011, T015, T020 can all start in parallel (tests for US2–US5)
- UI toggles T010, T014, T019, T024 can all be done in parallel once AppState (T004) is done
- T025, T028, T029 in Polish phase are independent of each other

---

## Parallel Example: Foundational + US1 in parallel

```bash
# Start these together:
Task T002: "Add 4 bool fields to EngineConfig in core/src/application/dto/engine_config.rs"
Task T003: "Add 4 bool fields to FfiConfig_v2 in core/src/presentation/ffi/types.rs"
Task T006: "Write ESC restore regression test in core/tests/esc_restore_test.rs"

# Once T002+T003 done, start:
Task T004: "Add AppStorage properties to AppState.swift"

# Once T004 done, start in parallel:
Task T005: "Wire config fields in RustBridgeSafe.swift"
Task T007: "Add ESC Restore toggle in GeneralSettingsView.swift"
```

---

## Implementation Strategy

### MVP First (US1 Only)

1. Complete Phase 1: Setup (T001)
2. Complete US1 implementation (T006, T007) — no Foundational phase needed
3. **STOP and VALIDATE**: Test US1 independently using quickstart.md
4. Demo ESC restore to confirm UX

### Incremental Delivery (Recommended)

1. Phase 1 + Foundational (T001–T005)
2. US1 in parallel with Foundational → validate
3. US2 → validate independently
4. US3 → validate independently (coordinate `telex_adapter.rs` edits with US2)
5. US4 → validate independently
6. US5 → validate independently
7. Polish (T025–T029)

### Parallel Team Strategy (if applicable)

Once Foundational is complete:
- Developer A: US1 (UI-only, can start during Foundational)
- Developer B: US2 Bracket Shortcuts
- Developer C: US3 Foreign Consonants (coordinate `telex_adapter.rs` with Dev B)
- Developer D: US4 Auto-Capitalise
- Developer E: US5 Word History

---

## Notes

- [P] tasks = different files, no shared-state dependencies
- [Story] label maps each task to its user story for traceability
- Each regression test MUST fail before its implementation task runs (per Constitution III)
- `telex_adapter.rs` is touched by both US2 (T009) and US3 (T013) — merge carefully if worked in parallel
- All new toggles default to OFF — do not change defaults under any circumstance (FR-010)
- `cargo clippy -- -D warnings` must pass before the feature is considered complete
