use goxviet_core::engine::Engine;

fn char_to_key(c: char) -> u16 {
    match c.to_ascii_lowercase() {
        'a' => 0,
        's' => 1,
        'd' => 2,
        'f' => 3,
        'h' => 4,
        'g' => 5,
        'z' => 6,
        'x' => 7,
        'c' => 8,
        'v' => 9,
        'b' => 11,
        'q' => 12,
        'w' => 13,
        'e' => 14,
        'r' => 15,
        'y' => 16,
        't' => 17,
        '1' => 18,
        '2' => 19,
        '3' => 20,
        '4' => 21,
        '5' => 23,
        '6' => 22,
        '7' => 26,
        '8' => 28,
        '9' => 25,
        '0' => 29,
        ' ' => 49,
        'o' => 31,
        'u' => 32,
        'i' => 34,
        'p' => 35,
        'l' => 37,
        'j' => 38,
        'k' => 40,
        'n' => 45,
        'm' => 46,
        _ => 255,
    }
}

fn type_word(engine: &mut Engine, word: &str) -> String {
    let mut output = String::new();

    for ch in word.chars() {
        let key = char_to_key(ch);
        let caps = ch.is_uppercase();
        let result = engine.on_key_ext(key, caps, false, false);

        if result.action == 1 {
            let backspace_count = result.backspace as usize;
            for _ in 0..backspace_count.min(output.len()) {
                output.pop();
            }

            for i in 0..result.count as usize {
                if let Some(c) = char::from_u32(result.chars[i]) {
                    output.push(c);
                }
            }
        } else {
            // Action 0 = pass through
            output.push(ch);
        }
    }

    output
}

#[test]
fn test_ung_is_vietnamese() {
    // Telex mode (0)
    let mut engine = Engine::new();
    engine.set_method(0);
    let output = type_word(&mut engine, "ung ");
    assert_eq!(output, "ung ", "Expected 'ung' to remain 'ung '");

    let mut engine = Engine::new();
    engine.set_method(0);
    let output = type_word(&mut engine, "uwng ");
    assert_eq!(output, "ưng ", "Expected 'uwng' to become 'ưng '");

    // VNI mode (1)
    let mut engine = Engine::new();
    engine.set_method(1);
    let output = type_word(&mut engine, "ung7 ");
    assert_eq!(output, "ưng ", "Expected 'ung7' to become 'ưng ' in VNI");
}

#[test]
fn test_homebrew_is_english() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex

    let output = type_word(&mut engine, "homebrew ");
    assert_eq!(
        output, "homebrew ",
        "Expected 'homebrew' to be restored as English"
    );
}

#[test]
fn test_ew_words_are_english() {
    let words = vec![
        "brew", "new", "view", "crew", "screw", "stew", "flew", "grew", "news",
    ];

    for word in words {
        let mut engine = Engine::new();
        engine.set_method(0); // Telex
        let input = format!("{} ", word);
        let output = type_word(&mut engine, &input);
        assert_eq!(
            output,
            format!("{} ", word),
            "Expected '{}' to be English",
            word
        );
    }
}

#[test]
fn test_telex_modifiers_remain_vietnamese() {
    let mut engine = Engine::new();
    engine.set_method(0);
    let output = type_word(&mut engine, "tuw ");
    assert_eq!(output, "tư ", "Expected 'tuw' to become 'tư ' (Vietnamese)");

    let mut engine = Engine::new();
    engine.set_method(0);
    let output = type_word(&mut engine, "mow ");
    assert_eq!(output, "mơ ", "Expected 'mow' to become 'mơ ' (Vietnamese)");

    let mut engine = Engine::new();
    engine.set_method(0);
    let output = type_word(&mut engine, "saw ");
    assert_eq!(output, "să ", "Expected 'saw' to become 'să ' (Vietnamese)");
}
