//! Test "tone-immediately-after-vowel" Telex typing pattern.
//!
//! In standard Telex, the tone key comes at the END: "cacs" → "các"
//! In this "tone-early" pattern, the tone key comes AFTER the vowel (before the final consonant):
//!   "casc"    → "các"
//!   "bafn"    → "bàn"
//!   "tieesp"  → "tiếp"
//!   "dduwofng"→ "đường"
//!
//! This test verifies that the engine correctly handles this common user typing style
//! for all words in the Vietnamese 69k word list.

use goxviet_core::application::dto::EngineConfig;
use goxviet_core::domain::entities::key_event::{Action, KeyEvent};
use goxviet_core::domain::ports::input::InputMethodId;
use goxviet_core::domain::ports::transformation::ToneStrategy;
use goxviet_core::presentation::di::Container;
use std::fs;
use std::io::Write;

// ─── Helper: type a sequence of characters through the engine ────────────────

fn type_word(container: &mut Container, input: &str) -> String {
    let mut screen = String::new();
    for ch in input.chars() {
        let keycode = goxviet_core::utils::char_to_key(ch);
        let is_shift = ch.is_uppercase();
        let key_event = KeyEvent::new(keycode, is_shift, false, false, false);
        let result = {
            let arc = container.processor_service();
            let mut guard = arc.lock().unwrap();
            guard.process_key(key_event)
        };
        match result {
            Ok(r) => {
                let bs = r.backspace_count();
                let txt = r.new_text().as_str().to_string();
                let action = r.action();
                let has_transform = matches!(action, Action::Replace { .. } | Action::Insert);
                for _ in 0..bs {
                    screen.pop();
                }
                if !txt.is_empty() {
                    screen.push_str(&txt);
                } else if ch == ' ' {
                    screen.push(' ');
                } else if !has_transform {
                    screen.push(ch);
                }
            }
            Err(_) => screen.push(ch),
        }
    }
    screen
}

fn new_telex_container() -> Container {
    Container::with_config(EngineConfig {
        input_method: InputMethodId::Telex,
        tone_strategy: ToneStrategy::Modern,
        enabled: true,
        smart_mode: true,
        spell_check: false,
        auto_correct: false,
        max_history_size: 100,
        buffer_timeout_ms: 1000,
        use_modern_tone_placement: true,
        enable_shortcuts: false,
        instant_restore_enabled: true,
        esc_restore_enabled: true,
        ..Default::default()
    })
}

// ─── Vietnamese character decomposition ──────────────────────────────────────

/// Returns (base_char, mark, tone_key) for a Vietnamese character.
/// mark: 'a'=â, 'b'=ă, 'e'=ê, 'o'=ô, 'h'=ơ/ư, 'd'=đ
/// tone_key: 's'=sắc, 'f'=huyền, 'r'=hỏi, 'x'=ngã, 'j'=nặng
fn decompose_vn(c: char) -> (char, Option<char>, Option<char>) {
    match c {
        'à' => ('a', None, Some('f')),
        'á' => ('a', None, Some('s')),
        'ả' => ('a', None, Some('r')),
        'ã' => ('a', None, Some('x')),
        'ạ' => ('a', None, Some('j')),
        'è' => ('e', None, Some('f')),
        'é' => ('e', None, Some('s')),
        'ẻ' => ('e', None, Some('r')),
        'ẽ' => ('e', None, Some('x')),
        'ẹ' => ('e', None, Some('j')),
        'ì' => ('i', None, Some('f')),
        'í' => ('i', None, Some('s')),
        'ỉ' => ('i', None, Some('r')),
        'ĩ' => ('i', None, Some('x')),
        'ị' => ('i', None, Some('j')),
        'ò' => ('o', None, Some('f')),
        'ó' => ('o', None, Some('s')),
        'ỏ' => ('o', None, Some('r')),
        'õ' => ('o', None, Some('x')),
        'ọ' => ('o', None, Some('j')),
        'ù' => ('u', None, Some('f')),
        'ú' => ('u', None, Some('s')),
        'ủ' => ('u', None, Some('r')),
        'ũ' => ('u', None, Some('x')),
        'ụ' => ('u', None, Some('j')),
        'ỳ' => ('y', None, Some('f')),
        'ý' => ('y', None, Some('s')),
        'ỷ' => ('y', None, Some('r')),
        'ỹ' => ('y', None, Some('x')),
        'ỵ' => ('y', None, Some('j')),
        'â' => ('a', Some('a'), None),
        'ầ' => ('a', Some('a'), Some('f')),
        'ấ' => ('a', Some('a'), Some('s')),
        'ẩ' => ('a', Some('a'), Some('r')),
        'ẫ' => ('a', Some('a'), Some('x')),
        'ậ' => ('a', Some('a'), Some('j')),
        'ă' => ('a', Some('b'), None),
        'ằ' => ('a', Some('b'), Some('f')),
        'ắ' => ('a', Some('b'), Some('s')),
        'ẳ' => ('a', Some('b'), Some('r')),
        'ẵ' => ('a', Some('b'), Some('x')),
        'ặ' => ('a', Some('b'), Some('j')),
        'ê' => ('e', Some('e'), None),
        'ề' => ('e', Some('e'), Some('f')),
        'ế' => ('e', Some('e'), Some('s')),
        'ể' => ('e', Some('e'), Some('r')),
        'ễ' => ('e', Some('e'), Some('x')),
        'ệ' => ('e', Some('e'), Some('j')),
        'ô' => ('o', Some('o'), None),
        'ồ' => ('o', Some('o'), Some('f')),
        'ố' => ('o', Some('o'), Some('s')),
        'ổ' => ('o', Some('o'), Some('r')),
        'ỗ' => ('o', Some('o'), Some('x')),
        'ộ' => ('o', Some('o'), Some('j')),
        'ơ' => ('o', Some('h'), None),
        'ờ' => ('o', Some('h'), Some('f')),
        'ớ' => ('o', Some('h'), Some('s')),
        'ở' => ('o', Some('h'), Some('r')),
        'ỡ' => ('o', Some('h'), Some('x')),
        'ợ' => ('o', Some('h'), Some('j')),
        'ư' => ('u', Some('h'), None),
        'ừ' => ('u', Some('h'), Some('f')),
        'ứ' => ('u', Some('h'), Some('s')),
        'ử' => ('u', Some('h'), Some('r')),
        'ữ' => ('u', Some('h'), Some('x')),
        'ự' => ('u', Some('h'), Some('j')),
        'đ' => ('d', Some('d'), None),
        _ => (c, None, None),
    }
}

/// Convert a Vietnamese word to Telex input with tone key placed IMMEDIATELY
/// after the vowel sequence (before the final consonant).
///
/// Examples:
///   "các"    → "casc"    (standard: "cacs")
///   "bàn"    → "bafn"    (standard: "banf")
///   "tiếp"   → "tieesp"  (standard: "tieesp" – same since s is last vowel modifier)
///   "đường"  → "dduwofng"(standard: "dduwowngf")
///
/// For open syllables (no final consonant), output is identical to standard Telex.
fn vn_to_telex_tone_early(word: &str) -> String {
    let chars: Vec<char> = word.chars().collect();
    let mut initial_and_vowel = String::new();
    let mut final_part = String::new();
    let mut tone: Option<char> = None;
    let mut seen_vowel = false;
    let mut in_final = false;
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        let (base_char, mark, char_tone) = decompose_vn(c);
        let base_lower = base_char.to_ascii_lowercase();

        // A character is a "vowel" if its base is a/e/i/o/u/y (and it's not đ)
        let is_vowel = matches!(base_lower, 'a' | 'e' | 'i' | 'o' | 'u' | 'y') && mark != Some('d');

        // Transition into final-consonant phase: first consonant after vowels
        if !in_final && seen_vowel && !is_vowel {
            in_final = true;
        }
        if is_vowel {
            seen_vowel = true;
        }

        // Special "oo" rhyme: two consecutive plain-o chars → emit 3 o's
        // (e.g. "boong" → "booong")
        if !in_final && i + 1 < chars.len() {
            let (b1, m1, t1) = decompose_vn(c);
            let (b2, m2, _t2) = decompose_vn(chars[i + 1]);
            if b1.to_ascii_lowercase() == 'o'
                && m1.is_none()
                && b2.to_ascii_lowercase() == 'o'
                && m2.is_none()
            {
                initial_and_vowel.push(b1);
                initial_and_vowel.push(b2);
                initial_and_vowel.push('o');
                if let Some(t) = t1 {
                    tone = Some(t);
                }
                i += 2;
                seen_vowel = true;
                continue;
            }
        }

        let target = if in_final {
            &mut final_part
        } else {
            &mut initial_and_vowel
        };

        // ươ pattern: track whether the previous char was ư so we skip 'w' after ơ
        let prev_is_u_with_horn = i > 0 && {
            let (pb, pm, _) = decompose_vn(chars[i - 1]);
            pm == Some('h') && pb.to_ascii_lowercase() == 'u'
        };
        let is_o_with_horn = mark == Some('h') && base_lower == 'o';
        let is_u_with_horn = mark == Some('h') && base_lower == 'u';

        target.push(base_char);

        if let Some(m) = mark {
            match m {
                'a' => target.push('a'), // â → aa
                'b' => target.push('w'), // ă → aw
                'e' => target.push('e'), // ê → ee
                'o' => {
                    // ô → oo, but skip in ươ pattern
                    if !(prev_is_u_with_horn && is_o_with_horn) {
                        target.push('o');
                    }
                }
                'h' => {
                    if is_u_with_horn {
                        target.push('w'); // ư → uw
                    } else if is_o_with_horn && !prev_is_u_with_horn {
                        target.push('w'); // ơ → ow (only standalone)
                    }
                }
                'd' => target.push('d'), // đ → dd
                _ => {}
            }
        }

        if char_tone.is_some() {
            tone = char_tone;
        }
        i += 1;
    }

    // Combine: [initial+vowel+marks] + [tone_key] + [final_consonant]
    let mut result = initial_and_vowel;
    if let Some(t) = tone {
        result.push(t);
    }
    result.push_str(&final_part);
    result
}

// ─── Check tone style equivalence ─────────────────────────────────────────────

fn to_modern_tone_simple(word: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = word.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if i + 1 < chars.len() && (c == 'o' || c == 'u') {
            let next = chars[i + 1];
            let (new_first, new_next) = match next {
                'à' if c == 'o' => ('ò', 'a'),
                'á' if c == 'o' => ('ó', 'a'),
                'ả' if c == 'o' => ('ỏ', 'a'),
                'ã' if c == 'o' => ('õ', 'a'),
                'ạ' if c == 'o' => ('ọ', 'a'),
                'è' if c == 'o' => ('ò', 'e'),
                'é' if c == 'o' => ('ó', 'e'),
                'ẻ' if c == 'o' => ('ỏ', 'e'),
                'ẽ' if c == 'o' => ('õ', 'e'),
                'ẹ' if c == 'o' => ('ọ', 'e'),
                'ỳ' if c == 'u' => ('ù', 'y'),
                'ý' if c == 'u' => ('ú', 'y'),
                'ỷ' if c == 'u' => ('ủ', 'y'),
                'ỹ' if c == 'u' => ('ũ', 'y'),
                'ỵ' if c == 'u' => ('ụ', 'y'),
                _ => {
                    result.push(c);
                    i += 1;
                    continue;
                }
            };
            result.push(new_first);
            result.push(new_next);
            i += 2;
        } else {
            result.push(c);
            i += 1;
        }
    }
    result
}

fn words_match(expected: &str, actual: &str) -> bool {
    expected == actual || to_modern_tone_simple(expected) == to_modern_tone_simple(actual)
}

// ─── Main test ────────────────────────────────────────────────────────────────

#[test]
fn test_tone_early_from_69k() {
    let content = fs::read_to_string("tests/data/vietnamese_69k_pure.txt")
        .expect("Cannot read vietnamese_69k_pure.txt");

    // Collect single-syllable words (no spaces, non-empty, lowercase/minimal)
    let raw_words: Vec<&str> = content
        .lines()
        .map(str::trim)
        .filter(|w| !w.is_empty() && !w.contains(' '))
        .collect();

    let mut total = 0usize;
    let mut passed = 0usize;
    let mut open_total = 0usize;
    let mut open_passed = 0usize;
    let mut closed_total = 0usize;
    let mut closed_passed = 0usize;
    let mut failures: Vec<(String, String, String)> = Vec::new(); // (word, input, actual)

    // We keep a single container and reset between words by using SPACE as word boundary
    // (SPACE commits the word in the engine, so next word starts fresh)
    let mut container = new_telex_container();

    for word_raw in &raw_words {
        let word = word_raw.to_lowercase();

        // Skip words with non-Vietnamese characters (digits, special chars, etc.)
        if word
            .chars()
            .any(|c| c.is_ascii_digit() || c == '-' || c == '\'' || c == '.')
        {
            continue;
        }

        let telex_input = vn_to_telex_tone_early(&word);

        // Skip trivial cases where the conversion produces empty or unchanged input
        if telex_input.is_empty() {
            continue;
        }

        // Determine if this is a closed syllable (has final consonant)
        // A closed syllable has a consonant after the vowel cluster
        let is_closed = is_closed_syllable(&word);

        let input_with_space = format!("{} ", telex_input);
        let expected = format!("{} ", word);

        let actual = type_word(&mut container, &input_with_space);

        let ok = words_match(&expected, &actual);

        total += 1;
        if ok {
            passed += 1;
        } else {
            failures.push((word.clone(), telex_input.clone(), actual.trim().to_string()));
        }

        if is_closed {
            closed_total += 1;
            if ok {
                closed_passed += 1;
            }
        } else {
            open_total += 1;
            if ok {
                open_passed += 1;
            }
        }
    }

    // Write failures to file for inspection
    let failures_path = "tests/failures/failures_tone_early_telex.txt";
    if let Ok(mut f) = fs::File::create(failures_path) {
        let _ = writeln!(
            f,
            "# Tone-early Telex failures ({} / {} failed)",
            total - passed,
            total
        );
        let _ = writeln!(f, "# Format: WORD | TELEX_INPUT | ACTUAL_OUTPUT");
        let _ = writeln!(f, "{:<20} {:<25} {:<20}", "WORD", "TELEX_INPUT", "ACTUAL");
        let _ = writeln!(f, "{}", "-".repeat(70));
        for (word, input, actual) in &failures {
            let _ = writeln!(f, "{:<20} {:<25} {:<20}", word, input, actual);
        }
    }

    let rate = if total > 0 {
        passed as f64 / total as f64 * 100.0
    } else {
        0.0
    };
    let closed_rate = if closed_total > 0 {
        closed_passed as f64 / closed_total as f64 * 100.0
    } else {
        0.0
    };
    let open_rate = if open_total > 0 {
        open_passed as f64 / open_total as f64 * 100.0
    } else {
        0.0
    };

    println!("\n═══════════════════════════════════════════════════════════");
    println!("         TONE-EARLY TELEX TYPING TEST RESULTS             ");
    println!("═══════════════════════════════════════════════════════════");
    println!("  Pattern: tone key comes BEFORE the final consonant      ");
    println!("  Example: 'các' typed as c-a-s-c (not c-a-c-s)          ");
    println!("───────────────────────────────────────────────────────────");
    println!(
        "  ALL WORDS:     {:>6} / {:>6}  ({:.2}%)",
        passed, total, rate
    );
    println!(
        "  CLOSED (CVC):  {:>6} / {:>6}  ({:.2}%)",
        closed_passed, closed_total, closed_rate
    );
    println!(
        "  OPEN   (CV):   {:>6} / {:>6}  ({:.2}%)",
        open_passed, open_total, open_rate
    );
    println!(
        "  Failures: {} (written to {})",
        total - passed,
        failures_path
    );
    println!("───────────────────────────────────────────────────────────");
    if !failures.is_empty() {
        let show = failures.iter().take(20);
        println!("  Sample failures:");
        println!("  {:<20} {:<25} {:<20}", "WORD", "TELEX_INPUT", "ACTUAL");
        for (word, input, actual) in show {
            println!("  {:<20} {:<25} {:<20}", word, input, actual);
        }
        if failures.len() > 20 {
            println!(
                "  ... and {} more (see {})",
                failures.len() - 20,
                failures_path
            );
        }
    }
    println!("═══════════════════════════════════════════════════════════\n");

    // Soft assert: we expect a very high pass rate for closed syllables
    // The specific tone-after-vowel fix should handle the majority of cases
    assert!(
        closed_rate >= 95.0,
        "Closed-syllable (CVC) tone-early pass rate {:.2}% is below 95% threshold \
         ({} failed out of {}). See {} for details.",
        closed_rate,
        closed_total - closed_passed,
        closed_total,
        failures_path
    );
}

/// Returns true if the word has a final consonant (closed syllable).
/// A closed syllable has at least one consonant after the vowel cluster.
fn is_closed_syllable(word: &str) -> bool {
    let chars: Vec<char> = word.chars().collect();
    let mut seen_vowel = false;
    for c in &chars {
        let (base, mark, _) = decompose_vn(*c);
        let base_lower = base.to_ascii_lowercase();
        let is_vowel = matches!(base_lower, 'a' | 'e' | 'i' | 'o' | 'u' | 'y') && mark != Some('d');
        if is_vowel {
            seen_vowel = true;
        } else if seen_vowel {
            // Consonant after vowel = final consonant → closed syllable
            return true;
        }
    }
    false
}

// ─── Unit tests for the tone-early converter ─────────────────────────────────

#[cfg(test)]
mod converter_tests {
    use super::*;

    #[test]
    fn test_converter_basic() {
        // Closed syllables: tone before final consonant
        assert_eq!(vn_to_telex_tone_early("các"), "casc");
        assert_eq!(vn_to_telex_tone_early("bàn"), "bafn");
        assert_eq!(vn_to_telex_tone_early("tiếp"), "tieesp");
        assert_eq!(vn_to_telex_tone_early("đường"), "dduwofng");
        assert_eq!(vn_to_telex_tone_early("miễn"), "mieexn");
        assert_eq!(vn_to_telex_tone_early("dũng"), "duxng");
        // Open syllables: same as standard (tone at end)
        assert_eq!(vn_to_telex_tone_early("bà"), "baf");
        assert_eq!(vn_to_telex_tone_early("ca"), "ca");
        assert_eq!(vn_to_telex_tone_early("quế"), "quees");
    }

    #[test]
    fn test_engine_tone_early() {
        let mut c = new_telex_container();
        assert_eq!(type_word(&mut c, "casc "), "các ");
        let mut c = new_telex_container();
        assert_eq!(type_word(&mut c, "bafn "), "bàn ");
        let mut c = new_telex_container();
        assert_eq!(type_word(&mut c, "tieesp "), "tiếp ");
        let mut c = new_telex_container();
        assert_eq!(type_word(&mut c, "dduwofng "), "đường ");
        let mut c = new_telex_container();
        assert_eq!(type_word(&mut c, "mieexn "), "miễn ");
    }
}
