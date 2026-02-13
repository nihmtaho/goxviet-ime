//! Rule-Based Phonotactic Validator Adapter
//!
//! Implements `SyllableValidator` using simple rule-based phonotactic checks.
//! This adapter provides fast, deterministic validation without complex FSM logic.

use crate::domain::{
    entities::syllable::Syllable,
    ports::validation::syllable_validator::{quick, SyllableValidator},
    value_objects::validation_result::{ValidationError, ValidationResult},
};

/// Rule-based phonotactic validator
///
/// This adapter implements syllable validation using straightforward phonotactic
/// rules provided by the `syllable_validator::quick` module.
///
/// # Validation Rules
///
/// 1. **Vowel required**: Syllable must have a non-empty vowel nucleus
/// 2. **Valid initial**: Initial consonant must be valid Vietnamese initial
/// 3. **Valid final**: Final consonant must be valid Vietnamese final
/// 4. **Tone-final rule**: Stop consonants (p, t, c, ch) only allow Sắc or Nặng tones
///
/// # Design
///
/// - **Simple**: Uses only quick check functions, no complex state machine
/// - **Deterministic**: Rule-based logic with predictable outcomes
/// - **Fast**: O(1) checks using constant arrays and simple pattern matching
///
/// # Examples
///
/// ```
/// use goxviet_core::infrastructure::adapters::validation::PhonotacticAdapter;
/// use goxviet_core::domain::ports::validation::syllable_validator::SyllableValidator;
/// use goxviet_core::domain::entities::{syllable::Syllable, tone::ToneType};
///
/// let validator = PhonotacticAdapter::new();
///
/// // Valid syllable
/// let valid = Syllable::from_parts("tr", "ươ", "ng", ToneType::Huyen);
/// assert!(validator.validate(&valid).is_valid());
///
/// // Invalid initial consonant
/// let invalid = Syllable::from_parts("bl", "a", "", ToneType::Ngang);
/// assert!(validator.validate(&invalid).is_invalid());
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct PhonotacticAdapter;

impl PhonotacticAdapter {
    /// Creates a new phonotactic validator adapter
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::infrastructure::adapters::validation::PhonotacticAdapter;
    /// let validator = PhonotacticAdapter::new();
    /// ```
    pub fn new() -> Self {
        Self
    }
}

impl SyllableValidator for PhonotacticAdapter {
    fn validate(&self, syllable: &Syllable) -> ValidationResult {
        // Rule 1: Vowel nucleus is required
        if !quick::has_vowel(syllable) {
            return ValidationResult::invalid(ValidationError::Empty);
        }

        // Rule 2: Initial consonant must be valid Vietnamese initial
        let initial = syllable.initial().as_str();
        if !quick::is_valid_initial(initial) {
            return ValidationResult::invalid(ValidationError::InvalidConsonant {
                consonant: initial.to_string(),
                context: "Invalid initial consonant".to_string(),
            });
        }

        // Rule 3: Final consonant must be valid Vietnamese final
        let final_c = syllable.final_consonant().as_str();
        if !quick::is_valid_final(final_c) {
            return ValidationResult::invalid(ValidationError::InvalidConsonant {
                consonant: final_c.to_string(),
                context: "Invalid final consonant".to_string(),
            });
        }

        // Rule 4: Tone-final consonant combination must be valid
        // Stop consonants (p, t, c, ch) can only have Sắc or Nặng tones
        if !quick::is_valid_tone_final(syllable.tone(), final_c) {
            return ValidationResult::invalid(ValidationError::InvalidTonePlacement {
                syllable: syllable.base_form(),
                reason: format!(
                    "Stop consonant '{}' cannot have {:?} tone",
                    final_c,
                    syllable.tone()
                ),
            });
        }

        // All rules passed
        ValidationResult::valid()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::tone::ToneType;

    #[test]
    fn test_phonotactic_valid_syllable() {
        let validator = PhonotacticAdapter::new();

        // "trường" (tr + ươ + ng + huyền)
        let truong = Syllable::from_parts("tr", "ươ", "ng", ToneType::Huyen);
        assert!(validator.validate(&truong).is_valid());

        // "tiếng" (t + iê + ng + sắc)
        let tieng = Syllable::from_parts("t", "iê", "ng", ToneType::Sac);
        assert!(validator.validate(&tieng).is_valid());

        // "hoa" (h + oa + "" + ngang)
        let hoa = Syllable::from_parts("h", "oa", "", ToneType::Ngang);
        assert!(validator.validate(&hoa).is_valid());

        // Vowel-initial syllable
        let anh = Syllable::from_parts("", "a", "nh", ToneType::Ngang);
        assert!(validator.validate(&anh).is_valid());
    }

    #[test]
    fn test_phonotactic_no_vowel() {
        let validator = PhonotacticAdapter::new();

        // Empty vowel
        let no_vowel = Syllable::from_parts("tr", "", "ng", ToneType::Ngang);
        let result = validator.validate(&no_vowel);
        assert!(result.is_invalid());
        assert_eq!(result.error(), Some(&ValidationError::Empty));
    }

    #[test]
    fn test_phonotactic_invalid_initial() {
        let validator = PhonotacticAdapter::new();

        // Invalid consonant cluster "bl" (not Vietnamese)
        let invalid = Syllable::from_parts("bl", "a", "", ToneType::Ngang);
        let result = validator.validate(&invalid);
        assert!(result.is_invalid());
        assert!(matches!(
            result.error(),
            Some(ValidationError::InvalidConsonant { .. })
        ));

        // Invalid initial "xyz"
        let xyz = Syllable::from_parts("xyz", "a", "", ToneType::Ngang);
        let result = validator.validate(&xyz);
        assert!(result.is_invalid());
    }

    #[test]
    fn test_phonotactic_invalid_final() {
        let validator = PhonotacticAdapter::new();

        // Invalid final consonant "b" (not allowed as final)
        let invalid = Syllable::from_parts("", "a", "b", ToneType::Ngang);
        let result = validator.validate(&invalid);
        assert!(result.is_invalid());
        assert!(matches!(
            result.error(),
            Some(ValidationError::InvalidConsonant { .. })
        ));

        // Invalid final "xyz"
        let xyz = Syllable::from_parts("", "a", "xyz", ToneType::Ngang);
        let result = validator.validate(&xyz);
        assert!(result.is_invalid());
    }

    #[test]
    fn test_phonotactic_tone_final_violation() {
        let validator = PhonotacticAdapter::new();

        // "cấp" (c + â + p + sắc) - VALID
        let cap_sac = Syllable::from_parts("c", "â", "p", ToneType::Sac);
        assert!(validator.validate(&cap_sac).is_valid());

        // "cập" (c + ạ + p + nặng) - VALID
        let cap_nang = Syllable::from_parts("c", "ạ", "p", ToneType::Nang);
        assert!(validator.validate(&cap_nang).is_valid());

        // "cảp" (c + ả + p + hỏi) - INVALID (hỏi with stop consonant -p)
        let cap_hoi = Syllable::from_parts("c", "ả", "p", ToneType::Hoi);
        let result = validator.validate(&cap_hoi);
        assert!(result.is_invalid());
        assert!(matches!(
            result.error(),
            Some(ValidationError::InvalidTonePlacement { .. })
        ));

        // Test all stop consonants
        for stop in &["p", "t", "c", "ch"] {
            // Sắc should be valid
            let sac = Syllable::from_parts("", "a", *stop, ToneType::Sac);
            assert!(validator.validate(&sac).is_valid());

            // Nặng should be valid
            let nang = Syllable::from_parts("", "a", *stop, ToneType::Nang);
            assert!(validator.validate(&nang).is_valid());

            // Hỏi should be invalid
            let hoi = Syllable::from_parts("", "a", *stop, ToneType::Hoi);
            assert!(validator.validate(&hoi).is_invalid());

            // Ngã should be invalid
            let nga = Syllable::from_parts("", "a", *stop, ToneType::Nga);
            assert!(validator.validate(&nga).is_invalid());
        }

        // Non-stop finals should allow all tones
        let cam = Syllable::from_parts("c", "ả", "m", ToneType::Hoi);
        assert!(validator.validate(&cam).is_valid());

        let can = Syllable::from_parts("c", "ã", "n", ToneType::Nga);
        assert!(validator.validate(&can).is_valid());
    }

    #[test]
    fn test_phonotactic_validate_strict() {
        let validator = PhonotacticAdapter::new();

        // Valid syllable
        let valid = Syllable::from_parts("tr", "ươ", "ng", ToneType::Huyen);
        assert!(validator.validate_strict(&valid).is_ok());

        // Invalid syllable
        let invalid = Syllable::from_parts("bl", "a", "", ToneType::Ngang);
        assert!(validator.validate_strict(&invalid).is_err());
    }
}
