//! Dictionary Repository
//!
//! Wrapper around the engine's dictionary for domain layer access.

use crate::data::chars::parse_char;
use crate::domain::value_objects::char_sequence::CharSequence;
use crate::infrastructure::external::english::dictionary::Dictionary;

/// Repository for English dictionary lookups
#[derive(Debug, Clone, Copy, Default)]
pub struct DictionaryRepo;

impl DictionaryRepo {
    /// Create a new dictionary repository
    pub fn new() -> Self {
        Self
    }

    /// Check if a sequence of keys forms an English word
    pub fn is_english_keys(&self, keys: &[u16]) -> bool {
        Dictionary::is_english(keys)
    }

    /// Check if raw keystroke sequence is a common English word
    pub fn is_common_english_word(&self, raw_keys: &[(u16, bool)]) -> bool {
        Dictionary::is_common_english_word(raw_keys)
    }

    /// Check if text is English by parsing each character
    ///
    /// Returns false if:
    /// - Any character cannot be parsed
    /// - The resulting key sequence is empty
    /// - The keys don't form an English word
    pub fn is_english_text(&self, text: &CharSequence) -> bool {
        let mut keys = Vec::new();
        
        for ch in text.chars() {
            match parse_char(ch) {
                Some(parsed) => keys.push(parsed.key),
                None => return false,
            }
        }
        
        if keys.is_empty() {
            return false;
        }
        
        self.is_english_keys(&keys)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::keys;

    #[test]
    fn test_is_english_keys_returns_true_for_of() {
        let repo = DictionaryRepo::new();
        let keys = vec![keys::O, keys::F];
        assert!(repo.is_english_keys(&keys));
    }

    #[test]
    fn test_is_english_text_returns_true_for_of() {
        let repo = DictionaryRepo::new();
        let text = CharSequence::from("of");
        assert!(repo.is_english_text(&text));
    }

    #[test]
    fn test_returns_false_for_a() {
        let repo = DictionaryRepo::new();
        let text = CharSequence::from("a");
        assert!(!repo.is_english_text(&text));
    }
}
