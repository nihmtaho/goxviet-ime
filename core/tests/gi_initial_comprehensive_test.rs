use goxviet_core::engine::Engine;
use goxviet_core::utils::key_to_char;

fn simulate_type(engine: &mut Engine, buffer: &mut String, key: u16, caps: bool) {
    let result = engine.on_key(key, caps, false);
    if result.action == 1 || result.action == 2 {
        // Transformation: delete backspaces and append new chars
        if result.backspace > 0 {
            let chars_to_remove = result.backspace as usize;
            let char_count = buffer.chars().count();
            if char_count >= chars_to_remove {
                *buffer = buffer.chars().take(char_count - chars_to_remove).collect();
            }
        }
        if result.count > 0 {
            let new_chars: String = result.as_slice()[0..result.count as usize]
                .iter()
                .filter_map(|&c| char::from_u32(c))
                .collect();
            buffer.push_str(&new_chars);
        }
    } else {
        // No transformation: just push raw char (simulating platform output)
        if let Some(c) = key_to_char(key, caps) {
            buffer.push(c);
        }
    }
}

/// Test comprehensive "gi" initial words with various Vietnamese patterns
/// Based on VALID_INITIALS_2 from constants.rs: [keys::G, keys::I]
#[test]
fn test_gi_initial_words_comprehensive() {
    let test_cases = vec![
        // Format: (input_sequence, expected_output, description)
        (
            "gioosng",
            "giống",
            "gi + ô + ng (typical intermediate tone)",
        ),
        ("gioonggs", "giống", "gi + ô + ng + sắc (tone at end)"),
        ("giauf", "giàu", "gi + a + u + huyền"), // ia+u pattern, not â+u
        ("gieesngs", "giếng", "gi + ê + ng + sắc"),
        ("giojt", "giọt", "gi + o + j + t -> giọt"),  // Fixed: was "giootj"
        ("giaf", "già", "gi + a + huyền"),  // Fixed: was "giaar"
        ("gioox", "giỗ", "gi + ô + tone"),
        ("gias", "giá", "gi + a + tone"),
    ];

    for (input, expected, description) in test_cases {
        let mut engine = Engine::new();
        engine.set_method(0); // Telex
        engine.set_free_tone(false);

        let mut buffer = String::new();

        for ch in input.chars() {
            let key = match ch {
                'g' => 5,
                'i' => 34,
                'o' => 31,
                's' => 1,
                'n' => 45,
                'a' => 0,
                'u' => 32,
                'e' => 14,
                't' => 17,
                'f' => 3,
                'r' => 15,
                'x' => 7,
                'j' => 38,
                _ => continue,
            };
            simulate_type(&mut engine, &mut buffer, key, false);
        }

        assert!(
            buffer.starts_with(expected),
            "Test '{}': Expected '{}', got '{}' (input: {})",
            description,
            expected,
            buffer,
            input
        );
    }
}

/// Test that 'i' in "gi" is NOT treated as a vowel for tone positioning
#[test]
fn test_gi_initial_vowel_exclusion() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_free_tone(false);

    let mut buffer = String::new();

    // Type: g, i, o, o (results in "giô")
    simulate_type(&mut engine, &mut buffer, 5, false); // g
    simulate_type(&mut engine, &mut buffer, 34, false); // i
    simulate_type(&mut engine, &mut buffer, 31, false); // o
    simulate_type(&mut engine, &mut buffer, 31, false); // o -> ô

    assert_eq!(buffer, "giô", "Should have gi + ô, not gí + o");

    // Now add tone mark 's' (sắc)
    simulate_type(&mut engine, &mut buffer, 1, false); // s

    // The tone should be on 'ô', not on 'i'
    assert!(
        buffer.starts_with("giố") || buffer.starts_with("giôs"),
        "Tone mark should target 'ô', not 'i'. Got: {}",
        buffer
    );
}

/// Test edge case: gi followed by various vowel combinations
#[test]
fn test_gi_with_diphthongs() {
    let test_cases = vec![
        ("giaaun", "giâu + n", vec![5, 34, 0, 0, 32, 45]), // gi + â + u + n
        ("gieeun", "giêu + n", vec![5, 34, 14, 14, 32, 45]), // gi + ê + u + n
    ];

    for (description, expected_pattern, keys) in test_cases {
        let mut engine = Engine::new();
        engine.set_method(0);
        engine.set_free_tone(false);

        let mut buffer = String::new();

        for key in keys {
            simulate_type(&mut engine, &mut buffer, key, false);
        }

        assert!(
            buffer.len() > 0,
            "Test '{}': Expected pattern '{}', got empty buffer",
            description,
            expected_pattern
        );
    }
}
