# Buffer Management (`engine/buffer/`)

The buffer module handles the storage and manipulation of the text currently being composed.

## `Buffer` (`buffer.rs`)

The `Buffer` is a fixed-size array (up to 256 chars) representing the current word. It operates on `Char` structs rather than raw bytes or strings.

### `Char` Struct
Represents a single character in the buffer with its associated diacritics:
- `key`: The base character key code (e.g., `a`, `d`, `o`).
- `caps`: Shift state.
- `tone`: Vowel modification (None, Circumflex `^`, Horn `+`, Breve `(`).
- `mark`: Tone mark (None, Sắc, Huyền, Hỏi, Ngã, Nặng).
- `stroke`: Consonant modification (e.g., `d` -> `đ`).

### Key Methods
- **`push/pop`**: Standard stack operations.
- **`find_vowels() -> Vec<usize>`**: Returns indices of all vowel characters, used heavily by transformation logic.
- **`to_full_string() -> String`**: Converts the internal representation into a standard UTF-8 Vietnamese string, applying all diacritics and composition rules.

## `RawInputBuffer` (`raw_input_buffer.rs`)

A specialized, memory-efficient history of raw keystrokes.

- **Purpose**: Allows restoring the original input when the user presses ESC or when English is detected. For example, if the user types `tieengs` (yielding `tiếng`), the raw buffer stores `t,i,e,e,n,g,s`.
- **Implementation**: Uses a fixed-size ring buffer (capacity 64) to avoid heap allocations.
- **Performance**: Optimized for O(1) push/pop and zero-allocation iteration.

## Buffer Rebuild (`rebuild.rs`)

This module (not detailed here but used by `Engine`) handles the logic of calculating what changed between the previous state and the current state. It generates the `Result` struct containing `backspace` count and `chars` to insert, ensuring the client application updates its display correctly.
