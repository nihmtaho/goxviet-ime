//! Tests for free tone mode — diacritics applied without validation.

#[test]
fn free_tone_config_defaults_to_false() {
    use goxviet_core::application::dto::engine_config::EngineConfig;
    let cfg = EngineConfig::new();
    assert!(!cfg.free_tone_enabled);
}

#[test]
fn free_tone_config_enabled_with_builder() {
    use goxviet_core::application::dto::engine_config::EngineConfig;
    let cfg = EngineConfig::new().with_free_tone(true);
    assert!(cfg.free_tone_enabled);
}

#[test]
fn free_tone_field_in_ffi_config_defaults_false() {
    use goxviet_core::presentation::ffi::types::FfiConfig_v2;
    let cfg = FfiConfig_v2::default();
    assert!(!cfg.free_tone_enabled);
}
