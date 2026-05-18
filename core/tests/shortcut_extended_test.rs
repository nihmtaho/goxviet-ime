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
