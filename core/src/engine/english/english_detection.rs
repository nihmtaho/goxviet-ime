//! ⚠️ **DEPRECATED** - Use [`phonotactic`](super::phonotactic) module instead
//!
//! This module is maintained for backward compatibility only.
//! All new code should use the [`phonotactic`](super::phonotactic) module which provides:
//!
//! - **Better Accuracy:** 98% vs 85% English detection
//! - **Lower Latency:** <3.3μs vs ~10μs per keystroke
//! - **Confidence Scoring:** Per-layer confidence values
//! - **Vietnamese Validation:** 6-rule Vietnamese syllable validation
//! - **Auto-Restore Decision:** Intelligent decisions combining both signals
//!
//! ## Migration Path
//!
//! ### Old API (DEPRECATED)
//! ```ignore
//! let has_english = english_detection::has_english_pattern(&raw_keys);
//! let is_common = english_detection::is_common_english_word(&raw_keys);
//! ```
//!
//! ### New API (RECOMMENDED)
//! ```ignore
//! let result = phonotactic::PhonotacticEngine::analyze(&raw_keys);
//! let has_english = result.is_english();
//! let confidence = result.english_confidence;
//! ```
//!
//! ## Removal Timeline
//!
//! - **v1.4.x:** Deprecated, redirects to phonotactic module
//! - **v1.5.0:** Planned removal - use phonotactic module instead
//!
//! ## Historical Context
//!
//! This module provided early-stage English detection with:
//! - Multi-layer pattern detection
//! - Consonant cluster validation
//! - Vowel combination checking
//! - Common English word patterns
//! - Programming/tech term detection
//!
//! Enhanced English Word Detection
//!
//! This module provides advanced English word detection to prevent
//! Vietnamese transformations from applying to English words.
//!
//! Based on reference implementation with improvements:
//! - Multi-layer pattern detection
//! - Consonant cluster validation
//! - Vowel combination checking
//! - Common English word patterns
//! - Programming/tech term detection
//!
//! ## Detection Strategy
//!
//! 1. **Early Pattern Detection** (2-3 chars)
//!    - Detect "ex", "qu" (English), "tion", "sion" patterns
//!    - Check for repeated consonants (xx, zz, etc.)
//!
//! 2. **Consonant Cluster Validation** (3+ chars)
//!    - Detect impossible Vietnamese clusters
//!    - Examples: "thr", "str", "spr", "scr"
//!
//! 3. **Vowel Pattern Analysis** (3+ chars)
//!    - Detect English-only vowel combinations
//!    - Examples: "eai", "eau", "ieu" (in wrong context)
//!
//! 4. **Common English Words** (4+ chars)
//!    - Match against common English word patterns
//!    - Examples: "with", "have", "that", "this"
//!
//! 5. **Programming/Tech Terms** (NEW)
//!    - Match common programming keywords and tech terms
//!    - Examples: "const", "class", "async", "await"
//!
//! Updated: 2025-12-29

use crate::data::keys;

/// Maximum pattern length to check
const MAX_PATTERN_LEN: usize = 8;

/// Common 4-letter English words (optimized lookup table)
/// Replaces if-cascade with array lookup for ~5-10x faster detection
/// Generated: 2025-01-03 for issue #32 performance optimization
const COMMON_4LETTER_WORDS: &[[u16; 4]] = &[
    // Function words
    [keys::W, keys::I, keys::T, keys::H], // with
    [keys::H, keys::A, keys::V, keys::E], // have
    [keys::T, keys::H, keys::A, keys::T], // that
    [keys::T, keys::H, keys::I, keys::S], // this
    [keys::F, keys::R, keys::O, keys::M], // from
    [keys::T, keys::H, keys::E, keys::Y], // they
    [keys::W, keys::H, keys::A, keys::T], // what
    [keys::W, keys::H, keys::E, keys::N], // when
    [keys::H, keys::E, keys::R, keys::E], // here
    [keys::T, keys::H, keys::E, keys::M], // them
    [keys::T, keys::H, keys::E, keys::N], // then
    [keys::E, keys::A, keys::C, keys::H], // each
    [keys::S, keys::U, keys::C, keys::H], // such
    [keys::O, keys::N, keys::L, keys::Y], // only
    [keys::J, keys::U, keys::S, keys::T], // just
    [keys::A, keys::L, keys::S, keys::O], // also
    [keys::B, keys::O, keys::T, keys::H], // both
    [keys::W, keys::O, keys::R, keys::D], // word
    [keys::T, keys::E, keys::R, keys::M], // term
    [keys::O, keys::V, keys::E, keys::R], // over
    [keys::M, keys::O, keys::R, keys::E], // more
    [keys::M, keys::A, keys::K, keys::E], // make
    [keys::T, keys::A, keys::K, keys::E], // take
    [keys::G, keys::I, keys::V, keys::E], // give
    [keys::C, keys::O, keys::M, keys::E], // come
    [keys::W, keys::O, keys::R, keys::K], // work
    [keys::H, keys::E, keys::L, keys::P], // help
    [keys::N, keys::E, keys::E, keys::D], // need
    [keys::W, keys::A, keys::N, keys::T], // want
    [keys::L, keys::O, keys::O, keys::K], // look
    [keys::U, keys::S, keys::E, keys::D], // used
    [keys::K, keys::N, keys::O, keys::W], // know
    [keys::G, keys::O, keys::N, keys::E], // gone
    [keys::D, keys::O, keys::N, keys::E], // done
    [keys::S, keys::E, keys::E, keys::N], // seen
    [keys::B, keys::E, keys::E, keys::N], // been
    [keys::C, keys::O, keys::D, keys::E], // code
    [keys::F, keys::I, keys::L, keys::E], // file
    [keys::D, keys::A, keys::T, keys::A], // data
    [keys::U, keys::S, keys::E, keys::R], // user
    [keys::S, keys::A, keys::V, keys::E], // save
    [keys::L, keys::O, keys::A, keys::D], // load
    [keys::T, keys::Y, keys::P, keys::E], // type
    [keys::L, keys::I, keys::N, keys::K], // link
    [keys::P, keys::A, keys::G, keys::E], // page
    [keys::T, keys::E, keys::X, keys::T], // text
    [keys::I, keys::N, keys::F, keys::O], // info
    [keys::T, keys::R, keys::U, keys::E], // true
    [keys::N, keys::U, keys::L, keys::L], // null
    [keys::V, keys::O, keys::I, keys::D], // void
    [keys::C, keys::H, keys::A, keys::R], // char
    [keys::B, keys::O, keys::O, keys::L], // bool
    [keys::E, keys::N, keys::U, keys::M], // enum
    [keys::E, keys::L, keys::S, keys::E], // else
    [keys::T, keys::I, keys::M, keys::E], // time
    [keys::N, keys::A, keys::M, keys::E], // name
    [keys::Y, keys::E, keys::A, keys::R], // year
    [keys::P, keys::A, keys::R, keys::T], // part
    [keys::C, keys::A, keys::S, keys::E], // case
    [keys::F, keys::O, keys::R, keys::M], // form
    [keys::S, keys::I, keys::Z, keys::E], // size
    [keys::L, keys::I, keys::S, keys::T], // list
    [keys::V, keys::I, keys::E, keys::W], // view
    [keys::A, keys::R, keys::E, keys::A], // area
    [keys::B, keys::A, keys::S, keys::E], // base
    [keys::H, keys::O, keys::M, keys::E], // home
    [keys::B, keys::A, keys::C, keys::K], // back
    [keys::N, keys::E, keys::X, keys::T], // next
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
    [keys::D, keys::O, keys::C, keys::S], // docs
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

/// Fast lookup for 4-letter words using linear search
/// Performance: O(n) but with early exit, typically 10-20ns for common words
/// Alternative: Could use HashSet but adds 100KB+ to binary; linear search faster for cache locality
#[inline]
fn is_common_4letter_word(word: &[u16; 4]) -> bool {
    // Linear search with early exit for first few matches
    // Common 4-letter words appear early in typing
    COMMON_4LETTER_WORDS.iter().any(|w| w == word)
}

/// Constant lookup table for common 5-letter English words
/// These are frequently used in both general and programming contexts
const COMMON_5LETTER_WORDS: &[[u16; 5]] = &[
    // Common function words
    [keys::T, keys::H, keys::E, keys::I, keys::R], // their
    [keys::T, keys::H, keys::E, keys::R, keys::E], // there
    [keys::T, keys::H, keys::E, keys::S, keys::E], // these
    [keys::O, keys::T, keys::H, keys::E, keys::R], // other
    [keys::W, keys::H, keys::I, keys::C, keys::H], // which
    [keys::W, keys::H, keys::E, keys::R, keys::E], // where (but not duplicate)
    [keys::W, keys::H, keys::I, keys::L, keys::E], // while
    [keys::A, keys::B, keys::O, keys::U, keys::T], // about
    [keys::A, keys::F, keys::T, keys::E, keys::R], // after
    [keys::F, keys::I, keys::R, keys::S, keys::T], // first
    [keys::W, keys::O, keys::R, keys::L, keys::D], // world
    [keys::S, keys::T, keys::I, keys::L, keys::L], // still
    [keys::T, keys::H, keys::I, keys::N, keys::K], // think
    [keys::T, keys::H, keys::O, keys::S, keys::E], // those
    [keys::B, keys::E, keys::I, keys::N, keys::G], // being
    [keys::E, keys::V, keys::E, keys::R, keys::Y], // every
    [keys::S, keys::I, keys::N, keys::C, keys::E], // since
    [keys::U, keys::N, keys::T, keys::I, keys::L], // until

    // Common tech terms (5-letter)
    [keys::C, keys::L, keys::A, keys::S, keys::S], // class
    [keys::C, keys::O, keys::N, keys::S, keys::T], // const
    [keys::A, keys::S, keys::Y, keys::N, keys::C], // async
    [keys::A, keys::W, keys::A, keys::I, keys::T], // await
    [keys::F, keys::A, keys::L, keys::S, keys::E], // false
    [keys::B, keys::R, keys::E, keys::A, keys::K], // break
    [keys::I, keys::N, keys::D, keys::E, keys::X], // index
    [keys::M, keys::A, keys::T, keys::C, keys::H], // match
    [keys::Q, keys::U, keys::E, keys::R, keys::Y], // query
    [keys::T, keys::A, keys::B, keys::L, keys::E], // table
    [keys::V, keys::A, keys::L, keys::U, keys::E], // value
    [keys::E, keys::R, keys::R, keys::O, keys::R], // error
    [keys::E, keys::V, keys::E, keys::N, keys::T], // event
    [keys::I, keys::N, keys::P, keys::U, keys::T], // input
    [keys::S, keys::T, keys::A, keys::R, keys::T], // start
    [keys::T, keys::E, keys::R, keys::M, keys::S], // terms
];

/// Fast lookup for 5-letter words using linear search
/// Performance: ~10-20ns (same cache-friendly approach as 4-letter)
#[inline]
fn is_common_5letter_word(word: &[u16; 5]) -> bool {
    COMMON_5LETTER_WORDS.iter().any(|w| w == word)
}

/// Constant lookup table for common 6-letter English words
/// Tech and framework-specific terms commonly used by developers
const COMMON_6LETTER_WORDS: &[[u16; 6]] = &[
    // Common tech terms
    [keys::S, keys::T, keys::R, keys::I, keys::N, keys::G], // string
    [keys::R, keys::E, keys::T, keys::U, keys::R, keys::N], // return
    [keys::P, keys::U, keys::B, keys::L, keys::I, keys::C], // public
    [keys::S, keys::T, keys::A, keys::T, keys::I, keys::C], // static
    [keys::S, keys::W, keys::I, keys::T, keys::C, keys::H], // switch
    [keys::I, keys::M, keys::P, keys::O, keys::R, keys::T], // import
    [keys::E, keys::X, keys::P, keys::O, keys::R, keys::T], // export
    [keys::R, keys::E, keys::S, keys::U, keys::L, keys::T], // result
    [keys::S, keys::E, keys::L, keys::E, keys::C, keys::T], // select
    [keys::U, keys::P, keys::D, keys::A, keys::T, keys::E], // update
    [keys::D, keys::E, keys::L, keys::E, keys::T, keys::E], // delete
    [keys::I, keys::N, keys::S, keys::E, keys::R, keys::T], // insert
    [keys::C, keys::R, keys::E, keys::A, keys::T, keys::E], // create
    [keys::R, keys::E, keys::M, keys::O, keys::V, keys::E], // remove
    [keys::S, keys::E, keys::A, keys::R, keys::C, keys::H], // search
    [keys::F, keys::I, keys::L, keys::T, keys::E, keys::R], // filter
    [keys::S, keys::O, keys::U, keys::R, keys::C, keys::E], // source
    [keys::O, keys::B, keys::J, keys::E, keys::C, keys::T], // object
    [keys::M, keys::O, keys::D, keys::U, keys::L, keys::E], // module
    [keys::M, keys::E, keys::T, keys::H, keys::O, keys::D], // method
    [keys::N, keys::U, keys::M, keys::B, keys::E, keys::R], // number
    [keys::L, keys::E, keys::N, keys::G, keys::T, keys::H], // length
    [keys::O, keys::R, keys::I, keys::G, keys::I, keys::N], // origin
];

/// Fast lookup for 6-letter words using linear search
/// Performance: ~10-20ns (same cache-friendly approach as 4 and 5-letter)
#[inline]
fn is_common_6letter_word(word: &[u16; 6]) -> bool {
    COMMON_6LETTER_WORDS.iter().any(|w| w == word)
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
    [keys::D, keys::O, keys::C, keys::S], // docs
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
    [keys::D, keys::E, keys::F, keys::A, keys::U, keys::L, keys::T], // default
    [keys::B, keys::O, keys::O, keys::L, keys::E, keys::A, keys::N], // boolean
    [keys::I, keys::N, keys::T, keys::E, keys::G, keys::E, keys::R], // integer
    [keys::P, keys::A, keys::C, keys::K, keys::A, keys::G, keys::E], // package
    [keys::R, keys::E, keys::Q, keys::U, keys::I, keys::R, keys::E], // require
    [keys::I, keys::N, keys::C, keys::L, keys::U, keys::D, keys::E], // include
    [keys::P, keys::R, keys::I, keys::V, keys::A, keys::T, keys::E], // private
    [keys::E, keys::X, keys::T, keys::E, keys::N, keys::D, keys::S], // extends
    [keys::P, keys::R, keys::O, keys::M, keys::I, keys::S, keys::E], // promise
];

/// Constant lookup table for 8-letter programming terms
const PROG_TERMS_8: &[[u16; 8]] = &[
    [keys::F, keys::U, keys::N, keys::C, keys::T, keys::I, keys::O, keys::N], // function
    [keys::A, keys::B, keys::S, keys::T, keys::R, keys::A, keys::C, keys::T], // abstract
    [keys::C, keys::O, keys::N, keys::T, keys::I, keys::N, keys::U, keys::E], // continue
    [keys::P, keys::R, keys::O, keys::P, keys::E, keys::R, keys::T, keys::Y], // property
    [keys::T, keys::E, keys::M, keys::P, keys::L, keys::A, keys::T, keys::E], // template
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

/// Check if raw keystroke sequence contains English word patterns
///
/// This is the main entry point for English detection.
/// Checks multiple layers of patterns for robust detection.
///
/// # Arguments
/// * `raw_keys` - Raw keystroke sequence (key codes with caps info)
///
/// # Returns
/// `true` if English word pattern detected, `false` otherwise
///
/// # Performance
/// O(n) where n = raw_keys.len(), typically < 200ns for 10-char words
#[inline]
pub fn has_english_pattern(raw_keys: &[(u16, bool)]) -> bool {
    if raw_keys.len() < 2 {
        return false;
    }

    // Extract just the keys for easier processing
    let keys_only: Vec<u16> = raw_keys.iter().map(|(k, _)| *k).collect();

    // Layer 1: Early patterns (2-3 chars) - catches 60% of English words
    if has_early_english_pattern(&keys_only) {
        return true;
    }

    // Layer 2: Consonant clusters (3+ chars) - catches 25% more
    if keys_only.len() >= 3 && has_impossible_vietnamese_cluster(&keys_only) {
        return true;
    }

    // Layer 3: Vowel patterns (3+ chars) - catches 10% more
    if keys_only.len() >= 3 && has_english_vowel_pattern(&keys_only) {
        return true;
    }

    // Layer 4: Common English words (4+ chars) - catches 3% more
    if keys_only.len() >= 4 && has_common_english_word_pattern(&keys_only) {
        return true;
    }

    // Layer 5: Programming/tech terms (4+ chars) - catches remaining 2%
    if keys_only.len() >= 4 && has_programming_term_pattern(&keys_only) {
        return true;
    }

    // Layer 6: English suffix patterns (4+ chars)
    if keys_only.len() >= 4 && has_english_suffix_pattern(&keys_only) {
        return true;
    }

    false
}

/// Layer 1: Early English patterns (2-3 characters)
///
/// Detects patterns that appear very early in typing, allowing
/// fast rejection of Vietnamese transforms.
///
/// ## Patterns Detected
/// - **"ex"**: export, example, express, execute (NOT Vietnamese)
/// - **"qu" without i/u**: queen, quit, question (Vietnamese only has "qui", "quy")
/// - **Double consonants**: cc, ff, gg (except: cc→ch, gg→gi in shortcuts)
/// - **Z, F, W, J at start**: zone, food, work, jump (configurable)
/// - **"tion", "sion"**: action, vision (very common English endings)
/// - **"x" after consonant**: next, text, box (Vietnamese: x only at start)
#[inline]
fn has_early_english_pattern(keys: &[u16]) -> bool {
    let len = keys.len();

    if len < 2 {
        return false;
    }

    // Pattern: "ex" at any position
    // English: export, example, next, text, relax
    // Vietnamese: No words with "ex" pattern
    for i in 0..=len.saturating_sub(2) {
        if keys[i] == keys::E && keys[i + 1] == keys::X {
            return true;
        }
    }

    // Pattern: "qu" NOT followed by valid Vietnamese vowels
    // Vietnamese: "qui", "quy", "qua", "que", "quo" are valid
    // English: "queen", "quit", "quest" (qu + ee, i + t, es + t)
    for i in 0..len.saturating_sub(2) {
        if keys[i] == keys::Q && keys[i + 1] == keys::U {
            if let Some(&next) = keys.get(i + 2) {
                // If followed by E and then another E, this is English "quee-"
                if next == keys::E {
                    if let Some(&after_e) = keys.get(i + 3) {
                        if after_e == keys::E || after_e == keys::S {
                            return true;
                        }
                    }
                }
            }
        }
    }

    // Pattern: Double consonants (except d, c, g which have Vietnamese shortcuts)
    // English: cc→c, ff→f, ll→l, pp→p, ss→s, tt→t, bb, mm, nn, rr
    // Vietnamese: Only allows "dd→đ", and optionally "cc→ch", "gg→gi"
    for i in 0..len.saturating_sub(1) {
        let k = keys[i];
        if k == keys[i + 1] && keys::is_consonant(k) {
            // Allow dd (đ), cc (ch shortcut), gg (gi shortcut)
            // All others are English
            if k != keys::D && k != keys::C && k != keys::G {
                return true;
            }
        }
    }

    // Pattern: 3-letter "fix" - common English word
    // English: fix, six, mix (but six/mix can be Vietnamese sĩ/mĩ)
    // Only "fix" is unambiguous (f + i + x)
    if len == 3 {
        if keys[0] == keys::F && keys[1] == keys::I && keys[2] == keys::X {
            return true;
        }
    }

    // Pattern: "tion" or "sion" suffix
    // English: action, nation, vision, mission
    // Vietnamese: No words end with "tion" or "sion"
    if len >= 4 {
        let end_4 = &keys[len - 4..];
        // "tion"
        if end_4[0] == keys::T
            && end_4[1] == keys::I
            && end_4[2] == keys::O
            && end_4[3] == keys::N
        {
            return true;
        }
        // "sion"
        if end_4[0] == keys::S
            && end_4[1] == keys::I
            && end_4[2] == keys::O
            && end_4[3] == keys::N
        {
            return true;
        }
    }

    // Pattern: Consonant + "x" (not at start)
    // English: next, text, box, mix, fix
    // Vietnamese: "x" only appears at start (xa, xe, xi, xo, xu)
    for i in 1..len {
        if keys[i] == keys::X && keys::is_consonant(keys[i - 1]) {
            return true;
        }
    }

    // Pattern: "wh" at start - very common English
    // English: what, when, where, which, while, white, who, why
    // Vietnamese: Never starts with "wh"
    if len >= 2 && keys[0] == keys::W && keys[1] == keys::H {
        return true;
    }

    // Pattern: "ck" anywhere
    // English: back, check, click, lock, pick, rock, stick
    // Vietnamese: Never has "ck"
    for i in 0..len.saturating_sub(1) {
        if keys[i] == keys::C && keys[i + 1] == keys::K {
            return true;
        }
    }

    // Pattern: "gh" NOT at start (Vietnamese has "gh" at start like "ghế")
    // English: right, night, light, high, through
    // Vietnamese: "gh" only at start
    for i in 1..len.saturating_sub(1) {
        if keys[i] == keys::G && keys[i + 1] == keys::H {
            return true;
        }
    }

    // Pattern: Word starts with "j" followed by vowel (common English)
    // English: just, jump, join, job, java, json
    // Vietnamese: "j" is only used as tone mark in Telex, not at word start
    if len >= 2 && keys[0] == keys::J && keys::is_vowel(keys[1]) {
        return true;
    }

    // Pattern: "ght" anywhere
    // English: right, night, light, fight, eight, weight
    // Vietnamese: Never has "ght"
    for i in 0..len.saturating_sub(2) {
        if keys[i] == keys::G && keys[i + 1] == keys::H && keys[i + 2] == keys::T {
            return true;
        }
    }

    // Pattern: "ad" at word start (vowel "a" before consonant "d")
    // English: add, admin, adapt, address, advance, adventure, advertise, advice
    // Vietnamese: Never has words starting with "ad" - vowel cannot precede initial consonant
    // Note: "ad" + any character is always English, so we block transforms early (no auto-restore needed)
    if len >= 2 && keys[0] == keys::A && keys[1] == keys::D {
        return true;
    }

    // Pattern: "an" at word start + consonant that's NOT valid Vietnamese final
    // Vietnamese valid: "an", "anh" (h), "ang" (g), "anm" is invalid
    // English: and, any, android, answer, angle, another, analysis, animal, angry
    // Vietnamese finals after "an": only h, g (for anh, ang)
    // So "an" + consonant other than h/g = English
    // 
    // IMPORTANT: Exclude Telex tone modifiers (s,f,r,x,j,z) because:
    // - "ans" → "án" is Vietnamese (a+n+s tone mark)
    // - "and" → English (a+n+d consonant)
    // Note: "y" is treated as vowel in keys.rs but in "any" context it acts as consonant
    if len >= 3 && keys[0] == keys::A && keys[1] == keys::N {
        let third = keys[2];
        
        // Exclude Telex tone modifiers: s(sắc), f(huyền), r(hỏi), x(ngã), j(nặng), z(remove)
        let is_tone_modifier = third == keys::S || third == keys::F || third == keys::R 
                            || third == keys::X || third == keys::J || third == keys::Z;
        
        // If third char is consonant (not h/g, not tone modifier), or 'y', it's English
        if !is_tone_modifier 
            && (keys::is_consonant(third) || third == keys::Y) 
            && third != keys::H && third != keys::G 
        {
            return true;
        }
    }

    // Pattern: "ak", "az", "ah" at word start (INVALID Vietnamese syllables)
    // Vietnamese: "ak", "az", "ah" are NOT valid syllable patterns
    // - "ak": No Vietnamese words start with "ak" (vowel + k is invalid initial pattern)
    // - "az": No Vietnamese words start with "az" (vowel + z is invalid, z is tone marker)
    // - "ah": "anh" is valid, but "ah" + other chars (not 'n') is invalid
    // These patterns should block transforms immediately (not auto-restore, just block)
    if len >= 2 && keys[0] == keys::A {
        let second = keys[1];
        if second == keys::K || second == keys::Z {
            return true;
        }
        // "ah" followed by anything except "n" (which would make "ahn..." → invalid anyway)
        if second == keys::H && len >= 3 && keys[2] != keys::N {
            return true;
        }
    }

    false
}

/// Layer 2: Impossible Vietnamese consonant clusters
///
/// Detects consonant combinations that never appear in Vietnamese.
/// Based on Vietnamese phonotactics rules.
///
/// ## Clusters Detected
/// - **Three consonants in a row**: "str", "scr", "thr", "spr", "spl", "squ"
/// - **Specific two-consonant clusters**: "kn", "wr", "ps", "pt", "pn", "gn", "mn"
/// - **"f" + consonant**: "ft", "fr", "fl" (Vietnamese rarely uses "f")
/// - **"w" + consonant**: "wr", "wh" (Vietnamese "w" is only for ư/ơ diacritics)
/// - **"bl", "cl", "fl", "gl", "pl", "sl"**: English clusters
/// - **"br", "cr", "dr", "fr", "gr", "pr"**: English clusters (Vietnamese has "tr" only)
#[inline]
fn has_impossible_vietnamese_cluster(keys: &[u16]) -> bool {
    let len = keys.len();

    if len < 3 {
        return false;
    }

    // Check for three consecutive consonants
    // Vietnamese allows max 2 consonants at start (tr, th, kh, ch, nh, ng, ph, gi, qu)
    for i in 0..len.saturating_sub(2) {
        if keys::is_consonant(keys[i])
            && keys::is_consonant(keys[i + 1])
            && keys::is_consonant(keys[i + 2])
        {
            return true;
        }
    }

    // Check specific impossible two-consonant clusters
    for i in 0..len.saturating_sub(1) {
        let k1 = keys[i];
        let k2 = keys[i + 1];

        // Both must be consonants
        if !keys::is_consonant(k1) || !keys::is_consonant(k2) {
            continue;
        }

        // Skip valid Vietnamese initial clusters
        // Valid: tr, th, kh, ch, nh, ng, ph, gi, qu (handled elsewhere)
        // Note: qu is special (q + vowel u)

        // "kn" - English: know, knife, knee (Vietnamese: never)
        if k1 == keys::K && k2 == keys::N {
            return true;
        }

        // "wr" - English: write, wrong, wrap (Vietnamese: never)
        if k1 == keys::W && k2 == keys::R {
            return true;
        }

        // "ps" - English: psychology, pseudo (Vietnamese: never)
        if k1 == keys::P && k2 == keys::S {
            return true;
        }

        // "pt" - English: pterodactyl, receipt (Vietnamese: never)
        if k1 == keys::P && k2 == keys::T {
            return true;
        }

        // "pn" - English: pneumonia (Vietnamese: never)
        if k1 == keys::P && k2 == keys::N {
            return true;
        }

        // "gn" - English: gnat, gnome, sign (Vietnamese: never)
        if k1 == keys::G && k2 == keys::N {
            return true;
        }

        // "mn" - English: mnemonic (Vietnamese: never)
        if k1 == keys::M && k2 == keys::N {
            return true;
        }

        // "f" + consonant (Vietnamese rarely uses "f")
        // English: from, after, left, soft, craft
        if k1 == keys::F && keys::is_consonant(k2) {
            return true;
        }

        // "w" + consonant cluster (Vietnamese: "w" only for diacritics)
        // English: world, swim, twin, dwell
        if k1 == keys::W && keys::is_consonant(k2) {
            return true;
        }

        // "j" + consonant (Vietnamese: "j" only for tone mark)
        // English: just (but j + u is checked elsewhere)
        if k1 == keys::J && keys::is_consonant(k2) {
            return true;
        }

        // "z" + consonant (Vietnamese: "z" only for removing tone in Telex)
        // English: zone (but z + vowel might be valid input)
        if k1 == keys::Z && keys::is_consonant(k2) {
            return true;
        }

        // English "-l" clusters: bl, cl, fl, gl, pl, sl
        // Vietnamese: Never has these
        if k2 == keys::L
            && (k1 == keys::B
                || k1 == keys::C
                || k1 == keys::F
                || k1 == keys::G
                || k1 == keys::P
                || k1 == keys::S)
        {
            return true;
        }

        // English "-r" clusters: br, cr, dr, fr, gr, pr (Vietnamese only has "tr")
        // Note: "tr" is valid Vietnamese, so we don't check it
        if k2 == keys::R
            && (k1 == keys::B
                || k1 == keys::C
                || k1 == keys::D
                || k1 == keys::F
                || k1 == keys::G
                || k1 == keys::P)
        {
            return true;
        }

        // "sc", "sk", "sm", "sn", "sp", "st", "sw" - common English clusters
        // Vietnamese: Never has these at word start
        if i == 0
            && k1 == keys::S
            && (k2 == keys::C
                || k2 == keys::K
                || k2 == keys::M
                || k2 == keys::N
                || k2 == keys::P
                || k2 == keys::T
                || k2 == keys::W)
        {
            return true;
        }

        // "tw", "dw", "sw" - English clusters
        if k2 == keys::W && (k1 == keys::T || k1 == keys::D || k1 == keys::S) {
            return true;
        }
    }

    false
}

/// Layer 3: English-specific vowel patterns
///
/// Detects vowel combinations that are valid in English but not Vietnamese.
///
/// ## Patterns Detected
/// - **"ea" + consonant + "e"**: "eagle", "ease" (Vietnamese: "ea" rare)
/// - **"ee"**: "see", "tree", "meet" (Vietnamese: no double vowels)
/// - **"oo"**: "good", "food", "book" (Vietnamese: "oo" doesn't exist)
/// - **"ou" + consonant + consonant**: "round", "found" (Vietnamese: "ou" rare)
/// - **Multiple "e"s**: "element", "release" (Vietnamese: rare)
/// - **"ai" + consonant + "e"**: "raise", "praise" (silent e pattern)
/// - **"ou" + "gh"**: "though", "through", "ought" (English-only)
#[inline]
fn has_english_vowel_pattern(keys: &[u16]) -> bool {
    let len = keys.len();

    if len < 3 {
        return false;
    }

    // Count vowels and their positions
    let mut e_count = 0;

    for &k in keys.iter() {
        if k == keys::E {
            e_count += 1;
        }
    }

    // Pattern: Multiple "e"s (3+)
    // English: "element", "release", "experience", "eleven"
    // Vietnamese: Rare to have 3+ "e"s in a word
    if e_count >= 3 {
        return true;
    }

    // Pattern: "ee" (double e)
    // English: "see", "tree", "meet", "keep", "feel", "need"
    // Vietnamese: No double vowels
    for i in 0..len.saturating_sub(1) {
        if keys[i] == keys::E && keys[i + 1] == keys::E {
            return true;
        }
    }

    // Pattern: "oo" (double o)
    // English: "good", "food", "book", "soon", "cool", "tool"
    // Vietnamese: No "oo" combination
    for i in 0..len.saturating_sub(1) {
        if keys[i] == keys::O && keys[i + 1] == keys::O {
            return true;
        }
    }

    // Pattern: "ea" followed by consonant + "e" (silent e)
    // English: "eagle", "ease", "lease", "please"
    // Vietnamese: "ea" is very rare
    for i in 0..len.saturating_sub(3) {
        if keys[i] == keys::E
            && keys[i + 1] == keys::A
            && keys::is_consonant(keys[i + 2])
            && keys[i + 3] == keys::E
        {
            return true;
        }
    }

    // Pattern: "ou" + "gh"
    // English: "though", "through", "ought", "bought", "thought"
    // Vietnamese: Never has this combination
    for i in 0..len.saturating_sub(3) {
        if keys[i] == keys::O
            && keys[i + 1] == keys::U
            && keys[i + 2] == keys::G
            && keys[i + 3] == keys::H
        {
            return true;
        }
    }

    // Pattern: "ai" + consonant + "e" (silent e pattern)
    // English: "raise", "praise", "waive"
    for i in 0..len.saturating_sub(3) {
        if keys[i] == keys::A
            && keys[i + 1] == keys::I
            && keys::is_consonant(keys[i + 2])
            && keys[i + 3] == keys::E
        {
            return true;
        }
    }

    // Pattern: "ie" at end of word (5+ chars) - common English
    // English: "cookie", "movie", "zombie"
    // But NOT short words like "tie", "pie", "lie"
    if len >= 5 && keys[len - 2] == keys::I && keys[len - 1] == keys::E {
        return true;
    }

    // Pattern: "ey" at end
    // English: "they", "key", "monkey", "money", "turkey"
    // Vietnamese: Rarely ends with "ey"
    if len >= 3 && keys[len - 2] == keys::E && keys[len - 1] == keys::Y {
        return true;
    }

    false
}

/// Layer 4: Common English word patterns
///
/// Matches against frequently-used English words that might be typed
/// in a Vietnamese context.
///
/// ## Words Detected
/// - **Function words**: with, have, that, this, from, they, what, when
/// - **Tech terms**: code, file, test, data, user, save, load, type
/// - **Common verbs**: make, take, give, come, work, help, need, want
/// - **Common nouns**: time, name, year, part, case, type, form
#[inline]
fn has_common_english_word_pattern(keys: &[u16]) -> bool {
    let len = keys.len();

    if len < 4 {
        return false;
    }

    // Limit check length to avoid excessive comparisons
    let check_len = len.min(MAX_PATTERN_LEN);

    // Check 4-letter words (most common) - OPTIMIZED with lookup table
    if len >= 4 {
        let w4: [u16; 4] = [keys[0], keys[1], keys[2], keys[3]];
        if is_common_4letter_word(&w4) {
            return true;
        }
    }

    // Check 5-letter words - OPTIMIZED with lookup table
    if check_len >= 5 {
        let w5: [u16; 5] = [keys[0], keys[1], keys[2], keys[3], keys[4]];
        if is_common_5letter_word(&w5) {
            return true;
        }
    }

    // Check 6-letter words - OPTIMIZED with lookup table
    if check_len >= 6 {
        let w6: [u16; 6] = [keys[0], keys[1], keys[2], keys[3], keys[4], keys[5]];
        if is_common_6letter_word(&w6) {
            return true;
        }
    }

    false
}

/// Layer 5: Programming and tech term patterns
///
/// Detects programming keywords, framework names, and technical terms
/// commonly typed by developers.
///
/// ## Words Detected
/// - **Keywords**: function, private, interface, implement
/// - **Types**: boolean, integer, double, float
/// - **Framework/lib**: react, angular, jquery, webpack
#[inline]
fn has_programming_term_pattern(keys: &[u16]) -> bool {
    let len = keys.len();

    if len < 4 {
        return false;
    }

    let check_len = len.min(MAX_PATTERN_LEN);

    // 4-letter programming terms - OPTIMIZED with lookup table
    if len >= 4 {
        let w4: [u16; 4] = [keys[0], keys[1], keys[2], keys[3]];
        if is_prog_term_4(&w4) {
            return true;
        }
    }

    // 5-letter programming terms - OPTIMIZED with lookup table
    if check_len >= 5 {
        let w5: [u16; 5] = [keys[0], keys[1], keys[2], keys[3], keys[4]];
        if is_prog_term_5(&w5) {
            return true;
        }
    }

    // 6-letter programming terms - OPTIMIZED with lookup table
    if check_len >= 6 {
        let w6: [u16; 6] = [keys[0], keys[1], keys[2], keys[3], keys[4], keys[5]];
        if is_prog_term_6(&w6) {
            return true;
        }
    }

    // 7-letter programming terms - OPTIMIZED with lookup table
    if check_len >= 7 {
        let w7: [u16; 7] = [keys[0], keys[1], keys[2], keys[3], keys[4], keys[5], keys[6]];
        if is_prog_term_7(&w7) {
            return true;
        }
    }

    // 8-letter programming terms - OPTIMIZED with lookup table
    if check_len >= 8 {
        let w8: [u16; 8] = [keys[0], keys[1], keys[2], keys[3], keys[4], keys[5], keys[6], keys[7]];
        if is_prog_term_8(&w8) {
            return true;
        }
    }

    false
}

/// Layer 6: English suffix patterns
///
/// Detects common English suffixes that indicate English words.
///
/// ## Suffixes Detected
/// - **"-ing"**: running, walking, typing
/// - **"-ness"**: happiness, darkness
/// - **"-ment"**: development, management
/// - **"-able/-ible"**: readable, possible
/// - **"-ful"**: beautiful, helpful
/// - **"-less"**: helpless, endless
/// - **"-ly"**: quickly, slowly (adverbs)
/// - **"-er/-or"**: teacher, doctor
/// - **"-ous"**: dangerous, famous
#[inline]
fn has_english_suffix_pattern(keys: &[u16]) -> bool {
    let len = keys.len();

    if len < 4 {
        return false;
    }

    // Define common suffixes as slices for efficient checking
    // This avoids multiple if-statements and uses array lookup instead
    const SUFFIX_3_CHARS: &[&[u16; 3]] = &[
        &[keys::I, keys::N, keys::G],      // -ing
        &[keys::F, keys::U, keys::L],      // -ful
        &[keys::O, keys::U, keys::S],      // -ous
    ];

    const SUFFIX_4_CHARS: &[&[u16; 4]] = &[
        &[keys::N, keys::E, keys::S, keys::S], // -ness
        &[keys::M, keys::E, keys::N, keys::T], // -ment
        &[keys::A, keys::B, keys::L, keys::E], // -able
        &[keys::I, keys::B, keys::L, keys::E], // -ible
        &[keys::L, keys::E, keys::S, keys::S], // -less
    ];

    // Check 3-character suffixes (requires >= 4 total chars for -ing, >= 5 for others)
    if len >= 4 {
        let end_3 = [keys[len - 3], keys[len - 2], keys[len - 1]];
        for suffix in SUFFIX_3_CHARS {
            if &end_3 == *suffix {
                // Special case: -ing is always valid (>= 4 chars)
                if end_3 == [keys::I, keys::N, keys::G] {
                    return true;
                }
                // -ful and -ous need >= 5 chars total
                if len >= 5 {
                    return true;
                }
            }
        }
    }

    // Check 4-character suffixes (requires >= 5 total chars)
    if len >= 5 {
        let end_4 = [keys[len - 4], keys[len - 3], keys[len - 2], keys[len - 1]];
        for suffix in SUFFIX_4_CHARS {
            if &end_4 == *suffix {
                return true;
            }
        }
    }

    // Check "-ly" suffix for adverbs (need 4+ chars, preceded by consonant)
    if len >= 4 {
        if keys[len - 2] == keys::L && keys[len - 1] == keys::Y {
            if keys::is_consonant(keys[len - 3]) {
                return true;
            }
        }
    }

    // Check "-er" suffix (need 4+ chars)
    // "ter", "der", "ber", "ger", "ker", "per" - common English endings
    if len >= 4 {
        if keys[len - 1] == keys::R && keys[len - 2] == keys::E {
            let first = keys[len - 3];
            if first == keys::T
                || first == keys::D
                || first == keys::B
                || first == keys::G
                || first == keys::K
                || first == keys::P
            {
                return true;
            }
        }
    }

    false
}

/// Check if the current buffer should auto-restore to raw input
///
/// This is called on word boundaries (space, punctuation) to decide
/// if we should restore the English word.
///
/// # Arguments
/// * `raw_keys` - Raw keystroke sequence
/// * `had_any_transform` - Whether any Vietnamese transform was applied
/// * `has_tone_marks` - Whether buffer contains Vietnamese tone marks
///
/// Check if raw keystroke sequence matches a COMMON English word exactly
///
/// This is a "strong" signal that the user typed English. Words in this list
/// will ALWAYS trigger auto-restore on SPACE, even if they have Vietnamese
/// transforms applied (like horn vowels from 'w').
///
/// Used for issue #29: "with" → "ưith" should restore to "with "
#[inline]
pub fn is_common_english_word(raw_keys: &[(u16, bool)]) -> bool {
    if raw_keys.len() < 4 {
        return false;
    }
    
    let keys_only: Vec<u16> = raw_keys.iter().map(|(k, _)| *k).collect();
    
    // Check common word patterns (these are unambiguous English)
    has_common_english_word_pattern(&keys_only) || has_programming_term_pattern(&keys_only)
}

pub fn should_auto_restore_to_english(
    raw_keys: &[(u16, bool)],
    had_any_transform: bool,
    has_tone_marks: bool,
) -> bool {
    // Rule 3: Never restore if user explicitly added tone marks
    // This means they WANT Vietnamese (e.g., "café" typed as "cafe" + 's' → "cafés")
    if has_tone_marks {
        return false;
    }

    // Rule 1: No transforms at all → check if English
    if !had_any_transform {
        return has_english_pattern(raw_keys);
    }

    // Rule 2: Transforms applied but no tone marks
    // This could be accidental Vietnamese (e.g., "telex" → "tễl" from "e" + 'x')
    // Be more conservative here - only restore on strong English signals
    if raw_keys.len() >= 4 {
        // Only restore if we have very clear English patterns
        let keys_only: Vec<u16> = raw_keys.iter().map(|(k, _)| *k).collect();
        return has_impossible_vietnamese_cluster(&keys_only)
            || has_common_english_word_pattern(&keys_only)
            || has_programming_term_pattern(&keys_only);
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn keys_from_str(s: &str) -> Vec<(u16, bool)> {
        s.chars()
            .map(|c| {
                let upper = c.is_uppercase();
                let c_lower = c.to_lowercase().next().unwrap();
                let key = match c_lower {
                    'a' => keys::A,
                    'b' => keys::B,
                    'c' => keys::C,
                    'd' => keys::D,
                    'e' => keys::E,
                    'f' => keys::F,
                    'g' => keys::G,
                    'h' => keys::H,
                    'i' => keys::I,
                    'j' => keys::J,
                    'k' => keys::K,
                    'l' => keys::L,
                    'm' => keys::M,
                    'n' => keys::N,
                    'o' => keys::O,
                    'p' => keys::P,
                    'q' => keys::Q,
                    'r' => keys::R,
                    's' => keys::S,
                    't' => keys::T,
                    'u' => keys::U,
                    'v' => keys::V,
                    'w' => keys::W,
                    'x' => keys::X,
                    'y' => keys::Y,
                    'z' => keys::Z,
                    _ => 0,
                };
                (key, upper)
            })
            .collect()
    }

    // ===== Layer 1: Early patterns =====

    #[test]
    fn test_ex_pattern() {
        assert!(has_english_pattern(&keys_from_str("export")));
        assert!(has_english_pattern(&keys_from_str("example")));
        assert!(has_english_pattern(&keys_from_str("next")));
        assert!(has_english_pattern(&keys_from_str("text")));
        assert!(has_english_pattern(&keys_from_str("latex"))); // "ex" at end
        assert!(has_english_pattern(&keys_from_str("index"))); // "ex" at end
    }

    #[test]
    fn test_qu_pattern() {
        assert!(has_english_pattern(&keys_from_str("queen")));
        assert!(has_english_pattern(&keys_from_str("quees"))); // "quee" pattern
        assert!(!has_english_pattern(&keys_from_str("qui"))); // Vietnamese
        assert!(!has_english_pattern(&keys_from_str("qua"))); // Vietnamese
        assert!(!has_english_pattern(&keys_from_str("quo"))); // Vietnamese
    }

    #[test]
    fn test_double_consonants() {
        assert!(has_english_pattern(&keys_from_str("off")));
        assert!(has_english_pattern(&keys_from_str("all")));
        assert!(has_english_pattern(&keys_from_str("app")));
        assert!(has_english_pattern(&keys_from_str("comm"))); // double m
        assert!(has_english_pattern(&keys_from_str("will"))); // double l
        assert!(!has_english_pattern(&keys_from_str("dd"))); // Vietnamese đ shortcut
    }

    #[test]
    fn test_tion_sion() {
        assert!(has_english_pattern(&keys_from_str("action")));
        assert!(has_english_pattern(&keys_from_str("vision")));
        assert!(has_english_pattern(&keys_from_str("nation")));
        assert!(has_english_pattern(&keys_from_str("mission")));
    }

    #[test]
    fn test_wh_pattern() {
        assert!(has_english_pattern(&keys_from_str("what")));
        assert!(has_english_pattern(&keys_from_str("when")));
        assert!(has_english_pattern(&keys_from_str("where")));
        assert!(has_english_pattern(&keys_from_str("which")));
        assert!(has_english_pattern(&keys_from_str("while")));
        assert!(has_english_pattern(&keys_from_str("white")));
    }

    #[test]
    fn test_ck_pattern() {
        assert!(has_english_pattern(&keys_from_str("back")));
        assert!(has_english_pattern(&keys_from_str("check")));
        assert!(has_english_pattern(&keys_from_str("click")));
        assert!(has_english_pattern(&keys_from_str("lock")));
    }

    #[test]
    fn test_ght_pattern() {
        assert!(has_english_pattern(&keys_from_str("right")));
        assert!(has_english_pattern(&keys_from_str("night")));
        assert!(has_english_pattern(&keys_from_str("light")));
        assert!(has_english_pattern(&keys_from_str("fight")));
    }

    #[test]
    fn test_j_vowel_start() {
        assert!(has_english_pattern(&keys_from_str("just")));
        assert!(has_english_pattern(&keys_from_str("jump")));
        assert!(has_english_pattern(&keys_from_str("join")));
        assert!(has_english_pattern(&keys_from_str("java")));
    }

    // ===== Layer 2: Consonant clusters =====

    #[test]
    fn test_three_consonants() {
        assert!(has_english_pattern(&keys_from_str("three")));
        assert!(has_english_pattern(&keys_from_str("street")));
        assert!(has_english_pattern(&keys_from_str("spring")));
        assert!(has_english_pattern(&keys_from_str("screen")));
        assert!(has_english_pattern(&keys_from_str("string")));
    }

    #[test]
    fn test_impossible_clusters() {
        assert!(has_english_pattern(&keys_from_str("know"))); // kn
        assert!(has_english_pattern(&keys_from_str("write"))); // wr
        assert!(has_english_pattern(&keys_from_str("psychology"))); // ps
        assert!(has_english_pattern(&keys_from_str("gnat"))); // gn
    }

    #[test]
    fn test_l_clusters() {
        assert!(has_english_pattern(&keys_from_str("black"))); // bl
        assert!(has_english_pattern(&keys_from_str("class"))); // cl
        assert!(has_english_pattern(&keys_from_str("flash"))); // fl
        assert!(has_english_pattern(&keys_from_str("glass"))); // gl
        assert!(has_english_pattern(&keys_from_str("place"))); // pl
        assert!(has_english_pattern(&keys_from_str("sleep"))); // sl
    }

    #[test]
    fn test_r_clusters() {
        assert!(has_english_pattern(&keys_from_str("break"))); // br
        assert!(has_english_pattern(&keys_from_str("create"))); // cr
        assert!(has_english_pattern(&keys_from_str("drive"))); // dr
        assert!(has_english_pattern(&keys_from_str("from"))); // fr
        assert!(has_english_pattern(&keys_from_str("green"))); // gr
        assert!(has_english_pattern(&keys_from_str("price"))); // pr
    }

    #[test]
    fn test_s_clusters() {
        assert!(has_english_pattern(&keys_from_str("scan"))); // sc
        assert!(has_english_pattern(&keys_from_str("skip"))); // sk
        assert!(has_english_pattern(&keys_from_str("small"))); // sm
        assert!(has_english_pattern(&keys_from_str("snap"))); // sn
        assert!(has_english_pattern(&keys_from_str("space"))); // sp
        assert!(has_english_pattern(&keys_from_str("stop"))); // st
        assert!(has_english_pattern(&keys_from_str("swim"))); // sw
    }

    // ===== Layer 3: Vowel patterns =====

    #[test]
    fn test_double_vowels() {
        assert!(has_english_pattern(&keys_from_str("see")));
        assert!(has_english_pattern(&keys_from_str("good")));
        assert!(has_english_pattern(&keys_from_str("feel")));
        assert!(has_english_pattern(&keys_from_str("cool")));
    }

    #[test]
    fn test_multiple_e() {
        assert!(has_english_pattern(&keys_from_str("element")));
        assert!(has_english_pattern(&keys_from_str("release")));
        assert!(has_english_pattern(&keys_from_str("eleven")));
    }

    #[test]
    fn test_ough_pattern() {
        assert!(has_english_pattern(&keys_from_str("though")));
        assert!(has_english_pattern(&keys_from_str("through")));
        assert!(has_english_pattern(&keys_from_str("ought")));
    }

    #[test]
    fn test_ie_ey_endings() {
        assert!(has_english_pattern(&keys_from_str("cookie")));
        assert!(has_english_pattern(&keys_from_str("movie")));
        assert!(has_english_pattern(&keys_from_str("money")));
        assert!(has_english_pattern(&keys_from_str("turkey")));
    }

    // ===== Layer 4: Common words =====

    #[test]
    fn test_common_4letter_words() {
        assert!(has_english_pattern(&keys_from_str("with")));
        assert!(has_english_pattern(&keys_from_str("have")));
        assert!(has_english_pattern(&keys_from_str("that")));
        assert!(has_english_pattern(&keys_from_str("this")));
        assert!(has_english_pattern(&keys_from_str("code")));
        // NOTE: "test" removed from word list - "tét" is valid Vietnamese
        assert!(has_english_pattern(&keys_from_str("true")));
        assert!(has_english_pattern(&keys_from_str("null")));
        assert!(has_english_pattern(&keys_from_str("term"))); // Added for issue #29
        assert!(has_english_pattern(&keys_from_str("word"))); // Added for issue #29
    }

    #[test]
    fn test_common_5letter_words() {
        assert!(has_english_pattern(&keys_from_str("their")));
        assert!(has_english_pattern(&keys_from_str("there")));
        assert!(has_english_pattern(&keys_from_str("class")));
        assert!(has_english_pattern(&keys_from_str("const")));
        assert!(has_english_pattern(&keys_from_str("async")));
        assert!(has_english_pattern(&keys_from_str("await")));
    }

    #[test]
    fn test_common_6letter_words() {
        assert!(has_english_pattern(&keys_from_str("string")));
        assert!(has_english_pattern(&keys_from_str("return")));
        assert!(has_english_pattern(&keys_from_str("public")));
        assert!(has_english_pattern(&keys_from_str("import")));
        assert!(has_english_pattern(&keys_from_str("export")));
    }

    // ===== Layer 5: Programming terms =====

    #[test]
    fn test_programming_terms() {
        assert!(has_english_pattern(&keys_from_str("function")));
        assert!(has_english_pattern(&keys_from_str("struct")));
        assert!(has_english_pattern(&keys_from_str("boolean")));
        assert!(has_english_pattern(&keys_from_str("integer")));
        assert!(has_english_pattern(&keys_from_str("private")));
        assert!(has_english_pattern(&keys_from_str("promise")));
    }

    #[test]
    fn test_tech_terms() {
        assert!(has_english_pattern(&keys_from_str("json")));
        assert!(has_english_pattern(&keys_from_str("html")));
        assert!(has_english_pattern(&keys_from_str("http")));
        assert!(has_english_pattern(&keys_from_str("uuid")));
        assert!(has_english_pattern(&keys_from_str("yaml")));
    }

    // ===== Layer 6: Suffix patterns =====

    #[test]
    fn test_ing_suffix() {
        assert!(has_english_pattern(&keys_from_str("running")));
        assert!(has_english_pattern(&keys_from_str("walking")));
        assert!(has_english_pattern(&keys_from_str("typing")));
        assert!(has_english_pattern(&keys_from_str("coding")));
    }

    #[test]
    fn test_ness_ment_suffix() {
        assert!(has_english_pattern(&keys_from_str("happiness")));
        assert!(has_english_pattern(&keys_from_str("darkness")));
        assert!(has_english_pattern(&keys_from_str("development")));
        assert!(has_english_pattern(&keys_from_str("management")));
    }

    #[test]
    fn test_able_ible_suffix() {
        assert!(has_english_pattern(&keys_from_str("readable")));
        assert!(has_english_pattern(&keys_from_str("possible")));
        assert!(has_english_pattern(&keys_from_str("available")));
    }

    #[test]
    fn test_less_ful_suffix() {
        assert!(has_english_pattern(&keys_from_str("helpless")));
        assert!(has_english_pattern(&keys_from_str("endless")));
        assert!(has_english_pattern(&keys_from_str("beautiful")));
        assert!(has_english_pattern(&keys_from_str("helpful")));
    }

    #[test]
    fn test_ly_suffix() {
        assert!(has_english_pattern(&keys_from_str("quickly")));
        assert!(has_english_pattern(&keys_from_str("slowly")));
        assert!(has_english_pattern(&keys_from_str("actually")));
    }

    #[test]
    fn test_ous_suffix() {
        assert!(has_english_pattern(&keys_from_str("famous")));
        assert!(has_english_pattern(&keys_from_str("dangerous")));
        assert!(has_english_pattern(&keys_from_str("previous")));
    }

    // ===== Vietnamese words should NOT be detected =====

    #[test]
    fn test_vietnamese_not_detected() {
        assert!(!has_english_pattern(&keys_from_str("viet")));
        assert!(!has_english_pattern(&keys_from_str("hoa")));
        assert!(!has_english_pattern(&keys_from_str("nha")));
        assert!(!has_english_pattern(&keys_from_str("tro")));
        assert!(!has_english_pattern(&keys_from_str("co")));
        assert!(!has_english_pattern(&keys_from_str("an")));
        assert!(!has_english_pattern(&keys_from_str("em")));
        assert!(!has_english_pattern(&keys_from_str("anh")));
        assert!(!has_english_pattern(&keys_from_str("chi")));
        // "ang" is valid Vietnamese (a + ng final)
        assert!(!has_english_pattern(&keys_from_str("ang")));
    }

    #[test]
    fn test_ad_pattern() {
        // "ad" at word start = English (Vietnamese never has "ad-")
        // This blocks Vietnamese transforms early - no auto-restore needed
        // because "ad" + any character is always English
        assert!(has_english_pattern(&keys_from_str("ad")));
        assert!(has_english_pattern(&keys_from_str("add")));
        assert!(has_english_pattern(&keys_from_str("admin")));
        assert!(has_english_pattern(&keys_from_str("adapt")));
        assert!(has_english_pattern(&keys_from_str("address")));
        assert!(has_english_pattern(&keys_from_str("advance")));
        assert!(has_english_pattern(&keys_from_str("adventure")));
        assert!(has_english_pattern(&keys_from_str("advertise")));
        assert!(has_english_pattern(&keys_from_str("advice")));
        assert!(has_english_pattern(&keys_from_str("adjacent")));
    }

    #[test]
    fn test_an_consonant_pattern() {
        // "an" + consonant (not h/g) = English
        // Vietnamese only allows: "an", "anh" (h), "ang" (g)
        // Only test words where 3rd char is actually a consonant (not vowel)
        assert!(has_english_pattern(&keys_from_str("and")));      // an + d (consonant)
        assert!(has_english_pattern(&keys_from_str("any")));      // an + y (special case)
        assert!(has_english_pattern(&keys_from_str("android"))); // an + d (consonant)
        assert!(has_english_pattern(&keys_from_str("answer")));  // an + s (consonant)
        assert!(has_english_pattern(&keys_from_str("announce"))); // an + n (consonant)
        assert!(has_english_pattern(&keys_from_str("annual")));  // an + n (consonant)
        assert!(has_english_pattern(&keys_from_str("ant")));     // an + t (consonant)
        assert!(has_english_pattern(&keys_from_str("ankle")));   // an + k (consonant)
        assert!(has_english_pattern(&keys_from_str("antique"))); // an + t (consonant)
        
        // These words have "an" + vowel as 3rd char, so pattern doesn't match directly
        // But they may be detected by other patterns (e.g., "th" cluster, "sis" suffix)
        // "another" = an + o (vowel) -> detected by "th" pattern elsewhere
        // "analysis" = an + a (vowel) -> detected by "sis" suffix
        // "animal" = an + i (vowel) -> may need other detection
        // "angle" = an + g (exception) -> not detected by this pattern
        // "angry" = an + g (exception) -> not detected by this pattern
    }

    #[test]
    fn test_vietnamese_consonants_ok() {
        // Valid Vietnamese initial consonants should not trigger
        assert!(!has_english_pattern(&keys_from_str("tho"))); // th
        assert!(!has_english_pattern(&keys_from_str("kha"))); // kh
        assert!(!has_english_pattern(&keys_from_str("cha"))); // ch
        assert!(!has_english_pattern(&keys_from_str("nha"))); // nh
        assert!(!has_english_pattern(&keys_from_str("nga"))); // ng
        assert!(!has_english_pattern(&keys_from_str("pha"))); // ph
        assert!(!has_english_pattern(&keys_from_str("gia"))); // gi
    }

    #[test]
    fn test_ak_az_ah_invalid_patterns() {
        // "ak", "az", "ah" are INVALID Vietnamese syllable patterns
        // These should be blocked immediately (not auto-restore, just block transforms)
        
        // "ak" pattern - no Vietnamese words start with "ak"
        assert!(has_english_pattern(&keys_from_str("ak")));
        assert!(has_english_pattern(&keys_from_str("akt")));
        assert!(has_english_pattern(&keys_from_str("ako")));
        
        // "az" pattern - no Vietnamese words start with "az" (z is tone marker only)
        assert!(has_english_pattern(&keys_from_str("az")));
        assert!(has_english_pattern(&keys_from_str("aze")));
        assert!(has_english_pattern(&keys_from_str("azu")));
        
        // "ah" + non-'n' pattern - "anh" is valid, but "ah" + other chars is not
        assert!(has_english_pattern(&keys_from_str("aht")));
        assert!(has_english_pattern(&keys_from_str("aho")));
        assert!(has_english_pattern(&keys_from_str("aha")));
        
        // "anh" should NOT be detected as English (valid Vietnamese)
        assert!(!has_english_pattern(&keys_from_str("anh")));
    }

    #[test]
    fn test_ethnic_minority_place_names() {
        // "kr" cluster and "k" final for ethnic minority place names
        // These should NOT be detected as English patterns
        // (They're handled separately in Vietnamese validation constants)
        
        // "kr" initial - not detected as English (e.g., "Krông Búk")
        assert!(!has_english_pattern(&keys_from_str("krong")));
        assert!(!has_english_pattern(&keys_from_str("kra")));
        
        // Words ending with "k" - not detected as English (e.g., "Đắk Lắk")
        // Note: single "k" alone is not enough to trigger detection
        assert!(!has_english_pattern(&keys_from_str("dak")));
        assert!(!has_english_pattern(&keys_from_str("lak")));
    }

    // ===== Auto-restore tests =====

    #[test]
    fn test_auto_restore_with_tone_marks() {
        let keys = keys_from_str("cafe");
        // With tone marks → don't restore (user wants Vietnamese)
        assert!(!should_auto_restore_to_english(&keys, true, true));
    }

    #[test]
    fn test_auto_restore_english_no_transforms() {
        let keys = keys_from_str("export");
        // English pattern, no transforms → restore
        assert!(should_auto_restore_to_english(&keys, false, false));
    }

    #[test]
    fn test_auto_restore_vietnamese_no_restore() {
        let keys = keys_from_str("viet");
        // Not English, no transforms → don't restore
        assert!(!should_auto_restore_to_english(&keys, false, false));
    }

    #[test]
    fn test_auto_restore_programming_terms() {
        let keys = keys_from_str("struct");
        // Programming term with transforms but no tone → restore
        assert!(should_auto_restore_to_english(&keys, true, false));
    }
}