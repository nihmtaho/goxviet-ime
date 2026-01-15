use goxviet_core::data::keys;
use goxviet_core::engine::Engine;

#[test]
fn test_fjz_always_english() {
    // Test that words starting with F, J, Z are always detected as English
    // and never transformed to Vietnamese

    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);

    // Test "facebook"
    for &key in &[
        keys::F,
        keys::A,
        keys::C,
        keys::E,
        keys::B,
        keys::O,
        keys::O,
        keys::K,
    ] {
        engine.on_key_ext(key, false, false, false);
    }
    let output = engine.get_buffer();
    assert_eq!(
        output, "facebook",
        "Words starting with 'f' should remain English"
    );

    // Reset engine
    engine = Engine::new();
    engine.set_method(0);
    engine.set_enabled(true);

    // Test "java"
    for &key in &[keys::J, keys::A, keys::V, keys::A] {
        engine.on_key_ext(key, false, false, false);
    }
    let output = engine.get_buffer();
    assert_eq!(
        output, "java",
        "Words starting with 'j' should remain English"
    );

    // Reset engine
    engine = Engine::new();
    engine.set_method(0);
    engine.set_enabled(true);

    // Test "zoom"
    for &key in &[keys::Z, keys::O, keys::O, keys::M] {
        engine.on_key_ext(key, false, false, false);
    }
    let output = engine.get_buffer();
    assert_eq!(
        output, "zoom",
        "Words starting with 'z' should remain English"
    );
}
