//! Input Method Port (Interface)
//!
//! Defines the abstraction for Vietnamese input methods following SOLID principles.
//!
//! # Design Principles
//!
//! - **ISP (Interface Segregation)**: Small, focused trait with only essential methods
//! - **DIP (Dependency Inversion)**: Domain layer defines interface, infrastructure implements
//! - **OCP (Open/Closed)**: Add new input methods by implementing trait, no modification needed
//!
//! # Architecture
//!
//! ```text
//! Domain Layer (this file)
//!     ↓ defines interface
//! Infrastructure Layer
//!     ↓ implements
//! TelexAdapter, VniAdapter
//! ```

use crate::domain::entities::{key_event::KeyEvent, tone::ToneType};

/// Input method identifier
///
/// Represents different Vietnamese input method schemes.
///
/// # Variants
///
/// - `Telex`: Uses letter combinations (aa→â, aw→ă, s→sắc, f→huyền)
/// - `Vni`: Uses number keys (6→â, 8→ă, 1→sắc, 2→huyền)
/// - `Plain`: No transformations (direct input)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputMethodId {
    /// Telex input method (default)
    Telex,
    /// VNI input method
    Vni,
    /// Plain mode (no Vietnamese transformations)
    Plain,
}

impl Default for InputMethodId {
    fn default() -> Self {
        Self::Telex
    }
}

impl InputMethodId {
    /// Creates from numeric ID (for FFI compatibility)
    ///
    /// # Mapping
    ///
    /// - `0` → Telex
    /// - `1` → Vni
    /// - `2+` → Plain
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::domain::ports::input::InputMethodId;
    /// assert_eq!(InputMethodId::from_id(0), InputMethodId::Telex);
    /// assert_eq!(InputMethodId::from_id(1), InputMethodId::Vni);
    /// assert_eq!(InputMethodId::from_id(99), InputMethodId::Plain);
    /// ```
    pub fn from_id(id: u8) -> Self {
        match id {
            0 => Self::Telex,
            1 => Self::Vni,
            _ => Self::Plain,
        }
    }

    /// Converts to numeric ID (for FFI compatibility)
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::domain::ports::input::InputMethodId;
    /// assert_eq!(InputMethodId::Telex.to_id(), 0);
    /// assert_eq!(InputMethodId::Vni.to_id(), 1);
    /// assert_eq!(InputMethodId::Plain.to_id(), 2);
    /// ```
    pub fn to_id(&self) -> u8 {
        match self {
            Self::Telex => 0,
            Self::Vni => 1,
            Self::Plain => 2,
        }
    }

    /// Checks if this method performs Vietnamese transformations
    ///
    /// # Returns
    ///
    /// - `true` for Telex and VNI (they transform input)
    /// - `false` for Plain (no transformations)
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::domain::ports::input::InputMethodId;
    /// assert!(InputMethodId::Telex.supports_transforms());
    /// assert!(InputMethodId::Vni.supports_transforms());
    /// assert!(!InputMethodId::Plain.supports_transforms());
    /// ```
    pub fn supports_transforms(&self) -> bool {
        !matches!(self, Self::Plain)
    }

    /// Gets human-readable name
    ///
    /// # Examples
    ///
    /// ```
    /// # use goxviet_core::domain::ports::input::InputMethodId;
    /// assert_eq!(InputMethodId::Telex.name(), "Telex");
    /// assert_eq!(InputMethodId::Vni.name(), "VNI");
    /// assert_eq!(InputMethodId::Plain.name(), "Plain");
    /// ```
    pub fn name(&self) -> &'static str {
        match self {
            Self::Telex => "Telex",
            Self::Vni => "VNI",
            Self::Plain => "Plain",
        }
    }
}

/// Diacritic modifier types
///
/// Represents the types of diacritic modifications that can be applied
/// to Vietnamese vowels.
///
/// # Examples
///
/// - `Circumflex`: a→â, e→ê, o→ô
/// - `Horn`: o→ơ, u→ư
/// - `Breve`: a→ă
/// - `Stroke`: d→đ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiacriticType {
    /// Circumflex (ˆ): â, ê, ô
    Circumflex,
    /// Horn (ʼ): ơ, ư
    Horn,
    /// Breve (˘): ă
    Breve,
    /// Stroke (đ): đ
    Stroke,
}

/// Input method port (interface)
///
/// Defines the contract for Vietnamese input method implementations.
/// This is a **port** in Clean Architecture - domain layer defines the interface,
/// infrastructure layer provides implementations.
///
/// # Design
///
/// - **Small Interface**: Only 4 essential methods (ISP principle)
/// - **Thread-Safe**: Requires `Send + Sync` for concurrent use
/// - **Stateless**: Methods don't mutate self (functional approach)
///
/// # Implementations
///
/// - `TelexAdapter` (infrastructure layer)
/// - `VniAdapter` (infrastructure layer)
///
/// # Examples
///
/// ```ignore
/// // Application layer uses the trait
/// fn process(method: &dyn InputMethod, event: KeyEvent) {
///     if let Some(tone) = method.detect_tone(&event) {
///         // Apply tone...
///     }
/// }
/// ```
pub trait InputMethod: Send + Sync {
    /// Returns the method identifier
    ///
    /// # Returns
    ///
    /// The `InputMethodId` enum value identifying this implementation.
    fn method_id(&self) -> InputMethodId;

    /// Detects if key event represents a tone mark
    ///
    /// # Arguments
    ///
    /// - `event`: The keyboard event to analyze
    ///
    /// # Returns
    ///
    /// - `Some(ToneType)` if the event triggers a tone mark
    /// - `None` if not a tone mark
    ///
    /// # Examples (conceptual)
    ///
    /// ```ignore
    /// // Telex: 's' → Sắc, 'f' → Huyền
    /// // VNI: '1' → Sắc, '2' → Huyền
    /// ```
    fn detect_tone(&self, event: &KeyEvent) -> Option<ToneType>;

    /// Detects if key event represents a diacritic modifier
    ///
    /// # Arguments
    ///
    /// - `event`: The keyboard event to analyze
    ///
    /// # Returns
    ///
    /// - `Some(DiacriticType)` if the event triggers a diacritic
    /// - `None` if not a diacritic modifier
    ///
    /// # Examples (conceptual)
    ///
    /// ```ignore
    /// // Telex: 'aa' → Circumflex (â), 'aw' → Breve (ă), 'dd' → Stroke (đ)
    /// // VNI: '6' → Circumflex, '8' → Breve, '9' → Stroke
    /// ```
    fn detect_diacritic(&self, event: &KeyEvent) -> Option<DiacriticType>;

    /// Checks if key event removes diacritics
    ///
    /// # Arguments
    ///
    /// - `event`: The keyboard event to analyze
    ///
    /// # Returns
    ///
    /// - `true` if this event removes diacritics/tones
    /// - `false` otherwise
    ///
    /// # Examples (conceptual)
    ///
    /// ```ignore
    /// // Telex: 'z' removes all marks
    /// // VNI: '0' removes all marks
    /// ```
    fn is_remove_mark(&self, event: &KeyEvent) -> bool;

    /// Checks if key event is any kind of modifier
    ///
    /// # Arguments
    ///
    /// - `event`: The keyboard event to analyze
    ///
    /// # Returns
    ///
    /// - `true` if event is a tone, diacritic, or remove mark
    /// - `false` if it's a regular character input
    ///
    /// # Default Implementation
    ///
    /// Calls `detect_tone`, `detect_diacritic`, and `is_remove_mark`.
    /// Override if you have a more efficient implementation.
    fn is_modifier(&self, event: &KeyEvent) -> bool {
        self.detect_tone(event).is_some()
            || self.detect_diacritic(event).is_some()
            || self.is_remove_mark(event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_method_id_from_to_id() {
        assert_eq!(InputMethodId::from_id(0), InputMethodId::Telex);
        assert_eq!(InputMethodId::from_id(1), InputMethodId::Vni);
        assert_eq!(InputMethodId::from_id(2), InputMethodId::Plain);
        assert_eq!(InputMethodId::from_id(99), InputMethodId::Plain);

        assert_eq!(InputMethodId::Telex.to_id(), 0);
        assert_eq!(InputMethodId::Vni.to_id(), 1);
        assert_eq!(InputMethodId::Plain.to_id(), 2);
    }

    #[test]
    fn test_input_method_id_default() {
        let default = InputMethodId::default();
        assert_eq!(default, InputMethodId::Telex);
    }

    #[test]
    fn test_input_method_id_supports_transforms() {
        assert!(InputMethodId::Telex.supports_transforms());
        assert!(InputMethodId::Vni.supports_transforms());
        assert!(!InputMethodId::Plain.supports_transforms());
    }

    #[test]
    fn test_input_method_id_name() {
        assert_eq!(InputMethodId::Telex.name(), "Telex");
        assert_eq!(InputMethodId::Vni.name(), "VNI");
        assert_eq!(InputMethodId::Plain.name(), "Plain");
    }

    #[test]
    fn test_diacritic_type_variants() {
        let types = vec![
            DiacriticType::Circumflex,
            DiacriticType::Horn,
            DiacriticType::Breve,
            DiacriticType::Stroke,
        ];

        // Ensure all types are distinct
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
    fn test_diacritic_type_clone_copy() {
        let d1 = DiacriticType::Circumflex;
        let d2 = d1; // Copy
        let d3 = d1.clone(); // Clone

        assert_eq!(d1, d2);
        assert_eq!(d1, d3);
    }

    #[test]
    fn test_input_method_id_round_trip() {
        for id in [InputMethodId::Telex, InputMethodId::Vni, InputMethodId::Plain] {
            let numeric = id.to_id();
            let restored = InputMethodId::from_id(numeric);
            assert_eq!(id, restored);
        }
    }
}
