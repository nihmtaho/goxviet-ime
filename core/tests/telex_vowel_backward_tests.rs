use goxviet_core::engine::Engine;

/// Test Telex backward application within vowel sequences
/// Example: "dau" + "a" should produce "dâu" (apply circumflex to first 'a')

#[test]
fn test_telex_daua_becomes_dau_circumflex() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    engine.set_modern_tone(false);

    // Type: d-a-u-a → should become "dâu" (circumflex on first 'a')
    engine.on_key(goxviet_core::data::keys::D, false, false);
    engine.on_key(goxviet_core::data::keys::A, false, false);
    engine.on_key(goxviet_core::data::keys::U, false, false);
    assert_eq!(engine.get_buffer(), "dau");

    // Type 'a' again - should apply backward to first 'a' (before 'u')
    engine.on_key(goxviet_core::data::keys::A, false, false);
    assert_eq!(
        engine.get_buffer(),
        "dâu",
        "Telex: dau + a should produce dâu (circumflex on first 'a')"
    );
}

#[test]
fn test_telex_caoa_becomes_cao_circumflex() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    engine.set_modern_tone(false);

    // Type: c-a-o-a → should become "câo" (circumflex on first 'a')
    engine.on_key(goxviet_core::data::keys::C, false, false);
    engine.on_key(goxviet_core::data::keys::A, false, false);
    engine.on_key(goxviet_core::data::keys::O, false, false);
    assert_eq!(engine.get_buffer(), "cao");

    // Type 'a' - should apply backward to first 'a'
    engine.on_key(goxviet_core::data::keys::A, false, false);
    assert_eq!(
        engine.get_buffer(),
        "câo",
        "Telex: cao + a should produce câo"
    );
}
