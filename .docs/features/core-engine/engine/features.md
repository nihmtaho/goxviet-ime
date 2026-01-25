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
