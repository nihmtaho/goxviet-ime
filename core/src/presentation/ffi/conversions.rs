//! FFI Type Conversions
//!
//! Safe conversions between Rust types and C FFI types.

use super::types::*;
use crate::application::dto::EngineConfig;
use crate::domain::ports::input::InputMethodId;
use crate::domain::value_objects::transformation::TransformResult;
use std::ffi::{CStr, CString};

// Define TonePlacementStyle locally if not in EngineConfig
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TonePlacementStyle {
    Old,
    New,
}

/// Convert Rust string to FFI string (UTF-8, null-terminated)
///
/// # Safety
/// Caller must free the returned pointer using `ime_free_string()`
pub fn to_ffi_string(s: &str) -> FfiString {
    match CString::new(s) {
        Ok(c_str) => c_str.into_raw(),
        Err(_) => std::ptr::null_mut(), // String contains null byte
    }
}

/// Convert FFI string to Rust String
///
/// # Safety
/// - `ptr` must be valid UTF-8 null-terminated string
/// - `ptr` must not be null
pub unsafe fn from_ffi_string(ptr: FfiConstString) -> Result<String, &'static str> {
    if ptr.is_null() {
        return Err("Null pointer");
    }

    match CStr::from_ptr(ptr).to_str() {
        Ok(s) => Ok(s.to_owned()),
        Err(_) => Err("Invalid UTF-8"),
    }
}

/// Free FFI string
///
/// # Safety
/// - `ptr` must have been created by `to_ffi_string()`
/// - Must only be called once per pointer
pub unsafe fn free_ffi_string(ptr: FfiString) {
    if !ptr.is_null() {
        drop(CString::from_raw(ptr));
    }
}

/// Convert FfiInputMethod to InputMethodId
pub fn to_input_method_id(method: FfiInputMethod) -> InputMethodId {
    match method {
        FfiInputMethod::Telex => InputMethodId::Telex,
        FfiInputMethod::Vni => InputMethodId::Vni,
    }
}

/// Convert InputMethodId to FfiInputMethod
pub fn from_input_method_id(id: InputMethodId) -> FfiInputMethod {
    match id {
        InputMethodId::Telex => FfiInputMethod::Telex,
        InputMethodId::Vni => FfiInputMethod::Vni,
        InputMethodId::Plain => FfiInputMethod::Telex, // Fallback
    }
}

/// Convert FfiToneStyle to TonePlacementStyle
pub fn to_tone_style(style: FfiToneStyle) -> TonePlacementStyle {
    match style {
        FfiToneStyle::Old => TonePlacementStyle::Old,
        FfiToneStyle::New => TonePlacementStyle::New,
    }
}

/// Convert TonePlacementStyle to FfiToneStyle
pub fn from_tone_style(style: TonePlacementStyle) -> FfiToneStyle {
    match style {
        TonePlacementStyle::Old => FfiToneStyle::Old,
        TonePlacementStyle::New => FfiToneStyle::New,
    }
}

/// Convert FfiConfig to EngineConfig
pub fn to_engine_config(config: FfiConfig) -> EngineConfig {
    EngineConfig {
        input_method: to_input_method_id(config.input_method),
        tone_strategy: crate::domain::ports::transformation::ToneStrategy::default(),
        enabled: true,
        smart_mode: config.smart_mode,
        spell_check: true,
        auto_correct: false,
        max_history_size: 100,
        buffer_timeout_ms: 1000,
        use_modern_tone_placement: matches!(config.tone_style, FfiToneStyle::New),
        enable_shortcuts: config.enable_shortcuts,
        instant_restore_enabled: true,
        esc_restore_enabled: false,
    }
}

/// Convert EngineConfig to FfiConfig
pub fn from_engine_config(config: &EngineConfig) -> FfiConfig {
    FfiConfig {
        input_method: from_input_method_id(config.input_method),
        tone_style: if config.use_modern_tone_placement {
            FfiToneStyle::New
        } else {
            FfiToneStyle::Old
        },
        smart_mode: config.smart_mode,
        enable_shortcuts: config.enable_shortcuts,
    }
}

/// Convert FfiConfig_v2 to EngineConfig
pub fn to_engine_config_v2(config: &FfiConfig_v2) -> EngineConfig {
    EngineConfig {
        input_method: to_input_method_id(config.input_method),
        tone_strategy: crate::domain::ports::transformation::ToneStrategy::default(),
        enabled: true,
        smart_mode: config.smart_mode,
        spell_check: true,
        auto_correct: false,
        max_history_size: 100,
        buffer_timeout_ms: 1000,
        use_modern_tone_placement: matches!(config.tone_style, FfiToneStyle::New),
        enable_shortcuts: config.enable_shortcuts,
        instant_restore_enabled: config.instant_restore_enabled,
        esc_restore_enabled: config.esc_restore_enabled,
    }
}

/// Convert EngineConfig to FfiConfig_v2
pub fn from_engine_config_v2(config: &EngineConfig) -> FfiConfig_v2 {
    FfiConfig_v2 {
        input_method: from_input_method_id(config.input_method),
        tone_style: if config.use_modern_tone_placement {
            FfiToneStyle::New
        } else {
            FfiToneStyle::Old
        },
        smart_mode: config.smart_mode,
        instant_restore_enabled: config.instant_restore_enabled,
        esc_restore_enabled: config.esc_restore_enabled,
        enable_shortcuts: config.enable_shortcuts,
    }
}

// ============================================================
// v2 ONLY - v1 to_ffi_process_result removed in v3.0.0
// ============================================================
// Use to_ffi_process_result_v2 instead

/// Convert TransformResult to FfiProcessResult_v2 (v2 API)
///
/// # Safety
/// Caller must free the `text` field using `ime_free_string_v2()`
pub fn to_ffi_process_result_v2(result: TransformResult) -> FfiProcessResult_v2 {
    let text_seq = result.new_text();
    let backspace_count = result.backspace_count();

    if text_seq.is_empty() && backspace_count == 0 {
        FfiProcessResult_v2 {
            text: std::ptr::null_mut(),
            backspace_count: 0,
            consumed: false,
        }
    } else {
        let ffi_str = to_ffi_string(text_seq.as_str());
        FfiProcessResult_v2 {
            text: ffi_str,
            backspace_count: backspace_count.min(255) as u8, // v2 uses u8
            consumed: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_ffi_string_valid() {
        let s = "hello";
        let ffi_str = to_ffi_string(s);
        assert!(!ffi_str.is_null());

        unsafe {
            let result = from_ffi_string(ffi_str);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), s);
            free_ffi_string(ffi_str);
        }
    }

    #[test]
    fn test_to_ffi_string_with_unicode() {
        let s = "việt nam";
        let ffi_str = to_ffi_string(s);
        assert!(!ffi_str.is_null());

        unsafe {
            let result = from_ffi_string(ffi_str);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), s);
            free_ffi_string(ffi_str);
        }
    }

    #[test]
    fn test_from_ffi_string_null() {
        unsafe {
            let result = from_ffi_string(std::ptr::null());
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), "Null pointer");
        }
    }

    #[test]
    fn test_input_method_conversions() {
        let telex = FfiInputMethod::Telex;
        let id = to_input_method_id(telex);
        assert_eq!(id, InputMethodId::Telex);
        assert_eq!(from_input_method_id(id), telex);

        let vni = FfiInputMethod::Vni;
        let id = to_input_method_id(vni);
        assert_eq!(id, InputMethodId::Vni);
        assert_eq!(from_input_method_id(id), vni);
    }

    #[test]
    fn test_tone_style_conversions() {
        let old = FfiToneStyle::Old;
        let style = to_tone_style(old);
        assert_eq!(style, TonePlacementStyle::Old);
        assert_eq!(from_tone_style(style), old);

        let new = FfiToneStyle::New;
        let style = to_tone_style(new);
        assert_eq!(style, TonePlacementStyle::New);
        assert_eq!(from_tone_style(style), new);
    }

    #[test]
    fn test_engine_config_conversions() {
        let ffi_config = FfiConfig {
            input_method: FfiInputMethod::Vni,
            tone_style: FfiToneStyle::Old,
            smart_mode: false,
            enable_shortcuts: true,
        };

        let engine_config = to_engine_config(ffi_config);
        assert_eq!(engine_config.input_method, InputMethodId::Vni);
        assert!(!engine_config.smart_mode);

        let back = from_engine_config(&engine_config);
        assert_eq!(back.input_method, ffi_config.input_method);
        assert_eq!(back.smart_mode, ffi_config.smart_mode);
    }

    #[test]
    fn test_to_ffi_process_result_v2_with_text() {
        let transform = TransformResult::new(
            crate::domain::entities::key_event::Action::Insert,
            crate::domain::value_objects::char_sequence::CharSequence::from("việt".to_string()),
        );

        let ffi_result = to_ffi_process_result_v2(transform);
        assert!(!ffi_result.text.is_null());
        assert!(ffi_result.consumed);

        unsafe {
            let text = from_ffi_string(ffi_result.text).unwrap();
            assert_eq!(text, "việt");
            free_ffi_string(ffi_result.text);
        }
    }

    #[test]
    fn test_to_ffi_process_result_v2_empty() {
        let transform = TransformResult::new(
            crate::domain::entities::key_event::Action::None,
            crate::domain::value_objects::char_sequence::CharSequence::empty(),
        );
        let ffi_result = to_ffi_process_result_v2(transform);

        assert!(ffi_result.text.is_null());
        assert_eq!(ffi_result.backspace_count, 0);
        assert!(!ffi_result.consumed);
    }
}
