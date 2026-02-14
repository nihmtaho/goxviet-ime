//! Telex Input Method Adapter
//!
//! Concrete implementation of the `InputMethod` port for Telex input scheme.
//!
//! # Telex Key Mappings
//!
//! **Tone Marks:**
//! - `s` → Sắc (acute)
//! - `f` → Huyền (grave)
//! - `r` → Hỏi (hook above)
//! - `x` → Ngã (tilde)
//! - `j` → Nặng (dot below)
//! - `z` → Remove tone
//!
//! **Diacritic Modifiers:**
//! - `aa` → Circumflex (â)
//! - `ee` → Circumflex (ê)
//! - `oo` → Circumflex (ô)
//! - `aw` → Breve (ă) or Horn (ơ, ư)
//! - `ow` → Horn (ơ)
//! - `uw` → Horn (ư)
//! - `w` → Smart horn/breve (context-dependent)
//! - `dd` → Stroke (đ)
//!
//! # Design
//!
//! - **Stateless**: All methods are pure functions
//! - **Thread-Safe**: Implements `Send + Sync`
//! - **Performance**: Simple match statements, O(1) lookups
//!
//! # Examples
//!
//! ```
//! use goxviet_core::infrastructure::adapters::input::TelexAdapter;
//! use goxviet_core::domain::ports::input::{InputMethod, InputMethodId};
//! use goxviet_core::domain::entities::key_event::KeyEvent;
//!
//! let adapter = TelexAdapter::new();
//! assert_eq!(adapter.method_id(), InputMethodId::Telex);
//!
//! // Detect tone mark
//! let key_s = KeyEvent::simple('s' as u16);
//! assert!(adapter.detect_tone(&key_s).is_some());
//! ```

use crate::domain::{
    entities::{key_event::KeyEvent, tone::ToneType},
    ports::input::{DiacriticType, InputMethod, InputMethodId},
};

/// Telex input method adapter
///
/// Implements the Telex input scheme for Vietnamese.
/// This is an **adapter** in Clean Architecture - it implements
/// a domain port (interface) using concrete logic.
///
/// # Thread Safety
///
/// This struct is `Send + Sync` and can be safely shared across threads.
#[derive(Debug, Clone, Copy, Default)]
pub struct TelexAdapter;

impl TelexAdapter {
    /// Creates a new Telex adapter
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::infrastructure::adapters::input::TelexAdapter;
    /// let adapter = TelexAdapter::new();
    /// ```
    pub fn new() -> Self {
        Self
    }

    /// Checks if a character is a tone mark key
    fn is_tone_key(ch: char) -> bool {
        matches!(ch, 's' | 'S' | 'f' | 'F' | 'r' | 'R' | 'x' | 'X' | 'j' | 'J')
    }

    /// Checks if a character is a diacritic modifier key
    fn is_diacritic_key(ch: char) -> bool {
        matches!(ch, 'a' | 'A' | 'e' | 'E' | 'o' | 'O' | 'w' | 'W' | 'd' | 'D')
    }

    /// Checks if a character is the remove mark key
    fn is_remove_key(ch: char) -> bool {
        matches!(ch, 'z' | 'Z')
    }
}

impl InputMethod for TelexAdapter {
    fn method_id(&self) -> InputMethodId {
        InputMethodId::Telex
    }

    fn detect_tone(&self, event: &KeyEvent) -> Option<ToneType> {
        let ch = event.as_char()?;

        match ch.to_ascii_lowercase() {
            's' => Some(ToneType::Sac),
            'f' => Some(ToneType::Huyen),
            'r' => Some(ToneType::Hoi),
            'x' => Some(ToneType::Nga),
            'j' => Some(ToneType::Nang),
            _ => None,
        }
    }

    fn detect_diacritic(&self, event: &KeyEvent) -> Option<DiacriticType> {
        let ch = event.as_char()?;

        match ch.to_ascii_lowercase() {
            // Circumflex: aa→â, ee→ê, oo→ô
            'a' | 'e' | 'o' => Some(DiacriticType::Circumflex),
            // Horn/Breve: w is context-dependent
            // aw→ă (breve), ow→ơ, uw→ư (horn)
            'w' => Some(DiacriticType::Horn), // Caller determines breve vs horn from context
            // Stroke: dd→đ
            'd' => Some(DiacriticType::Stroke),
            _ => None,
        }
    }

    fn is_remove_mark(&self, event: &KeyEvent) -> bool {
        event
            .as_char()
            .map(|ch| Self::is_remove_key(ch))
            .unwrap_or(false)
    }

    fn is_modifier(&self, event: &KeyEvent) -> bool {
        if let Some(ch) = event.as_char() {
            Self::is_tone_key(ch) || Self::is_diacritic_key(ch) || Self::is_remove_key(ch)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_key_event(ch: char) -> KeyEvent {
        KeyEvent::simple(ch as u16)
    }

    #[test]
    fn test_adapter_creation() {
        let adapter = TelexAdapter::new();
        assert_eq!(adapter.method_id(), InputMethodId::Telex);
    }

    #[test]
    fn test_default_impl() {
        let adapter = TelexAdapter::default();
        assert_eq!(adapter.method_id(), InputMethodId::Telex);
    }

    #[test]
    fn test_detect_tone_sac() {
        let adapter = TelexAdapter::new();
        let event = create_key_event('s');
        assert_eq!(adapter.detect_tone(&event), Some(ToneType::Sac));
    }

    #[test]
    fn test_detect_tone_huyen() {
        let adapter = TelexAdapter::new();
        let event = create_key_event('f');
        assert_eq!(adapter.detect_tone(&event), Some(ToneType::Huyen));
    }

    #[test]
    fn test_detect_tone_hoi() {
        let adapter = TelexAdapter::new();
        let event = create_key_event('r');
        assert_eq!(adapter.detect_tone(&event), Some(ToneType::Hoi));
    }

    #[test]
    fn test_detect_tone_nga() {
        let adapter = TelexAdapter::new();
        let event = create_key_event('x');
        assert_eq!(adapter.detect_tone(&event), Some(ToneType::Nga));
    }

    #[test]
    fn test_detect_tone_nang() {
        let adapter = TelexAdapter::new();
        let event = create_key_event('j');
        assert_eq!(adapter.detect_tone(&event), Some(ToneType::Nang));
    }

    #[test]
    fn test_detect_tone_uppercase() {
        let adapter = TelexAdapter::new();
        assert_eq!(adapter.detect_tone(&create_key_event('S')), Some(ToneType::Sac));
        assert_eq!(adapter.detect_tone(&create_key_event('F')), Some(ToneType::Huyen));
        assert_eq!(adapter.detect_tone(&create_key_event('X')), Some(ToneType::Nga));
    }

    #[test]
    fn test_detect_tone_none() {
        let adapter = TelexAdapter::new();
        let event = create_key_event('a');
        assert_eq!(adapter.detect_tone(&event), None);
    }

    #[test]
    fn test_detect_diacritic_circumflex_a() {
        let adapter = TelexAdapter::new();
        let event = create_key_event('a');
        assert_eq!(adapter.detect_diacritic(&event), Some(DiacriticType::Circumflex));
    }

    #[test]
    fn test_detect_diacritic_circumflex_e() {
        let adapter = TelexAdapter::new();
        let event = create_key_event('e');
        assert_eq!(adapter.detect_diacritic(&event), Some(DiacriticType::Circumflex));
    }

    #[test]
    fn test_detect_diacritic_circumflex_o() {
        let adapter = TelexAdapter::new();
        let event = create_key_event('o');
        assert_eq!(adapter.detect_diacritic(&event), Some(DiacriticType::Circumflex));
    }

    #[test]
    fn test_detect_diacritic_horn_w() {
        let adapter = TelexAdapter::new();
        let event = create_key_event('w');
        // Note: 'w' returns Horn, but context determines if it's actually Breve (ă)
        assert_eq!(adapter.detect_diacritic(&event), Some(DiacriticType::Horn));
    }

    #[test]
    fn test_detect_diacritic_stroke_d() {
        let adapter = TelexAdapter::new();
        let event = create_key_event('d');
        assert_eq!(adapter.detect_diacritic(&event), Some(DiacriticType::Stroke));
    }

    #[test]
    fn test_detect_diacritic_uppercase() {
        let adapter = TelexAdapter::new();
        assert_eq!(adapter.detect_diacritic(&create_key_event('A')), Some(DiacriticType::Circumflex));
        assert_eq!(adapter.detect_diacritic(&create_key_event('W')), Some(DiacriticType::Horn));
        assert_eq!(adapter.detect_diacritic(&create_key_event('D')), Some(DiacriticType::Stroke));
    }

    #[test]
    fn test_detect_diacritic_none() {
        let adapter = TelexAdapter::new();
        let event = create_key_event('b');
        assert_eq!(adapter.detect_diacritic(&event), None);
    }

    #[test]
    fn test_is_remove_mark() {
        let adapter = TelexAdapter::new();
        assert!(adapter.is_remove_mark(&create_key_event('z')));
        assert!(adapter.is_remove_mark(&create_key_event('Z')));
        assert!(!adapter.is_remove_mark(&create_key_event('a')));
    }

    #[test]
    fn test_is_modifier_tone_keys() {
        let adapter = TelexAdapter::new();
        assert!(adapter.is_modifier(&create_key_event('s')));
        assert!(adapter.is_modifier(&create_key_event('f')));
        assert!(adapter.is_modifier(&create_key_event('r')));
        assert!(adapter.is_modifier(&create_key_event('x')));
        assert!(adapter.is_modifier(&create_key_event('j')));
    }

    #[test]
    fn test_is_modifier_diacritic_keys() {
        let adapter = TelexAdapter::new();
        assert!(adapter.is_modifier(&create_key_event('a')));
        assert!(adapter.is_modifier(&create_key_event('e')));
        assert!(adapter.is_modifier(&create_key_event('o')));
        assert!(adapter.is_modifier(&create_key_event('w')));
        assert!(adapter.is_modifier(&create_key_event('d')));
    }

    #[test]
    fn test_is_modifier_remove_key() {
        let adapter = TelexAdapter::new();
        assert!(adapter.is_modifier(&create_key_event('z')));
    }

    #[test]
    fn test_is_modifier_regular_keys() {
        let adapter = TelexAdapter::new();
        assert!(!adapter.is_modifier(&create_key_event('b')));
        assert!(!adapter.is_modifier(&create_key_event('c')));
        assert!(!adapter.is_modifier(&create_key_event('h')));
        assert!(!adapter.is_modifier(&create_key_event('i')));
        assert!(!adapter.is_modifier(&create_key_event('n')));
    }

    #[test]
    fn test_clone_and_copy() {
        let adapter1 = TelexAdapter::new();
        let adapter2 = adapter1; // Copy
        let adapter3 = adapter1.clone(); // Clone

        assert_eq!(adapter1.method_id(), adapter2.method_id());
        assert_eq!(adapter1.method_id(), adapter3.method_id());
    }

    #[test]
    fn test_debug_impl() {
        let adapter = TelexAdapter::new();
        let debug_str = format!("{:?}", adapter);
        assert!(debug_str.contains("TelexAdapter"));
    }
}
