# Types and Configuration

The `types` module defines the core data structures and configuration options shared across the engine.

## Configuration (`config.rs`)

### `EngineConfig`
The central configuration struct used to initialize the engine.

-   **`method`**: Input method selection (`Telex`, `VNI`, `All`).
-   **`modern_tone`**: Boolean. Controls tone placement style.
    -   `true` (Modern): `hoà`, `thuý` (tone on second vowel).
    -   `false` (Traditional): `hòa`, `thúy` (tone on first vowel).
-   **`skip_w_shortcut`**: Disables `w` → `ư` at the start of a word (Telex only).
-   **`instant_restore_enabled`**: Toggle for the aggressive English auto-restore feature.

## FFI Integration Types (`types.rs`)

These types are designed for stable ABI compatibility with C/Swift/Java consumers.

### `Result`
The primary response struct from `ime_key()`.
-   **`action`**: `Send` (1), `Restore` (2), or `None` (0).
-   **`chars`**: **Heap-allocated** pointer (`*mut u32`) to the output characters.
-   **`backspace`**: Number of characters the client should delete before inserting `chars`.
-   **Memory Safety**: Consumers **MUST** call `ime_free(Result*)` to deallocate the `chars` buffer.

### `Action` Enum
-   `None`: key ignored by engine.
-   `Send`: engine consumed key, provides replacement.
-   `Restore`: engine requests restoration of raw input (legacy).

## Internal Types

### `Transform`
Enum tracking the specific transformation applied to a key (e.g., `Mark(key, value)`, `Tone`, `Stroke`).
-   Used for the **Undo** feature: Pressing a tone key twice (e.g., `s` then `s`) checks the history for a `Transform::Mark` triggered by `s` and removes it.
