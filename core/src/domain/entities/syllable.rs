//! Syllable Entity - Vietnamese Syllable Structure
//!
//! Core business entity representing a Vietnamese syllable with its components.

use crate::domain::entities::tone::ToneType;
use crate::domain::value_objects::char_sequence::CharSequence;
use std::fmt;

/// Vietnamese syllable entity
///
/// Represents the structure of a Vietnamese syllable:
/// - Initial consonant (phụ âm đầu): Optional, e.g., "tr" in "trời"
/// - Vowel nucleus (vần): Required, e.g., "ươ" in "trời"  
/// - Final consonant (phụ âm cuối): Optional, e.g., "ng" in "trong"
/// - Tone (thanh điệu): Required (can be Ngang/neutral)
///
/// # Business Invariants
/// - Must have at least a vowel (nucleus)
/// - Tone is always present (default is Ngang)
/// - Components must be valid Vietnamese characters
/// - Structure must follow Vietnamese phonotactics
///
/// # Examples
/// ```
/// # use goxviet_core::domain::entities::syllable::Syllable;
/// # use goxviet_core::domain::entities::tone::ToneType;
/// let syllable = Syllable::new()
///     .with_initial("tr")
///     .with_vowel("ươ")
///     .with_final("ng")
///     .with_tone(ToneType::Huyền);
/// 
/// assert_eq!(syllable.to_string(), "trường");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Syllable {
    /// Initial consonant (phụ âm đầu)
    initial: CharSequence,
    /// Vowel nucleus (vần)
    vowel: CharSequence,
    /// Final consonant (phụ âm cuối)
    final_consonant: CharSequence,
    /// Tone mark (thanh điệu)
    tone: ToneType,
}

impl Syllable {
    /// Create a new empty syllable
    ///
    /// Default state has no consonants, no vowel, and Ngang tone.
    pub fn new() -> Self {
        Self {
            initial: CharSequence::empty(),
            vowel: CharSequence::empty(),
            final_consonant: CharSequence::empty(),
            tone: ToneType::Ngang,
        }
    }

    /// Create syllable from components
    pub fn from_parts(
        initial: impl Into<CharSequence>,
        vowel: impl Into<CharSequence>,
        final_consonant: impl Into<CharSequence>,
        tone: ToneType,
    ) -> Self {
        Self {
            initial: initial.into(),
            vowel: vowel.into(),
            final_consonant: final_consonant.into(),
            tone,
        }
    }

    /// Set initial consonant (builder pattern)
    pub fn with_initial(mut self, initial: impl Into<CharSequence>) -> Self {
        self.initial = initial.into();
        self
    }

    /// Set vowel nucleus (builder pattern)
    pub fn with_vowel(mut self, vowel: impl Into<CharSequence>) -> Self {
        self.vowel = vowel.into();
        self
    }

    /// Set final consonant (builder pattern)
    pub fn with_final(mut self, final_consonant: impl Into<CharSequence>) -> Self {
        self.final_consonant = final_consonant.into();
        self
    }

    /// Set tone (builder pattern)
    pub fn with_tone(mut self, tone: ToneType) -> Self {
        self.tone = tone;
        self
    }

    /// Get initial consonant
    #[inline]
    pub fn initial(&self) -> &CharSequence {
        &self.initial
    }

    /// Get vowel nucleus
    #[inline]
    pub fn vowel(&self) -> &CharSequence {
        &self.vowel
    }

    /// Get final consonant
    #[inline]
    pub fn final_consonant(&self) -> &CharSequence {
        &self.final_consonant
    }

    /// Get tone
    #[inline]
    pub fn tone(&self) -> ToneType {
        self.tone
    }

    /// Check if syllable has initial consonant
    #[inline]
    pub fn has_initial(&self) -> bool {
        !self.initial.is_empty()
    }

    /// Check if syllable has vowel
    #[inline]
    pub fn has_vowel(&self) -> bool {
        !self.vowel.is_empty()
    }

    /// Check if syllable has final consonant
    #[inline]
    pub fn has_final(&self) -> bool {
        !self.final_consonant.is_empty()
    }

    /// Check if syllable has tone mark (not Ngang)
    #[inline]
    pub fn has_tone(&self) -> bool {
        self.tone.has_mark()
    }

    /// Check if syllable is valid (has at least a vowel)
    ///
    /// Business rule: A syllable must have at least a vowel nucleus.
    pub fn is_valid(&self) -> bool {
        self.has_vowel()
    }

    /// Check if syllable is empty
    pub fn is_empty(&self) -> bool {
        self.initial.is_empty() 
            && self.vowel.is_empty() 
            && self.final_consonant.is_empty()
    }

    /// Apply tone to this syllable
    ///
    /// This is a business operation that changes the tone.
    pub fn apply_tone(&mut self, tone: ToneType) {
        self.tone = tone;
    }

    /// Set initial consonant
    pub fn set_initial(&mut self, initial: impl Into<CharSequence>) {
        self.initial = initial.into();
    }

    /// Set vowel nucleus
    pub fn set_vowel(&mut self, vowel: impl Into<CharSequence>) {
        self.vowel = vowel.into();
    }

    /// Set final consonant
    pub fn set_final(&mut self, final_consonant: impl Into<CharSequence>) {
        self.final_consonant = final_consonant.into();
    }

    /// Clear all components
    pub fn clear(&mut self) {
        self.initial = CharSequence::empty();
        self.vowel = CharSequence::empty();
        self.final_consonant = CharSequence::empty();
        self.tone = ToneType::Ngang;
    }

    /// Get the base form (without tone)
    ///
    /// Returns the syllable structure without tone marks.
    pub fn base_form(&self) -> String {
        format!(
            "{}{}{}",
            self.initial.as_str(),
            self.vowel.as_str(),
            self.final_consonant.as_str()
        )
    }

    /// Get the full form (with tone)
    ///
    /// This is a simplified version. Actual tone application would be done
    /// by a transformer in the infrastructure layer.
    pub fn full_form(&self) -> String {
        // For now, just return base form
        // Real implementation would apply tone marks to the correct vowel
        // This is placeholder - actual implementation in transformer layer
        self.base_form()
    }

    /// Get syllable structure as tuple
    pub fn as_tuple(&self) -> (&str, &str, &str, ToneType) {
        (
            self.initial.as_str(),
            self.vowel.as_str(),
            self.final_consonant.as_str(),
            self.tone,
        )
    }

    /// Clone this syllable with a different tone
    pub fn with_tone_clone(&self, tone: ToneType) -> Self {
        Self {
            initial: self.initial.clone(),
            vowel: self.vowel.clone(),
            final_consonant: self.final_consonant.clone(),
            tone,
        }
    }

    /// Get the length of the syllable (total characters)
    pub fn len(&self) -> usize {
        self.initial.len() + self.vowel.len() + self.final_consonant.len()
    }

    /// Check if syllable matches a pattern
    ///
    /// Useful for validation and transformation rules.
    pub fn matches_pattern(&self, initial_pattern: &str, vowel_pattern: &str, final_pattern: &str) -> bool {
        (initial_pattern.is_empty() || self.initial.as_str() == initial_pattern)
            && (vowel_pattern.is_empty() || self.vowel.as_str() == vowel_pattern)
            && (final_pattern.is_empty() || self.final_consonant.as_str() == final_pattern)
    }
}

impl Default for Syllable {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Syllable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.full_form())
    }
}

/// Builder for constructing syllables
///
/// Provides a fluent API for building syllables step by step.
pub struct SyllableBuilder {
    syllable: Syllable,
}

impl SyllableBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            syllable: Syllable::new(),
        }
    }

    /// Set initial consonant
    pub fn initial(mut self, initial: impl Into<CharSequence>) -> Self {
        self.syllable.initial = initial.into();
        self
    }

    /// Set vowel nucleus
    pub fn vowel(mut self, vowel: impl Into<CharSequence>) -> Self {
        self.syllable.vowel = vowel.into();
        self
    }

    /// Set final consonant
    pub fn final_consonant(mut self, final_consonant: impl Into<CharSequence>) -> Self {
        self.syllable.final_consonant = final_consonant.into();
        self
    }

    /// Set tone
    pub fn tone(mut self, tone: ToneType) -> Self {
        self.syllable.tone = tone;
        self
    }

    /// Build the syllable
    pub fn build(self) -> Syllable {
        self.syllable
    }

    /// Build and validate
    ///
    /// Returns None if syllable is not valid.
    pub fn build_validated(self) -> Option<Syllable> {
        if self.syllable.is_valid() {
            Some(self.syllable)
        } else {
            None
        }
    }
}

impl Default for SyllableBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_syllable_creation() {
        let syllable = Syllable::new();
        assert!(syllable.is_empty());
        assert!(!syllable.is_valid()); // No vowel
        assert_eq!(syllable.tone(), ToneType::Ngang);
    }

    #[test]
    fn test_syllable_from_parts() {
        let syllable = Syllable::from_parts("tr", "ươ", "ng", ToneType::Huyen);
        assert_eq!(syllable.initial().as_str(), "tr");
        assert_eq!(syllable.vowel().as_str(), "ươ");
        assert_eq!(syllable.final_consonant().as_str(), "ng");
        assert_eq!(syllable.tone(), ToneType::Huyen);
    }

    #[test]
    fn test_syllable_builder_pattern() {
        let syllable = Syllable::new()
            .with_initial("h")
            .with_vowel("a")
            .with_tone(ToneType::Sac);

        assert_eq!(syllable.initial().as_str(), "h");
        assert_eq!(syllable.vowel().as_str(), "a");
        assert_eq!(syllable.tone(), ToneType::Sac);
    }

    #[test]
    fn test_syllable_has_components() {
        let syllable = Syllable::from_parts("h", "a", "", ToneType::Ngang);
        assert!(syllable.has_initial());
        assert!(syllable.has_vowel());
        assert!(!syllable.has_final());
        assert!(!syllable.has_tone()); // Ngang = no mark
    }

    #[test]
    fn test_syllable_is_valid() {
        let valid = Syllable::new().with_vowel("a");
        assert!(valid.is_valid());

        let invalid = Syllable::new(); // No vowel
        assert!(!invalid.is_valid());
    }

    #[test]
    fn test_syllable_apply_tone() {
        let mut syllable = Syllable::new()
            .with_vowel("a");

        assert_eq!(syllable.tone(), ToneType::Ngang);

        syllable.apply_tone(ToneType::Sac);
        assert_eq!(syllable.tone(), ToneType::Sac);
    }

    #[test]
    fn test_syllable_set_components() {
        let mut syllable = Syllable::new();
        
        syllable.set_initial("t");
        syllable.set_vowel("o");
        syllable.set_final("n");

        assert_eq!(syllable.initial().as_str(), "t");
        assert_eq!(syllable.vowel().as_str(), "o");
        assert_eq!(syllable.final_consonant().as_str(), "n");
    }

    #[test]
    fn test_syllable_clear() {
        let mut syllable = Syllable::from_parts("h", "a", "n", ToneType::Sac);
        
        syllable.clear();
        
        assert!(syllable.is_empty());
        assert_eq!(syllable.tone(), ToneType::Ngang);
    }

    #[test]
    fn test_syllable_base_form() {
        let syllable = Syllable::from_parts("tr", "ươ", "ng", ToneType::Huyen);
        assert_eq!(syllable.base_form(), "trương");
    }

    #[test]
    fn test_syllable_as_tuple() {
        let syllable = Syllable::from_parts("h", "a", "n", ToneType::Sac);
        let (i, v, f, t) = syllable.as_tuple();
        
        assert_eq!(i, "h");
        assert_eq!(v, "a");
        assert_eq!(f, "n");
        assert_eq!(t, ToneType::Sac);
    }

    #[test]
    fn test_syllable_with_tone_clone() {
        let syllable = Syllable::from_parts("h", "a", "", ToneType::Ngang);
        let cloned = syllable.with_tone_clone(ToneType::Huyen);

        assert_eq!(syllable.tone(), ToneType::Ngang);
        assert_eq!(cloned.tone(), ToneType::Huyen);
        assert_eq!(cloned.vowel().as_str(), "a");
    }

    #[test]
    fn test_syllable_len() {
        let syllable = Syllable::from_parts("tr", "ươ", "ng", ToneType::Ngang);
        assert_eq!(syllable.len(), 6); // "tr" + "ươ" + "ng"
    }

    #[test]
    fn test_syllable_matches_pattern() {
        let syllable = Syllable::from_parts("h", "a", "n", ToneType::Ngang);
        
        assert!(syllable.matches_pattern("h", "a", "n"));
        assert!(syllable.matches_pattern("h", "", "")); // Only initial matches
        assert!(!syllable.matches_pattern("t", "a", "n"));
    }

    #[test]
    fn test_syllable_builder() {
        let syllable = SyllableBuilder::new()
            .initial("c")
            .vowel("a")
            .final_consonant("o")
            .tone(ToneType::Sac)
            .build();

        assert_eq!(syllable.base_form(), "cao");
        assert_eq!(syllable.tone(), ToneType::Sac);
    }

    #[test]
    fn test_syllable_builder_validated() {
        let valid = SyllableBuilder::new()
            .vowel("a")
            .build_validated();
        assert!(valid.is_some());

        let invalid = SyllableBuilder::new()
            .initial("h")
            .build_validated(); // No vowel
        assert!(invalid.is_none());
    }

    #[test]
    fn test_syllable_display() {
        let syllable = Syllable::from_parts("h", "a", "", ToneType::Ngang);
        assert_eq!(format!("{}", syllable), "ha");
    }

    #[test]
    fn test_syllable_vietnamese_examples() {
        // Syllable: "trường" (tr + ươ + ng + huyen)
        let truong = Syllable::from_parts("tr", "ươ", "ng", ToneType::Huyen);
        assert!(truong.is_valid());
        assert_eq!(truong.base_form(), "trương");

        // Syllable: "tiếng" (t + iê + ng + sắc)
        let tieng = Syllable::from_parts("t", "iê", "ng", ToneType::Sac);
        assert!(tieng.is_valid());
        assert_eq!(tieng.base_form(), "tiêng");

        // Syllable: "việt" (v + iê + t + nặng)
        let viet = Syllable::from_parts("v", "iê", "t", ToneType::Nang);
        assert!(viet.is_valid());
        assert_eq!(viet.base_form(), "viêt");
    }
}
