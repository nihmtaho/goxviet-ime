#[cfg(test)]
mod english_word_modifier_bypass_test {
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
                    if let Some(c) = char::from_u32(result.as_slice()[i]) {
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
    fn test_trans_plus_f_no_tone() {
        // FIX VERIFICATION: Type "trans" (English) + "f" (tone modifier)
        // EXPECTED: "transf" (f is normal letter, NOT tone modifier)
        // BUG WAS: "tràns" (f applied tone to 'a')
        let output = type_and_collect("transf", 0); // 0=Telex
        assert_eq!(output, "transf", "Expected 'transf' but got '{}'", output);
        println!("✅ FIXED: 'trans' + 'f' → '{}' (no tone applied)", output);
    }

    #[test]
    fn test_restore_plus_s_no_tone() {
        // FIX VERIFICATION: Type "restore" (English) + "s" (tone modifier)
        // EXPECTED: "restores" (s is normal letter, NOT tone modifier)
        let output = type_and_collect("restores", 0); // 0=Telex
        assert_eq!(output, "restores", "Expected 'restores' but got '{}'", output);
        println!("✅ FIXED: 'restore' + 's' → '{}' (no tone applied)", output);
    }

    #[test]
    fn test_import_plus_x_no_tone() {
        // FIX VERIFICATION: Type "import" (English) + "x" (tone modifier)
        // EXPECTED: "importx" (x is normal letter, NOT tone modifier)
        let output = type_and_collect("importx", 0); // 0=Telex
        assert_eq!(output, "importx", "Expected 'importx' but got '{}'", output);
        println!("✅ FIXED: 'import' + 'x' → '{}' (no tone applied)", output);
    }

    #[test]
    fn test_syntax_plus_r_no_tone() {
        // FIX VERIFICATION: Type "syntax" (English) + "r" (tone modifier)
        // EXPECTED: "syntaxr" (r is normal letter, NOT tone modifier)
        let output = type_and_collect("syntaxr", 0); // 0=Telex
        assert_eq!(output, "syntaxr", "Expected 'syntaxr' but got '{}'", output);
        println!("✅ FIXED: 'syntax' + 'r' → '{}' (no tone applied)", output);
    }

    #[test]
    fn test_parse_plus_x_no_tone() {
        // FIX VERIFICATION: Type "parse" (English) + "x" (tone modifier)
        // EXPECTED: "parsex" (x is normal letter, NOT tone modifier)
        let output = type_and_collect("parsex", 0); // 0=Telex
        assert_eq!(output, "parsex", "Expected 'parsex' but got '{}'", output);
        println!("✅ FIXED: 'parse' + 'x' → '{}' (no tone applied)", output);
    }
}
