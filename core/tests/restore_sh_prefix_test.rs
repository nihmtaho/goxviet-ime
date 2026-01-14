//! Tests for restore prefix detection and sh- prefix
//!
//! This test file verifies:
//! 1. "restore" word is properly detected and auto-restores when space is pressed
//! 2. "sh-" prefix (short, shell, share) is detected as English (Vietnamese doesn't have "sh-")

use goxviet_core::engine::Engine;

/// Helper: Convert character to key code
fn char_to_key(ch: char) -> u16 {
    match ch.to_ascii_lowercase() {
        'a' => 0x41,
        'b' => 0x42,
        'c' => 0x43,
        'd' => 0x44,
        'e' => 0x45,
        'f' => 0x46,
        'g' => 0x47,
        'h' => 0x48,
        'i' => 0x49,
        'j' => 0x4A,
        'k' => 0x4B,
        'l' => 0x4C,
        'm' => 0x4D,
        'n' => 0x4E,
        'o' => 0x4F,
        'p' => 0x50,
        'q' => 0x51,
        'r' => 0x52,
        's' => 0x53,
        't' => 0x54,
        'u' => 0x55,
        'v' => 0x56,
        'w' => 0x57,
        'x' => 0x58,
        'y' => 0x59,
        'z' => 0x5A,
        ' ' => 0x20,
        _ => 0x00,
    }
}

/// Helper: Type a word and get output
fn type_and_collect(word: &str, method: u8) -> String {
    let mut engine = Engine::new();
    engine.set_method(method); // 0=Telex
    let mut output = String::new();

    for ch in word.chars() {
        let key = char_to_key(ch);
        let caps = ch.is_uppercase();
        let result = engine.on_key(key, caps, false);

        // Apply result to output
        if result.action == 1 {
            let bs_count = result.backspace as usize;
            for _ in 0..bs_count.min(output.len()) {
                output.pop();
            }
            for i in 0..result.count as usize {
                if let Some(c) = char::from_u32(result.chars[i]) {
                    output.push(c);
                }
            }
        } else {
            output.push(ch);
        }
    }
    output
}

#[test]
fn test_restore_word_detection() {
    // Test: Type "restore" in Telex
    // BEFORE FIX: "rểto" (Vietnamese transforms applied)
    // AFTER FIX: "restore" (detected as English via rest- prefix + -ore suffix)
    let output = type_and_collect("restore", 0); // 0=Telex
    assert_eq!(
        output, "restore",
        "Word 'restore' should remain unchanged, got '{}'",
        output
    );
    println!("✅ FIXED: 'restore' → '{}' (no transforms)", output);
}

#[test]
fn test_restore_with_space_auto_restore() {
    // Test: Type "restore " (with space) to trigger auto-restore check
    // Space should trigger auto-restore, keeping "restore" as English
    let output = type_and_collect("restore ", 0); // 0=Telex
    assert_eq!(
        output, "restore ",
        "Word 'restore' with space should remain unchanged, got '{}'",
        output.trim_end()
    );
    println!(
        "✅ FIXED: 'restore' + space → '{}' (auto-restore works)",
        output.trim_end()
    );
}

#[test]
fn test_sh_prefix_short() {
    // Test: Type "short" (sh- prefix)
    // Vietnamese doesn't have "sh-" consonant cluster
    // EXPECTED: "short" (detected as English)
    // BUG WAS: "shỏt" or "shõt" (Vietnamese tone applied)
    let output = type_and_collect("short", 0); // 0=Telex
    assert_eq!(
        output, "short",
        "Word 'short' should remain unchanged (sh- prefix), got '{}'",
        output
    );
    println!("✅ FIXED: 'short' → '{}' (sh- prefix detected)", output);
}

#[test]
fn test_sh_prefix_shell() {
    // Test: Type "shell"
    // EXPECTED: "shell" (sh- prefix is English)
    let output = type_and_collect("shell", 0); // 0=Telex
    assert_eq!(
        output, "shell",
        "Word 'shell' should remain unchanged (sh- prefix), got '{}'",
        output
    );
    println!("✅ FIXED: 'shell' → '{}' (sh- prefix detected)", output);
}

#[test]
fn test_sh_prefix_share() {
    // Test: Type "share"
    // EXPECTED: "share" (sh- prefix is English)
    let output = type_and_collect("share", 0); // 0=Telex
    assert_eq!(
        output, "share",
        "Word 'share' should remain unchanged (sh- prefix), got '{}'",
        output
    );
    println!("✅ FIXED: 'share' → '{}' (sh- prefix detected)", output);
}

#[test]
fn test_sh_prefix_show() {
    // Test: Type "show"
    // EXPECTED: "show" (sh- prefix is English)
    let output = type_and_collect("show", 0); // 0=Telex
    assert_eq!(
        output, "show",
        "Word 'show' should remain unchanged (sh- prefix), got '{}'",
        output
    );
    println!("✅ FIXED: 'show' → '{}' (sh- prefix detected)", output);
}

#[test]
fn test_restore_ore_suffix_words() {
    // Test other words ending with "-ore" suffix
    // These should also benefit from the -ore suffix detection
    
    let words = vec![
        "score",   // s+c+o+r+e
        "more",    // m+o+r+e
        "store",   // s+t+o+r+e
        "before",  // b+e+f+o+r+e
        "core",    // c+o+r+e
    ];

    for word in words {
        let output = type_and_collect(word, 0);
        assert_eq!(
            output, word,
            "Word '{}' should remain unchanged (-ore suffix), got '{}'",
            word, output
        );
        println!("✅ PASS: '{}' → '{}' (-ore suffix)", word, output);
    }
}
