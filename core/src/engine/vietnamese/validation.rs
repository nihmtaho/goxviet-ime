//! Centralized Vietnamese validation module
//!
//! Provides validation functions to be called BEFORE any transform is applied.
//! This ensures we only transform valid Vietnamese syllables.

use crate::engine_v2::vietnamese_validator::{ValidationResult, VietnameseSyllableValidator};

/// Check if a sequence of keys forms a valid Vietnamese syllable
#[inline]
pub fn is_valid_vietnamese_syllable(keys: &[u16]) -> bool {
    VietnameseSyllableValidator::validate(keys).is_valid
}

/// Check if adding a new key to current buffer would create valid Vietnamese
///
/// This is critical for transform decisions: we should only apply transforms
/// (like aa → â) if the resulting syllable would be valid Vietnamese.
#[inline]
pub fn would_be_valid_with_key(current_keys: &[u16], new_key: u16) -> bool {
    let mut simulated = current_keys.to_vec();
    simulated.push(new_key);
    is_valid_vietnamese_syllable(&simulated)
}

/// Get full validation result with confidence score
#[inline]
pub fn validate_with_confidence(keys: &[u16]) -> ValidationResult {
    VietnameseSyllableValidator::validate(keys)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::keys;

    #[test]
    fn test_valid_syllables() {
        assert!(is_valid_vietnamese_syllable(&[keys::A, keys::C, keys::H])); // ach
        assert!(is_valid_vietnamese_syllable(&[
            keys::N,
            keys::G,
            keys::H,
            keys::I,
            keys::A
        ])); // nghia
    }

    #[test]
    fn test_invalid_syllables() {
        assert!(!is_valid_vietnamese_syllable(&[keys::O, keys::C, keys::H])); // och
        assert!(!is_valid_vietnamese_syllable(&[
            keys::C,
            keys::L,
            keys::E,
            keys::A,
            keys::N
        ])); // clean
    }

    #[test]
    fn test_would_be_valid() {
        // "ach" + "a" would be "acha" - valid
        assert!(would_be_valid_with_key(
            &[keys::A, keys::C, keys::H],
            keys::A
        ));

        // "och" + "a" would be "ocha" - invalid (o + ch not allowed)
        assert!(!would_be_valid_with_key(
            &[keys::O, keys::C, keys::H],
            keys::A
        ));
    }
}
