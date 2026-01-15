use goxviet_core::data::keys;
use goxviet_core::engine::Engine;

#[test]
fn test_trans_step_by_step() {
    let mut engine = Engine::new();

    // Type 't'
    let r1 = engine.on_key(keys::T, false, false);
    println!("Step 1 - After 't': buffer='{}'", engine.get_buffer());

    // Type 'r'
    let r2 = engine.on_key(keys::R, false, false);
    println!("Step 2 - After 'r': buffer='{}'", engine.get_buffer());

    // Type 'a'
    let r3 = engine.on_key(keys::A, false, false);
    println!("Step 3 - After 'a': buffer='{}'", engine.get_buffer());

    // Type 'n'
    let r4 = engine.on_key(keys::N, false, false);
    println!("Step 4 - After 'n': buffer='{}'", engine.get_buffer());

    // Type 's' - THIS SHOULD TRIGGER ENGLISH DETECTION
    let r5 = engine.on_key(keys::S, false, false);
    println!("Step 5 - After 's': buffer='{}'", engine.get_buffer());
    assert_eq!(
        engine.get_buffer(),
        "trans",
        "After 's', buffer should be 'trans'"
    );

    // Type 'f' - THIS SHOULD BE BLOCKED BY is_english_word FLAG
    let r6 = engine.on_key(keys::F, false, false);
    println!("Step 6 - After 'f': buffer='{}'", engine.get_buffer());

    // CRITICAL ASSERTION
    assert_eq!(
        engine.get_buffer(),
        "transf",
        "FAILED: After 'f', buffer should be 'transf' (not 'tràns' or 'trànsf').\n\
         This means is_english_word was NOT set to true when typing 'trans'."
    );
}
