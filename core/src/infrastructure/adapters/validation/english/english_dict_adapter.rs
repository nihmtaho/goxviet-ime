use crate::data::is_english_word;

/// Dictionary-based English word detection adapter.
/// Wraps the 18k-word lookup for use in the language detection pipeline.
pub struct EnglishDictAdapter;

impl EnglishDictAdapter {
    pub fn new() -> Self {
        Self
    }

    /// Returns 100 if text is a known English word, 0 otherwise.
    pub fn confidence(&self, text: &str) -> u8 {
        if is_english_word(text) { 100 } else { 0 }
    }

    pub fn is_english(&self, text: &str) -> bool {
        is_english_word(text)
    }
}

impl Default for EnglishDictAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_word_returns_100() {
        let a = EnglishDictAdapter::new();
        assert_eq!(a.confidence("text"), 100);
        assert_eq!(a.confidence("expect"), 100);
    }

    #[test]
    fn unknown_returns_0() {
        let a = EnglishDictAdapter::new();
        assert_eq!(a.confidence("xin"), 0);
        assert_eq!(a.confidence(""), 0);
    }
}
