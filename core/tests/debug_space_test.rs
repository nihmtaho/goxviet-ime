//! Debug test to see what SPACE outputs after "assset"

use goxviet_core::data::keys;
use goxviet_core::engine::Engine;

fn char_to_key(c: char) -> u16 {
    match c.to_ascii_lowercase() {
        'a' => 0, 's' => 1, 'd' => 2, 'f' => 3, 'h' => 4, 'g' => 5, 'z' => 6, 'x' => 7,
        'c' => 8, 'v' => 9, 'b' => 11, 'q' => 12, 'w' => 13, 'e' => 14, 'r' => 15, 'y' => 16,
        't' => 17, 'o' => 31, 'u' => 32, 'i' => 34, 'p' => 35, 'l' => 37, 'j' => 38, 'k' => 40,
        'n' => 45, 'm' => 46, _ => 255,
    }
}

#[test]
fn debug_space_output() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex

    println!("\nTyping 'assset'...");
    for ch in "assset".chars() {
        let key = char_to_key(ch);
        let result = engine.on_key_ext(key, ch.is_ascii_uppercase(), false, false);
        println!("  After '{}': action={}, backspace={}, count={}", ch, result.action, result.backspace, result.count);
    }

    println!("\nNow pressing SPACE...");
    let result = engine.on_key_ext(keys::SPACE, false, false, false);
    println!("SPACE result: action={}, backspace={}, count={}", result.action, result.backspace, result.count);

    if result.action == 1 {
        let output: String = (0..result.count as usize)
            .filter_map(|i| char::from_u32(result.as_slice()[i]))
            .collect();
        println!("SPACE output: {:?}", output);
    } else {
        println!("SPACE output: None (action != 1)");
    }
}
