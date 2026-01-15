// Test for "tree" Vietnamese typing issue
use goxviet_core::data::keys;
use goxviet_core::engine::Engine;

#[test]
fn test_tree_vietnamese_typing() {
    let mut e = Engine::new();
    e.set_method(0); // Telex
    e.set_enabled(true);

    // Type "tree" - should allow Vietnamese "trê" transformation
    e.on_key(keys::T, false, false);
    assert_eq!(e.get_buffer(), "t");

    e.on_key(keys::R, false, false);
    assert_eq!(e.get_buffer(), "tr");

    e.on_key(keys::E, false, false);
    assert_eq!(e.get_buffer(), "tre");

    // Second 'e' should transform to 'ê'
    e.on_key(keys::E, false, false);
    assert_eq!(
        e.get_buffer(),
        "trê",
        "'tree' should transform to 'trê' in Vietnamese"
    );
}
