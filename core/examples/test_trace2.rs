use goxviet_core::engine::Engine;

fn char_to_key(c: char) -> u16 {
    match c.to_ascii_lowercase() {
        'a' => 0, 's' => 1, 'd' => 2, 'f' => 3, 'h' => 4, 'g' => 5,
        'z' => 6, 'x' => 7, 'c' => 8, 'v' => 9, 'b' => 11, 'q' => 12,
        'w' => 13, 'e' => 14, 'r' => 15, 'y' => 16, 't' => 17, 'o' => 31,
        'u' => 32, 'i' => 34, 'p' => 35, 'l' => 37, 'j' => 38, 'k' => 40,
        'n' => 45, 'm' => 46, ' ' => 49, _ => 255,
    }
}

fn type_word(engine: &mut Engine, word: &str) -> String {
    engine.clear();
    let mut output = String::new();
    for ch in word.chars() {
        let key = char_to_key(ch);
        if key == 255 { output.push(ch); continue; }
        let result = engine.on_key(key, ch.is_uppercase(), false);
        if result.action == 1 {
            let bs = result.backspace as usize;
            for _ in 0..bs.min(output.len()) { output.pop(); }
            for i in 0..result.count as usize {
                unsafe {
                    if let Some(c) = char::from_u32(*result.chars.offset(i as isize)) {
                        output.push(c);
                    }
                }
            }
        } else {
            output.push(ch);
        }
        eprintln!("  '{}' (key={:2}): action={}, bs={}, cnt={} => \"{}\"",
            ch, key, result.action, result.backspace, result.count, output);
    }
    // Space
    let result = engine.on_key(49, false, false);
    if result.action == 1 {
        let bs = result.backspace as usize;
        for _ in 0..bs.min(output.len()) { output.pop(); }
        for i in 0..result.count as usize {
            unsafe {
                if let Some(c) = char::from_u32(*result.chars.offset(i as isize)) {
                    output.push(c);
                }
            }
        }
    } else {
        output.push(' ');
    }
    eprintln!("  SPACE: action={}, bs={}, cnt={} => \"{}\"",
        result.action, result.backspace, result.count, output);
    output
}

fn main() {
    let mut engine = Engine::new();
    let words = vec!["different", "effect", "issue", "these", "there", "is", "his", "terms", "been"];
    for word in words {
        eprintln!("\n=== {} ===", word);
        let result = type_word(&mut engine, word);
        let expected = format!("{} ", word);
        let status = if result == expected { "OK" } else { "FAIL" };
        println!("{}: \"{}\" (expected \"{}\") [{}]", word, result.trim_end(), word, status);
    }
}
