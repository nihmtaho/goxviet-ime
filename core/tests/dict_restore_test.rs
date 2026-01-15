use goxviet_core::data::keys;
use goxviet_core::engine::Engine;

#[test]
fn test_dictionary_word_over_with_transforms() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex

    // In Telex, 'o' alone won't create a transform
    // But if user types "ov" then 'e' (which is a mark key in some contexts)
    // Let's trace what happens with "over"

    println!("=== Test: 'over' with possible transforms ===");

    // Type "o"
    let r1 = engine.on_key(keys::O, false, false);
    println!(
        "After 'o': buffer='{}', is_english={}, action={}",
        engine.get_buffer(),
        engine.is_english_word,
        r1.action
    );

    // Type "v"
    let r2 = engine.on_key(keys::V, false, false);
    println!(
        "After 'v': buffer='{}', is_english={}, action={}",
        engine.get_buffer(),
        engine.is_english_word,
        r2.action
    );

    // Type "e" - in Telex, 'e' can be a mark key after vowels
    let r3 = engine.on_key(keys::E, false, false);
    println!(
        "After 'e': buffer='{}', is_english={}, action={}",
        engine.get_buffer(),
        engine.is_english_word,
        r3.action
    );

    // Type "r" - should now detect "over" is English
    let r4 = engine.on_key(keys::R, false, false);
    println!(
        "After 'r': buffer='{}', is_english={}, action={}",
        engine.get_buffer(),
        engine.is_english_word,
        r4.action
    );

    assert_eq!(engine.get_buffer(), "over", "Buffer should be 'over'");
}

#[test]
fn test_word_with_actual_transform() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex

    println!("=== Test: 'oser' which might get 's' interpreted as tone ===");

    // Type "o"
    let _r1 = engine.on_key(keys::O, false, false);
    println!("After 'o': buffer='{}'", engine.get_buffer());

    // Type "s" - in Telex, 's' is sắc tone key
    let _r2 = engine.on_key(keys::S, false, false);
    println!(
        "After 's': buffer='{}' (should have tone if applied)",
        engine.get_buffer()
    );

    // Type "e"
    let _r3 = engine.on_key(keys::E, false, false);
    println!("After 'e': buffer='{}'", engine.get_buffer());

    // Type "r"
    let _r4 = engine.on_key(keys::R, false, false);
    println!("After 'r': buffer='{}'", engine.get_buffer());
}

#[test]
fn test_user_typing_same_key_twice() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex

    println!("=== Test: 'oo' which creates circumflex in Telex ===");

    // Type "o"
    let _r1 = engine.on_key(keys::O, false, false);
    println!("After 'o': buffer='{}'", engine.get_buffer());

    // Type "o" again - should create 'ô' in Telex
    let _r2 = engine.on_key(keys::O, false, false);
    println!(
        "After 'oo': buffer='{}' (should be 'ô')",
        engine.get_buffer()
    );

    // Type "v"
    let _r3 = engine.on_key(keys::V, false, false);
    println!("After 'v': buffer='{}'", engine.get_buffer());

    // Type "e"
    let _r4 = engine.on_key(keys::E, false, false);
    println!("After 'e': buffer='{}'", engine.get_buffer());

    // Type "r" - should detect "oover" or something
    let _r5 = engine.on_key(keys::R, false, false);
    println!(
        "After 'r': buffer='{}', is_english={}",
        engine.get_buffer(),
        engine.is_english_word
    );
}

#[test]
fn test_syntax_with_x_as_tone() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex

    println!("=== Test: 'syntax' where x is ngã tone key in Telex ===");

    // Type "s", "y", "n", "t", "a", "x"
    let keys_to_type = [
        (keys::S, "s"),
        (keys::Y, "y"),
        (keys::N, "n"),
        (keys::T, "t"),
        (keys::A, "a"),
        (keys::X, "x"), // x is ngã tone in Telex!
    ];

    for (key, ch) in keys_to_type.iter() {
        let r = engine.on_key(*key, false, false);
        println!(
            "After '{}': buffer='{}', is_english={}, action={}",
            ch,
            engine.get_buffer(),
            engine.is_english_word,
            r.action
        );
    }

    // Check if 'x' was applied as tone or if word was detected as English
    let buffer = engine.get_buffer();
    println!(
        "Final buffer: '{}' (should be 'syntax', not 'syntã' or similar)",
        buffer
    );

    // The 'x' key might have been interpreted as ngã tone on 'a' before detecting English
    // This is the REAL issue!
}
