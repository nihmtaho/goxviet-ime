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

#[test]
fn free_tone_survives_config_roundtrip() {
    use goxviet_core::presentation::ffi::conversions::{from_engine_config_v2, to_engine_config_v2};
    use goxviet_core::presentation::ffi::types::FfiConfig_v2;

    let mut ffi = FfiConfig_v2::default();
    ffi.free_tone_enabled = true;
    let engine_cfg = to_engine_config_v2(&ffi);
    assert!(
        engine_cfg.free_tone_enabled,
        "free_tone_enabled should survive FfiConfig_v2 → EngineConfig conversion"
    );
    let ffi_back = from_engine_config_v2(&engine_cfg);
    assert!(
        ffi_back.free_tone_enabled,
        "free_tone_enabled should survive EngineConfig → FfiConfig_v2 conversion"
    );
}
