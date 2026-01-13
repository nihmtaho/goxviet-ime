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
            let new_chars: String = result.chars[0..result.count as usize]
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

#[test]
fn test_giong_typing_with_free_tone_off() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_free_tone(false);

    let mut buffer = String::new();

    // Type: g, i, o, o, n, g, s
    simulate_type(&mut engine, &mut buffer, 5, false); // g
    simulate_type(&mut engine, &mut buffer, 34, false); // i
    simulate_type(&mut engine, &mut buffer, 31, false); // o
    simulate_type(&mut engine, &mut buffer, 31, false); // o -> ô
    simulate_type(&mut engine, &mut buffer, 45, false); // n
    simulate_type(&mut engine, &mut buffer, 5, false); // g
    simulate_type(&mut engine, &mut buffer, 1, false); // s (sắc mark)

    // Result should be transformed correctly
    assert!(
        buffer.starts_with("giống"),
        "Expected 'giống', got '{}'",
        buffer
    );
}

#[test]
fn test_may_typing_with_free_tone_off() {
    let mut engine = Engine::new();
    engine.set_method(0);
    engine.set_free_tone(false);

    let mut buffer = String::new();

    // Type: m, a, y, s
    simulate_type(&mut engine, &mut buffer, 46, false); // m
    simulate_type(&mut engine, &mut buffer, 0, false); // a
    simulate_type(&mut engine, &mut buffer, 16, false); // y
    simulate_type(&mut engine, &mut buffer, 1, false); // s (sắc mark)

    assert!(
        buffer.starts_with("máy"),
        "Expected 'máy', got '{}'",
        buffer
    );
}
#[test]
fn test_giong_typing_intermediate_tone_with_free_tone_off() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_free_tone(false);

    let mut buffer = String::new();

    // Type: g, i, o, o, s, n, g
    simulate_type(&mut engine, &mut buffer, 5, false); // g
    simulate_type(&mut engine, &mut buffer, 34, false); // i
    simulate_type(&mut engine, &mut buffer, 31, false); // o
    simulate_type(&mut engine, &mut buffer, 31, false); // o -> ô
    simulate_type(&mut engine, &mut buffer, 1, false); // s (sắc mark)
    simulate_type(&mut engine, &mut buffer, 45, false); // n
    simulate_type(&mut engine, &mut buffer, 5, false); // g

    // Result should be transformed correctly
    assert!(
        buffer.starts_with("giống"),
        "Expected 'giống', got '{}'",
        buffer
    );
}
