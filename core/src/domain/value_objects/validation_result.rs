//! Validation Result Value Object
//!
//! Represents the outcome of validation operations.

use std::fmt;

/// Result of syllable or text validation
///
/// Indicates whether input is valid Vietnamese, with details about
/// why it might be invalid.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValidationResult {
    /// Input is valid Vietnamese
    Valid,
    
    /// Input is invalid Vietnamese with reason
    Invalid {
        reason: ValidationError,
        position: Option<usize>,
    },
    
    /// Input is ambiguous (could be valid or invalid depending on context)
    Ambiguous {
        message: String,
    },
    
    /// Input appears to be English (not Vietnamese)
    NonVietnamese,
}

impl ValidationResult {
    /// Create a valid result
    pub fn valid() -> Self {
        ValidationResult::Valid
    }

    /// Create an invalid result with error
    pub fn invalid(reason: ValidationError) -> Self {
        ValidationResult::Invalid {
            reason,
            position: None,
        }
    }

    /// Create an invalid result with error and position
    pub fn invalid_at(reason: ValidationError, position: usize) -> Self {
        ValidationResult::Invalid {
            reason,
            position: Some(position),
        }
    }

    /// Create an ambiguous result
    pub fn ambiguous(message: impl Into<String>) -> Self {
        ValidationResult::Ambiguous {
            message: message.into(),
        }
    }

    /// Create non-Vietnamese result
    pub fn non_vietnamese() -> Self {
        ValidationResult::NonVietnamese
    }

    /// Check if validation passed
    #[inline]
    pub fn is_valid(&self) -> bool {
        matches!(self, ValidationResult::Valid)
    }

    /// Check if validation failed
    #[inline]
    pub fn is_invalid(&self) -> bool {
        matches!(self, ValidationResult::Invalid { .. })
    }

    /// Check if result is ambiguous
    #[inline]
    pub fn is_ambiguous(&self) -> bool {
        matches!(self, ValidationResult::Ambiguous { .. })
    }

    /// Check if input is non-Vietnamese
    #[inline]
    pub fn is_non_vietnamese(&self) -> bool {
        matches!(self, ValidationResult::NonVietnamese)
    }

    /// Get error reason if invalid
    pub fn error(&self) -> Option<&ValidationError> {
        match self {
            ValidationResult::Invalid { reason, .. } => Some(reason),
            _ => None,
        }
    }

    /// Get error position if available
    pub fn position(&self) -> Option<usize> {
        match self {
            ValidationResult::Invalid { position, .. } => *position,
            _ => None,
        }
    }

    /// Convert to Result type for error handling
    pub fn into_result(self) -> Result<(), ValidationError> {
        match self {
            ValidationResult::Valid => Ok(()),
            ValidationResult::Invalid { reason, .. } => Err(reason),
            ValidationResult::Ambiguous { message } => {
                Err(ValidationError::Ambiguous { message })
            }
            ValidationResult::NonVietnamese => Err(ValidationError::NonVietnamese),
        }
    }
}

impl Default for ValidationResult {
    fn default() -> Self {
        ValidationResult::Valid
    }
}

/// Validation error types
///
/// Describes why validation failed.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValidationError {
    /// Invalid consonant combination
    InvalidConsonant { 
        consonant: String,
        context: String,
    },
    
    /// Invalid vowel combination
    InvalidVowel {
        vowel: String,
        context: String,
    },
    
    /// Invalid tone placement
    InvalidTonePlacement {
        syllable: String,
        reason: String,
    },
    
    /// Syllable structure violation
    InvalidStructure {
        syllable: String,
        reason: String,
    },
    
    /// Phonotactic constraint violation
    PhonotacticViolation {
        rule: String,
        context: String,
    },
    
    /// Ambiguous case (could be valid or invalid)
    Ambiguous {
        message: String,
    },
    
    /// Not Vietnamese language
    NonVietnamese,
    
    /// Empty input
    Empty,
    
    /// Other error with custom message
    Other {
        message: String,
    },
}

impl ValidationError {
    /// Get human-readable error message
    pub fn message(&self) -> String {
        match self {
            ValidationError::InvalidConsonant { consonant, context } => {
                format!("Invalid consonant '{}' in context '{}'", consonant, context)
            }
            ValidationError::InvalidVowel { vowel, context } => {
                format!("Invalid vowel '{}' in context '{}'", vowel, context)
            }
            ValidationError::InvalidTonePlacement { syllable, reason } => {
                format!("Invalid tone placement in '{}': {}", syllable, reason)
            }
            ValidationError::InvalidStructure { syllable, reason } => {
                format!("Invalid syllable structure '{}': {}", syllable, reason)
            }
            ValidationError::PhonotacticViolation { rule, context } => {
                format!("Phonotactic rule '{}' violated in '{}'", rule, context)
            }
            ValidationError::Ambiguous { message } => {
                format!("Ambiguous: {}", message)
            }
            ValidationError::NonVietnamese => {
                "Not Vietnamese language".to_string()
            }
            ValidationError::Empty => {
                "Empty input".to_string()
            }
            ValidationError::Other { message } => message.clone(),
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl std::error::Error for ValidationError {}

impl fmt::Display for ValidationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationResult::Valid => write!(f, "Valid"),
            ValidationResult::Invalid { reason, position } => {
                if let Some(pos) = position {
                    write!(f, "Invalid at position {}: {}", pos, reason)
                } else {
                    write!(f, "Invalid: {}", reason)
                }
            }
            ValidationResult::Ambiguous { message } => {
                write!(f, "Ambiguous: {}", message)
            }
            ValidationResult::NonVietnamese => write!(f, "Non-Vietnamese"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_result_valid() {
        let result = ValidationResult::valid();
        assert!(result.is_valid());
        assert!(!result.is_invalid());
        assert!(!result.is_ambiguous());
    }

    #[test]
    fn test_validation_result_invalid() {
        let result = ValidationResult::invalid(ValidationError::Empty);
        assert!(!result.is_valid());
        assert!(result.is_invalid());
        assert!(result.error().is_some());
    }

    #[test]
    fn test_validation_result_invalid_at() {
        let result = ValidationResult::invalid_at(ValidationError::Empty, 5);
        assert!(result.is_invalid());
        assert_eq!(result.position(), Some(5));
    }

    #[test]
    fn test_validation_result_ambiguous() {
        let result = ValidationResult::ambiguous("Could be valid");
        assert!(result.is_ambiguous());
        assert!(!result.is_valid());
        assert!(!result.is_invalid());
    }

    #[test]
    fn test_validation_result_non_vietnamese() {
        let result = ValidationResult::non_vietnamese();
        assert!(result.is_non_vietnamese());
        assert!(!result.is_valid());
    }

    #[test]
    fn test_validation_error_invalid_consonant() {
        let error = ValidationError::InvalidConsonant {
            consonant: "xyz".to_string(),
            context: "start".to_string(),
        };
        let msg = error.message();
        assert!(msg.contains("xyz"));
        assert!(msg.contains("start"));
    }

    #[test]
    fn test_validation_error_invalid_vowel() {
        let error = ValidationError::InvalidVowel {
            vowel: "ae".to_string(),
            context: "middle".to_string(),
        };
        let msg = error.message();
        assert!(msg.contains("ae"));
        assert!(msg.contains("middle"));
    }

    #[test]
    fn test_validation_error_empty() {
        let error = ValidationError::Empty;
        assert_eq!(error.message(), "Empty input");
    }

    #[test]
    fn test_validation_error_non_vietnamese() {
        let error = ValidationError::NonVietnamese;
        assert_eq!(error.message(), "Not Vietnamese language");
    }

    #[test]
    fn test_validation_result_into_result() {
        let valid = ValidationResult::valid();
        assert!(valid.into_result().is_ok());

        let invalid = ValidationResult::invalid(ValidationError::Empty);
        assert!(invalid.into_result().is_err());

        let non_viet = ValidationResult::non_vietnamese();
        assert!(non_viet.into_result().is_err());
    }

    #[test]
    fn test_validation_result_display() {
        let valid = ValidationResult::valid();
        assert_eq!(format!("{}", valid), "Valid");

        let invalid = ValidationResult::invalid(ValidationError::Empty);
        assert!(format!("{}", invalid).contains("Invalid"));

        let ambiguous = ValidationResult::ambiguous("test");
        assert!(format!("{}", ambiguous).contains("Ambiguous"));
    }

    #[test]
    fn test_validation_error_display() {
        let error = ValidationError::Empty;
        assert_eq!(format!("{}", error), "Empty input");
    }
}
