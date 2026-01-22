# Engine V2 Overview

The `engine_v2` module represents the second generation of the core processing engine, focusing on performance, modularity, and more sophisticated language detection. It introduces a layered architecture for phonotactic analysis and strict Vietnamese validation.

## Module Structure

- **`english`**: Advanced English detection logic using an 8-layer phonotactic engine and optimized dictionary lookups.
- **`vietnamese_validator`**: Strict validation of Vietnamese syllables to prevent invalid transformations and improve auto-restore accuracy.
- **`fsm`**: Finite State Machine data and tables supporting the validator (bigram matrices, character properties).

## Key Improvements

1.  **Matrix-Based Analysis**: Uses bitmask matrices (`VIETNAMESE_BIGRAMS`) for O(1) validity checks of character pairs.
2.  **Layered Phonotactics**: English detection is no longer just a dictionary check but a multi-layer analysis of consonant clusters, suffixes, and impossible Vietnamese bigrams.
3.  **Strict Validation**: The engine can now definitively reject invalid Vietnamese sequences (e.g., "f", "j" initials, invalid vowel combinations), allowing for more aggressive English restoration when Vietnamese validity is low.
