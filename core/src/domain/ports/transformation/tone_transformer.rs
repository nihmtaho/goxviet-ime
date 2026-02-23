//! Tone Transformer Port
//!
//! Defines the abstraction for applying Vietnamese tone marks to syllables.
//!
//! # Design Principles
//!
//! - **ISP**: Small, focused interface with 2 main methods
//! - **DIP**: Domain defines interface, infrastructure implements
//! - **SRP**: Only transforms tones, not other diacritics
//!
//! # Architecture
//!
//! ```text
//! Domain Layer (this file)
//!     ↓ defines interface
//! Infrastructure Layer
//!     ↓ implements
//! TonePositioningAdapter, UnicodeComposerAdapter
//! ```

use crate::domain::{
    entities::{syllable::Syllable, tone::ToneType},
    value_objects::transformation::TransformResult,
};

/// Tone application strategy
///
/// Defines how tone marks should be applied to syllables.
///
/// # Strategies
///
/// - `Modern`: Modern style (tone on main vowel) - e.g., "hoà", "thuỷ"
/// - `Traditional`: Traditional style (tone on first vowel) - e.g., "hòa", "thùy"
/// - `Auto`: Automatically choose based on context
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToneStrategy {
    /// Modern style (follows Bộ GD&ĐT 2018 rules)
    Modern,
    /// Traditional style (pre-2018)
    Traditional,
    /// Auto-detect based on context
    Auto,
}

impl Default for ToneStrategy {
    fn default() -> Self {
        Self::Modern
    }
}

/// Tone transformer port (interface)
///
/// Transforms syllables by applying tone marks according to Vietnamese rules.
///
/// # Tone Positioning Rules
///
/// Vietnamese tone marks are placed on the **main vowel** of the syllable:
///
/// 1. **Single vowel**: Always on that vowel (a, e, i, o, u)
/// 2. **Double vowel**: Depends on vowel group:
///    - oa, oe, uy: Second vowel (Modern: hoà, Traditional: hòa)
///    - iê, uô, ươ: Second vowel (always ê, ô, ơ)
///    - ai, ao, au: Second vowel
/// 3. **Triple vowel**: Middle vowel (iêu → iếu, oai → oái, ươi → ươi)
///
/// # Implementations
///
/// - `TonePositioningAdapter`: Rule-based tone positioning
/// - `UnicodeComposerAdapter`: Unicode NFC/NFD composition
///
/// # Examples
///
/// ```ignore
/// let transformer: Box<dyn ToneTransformer> = Box::new(TonePositioningAdapter::new());
///
/// let syllable = Syllable::from_parts("h", "oa", "", ToneType::Ngang);
/// let result = transformer.apply_tone(&syllable, ToneType::Huyen);
///
/// assert!(result.is_modified());
/// assert_eq!(result.text().as_str(), "hoà"); // Modern style
/// ```
pub trait ToneTransformer: Send + Sync {
    /// Applies tone mark to a syllable
    ///
    /// # Arguments
    ///
    /// - `syllable`: The syllable to transform
    /// - `tone`: The tone to apply
    ///
    /// # Returns
    ///
    /// `TransformResult` containing the transformed text
    ///
    /// # Examples (conceptual)
    ///
    /// ```ignore
    /// // Single vowel
    /// apply_tone("ha", Sắc) => "há"
    ///
    /// // Double vowel
    /// apply_tone("hoa", Huyền) => "hoà" (Modern) or "hòa" (Traditional)
    ///
    /// // Triple vowel
    /// apply_tone("khuya", Hỏi) => "khuyả"
    /// ```
    fn apply_tone(&self, syllable: &Syllable, tone: ToneType) -> TransformResult;

    /// Removes tone mark from a syllable
    ///
    /// # Arguments
    ///
    /// - `syllable`: The syllable to transform
    ///
    /// # Returns
    ///
    /// `TransformResult` with tone removed (Ngang tone)
    ///
    /// # Examples (conceptual)
    ///
    /// ```ignore
    /// remove_tone("há") => "ha"
    /// remove_tone("hoà") => "hoa"
    /// remove_tone("khuyả") => "khuya"
    /// ```
    fn remove_tone(&self, syllable: &Syllable) -> TransformResult;

    /// Gets the tone strategy used by this transformer
    ///
    /// # Returns
    ///
    /// The `ToneStrategy` (Modern, Traditional, or Auto)
    fn strategy(&self) -> ToneStrategy {
        ToneStrategy::default()
    }
}

/// Quick tone transformation helpers
///
/// Utility functions for common tone operations.
pub mod quick {
    

    /// Finds main vowel position in vowel cluster
    ///
    /// Returns the index of the vowel that should receive the tone mark.
    ///
    /// # Rules
    ///
    /// - Single vowel: index 0
    /// - Double vowel with â/ê/ô/ơ/ư: That vowel's index
    /// - Otherwise: Second vowel
    ///
    /// # Arguments
    ///
    /// - `vowel`: The vowel cluster (e.g., "oa", "iê", "ươ")
    ///
    /// # Returns
    ///
    /// Character index (0-based) of main vowel, or 0 if empty
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::domain::ports::transformation::tone_transformer::quick;
    /// assert_eq!(quick::find_main_vowel_index("a"), 0);
    /// assert_eq!(quick::find_main_vowel_index("oa"), 1); // 'a'
    /// assert_eq!(quick::find_main_vowel_index("iê"), 1); // 'ê'
    /// assert_eq!(quick::find_main_vowel_index("ươ"), 1); // 'ơ'
    /// assert_eq!(quick::find_main_vowel_index("oai"), 1); // 'a' (middle)
    /// ```
    pub fn find_main_vowel_index(vowel: &str) -> usize {
        if vowel.is_empty() {
            return 0;
        }

        let chars: Vec<char> = vowel.chars().collect();

        // Single vowel
        if chars.len() == 1 {
            return 0;
        }

        // Special case: "ươ" → tone on 'ơ' (second vowel)
        if vowel == "ươ" || vowel.starts_with("ươ") {
            return 1;
        }

        // Look for special vowels (â, ê, ô, ơ, ư)
        for (i, &c) in chars.iter().enumerate() {
            if matches!(c, 'â' | 'ê' | 'ô' | 'ơ' | 'ư') {
                return i;
            }
        }

        // Default: second vowel for double, middle for triple
        if chars.len() == 2 {
            1
        } else {
            1 // Middle vowel for triple
        }
    }

    /// Checks if vowel cluster has special positioning rules
    ///
    /// # Arguments
    ///
    /// - `vowel`: The vowel cluster
    ///
    /// # Returns
    ///
    /// `true` if this cluster has non-standard rules
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::domain::ports::transformation::tone_transformer::quick;
    /// assert!(quick::has_special_rules("iê"));
    /// assert!(quick::has_special_rules("uô"));
    /// assert!(quick::has_special_rules("ươ"));
    /// assert!(!quick::has_special_rules("ai"));
    /// assert!(!quick::has_special_rules("oa"));
    /// ```
    pub fn has_special_rules(vowel: &str) -> bool {
        matches!(vowel, "iê" | "yê" | "uô" | "ươ" | "ưu")
    }
}

#[cfg(test)]
mod tests {
    use super::quick::*;
    use super::*;

    #[test]
    fn test_tone_strategy_default() {
        assert_eq!(ToneStrategy::default(), ToneStrategy::Modern);
    }

    #[test]
    fn test_tone_strategy_variants() {
        let strategies = vec![
            ToneStrategy::Modern,
            ToneStrategy::Traditional,
            ToneStrategy::Auto,
        ];

        for (i, s1) in strategies.iter().enumerate() {
            for (j, s2) in strategies.iter().enumerate() {
                if i == j {
                    assert_eq!(s1, s2);
                } else {
                    assert_ne!(s1, s2);
                }
            }
        }
    }

    #[test]
    fn test_find_main_vowel_index_single() {
        assert_eq!(find_main_vowel_index("a"), 0);
        assert_eq!(find_main_vowel_index("e"), 0);
        assert_eq!(find_main_vowel_index("i"), 0);
        assert_eq!(find_main_vowel_index("o"), 0);
        assert_eq!(find_main_vowel_index("u"), 0);
    }

    #[test]
    fn test_find_main_vowel_index_double_special() {
        // Special vowels get the tone
        assert_eq!(find_main_vowel_index("iê"), 1); // ê
        assert_eq!(find_main_vowel_index("uô"), 1); // ô
        assert_eq!(find_main_vowel_index("ươ"), 1); // ơ
        assert_eq!(find_main_vowel_index("âu"), 0); // â
    }

    #[test]
    fn test_find_main_vowel_index_double_regular() {
        // Regular: second vowel
        assert_eq!(find_main_vowel_index("oa"), 1);
        assert_eq!(find_main_vowel_index("ai"), 1);
        assert_eq!(find_main_vowel_index("au"), 1);
        assert_eq!(find_main_vowel_index("oi"), 1);
        assert_eq!(find_main_vowel_index("ui"), 1);
    }

    #[test]
    fn test_find_main_vowel_index_triple() {
        // Triple: middle vowel
        assert_eq!(find_main_vowel_index("oai"), 1); // 'a'
        assert_eq!(find_main_vowel_index("iêu"), 1); // 'ê'
        assert_eq!(find_main_vowel_index("ươi"), 1); // 'ơ'
        assert_eq!(find_main_vowel_index("uôi"), 1); // 'ô'
    }

    #[test]
    fn test_find_main_vowel_index_empty() {
        assert_eq!(find_main_vowel_index(""), 0);
    }

    #[test]
    fn test_has_special_rules() {
        // Special clusters
        assert!(has_special_rules("iê"));
        assert!(has_special_rules("yê"));
        assert!(has_special_rules("uô"));
        assert!(has_special_rules("ươ"));
        assert!(has_special_rules("ưu"));

        // Regular clusters
        assert!(!has_special_rules("ai"));
        assert!(!has_special_rules("oa"));
        assert!(!has_special_rules("oi"));
        assert!(!has_special_rules("ui"));
        assert!(!has_special_rules("au"));
    }

    #[test]
    fn test_vietnamese_examples_main_vowel() {
        // "trường" → vowel "ươ" → main is 'ơ' at index 1
        assert_eq!(find_main_vowel_index("ươ"), 1);

        // "tiếng" → vowel "iê" → main is 'ê' at index 1
        assert_eq!(find_main_vowel_index("iê"), 1);

        // "hoà" → vowel "oa" → main is 'a' at index 1 (Modern)
        assert_eq!(find_main_vowel_index("oa"), 1);

        // "khuya" → vowel "uya" → main is 'y' at index 1
        assert_eq!(find_main_vowel_index("uya"), 1);
    }
}
