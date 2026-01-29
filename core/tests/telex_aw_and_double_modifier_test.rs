use goxviet_core::data::keys;
use goxviet_core::engine::Engine;

/// Test case 1: [n,a,w,n,g] should transform to "năng" not "nawng"
#[test]
fn test_aw_modifier_nang() {
    let mut e = Engine::new();
    e.set_method(0); // Telex
    e.set_enabled(true);

    e.on_key(keys::N, false, false);
    e.on_key(keys::A, false, false);
    e.on_key(keys::W, false, false); // a + w = ă
    e.on_key(keys::N, false, false);
    e.on_key(keys::G, false, false);

    let buffer = e.get_buffer();
    assert_eq!(
        buffer, "năng",
        "Expected 'năng' but got '{}'. Pattern: n + a + w + n + g",
        buffer
    );
}

/// Test case 2: [l,a,w,n] should transform to "lăn" not "lawn"
#[test]
fn test_aw_modifier_lan() {
    let mut e = Engine::new();
    e.set_method(0); // Telex
    e.set_enabled(true);

    e.on_key(keys::L, false, false);
    e.on_key(keys::A, false, false);
    e.on_key(keys::W, false, false); // a + w = ă
    e.on_key(keys::N, false, false);

    let buffer = e.get_buffer();
    assert_eq!(
        buffer, "lăn",
        "Expected 'lăn' but got '{}'. Pattern: l + a + w + n",
        buffer
    );
}

/// Test case 3: [r,u,s,s,t] should auto-restore to "rust" not "russt"
/// "rust" is an English word, so double 's' should not be interpreted as two modifiers
#[test]
fn test_double_s_english_word_rust() {
    let mut e = Engine::new();
    e.set_method(0); // Telex
    e.set_enabled(true);

    e.on_key(keys::R, false, false);
    e.on_key(keys::U, false, false);
    e.on_key(keys::S, false, false); // First 's' (could be tone modifier)
    e.on_key(keys::S, false, false); // Second 's' (should be regular letter, not modifier)
    e.on_key(keys::T, false, false);

    let buffer = e.get_buffer();
    assert_eq!(
        buffer, "rust",
        "Expected 'rust' but got '{}'. Pattern: r + u + s + s + t (should auto-restore from russt)",
        buffer
    );
}

/// Test case 4: [h,o,a,w,r,c] should transform to "hoẳc" (ă with hỏi tone)
/// This tests the oa + w → oă pattern with hỏi tone mark
#[test]
fn test_oa_w_modifier_hoang_hoi() {
    let mut e = Engine::new();
    e.set_method(0); // Telex
    e.set_enabled(true);

    e.on_key(keys::H, false, false);
    e.on_key(keys::O, false, false);
    e.on_key(keys::A, false, false); // oa diphthong
    e.on_key(keys::W, false, false); // a + w = ă, creating oă pattern
    e.on_key(keys::R, false, false); // hỏi tone on ă
    e.on_key(keys::C, false, false);

    let buffer = e.get_buffer();
    assert_eq!(
        buffer, "hoẳc",
        "Expected 'hoẳc' but got '{}'. Pattern: h + o + a + w + r + c",
        buffer
    );
}

/// Test case 5: [h,o,a,w,j,c] should transform to "hoặc" (ă with nặng tone)
/// This tests the oa + w → oă pattern with nặng tone mark (the word "hoặc" = or)
#[test]
fn test_oa_w_modifier_hoac_nang() {
    let mut e = Engine::new();
    e.set_method(0); // Telex
    e.set_enabled(true);

    e.on_key(keys::H, false, false);
    e.on_key(keys::O, false, false);
    e.on_key(keys::A, false, false); // oa diphthong
    e.on_key(keys::W, false, false); // a + w = ă, creating oă pattern
    e.on_key(keys::J, false, false); // nặng tone on ă
    e.on_key(keys::C, false, false);

    let buffer = e.get_buffer();
    assert_eq!(buffer, "hoặc", 
        "Expected 'hoặc' but got '{}'. Pattern: h + o + a + w + j + c. Bug was: extra 'a' not removed before inserting 'ă'", buffer);
}

/// Test case 6: [l,a,w,w] should revert to "law" not "lăw"
/// The second 'w' should cancel the breve applied by the first 'w'
#[test]
fn test_aw_double_modifier_revert_laww() {
    let mut e = Engine::new();
    e.set_method(0); // Telex
    e.set_enabled(true);

    e.on_key(keys::L, false, false);
    e.on_key(keys::A, false, false);
    e.on_key(keys::W, false, false); // a + w = ă
    e.on_key(keys::W, false, false); // ă + w = a (revert)

    let buffer = e.get_buffer();
    assert_eq!(
        buffer, "law",
        "Expected 'law' but got '{}'. Pattern: l + a + w + w (second w should revert)",
        buffer
    );
}
