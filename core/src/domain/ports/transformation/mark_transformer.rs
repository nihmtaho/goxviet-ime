//! Mark Transformer Port
//!
//! Defines the abstraction for applying Vietnamese diacritic marks (circumflex, horn, breve).
//!
//! # Design Principles
//!
//! - **ISP**: Small interface with 3 methods
//! - **DIP**: Domain defines contract, infrastructure implements
//! - **SRP**: Only transforms diacritic marks, not tones
//!
//! # Architecture
//!
//! ```text
//! Domain Layer (this file)
//!     ↓ defines interface
//! Infrastructure Layer
//!     ↓ implements
//! MarkComposerAdapter, UnicodeNormalizerAdapter
//! ```

use crate::domain::{
    entities::syllable::Syllable,
    value_objects::{char_sequence::CharSequence, transformation::TransformResult},
};

/// Diacritic mark types for Vietnamese vowels
///
/// These are vowel modifiers (not tone marks).
///
/// # Examples
///
/// - **Circumflex**: a→â, e→ê, o→ô
/// - **Horn**: o→ơ, u→ư
/// - **Breve**: a→ă
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MarkType {
    /// Circumflex (^): a→â, e→ê, o→ô
    Circumflex,
    /// Horn (ʼ): o→ơ, u→ư
    Horn,
    /// Breve (˘): a→ă
    Breve,
}

impl MarkType {
    /// Gets the Unicode combining character for this mark
    ///
    /// # Returns
    ///
    /// Unicode code point for the combining diacritic
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::domain::ports::transformation::mark_transformer::MarkType;
    /// assert_eq!(MarkType::Circumflex.combining_char(), '\u{0302}');
    /// assert_eq!(MarkType::Horn.combining_char(), '\u{031B}');
    /// assert_eq!(MarkType::Breve.combining_char(), '\u{0306}');
    /// ```
    pub fn combining_char(&self) -> char {
        match self {
            Self::Circumflex => '\u{0302}', // Combining circumflex
            Self::Horn => '\u{031B}',       // Combining horn
            Self::Breve => '\u{0306}',      // Combining breve
        }
    }

    /// Applies mark to a vowel character
    ///
    /// # Arguments
    ///
    /// - `vowel`: Base vowel (a, e, o, u)
    ///
    /// # Returns
    ///
    /// - `Some(char)` with mark applied if valid
    /// - `None` if mark cannot be applied to this vowel
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::domain::ports::transformation::mark_transformer::MarkType;
    /// assert_eq!(MarkType::Circumflex.apply_to('a'), Some('â'));
    /// assert_eq!(MarkType::Circumflex.apply_to('e'), Some('ê'));
    /// assert_eq!(MarkType::Circumflex.apply_to('o'), Some('ô'));
    /// assert_eq!(MarkType::Circumflex.apply_to('i'), None); // Cannot apply
    ///
    /// assert_eq!(MarkType::Horn.apply_to('o'), Some('ơ'));
    /// assert_eq!(MarkType::Horn.apply_to('u'), Some('ư'));
    /// assert_eq!(MarkType::Horn.apply_to('a'), None); // Cannot apply
    ///
    /// assert_eq!(MarkType::Breve.apply_to('a'), Some('ă'));
    /// assert_eq!(MarkType::Breve.apply_to('e'), None); // Cannot apply
    /// ```
    pub fn apply_to(&self, vowel: char) -> Option<char> {
        match (self, vowel) {
            // Circumflex: a, e, o
            (Self::Circumflex, 'a') | (Self::Circumflex, 'A') => Some('â'),
            (Self::Circumflex, 'e') | (Self::Circumflex, 'E') => Some('ê'),
            (Self::Circumflex, 'o') | (Self::Circumflex, 'O') => Some('ô'),

            // Horn: o, u
            (Self::Horn, 'o') | (Self::Horn, 'O') => Some('ơ'),
            (Self::Horn, 'u') | (Self::Horn, 'U') => Some('ư'),

            // Breve: a only
            (Self::Breve, 'a') | (Self::Breve, 'A') => Some('ă'),

            // Invalid combinations
            _ => None,
        }
    }

    /// Checks if this mark can be applied to a vowel
    ///
    /// # Arguments
    ///
    /// - `vowel`: Base vowel character
    ///
    /// # Returns
    ///
    /// `true` if mark is valid for this vowel
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::domain::ports::transformation::mark_transformer::MarkType;
    /// assert!(MarkType::Circumflex.can_apply_to('a'));
    /// assert!(MarkType::Circumflex.can_apply_to('e'));
    /// assert!(!MarkType::Circumflex.can_apply_to('i'));
    ///
    /// assert!(MarkType::Horn.can_apply_to('o'));
    /// assert!(MarkType::Horn.can_apply_to('u'));
    /// assert!(!MarkType::Horn.can_apply_to('a'));
    ///
    /// assert!(MarkType::Breve.can_apply_to('a'));
    /// assert!(!MarkType::Breve.can_apply_to('e'));
    /// ```
    pub fn can_apply_to(&self, vowel: char) -> bool {
        self.apply_to(vowel).is_some()
    }
}

/// Mark transformer port (interface)
///
/// Transforms vowels by applying diacritic marks.
///
/// # Mark Application Rules
///
/// 1. **Circumflex (^)**: Only on a, e, o → â, ê, ô
/// 2. **Horn (ʼ)**: Only on o, u → ơ, ư
/// 3. **Breve (˘)**: Only on a → ă
///
/// # Implementations
///
/// - `MarkComposerAdapter`: Applies marks using precomposed characters
/// - `UnicodeNormalizerAdapter`: Applies marks using Unicode NFD/NFC
///
/// # Examples
///
/// ```ignore
/// let transformer: Box<dyn MarkTransformer> = Box::new(MarkComposerAdapter::new());
///
/// let text = CharSequence::from("hoa");
/// let result = transformer.apply_mark(&text, MarkType::Circumflex, 1);
///
/// assert!(result.is_modified());
/// assert_eq!(result.text().as_str(), "hôa"); // 'o' → 'ô'
/// ```
pub trait MarkTransformer: Send + Sync {
    /// Applies diacritic mark to a character in text
    ///
    /// # Arguments
    ///
    /// - `text`: The text containing the vowel
    /// - `mark`: The mark type to apply
    /// - `position`: Character index where to apply mark (0-based)
    ///
    /// # Returns
    ///
    /// `TransformResult` with mark applied
    ///
    /// # Examples (conceptual)
    ///
    /// ```ignore
    /// apply_mark("hoa", Circumflex, 1) => "hôa"
    /// apply_mark("cua", Horn, 1)       => "cưa"
    /// apply_mark("can", Breve, 1)      => "căn"
    /// ```
    fn apply_mark(&self, text: &CharSequence, mark: MarkType, position: usize)
        -> TransformResult;

    /// Applies mark to a syllable's vowel
    ///
    /// # Arguments
    ///
    /// - `syllable`: The syllable to transform
    /// - `mark`: The mark to apply
    ///
    /// # Returns
    ///
    /// `TransformResult` with mark applied to main vowel
    ///
    /// # Default Implementation
    ///
    /// Finds main vowel in syllable and applies mark.
    fn apply_mark_to_syllable(&self, syllable: &Syllable, mark: MarkType) -> TransformResult {
        let vowel = syllable.vowel();

        // Find position to apply mark (first valid vowel)
        let position = vowel
            .chars()
            .position(|c| mark.can_apply_to(c))
            .unwrap_or(0);

        // Apply mark to the vowel part
        self.apply_mark(vowel, mark, position)
    }

    /// Removes mark from text at position
    ///
    /// # Arguments
    ///
    /// - `text`: The text containing the marked vowel
    /// - `position`: Character index of the marked vowel
    ///
    /// # Returns
    ///
    /// `TransformResult` with mark removed
    ///
    /// # Examples (conceptual)
    ///
    /// ```ignore
    /// remove_mark("hôa", 1) => "hoa"
    /// remove_mark("cưa", 1) => "cua"
    /// remove_mark("căn", 1) => "can"
    /// ```
    fn remove_mark(&self, text: &CharSequence, position: usize) -> TransformResult;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mark_type_combining_char() {
        assert_eq!(MarkType::Circumflex.combining_char(), '\u{0302}');
        assert_eq!(MarkType::Horn.combining_char(), '\u{031B}');
        assert_eq!(MarkType::Breve.combining_char(), '\u{0306}');
    }

    #[test]
    fn test_mark_type_apply_circumflex() {
        assert_eq!(MarkType::Circumflex.apply_to('a'), Some('â'));
        assert_eq!(MarkType::Circumflex.apply_to('A'), Some('â'));
        assert_eq!(MarkType::Circumflex.apply_to('e'), Some('ê'));
        assert_eq!(MarkType::Circumflex.apply_to('E'), Some('ê'));
        assert_eq!(MarkType::Circumflex.apply_to('o'), Some('ô'));
        assert_eq!(MarkType::Circumflex.apply_to('O'), Some('ô'));

        // Invalid
        assert_eq!(MarkType::Circumflex.apply_to('i'), None);
        assert_eq!(MarkType::Circumflex.apply_to('u'), None);
    }

    #[test]
    fn test_mark_type_apply_horn() {
        assert_eq!(MarkType::Horn.apply_to('o'), Some('ơ'));
        assert_eq!(MarkType::Horn.apply_to('O'), Some('ơ'));
        assert_eq!(MarkType::Horn.apply_to('u'), Some('ư'));
        assert_eq!(MarkType::Horn.apply_to('U'), Some('ư'));

        // Invalid
        assert_eq!(MarkType::Horn.apply_to('a'), None);
        assert_eq!(MarkType::Horn.apply_to('e'), None);
        assert_eq!(MarkType::Horn.apply_to('i'), None);
    }

    #[test]
    fn test_mark_type_apply_breve() {
        assert_eq!(MarkType::Breve.apply_to('a'), Some('ă'));
        assert_eq!(MarkType::Breve.apply_to('A'), Some('ă'));

        // Invalid
        assert_eq!(MarkType::Breve.apply_to('e'), None);
        assert_eq!(MarkType::Breve.apply_to('o'), None);
        assert_eq!(MarkType::Breve.apply_to('u'), None);
    }

    #[test]
    fn test_mark_type_can_apply_to() {
        // Circumflex
        assert!(MarkType::Circumflex.can_apply_to('a'));
        assert!(MarkType::Circumflex.can_apply_to('e'));
        assert!(MarkType::Circumflex.can_apply_to('o'));
        assert!(!MarkType::Circumflex.can_apply_to('i'));
        assert!(!MarkType::Circumflex.can_apply_to('u'));

        // Horn
        assert!(MarkType::Horn.can_apply_to('o'));
        assert!(MarkType::Horn.can_apply_to('u'));
        assert!(!MarkType::Horn.can_apply_to('a'));
        assert!(!MarkType::Horn.can_apply_to('e'));

        // Breve
        assert!(MarkType::Breve.can_apply_to('a'));
        assert!(!MarkType::Breve.can_apply_to('e'));
        assert!(!MarkType::Breve.can_apply_to('o'));
    }

    #[test]
    fn test_mark_type_variants() {
        let types = vec![MarkType::Circumflex, MarkType::Horn, MarkType::Breve];

        for (i, t1) in types.iter().enumerate() {
            for (j, t2) in types.iter().enumerate() {
                if i == j {
                    assert_eq!(t1, t2);
                } else {
                    assert_ne!(t1, t2);
                }
            }
        }
    }

    #[test]
    fn test_vietnamese_examples() {
        // "hoa" → "hôa" (apply circumflex to 'o')
        let result = MarkType::Circumflex.apply_to('o');
        assert_eq!(result, Some('ô'));

        // "cua" → "cưa" (apply horn to 'u')
        let result = MarkType::Horn.apply_to('u');
        assert_eq!(result, Some('ư'));

        // "can" → "căn" (apply breve to 'a')
        let result = MarkType::Breve.apply_to('a');
        assert_eq!(result, Some('ă'));
    }
}
