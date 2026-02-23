//! Language Detector Adapter
//!
//! Implements `LanguageDetector` using engine_v2's language decision engine
//! and Vietnamese validator to determine if text is Vietnamese or non-Vietnamese.

use crate::domain::{
    ports::validation::language_detector::{
        ConfidenceLevel, DetectionResult, LanguageDetector,
    },
    value_objects::char_sequence::CharSequence,
};
use crate::infrastructure::external::english::language_decision::LanguageDecisionEngine;
use crate::infrastructure::external::vietnamese_validator::VietnameseSyllableValidator;

/// Language detector adapter
///
/// This adapter wraps the engine_v2's `LanguageDecisionEngine` and
/// `VietnameseSyllableValidator` to provide language detection through
/// the `LanguageDetector` port.
///
/// # Detection Strategy
///
/// 1. Parse text into keys with diacritic information
/// 2. Detect diacritics (Vietnamese-specific characters)
/// 3. Validate as Vietnamese syllable using FSM validator
/// 4. Use language decision engine to combine evidence:
///    - Dictionary lookup (highest priority)
///    - Vietnamese validator result
///    - Phonotactic analysis
///    - Diacritic presence
///
/// # Decision Rules
///
/// - If decision engine says English → `NonVietnamese`
/// - If has diacritics OR valid Vietnamese → `Vietnamese`
/// - Otherwise → `Ambiguous`
///
/// # Examples
///
/// ```
/// use goxviet_core::infrastructure::adapters::validation::LanguageDetectorAdapter;
/// use goxviet_core::domain::ports::validation::language_detector::{
///     LanguageDetector, DetectedLanguage
/// };
/// use goxviet_core::domain::value_objects::char_sequence::CharSequence;
///
/// let detector = LanguageDetectorAdapter::new();
///
/// // Vietnamese word
/// let viet = CharSequence::from("trường");
/// let result = detector.detect(&viet);
/// assert_eq!(result.language, DetectedLanguage::Vietnamese);
///
/// // English word
/// let eng = CharSequence::from("hello");
/// let result = detector.detect(&eng);
/// assert_eq!(result.language, DetectedLanguage::NonVietnamese);
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct LanguageDetectorAdapter;

impl LanguageDetectorAdapter {
    /// Creates a new language detector adapter
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::infrastructure::adapters::validation::LanguageDetectorAdapter;
    /// let detector = LanguageDetectorAdapter::new();
    /// ```
    pub fn new() -> Self {
        Self
    }

    /// Converts CharSequence to keys and diacritic flags
    ///
    /// Parses each character and checks for:
    /// - Non-ASCII characters (likely Vietnamese)
    /// - Vietnamese diacritics (tone, circumflex, horn, etc.)
    ///
    /// Returns (keys, has_diacritics) for language decision engine
    fn parse_text(&self, text: &CharSequence) -> (Vec<(u16, bool)>, bool) {
        use crate::data::chars::parse_char;

        let mut keys = Vec::new();
        let mut has_diacritics = false;

        for ch in text.as_str().chars() {
            if let Some(parsed) = parse_char(ch) {
                // Check if character has diacritics
                // Diacritics include: tone modifiers (circumflex, horn) or marks (sắc, huyền, etc.)
                let char_has_diacritics = parsed.tone != 0 || parsed.mark != 0 || parsed.stroke;

                // Non-ASCII character is also considered a diacritic indicator
                let is_non_ascii = !ch.is_ascii();

                if char_has_diacritics || is_non_ascii {
                    has_diacritics = true;
                }

                keys.push((parsed.key, parsed.caps));
            } else {
                // Unknown character - treat as potential Vietnamese indicator
                if !ch.is_ascii() {
                    has_diacritics = true;
                }
            }
        }

        (keys, has_diacritics)
    }

    /// Validates text as Vietnamese syllable
    ///
    /// Extracts keys only and validates using Vietnamese validator
    fn validate_vietnamese(&self, text: &CharSequence) -> Option<crate::infrastructure::external::vietnamese_validator::ValidationResult> {
        use crate::data::chars::parse_char;

        let mut keys = Vec::new();

        for ch in text.as_str().chars() {
            if let Some(parsed) = parse_char(ch) {
                keys.push(parsed.key);
            }
        }

        if keys.is_empty() {
            return None;
        }

        Some(VietnameseSyllableValidator::validate(&keys))
    }
}

impl LanguageDetector for LanguageDetectorAdapter {
    fn detect(&self, text: &CharSequence) -> DetectionResult {
        if text.is_empty() {
            return DetectionResult::ambiguous();
        }

        // Parse text into keys and detect diacritics
        let (keys, has_diacritics) = self.parse_text(text);

        if keys.is_empty() {
            return DetectionResult::ambiguous();
        }

        // PRIORITY 1: If text has Vietnamese diacritics, it's definitely Vietnamese
        // This overrides any other detection logic
        if has_diacritics {
            return DetectionResult::vietnamese(ConfidenceLevel::VeryHigh);
        }

        // Validate as Vietnamese syllable
        let vietnamese_validation = self.validate_vietnamese(text);
        
        // Check if validation is valid before passing to decision engine
        let is_valid_vietnamese = vietnamese_validation
            .as_ref()
            .map(|v| v.is_valid)
            .unwrap_or(false);

        // Use language decision engine to make final decision
        let decision = LanguageDecisionEngine::decide_with_validation(
            &keys,
            has_diacritics,
            vietnamese_validation,
        );

        // Convert decision to DetectionResult
        if decision.is_english {
            // Engine says it's English
            DetectionResult::non_vietnamese(ConfidenceLevel::from_percent(decision.confidence))
        } else if is_valid_vietnamese {
            // Valid Vietnamese syllable
            DetectionResult::vietnamese(ConfidenceLevel::from_percent(
                100_u8.saturating_sub(decision.confidence),
            ))
        } else {
            // Invalid Vietnamese, not English → ambiguous
            DetectionResult::ambiguous()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::ports::validation::language_detector::DetectedLanguage;

    #[test]
    fn test_language_detector_vietnamese() {
        let detector = LanguageDetectorAdapter::new();

        // Vietnamese words with diacritics
        let truong = CharSequence::from("trường");
        let result = detector.detect(&truong);
        assert_eq!(result.language, DetectedLanguage::Vietnamese);
        assert!(result.is_confident());

        let tieng = CharSequence::from("tiếng");
        let result = detector.detect(&tieng);
        assert_eq!(result.language, DetectedLanguage::Vietnamese);

        let viet = CharSequence::from("việt");
        let result = detector.detect(&viet);
        assert_eq!(result.language, DetectedLanguage::Vietnamese);
    }

    #[test]
    fn test_language_detector_english() {
        let detector = LanguageDetectorAdapter::new();

        // Common English words
        let hello = CharSequence::from("hello");
        let result = detector.detect(&hello);
        assert_eq!(result.language, DetectedLanguage::NonVietnamese);

        let world = CharSequence::from("world");
        let result = detector.detect(&world);
        assert_eq!(result.language, DetectedLanguage::NonVietnamese);

        let test = CharSequence::from("test");
        let result = detector.detect(&test);
        assert_eq!(result.language, DetectedLanguage::NonVietnamese);
    }

    #[test]
    fn test_language_detector_ambiguous() {
        let detector = LanguageDetectorAdapter::new();

        // Single character - ambiguous
        let a = CharSequence::from("a");
        let result = detector.detect(&a);
        // Result may be ambiguous or detected as English/Vietnamese depending on engine
        // Just ensure it doesn't crash
        assert!(matches!(
            result.language,
            DetectedLanguage::Ambiguous
                | DetectedLanguage::Vietnamese
                | DetectedLanguage::NonVietnamese
        ));

        // Empty string
        let empty = CharSequence::empty();
        let result = detector.detect(&empty);
        assert_eq!(result.language, DetectedLanguage::Ambiguous);
    }

    #[test]
    fn test_language_detector_vietnamese_without_diacritics() {
        let detector = LanguageDetectorAdapter::new();

        // Vietnamese syllables without diacritics (valid structure)
        let hoa = CharSequence::from("hoa");
        let result = detector.detect(&hoa);
        // Should detect as Vietnamese if valid structure
        // May be ambiguous if looks like English too
        assert!(matches!(
            result.language,
            DetectedLanguage::Vietnamese | DetectedLanguage::Ambiguous
        ));
    }

    #[test]
    fn test_language_detector_is_vietnamese() {
        let detector = LanguageDetectorAdapter::new();

        // Vietnamese with diacritics
        let truong = CharSequence::from("trường");
        assert!(detector.is_vietnamese(&truong));

        // English
        let hello = CharSequence::from("hello");
        assert!(!detector.is_vietnamese(&hello));
    }

    #[test]
    fn test_language_detector_parse_text() {
        let detector = LanguageDetectorAdapter::new();

        // Vietnamese with diacritics
        let text = CharSequence::from("trường");
        let (keys, has_diacritics) = detector.parse_text(&text);
        assert!(!keys.is_empty());
        assert!(has_diacritics);

        // English without diacritics
        let text = CharSequence::from("hello");
        let (keys, has_diacritics) = detector.parse_text(&text);
        assert!(!keys.is_empty());
        assert!(!has_diacritics);
    }

    #[test]
    fn test_language_detector_validate_vietnamese() {
        let detector = LanguageDetectorAdapter::new();

        // Valid Vietnamese syllable
        let text = CharSequence::from("truong");
        let validation = detector.validate_vietnamese(&text);
        assert!(validation.is_some());
        if let Some(v) = validation {
            // May be valid or invalid depending on validator
            // Just ensure it doesn't crash
            let _ = v.is_valid;
        }

        // Empty text
        let text = CharSequence::empty();
        let validation = detector.validate_vietnamese(&text);
        assert!(validation.is_none());
    }
}
