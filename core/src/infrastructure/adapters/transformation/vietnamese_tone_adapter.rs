//! Vietnamese Tone Adapter
//!
//! Implementation of ToneTransformer for Vietnamese tone mark application.
//!
//! Uses the data module's character transformation utilities and vowel phonology
//! rules to correctly place tone marks on Vietnamese syllables.

use crate::data::{
    chars::{mark as char_mark, parse_char, to_char, tone as char_tone},
    vowel::{Modifier, Phonology, Vowel},
};
use crate::domain::{
    entities::{syllable::Syllable, tone::ToneType},
    ports::transformation::tone_transformer::{ToneStrategy, ToneTransformer},
    value_objects::transformation::TransformResult,
};

/// Vietnamese tone adapter implementing tone transformation
///
/// This adapter applies Vietnamese tone marks to syllables following
/// modern or traditional positioning rules.
///
/// # Strategy
///
/// - `Modern`: Follows Bộ GD&ĐT 2018 rules (tone on main vowel)
/// - `Traditional`: Pre-2018 rules (tone on first vowel in some cases)
/// - `Auto`: Defaults to Modern
///
/// # Examples
///
/// ```
/// use goxviet_core::infrastructure::adapters::transformation::vietnamese_tone_adapter::VietnameseToneAdapter;
/// use goxviet_core::domain::ports::transformation::tone_transformer::{ToneTransformer, ToneStrategy};
/// use goxviet_core::domain::entities::{syllable::Syllable, tone::ToneType};
///
/// let adapter = VietnameseToneAdapter::new(ToneStrategy::Modern);
/// let syllable = Syllable::from_parts("h", "oa", "", ToneType::Ngang);
/// let result = adapter.apply_tone(&syllable, ToneType::Sac);
///
/// assert!(result.is_modified());
/// assert_eq!(result.new_text().as_str(), "hoá");
/// ```
#[derive(Debug, Clone)]
pub struct VietnameseToneAdapter {
    strategy: ToneStrategy,
}

impl VietnameseToneAdapter {
    /// Create a new Vietnamese tone adapter with the specified strategy
    ///
    /// # Arguments
    ///
    /// * `strategy` - The tone positioning strategy to use
    pub fn new(strategy: ToneStrategy) -> Self {
        Self { strategy }
    }

    /// Build a list of vowels from the syllable's vowel string
    ///
    /// Parses each character in the vowel string and extracts its key and tone modifier.
    fn build_vowel_list(vowel_str: &str) -> Vec<Vowel> {
        vowel_str
            .chars()
            .enumerate()
            .filter_map(|(pos, ch)| {
                parse_char(ch).map(|parsed| {
                    let modifier = match parsed.tone {
                        char_tone::CIRCUMFLEX => Modifier::Circumflex,
                        char_tone::HORN => Modifier::Horn,
                        _ => Modifier::None,
                    };
                    Vowel::new(parsed.key, modifier, pos)
                })
            })
            .collect()
    }

    /// Determine if we should use modern positioning rules
    fn use_modern_rules(&self) -> bool {
        match self.strategy {
            ToneStrategy::Modern => true,
            ToneStrategy::Traditional => false,
            ToneStrategy::Auto => true, // Auto defaults to Modern
        }
    }

    /// Check if syllable has 'qu' initial
    fn has_qu_initial(initial: &str) -> bool {
        initial.to_lowercase() == "qu"
    }

    /// Check if syllable has 'gi' initial
    fn has_gi_initial(initial: &str) -> bool {
        initial.to_lowercase() == "gi"
    }

    /// Map ToneType to char_mark value
    fn map_tone_to_mark(tone: ToneType) -> u8 {
        match tone {
            ToneType::Ngang => char_mark::NONE,
            ToneType::Sac => char_mark::SAC,
            ToneType::Huyen => char_mark::HUYEN,
            ToneType::Hoi => char_mark::HOI,
            ToneType::Nga => char_mark::NGA,
            ToneType::Nang => char_mark::NANG,
        }
    }

    /// Apply tone mark to a syllable
    ///
    /// This is the core transformation logic that:
    /// 1. Parses the vowel string into individual vowels
    /// 2. Determines the correct position for the tone mark
    /// 3. Applies the mark to that position
    /// 4. Rebuilds the syllable string
    fn transform_syllable(&self, syllable: &Syllable, tone: ToneType) -> TransformResult {
        let initial = syllable.initial().as_str();
        let vowel_str = syllable.vowel().as_str();
        let final_consonant = syllable.final_consonant().as_str();

        // Build vowel list
        let vowels = Self::build_vowel_list(vowel_str);
        if vowels.is_empty() {
            return TransformResult::none();
        }

        // Determine tone position
        let has_final = !final_consonant.is_empty();
        let modern = self.use_modern_rules();
        let has_qu = Self::has_qu_initial(initial);
        let has_gi = Self::has_gi_initial(initial);

        let tone_position =
            Phonology::find_tone_position(&vowels, has_final, modern, has_qu, has_gi);

        // Get the mark to apply
        let mark = Self::map_tone_to_mark(tone);

        // Rebuild vowel string with tone mark applied
        let mut new_vowel = String::new();
        let mut changed = false;

        for (idx, ch) in vowel_str.chars().enumerate() {
            if let Some(parsed) = parse_char(ch) {
                let new_mark = if idx == tone_position {
                    mark
                } else {
                    char_mark::NONE
                };

                // Check if this character needs to change
                if new_mark != parsed.mark {
                    changed = true;
                }

                // Rebuild character with new mark, preserving tone modifier
                if let Some(new_ch) = to_char(parsed.key, parsed.caps, parsed.tone, new_mark) {
                    new_vowel.push(new_ch);
                } else {
                    // If to_char fails, keep original
                    new_vowel.push(ch);
                }
            } else {
                // Keep non-parseable characters as-is
                new_vowel.push(ch);
            }
        }

        if !changed {
            return TransformResult::none();
        }

        // Build the complete syllable
        let new_text = format!("{}{}{}", initial, new_vowel, final_consonant);
        let original_text = format!("{}{}{}", initial, vowel_str, final_consonant);
        let backspace_count = original_text.chars().count() as u8;

        TransformResult::replace(backspace_count, new_text)
    }

    /// Remove tone marks from a syllable
    ///
    /// Clears all tone marks (sets mark to NONE) but preserves tone modifiers
    /// (circumflex, horn, breve).
    fn clear_tone(&self, syllable: &Syllable) -> TransformResult {
        let initial = syllable.initial().as_str();
        let vowel_str = syllable.vowel().as_str();
        let final_consonant = syllable.final_consonant().as_str();

        // Rebuild vowel string with no tone marks
        let mut new_vowel = String::new();
        let mut changed = false;

        for ch in vowel_str.chars() {
            if let Some(parsed) = parse_char(ch) {
                if parsed.mark != char_mark::NONE {
                    changed = true;
                }

                // Rebuild character with no mark, preserving tone modifier
                if let Some(new_ch) = to_char(parsed.key, parsed.caps, parsed.tone, char_mark::NONE)
                {
                    new_vowel.push(new_ch);
                } else {
                    // If to_char fails, keep original
                    new_vowel.push(ch);
                }
            } else {
                // Keep non-parseable characters as-is
                new_vowel.push(ch);
            }
        }

        if !changed {
            return TransformResult::none();
        }

        // Build the complete syllable
        let new_text = format!("{}{}{}", initial, new_vowel, final_consonant);
        let original_text = format!("{}{}{}", initial, vowel_str, final_consonant);
        let backspace_count = original_text.chars().count() as u8;

        TransformResult::replace(backspace_count, new_text)
    }
}

impl Default for VietnameseToneAdapter {
    fn default() -> Self {
        Self::new(ToneStrategy::Modern)
    }
}

impl ToneTransformer for VietnameseToneAdapter {
    fn apply_tone(&self, syllable: &Syllable, tone: ToneType) -> TransformResult {
        self.transform_syllable(syllable, tone)
    }

    fn remove_tone(&self, syllable: &Syllable) -> TransformResult {
        self.clear_tone(syllable)
    }

    fn strategy(&self) -> ToneStrategy {
        self.strategy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_tone_single_vowel() {
        let adapter = VietnameseToneAdapter::default();
        let syllable = Syllable::from_parts("", "a", "", ToneType::Ngang);
        let result = adapter.apply_tone(&syllable, ToneType::Sac);

        assert!(result.is_modified());
        assert_eq!(result.new_text().as_str(), "á");
        assert_eq!(result.backspace_count(), 1);
    }

    #[test]
    fn test_apply_tone_double_vowel_ie() {
        let adapter = VietnameseToneAdapter::new(ToneStrategy::Modern);
        let syllable = Syllable::from_parts("", "iê", "", ToneType::Ngang);
        let result = adapter.apply_tone(&syllable, ToneType::Sac);

        assert!(result.is_modified());
        assert_eq!(result.new_text().as_str(), "iế");
    }

    #[test]
    fn test_apply_tone_hoa_modern() {
        let adapter = VietnameseToneAdapter::new(ToneStrategy::Modern);
        let syllable = Syllable::from_parts("h", "oa", "", ToneType::Ngang);
        let result = adapter.apply_tone(&syllable, ToneType::Sac);

        assert!(result.is_modified());
        assert_eq!(result.new_text().as_str(), "hoá");
        assert_eq!(result.backspace_count(), 3);
    }

    #[test]
    fn test_remove_tone() {
        let adapter = VietnameseToneAdapter::default();
        let syllable = Syllable::from_parts("", "á", "", ToneType::Sac);
        let result = adapter.remove_tone(&syllable);

        assert!(result.is_modified());
        assert_eq!(result.new_text().as_str(), "a");
        assert_eq!(result.backspace_count(), 1);
    }

    #[test]
    fn test_no_change_returns_none() {
        let adapter = VietnameseToneAdapter::default();
        // Already has the tone we're applying
        let syllable = Syllable::from_parts("", "á", "", ToneType::Sac);
        let result = adapter.apply_tone(&syllable, ToneType::Sac);

        assert!(!result.is_modified());
        assert!(result.is_none());
    }

    #[test]
    fn test_empty_vowel_returns_none() {
        let adapter = VietnameseToneAdapter::default();
        let syllable = Syllable::from_parts("h", "", "ng", ToneType::Ngang);
        let result = adapter.apply_tone(&syllable, ToneType::Sac);

        assert!(!result.is_modified());
        assert!(result.is_none());
    }

    #[test]
    fn test_strategy() {
        let adapter = VietnameseToneAdapter::new(ToneStrategy::Traditional);
        assert_eq!(adapter.strategy(), ToneStrategy::Traditional);

        let adapter = VietnameseToneAdapter::default();
        assert_eq!(adapter.strategy(), ToneStrategy::Modern);
    }

    #[test]
    fn test_remove_tone_no_mark() {
        let adapter = VietnameseToneAdapter::default();
        let syllable = Syllable::from_parts("", "a", "", ToneType::Ngang);
        let result = adapter.remove_tone(&syllable);

        // No tone to remove, should return none
        assert!(!result.is_modified());
        assert!(result.is_none());
    }

    #[test]
    fn test_preserve_tone_modifiers() {
        let adapter = VietnameseToneAdapter::default();
        // Test with â (has circumflex modifier)
        let syllable = Syllable::from_parts("", "â", "", ToneType::Ngang);
        let result = adapter.apply_tone(&syllable, ToneType::Sac);

        assert!(result.is_modified());
        assert_eq!(result.new_text().as_str(), "ấ"); // Should preserve circumflex

        // Remove tone should keep modifier
        let syllable_with_tone = Syllable::from_parts("", "ấ", "", ToneType::Sac);
        let result = adapter.remove_tone(&syllable_with_tone);
        assert!(result.is_modified());
        assert_eq!(result.new_text().as_str(), "â"); // Circumflex preserved
    }
}
