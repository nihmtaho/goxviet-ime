# Library API (`lib.rs`)

`lib.rs` defines the public interface of the core engine, primarily focusing on the C-compatible FFI (Foreign Function Interface) which allows the Rust engine to be used by other languages (like Swift/Objective-C for macOS).

## Global State

The engine instance is held in a global static `Mutex`.

- `ENGINE`: `static ENGINE: Mutex<Option<Engine>>`
  - Thread-safe global singleton for the engine.

## FFI Functions

These functions are exported with `#[no_mangle]` and `extern "C"` to be callable from C/C++.

### Lifecycle

- **`ime_init()`**
    - Initializes the global engine instance.
    - **Must** be called exactly once before any other function.
    - Panics if the internal mutex is poisoned.

### Key Processing

- **`ime_key(key: u16, caps: bool, ctrl: bool) -> *mut Result`**
    - Process a standard key event.
    - **Arguments**:
        - `key`: macOS virtual keycode.
        - `caps`: Whether Caps Lock is on.
        - `ctrl`: Whether a control key (Cmd/Ctrl/Alt) is pressed.
    - **Returns**: A pointer to a `Result` struct (must be freed with `ime_free`).

- **`ime_key_ext(key: u16, caps: bool, ctrl: bool, shift: bool) -> *mut Result`**
    - Extended version of `ime_key` including the Shift key state.
    - Useful for VNI input where Shift+Number produces symbols (@, #, $) instead of tone marks.

- **`ime_free(r: *mut Result)`**
    - Frees the memory allocated for the `Result` struct returned by `ime_key`.
    - **Safety**: `r` must be a valid pointer from `ime_key` or `null`. Must be called exactly once per result.

### Configuration

- **`ime_method(method: u8)`**
    - Sets the input method.
    - `0`: Telex
    - `1`: VNI

- **`ime_enabled(enabled: bool)`**
    - Enables or disables the engine. When disabled, keys pass through processed.

- **`ime_skip_w_shortcut(skip: bool)`**
    - Configures `w` behavior in Telex.
    - `true`: `w` at word start remains `w`.
    - `false`: `w` at word start becomes `ư`.

- **`ime_esc_restore(enabled: bool)`**
    - `true`: pressing ESC restores the raw ASCII chars of the current word.
    - `false`: ESC passes through.

- **`ime_free_tone(enabled: bool)`**
    - `true`: Allows placing tones anywhere (skips strict spelling validation).
    - `false`: Enforces Vietnamese spelling rules.

- **`ime_modern(modern: bool)`**
    - `true`: Modern tone placement (e.g., "hoà").
    - `false`: Traditional tone placement (e.g., "hòa").

- **`ime_instant_restore(enabled: bool)`**
    - `true`: Automatically restores English words as soon as they are detected.

### State Management

- **`ime_clear()`**
    - Clears the current input buffer.
    - Should be called on word boundaries (space, punctuation, mouse clicks).

- **`ime_clear_all()`**
    - Clears all state, including word history.
    - Should be called when the cursor moves to a new location to prevent restoring context from a different location.

- **`ime_get_buffer() -> *const c_char`**
    - Returns a pointer to the current buffer content as a C string.
    - **Safety**: Returns a pointer to a static buffer; do not free.

### Shortcuts

- **`ime_add_shortcut(trigger: *const c_char, replacement: *const c_char) -> bool`**
    - Adds a user-defined shortcut.
    - Returns `true` if successful.

- **`ime_remove_shortcut(trigger: *const c_char)`**
    - Removes a specific shortcut.

- **`ime_clear_shortcuts()`**
    - Removes all shortcuts.

- **`ime_shortcuts_count() -> usize`**
    - Returns the number of active shortcuts.

- **`ime_shortcuts_capacity() -> usize`**
    - Returns the maximum number of shortcuts allowed.

- **`ime_shortcuts_is_at_capacity() -> bool`**
    - Checks if the shortcut table is full.

### Word Restoration

- **`ime_restore_word(word: *const c_char)`**
    - Restores the engine state from a given Vietnamese string.
    - Used when the user navigates back into a word to edit it.

## Internal Utilities

- **`lock_engine() -> MutexGuard`**
    - Helper to acquire the engine lock safely, handling poisoned mutexes if necessary.
