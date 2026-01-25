# Utilities (`utils.rs`)

`utils.rs` provides shared utility functions used across the engine modules, primarily for character processing and testing.

## Character Constants & Conversion

- **`key_to_char(key: u16, caps: bool) -> Option<char>`**
    - Converts a virtual keycode to its character representation.
    - Handles standard letters (A-Z) and numbers (0-9).
    - Respects the `caps` flag for uppercase/lowercase.

## Vowel Analysis

- **`collect_vowels(buf: &Buffer) -> Vec<Vowel>`**
    - Scans the buffer and extracts all vowel characters.
    - Returns a list of `Vowel` structs containing key, modifier (circumflex/horn), and position.
    - **Special Handling**:
        - Detects "gi" initials (e.g., "giống") to correctly identify which vowels are part of the nucleus vs the initial consonant.
        - Excludes 'i' in "gi" unless it forms a valid diphthong (like "gia").

## Structural Analysis

- **`has_final_consonant(buf: &Buffer, after_pos: usize) -> bool`**
    - Checks if there are any consonant characters in the buffer after the specified position.
    - Used to determine if a vowel is open or closed (important for tone placement rules).

- **`has_qu_initial(buf: &Buffer) -> bool`**
    - Checks if the buffer starts with the "qu" pattern.
    - Helps distinguish 'u' as a medial glide vs a nucleus vowel.

- **`has_gi_initial(buf: &Buffer) -> bool`**
    - Checks if the buffer starts with "gi" followed by another vowel.
    - Used to handle the unique phonology of "gi".

## Test Utilities (`mod test_utils`)

This module is compiled only for tests (`#[cfg(test)]`) but is exported for use by other modules' integration tests.

- **Key Mapping**
    - `char_to_key(c: char) -> u16`: Converts a char to its keycode (inverse of `key_to_char`).
    - `keys_from_str(s: &str) -> Vec<u16>`: Converts a string into a sequence of keycodes.

- **Typing Simulation**
    - `type_word(e: &mut Engine, input: &str) -> String`: Simulates typing a string into the engine and returns the resulting screen output. Handles special keys like Delete, Esc, Space.
    - `type_word_ext`: Extended version supporting specific VNI-like prefixes for testing raw inputs.

- **Test Runners**
    - `telex(cases: &[(&str, &str)])`: Runs a suite of Telex test cases.
    - `vni(cases: &[(&str, &str)])`: Runs a suite of VNI test cases.
    - `raw_mode(cases: &[(&str, &str)])`: Runs raw mode test cases.
