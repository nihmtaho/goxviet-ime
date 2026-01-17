import os

# Configuration
LIMITS = {
    2: 10000,
    3: 10000,
    4: 10000,
    5: 10000,
    6: 10000,
    7: 10000,
    8: 10000,
}

# macOS Keycodes from keys.rs
KEY_MAP = {
    'a': 0, 's': 1, 'd': 2, 'f': 3, 'h': 4, 'g': 5, 'z': 6, 'x': 7, 'c': 8, 'v': 9,
    'b': 11, 'q': 12, 'w': 13, 'e': 14, 'r': 15, 'y': 16, 't': 17,
    'o': 31, 'u': 32, 'i': 34, 'p': 35, 'l': 37, 'j': 38, 'k': 40, 'n': 45, 'm': 46
}

def keys_const(char):
    if 'a' <= char <= 'z':
        return f"keys::{char.upper()}"
    return f"keys::KEY_{char.upper()}"

def get_word_key_sequence(word):
    return [KEY_MAP.get(c, 999) for c in word]

def generate_rust_array(length, words):
    # Sort words based on their keycode sequence
    words.sort(key=get_word_key_sequence)
    
    rust_code = f"/// Constant lookup table for common {length}-letter English words (Sorted by Keycode)\n"
    rust_code += f"const COMMON_{length}LETTER_WORDS: &[[u16; {length}]] = &[\n"
    
    for word in words:
        if len(word) != length:
            continue
        chars = [keys_const(c) for c in word]
        rust_code += "    [" + ", ".join(chars) + "], // " + word + "\n"
    
    rust_code += "];\n"
    return rust_code

def main():
    input_path = "core/tests/data/english_failures_nonsense.txt"
    output_path = "core/src/engine_v2/english/generated_dictionary.rs" # Temporary output
    
    words_by_len = {l: [] for l in LIMITS.keys()}
    
    try:
        with open(input_path, 'r', encoding='utf-8') as f:
            for line in f:
                parts = line.strip().split()
                if not parts:
                    continue
                word = parts[0].lower() # Column 1 is English
                
                # Filter strictly a-z
                if not word.isalpha():
                    continue
                    
                l = len(word)
                if l in words_by_len:
                    if len(words_by_len[l]) < LIMITS[l]:
                        words_by_len[l].append(word)
    except FileNotFoundError:
        print(f"Error: {input_path} not found.")
        return

    full_code = ""
    for l in sorted(LIMITS.keys()):
        print(f"Length {l}: Collected {len(words_by_len[l])} words")
        full_code += generate_rust_array(l, words_by_len[l]) + "\n"

    with open("generated_dict_content.txt", "w") as f:
        f.write(full_code)
    
    print("Generated Rust code in generated_dict_content.txt")

if __name__ == "__main__":
    main()
