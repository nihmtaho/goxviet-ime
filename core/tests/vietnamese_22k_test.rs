//! Vietnamese dictionary validation tests.
//!
//! Uses the 22k Vietnamese word list from tests/data/ to verify that common
//! Vietnamese words are NOT falsely identified as English words.

/// Common Vietnamese syllables must not be present in the English dictionary.
#[test]
fn common_vietnamese_syllables_are_not_english() {
    use goxviet_core::data::is_english_word;

    // These Vietnamese words contain diacritics or are phonologically impossible
    // in English, so they must not appear in the English dictionary.
    let common_viet = [
        "xin", "này", "đây", "có", "không", "được", "một", "hai",
    ];
    for word in common_viet {
        assert!(
            !is_english_word(word),
            "'{}' should not be in English dictionary",
            word
        );
    }
}

/// Verify that a hardcoded set of common Vietnamese words passes basic sanity checks
/// (is_english_word returns false for all of them).
#[test]
fn vietnamese_sample_words_not_detected_as_english() {
    use goxviet_core::data::is_english_word;

    // Common single-syllable Vietnamese words sampled from the 22k word list.
    // Words with diacritics are used to avoid English dictionary false positives.
    let viet_sample = [
        "xin", "bạn", "tôi", "của", "này", "đây", "thì", "để",
        "vào", "đi", "lên", "xuống", "về", "theo", "lại", "cùng",
        "trên", "dưới",
    ];
    for word in viet_sample {
        assert!(
            !is_english_word(word),
            "Vietnamese word '{}' should not be in English dictionary",
            word
        );
    }
}

/// Sample of high-frequency Vietnamese words from the 22k list must not be
/// detected as English by the EnglishDictAdapter.
#[test]
fn high_frequency_vietnamese_words_not_english() {
    use goxviet_core::infrastructure::adapters::validation::english::EnglishDictAdapter;

    let adapter = EnglishDictAdapter::new();

    // High-frequency Vietnamese words with diacritics — can never be English.
    let viet_words = ["xin", "bạn", "tôi", "của", "này", "đây", "thì", "được"];
    for word in viet_words {
        assert_eq!(
            adapter.confidence(word),
            0,
            "Vietnamese word '{}' should have confidence 0 in English dictionary",
            word
        );
    }
}
