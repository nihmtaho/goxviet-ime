//! Integration test for Vietnamese typing sequences
//!
//! Tests real-world typing scenarios to ensure proper tone mark positioning

use goxviet_core::engine::Engine;

fn process_result(buffer: &mut String, result: goxviet_core::engine::Result) {
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
}

#[test]
fn test_viet_typing_sequence() {
    // Test: v i e s e t → viết (full transformation sequence)
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    
    // Type: v, i, e (no output - normal letters)
    engine.on_key(9, false, false);   // v
    engine.on_key(34, false, false);  // i
    engine.on_key(14, false, false);  // e
    
    // Type: s (sắc) - should place mark on 'e'
    let r_s = engine.on_key(1, false, false);
    assert_eq!(r_s.action, 1, "s should apply tone mark");
    
    // Type: e again - CRITICAL: should transform e→ê and reposition mark
    let r_e = engine.on_key(14, false, false);
    assert_eq!(r_e.action, 1, "e+e should trigger circumflex with mark repositioning");
    
    // Verify the transformation output contains ế
    let output: String = r_e.chars[0..r_e.count as usize]
        .iter()
        .filter_map(|&c| char::from_u32(c))
        .collect();
    assert!(output.contains('ế'), "Should output ế (ê with sắc mark), got: '{}'", output);
    
    // Type: t (no output - normal letter)
    engine.on_key(17, false, false);
}

#[test]
fn test_hoa_typing_sequence() {
    // Test: h o a s + SPACE → hoá 
    let mut engine = Engine::new();
    engine.set_method(0);
    let mut buffer = String::new();
    
    process_result(&mut buffer, engine.on_key(4, false, false));  // H
    process_result(&mut buffer, engine.on_key(31, false, false)); // O
    process_result(&mut buffer, engine.on_key(0, false, false));  // A
    process_result(&mut buffer, engine.on_key(1, false, false));  // S (sắc)
    process_result(&mut buffer, engine.on_key(49, false, false)); // SPACE
    
    assert!(buffer.starts_with("hoá"), "Expected 'hoá', got '{}'", buffer);
}

#[test]
fn test_quoc_typing_sequence() {
    // Test: q u o o c s + SPACE → quốc (o+o→ô, mark on ô)
    let mut engine = Engine::new();
    engine.set_method(0);
    let mut buffer = String::new();
    
    process_result(&mut buffer, engine.on_key(12, false, false)); // Q
    process_result(&mut buffer, engine.on_key(32, false, false)); // U
    process_result(&mut buffer, engine.on_key(31, false, false)); // O
    process_result(&mut buffer, engine.on_key(31, false, false)); // O again (→ô)
    process_result(&mut buffer, engine.on_key(8, false, false));  // C
    process_result(&mut buffer, engine.on_key(1, false, false));  // S (sắc)
    process_result(&mut buffer, engine.on_key(49, false, false)); // SPACE
    
    assert!(buffer.starts_with("quốc"), "Expected 'quốc', got '{}'", buffer);
}

#[test]
fn test_luong_typing_sequence() {
    // Test: l u o w n g x + SPACE → lưỡng (u+w→ư, o+w→ơ, mark on ơ)
    let mut engine = Engine::new();
    engine.set_method(0);
    let mut buffer = String::new();
    
    process_result(&mut buffer, engine.on_key(37, false, false)); // L
    process_result(&mut buffer, engine.on_key(32, false, false)); // U
    process_result(&mut buffer, engine.on_key(31, false, false)); // O
    process_result(&mut buffer, engine.on_key(13, false, false)); // W (u→ư, o→ơ)
    process_result(&mut buffer, engine.on_key(45, false, false)); // N
    process_result(&mut buffer, engine.on_key(5, false, false));  // G
    process_result(&mut buffer, engine.on_key(7, false, false));  // X (ngã)
    process_result(&mut buffer, engine.on_key(49, false, false)); // SPACE
    
    assert!(buffer.starts_with("lưỡng"), "Expected 'lưỡng', got '{}'", buffer);
}

#[test]
fn test_bien_typing_sequence() {
    // Test: b i e e n r + SPACE → biển (e+e→ê, mark on ê)
    let mut engine = Engine::new();
    engine.set_method(0);
    let mut buffer = String::new();
    
    process_result(&mut buffer, engine.on_key(11, false, false)); // B
    process_result(&mut buffer, engine.on_key(34, false, false)); // I
    process_result(&mut buffer, engine.on_key(14, false, false)); // E
    process_result(&mut buffer, engine.on_key(14, false, false)); // E again (→ê)
    process_result(&mut buffer, engine.on_key(45, false, false)); // N
    process_result(&mut buffer, engine.on_key(15, false, false)); // R (hỏi)
    process_result(&mut buffer, engine.on_key(49, false, false)); // SPACE
    
    assert!(buffer.starts_with("biển"), "Expected 'biển', got '{}'", buffer);
}

#[test]
fn test_chay_typing_sequence() {
    // Test: c h a y s + SPACE → cháy (mark on 'y', Rule 2: second vowel)
    let mut engine = Engine::new();
    engine.set_method(0);
    let mut buffer = String::new();
    
    process_result(&mut buffer, engine.on_key(8, false, false));  // C
    process_result(&mut buffer, engine.on_key(4, false, false));  // H
    process_result(&mut buffer, engine.on_key(0, false, false));  // A
    process_result(&mut buffer, engine.on_key(16, false, false)); // Y
    process_result(&mut buffer, engine.on_key(1, false, false));  // S (sắc)
    process_result(&mut buffer, engine.on_key(49, false, false)); // SPACE
    
    assert!(buffer.starts_with("cháy"), "Expected 'cháy', got '{}'", buffer);
}

#[test]
fn test_mark_repositioning_on_tone_change() {
    // Critical test: Mark must reposition when vowel gets diacritic
    // Sequence: i e s → ié (mark on e, Rule 2)
    //           i é + e → iê (e→ê)
    //           i ê + s → iết (mark MUST be on ê, Rule 1: diacritic priority)
    let mut engine = Engine::new();
    engine.set_method(0);
    let mut buffer = String::new();
    
    process_result(&mut buffer, engine.on_key(34, false, false)); // I
    process_result(&mut buffer, engine.on_key(14, false, false)); // E
    process_result(&mut buffer, engine.on_key(1, false, false));  // S (sắc) → ié
    process_result(&mut buffer, engine.on_key(14, false, false)); // E again → iê with mark
    process_result(&mut buffer, engine.on_key(17, false, false)); // T
    process_result(&mut buffer, engine.on_key(49, false, false)); // SPACE
    
    assert!(buffer.starts_with("iết"), "Expected 'iết', got '{}' - mark should be on ê", buffer);
}

#[test]
fn test_viet_intermediate_states() {
    // Test detailed sequence for "viết" - verifies each transformation step
    let mut engine = Engine::new();
    engine.set_method(0);
    
    // Step 1-3: v, i, e → no output (normal letters in buffer)
    let r1 = engine.on_key(9, false, false);
    assert_eq!(r1.action, 0, "Step 1: v should not output");
    
    let r2 = engine.on_key(34, false, false);
    assert_eq!(r2.action, 0, "Step 2: i should not output");
    
    let r3 = engine.on_key(14, false, false);
    assert_eq!(r3.action, 0, "Step 3: e should not output");
    
    // Step 4: s (tone mark) → should output transformation
    let r4 = engine.on_key(1, false, false);
    assert_eq!(r4.action, 1, "Step 4: s should trigger transformation");
    assert!(r4.count > 0, "Step 4: should output characters with mark");
    
    // Step 5: e (circumflex) → should trigger e→ê transformation
    let r5 = engine.on_key(14, false, false);
    assert_eq!(r5.action, 1, "Step 5: e+e should trigger circumflex transformation");
    assert!(r5.count > 0, "Step 5: should output ê with mark repositioned");
    
    // Verify the output contains ế (ê with sắc)
    let output5: String = r5.chars[0..r5.count as usize]
        .iter()
        .filter_map(|&c| char::from_u32(c))
        .collect();
    assert!(output5.contains('ế'), "Step 5 output should contain ế, got: '{}'", output5);
    
    // Step 6: t → no output (normal letter)
    let r6 = engine.on_key(17, false, false);
    assert_eq!(r6.action, 0, "Step 6: t should not output");
}

#[test]
fn test_viese_sequence() {
    // Test the core fix: v,i,e,s,e → viế (circumflex + mark repositioning)
    let mut engine = Engine::new();
    engine.set_method(0);
    
    // Type: v, i, e (no output)
    engine.on_key(9, false, false);  // v
    engine.on_key(34, false, false); // i
    engine.on_key(14, false, false); // e
    
    // Type: s (sắc) - should apply mark
    let r_s = engine.on_key(1, false, false);
    assert_eq!(r_s.action, 1, "s should trigger mark transformation");
    
    // Type: e again - CRITICAL TEST: should add circumflex and reposition mark
    let r_e = engine.on_key(14, false, false);
    assert_eq!(r_e.action, 1, "e+e should trigger circumflex transformation (this was the bug!)");
    
    // Verify output contains ế (ê with sắc mark)
    let output: String = r_e.chars[0..r_e.count as usize]
        .iter()
        .filter_map(|&c| char::from_u32(c))
        .collect();
    assert!(output.contains('ế'), "Output should contain ế (ê with sắc), got: '{}'", output);
}

#[test]
fn test_vieset_composition() {
    // Test: v,i,e,s,e,t → viết with proper composition buffer simulation
    // Simulates how an IME composition buffer works
    let mut engine = Engine::new();
    engine.set_method(0);
    let mut composition = String::new();
    
    // Helper to apply engine result to composition buffer
    fn apply_to_composition(composition: &mut String, result: goxviet_core::engine::Result) {
        if result.action == 1 {
            // Delete backspace characters from end of composition
            if result.backspace > 0 {
                let chars_to_keep = composition.chars().count().saturating_sub(result.backspace as usize);
                *composition = composition.chars().take(chars_to_keep).collect();
            }
            // Append new characters
            if result.count > 0 {
                let new_chars: String = result.chars[0..result.count as usize]
                    .iter()
                    .filter_map(|&c| char::from_u32(c))
                    .collect();
                composition.push_str(&new_chars);
            }
        }
    }
    
    // Type: v, i, e (no output, these go into internal buffer)
    engine.on_key(9, false, false);  // v
    engine.on_key(34, false, false); // i
    engine.on_key(14, false, false); // e
    
    // Type: s (sắc) - should output transformation
    let r_s = engine.on_key(1, false, false);
    assert_eq!(r_s.action, 1, "s should trigger mark transformation");
    apply_to_composition(&mut composition, r_s);
    // Composition now has the transformed character (é or similar)
    
    // Type: e again - should add circumflex and reposition mark
    let r_e = engine.on_key(14, false, false);
    assert_eq!(r_e.action, 1, "e+e should trigger circumflex transformation");
    apply_to_composition(&mut composition, r_e);
    // Composition should now have ê with mark
    assert!(composition.contains('ế'), "Composition should contain ế, got: '{}'", composition);
    
    // Type: t (no output)
    engine.on_key(17, false, false);
    
    // Type: SPACE - commits the word
    let r_space = engine.on_key(49, false, false);
    apply_to_composition(&mut composition, r_space);
    
    // Final composition should be "viết " (or at least start with "viết")
    assert!(composition.starts_with("viết") || composition.contains("viết"), 
        "Final composition should contain 'viết', got: '{}'", composition);
}

#[test]
fn test_uoi_with_horn() {
    // Test: u o w i s + SPACE → ưới (both u and o get horn, mark on ơ - middle diacritic)
    let mut engine = Engine::new();
    engine.set_method(0);
    let mut buffer = String::new();
    
    process_result(&mut buffer, engine.on_key(32, false, false)); // U
    process_result(&mut buffer, engine.on_key(31, false, false)); // O
    process_result(&mut buffer, engine.on_key(13, false, false)); // W (u→ư, o→ơ)
    process_result(&mut buffer, engine.on_key(34, false, false)); // I
    process_result(&mut buffer, engine.on_key(1, false, false));  // S (sắc)
    process_result(&mut buffer, engine.on_key(49, false, false)); // SPACE
    
    assert!(buffer.starts_with("ưới"), "Expected 'ưới', got '{}' - mark should be on ơ (middle diacritic)", buffer);
}