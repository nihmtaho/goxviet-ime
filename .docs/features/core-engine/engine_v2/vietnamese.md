# Vietnamese Validation (Engine V2)

The validation system ensures that the engine only produces phonologically valid Vietnamese syllables. This is crucial for distinguishing between typing errors, English words, and valid Vietnamese text. By identifying valid Vietnamese syllables, the engine can apply more conservative auto-restore thresholds (e.g., 95%) to protect intended Vietnamese text from accidental restoration during rapid typing.

## `VietnameseSyllableValidator` (`vietnamese_validator.rs`)

Provides O(1) static methods for validating key sequences.

### Core Validation Rules

1.  **Initial Consonants**: Checks against a strict set of allowed initials. Rejecting `F`, `J`, `W`, `Z` (unless part of specific loanword handling).
2.  **Cluster Constraints**: Rejects invalid initial clusters like `bl`, `fr`, `sk` (English clusters).
3.  **Bigram Check**: Uses the `fsm` tables to verify that every adjacent pair of characters is phonotactically allowed (e.g., `q` must be followed by `u`).
4.  **Distribution Rules**: Enforces C/K/G/GH rules (e.g., `k` goes with `i/e/ê`, `c` with others).
5.  **Vowel Sequences**: Validates vowel clusters against known Vietnamese diphthongs and triphthongs (e.g., `uyê` is valid, `aae` is not).
6.  **Coda Compatibility**: Ensures the final consonant matches the vowel nucleus (e.g., `a` can go with `ch`, but `u` cannot go with `ch` without a diacritic).

### Tone Integration (`validate_with_tones`)
Ensures tone marks are placed on valid vowels and do not violate specific rules:
-   **E+U**: `eu` is invalid, must be `êu`.
-   **Horn Rules**: `u`+`horn` (`ư`) is only valid in specific contexts (e.g., `ươ`, `ưi`).
-   **Breve**: `ă` cannot be combined with certain endings.

## Finite State Machine (`fsm`)

The `fsm` module provides the data structures that power validaton optimization.

### Tables (`fsm/tables`)

-   **`CHAR_PROPS`**: A 128-element array mapping ASCII characters to their properties (Vowel, Consonant, Invalid Initial, Invalid Coda). Allows O(1) property checks.
-   **`VIETNAMESE_BIGRAMS`**: A matrix where `matrix[char1]` returns a bitmask of all allowed `char2` successors.
    -   `row = VIETNAMESE_BIGRAMS[c1]`
    -   `is_valid = (row & (1 << c2)) != 0`
    -   This allows checking the validity of any character pair in a single bitwise operation.
