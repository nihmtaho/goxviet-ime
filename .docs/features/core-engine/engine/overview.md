# Engine Overview (`engine/mod.rs`)

The `Engine` struct is the central component of the library. It orchestrates the input processing pipeline, managing state, validation, transformation, and output generation.

## Architecture

The engine uses a **validation-first, pattern-based** approach. Instead of maintaining a complex state machine for every possible character transition, it:

1.  **Maintains a Buffer**: Holds the current word being composed.
2.  **Scans the Buffer**: On every keystroke, checking for patterns (English words, shortcuts, Vietnamese structures).
3.  **Transforms**: Applies changes to the buffer if valid (e.g., adding a tone, modifying a vowel).
4.  **Rebuilds Output**: Generates the final result for the application.

## Core `Engine` Struct

- **State**
    - `buf`: The current typing buffer (`Buffer`).
    - `method`: Current input method (Telex/VNI).
    - `shortcuts`: User-defined abbreviation table.
    - `raw_input`: Keystroke history (`RawInputBuffer`) for ESC restore.
    - `word_history`: Ring buffer of previous words for advanced backspace handling.

- **Configuration Flags**
    - `enabled`: Global on/off switch.
    - `raw_mode`: Skips transformations for raw input (e.g., password fields).
    - `skip_w_shortcut`: Disables `w` -> `ư` at word start.
    - `esc_restore_enabled`: Enables ESC key to undo transformations.
    - `free_tone_enabled`: Relaxes validation rules.
    - `modern_tone`: Toggles between traditional (`òa`) and modern (`oà`) tone placement.
    - `instant_restore_enabled`: Automatically restores English words.

## Key Processing Pipeline

The `process` method is the heart of the engine:

1.  **English Detection (Layer 1)**: Checks if the input matches known English words (e.g., "release", "telex"). If so, it may bypass Vietnamese transformations to prevent unwanted changes.
2.  **Modifier Check**: Determines if the key is a tone mark (`s`, `f`, `1`, `2`), vowel modifier (`w`, `aa`, `ee`), or stroke modifier (`d`).
    - **Revert**: If the same modifier is pressed again (e.g., `s` then `s`), it removes the tone.
3.  **Transformation Attempts**:
    - **Stroke**: Tries to convert `d` to `đ`.
    - **Tone**: Tries to apply acute, grave, hook, tilde, or dot tones.
    - **Mark**: Tries to apply circumflex, horn, or breve to vowels.
    - **Remove**: Tries to remove diacritics (`z` or `0`).
    - **W-Shortcut**: In Telex, tries to convert `w` to `ư` or `ươ`.
4.  **Normal Letter**: If no modifier applies, adds the character as a regular letter.
5.  **Output Rebuild**: After any change, calls `rebuild_output_from_entire_buffer` to generate the diff (backspace + replacements) for the application.

## Advanced Features

- **Word History**: The engine remembers the last few committed words. If the user backspaces over a space, it can "resurrect" the previous word into the buffer for editing.
- **English Auto-Restore**: If the user types what looks like a valid English word (detected via phonotactics or dictionary), the engine can automatically undo any accidental Vietnamese transformations. To prevent false positives during rapid typing (intermediate tone placement like `phast`), a high confidence threshold (**95%**) is required for words that form valid Vietnamese syllables.
- **Speculative Modifiers**: Even if a word looks like English, if a user explicitly types a modifier that creates a valid Vietnamese word, the engine allows it (resolving conflicts like "dis" vs "dí").
