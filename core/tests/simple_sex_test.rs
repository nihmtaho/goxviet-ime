//! Simple test for "se" + "x" -> "sẽ"

use goxviet_core::engine::Engine;
use goxviet_core::data::keys;

#[test]
fn test_se_x() {
    let mut e = Engine::new();
    e.set_method(0); // Telex

    println!("\n=== Test 's e x' → 'sẽ' ===");

    // Type 's'
    let r1 = e.on_key(keys::S, false, false);
    println!("After 's': action={}, buffer='{}'", r1.action, e.get_buffer());

    // Type 'e'
    let r2 = e.on_key(keys::E, false, false);
    println!("After 'e': action={}, buffer='{}'", r2.action, e.get_buffer());

    // Type 'x' (dấu ngã)
    let r3 = e.on_key(keys::X, false, false);
    println!("After 'x': action={}, backspace={}, count={}, buffer='{}'", r3.action, r3.backspace, r3.count, e.get_buffer());
    
    if r3.count > 0 {
        let chars: Vec<char> = r3.as_slice()[0..r3.count as usize]
            .iter()
            .filter_map(|&c| char::from_u32(c))
            .collect();
        println!("Result chars: {:?}", chars);
    }
    
    assert_eq!(r3.backspace, 1, "Should have backspace=1");
    assert_eq!(r3.count, 1, "Should emit 1 char");
    assert_eq!(e.get_buffer(), "sẽ", "Expected 'sẽ' but got '{}'", e.get_buffer());
}
