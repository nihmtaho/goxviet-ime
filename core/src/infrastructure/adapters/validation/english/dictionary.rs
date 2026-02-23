// dictionary.rs
//
// Optimized dictionary for common words and programming terms
// Uses linear search on sorted arrays for O(n) performance with high cache locality.

use crate::data::keys;
use crate::infrastructure::adapters::validation::english::dictionary_data;

/// Optimized dictionary for common words
pub struct Dictionary;

impl Dictionary {
    /// Check if a sequence of keys is a common English word or programming term
    pub fn is_english(keys: &[u16]) -> bool {
        if keys.len() < 2 {
            return false;
        }

        // Fast path for callers that already have plain key slices
        is_keys_english(keys)
    }

    /// Check if raw keystroke sequence matches a COMMON English word exactly
    pub fn is_common_english_word(raw_keys: &[(u16, bool)]) -> bool {
        let len = raw_keys.len();
        if len < 2 {
            return false;
        }

        // Avoid heap allocs for short words (common case)
        if len <= 16 {
            let mut buf = [0u16; 16];
            for i in 0..len {
                buf[i] = raw_keys[i].0;
            }
            return is_keys_english(&buf[..len]);
        }

        let mut keys_only = Vec::with_capacity(len);
        for (k, _) in raw_keys.iter() {
            keys_only.push(*k);
        }

        is_keys_english(&keys_only)
    }
}

#[inline]
fn is_keys_english(keys_only: &[u16]) -> bool {
    // MANUAL PATCH: "of", "off", "hex" (common words missing from binary)
    if (keys_only.len() == 2 && keys_only[0] == keys::O && keys_only[1] == keys::F)
        || (keys_only.len() == 3
            && keys_only[0] == keys::O
            && keys_only[1] == keys::F
            && keys_only[2] == keys::F)
        || (keys_only.len() == 3
            && keys_only[0] == keys::H
            && keys_only[1] == keys::E
            && keys_only[2] == keys::X)
    {
        return true;
    }

    // Check common word patterns (these are unambiguous English)
    has_common_english_word_pattern(keys_only) || has_programming_term_pattern(keys_only)
}

/// Constant lookup table for 4-letter programming terms
const PROG_TERMS_4: &[[u16; 4]] = &[
    [keys::F, keys::U, keys::N, keys::C], // func
    [keys::P, keys::R, keys::O, keys::P], // prop
    [keys::A, keys::R, keys::G, keys::S], // args
    [keys::S, keys::E, keys::L, keys::F], // self
    [keys::N, keys::O, keys::N, keys::E], // none
    [keys::S, keys::O, keys::M, keys::E], // some
    [keys::D, keys::E, keys::F, keys::S], // defs
    [keys::I, keys::N, keys::I, keys::T], // init
    [keys::M, keys::A, keys::I, keys::N], // main
    [keys::E, keys::X, keys::I, keys::T], // exit
    [keys::P, keys::A, keys::T, keys::H], // path
    [keys::A, keys::P, keys::P, keys::S], // apps
    [keys::T, keys::E, keys::M, keys::P], // temp
    [keys::C, keys::O, keys::P, keys::Y], // copy
    [keys::M, keys::O, keys::V, keys::E], // move
    [keys::P, keys::U, keys::S, keys::H], // push
    [keys::P, keys::U, keys::L, keys::L], // pull
    [keys::H, keys::A, keys::S, keys::H], // hash
    [keys::J, keys::S, keys::O, keys::N], // json
    [keys::Y, keys::A, keys::M, keys::L], // yaml
    [keys::H, keys::T, keys::M, keys::L], // html
    [keys::H, keys::T, keys::T, keys::P], // http
    [keys::U, keys::U, keys::I, keys::D], // uuid
];

/// Constant lookup table for 5-letter programming terms
const PROG_TERMS_5: &[[u16; 5]] = &[
    [keys::P, keys::R, keys::I, keys::N, keys::T], // print
    [keys::D, keys::E, keys::B, keys::U, keys::G], // debug
    [keys::S, keys::L, keys::E, keys::E, keys::P], // sleep
    [keys::S, keys::P, keys::A, keys::W, keys::N], // spawn
    [keys::Y, keys::I, keys::E, keys::L, keys::D], // yield
    [keys::T, keys::R, keys::A, keys::I, keys::T], // trait
    [keys::S, keys::T, keys::R, keys::U, keys::C], // struc
    [keys::U, keys::N, keys::I, keys::O, keys::N], // union
    [keys::T, keys::U, keys::P, keys::L, keys::E], // tuple
    [keys::A, keys::R, keys::R, keys::A, keys::Y], // array
    [keys::S, keys::L, keys::I, keys::C, keys::E], // slice
    [keys::R, keys::A, keys::N, keys::G, keys::E], // range
    [keys::T, keys::E, keys::L, keys::E, keys::X], // telex
    [keys::C, keys::L, keys::O, keys::N, keys::E], // clone
    [keys::C, keys::A, keys::T, keys::C, keys::H], // catch
    [keys::T, keys::H, keys::R, keys::O, keys::W], // throw
    [keys::F, keys::I, keys::N, keys::A, keys::L], // final
    [keys::S, keys::U, keys::P, keys::E, keys::R], // super
    [keys::F, keys::L, keys::O, keys::A, keys::T], // float
    [keys::I, keys::N, keys::T, keys::E, keys::R], // inter
    [keys::P, keys::A, keys::R, keys::S, keys::E], // parse
    [keys::F, keys::E, keys::T, keys::C, keys::H], // fetch
    [keys::P, keys::A, keys::T, keys::C, keys::H], // patch
    [keys::M, keys::E, keys::R, keys::G, keys::E], // merge
    [keys::S, keys::P, keys::L, keys::I, keys::T], // split
];

/// Constant lookup table for 6-letter programming terms
const PROG_TERMS_6: &[[u16; 6]] = &[
    [keys::S, keys::T, keys::R, keys::U, keys::C, keys::T], // struct
    [keys::D, keys::O, keys::U, keys::B, keys::L, keys::E], // double
    [keys::S, keys::Y, keys::N, keys::T, keys::A, keys::X], // syntax
    [keys::S, keys::C, keys::H, keys::E, keys::M, keys::A], // schema
    [keys::B, keys::U, keys::F, keys::F, keys::E, keys::R], // buffer
    [keys::S, keys::O, keys::C, keys::K, keys::E, keys::T], // socket
    [keys::S, keys::E, keys::R, keys::V, keys::E, keys::R], // server
    [keys::C, keys::L, keys::I, keys::E, keys::N, keys::T], // client
    [keys::T, keys::A, keys::R, keys::G, keys::E, keys::T], // target
    [keys::B, keys::U, keys::I, keys::L, keys::D, keys::S], // builds
    [keys::D, keys::E, keys::P, keys::L, keys::O, keys::Y], // deploy
    [keys::C, keys::O, keys::N, keys::F, keys::I, keys::G], // config
    [keys::C, keys::O, keys::M, keys::M, keys::I, keys::T], // commit
    [keys::B, keys::R, keys::A, keys::N, keys::C, keys::H], // branch
];

/// Constant lookup table for 7-letter programming terms
const PROG_TERMS_7: &[[u16; 7]] = &[
    [
        keys::D,
        keys::E,
        keys::F,
        keys::A,
        keys::U,
        keys::L,
        keys::T,
    ], // default
    [
        keys::B,
        keys::O,
        keys::O,
        keys::L,
        keys::E,
        keys::A,
        keys::N,
    ], // boolean
    [
        keys::C,
        keys::O,
        keys::N,
        keys::S,
        keys::O,
        keys::L,
        keys::E,
    ], // console
    [
        keys::I,
        keys::N,
        keys::T,
        keys::E,
        keys::G,
        keys::E,
        keys::R,
    ], // integer
    [
        keys::P,
        keys::A,
        keys::C,
        keys::K,
        keys::A,
        keys::G,
        keys::E,
    ], // package
    [
        keys::R,
        keys::E,
        keys::Q,
        keys::U,
        keys::I,
        keys::R,
        keys::E,
    ], // require
    [
        keys::I,
        keys::N,
        keys::C,
        keys::L,
        keys::U,
        keys::D,
        keys::E,
    ], // include
    [
        keys::P,
        keys::R,
        keys::I,
        keys::V,
        keys::A,
        keys::T,
        keys::E,
    ], // private
    [
        keys::E,
        keys::X,
        keys::T,
        keys::E,
        keys::N,
        keys::D,
        keys::S,
    ], // extends
    [
        keys::P,
        keys::R,
        keys::O,
        keys::M,
        keys::I,
        keys::S,
        keys::E,
    ], // promise
];

/// Constant lookup table for 8-letter programming terms
const PROG_TERMS_8: &[[u16; 8]] = &[
    [
        keys::F,
        keys::U,
        keys::N,
        keys::C,
        keys::T,
        keys::I,
        keys::O,
        keys::N,
    ], // function
    [
        keys::A,
        keys::B,
        keys::S,
        keys::T,
        keys::R,
        keys::A,
        keys::C,
        keys::T,
    ], // abstract
    [
        keys::C,
        keys::O,
        keys::N,
        keys::T,
        keys::I,
        keys::N,
        keys::U,
        keys::E,
    ], // continue
    [
        keys::P,
        keys::R,
        keys::O,
        keys::P,
        keys::E,
        keys::R,
        keys::T,
        keys::Y,
    ], // property
    [
        keys::T,
        keys::E,
        keys::M,
        keys::P,
        keys::L,
        keys::A,
        keys::T,
        keys::E,
    ], // template
];

#[inline]
fn is_prog_term_4(word: &[u16; 4]) -> bool {
    PROG_TERMS_4.iter().any(|w| w == word)
}

#[inline]
fn is_prog_term_5(word: &[u16; 5]) -> bool {
    PROG_TERMS_5.iter().any(|w| w == word)
}

#[inline]
fn is_prog_term_6(word: &[u16; 6]) -> bool {
    PROG_TERMS_6.iter().any(|w| w == word)
}

#[inline]
fn is_prog_term_7(word: &[u16; 7]) -> bool {
    PROG_TERMS_7.iter().any(|w| w == word)
}

#[inline]
fn is_prog_term_8(word: &[u16; 8]) -> bool {
    PROG_TERMS_8.iter().any(|w| w == word)
}

#[inline]
fn has_common_english_word_pattern(keys: &[u16]) -> bool {
    let len = keys.len();
    if len < 2 {
        return false;
    }

    // Strategy 1: Check Exact Length Match
    // We only want to trigger if the *entire* word is in the dictionary.
    // Checking prefixes (e.g. checking 2-letter dict for 3-letter word) causes false positives
    // Example: "bên" (VN) starts with "be" (EN). If we check prefix, "bên" is marked English.

    if len == 2 {
        let w2: [u16; 2] = [keys[0], keys[1]];
        if dictionary_data::is_common_2letter_word(&w2) {
            return true;
        }
    } else if len == 3 {
        let w3: [u16; 3] = [keys[0], keys[1], keys[2]];
        if dictionary_data::is_common_3letter_word(&w3) {
            return true;
        }
    } else if len == 4 {
        let w4: [u16; 4] = [keys[0], keys[1], keys[2], keys[3]];
        if dictionary_data::is_common_4letter_word(&w4) {
            return true;
        }
    } else if len == 5 {
        let w5: [u16; 5] = [keys[0], keys[1], keys[2], keys[3], keys[4]];
        if dictionary_data::is_common_5letter_word(&w5) {
            return true;
        }
    } else if len == 6 {
        let w6: [u16; 6] = [keys[0], keys[1], keys[2], keys[3], keys[4], keys[5]];
        if dictionary_data::is_common_6letter_word(&w6) {
            return true;
        }
    } else if len == 7 {
        let w7: [u16; 7] = [
            keys[0], keys[1], keys[2], keys[3], keys[4], keys[5], keys[6],
        ];
        if dictionary_data::is_common_7letter_word(&w7) {
            return true;
        }
    } else if len == 8 {
        let w8: [u16; 8] = [
            keys[0], keys[1], keys[2], keys[3], keys[4], keys[5], keys[6], keys[7],
        ];
        if dictionary_data::is_common_8letter_word(&w8) {
            return true;
        }
    } else if len == 9 {
        let w9: [u16; 9] = [
            keys[0], keys[1], keys[2], keys[3], keys[4], keys[5], keys[6], keys[7], keys[8],
        ];
        if dictionary_data::is_common_9letter_word(&w9) {
            return true;
        }
    } else if len == 10 {
        let w10: [u16; 10] = [
            keys[0], keys[1], keys[2], keys[3], keys[4], keys[5], keys[6], keys[7], keys[8],
            keys[9],
        ];
        if dictionary_data::is_common_10letter_word(&w10) {
            return true;
        }
    } else if len == 11 {
        let w11: [u16; 11] = [
            keys[0], keys[1], keys[2], keys[3], keys[4], keys[5], keys[6], keys[7], keys[8],
            keys[9], keys[10],
        ];
        if dictionary_data::is_common_11letter_word(&w11) {
            return true;
        }
    } else if len == 12 {
        let w12: [u16; 12] = [
            keys[0], keys[1], keys[2], keys[3], keys[4], keys[5], keys[6], keys[7], keys[8],
            keys[9], keys[10], keys[11],
        ];
        if dictionary_data::is_common_12letter_word(&w12) {
            return true;
        }
    } else if len == 13 {
        let w13: [u16; 13] = [
            keys[0], keys[1], keys[2], keys[3], keys[4], keys[5], keys[6], keys[7], keys[8],
            keys[9], keys[10], keys[11], keys[12],
        ];
        if dictionary_data::is_common_13letter_word(&w13) {
            return true;
        }
    } else if len == 14 {
        let w14: [u16; 14] = [
            keys[0], keys[1], keys[2], keys[3], keys[4], keys[5], keys[6], keys[7], keys[8],
            keys[9], keys[10], keys[11], keys[12], keys[13],
        ];
        if dictionary_data::is_common_14letter_word(&w14) {
            return true;
        }
    } else if len == 15 {
        let w15: [u16; 15] = [
            keys[0], keys[1], keys[2], keys[3], keys[4], keys[5], keys[6], keys[7], keys[8],
            keys[9], keys[10], keys[11], keys[12], keys[13], keys[14],
        ];
        if dictionary_data::is_common_15letter_word(&w15) {
            return true;
        }
    } else if len >= 16 {
        // For very long words, check the first 16 chars
        let w16: [u16; 16] = [
            keys[0], keys[1], keys[2], keys[3], keys[4], keys[5], keys[6], keys[7], keys[8],
            keys[9], keys[10], keys[11], keys[12], keys[13], keys[14], keys[15],
        ];
        if dictionary_data::is_common_16letter_word(&w16) {
            return true;
        }
    }

    false
}

#[inline]
fn has_programming_term_pattern(keys: &[u16]) -> bool {
    let len = keys.len();
    if len < 4 {
        return false;
    }
    let check_len = len.min(8);

    if check_len >= 4 {
        let w4: [u16; 4] = [keys[0], keys[1], keys[2], keys[3]];
        if is_prog_term_4(&w4) {
            return true;
        }
    }
    if check_len >= 5 {
        let w5: [u16; 5] = [keys[0], keys[1], keys[2], keys[3], keys[4]];
        if is_prog_term_5(&w5) {
            return true;
        }
    }
    if check_len >= 6 {
        let w6: [u16; 6] = [keys[0], keys[1], keys[2], keys[3], keys[4], keys[5]];
        if is_prog_term_6(&w6) {
            return true;
        }
    }
    if check_len >= 7 {
        let w7: [u16; 7] = [
            keys[0], keys[1], keys[2], keys[3], keys[4], keys[5], keys[6],
        ];
        if is_prog_term_7(&w7) {
            return true;
        }
    }
    if check_len >= 8 {
        let w8: [u16; 8] = [
            keys[0], keys[1], keys[2], keys[3], keys[4], keys[5], keys[6], keys[7],
        ];
        if is_prog_term_8(&w8) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_english_are() {
        let keys = [keys::A, keys::R, keys::E];
        assert!(Dictionary::is_english(&keys));
    }
}

#[test]
fn test_is_english_off() {
    let keys = [keys::O, keys::F, keys::F];
    println!("Keys: {:?}", keys);
    assert!(Dictionary::is_english(&keys));
}
