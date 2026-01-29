use goxviet_core::data::keys;
use goxviet_core::engine::Engine;

#[test]
fn test_pha_tone_only() {
    let mut e = Engine::new();
    e.set_method(0); // Telex
    e.set_enabled(true);

    e.on_key(keys::P, false, false);
    e.on_key(keys::H, false, false);
    e.on_key(keys::A, false, false);
    e.on_key(keys::S, false, false); // Should apply acute tone (sắc)

    let buffer = e.get_buffer();
    println!("Buffer after P,H,A,S: {}", buffer);
    assert_eq!(
        buffer, "phá",
        "Expected 'phá' but got '{}'. Pattern: p + h + a + s",
        buffer
    );
}
