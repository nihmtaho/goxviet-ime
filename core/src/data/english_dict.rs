//! 18k common English words for auto-restore detection.
//! Binary search on sorted static slice — zero heap allocation on each call.

const WORDS_RAW: &str = include_str!("english_dict_merged.txt");

use std::sync::OnceLock;

static SORTED_WORDS: OnceLock<Vec<&'static str>> = OnceLock::new();

fn words() -> &'static [&'static str] {
    SORTED_WORDS.get_or_init(|| {
        let mut v: Vec<&'static str> = WORDS_RAW.lines().filter(|l| !l.is_empty()).collect();
        v.sort_unstable();
        v.dedup();
        v
    })
}

/// Returns true if `text` matches a known English word (case-insensitive).
pub fn is_english_word(text: &str) -> bool {
    if text.is_empty() {
        return false;
    }
    let lower: std::borrow::Cow<str> = if text.bytes().all(|b| !b.is_ascii_uppercase()) {
        std::borrow::Cow::Borrowed(text)
    } else {
        std::borrow::Cow::Owned(text.to_lowercase())
    };
    words().binary_search(&lower.as_ref()).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_english_words() {
        assert!(is_english_word("the"));
        assert!(is_english_word("computer"));
        assert!(is_english_word("text"));
        assert!(is_english_word("expect"));
    }

    #[test]
    fn case_insensitive() {
        assert!(is_english_word("The"));
        assert!(is_english_word("COMPUTER"));
    }

    #[test]
    fn non_english_returns_false() {
        assert!(!is_english_word("xin"));
        assert!(!is_english_word("chào"));
        assert!(!is_english_word(""));
    }
}
