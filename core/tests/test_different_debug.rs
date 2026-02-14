use goxviet_core::engine::Engine;

fn char_to_key(c: char) -> u16 {
    match c.to_ascii_lowercase() {
        'a' => 0, 's' => 1, 'd' => 2, 'f' => 3, 'h' => 4, 'g' => 5, 'z' => 6,
        'x' => 7, 'c' => 8, 'v' => 9, 'b' => 11, 'q' => 12, 'w' => 13, 'e' => 14,
        'r' => 15, 'y' => 16, 't' => 17, 'o' => 31, 'u' => 32, 'i' => 34, 'p' => 35,
        'l' => 37, 'j' => 38, 'k' => 40, 'n' => 45, 'm' => 46,
        _ => 255,
    }
}

fn type_word_with_space(word: &str) -> String {
    let mut engine = Engine::new();
    engine.set_method(0);
    let mut output = String::new();
    for ch in word.chars() {
        let key = char_to_key(ch);
        let result = engine.on_key(key, false, false);
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
    }
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
    output
}

#[test]
fn test_double_consonant_words() {
    let words = [
        "different", "officer", "off", "coffee", "office", "tiffany", "afford", "affair",
        "buffalo", "effect", "effort", "difficult", "suffer", "offer", "offend",
        "rubber", "little", "apple", "common", "tennis", "happy", "connect",
        "possible", "balloon", "accept", "access", "account", "accomplish",
        "abbreviation", "accommodate", "aggressive", "assessment", "attention",
    ];
    let mut pass = 0;
    let mut fail = 0;
    for word in &words {
        let result = type_word_with_space(word);
        let trimmed = result.trim();
        if trimmed == *word {
            pass += 1;
        } else {
            fail += 1;
            eprintln!("FAIL: '{}' -> '{}' (expected '{}')", word, trimmed, word);
        }
    }
    eprintln!("Results: {} passed, {} failed out of {}", pass, fail, words.len());
    assert!(fail == 0, "{} words failed", fail);
}
