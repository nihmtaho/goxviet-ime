use std::cmp::Ordering;

/// Binary search a u16 array pattern within a byte slice (Little Endian)
/// Assumes data contains sequence of words, each of `word_len` length (in u16s)
/// So each word is `word_len * 2` bytes.
fn binary_search_in_bytes(data: &[u8], target: &[u16]) -> bool {
    let word_len = target.len();
    let word_size_bytes = word_len * 2;
    let num_words = data.len() / word_size_bytes;

    if num_words == 0 {
        return false;
    }

    let mut left = 0;
    let mut right = num_words;

    while left < right {
        let mid = left + (right - left) / 2;
        let offset = mid * word_size_bytes;

        // Read word at mid
        let mut match_ordering = Ordering::Equal;

        for i in 0..word_len {
            let b1 = data[offset + i * 2];
            let b2 = data[offset + i * 2 + 1];
            let val = u16::from_le_bytes([b1, b2]);

            match val.cmp(&target[i]) {
                Ordering::Equal => continue,
                ord => {
                    match_ordering = ord;
                    break;
                }
            }
        }

        match match_ordering {
            Ordering::Equal => {
                return true;
            }
            Ordering::Less => left = mid + 1,
            Ordering::Greater => right = mid,
        }
    }

    false
}

static DATA_2: &[u8] = include_bytes!("data/common_2chars.bin");
static DATA_3: &[u8] = include_bytes!("data/common_3chars.bin");
static DATA_4: &[u8] = include_bytes!("data/common_4chars.bin");
static DATA_5: &[u8] = include_bytes!("data/common_5chars.bin");
static DATA_6: &[u8] = include_bytes!("data/common_6chars.bin");
static DATA_7: &[u8] = include_bytes!("data/common_7chars.bin");
static DATA_8: &[u8] = include_bytes!("data/common_8chars.bin");
static DATA_9: &[u8] = include_bytes!("data/common_9chars.bin");
static DATA_10: &[u8] = include_bytes!("data/common_10chars.bin");
static DATA_11: &[u8] = include_bytes!("data/common_11chars.bin");
static DATA_12: &[u8] = include_bytes!("data/common_12chars.bin");
static DATA_13: &[u8] = include_bytes!("data/common_13chars.bin");
static DATA_14: &[u8] = include_bytes!("data/common_14chars.bin");
static DATA_15: &[u8] = include_bytes!("data/common_15chars.bin");
static DATA_16: &[u8] = include_bytes!("data/common_16chars.bin");

#[inline]
pub fn is_common_2letter_word(word: &[u16; 2]) -> bool {
    binary_search_in_bytes(DATA_2, word)
}

#[inline]
pub fn is_common_3letter_word(word: &[u16; 3]) -> bool {
    binary_search_in_bytes(DATA_3, word)
}

#[inline]
pub fn is_common_4letter_word(word: &[u16; 4]) -> bool {
    binary_search_in_bytes(DATA_4, word)
}

#[inline]
pub fn is_common_5letter_word(word: &[u16; 5]) -> bool {
    binary_search_in_bytes(DATA_5, word)
}

#[inline]
pub fn is_common_6letter_word(word: &[u16; 6]) -> bool {
    binary_search_in_bytes(DATA_6, word)
}

#[inline]
pub fn is_common_7letter_word(word: &[u16; 7]) -> bool {
    binary_search_in_bytes(DATA_7, word)
}

#[inline]
pub fn is_common_8letter_word(word: &[u16; 8]) -> bool {
    binary_search_in_bytes(DATA_8, word)
}

#[inline]
pub fn is_common_9letter_word(word: &[u16; 9]) -> bool {
    binary_search_in_bytes(DATA_9, word)
}

#[inline]
pub fn is_common_10letter_word(word: &[u16; 10]) -> bool {
    binary_search_in_bytes(DATA_10, word)
}

#[inline]
pub fn is_common_11letter_word(word: &[u16; 11]) -> bool {
    binary_search_in_bytes(DATA_11, word)
}

#[inline]
pub fn is_common_12letter_word(word: &[u16; 12]) -> bool {
    binary_search_in_bytes(DATA_12, word)
}

#[inline]
pub fn is_common_13letter_word(word: &[u16; 13]) -> bool {
    binary_search_in_bytes(DATA_13, word)
}

#[inline]
pub fn is_common_14letter_word(word: &[u16; 14]) -> bool {
    binary_search_in_bytes(DATA_14, word)
}

#[inline]
pub fn is_common_15letter_word(word: &[u16; 15]) -> bool {
    binary_search_in_bytes(DATA_15, word)
}

#[inline]
pub fn is_common_16letter_word(word: &[u16; 16]) -> bool {
    binary_search_in_bytes(DATA_16, word)
}
