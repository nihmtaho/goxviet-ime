//! DI Container Integration Tests
//!
//! Regression tests that capture container wiring behavior BEFORE the
//! Arc→Box factory-function refactor. These tests must pass on the
//! pre-refactor code and continue to pass after the refactor.

use goxviet_core::application::dto::EngineConfig;
use goxviet_core::domain::entities::key_event::KeyEvent;
use goxviet_core::domain::ports::input::InputMethodId;
use goxviet_core::domain::ports::transformation::ToneStrategy;
use goxviet_core::presentation::di::Container;

// ─── Helpers ─────────────────────────────────────────────────────────────────

fn telex_config() -> EngineConfig {
    EngineConfig {
        input_method: InputMethodId::Telex,
        tone_strategy: ToneStrategy::default(),
        enabled: true,
        smart_mode: false,
        spell_check: false,
        auto_correct: false,
        max_history_size: 100,
        buffer_timeout_ms: 1000,
        use_modern_tone_placement: false,
        enable_shortcuts: false,
        instant_restore_enabled: false,
        esc_restore_enabled: false,
        bracket_shortcuts_enabled: false,
        foreign_consonants_enabled: false,
        auto_capitalise_enabled: false,
        word_history_enabled: false,
    }
}

fn vni_config() -> EngineConfig {
    EngineConfig {
        input_method: InputMethodId::Vni,
        ..telex_config()
    }
}

fn process_char(container: &Container, ch: char) -> Result<(), String> {
    let keycode = goxviet_core::utils::char_to_key(ch);
    let key_event = KeyEvent::new(keycode, ch.is_uppercase(), false, false, false);
    let arc = container.processor_service();
    let mut guard = arc.lock().map_err(|e| e.to_string())?;
    guard
        .process_key(key_event)
        .map(|_| ())
        .map_err(|e| format!("{e:?}"))
}

// ─── Test cases ──────────────────────────────────────────────────────────────

/// TC1: Container with Telex config constructs without panic and processes 'a'.
#[test]
fn test_telex_container_processes_key_a() {
    let container = Container::with_config(telex_config());
    assert_eq!(container.get_config().input_method, InputMethodId::Telex);
    let result = process_char(&container, 'a');
    assert!(result.is_ok(), "process_key should not error: {result:?}");
}

/// TC2: Container with VNI config constructs without panic and processes '1'.
#[test]
fn test_vni_container_processes_key_1() {
    let container = Container::with_config(vni_config());
    assert_eq!(container.get_config().input_method, InputMethodId::Vni);
    let result = process_char(&container, '1');
    assert!(result.is_ok(), "process_key should not error: {result:?}");
}

/// TC3: update_config() rewires the engine — swap from Telex to VNI then process a key.
#[test]
fn test_update_config_rewires_input_method() {
    let mut container = Container::with_config(telex_config());
    assert_eq!(container.get_config().input_method, InputMethodId::Telex);

    container.update_config(vni_config());
    assert_eq!(container.get_config().input_method, InputMethodId::Vni);

    // Engine should be usable after rewiring
    let result = process_char(&container, 'a');
    assert!(
        result.is_ok(),
        "process_key after update_config should not error: {result:?}"
    );
}
