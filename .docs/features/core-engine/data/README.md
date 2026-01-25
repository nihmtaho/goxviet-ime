# English Dictionary Data (Text Format)

## Overview

This directory contains decoded text versions of the binary English word dictionaries used by GoxViet's English detection engine. These files were decoded from the binary format stored in `core/src/engine_v2/english/data/`.

## Files

The dictionaries are organized by word length:

| File | Description | Word Count |
| ---- | ----------- | ---------- |
| `common_2chars.txt` | 2-character English words | 461 |
| `common_3chars.txt` | 3-character English words | 3,975 |
| `common_4chars.txt` | 4-character English words | 5,867 |
| `common_5chars.txt` | 5-character English words | 9,699 |
| `common_6chars.txt` | 6-character English words | 12,923 |
| `common_7chars.txt` | 7-character English words | 14,115 |
| `common_8chars.txt` | 8-character English words | 13,074 |
| `common_9chars.txt` | 9-character English words | 10,883 |
| `common_10chars.txt` | 10-character English words | 8,403 |
| `common_11chars.txt` | 11-character English words | 5,837 |
| `common_12chars.txt` | 12-character English words | 3,762 |
| `common_13chars.txt` | 13-character English words | 2,364 |
| `common_14chars.txt` | 14-character English words | 1,264 |
| `common_15chars.txt` | 15-character English words | 721 |
| `common_16chars.txt` | 16-character English words | 317 |

**Total:** 93,665 English words

## Binary Format

The original binary files store words as sequences of key codes (u16, little-endian), based on the key mapping from `core/src/engine_v2/english/keys.rs`:

```rust
KEY_MAP = {
    'a': 0, 's': 1, 'd': 2, 'f': 3, 'h': 4, 'g': 5, 'z': 6, 'x': 7, 
    'c': 8, 'v': 9, 'b': 11, 'q': 12, 'w': 13, 'e': 14, 'r': 15, 
    'y': 16, 't': 17, 'o': 31, 'u': 32, 'i': 34, 'p': 35, 'l': 37, 
    'j': 38, 'k': 40, 'n': 45, 'm': 46
}
```

Each word is encoded as N key codes (where N is the word length), sorted by key sequence for efficient binary search during lookup.

## Word Selection

These words were carefully selected from English corpora with the following filters:

1. **Manual Blacklist:** Telex keys, tones, and common Vietnamese-English conflicts
2. **Conflict Check:** Excludes words that transform into valid Vietnamese unigrams
3. **Heuristic Filters:** Removes unlikely English words (e.g., ending in 'j', 'z', invalid 'f' patterns)

See `generate_optimized_dictionary.py` for the complete selection logic.

## Usage

These text files are for human reference only. The engine loads the binary versions directly for performance:

- **O(log n) lookup** via binary search on sorted key sequences
- **Minimal memory footprint** (compressed u16 encoding)
- **Zero allocations** during lookup

To regenerate the binary files from source, run:

```bash
python3 generate_optimized_dictionary.py
```

To decode binary files back to text (this directory):

```bash
python3 /tmp/decode_bin_dict.py
```

## See Also

- [Engine V2 English Detection](../engine_v2/english.md)
- [Vietnamese-English Bilingual Logic](../../../ADDING_ENGLISH_WORDS.md)
- Source code: `core/src/engine_v2/english/`
