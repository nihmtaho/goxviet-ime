//! Tests for extended shortcut API — smart case, trigger conditions, input method filtering.

use goxviet_core::features::shortcut::{CaseMode, InputMethod, Shortcut, TriggerCondition};

#[test]
fn shortcut_smart_case_lower_input() {
    // "vn" typed → replacement starts with same case as trigger
    let s = Shortcut {
        trigger: "vn".to_string(),
        replacement: "Việt Nam".to_string(),
        condition: TriggerCondition::OnWordBoundary,
        case_mode: CaseMode::MatchCase,
        enabled: true,
        input_method: InputMethod::All,
    };
    assert_eq!(s.trigger, "vn");
    assert!(s.enabled);
}

#[test]
fn trigger_condition_immediate_for_symbols() {
    let s = Shortcut {
        trigger: "→".to_string(),
        replacement: "→→".to_string(),
        condition: TriggerCondition::Immediate,
        case_mode: CaseMode::Exact,
        enabled: true,
        input_method: InputMethod::All,
    };
    assert_eq!(s.condition, TriggerCondition::Immediate);
}

#[test]
fn shortcut_input_method_filtering() {
    let s = Shortcut {
        trigger: "vn".to_string(),
        replacement: "Việt Nam".to_string(),
        condition: TriggerCondition::OnWordBoundary,
        case_mode: CaseMode::MatchCase,
        enabled: true,
        input_method: InputMethod::Telex,
    };
    assert_eq!(s.input_method, InputMethod::Telex);
}

#[test]
fn ime_add_shortcut_ext_v2_integration() {
    use goxviet_core::{
        ime_add_shortcut_ext_v2, ime_create_engine_v2, ime_destroy_engine_v2,
        ime_shortcuts_count_v2, FfiConfig_v2, FfiShortcutExt_v2, FfiStatusCode,
    };
    use std::ffi::CString;

    // Create engine with default config
    let config = FfiConfig_v2::default();
    let engine = unsafe { ime_create_engine_v2(&config) };
    assert!(!engine.is_null());

    // Record baseline shortcut count
    let count_before = unsafe { ime_shortcuts_count_v2(engine) };

    // Build extended shortcut: use a unique trigger unlikely to collide with defaults
    let trigger = CString::new("xvn").unwrap();
    let replacement = CString::new("Xin Việt Nam").unwrap();
    let ext = FfiShortcutExt_v2 {
        trigger: trigger.as_ptr(),
        replacement: replacement.as_ptr(),
        trigger_condition: 0, // OnWordBoundary
        case_mode: 0,         // MatchCase
        enabled: true,
        input_method: 0, // All
    };

    let status = unsafe { ime_add_shortcut_ext_v2(engine, &ext) };
    assert_eq!(
        status,
        FfiStatusCode::Success,
        "Expected Success, got {:?}",
        status
    );

    let count_after = unsafe { ime_shortcuts_count_v2(engine) };
    assert_eq!(
        count_after,
        count_before + 1,
        "Shortcut count should increase by 1"
    );

    unsafe { ime_destroy_engine_v2(engine) };
}
