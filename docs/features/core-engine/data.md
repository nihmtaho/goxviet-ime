# Data Modules (`data/`)

The `data` directory contains static data and definitions used throughout the engine, including character sets, key mappings, and phonological rules.

## `chars` (`chars.rs`)
Defines the Unicode character maps for Vietnamese.
- **Functions**:
    - `to_char(key, caps, tone, mark)`: Converts internal components into a valid Unicode character (precomposed or decomposed based on standard).
    - `get_d(caps)`: Returns `đ` or `Đ`.
- **Constants**: Tone and mark IDs.

## `keys` (`keys.rs`)
Defines virtual keycodes and helper functions.
- **Constants**: Map for standard keys (`A`=0, `S`=1, etc. - based on macOS/ANSI layout).
- **Helpers**:
    - `is_vowel(key)`: Checks if a key represents a vowel.
    - `is_consonant(key)`: Checks if a key represents a consonant.
    - `is_number(key)`: Checks if a key is a digit.
    - `is_break(key)`: Checks if a key is a delimiter (space, punctuation).

## `vowel` (`vowel.rs`)
Defines the complex phonology of Vietnamese vowels.
- **`Vowel` Struct**: Represents a vowel character with its modifier.
- **`Phonology` Struct**: Rules for valid vowel clusters (diphthongs/triphthongs) and horn compatibility.
- **Logic**: Used to validate if a sequence like `uoe` is valid or if `w` can apply a horn to a specific vowel cluster.

## `constants` (`constants.rs`)
General constants for the engine, such as valid final consonants.
