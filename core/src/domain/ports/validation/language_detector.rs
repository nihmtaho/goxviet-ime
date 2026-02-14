//! Language Detector Port
//!
//! Defines the abstraction for detecting language (Vietnamese vs non-Vietnamese text).
//!
//! # Design Principles
//!
//! - **ISP**: Small interface with 2 essential methods
//! - **DIP**: Domain defines contract, infrastructure implements
//! - **SRP**: Only detects language, no transformation
//!
//! # Use Cases
//!
//! - **Smart Mode**: Auto-disable Vietnamese input for English words
//! - **Mixed Content**: Handle documents with both Vietnamese and English
//! - **User Intent**: Detect when user is typing English vs Vietnamese
//!
//! # Architecture
//!
//! ```text
//! Domain Layer (this file)
//!     ↓ defines interface
//! Infrastructure Layer
//!     ↓ implements
//! DictionaryDetector, PhonotacticDetector, HeuristicDetector
//! ```

use crate::domain::value_objects::char_sequence::CharSequence;

/// Language type detected in input
///
/// Represents the detected language of text being typed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DetectedLanguage {
    /// Vietnamese text
    Vietnamese,
    /// Non-Vietnamese text (English, etc.)
    NonVietnamese,
    /// Ambiguous - needs more context
    Ambiguous,
}

/// Confidence level for language detection
///
/// Represents how confident the detector is in its classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ConfidenceLevel {
    /// Very low confidence (< 25%)
    VeryLow,
    /// Low confidence (25-50%)
    Low,
    /// Medium confidence (50-75%)
    Medium,
    /// High confidence (75-90%)
    High,
    /// Very high confidence (> 90%)
    VeryHigh,
}

impl ConfidenceLevel {
    /// Creates from percentage (0-100)
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::domain::ports::validation::language_detector::ConfidenceLevel;
    /// assert_eq!(ConfidenceLevel::from_percent(10), ConfidenceLevel::VeryLow);
    /// assert_eq!(ConfidenceLevel::from_percent(40), ConfidenceLevel::Low);
    /// assert_eq!(ConfidenceLevel::from_percent(60), ConfidenceLevel::Medium);
    /// assert_eq!(ConfidenceLevel::from_percent(80), ConfidenceLevel::High);
    /// assert_eq!(ConfidenceLevel::from_percent(95), ConfidenceLevel::VeryHigh);
    /// ```
    pub fn from_percent(percent: u8) -> Self {
        match percent {
            0..=24 => Self::VeryLow,
            25..=49 => Self::Low,
            50..=74 => Self::Medium,
            75..=89 => Self::High,
            _ => Self::VeryHigh,
        }
    }

    /// Gets minimum percentage for this level
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::domain::ports::validation::language_detector::ConfidenceLevel;
    /// assert_eq!(ConfidenceLevel::VeryLow.min_percent(), 0);
    /// assert_eq!(ConfidenceLevel::Low.min_percent(), 25);
    /// assert_eq!(ConfidenceLevel::Medium.min_percent(), 50);
    /// assert_eq!(ConfidenceLevel::High.min_percent(), 75);
    /// assert_eq!(ConfidenceLevel::VeryHigh.min_percent(), 90);
    /// ```
    pub fn min_percent(&self) -> u8 {
        match self {
            Self::VeryLow => 0,
            Self::Low => 25,
            Self::Medium => 50,
            Self::High => 75,
            Self::VeryHigh => 90,
        }
    }
}

/// Detection result with confidence
///
/// Combines detected language with confidence level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DetectionResult {
    /// The detected language
    pub language: DetectedLanguage,
    /// Confidence level
    pub confidence: ConfidenceLevel,
}

impl DetectionResult {
    /// Creates a new detection result
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::domain::ports::validation::language_detector::{
    /// #     DetectionResult, DetectedLanguage, ConfidenceLevel,
    /// # };
    /// let result = DetectionResult::new(
    ///     DetectedLanguage::Vietnamese,
    ///     ConfidenceLevel::High
    /// );
    /// assert_eq!(result.language, DetectedLanguage::Vietnamese);
    /// assert_eq!(result.confidence, ConfidenceLevel::High);
    /// ```
    pub fn new(language: DetectedLanguage, confidence: ConfidenceLevel) -> Self {
        Self {
            language,
            confidence,
        }
    }

    /// Creates Vietnamese detection with confidence
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::domain::ports::validation::language_detector::{
    /// #     DetectionResult, DetectedLanguage, ConfidenceLevel,
    /// # };
    /// let result = DetectionResult::vietnamese(ConfidenceLevel::High);
    /// assert_eq!(result.language, DetectedLanguage::Vietnamese);
    /// ```
    pub fn vietnamese(confidence: ConfidenceLevel) -> Self {
        Self::new(DetectedLanguage::Vietnamese, confidence)
    }

    /// Creates non-Vietnamese detection with confidence
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::domain::ports::validation::language_detector::{
    /// #     DetectionResult, DetectedLanguage, ConfidenceLevel,
    /// # };
    /// let result = DetectionResult::non_vietnamese(ConfidenceLevel::High);
    /// assert_eq!(result.language, DetectedLanguage::NonVietnamese);
    /// ```
    pub fn non_vietnamese(confidence: ConfidenceLevel) -> Self {
        Self::new(DetectedLanguage::NonVietnamese, confidence)
    }

    /// Creates ambiguous detection
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::domain::ports::validation::language_detector::{
    /// #     DetectionResult, DetectedLanguage,
    /// # };
    /// let result = DetectionResult::ambiguous();
    /// assert_eq!(result.language, DetectedLanguage::Ambiguous);
    /// ```
    pub fn ambiguous() -> Self {
        Self::new(DetectedLanguage::Ambiguous, ConfidenceLevel::VeryLow)
    }

    /// Checks if detection is confident (High or VeryHigh)
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::domain::ports::validation::language_detector::{
    /// #     DetectionResult, ConfidenceLevel,
    /// # };
    /// assert!(DetectionResult::vietnamese(ConfidenceLevel::High).is_confident());
    /// assert!(DetectionResult::vietnamese(ConfidenceLevel::VeryHigh).is_confident());
    /// assert!(!DetectionResult::vietnamese(ConfidenceLevel::Medium).is_confident());
    /// ```
    pub fn is_confident(&self) -> bool {
        matches!(
            self.confidence,
            ConfidenceLevel::High | ConfidenceLevel::VeryHigh
        )
    }
}

/// Language detector port (interface)
///
/// Detects whether input text is Vietnamese or non-Vietnamese.
///
/// # Detection Strategies
///
/// Implementations may use:
///
/// - **Dictionary Lookup**: Check if word exists in Vietnamese/English dictionaries
/// - **Phonotactic Analysis**: Analyze sound patterns (Vietnamese has specific rules)
/// - **Heuristic Rules**: Use simple rules (e.g., presence of Vietnamese diacritics)
/// - **Machine Learning**: Trained models for language classification
///
/// # Examples
///
/// ```ignore
/// let detector: Box<dyn LanguageDetector> = Box::new(DictionaryDetector::new());
///
/// let viet = CharSequence::from("trường");
/// assert_eq!(detector.detect(&viet).language, DetectedLanguage::Vietnamese);
///
/// let eng = CharSequence::from("hello");
/// assert_eq!(detector.detect(&eng).language, DetectedLanguage::NonVietnamese);
/// ```
pub trait LanguageDetector: Send + Sync {
    /// Detects language of input text
    ///
    /// # Arguments
    ///
    /// - `text`: The text to analyze
    ///
    /// # Returns
    ///
    /// `DetectionResult` with language and confidence level
    ///
    /// # Examples (conceptual)
    ///
    /// ```ignore
    /// // Vietnamese words
    /// detector.detect("trường") => Vietnamese (High confidence)
    /// detector.detect("tiếng")  => Vietnamese (High confidence)
    ///
    /// // English words
    /// detector.detect("hello")  => NonVietnamese (High confidence)
    /// detector.detect("world")  => NonVietnamese (High confidence)
    ///
    /// // Ambiguous
    /// detector.detect("a")      => Ambiguous (VeryLow confidence)
    /// detector.detect("an")     => Ambiguous (Low confidence)
    /// ```
    fn detect(&self, text: &CharSequence) -> DetectionResult;

    /// Checks if text is definitely Vietnamese
    ///
    /// # Arguments
    ///
    /// - `text`: The text to check
    ///
    /// # Returns
    ///
    /// - `true` if confident it's Vietnamese
    /// - `false` otherwise (non-Vietnamese or ambiguous)
    ///
    /// # Default Implementation
    ///
    /// Returns `true` if detection is Vietnamese with High/VeryHigh confidence.
    fn is_vietnamese(&self, text: &CharSequence) -> bool {
        let result = self.detect(text);
        result.language == DetectedLanguage::Vietnamese && result.is_confident()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detected_language_variants() {
        let vietnamese = DetectedLanguage::Vietnamese;
        let non_viet = DetectedLanguage::NonVietnamese;
        let ambiguous = DetectedLanguage::Ambiguous;

        assert_ne!(vietnamese, non_viet);
        assert_ne!(vietnamese, ambiguous);
        assert_ne!(non_viet, ambiguous);
    }

    #[test]
    fn test_confidence_level_from_percent() {
        assert_eq!(ConfidenceLevel::from_percent(0), ConfidenceLevel::VeryLow);
        assert_eq!(ConfidenceLevel::from_percent(24), ConfidenceLevel::VeryLow);
        assert_eq!(ConfidenceLevel::from_percent(25), ConfidenceLevel::Low);
        assert_eq!(ConfidenceLevel::from_percent(49), ConfidenceLevel::Low);
        assert_eq!(
            ConfidenceLevel::from_percent(50),
            ConfidenceLevel::Medium
        );
        assert_eq!(
            ConfidenceLevel::from_percent(74),
            ConfidenceLevel::Medium
        );
        assert_eq!(ConfidenceLevel::from_percent(75), ConfidenceLevel::High);
        assert_eq!(ConfidenceLevel::from_percent(89), ConfidenceLevel::High);
        assert_eq!(
            ConfidenceLevel::from_percent(90),
            ConfidenceLevel::VeryHigh
        );
        assert_eq!(
            ConfidenceLevel::from_percent(100),
            ConfidenceLevel::VeryHigh
        );
    }

    #[test]
    fn test_confidence_level_min_percent() {
        assert_eq!(ConfidenceLevel::VeryLow.min_percent(), 0);
        assert_eq!(ConfidenceLevel::Low.min_percent(), 25);
        assert_eq!(ConfidenceLevel::Medium.min_percent(), 50);
        assert_eq!(ConfidenceLevel::High.min_percent(), 75);
        assert_eq!(ConfidenceLevel::VeryHigh.min_percent(), 90);
    }

    #[test]
    fn test_confidence_level_ordering() {
        assert!(ConfidenceLevel::VeryLow < ConfidenceLevel::Low);
        assert!(ConfidenceLevel::Low < ConfidenceLevel::Medium);
        assert!(ConfidenceLevel::Medium < ConfidenceLevel::High);
        assert!(ConfidenceLevel::High < ConfidenceLevel::VeryHigh);
    }

    #[test]
    fn test_detection_result_new() {
        let result = DetectionResult::new(DetectedLanguage::Vietnamese, ConfidenceLevel::High);
        assert_eq!(result.language, DetectedLanguage::Vietnamese);
        assert_eq!(result.confidence, ConfidenceLevel::High);
    }

    #[test]
    fn test_detection_result_vietnamese() {
        let result = DetectionResult::vietnamese(ConfidenceLevel::High);
        assert_eq!(result.language, DetectedLanguage::Vietnamese);
        assert_eq!(result.confidence, ConfidenceLevel::High);
    }

    #[test]
    fn test_detection_result_non_vietnamese() {
        let result = DetectionResult::non_vietnamese(ConfidenceLevel::High);
        assert_eq!(result.language, DetectedLanguage::NonVietnamese);
        assert_eq!(result.confidence, ConfidenceLevel::High);
    }

    #[test]
    fn test_detection_result_ambiguous() {
        let result = DetectionResult::ambiguous();
        assert_eq!(result.language, DetectedLanguage::Ambiguous);
        assert_eq!(result.confidence, ConfidenceLevel::VeryLow);
    }

    #[test]
    fn test_detection_result_is_confident() {
        assert!(DetectionResult::vietnamese(ConfidenceLevel::VeryHigh).is_confident());
        assert!(DetectionResult::vietnamese(ConfidenceLevel::High).is_confident());
        assert!(!DetectionResult::vietnamese(ConfidenceLevel::Medium).is_confident());
        assert!(!DetectionResult::vietnamese(ConfidenceLevel::Low).is_confident());
        assert!(!DetectionResult::vietnamese(ConfidenceLevel::VeryLow).is_confident());
    }

    #[test]
    fn test_detection_result_clone_copy() {
        let result = DetectionResult::vietnamese(ConfidenceLevel::High);
        let cloned = result.clone();
        let copied = result;

        assert_eq!(result, cloned);
        assert_eq!(result, copied);
    }
}
