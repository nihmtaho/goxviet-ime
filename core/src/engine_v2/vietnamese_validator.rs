use crate::data::keys;
use crate::engine_v2::fsm::tables::{CHAR_PROPS, PROP_VOWEL, VIETNAMESE_BIGRAMS};

pub struct ValidationResult {
    pub is_valid: bool,
    pub confidence: u8,
}

pub struct VietnameseSyllableValidator;

impl VietnameseSyllableValidator {
    /// O(1) validation of Vietnamese syllable structure
    pub fn validate(keys: &[u16]) -> ValidationResult {
        println!("DEBUG: validate() called with {:?}", keys);
        if keys.is_empty() {
            return ValidationResult {
                is_valid: true,
                confidence: 100,
            };
        }

        let len = keys.len();

        // Rule 1: Validate initial consonants (comprehensive check from OpenKey)
        // Vietnamese allows specific initial consonants and clusters
        if !Self::is_valid_initial_consonant(keys) {
            println!("DEBUG: Rule 1 failed");
            return ValidationResult {
                is_valid: false,
                confidence: 0,
            };
        }

        // Rule 1.5: Check for invalid consonant clusters at the beginning
        // Vietnamese does not allow bl, cl, fl, br, cr, dr, fr, gr, pr, str, etc.
        if len >= 2 {
            let k1 = keys[0];
            let k2 = keys[1];
            if Self::is_invalid_consonant_cluster(k1, k2) {
                println!("DEBUG: Rule 1.5 cluster failed");
                return ValidationResult {
                    is_valid: false,
                    confidence: 0,
                };
            }

            // Check c/k/g/gh/ng/ngh distribution rules
            if Self::violates_ck_distribution(k1, k2) {
                println!("DEBUG: Rule 1.5 distribution failed");
                return ValidationResult {
                    is_valid: false,
                    confidence: 0,
                };
            }
        }

        // Rule 2 & 6: Bigram validation (O(1) per bigram)
        for i in 0..len.saturating_sub(1) {
            let k1 = keys[i];
            let k2 = keys[i + 1];

            if k1 < 128 && k2 < 128 {
                let allowed_next = VIETNAMESE_BIGRAMS[k1 as usize];
                if (allowed_next & (1 << k2 as u128)) == 0 {
                    // Check if it's a known vowel compound or allowed cluster
                    if !Self::is_allowed_exception(k1, k2) {
                        println!("DEBUG: Rule 2 Bigram failed for {:?} -> {:?}", k1, k2);
                        return ValidationResult {
                            is_valid: false,
                            confidence: 0,
                        };
                    }
                }
            }
        }

        // Rule 5: Coda validation
        let last = keys[len - 1];
        if len > 1
            && (CHAR_PROPS[last as usize] & crate::engine_v2::fsm::tables::PROP_CODA_INVALID) != 0
        {
            // Allow 'k' as a final consonant for names like "Đăk Lăk"
            if last == keys::K {
                // This is a special case, proceed.
            } else {
                // Check for valid compounds like 'ng', 'nh', 'ch'
                let prev = keys[len - 2];
                if !matches!(
                    (prev, last),
                    (keys::N, keys::G) | (keys::N, keys::H) | (keys::C, keys::H)
                ) {
                    println!("DEBUG: Rule 5 Coda failed (invalid coda char)");
                    return ValidationResult {
                        is_valid: false,
                        confidence: 0,
                    };
                }
            }
        }

        // Rule 6: Check invalid vowel + final consonant combinations
        // ô/ơ/u/ư + ch: ôch, ơch, uch, ưch are invalid
        // ô/ơ/u/ư + nh: ônh, ơnh, unh, ưnh are invalid
        // e/ê + ng: eng, êng are invalid
        if len >= 2 {
            let last = keys[len - 1];
            let prev = keys[len - 2];

            // Check for -ch ending
            if last == keys::H && prev == keys::C && len >= 3 {
                let vowel = keys[len - 3];
                if Self::is_invalid_vowel_before_ch(vowel) {
                    println!("DEBUG: Rule 6 CH check failed");
                    return ValidationResult {
                        is_valid: false,
                        confidence: 0,
                    };
                }
            }

            // Check for -nh ending
            if last == keys::H && prev == keys::N && len >= 3 {
                let vowel = keys[len - 3];
                if Self::is_invalid_vowel_before_nh(vowel) {
                    println!("DEBUG: Rule 6 NH check failed");
                    return ValidationResult {
                        is_valid: false,
                        confidence: 0,
                    };
                }
            }

            // Check for -ng ending
            if last == keys::G && prev == keys::N && len >= 3 {
                if !Self::is_valid_vowel_before_ng(keys, len) {
                    println!("DEBUG: Rule 6 NG check failed");
                    return ValidationResult {
                        is_valid: false,
                        confidence: 0,
                    };
                }
            }
        }

        // Rule 7: Validate vowel combinations (from OpenKey)
        if !Self::is_valid_vowel_sequence(keys) {
            println!("DEBUG: is_valid_vowel_sequence rejected {:?}", keys);
            return ValidationResult {
                is_valid: false,
                confidence: 0,
            };
        }

        // Rule 8: Validate vowel-coda compatibility (from OpenKey)
        // Certain vowels cannot appear before certain end consonants
        if !Self::is_valid_vowel_coda_pair(keys) {
            return ValidationResult {
                is_valid: false,
                confidence: 0,
            };
        }

        ValidationResult {
            is_valid: true,
            confidence: 100,
        }
    }

    /// Validate Vietnamese syllable with tone modifiers
    ///
    /// This validates both structure AND tone placement on vowels.
    /// Required for preventing invalid tone combinations like:
    /// - "eu" without circumflex (should be "êu")
    /// - "ăi" (breve + vowel - invalid)
    /// - "ieư" (should be "iêu")
    pub fn validate_with_tones(keys: &[u16], tones: &[u8]) -> ValidationResult {
        // First validate structure
        let struct_result = Self::validate(keys);
        if !struct_result.is_valid {
            return struct_result;
        }

        // Then validate tone placement
        if !Self::is_valid_tone_placement(keys, tones) {
            return ValidationResult {
                is_valid: false,
                confidence: 0,
            };
        }

        ValidationResult {
            is_valid: true,
            confidence: 100,
        }
    }

    /// Check if tone placement is valid for Vietnamese vowel patterns
    fn is_valid_tone_placement(keys: &[u16], tones: &[u8]) -> bool {
        use crate::data::chars::tone;

        if keys.len() != tones.len() {
            return false;
        }

        // Find vowel sequence

        let mut vowel_indices = Vec::new();
        for (i, &k) in keys.iter().enumerate() {
            if matches!(k, keys::A | keys::E | keys::I | keys::O | keys::U | keys::Y) {
                vowel_indices.push(i);
            } else if !vowel_indices.is_empty() {
                break; // Stop at first consonant after vowels
            }
        }

        if vowel_indices.len() < 2 {
            return true; // Single vowel - no tone placement rules
        }

        let vowel_keys: Vec<u16> = vowel_indices.iter().map(|&i| keys[i]).collect();
        let vowel_tones: Vec<u8> = vowel_indices.iter().map(|&i| tones[i]).collect();

        match vowel_keys.len() {
            2 => {
                let pair = [vowel_keys[0], vowel_keys[1]];

                // Rule 1: E+U requires circumflex on E
                // Valid: "êu", Invalid: "eu", "eư"
                if pair == [keys::E, keys::U] {
                    // E must have circumflex
                    if vowel_tones[0] != tone::CIRCUMFLEX {
                        return false;
                    }
                    // U can't have horn
                    if vowel_tones[1] == tone::HORN {
                        return false;
                    }
                }

                // Rule 2: Horn validation
                // Valid: "ươ", "ưa", "uơ" (rare but possible in dialects/typo), "ưi"
                // Invalid: "uư" (except ươ), "oư" (except ươ)

                // Check if any vowel has a horn
                if vowel_tones[0] == tone::HORN || vowel_tones[1] == tone::HORN {
                    // Valid patterns involving Horn:
                    // 'ư' (U+Horn): ưa, ươ, ưi
                    // 'ơ' (O+Horn): ơ, ơi, uơ, ươ
                    // 'ă' (A+Horn): oă (xoăn)
                    // Invalid: uư, aư, eư, etc.

                    let k1 = vowel_keys[0];
                    let k2 = vowel_keys[1];
                    let t1 = vowel_tones[0];
                    let t2 = vowel_tones[1];

                    if t1 == tone::HORN {
                        if k1 == keys::U {
                            // ư
                            // ưa, ươ, ưi, ưu (cưu, mưu) are valid
                            if !matches!(k2, keys::A | keys::O | keys::I | keys::U) {
                                return false;
                            }
                        } else if k1 == keys::O {
                            // ơ
                            // ơi passed. uơ (thuơ)
                            if !matches!(k2, keys::I | keys::U) {
                                return false;
                            }
                        } else if k1 == keys::A {
                            // ă
                            return false;
                        } else {
                            return false;
                        }
                    }

                    if t2 == tone::HORN {
                        if k2 == keys::O {
                            // ..ơ
                            // uơ, ươ valid. iơ (giờ) valid
                            if !matches!(k1, keys::U | keys::I) {
                                println!("DEBUG: Rejected O Horn (ơ) after {:?}", k1);
                                return false;
                            }
                        } else if k2 == keys::A {
                            // ..ă
                            // oă (xoăn), uă (quặc), iă (giặc) valid
                            if !matches!(k1, keys::O | keys::U | keys::I) {
                                println!("DEBUG: Rejected A Horn (ă) after {:?}", k1);
                                return false;
                            }
                        } else if k2 == keys::U {
                            // ..ư
                            // iư (giữ) valid
                            if !matches!(k1, keys::I) {
                                println!("DEBUG: Rejected U Horn (ư) after {:?}", k1);
                                return false;
                            }
                        } else {
                            println!("DEBUG: Rejected Horn on {:?}", k2);
                            return false;
                        }
                    }
                }

                // Rule 3: O+Circumflex (ô) followed by vowel restrictions
                // Valid: "ôi"
                // Invalid: "ôa", "ôe", "ôo", "ôu", "ôy" (unless "uô")
                if vowel_keys[0] == keys::O && vowel_tones[0] == tone::CIRCUMFLEX {
                    let next = vowel_keys[1];
                    if next != keys::I {
                        return false;
                    }
                }
            }
            3 => {
                // Rule 3b: O+Circumflex (ô) invalid as first vowel in triphthong
                // "ngoao" -> "ngôa" invalid. "ngoao" valid.
                if vowel_keys[0] == keys::O && vowel_tones[0] == tone::CIRCUMFLEX {
                    println!("DEBUG: Rule 3b Rejected O(Circ) as v1 (len 3)");
                    return false;
                }

                // Rule 3: Breve validation (ă)
                // Valid: "ăm", "ăn", "ăng", "oă" (xoăn)
                // Invalid: "ăi", "ăo", "ău", "ăy"
                if vowel_keys[0] == keys::A && vowel_tones[0] == tone::HORN {
                    // A with breve (horn on A = ă) followed by vowel is invalid
                    return false;
                }

                // Rule 6: Horn on U (ư) only valid after I or in ươ compound
                // Valid: "iư" (in "giư"), "ươ" (in "trươ", "lươ")
                // Invalid: "eư" (new+w), "aư", "oư" (except ươ compound)
                if vowel_keys[1] == keys::U && vowel_tones[1] == tone::HORN {
                    // Check if V1 is I (iư valid) or if it's ươ compound (U+O with horns)
                    let is_i_u = vowel_keys[0] == keys::I;
                    let is_uo_compound = vowel_keys[0] == keys::U && vowel_tones[0] == tone::HORN;

                    if !is_i_u && !is_uo_compound {
                        // ư after vowels other than i or ươ is invalid
                        return false;
                    }
                }

                let triple = [vowel_keys[0], vowel_keys[1], vowel_keys[2]];

                // Rule 4: U+Y+E requires circumflex on E
                // Valid: "uyê", Invalid: "uye", "uyeư"
                if triple == [keys::U, keys::Y, keys::E] {
                    if vowel_tones[2] == tone::HORN {
                        return false;
                    }
                }

                // Rule 5: I+E+U / Y+E+U require circumflex on E, U can't have horn
                // Valid: "iêu", "yêu"
                // Invalid: "ieư", "ieu", "yeư", "yeu"
                if triple == [keys::I, keys::E, keys::U] || triple == [keys::Y, keys::E, keys::U] {
                    // E (middle) must have circumflex
                    if vowel_tones[1] != tone::CIRCUMFLEX {
                        return false;
                    }
                    // U (last) can't have horn
                    if vowel_tones[2] == tone::HORN {
                        return false;
                    }
                }

                // Rule 7: U (ư) cannot be the 3rd vowel
                // "uou" where last u has horn -> "uoư" is invalid (e.g. from "quow")
                if vowel_keys[2] == keys::U && vowel_tones[2] == tone::HORN {
                    return false;
                }
            }
            _ => {}
        }

        true
    }

    #[inline]
    fn is_allowed_exception(k1: u16, k2: u16) -> bool {
        // Handle vowel-vowel sequences and common clusters not in bigram matrix yet
        if k1 >= 128 || k2 >= 128 {
            return true; // Unicode characters (diacritics) are definitely Vietnamese
        }

        let p1 = CHAR_PROPS[k1 as usize];
        let p2 = CHAR_PROPS[k2 as usize];

        if (p1 & PROP_VOWEL) != 0 && (p2 & PROP_VOWEL) != 0 {
            // Exceptions for vowel compounds not easily represented in bigram matrix
            // Use the authoritative reference for valid 2-vowel combinations
            return Self::is_valid_2vowel_combo(k1, k2);
        }

        // Consonant-vowel check is already handled by bigram matrix for all consonants

        // Allowed consonant clusters
        matches!(
            (k1, k2),
            (keys::T, keys::R)
                | (keys::T, keys::H)
                | (keys::C, keys::H)
                | (keys::N, keys::H)
                | (keys::N, keys::G)
                | (keys::P, keys::H)
                | (keys::K, keys::H)
                | (keys::G, keys::I)
                | (keys::Q, keys::U)
                | (keys::Y, keys::T) // buýt, huýt, xuýt
                | (keys::Y, keys::C) // huých, uỵch
                // Exceptions for ethnic minority names
                | (keys::A, keys::K) // Đăk Lăk
                | (keys::O, keys::K) // Kon Tum
        )
    }

    #[inline]
    fn is_invalid_consonant_cluster(k1: u16, k2: u16) -> bool {
        // Reject English/French consonant clusters not allowed in Vietnamese
        match k1 {
            keys::B => k2 == keys::L || k2 == keys::R, // bl, br
            keys::C => k2 == keys::L || k2 == keys::R, // cl, cr (note: ch is valid)
            keys::F => k2 == keys::L || k2 == keys::R, // fl, fr
            keys::G => k2 == keys::L || k2 == keys::R, // gl, gr (note: gh, gi are valid)
            keys::P => k2 == keys::L || k2 == keys::R, // pl, pr (note: ph is valid)
            keys::D => k2 == keys::R || k2 == keys::W, // dr, dw
            keys::S => matches!(
                k2,
                keys::C | keys::K | keys::L | keys::M | keys::N | keys::P | keys::W
            ), // sc, sk, sl, sm, sn, sp, sw
            keys::T => k2 == keys::W,                  // tw (note: tr, th are valid)
            _ => false,
        }
    }

    #[inline]
    fn violates_ck_distribution(k1: u16, k2: u16) -> bool {
        // Vietnamese has strict rules for c/k/g/gh/ng/ngh distribution:
        // - c before a, ă, â, o, ô, ơ, u, ư
        // - k before e, ê, i, y
        // - g before a, ă, â, o, ô, ơ, u, ư
        // - gh before e, ê, i
        // - ng before a, ă, â, o, ô, ơ, u, ư
        // - ngh before e, ê, i

        // Detect violations:
        // ce, ci (should be ke, ki)
        if k1 == keys::C && matches!(k2, keys::E | keys::I | keys::Y) {
            return true;
        }

        // ka, ko, ku (should be ca, co, cu)
        if k1 == keys::K && matches!(k2, keys::A | keys::O | keys::U) {
            return true;
        }

        // Note: ge, gi are valid but 'gi' is a separate phoneme /z/
        // gha, gho need to check in 3-char sequences

        false
    }

    #[inline]
    fn is_invalid_vowel_before_ch(vowel: u16) -> bool {
        // Only a, ê, i are valid before -ch
        // This checks raw input, diacritics ê would be handled differently
        // For now, check if it's an invalid base vowel
        // Invalid: o, u before ch (ô, ơ, ư would have diacritics already)
        matches!(vowel, keys::O | keys::U)
    }

    #[inline]
    fn is_invalid_vowel_before_nh(vowel: u16) -> bool {
        // Only a, ê, i, y are valid before -nh
        // Invalid: o, u before nh
        matches!(vowel, keys::O | keys::U)
    }

    /// Validate initial consonant based on OpenKey's comprehensive table
    /// Vietnamese allows specific single consonants and clusters at the start
    #[inline]
    fn is_valid_initial_consonant(keys: &[u16]) -> bool {
        if keys.is_empty() {
            return true;
        }

        let len = keys.len();
        let k1 = keys[0];

        // Check for vowel start (always valid in Vietnamese)
        if matches!(
            k1,
            keys::A | keys::E | keys::I | keys::O | keys::U | keys::Y
        ) {
            return true;
        }

        // Check 3-character clusters first (ngh)
        if len >= 3 {
            let k2 = keys[1];
            let k3 = keys[2];
            if k1 == keys::N && k2 == keys::G && k3 == keys::H {
                return true; // ngh
            }
        }

        // Check 2-character clusters
        if len >= 2 {
            let k2 = keys[1];
            if matches!(
                (k1, k2),
                (keys::P, keys::H)   // ph
                    | (keys::T, keys::H) // th
                    | (keys::T, keys::R) // tr
                    | (keys::G, keys::I) // gi
                    | (keys::C, keys::H) // ch
                    | (keys::N, keys::H) // nh
                    | (keys::N, keys::G) // ng
                    | (keys::K, keys::H) // kh
                    | (keys::G, keys::H) // gh
                    | (keys::Q, keys::U) // qu
            ) {
                return true;
            }
        }

        // Check single consonants (from OpenKey's _consonantTable)
        // Valid: B, C, D, G, H, K, L, M, N, P, Q, R, S, T, V, X
        // Invalid: F, J, W, Z (unless specifically allowed in some contexts)
        matches!(
            k1,
            keys::B
                | keys::C
                | keys::D
                | keys::G
                | keys::H
                | keys::K
                | keys::L
                | keys::M
                | keys::N
                | keys::P
                | keys::Q
                | keys::R
                | keys::S
                | keys::T
                | keys::V
                | keys::X
        )
    }

    /// Validate vowel sequences based on OpenKey's _vowelCombine table
    /// Vietnamese has specific rules for which vowel combinations are valid
    #[inline]
    fn is_valid_vowel_sequence(keys: &[u16]) -> bool {
        if keys.len() < 2 {
            return true; // Single vowel always valid
        }

        // Find vowel sequence in the syllable
        let mut vowel_start = None;
        let mut vowel_end = None;
        let mut finished_vowel_block = false;

        for (i, &k) in keys.iter().enumerate() {
            let is_vowel = matches!(k, keys::A | keys::E | keys::I | keys::O | keys::U | keys::Y);
            println!(
                "DEBUG: Loop i={} k={} is_vowel={} finished={}",
                i, k, is_vowel, finished_vowel_block
            );

            if is_vowel {
                if finished_vowel_block {
                    println!("DEBUG: Found multi-syllable key {} at index {}", k, i);
                    // Found a second vowel block after consonants -> Multi-syllable/Invalid
                    return false;
                }

                if vowel_start.is_none() {
                    vowel_start = Some(i);
                }
                vowel_end = Some(i);
            } else if vowel_start.is_some() {
                // Hit a consonant after vowels, mark block as finished
                finished_vowel_block = true;
            }
        }

        let Some(mut start) = vowel_start else {
            return true; // No vowels found, let other rules handle
        };
        let Some(end) = vowel_end else {
            return true;
        };

        // Special handling for 'gi':
        // If 'gi' is followed by another vowel (e.g., "giây", "giếng"),
        // the 'i' is part of the consonant digraph 'gi', not the vowel nucleus.
        // We should skip it for vowel sequence validation.
        if keys[start] == keys::I && start > 0 && keys[start - 1] == keys::G {
            // Check if there are other vowels after 'i'
            if end > start {
                start += 1;
            }
        }

        if start == end {
            return true; // Single vowel
        }

        // Check 2-vowel combinations (from OpenKey's _vowelCombine)
        if end - start == 1 {
            let v1 = keys[start];
            let v2 = keys[end];
            return Self::is_valid_2vowel_combo(v1, v2);
        }

        // Check 3-vowel combinations
        if end - start == 2 {
            let v1 = keys[start];
            let v2 = keys[start + 1];
            let v3 = keys[end];
            return Self::is_valid_3vowel_combo(v1, v2, v3);
        }

        // More than 3 vowels is generally invalid in Vietnamese
        false
    }

    /// Valid 2-vowel combinations from OpenKey's _vowelCombine
    #[inline]
    fn is_valid_2vowel_combo(v1: u16, v2: u16) -> bool {
        matches!(
            (v1, v2),
            // A combinations
            (keys::A, keys::I) | (keys::A, keys::O) | (keys::A, keys::U) | (keys::A, keys::Y)
            // E combinations
            | (keys::E, keys::O) | (keys::E, keys::U)
            // I combinations
            | (keys::I, keys::A) | (keys::I, keys::E) | (keys::I, keys::U) | (keys::I, keys::O)
            // O combinations
            | (keys::O, keys::A) | (keys::O, keys::E) | (keys::O, keys::I) | (keys::O, keys::O) | (keys::O, keys::U)
            // U combinations
            | (keys::U, keys::A) | (keys::U, keys::E) | (keys::U, keys::I) | (keys::U, keys::O) | (keys::U, keys::U) | (keys::U, keys::Y)
            // Y combinations
            | (keys::Y, keys::A) | (keys::Y, keys::E)
        )
    }

    /// Valid 3-vowel combinations from OpenKey's _vowelCombine
    #[inline]
    fn is_valid_3vowel_combo(v1: u16, v2: u16, v3: u16) -> bool {
        matches!(
            (v1, v2, v3),
            // O combinations
            (keys::O, keys::A, keys::I) | (keys::O, keys::A, keys::O) | (keys::O, keys::A, keys::Y) | (keys::O, keys::E, keys::O)
            // U combinations
            | (keys::U, keys::Y, keys::U) | (keys::U, keys::Y, keys::E) | (keys::U, keys::Y, keys::A)
            | (keys::U, keys::A, keys::I) | (keys::U, keys::A, keys::Y) | (keys::U, keys::A, keys::O) // quào (u-a-o)
            | (keys::U, keys::E, keys::O)
            // I combinations
            | (keys::I, keys::E, keys::U)
            | (keys::Y, keys::E, keys::U)
            // U-O combinations (ươ)
            | (keys::U, keys::O, keys::I) | (keys::U, keys::O, keys::U)
        )
    }

    /// Validate vowel-coda compatibility from OpenKey's validation logic
    /// Vietnamese has strict rules about which vowels can precede which end consonants
    #[inline]
    fn is_valid_vowel_coda_pair(keys: &[u16]) -> bool {
        let len = keys.len();
        if len < 3 {
            return true; // Too short to have vowel + coda
        }

        // Check for end consonant clusters
        let last = keys[len - 1];
        let prev = keys[len - 2];

        // -ch ending
        if last == keys::H && prev == keys::C {
            return Self::is_valid_vowel_before_ch_comprehensive(keys, len);
        }

        // -nh ending
        if last == keys::H && prev == keys::N {
            return Self::is_valid_vowel_before_nh_comprehensive(keys, len);
        }

        // -ng ending
        if last == keys::G && prev == keys::N {
            return Self::is_valid_vowel_before_ng(keys, len);
        }

        true // Other codas are handled by existing rules
    }

    /// Comprehensive check for vowels before -ch
    /// Valid: a, ê, i (ach, êch, ich)
    /// Invalid: ô, ơ, u, ư (ôch, ơch, uch, ưch)
    #[inline]
    fn is_valid_vowel_before_ch_comprehensive(keys: &[u16], len: usize) -> bool {
        if len < 3 {
            return true;
        }

        // Find the vowel sequence before -ch
        let vowel_end = len - 3; // Position before 'c' in 'ch'

        // Scan backwards to find all vowels
        let mut has_invalid_vowel = false;
        for i in (0..=vowel_end).rev() {
            let k = keys[i];
            if matches!(k, keys::O | keys::U) {
                // O or U before -ch is only invalid if it's the main vowel (immediately before coda)
                // They are valid as medial vowels (e.g., oach, uach, oạch, quạch)
                if i == vowel_end {
                    has_invalid_vowel = true;
                }
            }
            if !matches!(k, keys::A | keys::E | keys::I | keys::O | keys::U | keys::Y) {
                break; // Hit a consonant
            }
        }

        !has_invalid_vowel
    }

    /// Comprehensive check for vowels before -nh
    /// Valid: a, ê, i, y (anh, ênh, inh, ynh)
    /// Invalid: ô, ơ, u, ư (ônh, ơnh, unh, ưnh)
    #[inline]
    fn is_valid_vowel_before_nh_comprehensive(keys: &[u16], len: usize) -> bool {
        if len < 3 {
            return true;
        }

        let vowel_end = len - 3;
        let mut has_invalid_vowel = false;

        for i in (0..=vowel_end).rev() {
            let k = keys[i];
            if matches!(k, keys::O | keys::U) {
                // O or U before -nh is only invalid if it's the main vowel
                // Valid as medial: oanh, uanh (quanh, doanh)
                if i == vowel_end {
                    has_invalid_vowel = true;
                }
            }
            if !matches!(k, keys::A | keys::E | keys::I | keys::O | keys::U | keys::Y) {
                break;
            }
        }

        !has_invalid_vowel
    }

    /// Check for vowels before -ng
    /// Invalid: e, ê (eng, êng should use -nh instead)
    /// Valid: a, o, u, i, y
    #[inline]
    fn is_valid_vowel_before_ng(_keys: &[u16], _len: usize) -> bool {
        // Relaxed rule: Allow E before NG (e.g., "xà beng", "leng keng", "cà mèng")
        // While "êng" is technically invalid (should be "ênh"), "eng" is valid.
        // Since we check raw keys here (E = e or ê), we must be permissive.
        true
    }
}
