//! Vietnamese Mark Adapter
//!
//! Implementation of MarkTransformer for Vietnamese diacritic mark application.
//!
//! Handles application and removal of diacritic marks (circumflex, horn, breve)
//! on Vietnamese vowels while preserving tone marks.

use crate::data::chars::{parse_char, to_char, tone as char_tone};
use crate::domain::{
    ports::transformation::mark_transformer::{MarkTransformer, MarkType},
    value_objects::{char_sequence::CharSequence, transformation::TransformResult},
};

/// Vietnamese mark adapter implementing diacritic mark transformation
///
/// This adapter applies Vietnamese diacritic marks (circumflex, horn, breve)
/// to vowels while preserving existing tone marks.
///
/// # Supported Marks
///
/// - `Circumflex (^)`: a→â, e→ê, o→ô
/// - `Horn (ʼ)`: o→ơ, u→ư
/// - `Breve (˘)`: a→ă
///
/// # Examples
///
/// ```
/// use goxviet_core::infrastructure::adapters::transformation::vietnamese_mark_adapter::VietnameseMarkAdapter;
/// use goxviet_core::domain::ports::transformation::mark_transformer::{MarkTransformer, MarkType};
/// use goxviet_core::domain::value_objects::char_sequence::CharSequence;
///
/// let adapter = VietnameseMarkAdapter::new();
/// let text = CharSequence::from("a");
/// let result = adapter.apply_mark(&text, MarkType::Circumflex, 0);
///
/// assert!(result.is_modified());
/// assert_eq!(result.new_text().as_str(), "â");
/// ```
#[derive(Debug, Clone, Default)]
pub struct VietnameseMarkAdapter;

impl VietnameseMarkAdapter {
    /// Create a new Vietnamese mark adapter
    pub fn new() -> Self {
        Self
    }

    /// Map MarkType to tone modifier value
    ///
    /// Note: For 'a', Breve is represented as HORN in the char system
    /// (both use tone modifier value 2, but apply to different base keys)
    fn map_mark_to_tone_modifier(mark: MarkType, key: u16) -> u8 {
        use crate::data::keys;

        match mark {
            MarkType::Circumflex => match key {
                keys::A | keys::E | keys::O => char_tone::CIRCUMFLEX,
                _ => char_tone::NONE,
            },
            MarkType::Horn => match key {
                keys::O | keys::U => char_tone::HORN,
                _ => char_tone::NONE,
            },
            MarkType::Breve => match key {
                keys::A => char_tone::HORN, // Breve for 'a' uses HORN value
                _ => char_tone::NONE,
            },
        }
    }

    /// Check if a mark can be applied to a specific key
    fn can_apply_mark(mark: MarkType, key: u16) -> bool {
        use crate::data::keys;

        match mark {
            MarkType::Circumflex => matches!(key, keys::A | keys::E | keys::O),
            MarkType::Horn => matches!(key, keys::O | keys::U),
            MarkType::Breve => key == keys::A,
        }
    }
}

impl MarkTransformer for VietnameseMarkAdapter {
    fn apply_mark(&self, text: &CharSequence, mark: MarkType, position: usize) -> TransformResult {
        let text_str = text.as_str();
        let chars: Vec<char> = text_str.chars().collect();

        // Bounds check
        if position >= chars.len() {
            return TransformResult::none();
        }

        // Parse the character at the position
        let ch = chars[position];
        let parsed = match parse_char(ch) {
            Some(p) => p,
            None => return TransformResult::none(),
        };

        // Check if mark can be applied to this key
        if !Self::can_apply_mark(mark, parsed.key) {
            return TransformResult::none();
        }

        // Get the new tone modifier for this mark
        let new_tone_modifier = Self::map_mark_to_tone_modifier(mark, parsed.key);

        // If no change, return none
        if new_tone_modifier == parsed.tone {
            return TransformResult::none();
        }

        // Rebuild the character with new tone modifier, preserving the tone mark
        let new_ch = match to_char(parsed.key, parsed.caps, new_tone_modifier, parsed.mark) {
            Some(c) => c,
            None => return TransformResult::none(),
        };

        // Rebuild the text with the modified character
        let mut new_text = String::new();
        for (idx, &c) in chars.iter().enumerate() {
            if idx == position {
                new_text.push(new_ch);
            } else {
                new_text.push(c);
            }
        }

        let backspace_count = text.len() as u8;
        TransformResult::replace(backspace_count, new_text)
    }

    fn remove_mark(&self, text: &CharSequence, position: usize) -> TransformResult {
        let text_str = text.as_str();
        let chars: Vec<char> = text_str.chars().collect();

        // Bounds check
        if position >= chars.len() {
            return TransformResult::none();
        }

        // Parse the character at the position
        let ch = chars[position];
        let parsed = match parse_char(ch) {
            Some(p) => p,
            None => return TransformResult::none(),
        };

        // If already has no tone modifier, return none
        if parsed.tone == char_tone::NONE {
            return TransformResult::none();
        }

        // Rebuild the character with no tone modifier, preserving the tone mark
        let new_ch = match to_char(parsed.key, parsed.caps, char_tone::NONE, parsed.mark) {
            Some(c) => c,
            None => return TransformResult::none(),
        };

        // Rebuild the text with the modified character
        let mut new_text = String::new();
        for (idx, &c) in chars.iter().enumerate() {
            if idx == position {
                new_text.push(new_ch);
            } else {
                new_text.push(c);
            }
        }

        let backspace_count = text.len() as u8;
        TransformResult::replace(backspace_count, new_text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_circumflex_to_a() {
        let adapter = VietnameseMarkAdapter::new();
        let text = CharSequence::from("a");
        let result = adapter.apply_mark(&text, MarkType::Circumflex, 0);

        assert!(result.is_modified());
        assert_eq!(result.new_text().as_str(), "â");
        assert_eq!(result.backspace_count(), 1);
    }

    #[test]
    fn test_apply_circumflex_to_e() {
        let adapter = VietnameseMarkAdapter::new();
        let text = CharSequence::from("e");
        let result = adapter.apply_mark(&text, MarkType::Circumflex, 0);

        assert!(result.is_modified());
        assert_eq!(result.new_text().as_str(), "ê");
    }

    #[test]
    fn test_apply_circumflex_to_o() {
        let adapter = VietnameseMarkAdapter::new();
        let text = CharSequence::from("o");
        let result = adapter.apply_mark(&text, MarkType::Circumflex, 0);

        assert!(result.is_modified());
        assert_eq!(result.new_text().as_str(), "ô");
    }

    #[test]
    fn test_apply_horn_to_o() {
        let adapter = VietnameseMarkAdapter::new();
        let text = CharSequence::from("o");
        let result = adapter.apply_mark(&text, MarkType::Horn, 0);

        assert!(result.is_modified());
        assert_eq!(result.new_text().as_str(), "ơ");
    }

    #[test]
    fn test_apply_horn_to_u() {
        let adapter = VietnameseMarkAdapter::new();
        let text = CharSequence::from("u");
        let result = adapter.apply_mark(&text, MarkType::Horn, 0);

        assert!(result.is_modified());
        assert_eq!(result.new_text().as_str(), "ư");
    }

    #[test]
    fn test_apply_breve_to_a() {
        let adapter = VietnameseMarkAdapter::new();
        let text = CharSequence::from("a");
        let result = adapter.apply_mark(&text, MarkType::Breve, 0);

        assert!(result.is_modified());
        assert_eq!(result.new_text().as_str(), "ă");
    }

    #[test]
    fn test_apply_mark_invalid_vowel() {
        let adapter = VietnameseMarkAdapter::new();
        // Cannot apply circumflex to 'i'
        let text = CharSequence::from("i");
        let result = adapter.apply_mark(&text, MarkType::Circumflex, 0);

        assert!(!result.is_modified());
        assert!(result.is_none());
    }

    #[test]
    fn test_apply_mark_invalid_position() {
        let adapter = VietnameseMarkAdapter::new();
        let text = CharSequence::from("a");
        let result = adapter.apply_mark(&text, MarkType::Circumflex, 5);

        assert!(!result.is_modified());
        assert!(result.is_none());
    }

    #[test]
    fn test_apply_horn_to_invalid_vowel() {
        let adapter = VietnameseMarkAdapter::new();
        // Cannot apply horn to 'a'
        let text = CharSequence::from("a");
        let result = adapter.apply_mark(&text, MarkType::Horn, 0);

        assert!(!result.is_modified());
        assert!(result.is_none());
    }

    #[test]
    fn test_apply_breve_to_invalid_vowel() {
        let adapter = VietnameseMarkAdapter::new();
        // Cannot apply breve to 'e'
        let text = CharSequence::from("e");
        let result = adapter.apply_mark(&text, MarkType::Breve, 0);

        assert!(!result.is_modified());
        assert!(result.is_none());
    }

    #[test]
    fn test_remove_mark_from_circumflex() {
        let adapter = VietnameseMarkAdapter::new();
        let text = CharSequence::from("â");
        let result = adapter.remove_mark(&text, 0);

        assert!(result.is_modified());
        assert_eq!(result.new_text().as_str(), "a");
    }

    #[test]
    fn test_remove_mark_from_horn() {
        let adapter = VietnameseMarkAdapter::new();
        let text = CharSequence::from("ơ");
        let result = adapter.remove_mark(&text, 0);

        assert!(result.is_modified());
        assert_eq!(result.new_text().as_str(), "o");
    }

    #[test]
    fn test_remove_mark_from_breve() {
        let adapter = VietnameseMarkAdapter::new();
        let text = CharSequence::from("ă");
        let result = adapter.remove_mark(&text, 0);

        assert!(result.is_modified());
        assert_eq!(result.new_text().as_str(), "a");
    }

    #[test]
    fn test_remove_mark_no_modifier() {
        let adapter = VietnameseMarkAdapter::new();
        let text = CharSequence::from("a");
        let result = adapter.remove_mark(&text, 0);

        // No mark to remove
        assert!(!result.is_modified());
        assert!(result.is_none());
    }

    #[test]
    fn test_preserve_tone_mark_when_applying_modifier() {
        let adapter = VietnameseMarkAdapter::new();
        // Apply circumflex to 'á' (a with tone mark)
        let text = CharSequence::from("á");
        let result = adapter.apply_mark(&text, MarkType::Circumflex, 0);

        assert!(result.is_modified());
        assert_eq!(result.new_text().as_str(), "ấ"); // Should preserve sac tone mark
    }

    #[test]
    fn test_preserve_tone_mark_when_removing_modifier() {
        let adapter = VietnameseMarkAdapter::new();
        // Remove circumflex from 'ấ' (â with sac tone)
        let text = CharSequence::from("ấ");
        let result = adapter.remove_mark(&text, 0);

        assert!(result.is_modified());
        assert_eq!(result.new_text().as_str(), "á"); // Should preserve sac tone mark
    }

    #[test]
    fn test_apply_mark_in_word() {
        let adapter = VietnameseMarkAdapter::new();
        let text = CharSequence::from("hoa");
        // Apply circumflex to 'o' at position 1
        let result = adapter.apply_mark(&text, MarkType::Circumflex, 1);

        assert!(result.is_modified());
        assert_eq!(result.new_text().as_str(), "hôa");
        assert_eq!(result.backspace_count(), 3);
    }

    #[test]
    fn test_no_change_when_already_has_mark() {
        let adapter = VietnameseMarkAdapter::new();
        let text = CharSequence::from("â");
        // Try to apply circumflex again
        let result = adapter.apply_mark(&text, MarkType::Circumflex, 0);

        assert!(!result.is_modified());
        assert!(result.is_none());
    }
}
