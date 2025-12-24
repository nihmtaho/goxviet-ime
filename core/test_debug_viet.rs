use goxviet_core::engine::Engine;

fn main() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    
    let mut buffer = String::new();
    
    println!("\n=== Testing: v i e s e t → viết ===\n");
    
    // v
    println!("Step 1: Press 'v' (key=9)");
    let r = engine.on_key(9, false, false);
    println!("  action={} bs={} count={}", r.action, r.backspace, r.count);
    if r.backspace > 0 {
        let chars_to_remove = r.backspace as usize;
        let char_count = buffer.chars().count();
        if char_count >= chars_to_remove {
            buffer = buffer.chars().take(char_count - chars_to_remove).collect();
        }
    }
    if r.count > 0 {
        let new_chars: String = r.chars[0..r.count as usize]
            .iter()
            .filter_map(|&c| char::from_u32(c))
            .collect();
        buffer.push_str(&new_chars);
        println!("  output: '{}'", new_chars);
    }
    println!("  buffer: '{}'", buffer);
    
    // i
    println!("\nStep 2: Press 'i' (key=34)");
    let r = engine.on_key(34, false, false);
    println!("  action={} bs={} count={}", r.action, r.backspace, r.count);
    if r.backspace > 0 {
        let chars_to_remove = r.backspace as usize;
        let char_count = buffer.chars().count();
        if char_count >= chars_to_remove {
            buffer = buffer.chars().take(char_count - chars_to_remove).collect();
        }
    }
    if r.count > 0 {
        let new_chars: String = r.chars[0..r.count as usize]
            .iter()
            .filter_map(|&c| char::from_u32(c))
            .collect();
        buffer.push_str(&new_chars);
        println!("  output: '{}'", new_chars);
    }
    println!("  buffer: '{}'", buffer);
    
    // e
    println!("\nStep 3: Press 'e' (key=14)");
    let r = engine.on_key(14, false, false);
    println!("  action={} bs={} count={}", r.action, r.backspace, r.count);
    if r.backspace > 0 {
        let chars_to_remove = r.backspace as usize;
        let char_count = buffer.chars().count();
        if char_count >= chars_to_remove {
            buffer = buffer.chars().take(char_count - chars_to_remove).collect();
        }
    }
    if r.count > 0 {
        let new_chars: String = r.chars[0..r.count as usize]
            .iter()
            .filter_map(|&c| char::from_u32(c))
            .collect();
        buffer.push_str(&new_chars);
        println!("  output: '{}'", new_chars);
    }
    println!("  buffer: '{}'", buffer);
    
    // s (tone mark sắc)
    println!("\nStep 4: Press 's' (key=1) - tone mark");
    let r = engine.on_key(1, false, false);
    println!("  action={} bs={} count={}", r.action, r.backspace, r.count);
    if r.backspace > 0 {
        let chars_to_remove = r.backspace as usize;
        let char_count = buffer.chars().count();
        if char_count >= chars_to_remove {
            buffer = buffer.chars().take(char_count - chars_to_remove).collect();
        }
        println!("  backspace {} chars", r.backspace);
    }
    if r.count > 0 {
        let new_chars: String = r.chars[0..r.count as usize]
            .iter()
            .filter_map(|&c| char::from_u32(c))
            .collect();
        buffer.push_str(&new_chars);
        println!("  output: '{}'", new_chars);
    }
    println!("  buffer: '{}' (expected: 'vié')", buffer);
    
    // e again (should make ê)
    println!("\nStep 5: Press 'e' again (key=14) - should create ê");
    let r = engine.on_key(14, false, false);
    println!("  action={} bs={} count={}", r.action, r.backspace, r.count);
    if r.backspace > 0 {
        let chars_to_remove = r.backspace as usize;
        let char_count = buffer.chars().count();
        if char_count >= chars_to_remove {
            buffer = buffer.chars().take(char_count - chars_to_remove).collect();
        }
        println!("  backspace {} chars", r.backspace);
    }
    if r.count > 0 {
        let new_chars: String = r.chars[0..r.count as usize]
            .iter()
            .filter_map(|&c| char::from_u32(c))
            .collect();
        buffer.push_str(&new_chars);
        println!("  output: '{}'", new_chars);
    }
    println!("  buffer: '{}' (expected: 'viê' with mark on ê)", buffer);
    
    // t
    println!("\nStep 6: Press 't' (key=17)");
    let r = engine.on_key(17, false, false);
    println!("  action={} bs={} count={}", r.action, r.backspace, r.count);
    if r.backspace > 0 {
        let chars_to_remove = r.backspace as usize;
        let char_count = buffer.chars().count();
        if char_count >= chars_to_remove {
            buffer = buffer.chars().take(char_count - chars_to_remove).collect();
        }
    }
    if r.count > 0 {
        let new_chars: String = r.chars[0..r.count as usize]
            .iter()
            .filter_map(|&c| char::from_u32(c))
            .collect();
        buffer.push_str(&new_chars);
        println!("  output: '{}'", new_chars);
    }
    println!("  buffer: '{}' (expected: 'viết')", buffer);
    
    // SPACE to commit
    println!("\nStep 7: Press SPACE (key=49) to commit");
    let r = engine.on_key(49, false, false);
    println!("  action={} bs={} count={}", r.action, r.backspace, r.count);
    if r.backspace > 0 {
        let chars_to_remove = r.backspace as usize;
        let char_count = buffer.chars().count();
        if char_count >= chars_to_remove {
            buffer = buffer.chars().take(char_count - chars_to_remove).collect();
        }
    }
    if r.count > 0 {
        let new_chars: String = r.chars[0..r.count as usize]
            .iter()
            .filter_map(|&c| char::from_u32(c))
            .collect();
        buffer.push_str(&new_chars);
        println!("  output: '{}'", new_chars);
    }
    println!("\n=== Final buffer: '{}' ===", buffer);
    println!("Expected: 'viết '");
    
    if buffer.starts_with("viết") {
        println!("✅ TEST PASSED");
    } else {
        println!("❌ TEST FAILED");
    }
}