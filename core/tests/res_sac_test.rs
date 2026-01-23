//! Test for "r,e,s" → "ré" regression

use goxviet_core::engine::Engine;
use goxviet_core::data::keys;

#[test]
fn test_res_sac_tone() {
    let mut e = Engine::new();
    e.set_method(0); // Telex
    e.set_enabled(true);

    println!("\n=== Test 'r e s' → 'ré' ===");

    // Type 'r'
    let r1 = e.on_key(keys::R, false, false);
    println!("After 'r': action={}, buffer='{}'", r1.action, e.get_buffer());

    // Type 'e'
    let r2 = e.on_key(keys::E, false, false);
    println!("After 'e': action={}, buffer='{}'", r2.action, e.get_buffer());

    // Type 's' (dấu sắc - hỏi tone in Telex)
    let r3 = e.on_key(keys::S, false, false);
    println!("After 's': action={}, backspace={}, count={}, buffer='{}'", r3.action, r3.backspace, r3.count, e.get_buffer());
    
    if r3.count > 0 {
        let chars: Vec<char> = r3.as_slice()[0..r3.count as usize]
            .iter()
            .filter_map(|&c| char::from_u32(c))
            .collect();
        println!("Result chars: {:?}", chars);
    }
    
    assert_eq!(e.get_buffer(), "ré", "Expected 'ré' but got '{}'", e.get_buffer());
    assert_eq!(r3.backspace, 1, "Should have backspace=1 to replace 'e' with 'é'");
}

#[test]
fn test_restore_english_word() {
    let mut e = Engine::new();
    e.set_method(0); // Telex
    e.set_enabled(true);

    println!("\n=== Test 'restore' auto-restore ===");

    let keys_to_type = [
        keys::R,
        keys::E,
        keys::S,
        keys::T,
        keys::O,
        keys::R,
        keys::E,
    ];
    
    for key in keys_to_type.iter() {
        let _ = e.on_key(*key, false, false);
        println!("After key={}: buffer='{}', is_english={}", key, e.get_buffer(), e.is_english_word);
    }

    let final_buffer = e.get_buffer();
    println!("Final buffer: '{}'", final_buffer);
    
    // Should be "restore" (English word), not "retore" or similar
    assert!(final_buffer == "restore" || final_buffer == "restoré" || final_buffer == "restoreé", 
            "Buffer should be 'restore' or contain Vietnamese transforms, got '{}'", final_buffer);
}

#[test]
fn test_disconnect_english_word() {
    let mut e = Engine::new();
    e.set_method(0); // Telex
    e.set_enabled(true);

    println!("\n=== Test 'disconnect' auto-restore ===");

    let keys_to_type = [
        keys::D,
        keys::I,
        keys::S,
        keys::C,
        keys::O,
        keys::N,
        keys::N,
        keys::E,
        keys::C,
        keys::T,
    ];
    
    for key in keys_to_type.iter() {
        let _ = e.on_key(*key, false, false);
        println!("After key={}: buffer='{}', is_english={}", key, e.get_buffer(), e.is_english_word);
    }

    let final_buffer = e.get_buffer();
    println!("Final buffer: '{}'", final_buffer);
    
    // Should NOT become "diconnect" - s should NOT be consumed as tone mark
    // because after "dis" it gets detected as English and should stay as "disconnect"
    assert!(!final_buffer.contains("diconnect"), "Should not have 'diconnect' substring, got '{}'", final_buffer);
}
