use goxviet_core::engine::Engine;

#[test]
fn debug_buffer_state() {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    engine.set_enabled(true);
    engine.set_modern_tone(false);
    engine.set_english_auto_restore(false);

    // Type: c-a-m
    engine.on_key(goxviet_core::data::keys::C, false, false);
    engine.on_key(goxviet_core::data::keys::A, false, false);
    engine.on_key(goxviet_core::data::keys::M, false, false);
    println!("After 'cam': buffer='{}'", engine.get_buffer());
    print_buffer_details(&engine, "cam");

    // Type: a (first 'a' - should apply backward)
    engine.on_key(goxviet_core::data::keys::A, false, false);
    println!("\nAfter first 'a': buffer='{}'", engine.get_buffer());
    print_buffer_details(&engine, "câm");

    // Type: a (second 'a' - should be rejected)
    engine.on_key(goxviet_core::data::keys::A, false, false);
    println!("\nAfter second 'a': buffer='{}'", engine.get_buffer());
    print_buffer_details(&engine, "câm (should stay same)");
}

fn print_buffer_details(engine: &Engine, label: &str) {
    println!("=== Buffer details for '{}' ===", label);
    // We can't access private buffer directly, but we can infer from output
    println!("Output: '{}'", engine.get_buffer());
}
