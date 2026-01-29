use goxviet_core::data::chars::parse_char;
use goxviet_core::data::keys;
use goxviet_core::engine::Engine;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Test Vietnamese words to ensure they are correctly produced
#[test]
fn test_vietnamese_22k_pass_rate() {
    let file =
        File::open("tests/data/vietnamese_22k.txt").expect("Could not open vietnamese_22k.txt");
    let reader = BufReader::new(file);

    let mut total_count = 0;
    let mut pass_count = 0;
    let mut fail_examples = Vec::new();

    for line in reader.lines() {
        let word = line.expect("Could not read line").trim().to_string();

        // Skip empty lines, multi-word phrases, and words with hyphens
        if word.is_empty() || word.contains(' ') || word.contains('-') {
            continue;
        }

        // Skip words with non-alphabetic characters
        if word.chars().any(|c| !c.is_alphabetic()) {
            continue;
        }

        total_count += 1;

        // Generate Telex input sequence for this Vietnamese word
        let telex_input = match vietnamese_to_telex(&word) {
            Some(input) => input,
            None => {
                // Skip words we can't convert (rare edge cases)
                total_count -= 1;
                continue;
            }
        };

        let mut engine = Engine::new();
        engine.set_method(0); // Telex

        // Type the Telex sequence
        for (key, caps) in telex_input {
            engine.on_key(key, caps, false);
        }

        let result = engine.get_buffer();

        // Vietnamese words should be correctly produced
        if result == word {
            pass_count += 1;
        } else {
            if fail_examples.len() < 20 {
                fail_examples.push((word.clone(), result));
            }
        }
    }

    let pass_rate = (pass_count as f64 / total_count as f64) * 100.0;

    println!("\n=== Vietnamese 22k Pass Rate ===");
    println!("Total words tested: {}", total_count);
    println!("Passed (correct output): {}", pass_count);
    println!("Failed (incorrect output): {}", total_count - pass_count);
    println!("Pass rate: {:.2}%", pass_rate);

    if !fail_examples.is_empty() {
        println!("\nFirst {} failures:", fail_examples.len());
        for (expected, actual) in fail_examples {
            println!("  Expected '{}', got '{}'", expected, actual);
        }
    }
}

/// Convert Vietnamese word to Telex input sequence
fn vietnamese_to_telex(word: &str) -> Option<Vec<(u16, bool)>> {
    let mut result = Vec::new();
    let mut chars: Vec<char> = word.chars().collect();

    for c in chars {
        let parsed = parse_char(c)?;
        let caps = parsed.caps;

        // Add base key
        result.push((parsed.key, caps));

        // Add tone modifier (circumflex, horn, breve)
        if parsed.tone > 0 {
            let modifier_key = match (parsed.key, parsed.tone) {
                (keys::A, 1) => Some(keys::A), // aa -> â
                (keys::A, 2) => Some(keys::W), // aw -> ă
                (keys::E, 1) => Some(keys::E), // ee -> ê
                (keys::O, 1) => Some(keys::O), // oo -> ô
                (keys::O, 2) => Some(keys::W), // ow -> ơ
                (keys::U, 2) => Some(keys::W), // uw -> ư
                _ => None,
            };
            if let Some(k) = modifier_key {
                result.push((k, false));
            }
        }

        // Add stroke for đ
        if parsed.stroke {
            result.push((keys::D, false));
        }

        // Add tone mark (sắc, huyền, hỏi, ngã, nặng)
        if parsed.mark > 0 {
            let mark_key = match parsed.mark {
                1 => keys::S, // sắc
                2 => keys::F, // huyền
                3 => keys::R, // hỏi
                4 => keys::X, // ngã
                5 => keys::J, // nặng
                _ => continue,
            };
            result.push((mark_key, false));
        }
    }

    Some(result)
}
