//! VNI Input Method Adapter
//!
//! Concrete implementation of the `InputMethod` port for VNI input scheme.
//!
//! # VNI Key Mappings
//!
//! **Tone Marks:**
//! - `1` → Sắc (acute)
//! - `2` → Huyền (grave)
//! - `3` → Hỏi (hook above)
//! - `4` → Ngã (tilde)
//! - `5` → Nặng (dot below)
//! - `0` → Remove tone
//!
//! **Diacritic Modifiers:**
//! - `6` → Circumflex (â, ê, ô)
//! - `7` → Horn (ơ, ư)
//! - `8` → Breve (ă)
//! - `9` → Stroke (đ)
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
//! use goxviet_core::infrastructure::adapters::input::VniAdapter;
//! use goxviet_core::domain::ports::input::{InputMethod, InputMethodId};
//! use goxviet_core::domain::entities::key_event::KeyEvent;
//!
//! let adapter = VniAdapter::new();
//! assert_eq!(adapter.method_id(), InputMethodId::Vni);
//!
//! // Detect tone mark
//! let key_1 = KeyEvent::simple('1' as u16);
//! assert!(adapter.detect_tone(&key_1).is_some());
//! ```

use crate::domain::{
    entities::{key_event::KeyEvent, tone::ToneType},
    ports::input::{DiacriticType, InputMethod, InputMethodId},
};

/// VNI input method adapter
///
/// Implements the VNI input scheme for Vietnamese.
/// This is an **adapter** in Clean Architecture - it implements
/// a domain port (interface) using concrete logic.
///
/// # Thread Safety
///
/// This struct is `Send + Sync` and can be safely shared across threads.
#[derive(Debug, Clone, Copy, Default)]
pub struct VniAdapter;

impl VniAdapter {
    /// Creates a new VNI adapter
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::infrastructure::adapters::input::VniAdapter;
    /// let adapter = VniAdapter::new();
    /// ```
    pub fn new() -> Self {
        Self
    }

    /// Checks if a character is a tone mark key (1-5, 0)
    fn is_tone_key(ch: char) -> bool {
        matches!(ch, '0' | '1' | '2' | '3' | '4' | '5')
    }

    /// Checks if a character is a diacritic modifier key (6-9)
    fn is_diacritic_key(ch: char) -> bool {
        matches!(ch, '6' | '7' | '8' | '9')
    }
}

impl InputMethod for VniAdapter {
    fn method_id(&self) -> InputMethodId {
        InputMethodId::Vni
    }

    fn detect_tone(&self, event: &KeyEvent) -> Option<ToneType> {
        let ch = event.as_char()?;

        match ch {
            '1' => Some(ToneType::Sac),
            '2' => Some(ToneType::Huyen),
            '3' => Some(ToneType::Hoi),
            '4' => Some(ToneType::Nga),
            '5' => Some(ToneType::Nang),
            _ => None,
        }
    }

    fn detect_diacritic(&self, event: &KeyEvent) -> Option<DiacriticType> {
        let ch = event.as_char()?;

        match ch {
            '6' => Some(DiacriticType::Circumflex), // â, ê, ô
            '7' => Some(DiacriticType::Horn),       // ơ, ư
            '8' => Some(DiacriticType::Breve),      // ă
            '9' => Some(DiacriticType::Stroke),     // đ
            _ => None,
        }
    }

    fn is_remove_mark(&self, event: &KeyEvent) -> bool {
        event.as_char().map(|ch| ch == '0').unwrap_or(false)
    }

    fn is_modifier(&self, event: &KeyEvent) -> bool {
        if let Some(ch) = event.as_char() {
            Self::is_tone_key(ch) || Self::is_diacritic_key(ch)
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
        let adapter = VniAdapter::new();
        assert_eq!(adapter.method_id(), InputMethodId::Vni);
    }

    #[test]
    fn test_default_impl() {
        let adapter = VniAdapter::default();
        assert_eq!(adapter.method_id(), InputMethodId::Vni);
    }

    #[test]
    fn test_detect_tone_sac() {
        let adapter = VniAdapter::new();
        let event = create_key_event('1');
        assert_eq!(adapter.detect_tone(&event), Some(ToneType::Sac));
    }

    #[test]
    fn test_detect_tone_huyen() {
        let adapter = VniAdapter::new();
        let event = create_key_event('2');
        assert_eq!(adapter.detect_tone(&event), Some(ToneType::Huyen));
    }

    #[test]
    fn test_detect_tone_hoi() {
        let adapter = VniAdapter::new();
        let event = create_key_event('3');
        assert_eq!(adapter.detect_tone(&event), Some(ToneType::Hoi));
    }

    #[test]
    fn test_detect_tone_nga() {
        let adapter = VniAdapter::new();
        let event = create_key_event('4');
        assert_eq!(adapter.detect_tone(&event), Some(ToneType::Nga));
    }

    #[test]
    fn test_detect_tone_nang() {
        let adapter = VniAdapter::new();
        let event = create_key_event('5');
        assert_eq!(adapter.detect_tone(&event), Some(ToneType::Nang));
    }

    #[test]
    fn test_detect_tone_none() {
        let adapter = VniAdapter::new();
        let event = create_key_event('a');
        assert_eq!(adapter.detect_tone(&event), None);
    }

    #[test]
    fn test_detect_diacritic_circumflex() {
        let adapter = VniAdapter::new();
        let event = create_key_event('6');
        assert_eq!(adapter.detect_diacritic(&event), Some(DiacriticType::Circumflex));
    }

    #[test]
    fn test_detect_diacritic_horn() {
        let adapter = VniAdapter::new();
        let event = create_key_event('7');
        assert_eq!(adapter.detect_diacritic(&event), Some(DiacriticType::Horn));
    }

    #[test]
    fn test_detect_diacritic_breve() {
        let adapter = VniAdapter::new();
        let event = create_key_event('8');
        assert_eq!(adapter.detect_diacritic(&event), Some(DiacriticType::Breve));
    }

    #[test]
    fn test_detect_diacritic_stroke() {
        let adapter = VniAdapter::new();
        let event = create_key_event('9');
        assert_eq!(adapter.detect_diacritic(&event), Some(DiacriticType::Stroke));
    }

    #[test]
    fn test_detect_diacritic_none() {
        let adapter = VniAdapter::new();
        let event = create_key_event('a');
        assert_eq!(adapter.detect_diacritic(&event), None);
    }

    #[test]
    fn test_is_remove_mark() {
        let adapter = VniAdapter::new();
        assert!(adapter.is_remove_mark(&create_key_event('0')));
        assert!(!adapter.is_remove_mark(&create_key_event('1')));
        assert!(!adapter.is_remove_mark(&create_key_event('a')));
    }

    #[test]
    fn test_is_modifier_tone_keys() {
        let adapter = VniAdapter::new();
        assert!(adapter.is_modifier(&create_key_event('0'))); // Remove
        assert!(adapter.is_modifier(&create_key_event('1')));
        assert!(adapter.is_modifier(&create_key_event('2')));
        assert!(adapter.is_modifier(&create_key_event('3')));
        assert!(adapter.is_modifier(&create_key_event('4')));
        assert!(adapter.is_modifier(&create_key_event('5')));
    }

    #[test]
    fn test_is_modifier_diacritic_keys() {
        let adapter = VniAdapter::new();
        assert!(adapter.is_modifier(&create_key_event('6')));
        assert!(adapter.is_modifier(&create_key_event('7')));
        assert!(adapter.is_modifier(&create_key_event('8')));
        assert!(adapter.is_modifier(&create_key_event('9')));
    }

    #[test]
    fn test_is_modifier_regular_keys() {
        let adapter = VniAdapter::new();
        assert!(!adapter.is_modifier(&create_key_event('a')));
        assert!(!adapter.is_modifier(&create_key_event('b')));
        assert!(!adapter.is_modifier(&create_key_event('h')));
    }

    #[test]
    fn test_clone_and_copy() {
        let adapter1 = VniAdapter::new();
        let adapter2 = adapter1; // Copy
        let adapter3 = adapter1.clone(); // Clone

        assert_eq!(adapter1.method_id(), adapter2.method_id());
        assert_eq!(adapter1.method_id(), adapter3.method_id());
    }

    #[test]
    fn test_debug_impl() {
        let adapter = VniAdapter::new();
        let debug_str = format!("{:?}", adapter);
        assert!(debug_str.contains("VniAdapter"));
    }
}
