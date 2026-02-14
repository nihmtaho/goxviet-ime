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

fn main() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    
    let word = "received";
    let mut output = String::new();
    
    for (step, ch) in word.chars().enumerate() {
        let key = char_to_key(ch);
        let result = engine.on_key(key, false, false);
        
        let action = result.action;
        let bs = result.backspace;
        let count = result.count;
        
        if action == 1 {
            let before = output.clone();
            for _ in 0..bs.min(output.len() as u8) {
                output.pop();
            }
            let after_bs = output.clone();
            for i in 0..count as usize {
                unsafe {
                    if let Some(c) = char::from_u32(*result.chars.offset(i as isize)) {
                        output.push(c);
                    }
                }
            }
            eprintln!("Step {} '{}': action={}, bs={}, count={} | '{}' -> bs -> '{}' -> chars -> '{}'",
                step, ch, action, bs, count, before, after_bs, output);
        } else {
            output.push(ch);
            eprintln!("Step {} '{}': action={} (passthrough) | output='{}'", step, ch, action, output);
        }
    }
    
    // Space
    let result = engine.on_key(49, false, false);
    if result.action == 1 {
        let before = output.clone();
        for _ in 0..result.backspace.min(output.len() as u8) {
            output.pop();
        }
        for i in 0..result.count as usize {
            unsafe {
                if let Some(c) = char::from_u32(*result.chars.offset(i as isize)) {
                    output.push(c);
                }
            }
        }
        eprintln!("Space: action=1, bs={}, count={} | '{}' -> '{}'", result.backspace, result.count, before, output);
    } else {
        output.push(' ');
        eprintln!("Space: passthrough | output='{}'", output);
    }
    
    println!("FINAL: '{}' -> '{}'", word, output.trim());
}
