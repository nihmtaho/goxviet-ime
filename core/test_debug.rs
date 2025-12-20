use vietnamese_ime_core::engine::Engine;

fn main() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    
    // Test 1: Simple word
    println!("\n=== Test 1: Simple word ===");
    let r1 = engine.on_key_ext(17, false, false, false); // t
    println!("t: action={} bs={} count={}", r1.action, r1.backspace, r1.count);
    
    let r2 = engine.on_key_ext(4, false, false, false); // h
    println!("h: action={} bs={} count={}", r2.action, r2.backspace, r2.count);
    
    // Backspace
    let rb = engine.on_key_ext(51, false, false, false); // DELETE
    println!("DELETE: action={} bs={} count={}", rb.action, rb.backspace, rb.count);
    for i in 0..rb.count {
        if let Some(ch) = char::from_u32(rb.chars[i as usize]) {
            print!("{}", ch);
        }
    }
    println!();
}
