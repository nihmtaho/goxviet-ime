use crate::data::keys;
use crate::engine_v2::fsm::tables::{CHAR_PROPS, PROP_CONSONANT};

pub struct PhonotacticResult {
    pub english_confidence: u8,
    pub is_english: bool,
}

pub struct PhonotacticEngine;

impl PhonotacticEngine {
    /// O(1) analysis of key sequence using 8-layer phonotactic model
    #[inline]
    pub fn analyze(keys: &[(u16, bool)]) -> PhonotacticResult {
        if keys.is_empty() {
            return PhonotacticResult {
                english_confidence: 0,
                is_english: false,
            };
        }

        let len = keys.len();
        let mut english_score = 0;

        // LAYER 1: Single letters not in Vietnamese (f, j, w, z)
        let first = keys[0].0;
        let last = keys[len - 1].0;

        // 'Z' at the start always indicates English/Foreign (Layer 1 fix)
        if first == keys::Z {
            return PhonotacticResult {
                english_confidence: 100,
                is_english: true,
            };
        }

        if matches!(first, keys::F | keys::J | keys::W) {
            english_score += 100;
        }
        if len > 1
            && matches!(
                last,
                keys::F | keys::J | keys::W | keys::Z | keys::S | keys::X
            )
        {
            // Special case: 'w' at the end is a Vietnamese modifier for 'u', 'o', 'a'
            let prev = keys[len - 2].0;
            let is_telex_vowel_modifier =
                last == keys::W && matches!(prev, keys::U | keys::O | keys::A);

            if !is_telex_vowel_modifier {
                // s and x at the end of raw keys are typical of English (bus, box)
                // though they are also tone marks in Telex (as, ax).
                // We give it a high score but rely on LanguageDecisionEngine to combine with dictionary/validator.
                english_score += 90;
            }
        }

        // LAYER 2: Non-coda consonants in Vietnamese at end (b, d, g, k, l, r, v)
        if len > 1
            && (CHAR_PROPS[last as usize] & crate::engine_v2::fsm::tables::PROP_CODA_INVALID) != 0
        {
            // Already handled F, J, W, Z, S, X in Layer 1
            if !matches!(
                last,
                keys::F | keys::J | keys::W | keys::Z | keys::S | keys::X
            ) {
                // Exceptions: 'nh', 'ng', 'ch' are valid codas but contain 'h/g/h'
                let prev = keys[len - 2].0;
                let is_viet_coda = match last {
                    keys::H => prev == keys::N || prev == keys::C,
                    keys::G => prev == keys::N,
                    keys::W => matches!(prev, keys::U | keys::O | keys::A), // Telex modifiers
                    _ => false,
                };
                if !is_viet_coda {
                    english_score += 85;
                }
            }
        }

        if len >= 2 {
            let k1 = keys[0].0;
            let k2 = keys[1].0;

            // LAYER 3 & 4 & 5 & 6 & 7: Onset clusters
            if Self::is_english_onset(k1, k2) {
                english_score += 90;
            }

            if len >= 3 {
                let k3 = keys[2].0;
                if Self::is_3_consonant_onset(k1, k2, k3) {
                    english_score += 95;
                }
            }
        }

        // CODA CLUSTERS (Layer 3-8)
        if len >= 2 {
            let k_penult = keys[len - 2].0;
            let k_last = keys[len - 1].0;
            if Self::is_english_coda(k_penult, k_last) {
                english_score += 90;
            }

            if len >= 3 {
                let k_3rd_last = keys[len - 3].0;
                if Self::is_3_consonant_coda(k_3rd_last, k_penult, k_last) {
                    english_score += 95;
                }
            }
        }

        // SPECIAL RULE: Double consonants
        for i in 0..len.saturating_sub(1) {
            let k1 = keys[i].0;
            let k2 = keys[i + 1].0;
            if k1 == k2 && (CHAR_PROPS[k1 as usize] & PROP_CONSONANT) != 0 {
                // Vietnamese ONLY allows 'dd' (Đ)
                if k1 != keys::D {
                    english_score += 95;
                    break;
                }
            }
        }

        // LAYER 9: English Suffixes (-ified, -ous, -ory, -ine)
        if Self::is_english_suffix(keys) {
            return PhonotacticResult {
                english_confidence: 100,
                is_english: true,
            };
        }

        PhonotacticResult {
            english_confidence: (english_score as u8).min(100),
            is_english: english_score >= 80,
        }
    }

    #[inline]
    fn is_english_suffix(keys: &[(u16, bool)]) -> bool {
        let len = keys.len();
        if len < 3 {
            return false;
        }

        // Check last 2 chars
        if len >= 2 {
            let h2 = (keys[len - 2].0 as u32)
                .wrapping_mul(31)
                .wrapping_add(keys[len - 1].0 as u32);

            if matches!(h2, 447 | 1062) {
                // ew (brew, view, new), ic (basic, magic, music)
                return true;
            }
        }

        // Check last 3 chars
        let h3 = (keys[len - 3].0 as u32)
            .wrapping_mul(31)
            .wrapping_add(keys[len - 2].0 as u32)
            .wrapping_mul(31)
            .wrapping_add(keys[len - 1].0 as u32);

        if matches!(h3, 30784 | 30272 | 34083) {
            // ous, ory, ine
            return true;
        }

        // Check last 4 chars for -tion, -sion
        if len >= 4 {
            let h4 = (keys[len - 4].0 as u32)
                .wrapping_mul(31)
                .wrapping_add(keys[len - 3].0 as u32)
                .wrapping_mul(31)
                .wrapping_add(keys[len - 2].0 as u32)
                .wrapping_mul(31)
                .wrapping_add(keys[len - 1].0 as u32);

            if matches!(h4, 540127 | 63471) {
                // tion, sion
                return true;
            }
        }

        // Check last 5 chars (-ified)
        if len >= 5 {
            let h5 = (keys[len - 5].0 as u32)
                .wrapping_mul(31)
                .wrapping_add(keys[len - 4].0 as u32)
                .wrapping_mul(31)
                .wrapping_add(keys[len - 3].0 as u32)
                .wrapping_mul(31)
                .wrapping_add(keys[len - 2].0 as u32)
                .wrapping_mul(31)
                .wrapping_add(keys[len - 1].0 as u32);

            if h5 == 31522197 {
                return true;
            }
        }

        false
    }

    #[inline]
    fn is_english_onset(k1: u16, k2: u16) -> bool {
        match k1 {
            keys::B | keys::C | keys::F | keys::G => k2 == keys::L || k2 == keys::R,
            keys::P => k2 == keys::L || k2 == keys::R || k2 == keys::S, // pl-, pr-, ps- (ph- is Viet)
            keys::D => k2 == keys::R,                                   // dr- (dd- is Viet)
            keys::S => matches!(
                k2,
                keys::C | keys::K | keys::L | keys::M | keys::N | keys::P | keys::T | keys::W
            ),
            keys::T => k2 == keys::W, // tw- (tr-, th- are Viet)
            keys::K => k2 == keys::N, // kn- (kh- is Viet)
            _ => false,
        }
    }

    #[inline]
    fn is_3_consonant_onset(k1: u16, k2: u16, k3: u16) -> bool {
        match k1 {
            keys::S => match k2 {
                keys::T => k3 == keys::R,                  // str-
                keys::P => k3 == keys::R || k3 == keys::L, // spr-, spl-
                keys::C => k3 == keys::R,                  // scr-
                keys::H => k3 == keys::R,                  // shr-
                _ => false,
            },
            _ => false,
        }
    }

    #[inline]
    fn is_english_coda(k1: u16, k2: u16) -> bool {
        match k1 {
            keys::L => matches!(k2, keys::D | keys::F | keys::K | keys::P | keys::T),
            keys::R => matches!(
                k2,
                keys::B | keys::D | keys::K | keys::M | keys::N | keys::P | keys::T | keys::O
            ), // Added O for -ron (baron, iron)
            keys::S => matches!(k2, keys::K | keys::P | keys::T | keys::O | keys::N), // Added O, N for -son (mason, reason, season, person, poison, prison, lesson)
            keys::M => k2 == keys::P,                                                 // -mp
            keys::N => matches!(k2, keys::D | keys::T | keys::K), // -nd, -nt, -nk
            keys::T => k2 == keys::S || k2 == keys::O, // -ts, -to (added O for -ton: button, cotton)
            keys::P => k2 == keys::S || k2 == keys::T, // -ps, -pt
            keys::K => k2 == keys::S,                  // -ks
            keys::D => k2 == keys::S,                  // -ds
            keys::B => k2 == keys::S,                  // -bs
            keys::F => k2 == keys::T,                  // -ft
            keys::C => k2 == keys::T,                  // -ct
            _ => false,
        }
    }

    #[inline]
    fn is_3_consonant_coda(k1: u16, k2: u16, k3: u16) -> bool {
        match k2 {
            keys::T => k3 == keys::S && (k1 == keys::F || k1 == keys::P || k1 == keys::N), // -fts, -pts, -nts
            keys::M => k2 == keys::P && k3 == keys::L,                                     // -mpl
            keys::P => k2 == keys::L && (k1 == keys::M || k1 == keys::N), // -mpl, -ndl
            _ => false,
        }
    }
}
