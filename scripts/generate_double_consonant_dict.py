#!/usr/bin/env python3
"""
Generate double consonant word list for triple-tone Telex auto-correction.

Reads english_100k.txt (frequency-sorted), filters for words containing
Telex tone-marker double consonants (ff, ss, rr, xx, jj), and outputs
one word per line sorted alphabetically.

These are used at SPACE boundary to correct triple-tone typos:
  - "assset" → "asset" (sss → ss)
  - "offfer" → "offer" (fff → ff)
  - "corrrect" → "correct" (rrr → rr)

Usage:
  python3 scripts/generate_double_consonant_dict.py
"""

import sys
import os

INPUT_PATH = os.path.join(
    os.path.dirname(__file__), "..", "core", "tests", "data", "english_100k.txt"
)
OUTPUT_PATH = os.path.join(
    os.path.dirname(__file__), "..", "core", "src", "data", "double_consonant_words.txt"
)

# Telex tone-marker consonants that can appear tripled accidentally:
#   s = sắc, f = huyền, r = hỏi, x = ngã, j = nặng
DOUBLE_PATTERNS = ["ff", "ss", "rr", "xx", "jj"]


def has_tone_marker_double(word: str) -> bool:
    return any(pat in word for pat in DOUBLE_PATTERNS)


def main():
    input_path = os.path.abspath(INPUT_PATH)
    output_path = os.path.abspath(OUTPUT_PATH)

    if not os.path.exists(input_path):
        print(f"ERROR: Input file not found: {input_path}", file=sys.stderr)
        sys.exit(1)

    words = set()
    with open(input_path, "r", encoding="utf-8") as f:
        for line in f:
            word = line.strip().lower()
            # Single word only (no spaces), length 3-20, alpha only
            if not word or " " in word or not word.isalpha():
                continue
            if len(word) < 3 or len(word) > 20:
                continue
            if has_tone_marker_double(word):
                words.add(word)

    sorted_words = sorted(words)

    os.makedirs(os.path.dirname(output_path), exist_ok=True)
    with open(output_path, "w", encoding="utf-8") as f:
        for word in sorted_words:
            f.write(word + "\n")

    print(f"Generated {len(sorted_words)} words → {output_path}")


if __name__ == "__main__":
    main()
