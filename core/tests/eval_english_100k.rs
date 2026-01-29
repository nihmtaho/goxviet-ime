use goxviet_core::engine::Engine;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Test English words to ensure they are NOT transformed by the Vietnamese engine
#[test]
fn test_english_100k_pass_rate() {
    let file = File::open("tests/data/english_100k.txt").expect("Could not open english_100k.txt");
    let reader = BufReader::new(file);

    let mut total_count = 0;
    let mut pass_count = 0;
    let mut fail_examples = Vec::new();

    for line in reader.lines() {
        let word = line.expect("Could not read line").trim().to_string();

        // Skip empty lines, single letters, and words with punctuation/numbers
        if word.is_empty() || word.len() == 1 || word.chars().any(|c| !c.is_ascii_alphabetic()) {
            continue;
        }

        total_count += 1;

        let mut engine = Engine::new();
        engine.set_method(0); // Telex

        // Type the word
        for c in word.chars() {
            let key = char_to_key(c);
            engine.on_key(key, c.is_ascii_uppercase(), false);
        }

        let result = engine.get_buffer();

        // English words should remain unchanged (no Vietnamese transformations)
        if result == word {
            pass_count += 1;
        } else {
            if fail_examples.len() < 20 {
                fail_examples.push((word.clone(), result));
            }
        }
    }

    let pass_rate = (pass_count as f64 / total_count as f64) * 100.0;

    println!("\n=== English 100k Pass Rate ===");
    println!("Total words tested: {}", total_count);
    println!("Passed (unchanged): {}", pass_count);
    println!("Failed (transformed): {}", total_count - pass_count);
    println!("Pass rate: {:.2}%", pass_rate);

    if !fail_examples.is_empty() {
        println!("\nFirst {} failures:", fail_examples.len());
        for (input, output) in fail_examples {
            println!("  '{}' → '{}'", input, output);
        }
    }
}

fn char_to_key(c: char) -> u16 {
    use goxviet_core::data::keys;
    match c.to_ascii_lowercase() {
        'a' => keys::A,
        'b' => keys::B,
        'c' => keys::C,
        'd' => keys::D,
        'e' => keys::E,
        'f' => keys::F,
        'g' => keys::G,
        'h' => keys::H,
        'i' => keys::I,
        'j' => keys::J,
        'k' => keys::K,
        'l' => keys::L,
        'm' => keys::M,
        'n' => keys::N,
        'o' => keys::O,
        'p' => keys::P,
        'q' => keys::Q,
        'r' => keys::R,
        's' => keys::S,
        't' => keys::T,
        'u' => keys::U,
        'v' => keys::V,
        'w' => keys::W,
        'x' => keys::X,
        'y' => keys::Y,
        'z' => keys::Z,
        _ => 255,
    }
}
