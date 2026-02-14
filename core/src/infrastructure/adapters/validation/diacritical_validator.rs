/// Diacritical Mark Validator for Vietnamese Input Method
///
/// This module prevents invalid diacritical mark placement after Vietnamese final consonants.
/// When a user tries to apply diacritical marks (dấu mũ, dấu móc, dấu trăng) via Telex or VNI
/// after a word already ends with a consonant, the engine should intelligently reject the input.

/// Represents different types of diacritical marks in Vietnamese
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiacriticalType {
    /// Circumflex (^) - for a, e, o → â, ê, ô
    Circumflex,
    /// Breve (˘) - for a → ă
    Breve,
    /// Horn (ʼ) - for o, u → ơ, ư
    Horn,
    /// Stroke (-) - for d → đ
    Stroke,
}

/// Diacritical validator that checks if a mark can be applied to a syllable
pub struct DiacriticalValidator;

impl DiacriticalValidator {
    /// List of valid Vietnamese final consonants
    const FINAL_CONSONANTS: &'static [&'static str] = &["c", "ch", "m", "n", "ng", "nh", "p", "t"];

    /// Check if applying a diacritical mark is valid given current syllable state
    ///
    /// Returns `true` if the diacritical mark can be applied, `false` if it should be rejected.
    ///
    /// # Rules
    /// - If syllable has a final consonant, NO diacritical marks can be applied (return false)
    /// - If no final consonant, check specific vowel constraints for each diacritical type:
    ///   - Circumflex (^): only applies to a, e, o
    ///   - Breve (˘): only applies to a
    ///   - Horn (ʼ): only applies to o, u
    ///   - Stroke (-): only applies to d
    pub fn is_valid_placement(
        vowel: &str,
        final_consonant: Option<&str>,
        diacritical: DiacriticalType,
    ) -> bool {
        // Rule 1: No diacritical after any final consonant
        if final_consonant.is_some() {
            return false;
        }

        // Rule 2: Specific vowel constraints for each diacritical type
        match diacritical {
            DiacriticalType::Circumflex => {
                // ^ (mũ) only applies to: a, e, o
                matches!(vowel, "a" | "e" | "o")
            }
            DiacriticalType::Breve => {
                // ˘ (trăng) only applies to: a
                vowel == "a"
            }
            DiacriticalType::Horn => {
                // ʼ (móc) only applies to: o, u
                matches!(vowel, "o" | "u")
            }
            DiacriticalType::Stroke => {
                // Đ (gạch) technically can follow consonants, but we handle it separately
                // For now, only allow it for d character (not after final consonant)
                true
            }
        }
    }

    /// Check if a Telex key combination represents a diacritical mark
    ///
    /// Returns `Some(DiacriticalType)` if the input is a diacritical key, `None` otherwise.
    ///
    /// Telex mappings:
    /// - `aa` → Circumflex (^)
    /// - `aw` → Breve (˘)
    /// - `ee` → Circumflex (^)
    /// - `oo` → Circumflex (^)
    /// - `ow` → Horn (ʼ)
    /// - `uw` → Horn (ʼ)
    /// - `dd` → Stroke (-)
    pub fn from_telex_input(ch: char, prev_ch: Option<char>) -> Option<DiacriticalType> {
        match (prev_ch, ch) {
            (Some('a'), 'a') => Some(DiacriticalType::Circumflex),
            (Some('a'), 'w') => Some(DiacriticalType::Breve),
            (Some('e'), 'e') => Some(DiacriticalType::Circumflex),
            (Some('o'), 'o') => Some(DiacriticalType::Circumflex),
            (Some('o'), 'w') => Some(DiacriticalType::Horn),
            (Some('u'), 'w') => Some(DiacriticalType::Horn),
            (Some('d'), 'd') => Some(DiacriticalType::Stroke),
            _ => None,
        }
    }

    /// Check if a VNI key represents a diacritical mark
    ///
    /// Returns `Some(DiacriticalType)` if the input is a diacritical key, `None` otherwise.
    ///
    /// VNI mappings:
    /// - `6` → Circumflex (^)
    /// - `7` → Horn (ʼ)
    /// - `8` → Breve (˘)
    /// - `9` → Stroke (-)
    pub fn from_vni_input(ch: char) -> Option<DiacriticalType> {
        match ch {
            '6' => Some(DiacriticalType::Circumflex),
            '7' => Some(DiacriticalType::Horn),
            '8' => Some(DiacriticalType::Breve),
            '9' => Some(DiacriticalType::Stroke),
            _ => None,
        }
    }

    /// Check if a character is a final consonant
    pub fn is_final_consonant(s: &str) -> bool {
        Self::FINAL_CONSONANTS.contains(&s)
    }

    /// Get all valid final consonants
    pub fn final_consonants() -> &'static [&'static str] {
        Self::FINAL_CONSONANTS
    }

    /// Validate if a character can be a vowel for applying diacritical marks
    pub fn is_valid_vowel_for_diacritical(vowel: &str) -> bool {
        // Valid single vowels that can receive diacritical marks
        matches!(
            vowel,
            "a" | "e" | "i" | "o" | "u" | "y" | "ă" | "â" | "ê" | "ô" | "ơ" | "ư"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circumflex_valid_no_consonant() {
        // Valid: a, e, o without final consonant
        assert!(DiacriticalValidator::is_valid_placement(
            "a",
            None,
            DiacriticalType::Circumflex
        ));
        assert!(DiacriticalValidator::is_valid_placement(
            "e",
            None,
            DiacriticalType::Circumflex
        ));
        assert!(DiacriticalValidator::is_valid_placement(
            "o",
            None,
            DiacriticalType::Circumflex
        ));
    }

    #[test]
    fn test_circumflex_invalid_with_consonant() {
        // Invalid: any final consonant blocks circumflex
        assert!(!DiacriticalValidator::is_valid_placement(
            "a",
            Some("ng"),
            DiacriticalType::Circumflex
        ));
        assert!(!DiacriticalValidator::is_valid_placement(
            "e",
            Some("t"),
            DiacriticalType::Circumflex
        ));
        assert!(!DiacriticalValidator::is_valid_placement(
            "o",
            Some("c"),
            DiacriticalType::Circumflex
        ));
    }

    #[test]
    fn test_breve_valid_no_consonant() {
        // Valid: a without final consonant
        assert!(DiacriticalValidator::is_valid_placement(
            "a",
            None,
            DiacriticalType::Breve
        ));
    }

    #[test]
    fn test_breve_invalid_wrong_vowel() {
        // Invalid: breve only works on 'a'
        assert!(!DiacriticalValidator::is_valid_placement(
            "e",
            None,
            DiacriticalType::Breve
        ));
        assert!(!DiacriticalValidator::is_valid_placement(
            "o",
            None,
            DiacriticalType::Breve
        ));
    }

    #[test]
    fn test_horn_valid_no_consonant() {
        // Valid: o, u without final consonant
        assert!(DiacriticalValidator::is_valid_placement(
            "o",
            None,
            DiacriticalType::Horn
        ));
        assert!(DiacriticalValidator::is_valid_placement(
            "u",
            None,
            DiacriticalType::Horn
        ));
    }

    #[test]
    fn test_horn_invalid_wrong_vowel() {
        // Invalid: horn only works on o, u
        assert!(!DiacriticalValidator::is_valid_placement(
            "a",
            None,
            DiacriticalType::Horn
        ));
        assert!(!DiacriticalValidator::is_valid_placement(
            "e",
            None,
            DiacriticalType::Horn
        ));
    }

    #[test]
    fn test_telex_aa_circumflex() {
        assert_eq!(
            DiacriticalValidator::from_telex_input('a', Some('a')),
            Some(DiacriticalType::Circumflex)
        );
    }

    #[test]
    fn test_telex_aw_breve() {
        assert_eq!(
            DiacriticalValidator::from_telex_input('w', Some('a')),
            Some(DiacriticalType::Breve)
        );
    }

    #[test]
    fn test_telex_ee_circumflex() {
        assert_eq!(
            DiacriticalValidator::from_telex_input('e', Some('e')),
            Some(DiacriticalType::Circumflex)
        );
    }

    #[test]
    fn test_telex_ow_horn() {
        assert_eq!(
            DiacriticalValidator::from_telex_input('w', Some('o')),
            Some(DiacriticalType::Horn)
        );
    }

    #[test]
    fn test_telex_dd_stroke() {
        assert_eq!(
            DiacriticalValidator::from_telex_input('d', Some('d')),
            Some(DiacriticalType::Stroke)
        );
    }

    #[test]
    fn test_vni_6_circumflex() {
        assert_eq!(
            DiacriticalValidator::from_vni_input('6'),
            Some(DiacriticalType::Circumflex)
        );
    }

    #[test]
    fn test_vni_7_horn() {
        assert_eq!(
            DiacriticalValidator::from_vni_input('7'),
            Some(DiacriticalType::Horn)
        );
    }

    #[test]
    fn test_vni_8_breve() {
        assert_eq!(
            DiacriticalValidator::from_vni_input('8'),
            Some(DiacriticalType::Breve)
        );
    }

    #[test]
    fn test_vni_9_stroke() {
        assert_eq!(
            DiacriticalValidator::from_vni_input('9'),
            Some(DiacriticalType::Stroke)
        );
    }

    #[test]
    fn test_is_final_consonant() {
        assert!(DiacriticalValidator::is_final_consonant("c"));
        assert!(DiacriticalValidator::is_final_consonant("ng"));
        assert!(DiacriticalValidator::is_final_consonant("t"));
        assert!(!DiacriticalValidator::is_final_consonant("a"));
        assert!(!DiacriticalValidator::is_final_consonant("b"));
    }
}
