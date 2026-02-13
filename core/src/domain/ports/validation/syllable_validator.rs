//! Syllable Validator Port
//!
//! Defines the abstraction for Vietnamese syllable validation.
//!
//! # Design Principles
//!
//! - **ISP**: Small, focused interface with 2 methods only
//! - **DIP**: Domain defines interface, infrastructure implements
//! - **SRP**: Only concerned with syllable validation, nothing else
//!
//! # Architecture
//!
//! ```text
//! Domain Layer (this file)
//!     ↓ defines interface
//! Infrastructure Layer
//!     ↓ implements
//! FsmValidatorAdapter, PhonотacticAdapter
//! ```

use crate::domain::{
    entities::syllable::Syllable,
    value_objects::validation_result::{ValidationError, ValidationResult},
};

/// Syllable validator port (interface)
///
/// Validates Vietnamese syllable structure according to phonotactic rules.
///
/// # Validation Rules
///
/// A valid Vietnamese syllable must satisfy:
///
/// 1. **Vowel nucleus is required** (a, ă, â, e, ê, i, o, ô, ơ, u, ư, y)
/// 2. **Initial consonant constraints** (e.g., /p/ rare at start)
/// 3. **Final consonant restrictions** (only: c, ch, m, n, ng, nh, p, t, i/y, u/o)
/// 4. **Tone-final rules** (p, t, c, ch only allow Sắc or Nặng)
/// 5. **Vowel-final compatibility** (e.g., ô/ơ/u/ư don't precede -ch)
///
/// # Implementations
///
/// - `FsmValidatorAdapter`: Finite State Machine based validation
/// - `PhonotacticAdapter`: Rule-based phonotactic validation
/// - `DictionaryAdapter`: Dictionary lookup validation
///
/// # Examples
///
/// ```ignore
/// let validator: Box<dyn SyllableValidator> = Box::new(FsmValidatorAdapter::new());
///
/// let valid = Syllable::from_parts("tr", "ươ", "ng", ToneType::Huyen);
/// assert!(validator.validate(&valid).is_valid());
///
/// let invalid = Syllable::from_parts("", "", "", ToneType::Ngang);
/// assert!(validator.validate(&invalid).is_invalid());
/// ```
pub trait SyllableValidator: Send + Sync {
    /// Validates a syllable structure
    ///
    /// # Arguments
    ///
    /// - `syllable`: The syllable to validate
    ///
    /// # Returns
    ///
    /// - `ValidationResult::Valid` if syllable follows Vietnamese phonotactics
    /// - `ValidationResult::Invalid(error)` if violates rules
    /// - `ValidationResult::Ambiguous` if needs more context
    ///
    /// # Examples (conceptual)
    ///
    /// ```ignore
    /// // Valid: "trường" (tr + ươ + ng + huyền)
    /// validator.validate(&truong) => Valid
    ///
    /// // Invalid: Empty vowel
    /// validator.validate(&no_vowel) => Invalid(MissingVowel)
    ///
    /// // Invalid: "tảp" (hỏi tone with -p final)
    /// validator.validate(&tap_hoi) => Invalid(InvalidToneFinalCombination)
    /// ```
    fn validate(&self, syllable: &Syllable) -> ValidationResult;

    /// Validates syllable with detailed error reporting
    ///
    /// # Arguments
    ///
    /// - `syllable`: The syllable to validate
    ///
    /// # Returns
    ///
    /// - `Ok(())` if valid
    /// - `Err(ValidationError)` with detailed error information
    ///
    /// # Default Implementation
    ///
    /// Calls `validate()` and converts result to `Result`.
    /// Override for custom error messages.
    fn validate_strict(&self, syllable: &Syllable) -> Result<(), ValidationError> {
        self.validate(syllable).into_result()
    }
}

/// Quick validation functions (convenience)
///
/// These are helper functions that implement common validation checks
/// without requiring a full `SyllableValidator` implementation.
pub mod quick {
    use super::*;
    use crate::domain::entities::tone::ToneType;

    /// Checks if syllable has required vowel nucleus
    ///
    /// # Arguments
    ///
    /// - `syllable`: Syllable to check
    ///
    /// # Returns
    ///
    /// - `true` if vowel is present and non-empty
    /// - `false` otherwise
    pub fn has_vowel(syllable: &Syllable) -> bool {
        !syllable.vowel().is_empty()
    }

    /// Validates tone-final consonant combination
    ///
    /// Rule: Final consonants p, t, c, ch can only have Sắc or Nặng tones.
    ///
    /// # Arguments
    ///
    /// - `tone`: The tone type
    /// - `final_consonant`: The final consonant (may be empty)
    ///
    /// # Returns
    ///
    /// - `true` if combination is valid
    /// - `false` if violates rule
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::domain::{
    /// #     entities::tone::ToneType,
    /// #     ports::validation::syllable_validator::quick,
    /// # };
    /// assert!(quick::is_valid_tone_final(ToneType::Sac, "p"));
    /// assert!(quick::is_valid_tone_final(ToneType::Nang, "t"));
    /// assert!(!quick::is_valid_tone_final(ToneType::Hoi, "p")); // Invalid!
    /// assert!(quick::is_valid_tone_final(ToneType::Hoi, "ng")); // OK (not stop consonant)
    /// ```
    pub fn is_valid_tone_final(tone: ToneType, final_consonant: &str) -> bool {
        // Stop consonants (p, t, c, ch) only allow Sắc or Nặng
        let is_stop = matches!(final_consonant, "p" | "t" | "c" | "ch");

        if is_stop {
            matches!(tone, ToneType::Sac | ToneType::Nang)
        } else {
            true // All tones valid for non-stop finals
        }
    }

    /// Checks if initial consonant is valid in Vietnamese
    ///
    /// # Arguments
    ///
    /// - `initial`: Initial consonant string
    ///
    /// # Returns
    ///
    /// - `true` if valid Vietnamese initial
    /// - `false` if invalid (e.g., consonant clusters like "bl", "kr")
    pub fn is_valid_initial(initial: &str) -> bool {
        // Empty is valid (vowel-initial syllables)
        if initial.is_empty() {
            return true;
        }

        // Valid Vietnamese initials
        const VALID_INITIALS: &[&str] = &[
            // Single consonants
            "b", "c", "d", "đ", "g", "h", "k", "l", "m", "n", "p", "q", "r", "s", "t", "v", "x",
            // Digraphs
            "ch", "gh", "gi", "kh", "ng", "nh", "ph", "qu", "th", "tr",
            // Trigraph
            "ngh",
        ];

        VALID_INITIALS.contains(&initial)
    }

    /// Checks if final consonant is valid in Vietnamese
    ///
    /// # Arguments
    ///
    /// - `final_c`: Final consonant string
    ///
    /// # Returns
    ///
    /// - `true` if valid Vietnamese final
    /// - `false` if invalid
    pub fn is_valid_final(final_c: &str) -> bool {
        // Empty is valid (open syllables)
        if final_c.is_empty() {
            return true;
        }

        // Valid Vietnamese finals
        const VALID_FINALS: &[&str] = &[
            // Stops
            "c", "ch", "p", "t", // Nasals
            "m", "n", "ng", "nh", // Semivowels (treated as finals in Vietnamese)
            "i", "y", "u", "o",
        ];

        VALID_FINALS.contains(&final_c)
    }
}

#[cfg(test)]
mod tests {
    use super::quick::*;
    use super::*;
    use crate::domain::entities::tone::ToneType;

    // Test quick validation functions

    #[test]
    fn test_has_vowel() {
        let with_vowel = Syllable::from_parts("tr", "ươ", "ng", ToneType::Ngang);
        assert!(has_vowel(&with_vowel));

        let no_vowel = Syllable::from_parts("tr", "", "ng", ToneType::Ngang);
        assert!(!has_vowel(&no_vowel));
    }

    #[test]
    fn test_is_valid_tone_final_stop_consonants() {
        // Stop consonants (p, t, c, ch) only allow Sắc or Nặng
        for final_c in &["p", "t", "c", "ch"] {
            assert!(is_valid_tone_final(ToneType::Sac, final_c));
            assert!(is_valid_tone_final(ToneType::Nang, final_c));

            assert!(!is_valid_tone_final(ToneType::Ngang, final_c));
            assert!(!is_valid_tone_final(ToneType::Huyen, final_c));
            assert!(!is_valid_tone_final(ToneType::Hoi, final_c));
            assert!(!is_valid_tone_final(ToneType::Nga, final_c));
        }
    }

    #[test]
    fn test_is_valid_tone_final_non_stop() {
        // Non-stop finals allow all tones
        for final_c in &["m", "n", "ng", "nh", "i", "u", ""] {
            assert!(is_valid_tone_final(ToneType::Ngang, final_c));
            assert!(is_valid_tone_final(ToneType::Huyen, final_c));
            assert!(is_valid_tone_final(ToneType::Sac, final_c));
            assert!(is_valid_tone_final(ToneType::Hoi, final_c));
            assert!(is_valid_tone_final(ToneType::Nga, final_c));
            assert!(is_valid_tone_final(ToneType::Nang, final_c));
        }
    }

    #[test]
    fn test_is_valid_initial() {
        // Valid singles
        assert!(is_valid_initial("b"));
        assert!(is_valid_initial("t"));
        assert!(is_valid_initial("đ"));

        // Valid digraphs
        assert!(is_valid_initial("ch"));
        assert!(is_valid_initial("tr"));
        assert!(is_valid_initial("qu"));

        // Valid trigraph
        assert!(is_valid_initial("ngh"));

        // Empty (vowel-initial)
        assert!(is_valid_initial(""));

        // Invalid (consonant clusters)
        assert!(!is_valid_initial("bl"));
        assert!(!is_valid_initial("kr"));
        assert!(!is_valid_initial("str"));
        assert!(!is_valid_initial("xyz"));
    }

    #[test]
    fn test_is_valid_final() {
        // Valid stops
        assert!(is_valid_final("c"));
        assert!(is_valid_final("t"));
        assert!(is_valid_final("p"));
        assert!(is_valid_final("ch"));

        // Valid nasals
        assert!(is_valid_final("m"));
        assert!(is_valid_final("n"));
        assert!(is_valid_final("ng"));
        assert!(is_valid_final("nh"));

        // Valid semivowels
        assert!(is_valid_final("i"));
        assert!(is_valid_final("u"));

        // Empty (open syllable)
        assert!(is_valid_final(""));

        // Invalid
        assert!(!is_valid_final("b"));
        assert!(!is_valid_final("d"));
        assert!(!is_valid_final("tr"));
        assert!(!is_valid_final("xyz"));
    }

    #[test]
    fn test_vietnamese_examples_validation() {
        // "trường" (tr + ươ + ng + huyền) - VALID
        let truong = Syllable::from_parts("tr", "ươ", "ng", ToneType::Huyen);
        assert!(has_vowel(&truong));
        assert!(is_valid_initial("tr"));
        assert!(is_valid_final("ng"));
        assert!(is_valid_tone_final(ToneType::Huyen, "ng"));

        // "cấp" (c + â + p + sắc) - VALID
        let cap = Syllable::from_parts("c", "â", "p", ToneType::Sac);
        assert!(has_vowel(&cap));
        assert!(is_valid_initial("c"));
        assert!(is_valid_final("p"));
        assert!(is_valid_tone_final(ToneType::Sac, "p"));

        // "cảp" (c + ả + p + hỏi) - INVALID (hỏi with stop final)
        assert!(!is_valid_tone_final(ToneType::Hoi, "p"));

        // "tiếng" (t + iê + ng + sắc) - VALID
        let tieng = Syllable::from_parts("t", "iê", "ng", ToneType::Sac);
        assert!(has_vowel(&tieng));
        assert!(is_valid_initial("t"));
        assert!(is_valid_final("ng"));
        assert!(is_valid_tone_final(ToneType::Sac, "ng"));
    }
}
