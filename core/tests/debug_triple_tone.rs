//! Debug test to trace triple-tone display behavior keystroke by keystroke

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

fn trace_typing(keystrokes: &str) {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    let mut display = String::new();

    println!("\n=== Typing sequence: '{}' ===", keystrokes);
    for (i, ch) in keystrokes.chars().enumerate() {
        let key = char_to_key(ch);
        let caps = ch.is_ascii_uppercase();
        // Use on_key_ext to populate raw_input for the TRIPLE-TONE GUARD to work
        let result = engine.on_key_ext(key, caps, false, false);

        if result.action == 1 {
            let backspace_count = result.backspace as usize;
            for _ in 0..backspace_count.min(display.len()) {
                display.pop();
            }
            for j in 0..result.count as usize {
                if let Some(c) = char::from_u32(result.as_slice()[j]) {
                    display.push(c);
                }
            }
        } else {
            display.push(ch);
        }

        println!(
            "After keystroke {}: '{}' (action={}, backspace={}, count={}) → display: '{}'",
            i + 1,
            ch,
            result.action,
            result.backspace,
            result.count,
            display
        );
    }
}

#[test]
fn debug_assset() {
    trace_typing("assse");
    trace_typing("assset");
}

#[test]
fn debug_passed() {
    // p-a-s-s-s-e-d (3 s's = triple-tone)
    trace_typing("passsed");
}

#[test]
fn debug_assset_full() {
    trace_typing("assset");
}
