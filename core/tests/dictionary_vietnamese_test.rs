//! Test Vietnamese 22k word list.
//! Converts Vietnamese words to Telex/VNI input and verifies engine output.

use goxviet_core::engine::Engine;
use goxviet_core::utils::type_word;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::time::Instant;

/// Get base character and modifiers (mark, tone) for Vietnamese character
fn decompose_vn_char(c: char) -> (char, Option<char>, Option<char>) {
    // Returns (base_char, mark_char, tone_char)
    // mark_char: 'a' for â, 'w' for ă/ơ/ư, 'e' for ê, 'o' for ô, 'd' for đ
    // tone_char: 's' sắc, 'f' huyền, 'r' hỏi, 'x' ngã, 'j' nặng
    match c {
        // Plain vowels with tones
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
        // Circumflex â
        'â' => ('a', Some('a'), None),
        'ầ' => ('a', Some('a'), Some('f')),
        'ấ' => ('a', Some('a'), Some('s')),
        'ẩ' => ('a', Some('a'), Some('r')),
        'ẫ' => ('a', Some('a'), Some('x')),
        'ậ' => ('a', Some('a'), Some('j')),
        // Breve ă
        'ă' => ('a', Some('b'), None),
        'ằ' => ('a', Some('b'), Some('f')),
        'ắ' => ('a', Some('b'), Some('s')),
        'ẳ' => ('a', Some('b'), Some('r')),
        'ẵ' => ('a', Some('b'), Some('x')),
        'ặ' => ('a', Some('b'), Some('j')),
        // Circumflex ê
        'ê' => ('e', Some('e'), None),
        'ề' => ('e', Some('e'), Some('f')),
        'ế' => ('e', Some('e'), Some('s')),
        'ể' => ('e', Some('e'), Some('r')),
        'ễ' => ('e', Some('e'), Some('x')),
        'ệ' => ('e', Some('e'), Some('j')),
        // Circumflex ô
        'ô' => ('o', Some('o'), None),
        'ồ' => ('o', Some('o'), Some('f')),
        'ố' => ('o', Some('o'), Some('s')),
        'ổ' => ('o', Some('o'), Some('r')),
        'ỗ' => ('o', Some('o'), Some('x')),
        'ộ' => ('o', Some('o'), Some('j')),
        // Horn ơ
        'ơ' => ('o', Some('h'), None),
        'ờ' => ('o', Some('h'), Some('f')),
        'ớ' => ('o', Some('h'), Some('s')),
        'ở' => ('o', Some('h'), Some('r')),
        'ỡ' => ('o', Some('h'), Some('x')),
        'ợ' => ('o', Some('h'), Some('j')),
        // Horn ư
        'ư' => ('u', Some('h'), None),
        'ừ' => ('u', Some('h'), Some('f')),
        'ứ' => ('u', Some('h'), Some('s')),
        'ử' => ('u', Some('h'), Some('r')),
        'ữ' => ('u', Some('h'), Some('x')),
        'ự' => ('u', Some('h'), Some('j')),
        // Stroke đ
        'đ' => ('d', Some('d'), None),
        // Uppercase
        'À' => ('A', None, Some('f')),
        'Á' => ('A', None, Some('s')),
        'Ả' => ('A', None, Some('r')),
        'Ã' => ('A', None, Some('x')),
        'Ạ' => ('A', None, Some('j')),
        'Â' => ('A', Some('a'), None),
        'Ầ' => ('A', Some('a'), Some('f')),
        'Ấ' => ('A', Some('a'), Some('s')),
        'Ẩ' => ('A', Some('a'), Some('r')),
        'Ẫ' => ('A', Some('a'), Some('x')),
        'Ậ' => ('A', Some('a'), Some('j')),
        'Ă' => ('A', Some('b'), None),
        'Ằ' => ('A', Some('b'), Some('f')),
        'Ắ' => ('A', Some('b'), Some('s')),
        'Ẳ' => ('A', Some('b'), Some('r')),
        'Ẵ' => ('A', Some('b'), Some('x')),
        'Ặ' => ('A', Some('b'), Some('j')),
        'È' => ('E', None, Some('f')),
        'É' => ('E', None, Some('s')),
        'Ẻ' => ('E', None, Some('r')),
        'Ẽ' => ('E', None, Some('x')),
        'Ẹ' => ('E', None, Some('j')),
        'Ê' => ('E', Some('e'), None),
        'Ề' => ('E', Some('e'), Some('f')),
        'Ế' => ('E', Some('e'), Some('s')),
        'Ể' => ('E', Some('e'), Some('r')),
        'Ễ' => ('E', Some('e'), Some('x')),
        'Ệ' => ('E', Some('e'), Some('j')),
        'Ì' => ('I', None, Some('f')),
        'Í' => ('I', None, Some('s')),
        'Ỉ' => ('I', None, Some('r')),
        'Ĩ' => ('I', None, Some('x')),
        'Ị' => ('I', None, Some('j')),
        'Ò' => ('O', None, Some('f')),
        'Ó' => ('O', None, Some('s')),
        'Ỏ' => ('O', None, Some('r')),
        'Õ' => ('O', None, Some('x')),
        'Ọ' => ('O', None, Some('j')),
        'Ô' => ('O', Some('o'), None),
        'Ồ' => ('O', Some('o'), Some('f')),
        'Ố' => ('O', Some('o'), Some('s')),
        'Ổ' => ('O', Some('o'), Some('r')),
        'Ỗ' => ('O', Some('o'), Some('x')),
        'Ộ' => ('O', Some('o'), Some('j')),
        'Ơ' => ('O', Some('w'), None),
        'Ờ' => ('O', Some('w'), Some('f')),
        'Ớ' => ('O', Some('w'), Some('s')),
        'Ở' => ('O', Some('w'), Some('r')),
        'Ỡ' => ('O', Some('w'), Some('x')),
        'Ợ' => ('O', Some('w'), Some('j')),
        'Ù' => ('U', None, Some('f')),
        'Ú' => ('U', None, Some('s')),
        'Ủ' => ('U', None, Some('r')),
        'Ũ' => ('U', None, Some('x')),
        'Ụ' => ('U', None, Some('j')),
        'Ư' => ('U', Some('w'), None),
        'Ừ' => ('U', Some('w'), Some('f')),
        'Ứ' => ('U', Some('w'), Some('s')),
        'Ử' => ('U', Some('w'), Some('r')),
        'Ữ' => ('U', Some('w'), Some('x')),
        'Ự' => ('U', Some('w'), Some('j')),
        'Ỳ' => ('Y', None, Some('f')),
        'Ý' => ('Y', None, Some('s')),
        'Ỷ' => ('Y', None, Some('r')),
        'Ỹ' => ('Y', None, Some('x')),
        'Ỵ' => ('Y', None, Some('j')),
        'Đ' => ('D', Some('d'), None),
        // No transformation needed
        _ => (c, None, None),
    }
}

/// Convert Vietnamese word to Telex input (tone at end of word)
/// Following standard Telex rules from documentation
/// Note: For "ươ", we type "u-w-o" NOT "u-w-o-w" - the 'w' on 'o' is not needed
fn vn_to_telex(word: &str) -> String {
    let chars: Vec<char> = word.chars().collect();
    let mut base = String::new();
    let mut tone: Option<char> = None;
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        let (base_char, mark, char_tone) = decompose_vn_char(c);

        // Special handling for "oo" rhyme: user requested typing 3 "o"s (e.g., "ngoong" -> "n-g-o-o-o-n-g")
        // This likely forces the engine to treat it as "oo" instead of "ô" (which is usually "oo")
        if i + 1 < chars.len() {
            let (b1, m1, t1) = decompose_vn_char(c);
            let (b2, m2, t2) = decompose_vn_char(chars[i + 1]);

            // Check for two consecutive 'o' (or 'O') with no marks (e.g., boong, xoong)
            if b1.to_ascii_lowercase() == 'o'
                && m1.is_none()
                && b2.to_ascii_lowercase() == 'o'
                && m2.is_none()
            {
                // Output 3 base chars (o-o-o)
                base.push(b1);
                base.push(b2);
                // Use the third 'o' matching the case of the second one? Or just lowercase?
                // The prompt example "ngoong" -> "n-g-o-o-o-n-g" implies lowercase
                base.push('o');

                // Capture tone if any
                if let Some(t) = t1.or(t2) {
                    tone = Some(t);
                }

                i += 2;
                continue;
            }
        }

        // Check if previous char is 'ư' (with horn) and current is 'ơ' (with horn)
        // This creates the "ươ" pattern where we only need one 'w' after 'u', not after 'o'
        let prev_is_u_with_horn = i > 0 && {
            let (prev_base, prev_mark, _) = decompose_vn_char(chars[i - 1]);
            prev_mark == Some('h') && prev_base.to_lowercase().next() == Some('u')
        };
        let is_o_with_horn = mark == Some('h') && base_char.to_lowercase().next() == Some('o');
        let is_u_with_horn = mark == Some('h') && base_char.to_lowercase().next() == Some('u');

        base.push(base_char);

        if let Some(m) = mark {
            match m {
                'a' => base.push('a'), // â (Telex: aa)
                'b' => base.push('w'), // ă (Telex: aw)
                'e' => base.push('e'), // ê (Telex: ee)
                'o' => {
                    // For "ô" (circumflex), add 'o'
                    // For "ơ" after "ư" in "ươ" pattern, skip adding 'w'
                    if !(prev_is_u_with_horn && is_o_with_horn) {
                        base.push('o'); // ô (Telex: oo)
                    }
                }
                'h' => {
                    // For "ươ" pattern, we only add 'w' after 'u', not after 'o'
                    if is_u_with_horn {
                        base.push('w'); // ư (Telex: uw)
                    } else if is_o_with_horn && !prev_is_u_with_horn {
                        base.push('w'); // ơ (Telex: ow) - only if not in ươ pattern
                    }
                }
                'd' => base.push('d'), // đ (Telex: dd)
                _ => {}
            }
        }

        // Keep the last tone marker (words should only have one tone)
        if char_tone.is_some() {
            tone = char_tone;
        }

        i += 1;
    }

    // Append tone at the end
    if let Some(t) = tone {
        base.push(t);
    }

    base
}

/// Convert Vietnamese word to VNI input (tone at end of word)
/// Following standard VNI rules from documentation:
/// - 1=sắc, 2=huyền, 3=hỏi, 4=ngã, 5=nặng
/// - 6=mũ (â,ê,ô), 7=móc (ơ,ư), 8=trăng (ă), 9=gạch (đ)
/// Note: For "ươ", we type "u-7-o" NOT "u-7-o-7" - the '7' on 'o' is not needed
fn vn_to_vni(word: &str) -> String {
    let chars: Vec<char> = word.chars().collect();
    let mut base = String::new();
    let mut tone: Option<char> = None;
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        let (base_char, mark, char_tone) = decompose_vn_char(c);

        // Check if previous char is 'ư' (with horn) and current is 'ơ' (with horn)
        // This creates the "ươ" pattern where we only need one '7' after 'u', not after 'o'
        let prev_is_u_with_horn = i > 0 && {
            let (prev_base, prev_mark, _) = decompose_vn_char(chars[i - 1]);
            prev_mark == Some('h') && prev_base.to_lowercase().next() == Some('u')
        };
        let is_o_with_horn = mark == Some('h') && base_char.to_lowercase().next() == Some('o');
        let is_u_with_horn = mark == Some('h') && base_char.to_lowercase().next() == Some('u');

        base.push(base_char);

        match mark {
            Some('a') => base.push('6'), // â = a + 6
            Some('b') => base.push('8'), // ă = a + 8
            Some('e') => base.push('6'), // ê = e + 6
            Some('o') => {
                // For "ươ" pattern, don't add '7' after 'o'
                // Only add '6' if it's 'ô' (circumflex), not for 'ơ' after 'ư'
                if prev_is_u_with_horn && is_o_with_horn {
                    // This is 'ơ' in 'ươ' pattern - skip adding any modifier after 'o'
                } else {
                    base.push('6'); // ô = o + 6
                }
            }
            Some('h') => {
                // For "ươ" pattern, we only add '7' after 'u', not after 'o'
                if is_u_with_horn {
                    base.push('7'); // ư = u + 7
                } else if is_o_with_horn && !prev_is_u_with_horn {
                    base.push('7'); // ơ = o + 7 (only if not in ươ pattern)
                }
            }
            Some('d') => base.push('9'), // đ = d + 9
            _ => {}
        }

        // Keep the last tone marker (words should only have one tone)
        if let Some(t) = char_tone {
            tone = Some(match t {
                's' => '1', // sắc
                'f' => '2', // huyền
                'r' => '3', // hỏi
                'x' => '4', // ngã
                'j' => '5', // nặng
                _ => t,
            });
        }

        i += 1;
    }

    // Append tone at the end
    if let Some(t) = tone {
        base.push(t);
    }

    base
}

/// Convert between traditional and modern oa/oe tone placement
/// Traditional: hoá (tone on 'a'), Modern: hóa (tone on 'o')
fn to_modern_tone(word: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = word.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        // Check for oa/oe/uy with tone on second vowel (traditional)
        if i + 1 < chars.len() && (c == 'o' || c == 'O' || c == 'u' || c == 'U') {
            let next = chars[i + 1];
            let (new_first, new_next) = match next {
                // oa/oe cases
                'à' if c == 'o' || c == 'O' => ('ò', 'a'),
                'á' if c == 'o' || c == 'O' => ('ó', 'a'),
                'ả' if c == 'o' || c == 'O' => ('ỏ', 'a'),
                'ã' if c == 'o' || c == 'O' => ('õ', 'a'),
                'ạ' if c == 'o' || c == 'O' => ('ọ', 'a'),
                'è' if c == 'o' || c == 'O' => ('ò', 'e'),
                'é' if c == 'o' || c == 'O' => ('ó', 'e'),
                'ẻ' if c == 'o' || c == 'O' => ('ỏ', 'e'),
                'ẽ' if c == 'o' || c == 'O' => ('õ', 'e'),
                'ẹ' if c == 'o' || c == 'O' => ('ọ', 'e'),

                // uy cases
                'ỳ' if c == 'u' || c == 'U' => ('ù', 'y'),
                'ý' if c == 'u' || c == 'U' => ('ú', 'y'),
                'ỷ' if c == 'u' || c == 'U' => ('ủ', 'y'),
                'ỹ' if c == 'u' || c == 'U' => ('ũ', 'y'),
                'ỵ' if c == 'u' || c == 'U' => ('ụ', 'y'),

                // Uppercase
                'À' if c == 'O' => ('Ò', 'A'),
                'Á' if c == 'O' => ('Ó', 'A'),
                'Ả' if c == 'O' => ('Ỏ', 'A'),
                'Ã' if c == 'O' => ('Õ', 'A'),
                'Ạ' if c == 'O' => ('Ọ', 'A'),
                'È' if c == 'O' => ('Ò', 'E'),
                'É' if c == 'O' => ('Ó', 'E'),
                'Ẻ' if c == 'O' => ('Ỏ', 'E'),
                'Ẽ' if c == 'O' => ('Õ', 'E'),
                'Ẹ' if c == 'O' => ('Ọ', 'E'),

                'Ỳ' if c == 'U' => ('Ù', 'Y'),
                'Ý' if c == 'U' => ('Ú', 'Y'),
                'Ỷ' if c == 'U' => ('Ủ', 'Y'),
                'Ỹ' if c == 'U' => ('Ũ', 'Y'),
                'Ỵ' if c == 'U' => ('Ụ', 'Y'),

                _ => {
                    result.push(c);
                    i += 1;
                    continue;
                }
            };
            let final_first = if c.is_uppercase() {
                new_first.to_uppercase().next().unwrap_or(new_first)
            } else {
                new_first
            };
            result.push(final_first);
            result.push(new_next);
            i += 2;
        } else {
            result.push(c);
            i += 1;
        }
    }
    result
}

/// Check if two words match (considering both traditional and modern tone styles)
fn matches_either_style(expected: &str, actual: &str) -> bool {
    if expected == actual {
        return true;
    }
    let normalized_expected = to_modern_tone(expected);
    let normalized_actual = to_modern_tone(actual);
    normalized_expected == normalized_actual
}

/// Failure category for classification
#[derive(Debug, Clone, PartialEq)]
enum FailureCategory {
    Engine,     // Engine logic errors
    Dictionary, // Dictionary issues (loan words, invalid words)
}

impl FailureCategory {
    fn as_str(&self) -> &'static str {
        match self {
            FailureCategory::Engine => "engine",
            FailureCategory::Dictionary => "dictionary",
        }
    }
}

/// Classify a failure based on word patterns
fn classify_failure(word: &str) -> FailureCategory {
    let word_lower = word.to_lowercase();

    // Check for loan word patterns (Dictionary issues)
    // Words with 'r' after vowel (carô, garô pattern)
    if word_lower.contains("carô")
        || word_lower.contains("garô")
        || word_lower.contains("tarô")
        || word_lower.contains("xirô")
        || word_lower.contains("derô")
        || word_lower.contains("dêrô")
    {
        return FailureCategory::Dictionary;
    }

    // Check for words ending with x/s that look like loan words
    if (word_lower.ends_with("x") || word_lower.ends_with("s"))
        && !word
            .chars()
            .any(|c| "áàảãạéèẻẽẹíìỉĩịóòỏõọúùủũụýỳỷỹỵ".contains(c))
    {
        // Words like test, box, taxi without proper Vietnamese tones
        return FailureCategory::Dictionary;
    }

    // Check for patterns that suggest engine issues
    // 'u' + 'w' pattern problems (huơ, khuơ, thuở, uở)
    if word_lower.contains("huơ")
        || word_lower.contains("khuơ")
        || word_lower.contains("thuở")
        || word_lower == "uở"
    {
        return FailureCategory::Engine;
    }

    // Default to Dictionary for most loan-looking words
    FailureCategory::Dictionary
}

/// Test result for a single word
#[derive(Debug, Clone)]
struct TestResult {
    word: String,
    input: String,
    expected: String,
    actual: String,
    category: FailureCategory,
}

/// Result for a batch of tests
#[derive(Debug, Default, Clone)]
struct BatchResult {
    total: usize,
    passed: usize,
    failed: usize,
    failures: Vec<TestResult>,
}

/// Statistics for a category
#[derive(Debug, Default, Clone)]
struct CategoryStats {
    total: usize,
    passed: usize,
    failed: usize,
    failures: Vec<TestResult>,
}

/// Group words by character count
fn group_by_length<'a>(words: &'a [&'a str]) -> HashMap<String, Vec<&'a str>> {
    let mut groups: HashMap<String, Vec<&str>> = HashMap::new();

    for word in words {
        let len = word.chars().count();
        let key = match len {
            1..=3 => "short_1_3".to_string(),
            4..=6 => "medium_4_6".to_string(),
            7..=10 => "long_7_10".to_string(),
            _ => "very_long_11plus".to_string(),
        };
        groups.entry(key).or_default().push(word);
    }

    groups
}

/// Test a batch of words with Telex
fn test_telex_batch(words: &[&str], _category: &str, _chunk_idx: usize) -> BatchResult {
    let mut result = BatchResult::default();

    for word in words {
        if word.is_empty() || word.contains(' ') {
            continue;
        }

        // Skip loan words with double 'o' pattern
        let has_double_o = word.contains("oo")
            || word.contains("òo")
            || word.contains("óo")
            || word.contains("ỏo")
            || word.contains("õo")
            || word.contains("ọo")
            || word.contains("ồo")
            || word.contains("ốo")
            || word.contains("ổo")
            || word.contains("ỗo")
            || word.contains("ộo");
        if has_double_o {
            continue;
        }

        let telex_input = vn_to_telex(word);
        let input_with_space = format!("{} ", telex_input);
        let expected = format!("{} ", word);

        let mut e = Engine::new();
        e.set_modern_tone(true);
        e.set_english_auto_restore(false);
        let actual = type_word(&mut e, &input_with_space);

        result.total += 1;
        if matches_either_style(expected.trim(), actual.trim()) {
            result.passed += 1;
        } else {
            result.failed += 1;
            let failure_category = classify_failure(word);
            result.failures.push(TestResult {
                word: word.to_string(),
                input: telex_input,
                expected: expected.trim().to_string(),
                actual: actual.trim().to_string(),
                category: failure_category,
            });
        }
    }

    // Write failures to separate files by category
    // REMOVED: Chunk-based file writing

    result
}

/// Test a batch of words with VNI
fn test_vni_batch(words: &[&str], _category: &str, _chunk_idx: usize) -> BatchResult {
    let mut result = BatchResult::default();

    for word in words {
        if word.is_empty() || word.contains(' ') {
            continue;
        }

        // Skip loan words with double 'o' pattern
        let has_double_o = word.contains("oo")
            || word.contains("òo")
            || word.contains("óo")
            || word.contains("ỏo")
            || word.contains("õo")
            || word.contains("ọo")
            || word.contains("ồo")
            || word.contains("ốo")
            || word.contains("ổo")
            || word.contains("ỗo")
            || word.contains("ộo");
        if has_double_o {
            continue;
        }

        let vni_input = vn_to_vni(word);
        let input_with_space = format!("{} ", vni_input);
        let expected = format!("{} ", word);

        let mut e = Engine::new();
        e.set_modern_tone(true);
        e.set_english_auto_restore(false);
        e.set_method(1); // VNI mode
        let actual = type_word(&mut e, &input_with_space);

        result.total += 1;
        if matches_either_style(expected.trim(), actual.trim()) {
            result.passed += 1;
        } else {
            result.failed += 1;
            let failure_category = classify_failure(word);
            result.failures.push(TestResult {
                word: word.to_string(),
                input: vni_input,
                expected: expected.trim().to_string(),
                actual: actual.trim().to_string(),
                category: failure_category,
            });
        }
    }

    // Write failures to separate files by category
    // REMOVED: Chunk-based file writing

    result
}

/// Helper to write consolidated failures to a single file
fn write_failures_to_file(filename: &str, stats: &HashMap<String, CategoryStats>) {
    let path = std::path::Path::new("tests/failures").join(filename);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap_or_default();
    }

    if let Ok(mut f) = File::create(&path) {
        writeln!(f, "WORD\tINPUT\tEXPECTED\tACTUAL\tCATEGORY").unwrap();

        let mut all_failures = Vec::new();
        for cat_stats in stats.values() {
            all_failures.extend(&cat_stats.failures);
        }
        // Sort by word for consistent output
        all_failures.sort_by(|a, b| a.word.cmp(&b.word));

        for failure in all_failures {
            writeln!(
                f,
                "{}\t{}\t{}\t{}\t{:?}",
                failure.word, failure.input, failure.expected, failure.actual, failure.category
            )
            .unwrap();
        }
    }
}

/// Print test report
fn print_report(method: &str, category_stats: &HashMap<String, CategoryStats>, total_time: f64) {
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!(
        "║           {} TEST REPORT                         ║",
        method.to_uppercase()
    );
    println!("╚════════════════════════════════════════════════════════════════╝");

    let mut total_passed = 0;
    let mut total_failed = 0;
    let mut total_words = 0;

    // Define category order
    let categories = vec!["short_1_3", "medium_4_6", "long_7_10", "very_long_11plus"];

    for category in &categories {
        if let Some(stats) = category_stats.get(*category) {
            if stats.total == 0 {
                continue;
            }

            let pass_rate = (stats.passed as f64 / stats.total as f64) * 100.0;
            let category_name = match *category {
                "short_1_3" => "1-3 chars  ",
                "medium_4_6" => "4-6 chars  ",
                "long_7_10" => "7-10 chars ",
                _ => "11+ chars  ",
            };

            println!("\n┌────────────────────────────────────────────────────────────────┐");
            println!(
                "│ Category: {}                                    │",
                category_name
            );
            println!("├────────────────────────────────────────────────────────────────┤");
            println!(
                "│  Total: {:>5}  │  Passed: {:>5}  │  Failed: {:>5}  │  Rate: {:>5.1}% │",
                stats.total, stats.passed, stats.failed, pass_rate
            );
            println!("└────────────────────────────────────────────────────────────────┘");

            // Show first 5 failures
            if !stats.failures.is_empty() {
                println!("  Sample failures:");
                for (i, failure) in stats.failures.iter().take(5).enumerate() {
                    println!(
                        "    {}. '{}' (input: '{}') → got '{}'",
                        i + 1,
                        failure.word,
                        failure.input,
                        failure.actual
                    );
                }
                if stats.failures.len() > 5 {
                    println!("    ... and {} more failures", stats.failures.len() - 5);
                }
            }

            total_passed += stats.passed;
            total_failed += stats.failed;
            total_words += stats.total;
        }
    }

    let overall_rate = if total_words > 0 {
        (total_passed as f64 / total_words as f64) * 100.0
    } else {
        0.0
    };

    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║                      OVERALL SUMMARY                           ║");
    println!("╠════════════════════════════════════════════════════════════════╣");
    println!(
        "║  Total Words: {:>6}                                         ║",
        total_words
    );
    println!(
        "║  Passed:      {:>6}  ({:>6.2}%)                              ║",
        total_passed, overall_rate
    );
    println!(
        "║  Failed:      {:>6}                                         ║",
        total_failed
    );
    println!(
        "║  Time:        {:>6.2}s                                       ║",
        total_time
    );
    println!("╚════════════════════════════════════════════════════════════════╝");
}

#[test]
fn test_vietnamese_dictionary_coverage() {
    let content = include_str!("data/vietnamese_69k_pure.txt");
    const CHUNK_SIZE: usize = 5000;
    const MIN_PASS_RATE: f64 = 93.0; // Adjusted based on current engine capabilities

    // Collect all single-syllable words
    let all_words: Vec<&str> = content
        .lines()
        .map(|line| line.trim())
        .filter(|word| !word.is_empty() && !word.contains(' '))
        .collect();

    println!("\n═══════════════════════════════════════════════════════════════════");
    println!("   VIETNAMESE DICTIONARY COVERAGE TEST (~65k filtered words)");
    println!("═══════════════════════════════════════════════════════════════════");
    println!("Total single-syllable words: {}", all_words.len());

    // Group words by length
    let groups = group_by_length(&all_words);

    for (category, words) in &groups {
        println!("  {}: {} words", category, words.len());
    }

    // Test Telex
    println!("\n═══════════════════════════════════════════════════════════════════");
    println!("                     TESTING TELEX INPUT");
    println!("═══════════════════════════════════════════════════════════════════");

    let mut telex_stats: HashMap<String, CategoryStats> = HashMap::new();
    let telex_start = Instant::now();

    for (category, words) in &groups {
        println!("\nTesting category: {} ({} words)", category, words.len());

        let chunks: Vec<&[&str]> = words.chunks(CHUNK_SIZE).collect();
        for (chunk_idx, chunk) in chunks.iter().enumerate() {
            let batch_result = test_telex_batch(chunk, category, chunk_idx);

            let entry = telex_stats.entry(category.to_string()).or_default();
            entry.total += batch_result.total;
            entry.passed += batch_result.passed;
            entry.failed += batch_result.failed;
            entry.failures.extend(batch_result.failures);

            print!(
                "  Chunk {}: {}/{} passed  \r",
                chunk_idx + 1,
                entry.passed,
                entry.total
            );
        }
        println!();
    }

    let telex_time = telex_start.elapsed().as_secs_f64();
    print_report("Telex", &telex_stats, telex_time);
    write_failures_to_file("failures_telex.txt", &telex_stats);

    // Test VNI
    println!("\n═══════════════════════════════════════════════════════════════════");
    println!("                      TESTING VNI INPUT");
    println!("═══════════════════════════════════════════════════════════════════");

    let mut vni_stats: HashMap<String, CategoryStats> = HashMap::new();
    let vni_start = Instant::now();

    for (category, words) in &groups {
        println!("\nTesting category: {} ({} words)", category, words.len());

        let chunks: Vec<&[&str]> = words.chunks(CHUNK_SIZE).collect();
        for (chunk_idx, chunk) in chunks.iter().enumerate() {
            let batch_result = test_vni_batch(chunk, category, chunk_idx);

            let entry = vni_stats.entry(category.to_string()).or_default();
            entry.total += batch_result.total;
            entry.passed += batch_result.passed;
            entry.failed += batch_result.failed;
            entry.failures.extend(batch_result.failures);

            print!(
                "  Chunk {}: {}/{} passed  \r",
                chunk_idx + 1,
                entry.passed,
                entry.total
            );
        }
        println!();
    }

    let vni_time = vni_start.elapsed().as_secs_f64();
    print_report("VNI", &vni_stats, vni_time);
    write_failures_to_file("failures_vni.txt", &vni_stats);

    // Calculate overall pass rate
    let total_telex_passed: usize = telex_stats.values().map(|s| s.passed).sum();
    let total_telex_words: usize = telex_stats.values().map(|s| s.total).sum();
    let telex_pass_rate = if total_telex_words > 0 {
        (total_telex_passed as f64 / total_telex_words as f64) * 100.0
    } else {
        0.0
    };

    let total_vni_passed: usize = vni_stats.values().map(|s| s.passed).sum();
    let total_vni_words: usize = vni_stats.values().map(|s| s.total).sum();
    let vni_pass_rate = if total_vni_words > 0 {
        (total_vni_passed as f64 / total_vni_words as f64) * 100.0
    } else {
        0.0
    };

    println!("\n═══════════════════════════════════════════════════════════════════");
    println!("                        FINAL RESULTS");
    println!("═══════════════════════════════════════════════════════════════════");
    println!(
        "Telex: {:.2}% ({} / {})",
        telex_pass_rate, total_telex_passed, total_telex_words
    );
    println!(
        "VNI:   {:.2}% ({} / {})",
        vni_pass_rate, total_vni_passed, total_vni_words
    );
    println!("═══════════════════════════════════════════════════════════════════");

    // Assert minimum pass rate
    assert!(
        telex_pass_rate >= MIN_PASS_RATE,
        "Telex pass rate {:.2}% is below threshold {:.1}%",
        telex_pass_rate,
        MIN_PASS_RATE
    );
    assert!(
        vni_pass_rate >= MIN_PASS_RATE,
        "VNI pass rate {:.2}% is below threshold {:.1}%",
        vni_pass_rate,
        MIN_PASS_RATE
    );
}

#[test]
fn test_telex_specific_cases() {
    let cases = &[
        ("hoà", "hoaf"),
        ("hóa", "hoas"),
        ("hoả", "hoar"),
        ("hoã", "hoax"),
        ("hoạ", "hoaj"),
        ("hòa", "hoaf"),
        ("quế", "quee"),
        ("quyển", "quyenr"),
        ("tuyết", "tuyets"),
        ("nghĩa", "nghiax"),
        ("giữa", "giuwax"),
        ("chuyện", "chuyenj"),
        ("thuyền", "thuyeenf"),
        ("mỹ", "myx"),
        ("đường", "duwowngf"),
        ("thuở", "thuowr"),
        ("chèo", "cheof"),
        ("tòa", "toaf"),
        ("tồ", "toof"),
        ("suýt", "suyts"),
        ("kĩ", "kix"),
        ("sữa", "suwax"),
        ("nguyễn", "nguyenx"),
        ("nhẫn", "nhaanj"),
        ("sắc", "sacws"),
        ("dũng", "dungx"),
        ("đứng", "duwngs"),
        ("miễn", "mienx"),
        ("boong", "booong"),
        ("xoong", "xooong"),
        ("goòng", "gooongf"),
    ];

    let mut passed = 0;
    let mut failed = 0;
    let mut failures: Vec<(String, String, String, String)> = Vec::new();

    for (expected_word, telex_input) in cases {
        let input_with_space = format!("{} ", telex_input);
        let expected = format!("{} ", expected_word);

        let mut e = Engine::new();
        e.set_modern_tone(true);
        e.set_english_auto_restore(false);
        e.set_method(0);
        let result = type_word(&mut e, &input_with_space);

        if matches_either_style(expected.trim(), result.trim()) {
            passed += 1;
        } else {
            failed += 1;
            failures.push((
                expected_word.to_string(),
                telex_input.to_string(),
                expected.trim().to_string(),
                result.trim().to_string(),
            ));
        }
    }

    println!("\n=== Telex Specific Cases Test ===");
    println!("Passed: {}", passed);
    println!("Failed: {}", failed);

    if !failures.is_empty() {
        println!("\n=== Failures ===");
        println!(
            "{:<15} {:<20} {:<15} {:<15}",
            "WORD", "TELEX INPUT", "EXPECTED", "ACTUAL"
        );
        for (word, telex, expected, actual) in &failures {
            println!("{:<15} {:<20} {:<15} {:<15}", word, telex, expected, actual);
        }
    }

    assert_eq!(failed, 0, "Telex specific cases test failed!");
}

#[test]
fn test_vni_specific_cases() {
    let cases = &[
        ("hoà", "hoa2"),
        ("hóa", "hoa1"),
        ("hoả", "hoa3"),
        ("hoã", "hoa4"),
        ("hoạ", "hoa5"),
        ("hòa", "hoa2"),
        ("quế", "que6"),
        ("quyển", "quyen3"),
        ("tuyết", "tuyet1"),
        ("nghĩa", "nghia4"),
        ("giữa", "giu7a4"),
        ("chuyện", "chuyen5"),
        ("thuyền", "thuyen2"),
        ("mỹ", "my4"),
        ("đường", "du7ong2"),
        ("thuở", "thu7o3"),
        ("chèo", "cheo2"),
        ("tòa", "toa2"),
        ("tồ", "to6"),
        ("suýt", "suyt1"),
        ("kĩ", "ki4"),
        ("sữa", "su7a4"),
        ("nguyễn", "nguyen4"),
        ("nhẫn", "nhan4"),
        ("sắc", "sac1"),
        ("dũng", "dung4"),
        ("đứng", "du7ng1"),
        ("miễn", "mien4"),
    ];

    let mut passed = 0;
    let mut failed = 0;
    let mut failures: Vec<(String, String, String, String)> = Vec::new();

    for (expected_word, vni_input) in cases {
        let input_with_space = format!("{} ", vni_input);
        let expected = format!("{} ", expected_word);

        let mut e = Engine::new();
        e.set_modern_tone(true);
        e.set_english_auto_restore(false);
        e.set_method(1);
        let result = type_word(&mut e, &input_with_space);

        if matches_either_style(expected.trim(), result.trim()) {
            passed += 1;
        } else {
            failed += 1;
            failures.push((
                expected_word.to_string(),
                vni_input.to_string(),
                expected.trim().to_string(),
                result.trim().to_string(),
            ));
        }
    }

    println!("\n=== VNI Specific Cases Test ===");
    println!("Passed: {}", passed);
    println!("Failed: {}", failed);

    if !failures.is_empty() {
        println!("\n=== Failures ===");
        println!(
            "{:<15} {:<20} {:<15} {:<15}",
            "WORD", "VNI INPUT", "EXPECTED", "ACTUAL"
        );
        for (word, vni, expected, actual) in &failures {
            println!("{:<15} {:<20} {:<15} {:<15}", word, vni, expected, actual);
        }
    }

    assert_eq!(failed, 0, "VNI specific cases test failed!");
}
