//! Verify triple-tone fix meets user requirements:
//! 1. When typing "a-s-s-s" → display should be "ass" (not "asss")
//! 2. When typing "a-s-s-e-t" → display should be "asset" (double-key revert works)
//! 3. When typing "a-s-s-s-e-t" + SPACE → output should be "asset " (SPACE boundary correction)

use goxviet_core::data::keys;
use goxviet_core::engine::Engine;

fn char_to_key(c: char) -> u16 {
    match c.to_ascii_lowercase() {
        'a' => 0, 's' => 1, 'd' => 2, 'f' => 3, 'h' => 4, 'g' => 5, 'z' => 6, 'x' => 7,
        'c' => 8, 'v' => 9, 'b' => 11, 'q' => 12, 'w' => 13, 'e' => 14, 'r' => 15, 'y' => 16,
        't' => 17, 'o' => 31, 'u' => 32, 'i' => 34, 'p' => 35, 'l' => 37, 'j' => 38, 'k' => 40,
        'n' => 45, 'm' => 46, _ => 255,
    }
}

fn get_display_at_keystroke(keystrokes: &str, stop_at_pos: usize) -> String {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex
    let mut display = String::new();

    for (i, ch) in keystrokes.chars().enumerate() {
        if i >= stop_at_pos {
            break;
        }
        let key = char_to_key(ch);
        let caps = ch.is_ascii_uppercase();
        let result = engine.on_key(key, caps, false);

        if result.action == 1 {
            let backspace_count = result.backspace as usize;
            for _ in 0..backspace_count.min(display.len()) {
                display.pop();
            }
            for j in 0..result.count as usize {
                if let Some(c) = char::from_u32(result.as_slice()[j]) {
                    display.push(c);
                }
            }
        } else {
            display.push(ch);
        }
    }
    display
}

fn type_then_space(keystrokes: &str) -> Option<String> {
    let mut engine = Engine::new();
    engine.set_method(0); // Telex

    for ch in keystrokes.chars() {
        let key = char_to_key(ch);
        engine.on_key_ext(key, ch.is_ascii_uppercase(), false, false);
    }

    let r = engine.on_key_ext(keys::SPACE, false, false, false);
    if r.action == 1 {
        let output: String = (0..r.count as usize)
            .filter_map(|i| char::from_u32(r.as_slice()[i]))
            .collect();
        Some(output)
    } else {
        None
    }
}

/// Requirement 1: "a-s-s-s" should display as "ass" (not "asss")
#[test]
fn test_requirement_1_triple_s_display() {
    let display = get_display_at_keystroke("asss", 4);
    assert_eq!(
        display, "ass",
        "Requirement 1 FAILED: 'a-s-s-s' should display as 'ass', got '{}'",
        display
    );
}

/// Requirement 2: "a-s-s-e-t" should display as "asset"
#[test]
fn test_requirement_2_double_s_asset() {
    let display = get_display_at_keystroke("asset", 5);
    assert_eq!(
        display, "asset",
        "Requirement 2 FAILED: 'a-s-s-e-t' should display as 'asset', got '{}'",
        display
    );
}

/// Requirement 3: "a-s-s-s-e-t" + SPACE should output "asset "
#[test]
fn test_requirement_3_triple_s_space_correction() {
    let result = type_then_space("assset");
    assert_eq!(
        result,
        Some("asset ".to_string()),
        "Requirement 3 FAILED: 'a-s-s-s-e-t' + SPACE should output 'asset ', got {:?}",
        result
    );
}

/// Verify same logic works for other tone markers (f, r, etc)
#[test]
fn test_other_tone_markers() {
    // Triple-f should display as "aff" not "afff"
    let display_f = get_display_at_keystroke("afff", 4);
    assert_eq!(display_f, "aff");

    // Triple-r should display as "arr" not "arrr"
    let display_r = get_display_at_keystroke("arrr", 4);
    assert_eq!(display_r, "arr");
}
