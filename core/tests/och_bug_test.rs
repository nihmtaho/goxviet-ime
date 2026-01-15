// Test case for the bug: "ochaa" should not transform to "ochâ"
use goxviet_core::engine::Engine;
use goxviet_core::utils::type_word;

#[test]
fn test_och_invalid_combination() {
    let mut e = Engine::new();
    e.set_method(0); // Telex
    e.set_enabled(true);

    // "ochaa" should stay as "ochaa" because "och" is invalid Vietnamese
    // o + ch is not allowed in Vietnamese
    let result = type_word(&mut e, "ochaa");
    assert_eq!(
        result, "ochaa",
        "'ochaa' should not transform because 'och' is invalid Vietnamese"
    );
}

#[test]
fn test_ach_valid_combination() {
    let mut e = Engine::new();
    e.set_method(0); // Telex
    e.set_enabled(true);

    // "achaa" should transform to "achâ" because "ach" is valid Vietnamese
    let result = type_word(&mut e, "achaa");
    assert_eq!(
        result, "achâ",
        "'achaa' should transform to 'achâ' because 'ach' is valid Vietnamese"
    );
}
