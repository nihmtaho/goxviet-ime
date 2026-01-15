use goxviet_core::data::keys;
use goxviet_core::engine::Engine;
use goxviet_core::engine_v2::english::dictionary::Dictionary;

#[test]
fn test_dictionary_push() {
    // Test dictionary directly with keycodes
    let keys_push = vec![keys::P, keys::U, keys::S, keys::H];
    let is_eng = Dictionary::is_english(&keys_push);
    println!("push keycodes: {:?}", keys_push);
    println!("is_english: {}", is_eng);
    assert!(is_eng, "push should be in English dictionary");
}

#[test]
fn test_dictionary_merge() {
    let keys_merge = vec![keys::M, keys::E, keys::R, keys::G, keys::E];
    let is_eng = Dictionary::is_english(&keys_merge);
    println!("merge keycodes: {:?}", keys_merge);
    println!("is_english: {}", is_eng);
    assert!(is_eng, "merge should be in English dictionary");
}

#[test]
fn test_dictionary_basic() {
    let keys_basic = vec![keys::B, keys::A, keys::S, keys::I, keys::C];
    let is_eng = Dictionary::is_english(&keys_basic);
    println!("basic keycodes: {:?}", keys_basic);
    println!("is_english: {}", is_eng);
    // basic is NOT in dictionary, but should be detected via phonotactic -ic suffix
}

#[test]
fn test_push_with_transforms() {
    let mut e = Engine::new();
    e.set_method(0); // Telex
    e.set_enabled(true);

    // Type "push" key by key and check buffer after each
    e.on_key_ext(keys::P, false, false, false);
    println!("After P: '{}'", e.get_buffer());

    e.on_key_ext(keys::U, false, false, false);
    println!("After U: '{}'", e.get_buffer());

    e.on_key_ext(keys::S, false, false, false);
    println!("After S: '{}'", e.get_buffer());

    e.on_key_ext(keys::H, false, false, false);
    println!("After H: '{}'", e.get_buffer());

    let output = e.get_buffer();
    assert_eq!(output, "push", "push should remain as English");
}

#[test]
fn test_merge_with_transforms() {
    let mut e = Engine::new();
    e.set_method(0); // Telex
    e.set_enabled(true);

    // Type "merge" key by key and check buffer after each
    e.on_key_ext(keys::M, false, false, false);
    println!("After M: '{}'", e.get_buffer());

    e.on_key_ext(keys::E, false, false, false);
    println!("After E: '{}'", e.get_buffer());

    e.on_key_ext(keys::R, false, false, false);
    println!("After R (should apply hỏi?): '{}'", e.get_buffer());

    e.on_key_ext(keys::G, false, false, false);
    println!("After G: '{}'", e.get_buffer());

    e.on_key_ext(keys::E, false, false, false);
    println!("After E: '{}'", e.get_buffer());

    let output = e.get_buffer();
    assert_eq!(output, "merge", "merge should be auto-restored as English");
}
