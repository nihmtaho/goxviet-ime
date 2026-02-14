//! Validate Input Use Case
//!
//! Use case for validating Vietnamese text input.
//!
//! # Responsibilities
//!
//! - Validate syllable structure
//! - Check phonotactic rules
//! - Detect invalid combinations
//!
//! # Design
//!
//! This is a Query pattern implementation - it returns validation results
//! without modifying state.

use crate::domain::{
    entities::syllable::Syllable,
    ports::validation::SyllableValidator,
    value_objects::{char_sequence::CharSequence, validation_result::ValidationResult},
};

/// Validation request
#[derive(Debug, Clone)]
pub struct ValidationRequest {
    /// Text to validate
    pub text: CharSequence,
}

impl ValidationRequest {
    /// Creates a new validation request
    pub fn new(text: impl Into<CharSequence>) -> Self {
        Self {
            text: text.into(),
        }
    }
}

/// Validate input use case
///
/// Validates Vietnamese text against phonotactic rules.
///
/// # Examples
///
/// ```ignore
/// let use_case = ValidateInputUseCase::new(validator);
/// let request = ValidationRequest::new("trường");
/// let result = use_case.execute(&request);
/// assert!(result.is_valid());
/// ```
pub struct ValidateInputUseCase {
    validator: Box<dyn SyllableValidator>,
}

impl ValidateInputUseCase {
    /// Creates a new use case with injected validator
    pub fn new(validator: Box<dyn SyllableValidator>) -> Self {
        Self { validator }
    }

    /// Executes the validation
    ///
    /// # Arguments
    ///
    /// * `request` - Validation request containing text to validate
    ///
    /// # Returns
    ///
    /// `ValidationResult` with validation outcome
    pub fn execute(&self, request: &ValidationRequest) -> ValidationResult {
        // Empty text is valid
        if request.text.is_empty() {
            return ValidationResult::valid();
        }

        // Convert text to syllable for validation
        // Use simple approach: just vowel nucleus for now
        let syllable = Syllable::new().with_vowel(request.text.clone());
        self.validator.validate(&syllable)
    }

    /// Quick validation without detailed errors
    ///
    /// Faster than `execute()` when you only need a boolean result.
    pub fn is_valid(&self, text: &CharSequence) -> bool {
        if text.is_empty() {
            return true;
        }

        let syllable = Syllable::new().with_vowel(text.clone());
        self.validator.validate(&syllable).is_valid()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::validation_result::ValidationError;

    struct MockValidator {
        always_valid: bool,
    }

    impl MockValidator {
        fn new(always_valid: bool) -> Self {
            Self { always_valid }
        }
    }

    impl SyllableValidator for MockValidator {
        fn validate(&self, _syllable: &Syllable) -> ValidationResult {
            if self.always_valid {
                ValidationResult::valid()
            } else {
                ValidationResult::invalid(
                    ValidationError::InvalidStructure { 
                        syllable: "test".to_string(), 
                        reason: "Mock validation error".to_string() 
                    }
                )
            }
        }
    }

    #[test]
    fn test_use_case_creation() {
        let use_case = ValidateInputUseCase::new(Box::new(MockValidator::new(true)));
        let request = ValidationRequest::new("test");
        let result = use_case.execute(&request);
        assert!(result.is_valid());
    }

    #[test]
    fn test_validate_empty_text() {
        let use_case = ValidateInputUseCase::new(Box::new(MockValidator::new(true)));
        let request = ValidationRequest::new("");
        let result = use_case.execute(&request);
        assert!(result.is_valid());
    }

    #[test]
    fn test_validate_valid_text() {
        let use_case = ValidateInputUseCase::new(Box::new(MockValidator::new(true)));
        let request = ValidationRequest::new("hoa");
        let result = use_case.execute(&request);
        assert!(result.is_valid());
    }

    #[test]
    fn test_validate_invalid_text() {
        let use_case = ValidateInputUseCase::new(Box::new(MockValidator::new(false)));
        let request = ValidationRequest::new("xyz");
        let result = use_case.execute(&request);
        assert!(!result.is_valid());
    }

    #[test]
    fn test_is_valid_empty() {
        let use_case = ValidateInputUseCase::new(Box::new(MockValidator::new(true)));
        assert!(use_case.is_valid(&CharSequence::empty()));
    }

    #[test]
    fn test_is_valid_valid_text() {
        let use_case = ValidateInputUseCase::new(Box::new(MockValidator::new(true)));
        assert!(use_case.is_valid(&CharSequence::from("hoa")));
    }

    #[test]
    fn test_is_valid_invalid_text() {
        let use_case = ValidateInputUseCase::new(Box::new(MockValidator::new(false)));
        assert!(!use_case.is_valid(&CharSequence::from("xyz")));
    }

    #[test]
    fn test_request_creation() {
        let request = ValidationRequest::new("test");
        assert_eq!(request.text.as_str(), "test");
    }

    #[test]
    fn test_request_from_string() {
        let request = ValidationRequest::new(String::from("test"));
        assert_eq!(request.text.as_str(), "test");
    }

    #[test]
    fn test_vietnamese_text() {
        let use_case = ValidateInputUseCase::new(Box::new(MockValidator::new(true)));
        let request = ValidationRequest::new("trường");
        let result = use_case.execute(&request);
        assert!(result.is_valid());
    }
}
