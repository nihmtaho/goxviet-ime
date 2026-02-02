use goxviet_core::engine::Engine;

#[test]
fn test_basic_typing() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);

    // Type 's'
    let result_s = engine.on_key(1, false, false); // s = key 1
    println!("After 's': backspace={}, count={}, buf={:?}", result_s.backspace, result_s.count, engine.get_buffer());

    // Type 'a'
    let result_a = engine.on_key(0, false, false); // a = key 0
    println!("After 'a': backspace={}, count={}, buf={:?}", result_a.backspace, result_a.count, engine.get_buffer());

    // Type 'n'
    let result_n = engine.on_key(45, false, false); // n = key 45
    println!("After 'n': backspace={}, count={}, buf={:?}", result_n.backspace, result_n.count, engine.get_buffer());

    println!("Final buffer: '{}'", engine.get_buffer());
}
