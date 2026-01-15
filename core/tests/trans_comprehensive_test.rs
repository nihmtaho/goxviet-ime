use goxviet_core::data::keys;
use goxviet_core::engine::Engine;

#[test]
fn test_trans_comprehensive() {
    let mut engine = Engine::new();

    // Type 't' - should be normal
    engine.on_key(keys::T, false, false);
    assert_eq!(engine.get_buffer(), "t");

    // Type 'r' - should be normal
    engine.on_key(keys::R, false, false);
    assert_eq!(engine.get_buffer(), "tr");

    // Type 'a' - should be normal
    engine.on_key(keys::A, false, false);
    assert_eq!(engine.get_buffer(), "tra");

    // Type 'n' - should be normal
    engine.on_key(keys::N, false, false);
    assert_eq!(engine.get_buffer(), "tran");

    // Type 's' - should detect as English and set is_english_word = true
    engine.on_key(keys::S, false, false);
    assert_eq!(
        engine.get_buffer(),
        "trans",
        "After typing 's', buffer should be 'trans'"
    );

    // Type 'f' - should NOT apply tone because is_english_word = true
    engine.on_key(keys::F, false, false);
    assert_eq!(
        engine.get_buffer(),
        "transf",
        "After typing 'f', buffer should be 'transf', not 'trànsf'"
    );

    // Type 'o' - should continue as English
    engine.on_key(keys::O, false, false);
    assert_eq!(engine.get_buffer(), "transfo");

    // Type 'r' - should continue as English
    engine.on_key(keys::R, false, false);
    assert_eq!(engine.get_buffer(), "transfor");

    // Type 'm' - should continue as English
    engine.on_key(keys::M, false, false);
    assert_eq!(engine.get_buffer(), "transform");
}

#[test]
fn test_vietnamese_still_works() {
    let mut engine = Engine::new();

    // Type "cố" - should work normally
    engine.on_key(keys::C, false, false);
    assert_eq!(engine.get_buffer(), "c");

    engine.on_key(keys::O, false, false);
    assert_eq!(engine.get_buffer(), "co");

    engine.on_key(keys::F, false, false); // tone f
    assert_eq!(engine.get_buffer(), "cò");
}
