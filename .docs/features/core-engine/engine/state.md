# State Management Module

The `state` module handles the engine's internal memory of past actions, enabling undo/redo functionality and intelligent restoration of input.

## Word History (`history.rs`)

A fixed-size **Ring Buffer** (`WordHistory`) that stores the last N committed words.

-   **Purpose**: Enables the "Backspace after Space" feature. If a user commits a word (e.g., "việt ") and immediately presses backspace, the engine restores the previous word's state from history, allowing them to edit the previous word ("việt").
-   **Architecture**:
    -   Stores pairs of `(Buffer, RawInputBuffer)`.
    -   **Stack Allocated**: Uses fixed-size arrays (`[Buffer; 3]`) to avoid heap allocations during typing.
    -   **Performance**: O(1) push/pop operations.

## Restoration Utilities (`restore.rs`)

Logic for reverting Vietnamese transformations back to raw ASCII input.

### Auto-Restore English
`auto_restore_english`: Triggered when the engine detects an English word that was accidentally transformed (e.g., typing "telex" results in "tễl").
-   Reconstructs raw input from `RawInputBuffer`.
-   Appends a space for better UX.

### Instant Restore
`instant_restore_english`: Immediate restoration for high-confidence English detection mid-word.
-   **Incremental Optimization**: Instead of deleting and retyping the entire word, it attempts to only backspace to the first transformed character and retype from there, significantly reducing the number of keystrokes sent to the OS.

### ESC Key Restore
`restore_to_raw`: Restores the active buffer to its raw keystrokes when ESC is pressed. Useful for quick corrections when the engine misinterprets intention.
