//! Vietnamese Language Data Modules
//!
//! This module contains all linguistic data for Vietnamese input:
//! - `keys`: Virtual keycode definitions (platform-specific)
//! - `chars`: Unicode character conversion (includes tone/mark constants)
//! - `vowel`: Vietnamese vowel phonology system
//! - `double_consonant`: English words with tone-marker double consonants (ff/ss/rr/xx/jj)

pub mod auto_capitalise;
pub mod chars;
pub mod constants;
pub mod double_consonant;
pub mod keys;
pub mod viet_syllables;
pub mod vowel;

pub use chars::{get_d, mark, to_char, tone};
pub use constants::*;
pub use double_consonant::is_double_consonant_word;
pub use keys::{is_break, is_letter, is_vowel};
pub use vowel::{Modifier, Phonology, Role, Vowel};
