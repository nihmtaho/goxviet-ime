//! Validation Ports (Interfaces)
//!
//! Defines abstractions for input validation following SOLID principles.

pub mod language_detector;
pub mod syllable_validator;

// Re-export main types
pub use language_detector::{
    ConfidenceLevel, DetectedLanguage, DetectionResult, LanguageDetector,
};
pub use syllable_validator::SyllableValidator;
