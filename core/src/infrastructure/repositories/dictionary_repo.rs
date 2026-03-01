//! Dictionary Repository
//!
//! The English word dictionary was removed in Sprint C (Vietnamese-first refactor).
//! Vietnamese syllable lookups are now provided by `data::viet_syllables`.
//!
//! This stub is kept to avoid breaking the public module tree; callers should
//! migrate to `is_valid_vietnamese_syllable` from `goxviet_core::data::viet_syllables`.

use crate::domain::value_objects::char_sequence::CharSequence;

/// Repository stub — English dictionary removed in Sprint C.
#[derive(Debug, Clone, Copy, Default)]
pub struct DictionaryRepo;

impl DictionaryRepo {
    pub fn new() -> Self {
        Self
    }

    /// Always returns false — English dictionary removed.
    pub fn is_english_keys(&self, _keys: &[u16]) -> bool {
        false
    }

    /// Always returns false — English dictionary removed.
    pub fn is_common_english_word(&self, _raw_keys: &[(u16, bool)]) -> bool {
        false
    }

    /// Always returns false — English dictionary removed.
    pub fn is_english_text(&self, _text: &CharSequence) -> bool {
        false
    }
}
