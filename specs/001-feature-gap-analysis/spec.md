# Feature Specification: GoxViet Feature Gap Analysis

**Feature Branch**: `001-feature-gap-analysis`
**Created**: 2026-04-04
**Status**: Draft
**Input**: Read `.uvasx/`, then identify features GoxViet does not have or has incompletely implemented.

---

## Overview

This specification documents features present in the Gõ Nhanh reference implementation that are either absent from or incompletely implemented in GoxViet. Each gap is described in user-facing terms so individual items can be prioritised, planned, and implemented incrementally.

---

## Clarifications

### Session 2026-04-05

- Q: What defines a "recognised Vietnamese abbreviation pattern" for auto-capitalise exclusion? → A: A fixed built-in list of common Vietnamese prose abbreviations (v.v., v.d., tr., tp., PGS., TS., etc.) baked into the engine.
- Q: What does pressing Backspace immediately after Space do to the word history? → A: Backspace-after-Space always restores the previous word's buffer; typing any non-Backspace character after Space cancels history for that position.
- Q: Is per-app bracket shortcut overriding a hard requirement or a nice-to-have? → A: Nice-to-have; initial release uses a global toggle only; per-app Smart Mode override is deferred.
- Q: When foreign consonants are enabled, how should standalone `w` at word-start behave in Telex mode? → A: `w` at word-start position = literal foreign consonant (e.g., "wifi" stays "wifi"); `w` after a vowel = Telex horn modifier as normal (context-sensitive).

---

## User Scenarios & Testing

### User Story 1 – ESC Key Restore is user-configurable (Priority: P1)

A user wants to recover their original keystrokes after an unwanted Vietnamese transformation. They press ESC and the buffer reverts to raw ASCII (e.g., "thẽ" → "the"). GoxViet has the internal config field `escRestoreEnabled` but no Settings UI toggle, making the feature inaccessible to non-technical users.

**Why this priority**: The engine plumbing already exists — this is a UI-only gap with immediate user impact.

**Independent Test**: Enable the ESC Restore toggle in General Settings; type a Vietnamese-transformed word; press ESC; verify the buffer reverts to raw ASCII.

**Acceptance Scenarios**:

1. **Given** the user opens General Settings, **When** they look for an ESC Restore option, **Then** a clearly-labelled toggle is visible.
2. **Given** the toggle is ON and the user typed a transformed buffer (e.g., "thẽ"), **When** the user presses ESC, **Then** the buffer resets to raw keystrokes ("the").
3. **Given** the toggle is OFF, **When** the user presses ESC, **Then** ESC passes through to the active application unchanged.

---

### User Story 2 – Bracket Shortcuts for ơ and ư in Telex Mode (Priority: P2)

A power Telex user wants to press `[` to insert `ơ` and `]` to insert `ư` without typing the full modifier sequence. This shortcut exists in Gõ Nhanh but is completely absent in GoxViet.

**Why this priority**: Many long-time Telex users (Unikey era) rely on bracket shortcuts; adding it removes a blocker for switchers.

**Independent Test**: Enable bracket shortcuts in Settings; switch to Telex mode; type `[` and confirm `ơ` is inserted; type `]` and confirm `ư` is inserted.

**Acceptance Scenarios**:

1. **Given** bracket shortcuts are enabled and Telex mode is active, **When** the user types `[`, **Then** `ơ` is inserted.
2. **Given** bracket shortcuts are enabled and Telex mode is active, **When** the user types `]`, **Then** `ư` is inserted.
3. **Given** bracket shortcuts are disabled, **When** the user types `[` or `]`, **Then** the literal bracket character is inserted.
4. **Given** VNI mode is active, **When** the user types `[` or `]`, **Then** bracket shortcuts have no effect regardless of the toggle state.

---

### User Story 3 – Allow Foreign Consonants (z, w, j, f) for Loanwords (Priority: P2)

A user typing Vietnamese loanwords like "zoom", "wifi", or "jazz" wants GoxViet to accept `z`, `w`, `j`, `f` as valid word-initial consonants and apply tone/diacritic marks to them. Currently GoxViet forces English auto-restore on these initials.

**Why this priority**: Vietnamese has a growing body of loanwords using these initials; the reference implementation supports them.

**Independent Test**: Enable foreign consonants in Settings; type "zooms" in Telex mode; verify `z` is treated as a valid initial and diacritics apply normally.

**Acceptance Scenarios**:

1. **Given** foreign consonants are enabled, **When** a word starts with z/w/j/f followed by a Vietnamese vowel+tone sequence, **Then** the output includes the foreign initial with the correct diacritic.
2. **Given** foreign consonants are disabled (default), **When** the user types a word starting with z/w/j/f, **Then** existing English auto-restore behaviour applies unchanged.
3. **Given** foreign consonants are enabled and no Vietnamese tone markers are typed, **When** the word starts with z/w/j/f, **Then** the word is passed through unchanged.

---

### User Story 4 – Auto-Capitalise After Sentence-Ending Punctuation (Priority: P3)

A user writing long-form Vietnamese text wants the first letter after `.`, `!`, `?`, or Enter to be automatically capitalised. This feature exists in Gõ Nhanh but is absent in GoxViet.

**Why this priority**: UX polish for prose writers; must handle edge cases (numbers, abbreviations) to avoid false positives.

**Independent Test**: Enable auto-capitalise in Settings; type "xin chào. t"; verify "t" is automatically capitalised to "T".

**Acceptance Scenarios**:

1. **Given** auto-capitalise is enabled, **When** the user types `.` + space + a lowercase letter, **Then** that letter is automatically capitalised.
2. **Given** auto-capitalise is enabled, **When** the user presses Enter + a lowercase letter, **Then** that letter is automatically capitalised.
3. **Given** auto-capitalise is enabled, **When** the preceding token ends a decimal number (e.g., "3.14"), **Then** no capitalisation is applied.
4. **Given** auto-capitalise is disabled, **When** any of the above inputs are given, **Then** capitalisation is unchanged.

---

### User Story 5 – Backspace-After-Space Restores Previous Word (Priority: P3)

A user confirms a word by pressing Space, then immediately realises it was wrong. They press Backspace once and re-enter editing mode for the previous word. Gõ Nhanh maintains a ring buffer (capacity 10) for this; GoxViet has no equivalent.

**Why this priority**: Significantly reduces correction friction; requires moderate engine changes.

**Independent Test**: Type "xin" + Space; press Backspace; verify the buffer is restored to "xin" in editable state.

**Acceptance Scenarios**:

1. **Given** the user confirmed a word with Space, **When** they press Backspace immediately after, **Then** the previous word's buffer is restored and editable.
2. **Given** up to 10 confirmed words are in history, **When** the user presses Backspace after multiple spaces, **Then** they can step back through previous words one at a time.
3. **Given** the user types one or more non-Backspace characters after pressing Space, **When** they then press Backspace, **Then** it deletes the most recently typed character (normal behaviour — history restore is no longer available for that Space).
4. **Given** the user switches focused apps or toggles the IME off, **When** they return, **Then** the history buffer is cleared.

---

### Edge Cases

- Bracket shortcuts (`[`, `]`) may conflict with apps that use brackets for their own purposes (code editors, Vim). The initial release provides a global toggle only; per-app Smart Mode override is a deferred enhancement.
- Foreign consonant `w` is context-sensitive when foreign consonants are enabled: word-start `w` = literal consonant (enabling "wifi", "web"); post-vowel `w` = Telex horn modifier ("ow" → "ơ", "uw" → "ư"). Bracket shortcut `[` provides `ơ` and `]` provides `ư` as an alternative when bracket shortcuts are also enabled.
- Auto-capitalise must not fire inside numeric decimals ("3.14") or after common Vietnamese abbreviations (v.v., v.d.).
- Word history buffer must be cleared on IME toggle, app focus change, and explicit buffer reset.

---

## Requirements

### Functional Requirements

- **FR-001**: GoxViet MUST expose an "ESC Restore" toggle in General Settings that enables/disables ESC-key buffer reversion to raw ASCII.
- **FR-002**: In Telex mode, GoxViet MUST insert `ơ` when `[` is pressed and `ư` when `]` is pressed, when the bracket shortcuts feature is enabled.
- **FR-003**: The bracket shortcuts toggle MUST default to OFF and be scoped to Telex mode only.
- **FR-004**: GoxViet MUST allow users to enable `z`, `w`, `j`, `f` as valid Vietnamese word-initial consonants for loanword input via a Settings toggle.
- **FR-005**: When foreign consonants are enabled, GoxViet MUST apply tone marks and diacritics to words whose initial is z/w/j/f using the same phonotactic rules as native initials. For `w` specifically: at word-start position it MUST be treated as a literal consonant (e.g., "wifi" → "wifi"); mid-word after a vowel it MUST retain its Telex horn-modifier role (e.g., "ow" → "ơ").
- **FR-006**: GoxViet MUST provide an auto-capitalise toggle that, when enabled, automatically capitalises the first character typed after `.`, `!`, `?`, or Enter.
- **FR-007**: Auto-capitalise MUST NOT trigger when the preceding context is a numeric decimal or a token matching a fixed built-in Vietnamese abbreviation list (including at minimum: v.v., v.d., tr., tp., PGS., TS.).
- **FR-008**: GoxViet MUST maintain a word history ring buffer (minimum capacity 10) allowing the user to re-enter editing mode for the previously confirmed word by pressing Backspace immediately after Space.
- **FR-009**: The word history buffer position for a given Space MUST be invalidated when the user types any non-Backspace character after that Space. The entire buffer MUST be cleared when the IME is toggled off or when the focused application changes.
- **FR-010**: All new feature toggles (FR-001 through FR-009) MUST default to OFF to preserve existing user experience.

### Key Entities

- **WordHistoryBuffer**: Ring buffer of N most-recently confirmed words (raw keystroke sequences + rendered text); cleared on context switch.
- **BracketShortcutConfig**: Maps `[` → `ơ` and `]` → `ư`; Telex-mode-scoped.
- **ForeignConsonantConfig**: Set of allowed non-traditional initials (z, w, j, f); stored in per-engine config alongside other input method settings.
- **AutoCapitaliseRule**: Trigger conditions (sentence-ending punctuation, newline) plus exclusion patterns (numeric context, abbreviation list).

---

## Success Criteria

### Measurable Outcomes

- **SC-001**: After FR-001 ships, 100% of users who enable ESC Restore can revert a transformed buffer to raw ASCII in one keypress with no application crashes.
- **SC-002**: Bracket shortcuts (FR-002/FR-003) allow Telex users to insert `ơ` or `ư` in one keystroke instead of two, with zero false triggers in VNI mode.
- **SC-003**: Foreign consonant support (FR-004/FR-005) produces correct diacritical output for at least 20 documented Vietnamese loanword patterns beginning with z/w/j/f.
- **SC-004**: Auto-capitalise (FR-006/FR-007) produces zero false positives for numeric decimal patterns and for all tokens in the built-in Vietnamese abbreviation list, and correctly capitalises ≥ 95% of sentence boundaries in standard Vietnamese prose.
- **SC-005**: Word history (FR-008/FR-009) allows recovery and re-editing of the immediately preceding confirmed word in exactly one Backspace keypress, with no data corruption across the last 10 words.

---

## Assumptions

- All changes are implemented in Rust (`core/`) and exposed through the existing FFI v2 API; the macOS Swift layer calls these without API surface changes.
- `escRestoreEnabled` is already wired into the engine — FR-001 requires only a Settings UI change (Swift side).
- Windows and Linux platform implementations are explicitly out of scope for this specification.
- Mobile platforms (iOS, Android) are out of scope for this specification.
- When foreign consonants are enabled, `w` is context-sensitive: literal consonant at word-start, Telex horn modifier after a vowel. Bracket shortcuts (`[` → `ơ`, `]` → `ư`) are complementary and can be enabled independently.
- Each user story will be broken out into its own `/speckit-plan` + `/speckit-tasks` cycle before implementation begins; this spec serves as the macOS feature roadmap overview.
