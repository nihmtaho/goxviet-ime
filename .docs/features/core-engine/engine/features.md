# Features Module

The `features` module implements user-facing features that sit on top of the core transformation engine, primarily the **Shortcut System**.

## Shortcut System (`shortcut.rs`)

The `ShortcutTable` allows users to define custom abbreviations (macros) that expand into longer text.

### Key Capabilities

-   **Input Method Specificity**: Shortcuts can be global (`InputMethod::All`) or restricted to specific modes (`Telex` or `VNI`).
-   **Trigger Conditions**:
    -   `Immediate`: Expands as soon as the trigger is typed (e.g., `w` → `ư` in Telex logic).
    -   `OnWordBoundary`: Expands only when a boundary key (space, punctuation) is pressed (e.g., `vn` + space → `Việt Nam `).
-   **Case Preservation**:
    -   `Exact`: Output is exactly as defined.
    -   `MatchCase`: Adapts output case to input (e.g., `vn` → `Việt Nam`, `VN` → `VIỆT NAM`).

### Memory Management
To prevent unbounded memory growth, the table size is limited by `MAX_SHORTCUTS` (default 200). It uses `HashMap` for O(1) lookups and maintains a `sorted_triggers` vector (longest-first) for correct matching priority.

### Replacement Validation
Replacements are truncated to `MAX_REPLACEMENT_LEN` (matches the `Result` buffer size minus padding) to ensure they can be safely passed through the FFI boundary.

## Raw Input Buffer & English Detection

To enable robust English detection and auto-restore functionality, the engine maintains a complete history of all keystroke inputs in the **raw input buffer** (`raw_input`). This buffer records **every key pressed**, even if that key is internally treated as a modifier (e.g., `s` in Telex for tone marking, or `aa` for circumflex diacritics).

### Why Complete Keystroke History Matters

The dictionary-based English detection engine needs the original key sequence to make accurate decisions:

#### Example: "console"

- User types: `c` → `o` → `n` → `s` → `o` → `l` → `e`
- If `s` is skipped from the raw buffer, the engine only sees: `c`, `o`, `n`, `o`, `l`, `e` → `coole` (incomplete match)
- With complete history: `console` → Perfect dictionary match → "This is English, don't transform."

#### Example: "roadmap"

- User types: `r` → `o` → `a` → `d` → `m` → `a` → `p`
- Without `a` in raw buffer: `romap` (missing vowels, invalid Vietnamese) → possible false transformation
- With complete history: `roadmap` → Dictionary match confirmed → Protected from transformation

### Auto-Restore with Raw Buffer

When the engine detects an English word that has been accidentally transformed (e.g., "rõadmap" instead of "roadmap"), it can restore the original by:

1. Comparing the current transformed output against the raw input buffer
2. Calculating English confidence from the raw input history
3. If confidence is high enough (≥ 95% for valid Vietnamese syllables), returning a `Restore` action
4. Platform layer then deletes the transformed text and inserts the original English word

Without the complete raw buffer, this restoration logic would fail silently, leaving the user with unwanted Vietnamese transformations.
