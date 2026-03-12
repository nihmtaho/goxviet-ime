//! Debug test to trace SPACE behavior after English words

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

fn trace_word_then_space(word: &str) {
    let mut engine = Engine::new();
    engine.set_method(0);
    let mut display = String::new();

    println!("\n=== Typing '{}' + SPACE ===", word);
    for ch in word.chars() {
        let key = char_to_key(ch);
        let result = engine.on_key_ext(key, ch.is_ascii_uppercase(), false, false);
        if result.action == 1 {
            let bs = result.backspace as usize;
            for _ in 0..bs.min(display.len()) { display.pop(); }
            for i in 0..result.count as usize {
                if let Some(c) = char::from_u32(result.as_slice()[i]) { display.push(c); }
            }
        } else {
            display.push(ch);
        }
    }
    println!("  Display before SPACE: '{}'", display);
    println!("  Buffer: '{}'", engine.get_buffer());

    let r = engine.on_key_ext(keys::SPACE, false, false, false);
    println!("  SPACE: action={}, backspace={}, count={}", r.action, r.backspace, r.count);
    if r.action == 1 {
        let out: String = (0..r.count as usize)
            .filter_map(|i| char::from_u32(r.as_slice()[i]))
            .collect();
        println!("  SPACE output: {:?}", out);
        for _ in 0..r.backspace as usize { if !display.is_empty() { display.pop(); } }
        for c in out.chars() { display.push(c); }
    } else {
        display.push(' ');
    }
    println!("  Final display: '{}'", display);
}

#[test]
fn debug_space_scenarios() {
    trace_word_then_space("asset");   // normal (a-s-s-e-t)
    trace_word_then_space("assset");  // triple (a-s-s-s-e-t)
    trace_word_then_space("offer");   // normal
    trace_word_then_space("offfer");  // triple
}
