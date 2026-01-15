// dictionary.rs
//
// Optimized dictionary for common words and programming terms
// Uses linear search on sorted arrays for O(n) performance with high cache locality.

use crate::data::keys;

/// Optimized dictionary for common words
pub struct Dictionary;

impl Dictionary {
    /// Check if a sequence of keys is a common English word or programming term
    pub fn is_english(keys: &[u16]) -> bool {
        let raw_keys: Vec<(u16, bool)> = keys.iter().map(|&k| (k, false)).collect();
        Self::is_common_english_word(&raw_keys)
    }

    /// Check if raw keystroke sequence matches a COMMON English word exactly
    pub fn is_common_english_word(raw_keys: &[(u16, bool)]) -> bool {
        if raw_keys.len() < 4 {
            return false;
        }

        let keys_only: Vec<u16> = raw_keys.iter().map(|(k, _)| *k).collect();

        // Check common word patterns (these are unambiguous English)
        has_common_english_word_pattern(&keys_only) || has_programming_term_pattern(&keys_only)
    }
}

/// Constant lookup table for common 4-letter English words
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

#[inline]
fn is_common_4letter_word(word: &[u16; 4]) -> bool {
    COMMON_4LETTER_WORDS.iter().any(|w| w == word)
}

/// Constant lookup table for common 5-letter English words
const COMMON_5LETTER_WORDS: &[[u16; 5]] = &[
    // Common function words
    [keys::T, keys::H, keys::E, keys::I, keys::R], // their
    [keys::T, keys::H, keys::E, keys::R, keys::E], // there
    [keys::T, keys::H, keys::E, keys::S, keys::E], // these
    [keys::O, keys::T, keys::H, keys::E, keys::R], // other
    [keys::W, keys::H, keys::I, keys::C, keys::H], // which
    [keys::W, keys::H, keys::E, keys::R, keys::E], // where
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
    // Common tech terms
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
    [keys::L, keys::A, keys::Y, keys::E, keys::R], // layer
];

#[inline]
fn is_common_5letter_word(word: &[u16; 5]) -> bool {
    COMMON_5LETTER_WORDS.iter().any(|w| w == word)
}

/// Constant lookup table for common 6-letter English words
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
    [keys::B, keys::E, keys::T, keys::T, keys::E, keys::R], // better
    [keys::S, keys::T, keys::R, keys::E, keys::S, keys::S], // stress
];

#[inline]
fn is_common_6letter_word(word: &[u16; 6]) -> bool {
    COMMON_6LETTER_WORDS.iter().any(|w| w == word)
}

/// Constant lookup table for common 7-letter English words
const COMMON_7LETTER_WORDS: &[[u16; 7]] = &[
    [
        keys::I,
        keys::M,
        keys::P,
        keys::R,
        keys::O,
        keys::V,
        keys::E,
    ], // improve
    [
        keys::R,
        keys::E,
        keys::S,
        keys::T,
        keys::O,
        keys::R,
        keys::E,
    ], // restore
    [
        keys::R,
        keys::E,
        keys::L,
        keys::E,
        keys::A,
        keys::S,
        keys::E,
    ], // release
    [
        keys::R,
        keys::E,
        keys::V,
        keys::E,
        keys::R,
        keys::S,
        keys::E,
    ], // reverse
    [
        keys::E,
        keys::X,
        keys::P,
        keys::R,
        keys::E,
        keys::S,
        keys::S,
    ], // express
    [
        keys::E,
        keys::X,
        keys::A,
        keys::M,
        keys::P,
        keys::L,
        keys::E,
    ], // example
    [
        keys::S,
        keys::U,
        keys::P,
        keys::P,
        keys::O,
        keys::R,
        keys::T,
    ], // support
    [
        keys::R,
        keys::E,
        keys::Q,
        keys::U,
        keys::E,
        keys::S,
        keys::T,
    ], // request
    [
        keys::P,
        keys::R,
        keys::O,
        keys::J,
        keys::E,
        keys::C,
        keys::T,
    ], // project
    [
        keys::S,
        keys::E,
        keys::R,
        keys::V,
        keys::I,
        keys::C,
        keys::E,
    ], // service
    [
        keys::C,
        keys::O,
        keys::N,
        keys::T,
        keys::E,
        keys::N,
        keys::T,
    ], // content
    [
        keys::V,
        keys::E,
        keys::R,
        keys::S,
        keys::I,
        keys::O,
        keys::N,
    ], // version
    [
        keys::D,
        keys::I,
        keys::S,
        keys::P,
        keys::L,
        keys::A,
        keys::Y,
    ], // display
];

#[inline]
fn is_common_7letter_word(word: &[u16; 7]) -> bool {
    COMMON_7LETTER_WORDS.iter().any(|w| w == word)
}

/// Constant lookup table for common 8-letter English words
const COMMON_8LETTER_WORDS: &[[u16; 8]] = &[
    [
        keys::G,
        keys::E,
        keys::N,
        keys::E,
        keys::R,
        keys::A,
        keys::T,
        keys::E,
    ], // generate
    [
        keys::R,
        keys::E,
        keys::G,
        keys::I,
        keys::S,
        keys::T,
        keys::E,
        keys::R,
    ], // register
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
        keys::D,
        keys::A,
        keys::T,
        keys::A,
        keys::B,
        keys::A,
        keys::S,
        keys::E,
    ], // database
    [
        keys::L,
        keys::A,
        keys::N,
        keys::G,
        keys::U,
        keys::A,
        keys::G,
        keys::E,
    ], // language
    [
        keys::S,
        keys::E,
        keys::T,
        keys::T,
        keys::I,
        keys::N,
        keys::G,
        keys::S,
    ], // settings
];

#[inline]
fn is_common_8letter_word(word: &[u16; 8]) -> bool {
    COMMON_8LETTER_WORDS.iter().any(|w| w == word)
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
    if len < 4 {
        return false;
    }
    let check_len = len.min(8);

    // Strategy 1: Check from START (original logic)
    if len >= 4 {
        let w4: [u16; 4] = [keys[0], keys[1], keys[2], keys[3]];
        if is_common_4letter_word(&w4) {
            return true;
        }
    }
    if check_len >= 5 {
        let w5: [u16; 5] = [keys[0], keys[1], keys[2], keys[3], keys[4]];
        if is_common_5letter_word(&w5) {
            return true;
        }
    }
    if check_len >= 6 {
        let w6: [u16; 6] = [keys[0], keys[1], keys[2], keys[3], keys[4], keys[5]];
        if is_common_6letter_word(&w6) {
            return true;
        }
    }
    if check_len >= 7 {
        let w7: [u16; 7] = [
            keys[0], keys[1], keys[2], keys[3], keys[4], keys[5], keys[6],
        ];
        if is_common_7letter_word(&w7) {
            return true;
        }
    }
    if check_len >= 8 {
        let w8: [u16; 8] = [
            keys[0], keys[1], keys[2], keys[3], keys[4], keys[5], keys[6], keys[7],
        ];
        if is_common_8letter_word(&w8) {
            return true;
        }
    }

    // Strategy 2: Check from END (for cases like [o,o,v,e,r] -> check [o,v,e,r])
    // This catches transform-prefixed words
    if len >= 5 {
        // Check if last 4 characters form a word
        let start = len - 4;
        let w4: [u16; 4] = [
            keys[start],
            keys[start + 1],
            keys[start + 2],
            keys[start + 3],
        ];
        if is_common_4letter_word(&w4) {
            return true;
        }
    }
    if len >= 6 {
        // Check if last 5 characters form a word
        let start = len - 5;
        let w5: [u16; 5] = [
            keys[start],
            keys[start + 1],
            keys[start + 2],
            keys[start + 3],
            keys[start + 4],
        ];
        if is_common_5letter_word(&w5) {
            return true;
        }
    }
    if len >= 7 {
        // Check if last 6 characters form a word
        let start = len - 6;
        let w6: [u16; 6] = [
            keys[start],
            keys[start + 1],
            keys[start + 2],
            keys[start + 3],
            keys[start + 4],
            keys[start + 5],
        ];
        if is_common_6letter_word(&w6) {
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

    if len >= 4 {
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
