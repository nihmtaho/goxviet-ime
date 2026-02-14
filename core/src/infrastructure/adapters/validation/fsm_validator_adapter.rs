//! FSM-Based Vietnamese Syllable Validator Adapter
//!
//! Implements `SyllableValidator` using the engine_v2's FSM-based Vietnamese validator.
//! This adapter provides high-performance validation using finite state machine logic
//! and comprehensive phonotactic rules.

use crate::domain::{
    entities::syllable::Syllable,
    ports::validation::syllable_validator::{quick, SyllableValidator},
    value_objects::validation_result::{ValidationError, ValidationResult},
};
use crate::infrastructure::external::vietnamese_validator::VietnameseSyllableValidator;

/// FSM-based Vietnamese syllable validator
///
/// This adapter wraps the engine_v2's `VietnameseSyllableValidator` to provide
/// domain-level validation through the `SyllableValidator` port.
///
/// # Validation Strategy
///
/// 1. Parse syllable characters into keys and tone modifiers
/// 2. Check for vowel presence (required for valid syllable)
/// 3. Validate structure using FSM-based validator with tone information
/// 4. Enforce tone-final consonant rules (p, t, c, ch with Sắc/Nặng only)
///
/// # Examples
///
/// ```
/// use goxviet_core::infrastructure::adapters::validation::FsmValidatorAdapter;
/// use goxviet_core::domain::ports::validation::syllable_validator::SyllableValidator;
/// use goxviet_core::domain::entities::{syllable::Syllable, tone::ToneType};
///
/// let validator = FsmValidatorAdapter::new();
///
/// // Valid syllable
/// let valid = Syllable::from_parts("tr", "ươ", "ng", ToneType::Huyen);
/// assert!(validator.validate(&valid).is_valid());
///
/// // Invalid: no vowel
/// let no_vowel = Syllable::from_parts("tr", "", "ng", ToneType::Ngang);
/// assert!(validator.validate(&no_vowel).is_invalid());
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct FsmValidatorAdapter;

impl FsmValidatorAdapter {
    /// Creates a new FSM validator adapter
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::infrastructure::adapters::validation::FsmValidatorAdapter;
    /// let validator = FsmValidatorAdapter::new();
    /// ```
    pub fn new() -> Self {
        Self
    }

    /// Converts syllable to keys and tone modifiers for validation
    ///
    /// Parses each character in the syllable (initial + vowel + final) into
    /// its components (key, tone modifier, mark, etc.).
    ///
    /// Returns None if any character cannot be parsed or if vowel is missing.
    fn parse_syllable(&self, syllable: &Syllable) -> Option<(Vec<u16>, Vec<u8>)> {
        use crate::data::chars::parse_char;

        let mut keys = Vec::new();
        let mut tones = Vec::new();

        // Parse all components: initial + vowel + final
        let full_text = format!(
            "{}{}{}",
            syllable.initial().as_str(),
            syllable.vowel().as_str(),
            syllable.final_consonant().as_str()
        );

        for ch in full_text.chars() {
            if let Some(parsed) = parse_char(ch) {
                keys.push(parsed.key);
                tones.push(parsed.tone);
            } else {
                // Unknown character - cannot parse
                return None;
            }
        }

        Some((keys, tones))
    }
}

impl SyllableValidator for FsmValidatorAdapter {
    fn validate(&self, syllable: &Syllable) -> ValidationResult {
        // Rule 1: Must have vowel nucleus
        if !quick::has_vowel(syllable) {
            return ValidationResult::invalid(ValidationError::Empty);
        }

        // Parse syllable into keys and tone modifiers
        let (keys, tones) = match self.parse_syllable(syllable) {
            Some(parsed) => parsed,
            None => {
                return ValidationResult::invalid(ValidationError::InvalidStructure {
                    syllable: syllable.base_form(),
                    reason: "Failed to parse syllable characters".to_string(),
                });
            }
        };

        // Rule 2: Validate using FSM-based validator with tone information
        let validation_result = VietnameseSyllableValidator::validate_with_tones(&keys, &tones);

        if !validation_result.is_valid {
            return ValidationResult::invalid(ValidationError::InvalidStructure {
                syllable: syllable.base_form(),
                reason: "FSM validator rejected syllable structure".to_string(),
            });
        }

        // Rule 3: Enforce tone-final consonant rule
        // Stop consonants (p, t, c, ch) can only have Sắc or Nặng tones
        if !quick::is_valid_tone_final(syllable.tone(), syllable.final_consonant().as_str()) {
            return ValidationResult::invalid(ValidationError::InvalidTonePlacement {
                syllable: syllable.base_form(),
                reason: format!(
                    "Stop consonant '{}' cannot have {:?} tone",
                    syllable.final_consonant().as_str(),
                    syllable.tone()
                ),
            });
        }

        ValidationResult::valid()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::tone::ToneType;

    #[test]
    fn test_fsm_validator_valid_syllable() {
        let validator = FsmValidatorAdapter::new();

        // "trường" (tr + ươ + ng + huyền)
        let truong = Syllable::from_parts("tr", "ươ", "ng", ToneType::Huyen);
        assert!(validator.validate(&truong).is_valid());

        // "tiếng" (t + iê + ng + sắc)
        let tieng = Syllable::from_parts("t", "iê", "ng", ToneType::Sac);
        assert!(validator.validate(&tieng).is_valid());

        // "hoa" (h + oa + "" + ngang)
        let hoa = Syllable::from_parts("h", "oa", "", ToneType::Ngang);
        assert!(validator.validate(&hoa).is_valid());
    }

    #[test]
    fn test_fsm_validator_no_vowel() {
        let validator = FsmValidatorAdapter::new();

        // Empty vowel
        let no_vowel = Syllable::from_parts("tr", "", "ng", ToneType::Ngang);
        let result = validator.validate(&no_vowel);
        assert!(result.is_invalid());
        assert_eq!(result.error(), Some(&ValidationError::Empty));
    }

    #[test]
    fn test_fsm_validator_invalid_structure() {
        let validator = FsmValidatorAdapter::new();

        // Invalid consonant cluster "bl" (not Vietnamese)
        let invalid = Syllable::from_parts("bl", "a", "", ToneType::Ngang);
        let result = validator.validate(&invalid);
        // Note: parse_char might fail for 'bl' or FSM will reject it
        assert!(result.is_invalid());
    }

    #[test]
    fn test_fsm_validator_tone_final_violation() {
        let validator = FsmValidatorAdapter::new();

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

        // "cấm" (c + ấ + m + sắc) - VALID (m is not a stop consonant)
        let cam_sac = Syllable::from_parts("c", "ấ", "m", ToneType::Sac);
        assert!(validator.validate(&cam_sac).is_valid());
    }

    #[test]
    fn test_fsm_validator_validate_strict() {
        let validator = FsmValidatorAdapter::new();

        // Valid syllable
        let valid = Syllable::from_parts("tr", "ươ", "ng", ToneType::Huyen);
        assert!(validator.validate_strict(&valid).is_ok());

        // Invalid syllable
        let invalid = Syllable::from_parts("tr", "", "ng", ToneType::Ngang);
        assert!(validator.validate_strict(&invalid).is_err());
    }
}
