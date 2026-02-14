use goxviet_core::engine::Engine;

fn char_to_key(c: char) -> u16 {
    match c.to_ascii_lowercase() {
        'a' => 0, 's' => 1, 'd' => 2, 'f' => 3, 'h' => 4, 'g' => 5, 'z' => 6,
        'x' => 7, 'c' => 8, 'v' => 9, 'b' => 11, 'q' => 12, 'w' => 13,
        'e' => 14, 'r' => 15, 'y' => 16, 't' => 17, 'o' => 31, 'u' => 32,
        'i' => 34, 'p' => 35, 'l' => 37, 'j' => 38, 'k' => 40, 'n' => 45, 'm' => 46,
        _ => 255,
    }
}

fn test_word(word: &str) -> String {
    let mut engine = Engine::new();
    engine.set_method(0);
    engine.set_enabled(true);
    let mut output = String::new();
    for ch in word.chars() {
        let key = char_to_key(ch);
        let result = engine.on_key(key, false, false);
        if result.action == 1 {
            for _ in 0..result.backspace.min(output.len() as u8) { output.pop(); }
            for i in 0..result.count as usize {
                unsafe { if let Some(c) = char::from_u32(*result.chars.offset(i as isize)) { output.push(c); } }
            }
        } else { output.push(ch); }
    }
    // Space
    let result = engine.on_key(49, false, false);
    if result.action == 1 {
        for _ in 0..result.backspace.min(output.len() as u8) { output.pop(); }
        for i in 0..result.count as usize {
            unsafe { if let Some(c) = char::from_u32(*result.chars.offset(i as isize)) { output.push(c); } }
        }
    }
    output.trim().to_string()
}

fn main() {
    let words = ["sixteen", "depression", "renewed", "recession", "wednesday", "iowa"];
    for word in &words {
        let out = test_word(word);
        let ok = if *word == out { "✓" } else { "✗" };
        println!("{} {:15} -> {}", ok, word, out);
    }
}
