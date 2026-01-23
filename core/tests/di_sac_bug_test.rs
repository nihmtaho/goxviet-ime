//! Test case for bug: typing "d i s" should produce "dí" not "dis"

use goxviet_core::engine::Engine;
use goxviet_core::data::keys;

#[test]
fn test_di_with_sac_tone() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex

    println!("\n=== Test 'd i s' → 'dí' ===");

    // Type 'd'
    let r1 = engine.on_key(keys::D, false, false);
    println!("After 'd': action={}, buffer='{}'", r1.action, engine.get_buffer());
    assert_eq!(r1.action, 0, "d should pass through");

    // Type 'i'
    let r2 = engine.on_key(keys::I, false, false);
    println!("After 'i': action={}, buffer='{}'", r2.action, engine.get_buffer());
    assert_eq!(r2.action, 0, "i should pass through");

    // Type 's' (sắc tone mark)
    let r3 = engine.on_key(keys::S, false, false);
    println!("After 's': action={}, buffer='{}'", r3.action, engine.get_buffer());
        println!("Result: action={}, backspace={}, count={}", r3.action, r3.backspace, r3.count);
        if r3.count > 0 {
            let chars: Vec<char> = r3.as_slice()[0..r3.count as usize]
                .iter()
                .filter_map(|&c| char::from_u32(c))
                .collect();
            println!("Result chars: {:?}", chars);
        }
    
    // BUG: This should be action=1 (transform), not 0 (pass through)
    assert_eq!(r3.action, 1, "'d i s' should trigger tone mark transformation");
    assert_eq!(engine.get_buffer(), "dí", "Expected 'dí' but got '{}'", engine.get_buffer());
}
