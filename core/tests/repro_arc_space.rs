use goxviet_core::data::keys;
/// Reproduction: Arc Browser space adding 'c'
/// Input: "đã có" + SPACE
/// Expected: "đã có "
/// Actual (reported): "đã có c"
use goxviet_core::engine::Engine;

#[test]
fn test_arc_space_bug() {
    let mut engine = Engine::new(); // Telex is default

    // Type "đã có"
    // đ = dd, ã = a + s, space, c, ó = o + s
    let sequence = vec![
        (keys::D, false),     // d
        (keys::D, false),     // d -> đ
        (keys::A, false),     // a
        (keys::S, false),     // s -> á
        (keys::SPACE, false), // space (commit "đá")
        (keys::C, false),     // c
        (keys::O, false),     // o
        (keys::S, false),     // s -> ó
    ];

    for (key, caps) in sequence {
        engine.on_key(key, caps, false);
    }

    // Space to separate words
    let result = engine.on_key(keys::SPACE, false, false);

    // Check result
    println!("DEBUG: Backspace = {}", result.backspace);
    println!("DEBUG: Count = {}", result.count);

    if result.count > 0 {
        let chars: Vec<char> =
            unsafe { std::slice::from_raw_parts(result.chars, result.count as usize) }
                .iter()
                .map(|&c| char::from_u32(c).unwrap())
                .collect();

        println!("DEBUG: Output = {:?}", chars);
        let output: String = chars.iter().collect();
        println!("DEBUG: Output string = '{}'", output);
    }
}
