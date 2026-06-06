//! Typing-order permutation and shortcut robustness tests.
//!
//! Verifies the shortcut system correctly handles add/lookup roundtrips
//! and that CaseMode/InputMethod configuration is stored faithfully.

use goxviet_core::features::shortcut::{CaseMode, InputMethod, Shortcut, ShortcutTable,
                                        TriggerCondition};

#[test]
fn shortcut_table_add_then_lookup_roundtrip() {
    let mut table = ShortcutTable::new();
    let shortcut = Shortcut {
        trigger: "xvn".to_string(),
        replacement: "Xin Chào".to_string(),
        condition: TriggerCondition::OnWordBoundary,
        case_mode: CaseMode::MatchCase,
        enabled: true,
        input_method: InputMethod::All,
    };
    table.add(shortcut);

    let found = table.lookup("xvn");
    assert!(found.is_some(), "Shortcut 'xvn' should be found after being added");
    let (trigger, sc) = found.unwrap();
    assert_eq!(trigger, "xvn");
    assert_eq!(sc.replacement, "Xin Chào");
}

#[test]
fn shortcut_table_lookup_returns_none_for_unknown_trigger() {
    let table = ShortcutTable::new();
    assert!(
        table.lookup("unknown").is_none(),
        "Lookup for 'unknown' on empty table should return None"
    );
}

#[test]
fn shortcut_case_mode_match_case_stored_correctly() {
    let s = Shortcut {
        trigger: "vn".to_string(),
        replacement: "Việt Nam".to_string(),
        condition: TriggerCondition::OnWordBoundary,
        case_mode: CaseMode::MatchCase,
        enabled: true,
        input_method: InputMethod::All,
    };

    assert_eq!(s.trigger, "vn");
    assert_eq!(s.case_mode, CaseMode::MatchCase);
    assert_eq!(s.replacement, "Việt Nam");
}

#[test]
fn shortcut_input_method_telex_stored_correctly() {
    let s = Shortcut::telex("hcm", "Hồ Chí Minh");
    assert_eq!(s.input_method, InputMethod::Telex);
    assert_eq!(s.condition, TriggerCondition::Immediate);
}

#[test]
fn shortcut_input_method_vni_stored_correctly() {
    let s = Shortcut::vni("hcm", "Hồ Chí Minh");
    assert_eq!(s.input_method, InputMethod::Vni);
}

#[test]
fn shortcut_disabled_not_returned_by_lookup() {
    let mut table = ShortcutTable::new();
    let mut sc = Shortcut::new("abc", "ABC expansion");
    sc.enabled = false;
    table.add(sc);

    assert!(
        table.lookup("abc").is_none(),
        "Disabled shortcut should not be returned by lookup"
    );
}

#[test]
fn shortcut_table_remove_works() {
    let mut table = ShortcutTable::new();
    table.add(Shortcut::new("rm", "remove me"));
    assert!(table.lookup("rm").is_some(), "Shortcut should exist before removal");

    table.remove("rm");
    assert!(table.lookup("rm").is_none(), "Shortcut should not exist after removal");
}
