# English Detection (Engine V2)

The `engine_v2::english` module provides robust English language detection capabilities, essential for the "Auto Restore" feature. It distinguishes intended English words from accidental Vietnamese transformations (e.g., typing "term" vs "tờm").

## Components

### `PhonotacticEngine` (`phonotactic.rs`)

An 8-layer analysis engine that calculates a confidence score for whether a key sequence is English.

#### Detection Layers
1.  **Invalid Initials**: Detects letters that never start Vietnamese words (`F`, `J`, `W`, `Z`, `SH-`).
2.  **Onset Clusters**: Detects English-specific initial clusters (`bl`, `str`, `pl`, `gr`, etc.).
3.  **Double Consonants**: Detects doubled consonants (`ll`, `ss`, `rr`), which are rare or non-existent in Vietnamese.
4.  **Suffixes**: Identifies common English endings (`-tion`, `-ing`, `-ed`, `-ly`, `-ment`).
5.  **Coda Clusters**: Checks for valid English ending clusters (`st`, `nd`, `ct`, `mp`).
6.  **Prefixes**: Identifies English prefixes (`un-`, `re-`, `pre-`, `dis-`).
7.  **Vowel Patterns**: Detects English vowel digraphs (`ea`, `ou`).
8.  **Impossible Bigrams**: Identifies character pairs impossible in Vietnamese (`qb`, `zf`, etc.).

#### Confidence Calculation
The engine returns a `PhonotacticResult` containing:
-   `layer_scores`: Individual confidence from each layer.
-   `matched_layers`: Bitmask of which layers triggered.
-   `english_confidence`: A weighted average score (0-100%).

### `Dictionary` (`dictionary.rs`)

A highly optimized, O(1) dictionary lookup for:
-   **Common English Words**: High-frequency words (e.g., "the", "and", "that").
-   **Programming Terms**: Reserved keywords and common terms (e.g., "const", "print", "function", "array").

This module avoids heap allocations for short words by using stack arrays and static lookup tables (`PROG_TERMS_4`, `PROG_TERMS_5`, etc.).

### `LanguageDecisionEngine` (`language_decision.rs`)

The central decision-maker that combines signals from:
1.  **Dictionary Lookup**: Highest priority (100% confidence).
2.  **Vietnamese Validation**: Penalizes English score if the input forms a valid Vietnamese syllable.
3.  **Phonotactic Analysis**: Adds to the English confidence score.
4.  **Diacritics**: Presence of explicit Vietnamese marks (ê, ư, tone marks) heavily penalizes the English score.

It outputs a `DecisionResult` with a final boolean `is_english` and a confidence score.
