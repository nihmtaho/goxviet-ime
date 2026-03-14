//! Regression test for failures_telex.txt cases.
//! Words falsely detected as English due to VR rhotic pattern firing on
//! Telex tone key 'r' (hỏi) when the syllable is not in TuDien dictionary.

use goxviet_core::application::dto::EngineConfig;
use goxviet_core::domain::entities::key_event::{Action, KeyEvent};
use goxviet_core::domain::ports::input::InputMethodId;
use goxviet_core::domain::ports::transformation::ToneStrategy;
use goxviet_core::presentation::di::Container;

fn type_word(container: &mut Container, input: &str) -> String {
    let mut screen = String::new();
    for ch in input.chars() {
        let keycode = goxviet_core::utils::char_to_key(ch);
        let key_event = KeyEvent::new(keycode, ch.is_uppercase(), false, false, false);
        let process_result = {
            let processor_arc = container.processor_service();
            let mut guard = processor_arc.lock().unwrap();
            guard.process_key(key_event)
        };
        if let Ok(result) = process_result {
            let backspace = result.backspace_count();
            let new_text = result.new_text().as_str();
            let action = result.action();
            let has_transform = matches!(action, Action::Replace { .. } | Action::Insert);
            for _ in 0..backspace {
                screen.pop();
            }
            if !new_text.is_empty() {
                screen.push_str(new_text);
            } else if ch == ' ' {
                screen.push(' ');
            } else if !has_transform {
                screen.push(ch);
            }
        }
    }
    screen.trim_end().to_string()
}

fn make_container() -> Container {
    let config = EngineConfig {
        input_method: InputMethodId::Telex,
        tone_strategy: ToneStrategy::Modern,
        enabled: true,
        smart_mode: true,
        spell_check: false,
        auto_correct: false,
        max_history_size: 100,
        buffer_timeout_ms: 1000,
        use_modern_tone_placement: true,
        enable_shortcuts: false,
        instant_restore_enabled: true,
        esc_restore_enabled: true,
    };
    Container::with_config(config)
}

#[test]
fn test_failures_telex_txt_regression() {
    // All cases from core/tests/failures/failures_telex.txt
    let cases = vec![
        ("dir ", "dỉ"),
        ("mieengr ", "miểng"),
        ("miur ", "mỉu"),
        ("ngangr ", "ngảng"),
        ("nhoongr ", "nhổng"),
        ("phungr ", "phủng"),
        ("quir ", "quỉ"),
        ("rongr ", "rỏng"),
        ("sur ", "sủ"),
        ("thuwngr ", "thửng"),
    ];

    let mut failed = Vec::new();
    for (input, expected) in &cases {
        let mut container = make_container();
        let actual = type_word(&mut container, input);
        if actual != *expected {
            failed.push(format!("  {:12} → expected '{}', got '{}'", input.trim(), expected, actual));
        }
    }

    if !failed.is_empty() {
        panic!(
            "failures_telex.txt cases not fixed:\n{}",
            failed.join("\n")
        );
    }
}
