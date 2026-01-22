# Vietnamese Processing (`engine/vietnamese/`)

This directory contains the logic for Vietnamese phonology, transformations, and validation.

## Transformations (`transform.rs`)

Handles the application of input modifiers to the buffer.

- **Pattern-Based**: Functions like `apply_tone` and `apply_mark` scan the entire buffer to find valid targets based on the input method rules, rather than assuming the cursor position is the target.
- **`apply_tone`**: Adds circumflex ($aa, ee, oo$), horn ($w$), or breve ($aw$).
    - *Example*: In `duow`, applying `w` targets both `u` and `o` to create `ươ` (u-horn, o-horn).
- **`apply_mark`**: Adds tone marks (sắc, huyền, etc.).
    - Uses `tone_positioning` to find the correct vowel to place the mark on.
- **`apply_stroke`**: Scans for `d` to convert to `đ`.

## Tone Positioning (`tone_positioning.rs`)

Implements the complex rules for where to place tone marks in Vietnamese.

**Core Principle**: Tone placement is determined by **phonology**, not typing order.

### Rules (Priority Order)
1.  **Diacritic Priority**: If a vowel has a diacritic (â, ê, ô, ơ, ư), it gets the tone mark.
    - *Example*: `viết` (on ê), `lưỡng` (on ơ).
2.  **Second Vowel Rule**: If no diacritics, the mark usually goes on the second vowel of a cluster.
    - *Example*: `hoá` (on a), `tuý` (on y).
3.  **Final Consonant**: Helper rules for when a final consonant locks the tone position.

The `reposition_mark` function is called whenever the buffer changes (e.g. adding a circumflex to `e` in `vie` -> `viê`) to ensure the mark moves to the correct vowel (e.g. from `i` to `ê` in `viết`).

## Syllable Parsing (`syllable.rs`)

Parses the buffer into the structural components of a Vietnamese syllable:
`[Initial Consonant] [Glide] [Vowel Nucleus] [Final Consonant]`

- **`Syllable` Struct**: Contains indices for each component.
- **Parsing Logic**: Uses "longest-match-first" from the start of the buffer.
- **Special Cases**:
    - `gi` and `qu` handling: `gi` can be an initial consonant (e.g. `già`) or part of `g`+`i` (e.g. `ghi`). `qu` is treated as a unit.
- **Usage**: Used primarily for validation (checking if a word structure is permissible) and for some transformation logic.

## Validation (`validation.rs`)

(Note: While not fully detailed in the viewed source, this module typically uses the `Syllable` parser to check against valid Vietnamese syllable structures to prevent invalid words like `bca`, `kx`, etc.)
