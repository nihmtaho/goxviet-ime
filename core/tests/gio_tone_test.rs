use goxviet_core::engine::Engine;
use goxviet_core::utils::key_to_char;

fn simulate_type(engine: &mut Engine, buffer: &mut String, key: u16, caps: bool) {
    let result = engine.on_key(key, caps, false);
    if result.action == 1 || result.action == 2 {
        if result.backspace > 0 {
            let chars_to_remove = result.backspace as usize;
            let char_count = buffer.chars().count();
            if char_count >= chars_to_remove {
                *buffer = buffer.chars().take(char_count - chars_to_remove).collect();
            }
        }
        if result.count > 0 {
            let new_chars: String = result.chars[0..result.count as usize]
                .iter()
                .filter_map(|&c| char::from_u32(c))
                .collect();
            buffer.push_str(&new_chars);
        }
    } else {
        if let Some(c) = key_to_char(key, caps) {
            buffer.push(c);
        }
    }
}

#[test]
fn test_gio_with_tone_marks() {
    let test_cases = vec![
        ("gios", "gió", "gi + o + sắc"),
        ("giof", "giò", "gi + o + huyền"),
        ("giox", "giõ", "gi + o + ngã"),
        ("gioj", "giọ", "gi + o + nặng"),
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
                'f' => 3,
                'x' => 7,
                'j' => 38,
                _ => continue,
            };
            simulate_type(&mut engine, &mut buffer, key, false);
        }

        assert_eq!(
            buffer, expected,
            "Test '{}': Expected '{}', got '{}' (input: {})",
            description, expected, buffer, input
        );
    }
}
