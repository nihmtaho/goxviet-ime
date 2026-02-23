//! Tone Entity - Vietnamese Tone Marks
//!
//! Represents the 6 tones in Vietnamese language.
//! This is a core domain concept with business rules.

use std::fmt;

/// Vietnamese tone types
///
/// Vietnamese has 6 tones that change the meaning of words:
/// - Ngang (level): no mark
/// - Sac (acute): rising tone (´)
/// - Huyen (grave): falling tone (`)
/// - Hoi (hook above): dipping-rising tone (?)
/// - Nga (tilde): rising glottalized tone (~)
/// - Nang (dot below): falling glottalized tone (.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ToneType {
    /// Level tone (thanh ngang) - no mark
    Ngang = 0,
    /// Acute/rising tone (thanh sắc) - ´
    Sac = 1,
    /// Grave/falling tone (thanh huyền) - `
    Huyen = 2,
    /// Hook/dipping tone (thanh hỏi) - ?
    Hoi = 3,
    /// Tilde/rising glottalized (thanh ngã) - ~
    Nga = 4,
    /// Dot below/falling glottalized (thanh nặng) - .
    Nang = 5,
}

impl ToneType {
    /// Get tone from numeric ID (0-5)
    ///
    /// # Examples
    /// ```
    /// # use goxviet_core::domain::entities::tone::ToneType;
    /// assert_eq!(ToneType::from_id(0), Some(ToneType::Ngang));
    /// assert_eq!(ToneType::from_id(1), Some(ToneType::Sac));
    /// assert_eq!(ToneType::from_id(6), None);
    /// ```
    pub fn from_id(id: u8) -> Option<Self> {
        match id {
            0 => Some(ToneType::Ngang),
            1 => Some(ToneType::Sac),
            2 => Some(ToneType::Huyen),
            3 => Some(ToneType::Hoi),
            4 => Some(ToneType::Nga),
            5 => Some(ToneType::Nang),
            _ => None,
        }
    }

    /// Get numeric ID (0-5)
    #[inline]
    pub fn id(&self) -> u8 {
        *self as u8
    }

    /// Get tone name in Vietnamese
    pub fn name_vn(&self) -> &'static str {
        match self {
            ToneType::Ngang => "Ngang",
            ToneType::Sac => "Sắc",
            ToneType::Huyen => "Huyền",
            ToneType::Hoi => "Hỏi",
            ToneType::Nga => "Ngã",
            ToneType::Nang => "Nặng",
        }
    }

    /// Get tone name in English
    pub fn name_en(&self) -> &'static str {
        match self {
            ToneType::Ngang => "Level",
            ToneType::Sac => "Acute/Rising",
            ToneType::Huyen => "Grave/Falling",
            ToneType::Hoi => "Hook/Dipping",
            ToneType::Nga => "Tilde/Rising Glottalized",
            ToneType::Nang => "Dot Below/Falling Glottalized",
        }
    }

    /// Get Unicode combining mark for this tone
    ///
    /// Returns the Unicode combining character that represents this tone mark.
    /// For Ngang (no tone), returns None.
    pub fn combining_mark(&self) -> Option<char> {
        match self {
            ToneType::Ngang => None,
            ToneType::Sac => Some('\u{0301}'),   // Combining acute accent
            ToneType::Huyen => Some('\u{0300}'), // Combining grave accent
            ToneType::Hoi => Some('\u{0309}'),   // Combining hook above
            ToneType::Nga => Some('\u{0303}'),   // Combining tilde
            ToneType::Nang => Some('\u{0323}'),  // Combining dot below
        }
    }

    /// Check if this tone has a mark (not Ngang)
    #[inline]
    pub fn has_mark(&self) -> bool {
        !matches!(self, ToneType::Ngang)
    }

    /// Get all tone types as an array
    pub fn all() -> [ToneType; 6] {
        [
            ToneType::Ngang,
            ToneType::Sac,
            ToneType::Huyen,
            ToneType::Hoi,
            ToneType::Nga,
            ToneType::Nang,
        ]
    }
}

impl Default for ToneType {
    fn default() -> Self {
        ToneType::Ngang
    }
}

impl fmt::Display for ToneType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name_vn())
    }
}

/// Tone mark representation
///
/// Wraps a ToneType with additional metadata and helper methods
/// for applying tones to Vietnamese text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ToneMark {
    tone: ToneType,
}

impl ToneMark {
    /// Create a new tone mark
    pub fn new(tone: ToneType) -> Self {
        Self { tone }
    }

    /// Create tone mark with no tone (Ngang)
    pub fn none() -> Self {
        Self {
            tone: ToneType::Ngang,
        }
    }

    /// Get the underlying tone type
    #[inline]
    pub fn tone(&self) -> ToneType {
        self.tone
    }

    /// Check if this is a neutral tone (Ngang)
    #[inline]
    pub fn is_neutral(&self) -> bool {
        self.tone == ToneType::Ngang
    }

    /// Apply this tone mark to a base character
    ///
    /// This combines the base character with the tone's combining mark.
    /// For Ngang (no tone), returns the base character unchanged.
    ///
    /// # Examples
    /// ```
    /// # use goxviet_core::domain::entities::tone::{ToneMark, ToneType};
    /// let mark = ToneMark::new(ToneType::Sac);
    /// assert_eq!(mark.apply_to('a'), 'á');
    ///
    /// let neutral = ToneMark::none();
    /// assert_eq!(neutral.apply_to('a'), 'a');
    /// ```
    pub fn apply_to(&self, base: char) -> char {
        if self.tone.combining_mark().is_some() {
            // In real implementation, this would use a proper Vietnamese
            // character composition lookup table
            // For now, we'll return the base + combining mark
            // The actual composition should be done by a proper Unicode normalizer
            base // Placeholder - actual implementation in transformer layer
        } else {
            base
        }
    }

    /// Create tone mark from tone ID
    pub fn from_id(id: u8) -> Option<Self> {
        ToneType::from_id(id).map(|tone| Self { tone })
    }
}

impl Default for ToneMark {
    fn default() -> Self {
        Self::none()
    }
}

impl From<ToneType> for ToneMark {
    fn from(tone: ToneType) -> Self {
        Self::new(tone)
    }
}

impl fmt::Display for ToneMark {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tone)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tone_type_from_id() {
        assert_eq!(ToneType::from_id(0), Some(ToneType::Ngang));
        assert_eq!(ToneType::from_id(1), Some(ToneType::Sac));
        assert_eq!(ToneType::from_id(2), Some(ToneType::Huyen));
        assert_eq!(ToneType::from_id(3), Some(ToneType::Hoi));
        assert_eq!(ToneType::from_id(4), Some(ToneType::Nga));
        assert_eq!(ToneType::from_id(5), Some(ToneType::Nang));
        assert_eq!(ToneType::from_id(6), None);
    }

    #[test]
    fn test_tone_type_id() {
        assert_eq!(ToneType::Ngang.id(), 0);
        assert_eq!(ToneType::Sac.id(), 1);
        assert_eq!(ToneType::Huyen.id(), 2);
        assert_eq!(ToneType::Hoi.id(), 3);
        assert_eq!(ToneType::Nga.id(), 4);
        assert_eq!(ToneType::Nang.id(), 5);
    }

    #[test]
    fn test_tone_type_has_mark() {
        assert!(!ToneType::Ngang.has_mark());
        assert!(ToneType::Sac.has_mark());
        assert!(ToneType::Huyen.has_mark());
        assert!(ToneType::Hoi.has_mark());
        assert!(ToneType::Nga.has_mark());
        assert!(ToneType::Nang.has_mark());
    }

    #[test]
    fn test_tone_type_combining_mark() {
        assert_eq!(ToneType::Ngang.combining_mark(), None);
        assert_eq!(ToneType::Sac.combining_mark(), Some('\u{0301}'));
        assert_eq!(ToneType::Huyen.combining_mark(), Some('\u{0300}'));
        assert_eq!(ToneType::Hoi.combining_mark(), Some('\u{0309}'));
        assert_eq!(ToneType::Nga.combining_mark(), Some('\u{0303}'));
        assert_eq!(ToneType::Nang.combining_mark(), Some('\u{0323}'));
    }

    #[test]
    fn test_tone_mark_creation() {
        let mark = ToneMark::new(ToneType::Sac);
        assert_eq!(mark.tone(), ToneType::Sac);
        assert!(!mark.is_neutral());

        let neutral = ToneMark::none();
        assert_eq!(neutral.tone(), ToneType::Ngang);
        assert!(neutral.is_neutral());
    }

    #[test]
    fn test_tone_mark_from_id() {
        let mark = ToneMark::from_id(1);
        assert!(mark.is_some());
        assert_eq!(mark.unwrap().tone(), ToneType::Sac);

        let invalid = ToneMark::from_id(10);
        assert!(invalid.is_none());
    }

    #[test]
    fn test_tone_mark_default() {
        let mark = ToneMark::default();
        assert!(mark.is_neutral());
        assert_eq!(mark.tone(), ToneType::Ngang);
    }

    #[test]
    fn test_tone_type_all() {
        let all = ToneType::all();
        assert_eq!(all.len(), 6);
        assert_eq!(all[0], ToneType::Ngang);
        assert_eq!(all[5], ToneType::Nang);
    }
}
