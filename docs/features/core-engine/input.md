# Input Methods (`input/`)

The `input` module defines the key mappings and behaviors for the supported input methods: Telex and VNI. The engine uses these definitions to interpret keystrokes as tones, marks, or modifications.

## Core Traits (`mod.rs`)

### `Method` Trait
Defines the interface that every input method must implement:

- **`mark(key: u16) -> Option<u8>`**: Checks if a key acts as a mark modifier (e.g., adding a hat). Returns the mark type ID (using constants from `data::chars::mark`).
- **`tone(key: u16) -> Option<ToneType>`**: Checks if a key acts as a vowel modifier (Circumflex, Horn, Breve).
- **`tone_targets(key: u16) -> &'static [u16]`**: Returns which vowels the tone modifier applies to.
- **`stroke(key: u16) -> bool`**: Checks if the key triggers a "stroke" modification (specifically `d` -> `đ`).
- **`remove(key: u16) -> bool`**: Checks if the key is a "reset" key that removes diacritics (e.g., 'z' in Telex, '0' in VNI).

### `ToneType` Enum
Classifies the type of modification a key performs on a vowel:
- `Circumflex`: Adds a hat (â, ê, ô).
- `Horn`: Adds a horn (ơ, ư).
- `Breve`: Adds a smile (ă).

## Implementations

### Telex (`telex.rs`)
The standard Telex input method.

- **Tones**:
    - `S`: Sắc (Acute)
    - `F`: Huyền (Grave)
    - `R`: Hỏi (Hook)
    - `X`: Ngã (Tilde)
    - `J`: Nặng (Dot)
- **Marks**:
    - `A`, `W`: Breve (ă) - mapped via `mark` or `tone` depending on context.
    - `E`, `O`: Circumflex (ê, ô).
    - `W`: Horn (ư, ơ).
- **Stroke**: `D` maps to `đ`.
- **Remove**: `Z` removes tone marks.

### VNI (`vni.rs`)
The number-based VNI input method.

- **Tones (Numeric)**:
    - `1`: Sắc
    - `2`: Huyền
    - `3`: Hỏi
    - `4`: Ngã
    - `5`: Nặng
- **Marks**:
    - `6`: Circumflex (â, ê, ô)
    - `7`: Horn (ơ, ư)
    - `8`: Breve (ă)
    - `9`: Stroke (đ)
- **Remove**: `0` removes tone marks.

## Usage
The global function `get(id: u8) -> &'static dyn Method` returns the method instance based on the ID (`0` for Telex, `1` for VNI).
