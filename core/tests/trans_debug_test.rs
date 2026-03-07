use goxviet_core::data::keys;
use goxviet_core::engine::Engine;

#[test]
fn test_trans_step_by_step() {
    let mut engine = Engine::new();

    // Type 't'
    let r1 = engine.on_key(keys::T, false, false);
    println!("Step 1 - After 't': buffer='{}'", engine.get_buffer());

    // Type 'r'
    let r2 = engine.on_key(keys::R, false, false);
    println!("Step 2 - After 'r': buffer='{}'", engine.get_buffer());

    // Type 'a'
    let r3 = engine.on_key(keys::A, false, false);
    println!("Step 3 - After 'a': buffer='{}'", engine.get_buffer());

    // Type 'n'
    let r4 = engine.on_key(keys::N, false, false);
    println!("Step 4 - After 'n': buffer='{}'", engine.get_buffer());

    // Type 's' - THIS SHOULD TRIGGER ENGLISH DETECTION
    let r5 = engine.on_key(keys::S, false, false);
    println!("Step 5 - After 's': buffer='{}'", engine.get_buffer());
    assert_eq!(
        engine.get_buffer(),
        "trans",
        "After 's', buffer should be 'trans'"
    );

    // Type 'f' - THIS SHOULD BE BLOCKED BY is_english_word FLAG
    let r6 = engine.on_key(keys::F, false, false);
    println!("Step 6 - After 'f': buffer='{}'", engine.get_buffer());

    // CRITICAL ASSERTION
    assert_eq!(
        engine.get_buffer(),
        "transf",
        "FAILED: After 'f', buffer should be 'transf' (not 'tràns' or 'trànsf').\n\
         This means is_english_word was NOT set to true when typing 'trans'."
    );
}

#[test]
fn debug_risch() {
    use goxviet_core::data::keys;
    use goxviet_core::engine::{Action, Engine};

    // r+i+s+c+h → expected "rích"
    let mut e = Engine::new();
    let chars = vec![('r', keys::R), ('i', keys::I), ('s', keys::S), ('c', keys::C), ('h', keys::H)];
    let mut screen = String::new();
    for (ch, key) in chars {
        let r = e.on_key(key, false, false);
        let action = r.action;
        let bs = r.backspace;
        let count = r.count;
        let text: String = (0..count as usize).filter_map(|i| {
            unsafe { char::from_u32(*r.chars.offset(i as isize)) }
        }).collect();
        println!("key='{}' action={} bs={} text='{}' buf='{}' screen_before='{}'",
            ch, action, bs, text, e.get_buffer(), screen);
        if action == Action::Send as u8 {
            for _ in 0..bs { screen.pop(); }
            screen.push_str(&text);
        } else {
            screen.push(ch);
        }
        println!("  screen_after='{}'", screen);
    }
    println!("FINAL: screen='{}'", screen);
    assert_eq!(screen, "rích", "Expected 'rích'");
}

#[test]
fn debug_huyjch() {
    use goxviet_core::data::keys;
    use goxviet_core::engine::{Action, Engine};

    // h+u+y+j+c+h → expected "huỵch"
    let mut e = Engine::new();
    let chars = vec![('h', keys::H), ('u', keys::U), ('y', keys::Y), ('j', keys::J), ('c', keys::C), ('h', keys::H)];
    let mut screen = String::new();
    for (ch, key) in chars {
        let r = e.on_key(key, false, false);
        let action = r.action;
        let bs = r.backspace;
        let count = r.count;
        let text: String = (0..count as usize).filter_map(|i| {
            unsafe { char::from_u32(*r.chars.offset(i as isize)) }
        }).collect();
        println!("key='{}' action={} bs={} text='{}' buf='{}' screen_before='{}'",
            ch, action, bs, text, e.get_buffer(), screen);
        if action == Action::Send as u8 {
            for _ in 0..bs { screen.pop(); }
            screen.push_str(&text);
        } else {
            screen.push(ch);
        }
        println!("  screen_after='{}'", screen);
    }
    println!("FINAL: screen='{}'", screen);
    assert_eq!(screen, "huỵch", "Expected 'huỵch'");
}

#[test]
fn debug_coxng() {
    use goxviet_core::data::keys;
    use goxviet_core::engine::{Action, Engine};

    // c+o+x+n+g → expected "cõng"
    let mut e = Engine::new();
    let chars = vec![('c', keys::C), ('o', keys::O), ('x', keys::X), ('n', keys::N), ('g', keys::G)];
    let mut screen = String::new();
    for (ch, key) in chars {
        let r = e.on_key(key, false, false);
        let action = r.action;
        let bs = r.backspace;
        let count = r.count;
        let text: String = (0..count as usize).filter_map(|i| {
            unsafe { char::from_u32(*r.chars.offset(i as isize)) }
        }).collect();
        println!("key='{}' action={} bs={} text='{}' buf='{}' screen_before='{}'",
            ch, action, bs, text, e.get_buffer(), screen);
        if action == Action::Send as u8 {
            for _ in 0..bs { screen.pop(); }
            screen.push_str(&text);
        } else {
            screen.push(ch);
        }
        println!("  screen_after='{}'", screen);
    }
    println!("FINAL: screen='{}'", screen);
    assert_eq!(screen, "cõng", "Expected 'cõng'");
}

#[test]
fn debug_tone_early_gia() {
    use goxviet_core::data::keys;
    use goxviet_core::engine::{Action, Engine};
    use goxviet_core::utils::type_word;

    // Step-by-step for "giasc" → expected "giác"
    let mut e = Engine::new();
    let chars = vec![('g', keys::G), ('i', keys::I), ('a', keys::A), ('s', keys::S), ('c', keys::C)];
    let mut screen = String::new();
    for (ch, key) in chars {
        let r = e.on_key(key, false, false);
        let action = r.action;
        let bs = r.backspace;
        let count = r.count;
        let text: String = (0..count as usize).filter_map(|i| {
            unsafe { char::from_u32(*r.chars.offset(i as isize)) }
        }).collect();
        println!("key='{}' action={} bs={} text='{}' buf='{}' screen_before='{}'",
            ch, action, bs, text, e.get_buffer(), screen);
        if action == Action::Send as u8 {
            for _ in 0..bs { screen.pop(); }
            screen.push_str(&text);
        } else {
            screen.push(ch);
        }
        println!("  screen_after='{}'", screen);
    }
    println!("FINAL: screen='{}'", screen);
    assert_eq!(screen, "giác", "Expected 'giác'");
}
