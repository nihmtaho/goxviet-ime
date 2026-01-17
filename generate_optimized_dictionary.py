import os
import struct
import unicodedata

# Configuration
LIMITS = {
    2: 50,    # Very restrictive for 2 letters
    3: 300,   # Top 300 3-letter words
    4: 1000,  # Top 1000
    5: 1000,
    6: 800,
    7: 500,
    8: 500,
}

# Key mapping from keys.rs
KEY_MAP = {
    'a': 0, 's': 1, 'd': 2, 'f': 3, 'h': 4, 'g': 5, 'z': 6, 'x': 7, 'c': 8, 'v': 9,
    'b': 11, 'q': 12, 'w': 13, 'e': 14, 'r': 15, 'y': 16, 't': 17,
    'o': 31, 'u': 32, 'i': 34, 'p': 35, 'l': 37, 'j': 38, 'k': 40, 'n': 45, 'm': 46
}

# Manual blacklist (Telex keys, tones, common conflicts)
BLACKLIST = {
    "aa", "aw", "ee", "oo", "ow", "uw", "dd", "as", "af", "ar", "ax", "aj",
    "if", "is", "it", "in", "im", "of", "or", "on", "us", "uk", "um", "an", "am", "at", 
    "to", "ti", "tu", "te", "ta", "so", "si", "se", "sa", "su", "bo", "bi", "be", "ba", "bu",
    "co", "ci", "ce", "ca", "cu", "do", "di", "de", "da", "du", "fo", "fi", "fe", "fa", "fu",
    "go", "gi", "ge", "ga", "gu", "ho", "hi", "he", "ha", "hu", "lo", "li", "le", "la", "lu",
    "mo", "mi", "me", "ma", "mu", "no", "ni", "ne", "na", "nu", "po", "pi", "pe", "pa", "pu",
    "ro", "ri", "re", "ra", "ru", "xo", "xi", "xe", "xa", "xu", "yo", "yi", "ye", "ya", "yu",
    "my", "by", "hy", "ky", "ly", "my", "ny", "py", "ry", "sy", "ty", "vy", "xy", "zy"
}

def load_vietnamese_unigrams(path):
    unigrams = set()
    try:
        with open(path, 'r', encoding='utf-8') as f:
            for line in f:
                words = line.strip().lower().split()
                for w in words:
                    # Normalize to NFC
                    w_nfc = unicodedata.normalize('NFC', w)
                    unigrams.add(w_nfc)
        
        # Add single vowels manually if missing
        manual_add = ["ă", "â", "ê", "ô", "ơ", "ư", "đ", "uơ", "ươ"]
        for w in manual_add:
            unigrams.add(unicodedata.normalize('NFC', w))
            
    except FileNotFoundError:
        print(f"Warning: {path} not found.")
    return unigrams

def get_word_key_sequence(word):
    return [KEY_MAP.get(c, 999) for c in word]

def main():
    base_dir = "core/tests/data"
    failures_path = os.path.join(base_dir, "english_100k_failures.txt") # Use source
    vn_dict_path = os.path.join(base_dir, "vietnamese_22k.txt")
    
    # Load VN unigrams
    vn_unigrams = load_vietnamese_unigrams(vn_dict_path)
    print(f"Loaded {len(vn_unigrams)} Vietnamese unigrams.")

    # Process English words
    words_by_len = {i: [] for i in range(2, 9)}
    
    try:
        with open(failures_path, 'r', encoding='utf-8') as f:
            for line in f:
                parts = line.strip().split()
                if len(parts) < 2: continue
                
                english_word = parts[0].lower()
                transformed = parts[1].lower() # This is what the engine produced
                
                # Normalize transformed result
                transformed_nfc = unicodedata.normalize('NFC', transformed)
                
                # Filter Logic
                if not english_word.isalpha(): continue
                if len(english_word) < 2 or len(english_word) > 8: continue
                
                # 1. Manual Blacklist
                if english_word in BLACKLIST: continue
                
                # 2. Conflict Check:
                # If the transformed result is a valid Vietnamese word (unigram),
                # IMPLYING matching input -> valid word. 
                # e.g. "orn" -> "ỏn". "ỏn" in unigrams -> Conflict.
                if transformed_nfc in vn_unigrams:
                    # print(f"Excluded conflict: {english_word} -> {transformed_nfc}")
                    continue
                
                # 3. Heuristic Junk Filter (for words that failed to transform but look like Telex)
                # Exclude words ending in 'j', 'z', or invalid 'f' sequences
                if english_word.endswith('j') or english_word.endswith('z'):
                    continue
                
                if english_word.endswith('f'):
                    # Allowed 'f' endings: ff, lf, rf, vowels+f (if, of)
                    # Disallow: nf, mf, tf, etc.
                    if len(english_word) >= 2:
                        penult = english_word[-2]
                        if penult not in "aeiouylrf":
                            continue
                
                # 3. Exact match check (if dict has "car" and "car" is valid phrase)
                # But English "car" -> "cả" (telex). "cả" in unigrams? Yes. so 'car' excluded above.
                # What if "ba" -> "ba". "ba" IS in unigrams. Excluded.
                
                words_by_len[len(english_word)].append(english_word)
                
    except FileNotFoundError:
        print("Failures file not found.")
        return

    # Sort and write binary
    output_dir = "core/src/engine_v2/english/data"
    os.makedirs(output_dir, exist_ok=True)
    
    for length, words in words_by_len.items():
        # Dedup
        words = list(set(words))
        
        # Sort by Keycode
        words.sort(key=get_word_key_sequence)
        
        # Limit? Or keep all safe ones?
        # User accepted 23k, so keeping safe ones is better than limit.
        # But for optimization, maybe apply limit if too many?
        # Let's keep ALL safe words.
        
        # Write binary file
        bin_path = os.path.join(output_dir, f"common_{length}chars.bin")
        count = 0
        with open(bin_path, 'wb') as fb:
            for w in words:
                key_seq = get_word_key_sequence(w)
                if 999 in key_seq: continue # Invalid keys
                
                # Pack u16 Little Endian
                for k in key_seq:
                    fb.write(struct.pack('<H', k))
                count += 1
        
        print(f"Length {length}: {count} safe words written to {bin_path}")

if __name__ == "__main__":
    main()
