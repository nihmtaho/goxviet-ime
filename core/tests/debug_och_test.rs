// Debug test to understand the flow
use goxviet_core::data::keys;
use goxviet_core::engine::Engine;

#[test]
fn debug_och_flow() {
    let mut e = Engine::new();
    e.set_method(0); // Telex
    e.set_enabled(true);

    // Type step by step and see what happens
    println!("\n=== Typing 'ochaa' step by step ===");

    e.on_key(keys::O, false, false);
    println!("After 'o': {:?}", e.get_buffer());

    e.on_key(keys::C, false, false);
    println!("After 'c': {:?}", e.get_buffer());

    e.on_key(keys::H, false, false);
    println!("After 'h': {:?}", e.get_buffer());

    e.on_key(keys::A, false, false);
    println!("After 'a': {:?}", e.get_buffer());

    e.on_key(keys::A, false, false);
    println!("After 'aa': {:?}", e.get_buffer());
}

#[test]
fn debug_ach_flow() {
    let mut e = Engine::new();
    e.set_method(0); // Telex
    e.set_enabled(true);

    println!("\n=== Typing 'achaa' step by step ===");

    e.on_key(keys::A, false, false);
    println!("After 'a': {:?}", e.get_buffer());

    e.on_key(keys::C, false, false);
    println!("After 'c': {:?}", e.get_buffer());

    e.on_key(keys::H, false, false);
    println!("After 'h': {:?}", e.get_buffer());

    e.on_key(keys::A, false, false);
    println!("After 'a': {:?}", e.get_buffer());

    e.on_key(keys::A, false, false);
    println!("After 'aa': {:?}", e.get_buffer());
}
