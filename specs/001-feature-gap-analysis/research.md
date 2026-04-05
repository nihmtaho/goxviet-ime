# Research: GoxViet Feature Gap (US1–US5)

**Phase**: 0 | **Date**: 2026-04-05 | **Plan**: [plan.md](plan.md)

---

## Decision 1: ESC Restore (US1) — UI-only change

**Decision**: No engine or FFI changes required; only a Swift UI toggle is missing.

**Rationale**: `esc_restore_enabled: bool` already exists in `EngineConfig`
(`core/src/application/dto/engine_config.rs:70`) and is already exposed in `FfiConfig_v2`
(`core/src/presentation/ffi/types.rs:220`). Default is `false`. The Swift
`GeneralSettingsView.swift` has no toggle for it. One SwiftUI `Toggle` binding in the
Smart Features section is sufficient.

**Alternatives considered**: Adding a new FFI entry-point — rejected; unnecessary given the
field is already in `FfiConfig_v2`.

---

## Decision 2: Bracket Shortcuts (US2) — Telex adapter + config + UI

**Decision**: Add `bracket_shortcuts_enabled: bool` to `EngineConfig` and `FfiConfig_v2`.
Intercept `[` (keycode 33) and `]` (keycode 30) early in the Telex adapter when the flag is
set; emit `ơ` and `ư` respectively without further FSM processing. No-op in VNI mode.

**Rationale**: `[` and `]` are completely absent from `telex_adapter.rs`. They are not tone
markers, diacritics, or remove-keys and fall through to the pass-through path. The cleanest
intercept point is before `is_modifier()` is called, guarded by a config flag.

Key constants already defined in `core/src/data/keys.rs`:
- `LBRACKET = 33`
- `RBRACKET = 30`

**Alternatives considered**: Handling in the unified engine layer — rejected; Telex adapter
is the correct layer (Input in infrastructure). VNI must be unaffected, so mode-specific
handling in the adapter is cleaner than a unified-engine branch.

---

## Decision 3: Foreign Consonants (US3) — FSM bypass + context-aware `w` + config + UI

**Decision**: Add `foreign_consonants_enabled: bool` to `EngineConfig` and `FfiConfig_v2`.
When the flag is set, skip the `PROP_INITIAL_INVALID` check for `z`, `j`, `f`, and
position-aware `w` in the FSM validator. For `w`: if the current buffer is empty (word-start
position), treat as literal consonant; if the buffer contains a vowel (post-vowel position),
retain existing Telex horn-modifier behaviour.

**Rationale**: The FSM table at `core/src/infrastructure/adapters/validation/fsm/tables/mod.rs`
marks F (line 25), J (line 28), W (line 39), Z (line 41) with `PROP_INITIAL_INVALID`. The
English auto-restore path fires when a word-initial key fails this check. Adding a config-
gated bypass before the property lookup is an O(1) operation that touches no other validation
paths.

The `w`-at-word-start vs. post-vowel distinction must be decided in the Telex adapter's key
processing, where buffer state is accessible before FSM validation.

**Alternatives considered**: Modifying the FSM table unconditionally — rejected; that would
break English auto-restore for all users without the opt-in. A separate "foreign initial"
lookup table — rejected; the single `PROP_INITIAL_INVALID` bypass is simpler and verifiable.

---

## Decision 4: Auto-Capitalise (US4) — New engine state + static abbreviation data + config + UI

**Decision**: Add `auto_capitalise_enabled: bool` to `EngineConfig` and `FfiConfig_v2`. Track
a `sentence_boundary: bool` flag in engine state. Set it to `true` when Space, Enter, `!`,
`?` is processed and the previous confirmed output does not end in an abbreviation or decimal.
When `sentence_boundary` is true and the next key is a lowercase letter, capitalise it (output
the uppercase form). Use a static `&[&str]` list for abbreviation detection.

**Abbreviation list (minimum, baked into engine as `data/auto_capitalise.rs`):**
`["v.v.", "v.d.", "tr.", "tp.", "pgs.", "ts.", "ths.", "bs.", "gs.", "pgsts.", "đ.", "no.", "no"]`

Case-insensitive comparison against the last confirmed word before the period.

**Decimal detection**: If the character immediately before `.` is an ASCII digit (`0–9`),
`sentence_boundary` is NOT set.

**Performance**: The abbreviation list is a static sorted `&[&str]`; binary search is O(log n)
and runs only on Space/Enter (not on every keypress). The `sentence_boundary` flag is a single
`bool` in engine state — no allocation.

**Alternatives considered**: Regex-based detection — rejected; no regex crate on the hot path,
violates no-heap rule. User-configurable list — deferred per clarification session.

---

## Decision 5: Word History / Backspace-After-Space (US5) — Extend existing infrastructure

**Decision**: `WordHistory` ring buffer already exists at
`core/src/infrastructure/engine/state/history.rs` with capacity 3. Increase
`HISTORY_CAPACITY` to 10. Verify that the existing push/pop/invalidation logic matches
FR-008/FR-009:
- Space pushes current buffer to ring ✅ (confirmed in `mod.rs:499` area)
- Backspace at word-start pops ring ✅ (confirmed in `history.rs`)
- Non-Backspace after Space invalidates the last entry — **needs verification**

Add `word_history_enabled: bool` to `EngineConfig` and `FfiConfig_v2` to gate the feature
behind a Settings toggle. Add Swift UI toggle.

**Rationale**: The infrastructure exists and is architecturally correct. The main work is:
1. Increase capacity from 3 → 10
2. Add the config gate
3. Verify/add the "non-Backspace after Space invalidates entry" path (FR-009)
4. Wire UI toggle

**Alternatives considered**: Reimplementing from scratch — rejected; existing `WordHistory`
follows the same ring-buffer pattern and is already tested.

---

## Summary of Required Changes

| Story | Rust Core | FFI v2 Config | Swift UI |
|-------|-----------|---------------|----------|
| US1 ESC Restore | None | None (already there) | Add toggle |
| US2 Bracket Shortcuts | `telex_adapter.rs` | Add `bracket_shortcuts_enabled` | Add toggle |
| US3 Foreign Consonants | FSM bypass + `telex_adapter.rs` | Add `foreign_consonants_enabled` | Add toggle |
| US4 Auto-Capitalise | New state + static data | Add `auto_capitalise_enabled` | Add toggle |
| US5 Word History | Bump capacity + add gate + verify FR-009 path | Add `word_history_enabled` | Add toggle |
