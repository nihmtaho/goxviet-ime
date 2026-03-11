//! InputMethodConfig — Data-Driven Input Method Definition (T6.1)
//!
//! Represents a fully data-driven key mapping for Vietnamese input methods.
//! Replaces hardcoded Telex/VNI logic; Swift defines these mappings and
//! passes them to Rust core via `ime_load_input_config_v2`.
//!
//! ## Design
//! - `InputAction` encodes every action a key can trigger in the IME.
//! - `InputMethodConfig` maps a `char` key to an `InputAction`.
//! - Built-ins `telex()` and `vni()` replicate the current hardcoded behavior.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// All possible actions a key can trigger in the Vietnamese IME.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InputAction {
    /// Add dấu sắc (acute accent): á, ắ, ấ, …
    ToneSac,
    /// Add dấu huyền (grave accent): à, ằ, ầ, …
    ToneHuyen,
    /// Add dấu hỏi (hook above): ả, ẳ, ẩ, …
    ToneHoi,
    /// Add dấu ngã (tilde): ã, ẵ, ẫ, …
    ToneNga,
    /// Add dấu nặng (dot below): ạ, ặ, ậ, …
    ToneNang,
    /// Xóa dấu — remove tone mark
    XoaDau,
    /// Circumflex on 'a': aa → â
    ModA,
    /// Circumflex on 'e': ee → ê
    ModE,
    /// Circumflex on 'o': oo → ô
    ModO,
    /// Breve on 'a': aw → ă
    ModAW,
    /// Horn on 'o': ow → ơ
    ModOW,
    /// Horn on 'u': uw/w → ư
    ModUW,
    /// Stroke 'd': dd → đ
    StrokeD,
    /// Smart ươ compound: uow → ươ
    CompoundUOA,
}

/// Data-driven Vietnamese input method configuration.
///
/// A flat `HashMap<char, InputAction>` fully specifies how each key behaves.
/// Swift passes a JSON-encoded config to `ime_load_input_config_v2`; Rust
/// engine adapts its processing to the received config.
///
/// ## Example — minimal Telex config
/// ```json
/// {
///   "name": "telex",
///   "mappings": {
///     "s": "tone_sac",
///     "f": "tone_huyen",
///     "aa": "mod_a"
///   }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InputMethodConfig {
    /// Human-readable name: "telex", "vni", or custom
    pub name: String,
    /// Key (as string, length 1 or 2 for digraphs) → InputAction
    pub mappings: HashMap<String, InputAction>,
}

impl InputMethodConfig {
    /// Built-in Telex configuration (mirrors existing `Telex` Method impl)
    ///
    /// Tone marks: s f r x j → sắc huyền hỏi ngã nặng
    /// Remove: z
    /// Modifiers: aa ee oo → â ê ô | aw ow uw w → ă ơ ư
    /// Stroke: dd → đ
    pub fn telex() -> Self {
        let mut m = HashMap::new();
        // Tone marks
        m.insert("s".into(), InputAction::ToneSac);
        m.insert("f".into(), InputAction::ToneHuyen);
        m.insert("r".into(), InputAction::ToneHoi);
        m.insert("x".into(), InputAction::ToneNga);
        m.insert("j".into(), InputAction::ToneNang);
        // Remove tone
        m.insert("z".into(), InputAction::XoaDau);
        // Circumflex modifiers
        m.insert("aa".into(), InputAction::ModA);
        m.insert("ee".into(), InputAction::ModE);
        m.insert("oo".into(), InputAction::ModO);
        // Horn/breve modifiers
        m.insert("aw".into(), InputAction::ModAW);
        m.insert("ow".into(), InputAction::ModOW);
        m.insert("uw".into(), InputAction::ModUW);
        m.insert("w".into(), InputAction::ModUW);
        // Stroke
        m.insert("dd".into(), InputAction::StrokeD);
        // Smart compound
        m.insert("uow".into(), InputAction::CompoundUOA);

        Self {
            name: "telex".into(),
            mappings: m,
        }
    }

    /// Built-in VNI configuration (mirrors existing `Vni` Method impl)
    ///
    /// Tone marks: 1 2 3 4 5 → sắc huyền hỏi ngã nặng
    /// Remove: 0
    /// Circumflex: 6 → â/ê/ô
    /// Horn: 7 → ơ/ư
    /// Breve: 8 → ă
    /// Stroke: 9 → đ
    pub fn vni() -> Self {
        let mut m = HashMap::new();
        // Tone marks
        m.insert("1".into(), InputAction::ToneSac);
        m.insert("2".into(), InputAction::ToneHuyen);
        m.insert("3".into(), InputAction::ToneHoi);
        m.insert("4".into(), InputAction::ToneNga);
        m.insert("5".into(), InputAction::ToneNang);
        // Remove tone
        m.insert("0".into(), InputAction::XoaDau);
        // Circumflex (applied to last a/e/o in buffer): 6
        m.insert("6".into(), InputAction::ModA); // â/ê/ô — engine decides target
                                                 // Horn: 7 → ơ / ư
        m.insert("7".into(), InputAction::ModOW); // engine resolves ơ vs ư context
                                                  // Breve: 8 → ă
        m.insert("8".into(), InputAction::ModAW);
        // Stroke: 9 → đ
        m.insert("9".into(), InputAction::StrokeD);

        Self {
            name: "vni".into(),
            mappings: m,
        }
    }

    /// Serialize to JSON string for FFI transfer
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Deserialize from JSON bytes (used in FFI endpoint)
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(bytes)
    }

    /// Returns the canonical input method name ("telex" or "vni")
    /// so the engine can select the corresponding Method implementation.
    pub fn method_name(&self) -> &str {
        &self.name
    }

    /// Returns the method_id compatible with `crate::shared::types::config::InputMethod`
    /// 0 = Telex, 1 = VNI
    pub fn method_id(&self) -> u8 {
        match self.name.to_lowercase().as_str() {
            "vni" => 1,
            _ => 0, // telex or custom → default to telex
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_telex_has_min_11_mappings() {
        let config = InputMethodConfig::telex();
        assert!(
            config.mappings.len() >= 11,
            "Telex should have ≥ 11 mappings, got {}",
            config.mappings.len()
        );
    }

    #[test]
    fn test_vni_has_min_10_mappings() {
        let config = InputMethodConfig::vni();
        assert!(
            config.mappings.len() >= 10,
            "VNI should have ≥ 10 mappings, got {}",
            config.mappings.len()
        );
    }

    #[test]
    fn test_telex_tone_mappings() {
        let config = InputMethodConfig::telex();
        assert_eq!(config.mappings.get("s"), Some(&InputAction::ToneSac));
        assert_eq!(config.mappings.get("f"), Some(&InputAction::ToneHuyen));
        assert_eq!(config.mappings.get("r"), Some(&InputAction::ToneHoi));
        assert_eq!(config.mappings.get("x"), Some(&InputAction::ToneNga));
        assert_eq!(config.mappings.get("j"), Some(&InputAction::ToneNang));
        assert_eq!(config.mappings.get("z"), Some(&InputAction::XoaDau));
    }

    #[test]
    fn test_telex_modifier_mappings() {
        let config = InputMethodConfig::telex();
        assert_eq!(config.mappings.get("aa"), Some(&InputAction::ModA));
        assert_eq!(config.mappings.get("ee"), Some(&InputAction::ModE));
        assert_eq!(config.mappings.get("oo"), Some(&InputAction::ModO));
        assert_eq!(config.mappings.get("aw"), Some(&InputAction::ModAW));
        assert_eq!(config.mappings.get("ow"), Some(&InputAction::ModOW));
        assert_eq!(config.mappings.get("uw"), Some(&InputAction::ModUW));
        assert_eq!(config.mappings.get("dd"), Some(&InputAction::StrokeD));
    }

    #[test]
    fn test_vni_tone_mappings() {
        let config = InputMethodConfig::vni();
        assert_eq!(config.mappings.get("1"), Some(&InputAction::ToneSac));
        assert_eq!(config.mappings.get("2"), Some(&InputAction::ToneHuyen));
        assert_eq!(config.mappings.get("3"), Some(&InputAction::ToneHoi));
        assert_eq!(config.mappings.get("4"), Some(&InputAction::ToneNga));
        assert_eq!(config.mappings.get("5"), Some(&InputAction::ToneNang));
        assert_eq!(config.mappings.get("0"), Some(&InputAction::XoaDau));
    }

    #[test]
    fn test_vni_modifier_mappings() {
        let config = InputMethodConfig::vni();
        assert_eq!(config.mappings.get("9"), Some(&InputAction::StrokeD));
    }

    #[test]
    fn test_telex_name() {
        assert_eq!(InputMethodConfig::telex().name, "telex");
        assert_eq!(InputMethodConfig::telex().method_id(), 0);
    }

    #[test]
    fn test_vni_name() {
        assert_eq!(InputMethodConfig::vni().name, "vni");
        assert_eq!(InputMethodConfig::vni().method_id(), 1);
    }

    #[test]
    fn test_json_roundtrip_telex() {
        let original = InputMethodConfig::telex();
        let json = original.to_json().expect("serialize");
        let restored = InputMethodConfig::from_json_bytes(json.as_bytes()).expect("deserialize");
        assert_eq!(original, restored);
    }

    #[test]
    fn test_json_roundtrip_vni() {
        let original = InputMethodConfig::vni();
        let json = original.to_json().expect("serialize");
        let restored = InputMethodConfig::from_json_bytes(json.as_bytes()).expect("deserialize");
        assert_eq!(original, restored);
    }

    #[test]
    fn test_custom_config_from_json() {
        let json = r#"{"name":"telex","mappings":{"s":"tone_sac","f":"tone_huyen"}}"#;
        let config = InputMethodConfig::from_json_bytes(json.as_bytes()).expect("deserialize");
        assert_eq!(config.name, "telex");
        assert_eq!(config.mappings.get("s"), Some(&InputAction::ToneSac));
    }

    #[test]
    fn test_invalid_json_returns_error() {
        let result = InputMethodConfig::from_json_bytes(b"not json");
        assert!(result.is_err());
    }
}
