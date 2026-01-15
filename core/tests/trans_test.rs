use goxviet_core::data::keys;
use goxviet_core::engine::Engine;

#[test]
fn test_trans_prefix_s() {
    let mut engine = Engine::new();

    // Type 'trans'
    engine.on_key(keys::T, false, false);
    engine.on_key(keys::R, false, false);
    engine.on_key(keys::A, false, false);
    engine.on_key(keys::N, false, false);
    engine.on_key(keys::S, false, false);

    assert_eq!(engine.get_buffer(), "trans");

    // Type 's' (tone s)
    engine.on_key(keys::S, false, false);
    assert_eq!(engine.get_buffer(), "transs");
}

#[test]
fn test_trans_f() {
    let mut engine = Engine::new();

    // Type 'trans'
    engine.on_key(keys::T, false, false);
    engine.on_key(keys::R, false, false);
    engine.on_key(keys::A, false, false);
    engine.on_key(keys::N, false, false);
    engine.on_key(keys::S, false, false);

    assert_eq!(engine.get_buffer(), "trans");

    // Type 'f' (tone f)
    engine.on_key(keys::F, false, false);
    assert_eq!(engine.get_buffer(), "transf");
}

#[test]
fn test_triple_vowels() {
    let mut engine = Engine::new();

    // aaa
    engine.on_key(keys::A, false, false); // a
    engine.on_key(keys::A, false, false); // â
    engine.on_key(keys::A, false, false); // aaa (definite English)
    assert_eq!(engine.get_buffer(), "aaa");

    engine.clear();
    // eee
    engine.on_key(keys::E, false, false); // e
    engine.on_key(keys::E, false, false); // ê
    engine.on_key(keys::E, false, false); // eee (definite English)
    assert_eq!(engine.get_buffer(), "eee");
}

#[test]
fn test_double_tone_keys() {
    let mut engine = Engine::new();

    // ss
    engine.on_key(keys::A, false, false); // a
    engine.on_key(keys::S, false, false); // á
    engine.on_key(keys::S, false, false); // ass (definite English)
    assert_eq!(engine.get_buffer(), "ass");

    engine.clear();
    // ff
    engine.on_key(keys::A, false, false); // a
    engine.on_key(keys::F, false, false); // à
    engine.on_key(keys::F, false, false); // aff (definite English)
    assert_eq!(engine.get_buffer(), "aff");
}

#[test]
fn test_double_w_after_u_o() {
    let mut engine = Engine::new();

    // uww
    engine.on_key(keys::U, false, false); // u
    engine.on_key(keys::W, false, false); // ư
    engine.on_key(keys::W, false, false); // uww (definite English)
    assert_eq!(engine.get_buffer(), "uww");

    engine.clear();
    // oww
    engine.on_key(keys::O, false, false); // o
    engine.on_key(keys::W, false, false); // ơ
    engine.on_key(keys::W, false, false); // oww (definite English)
    assert_eq!(engine.get_buffer(), "oww");
}
