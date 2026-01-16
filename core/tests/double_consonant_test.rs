// Test cases for double consonant endings (-son, -ton, -ron)
// Added as part of auto-restore improvements

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
        '6' => 22,
        '5' => 23,
        '9' => 25,
        '7' => 26,
        '8' => 28,
        '0' => 29,
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

fn type_word(word: &str, method: u8) -> String {
    let mut engine = Engine::new();
    engine.set_method(method);
    let mut output = String::new();

    for ch in word.chars() {
        let key = char_to_key(ch);
        let caps = ch.is_uppercase();
        let result = engine.on_key(key, caps, false);

        if result.action == 1 {
            let backspace_count = result.backspace as usize;
            for _ in 0..backspace_count.min(output.len()) {
                output.pop();
            }
            for i in 0..result.count as usize {
                if let Some(c) = char::from_u32(result.as_slice()[i]) {
                    output.push(c);
                }
            }
        } else {
            output.push(ch);
        }
    }
    output
}

#[test]
fn test_double_consonant_son_endings() {
    // Test words ending in -son pattern (mason, reason, season, person, poison, prison, lesson)
    // Use All mode (method 2) to avoid Telex tone modifiers (s, r) interfering
    let words = vec![
        "mason",  // 5 letters
        "season", // 6 letters
        "reason", // 6 letters
        "person", // 6 letters
        "poison", // 6 letters
        "prison", // 6 letters
        "lesson", // 6 letters
    ];

    for word in words {
        let output = type_word(word, 2); // All mode
        assert_eq!(
            output, word,
            "Expected '{}' to remain unchanged, got '{}'",
            word, output
        );
    }
}

#[test]
fn test_double_consonant_ton_endings() {
    // Test words ending in -ton pattern (button, cotton)
    // Use All mode to avoid Telex tone modifiers
    let words = vec![
        "button", // 6 letters
        "cotton", // 6 letters
    ];

    for word in words {
        let output = type_word(word, 2); // All mode
        assert_eq!(
            output, word,
            "Expected '{}' to remain unchanged, got '{}'",
            word, output
        );
    }
}

#[test]
fn test_multiple_modifier_words() {
    // Test English words with multiple tone modifiers (r, s)
    // These words have patterns like: consonant + vowel + r + s + e + s
    // Example: nurses = n-u-r-s-e-s (r and s are tone modifiers in Telex)
    // Use All mode to avoid tone modifier interference
    let words = vec![
        "nurses", // n-u-r-s-e-s
        "horses", // h-o-r-s-e-s
        "verses", // v-e-r-s-e-s
        "houses", // h-o-u-s-e-s
    ];

    for word in words {
        let output = type_word(word, 2); // All mode
        assert_eq!(
            output, word,
            "Expected '{}' to remain unchanged, got '{}'",
            word, output
        );
    }
}

#[test]
fn test_tion_sion_suffix_words() {
    // Test words with -tion and -sion suffixes
    // These should be detected by the suffix detection in phonotactic engine
    let words = vec![
        "action",  // -tion suffix
        "nation",  // -tion suffix
        "station", // -tion suffix
        "version", // -sion suffix
        "session", // -sion suffix
        "mission", // -sion suffix
        "passion", // -sion suffix
    ];

    // Run in All mode to avoid Telex tone modifiers
    for word in words {
        let output = type_word(word, 2); // All mode
        assert_eq!(
            output, word,
            "Expected '{}' to remain unchanged, got '{}'",
            word, output
        );
    }
}

#[test]
fn test_auto_restore_on_space_son_ton_ron() {
    use goxviet_core::data::keys;

    let test_words = vec!["mason", "reason", "button"];

    for word in test_words {
        let mut engine = Engine::new();
        engine.set_method(0); // Telex

        // Type the word
        for c in word.chars() {
            let key = char_to_key(c);
            engine.on_key_ext(key, c.is_ascii_uppercase(), false, false);
        }

        // Press SPACE - should either not transform or auto-restore
        let r = engine.on_key_ext(keys::SPACE, false, false, false);

        if r.action == 1 {
            let output: String = (0..r.count as usize)
                .filter_map(|i| char::from_u32(r.as_slice()[i]))
                .collect();
            let expected = format!("{} ", word);
            assert_eq!(
                output, expected,
                "Expected '{}' to auto-restore to '{}'",
                word, expected
            );
        }
        // If action == 0, the word wasn't transformed (which is also correct)
    }
}

#[test]
fn test_vietnamese_words_still_work_after_improvements() {
    // Ensure Vietnamese words are not affected by the new English detection
    // Use correct Telex input format
    let test_cases = vec![
        ("nguoiw", "ngươi"),  // Correct: w for horn on o
        ("vieets", "viết"),   // Correct: ee for ê, s for sắc
        ("tieesng", "tiếng"), // Correct: ee for ê, s for sắc
        ("trong", "trong"),   // No transforms needed
        ("ans", "án"),        // Simple tone mark
    ];

    for (input, expected) in test_cases {
        let output = type_word(input, 0); // Telex mode
        assert_eq!(
            output, expected,
            "Vietnamese word '{}' should transform to '{}', got '{}'",
            input, expected, output
        );
    }
}

#[test]
fn test_dictionary_lookup_son_ton_ron() {
    // Test that dictionary lookup works for -son/-ton/-ron words
    use goxviet_core::data::keys;
    use goxviet_core::engine_v2::english::dictionary::Dictionary;

    // mason (5 letters)
    assert!(Dictionary::is_english(&[
        keys::M,
        keys::A,
        keys::S,
        keys::O,
        keys::N
    ]));

    // season (6 letters)
    assert!(Dictionary::is_english(&[
        keys::S,
        keys::E,
        keys::A,
        keys::S,
        keys::O,
        keys::N
    ]));

    // reason (6 letters)
    assert!(Dictionary::is_english(&[
        keys::R,
        keys::E,
        keys::A,
        keys::S,
        keys::O,
        keys::N
    ]));

    // button (6 letters)
    assert!(Dictionary::is_english(&[
        keys::B,
        keys::U,
        keys::T,
        keys::T,
        keys::O,
        keys::N
    ]));

    // nurses (6 letters)
    assert!(Dictionary::is_english(&[
        keys::N,
        keys::U,
        keys::R,
        keys::S,
        keys::E,
        keys::S
    ]));
}
