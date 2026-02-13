//! Matrix-Based Phonotactic Analysis Engine
//!
//! 8-layer English phonotactic detection with Vietnamese validation.
//! Provides confidence scores for each detection layer.

use crate::data::keys;

/// Phonotactic detection result with layer-wise confidence
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhonotacticResult {
    /// Overall English confidence (0-100%)
    pub english_confidence: u8,
    /// Layer confidences (0-100%)
    pub layer_scores: [u8; 8],
    /// Which layers matched
    pub matched_layers: u8, // bitmask
}

impl PhonotacticResult {
    /// Check if any layer detected English (>0% confidence)
    pub fn is_english(&self) -> bool {
        self.matched_layers > 0
    }

    /// Get combined confidence (weighted average of matched layers)
    pub fn combined_confidence(&self) -> u8 {
        if self.matched_layers == 0 {
            return 0;
        }

        let mut total = 0u32;
        let mut count = 0u32;

        for (i, &score) in self.layer_scores.iter().enumerate() {
            if (self.matched_layers & (1 << i)) != 0 {
                total += score as u32;
                count += 1;
            }
        }

        if count == 0 {
            0
        } else {
            ((total / count).min(100)) as u8
        }
    }
}

/// Matrix-based phonotactic detection
pub struct PhonotacticEngine;

impl PhonotacticEngine {
    /// Analyze sequence for English phonotactic patterns (8 layers)
    pub fn analyze(keys: &[(u16, bool)]) -> PhonotacticResult {
        let mut result = PhonotacticResult {
            english_confidence: 0,
            layer_scores: [0u8; 8],
            matched_layers: 0,
        };

        if keys.is_empty() {
            return result;
        }

        // Layer 1: Invalid initials (F, J, W, Z)
        result.layer_scores[0] = Self::check_invalid_initials(keys);
        if result.layer_scores[0] > 0 {
            result.matched_layers |= 1 << 0;
        }

        // Layer 2: Onset clusters (bl, br, cl, cr, dr, fl, fr, gl, gr, pl, pr, etc.)
        result.layer_scores[1] = Self::check_onset_clusters(keys);
        if result.layer_scores[1] > 0 {
            result.matched_layers |= 1 << 1;
        }

        // Layer 3: Double consonants (ll, ss, ff, rr, etc.)
        result.layer_scores[2] = Self::check_double_consonants(keys);
        if result.layer_scores[2] > 0 {
            result.matched_layers |= 1 << 2;
        }

        // Layer 4: English suffixes (-tion, -ing, -ed, -ly, -er, -est)
        result.layer_scores[3] = Self::check_suffixes(keys);
        if result.layer_scores[3] > 0 {
            result.matched_layers |= 1 << 3;
        }

        // Layer 5: Coda clusters (st, nd, nt, mp, ng, nk, etc.)
        result.layer_scores[4] = Self::check_coda_clusters(keys);
        if result.layer_scores[4] > 0 {
            result.matched_layers |= 1 << 4;
        }

        // Layer 6: English prefixes (un-, re-, pre-, dis-, over-, etc.)
        result.layer_scores[5] = Self::check_prefixes(keys);
        if result.layer_scores[5] > 0 {
            result.matched_layers |= 1 << 5;
        }

        // Layer 7: Vowel patterns (ea, ou, oo, ai, oi, etc.)
        result.layer_scores[6] = Self::check_vowel_patterns(keys);
        if result.layer_scores[6] > 0 {
            result.matched_layers |= 1 << 6;
        }

        // Layer 8: Impossible bigrams (qb, qd, qf, zs, zn, etc.)
        result.layer_scores[7] = Self::check_impossible_bigrams(keys);
        if result.layer_scores[7] > 0 {
            result.matched_layers |= 1 << 7;
        }

        // Calculate overall confidence (weighted by layer specificity)
        // Weights updated 2026-01: L6 Prefix confidence increased to 95 for strong prefixes (imp-, rest-)
        let weights = [100, 98, 95, 90, 91, 95, 85, 80];
        let mut weighted_sum = 0u32;
        let mut weight_sum = 0u32;

        for (i, &score) in result.layer_scores.iter().enumerate() {
            if (result.matched_layers & (1 << i)) != 0 {
                weighted_sum += (score as u32) * (weights[i] as u32);
                weight_sum += weights[i] as u32;
            }
        }

        result.english_confidence = if weight_sum > 0 {
            ((weighted_sum / weight_sum).min(100)) as u8
        } else {
            0
        };

        result
    }

    /// L1: Check for invalid initials (F, J, W, Z at word start, SH- prefix)
    /// Vietnamese doesn't start with F, J, W, Z in native words
    /// Vietnamese also doesn't have SH- consonant cluster
    fn check_invalid_initials(keys: &[(u16, bool)]) -> u8 {
        if keys.is_empty() {
            return 0;
        }

        let first_key = keys[0].0;

        // F, J, W, Z are NEVER valid Vietnamese initials
        match first_key {
            keys::F | keys::J | keys::W | keys::Z => 100, // Definitely English
            _ => {
                // Check for SH- prefix (Vietnamese doesn't have this cluster)
                if keys.len() >= 2 && first_key == keys::S && keys[1].0 == keys::H {
                    return 100; // Definitely English (sh-, short, shell, share, etc.)
                }
                0
            }
        }
    }

    /// L2: Check for consonant clusters (bl, br, cl, cr, dr, fl, etc.)
    /// Vietnamese allows very limited clusters
    fn check_onset_clusters(keys: &[(u16, bool)]) -> u8 {
        // Valid English onset clusters
        const CLUSTERS: &[&[u16; 2]] = &[
            &[keys::B, keys::L], // bl
            &[keys::B, keys::R], // br
            &[keys::C, keys::L], // cl
            &[keys::C, keys::R], // cr
            &[keys::D, keys::R], // dr
            &[keys::F, keys::L], // fl
            &[keys::F, keys::R], // fr
            &[keys::G, keys::L], // gl
            &[keys::G, keys::R], // gr
            &[keys::P, keys::L], // pl
            &[keys::P, keys::R], // pr
            &[keys::S, keys::C], // sc
            &[keys::S, keys::K], // sk
            &[keys::S, keys::L], // sl
            &[keys::S, keys::M], // sm
            &[keys::S, keys::N], // sn
            &[keys::S, keys::P], // sp
            &[keys::S, keys::T], // st
            &[keys::S, keys::W], // sw
            // Removed: th, tr (Vietnamese compatible)
            &[keys::T, keys::W], // tw
            &[keys::V, keys::R], // vr
            &[keys::W, keys::H], // wh
            &[keys::W, keys::R], // wr
        ];

        if keys.len() < 2 {
            return 0;
        }

        let first = keys[0].0;
        let second = keys[1].0;

        for cluster in CLUSTERS {
            if first == cluster[0] && second == cluster[1] {
                return 98; // Extremely likely English
            }
        }

        0
    }

    /// L3: Check for double consonants (ll, ss, ff, rr, tt, pp, cc, mm, nn, gg, bb, etc.)
    /// Vietnamese doesn't have doubled consonants in same syllable (except dd→đ which is handled by Telex)
    fn check_double_consonants(keys: &[(u16, bool)]) -> u8 {
        // All consonants that can double in English but NOT in Vietnamese
        // Excludes: D (dd→đ in Telex), A/E/O (aa→â, ee→ê, oo→ô in Telex)
        const DOUBLE_CONSONANTS: &[u16] = &[
            keys::B, keys::C, keys::F, keys::G, keys::H, keys::K, keys::L,
            keys::M, keys::N, keys::P, keys::R, keys::S, keys::T, keys::V,
            keys::Z,
        ];

        for i in 0..keys.len().saturating_sub(1) {
            let curr = keys[i].0;
            let next = keys[i + 1].0;

            if curr == next && DOUBLE_CONSONANTS.contains(&curr) {
                return 95; // Likely English
            }
        }

        0
    }

    /// L4: Check for English suffixes
    fn check_suffixes(keys: &[(u16, bool)]) -> u8 {
        // Suffix patterns (last 3-4-5 keys)
        const SUFFIXES_3: &[&[u16; 3]] = &[
            &[keys::I, keys::N, keys::G],     // -ing
            &[keys::E, keys::D, keys::SPACE], // -ed (placeholder)
            &[keys::L, keys::Y, keys::SPACE], // -ly
            &[keys::E, keys::R, keys::SPACE], // -er
            &[keys::O, keys::R, keys::E],     // -ore
            &[keys::I, keys::V, keys::E],     // -ive (active, native, massive)
            &[keys::O, keys::U, keys::S],     // -ous (various, serious, obvious)
            &[keys::A, keys::T, keys::E],     // -ate (private, create, state)
            &[keys::I, keys::T, keys::Y],     // -ity (quality, city, ability)
            &[keys::A, keys::L, keys::L],     // -all (overall, install)
            &[keys::F, keys::U, keys::L],     // -ful (beautiful, careful)
        ];

        const SUFFIXES_4: &[&[u16; 4]] = &[
            &[keys::T, keys::I, keys::O, keys::N], // -tion
            &[keys::S, keys::I, keys::O, keys::N], // -sion
            &[keys::N, keys::E, keys::S, keys::S], // -ness
            &[keys::M, keys::E, keys::N, keys::T], // -ment
            &[keys::A, keys::B, keys::L, keys::E], // -able
            &[keys::I, keys::B, keys::L, keys::E], // -ible
            &[keys::A, keys::N, keys::C, keys::E], // -ance (performance, instance)
            &[keys::E, keys::N, keys::C, keys::E], // -ence (difference, reference)
            &[keys::I, keys::T, keys::E, keys::D], // -ited (limited, visited)
        ];

        const SUFFIXES_5: &[&[u16; 5]] = &[
            &[keys::A, keys::T, keys::I, keys::O, keys::N], // -ation
        ];

        if keys.len() >= 3 {
            for suffix in SUFFIXES_3 {
                let start = keys.len() - 3;
                if &keys[start..start + 3]
                    .iter()
                    .map(|k| k.0)
                    .collect::<Vec<_>>()[..]
                    == &suffix[..]
                {
                    return 90;
                }
            }
        }

        if keys.len() >= 4 {
            for suffix in SUFFIXES_4 {
                let start = keys.len() - 4;
                if &keys[start..start + 4]
                    .iter()
                    .map(|k| k.0)
                    .collect::<Vec<_>>()[..]
                    == &suffix[..]
                {
                    return 90;
                }
            }
        }

        if keys.len() >= 5 {
            for suffix in SUFFIXES_5 {
                let start = keys.len() - 5;
                if &keys[start..start + 5]
                    .iter()
                    .map(|k| k.0)
                    .collect::<Vec<_>>()[..]
                    == &suffix[..]
                {
                    return 90;
                }
            }
        }

        0
    }

    /// L5: Check for coda clusters (st, nd, nt, mp, ng, etc.)
    /// These occur at word end in English
    fn check_coda_clusters(keys: &[(u16, bool)]) -> u8 {
        const CODA_PAIRS: &[&[u16; 2]] = &[
            &[keys::S, keys::T], // st
            &[keys::N, keys::D], // nd
            &[keys::N, keys::T], // nt
            &[keys::M, keys::P], // mp
            // Removed: ng (Vietnamese compatible)
            &[keys::N, keys::K], // nk
            &[keys::L, keys::D], // ld
            &[keys::L, keys::T], // lt
            &[keys::R, keys::D], // rd
            &[keys::R, keys::N], // rn
            &[keys::R, keys::S], // rs
            &[keys::R, keys::T], // rt
            &[keys::F, keys::T], // ft
            &[keys::L, keys::S], // ls
            &[keys::L, keys::Z], // lz
        ];

        if keys.len() >= 2 {
            for pair in CODA_PAIRS {
                for i in 0..keys.len().saturating_sub(1) {
                    let curr = keys[i].0;
                    let next = keys[i + 1].0;

                    if curr == pair[0] && next == pair[1] {
                        // Found a coda pair, but need to check context
                        // Reject coda if it's followed by a vowel that starts a new syllable
                        // (This prevents false positives like "syntax" = "sy-ntax")
                        if i + 2 < keys.len() {
                            let after_coda = keys[i + 2].0;

                            // Special check: reject "nt" specifically when followed by vowel
                            // because "nt" + vowel usually indicates next syllable's initial "nt" cluster
                            // which doesn't exist in English (Vietnamese has this in "tion" → "tion" as separate)
                            if curr == keys::N && next == keys::T && Self::is_vowel(after_coda) {
                                continue; // Skip this false positive
                            }

                            // Other codas before vowel might still be valid syllable boundaries
                            // like "mp" before vowel in "improve"
                        }

                        return 91; // Coda cluster found
                    }
                }
            }
        }

        0
    }

    /// Check if a key is a vowel
    fn is_vowel(key: u16) -> bool {
        matches!(
            key,
            keys::A | keys::E | keys::I | keys::O | keys::U | keys::Y
        )
    }

    /// L6: Check for English prefixes (un-, re-, pre-, dis-, imp-, rest-, etc.)
    fn check_prefixes(keys: &[(u16, bool)]) -> u8 {
        const PREFIXES_3: &[&[u16; 3]] = &[
            &[keys::P, keys::R, keys::E], // pre-
            &[keys::D, keys::I, keys::S], // dis-
            &[keys::O, keys::V, keys::E], // ove-
            &[keys::I, keys::M, keys::P], // imp- (improve, import, implement)
            &[keys::E, keys::X, keys::P], // exp- (express, export, explain)
            &[keys::C, keys::O, keys::N], // con- (control, consider, content)
            &[keys::S, keys::U, keys::B], // sub- (subject, submit)
        ];

        const PREFIXES_4: &[&[u16; 4]] = &[
            &[keys::R, keys::E, keys::S, keys::T], // rest- (restore, restrict)
            &[keys::O, keys::V, keys::E, keys::R], // over- (overall, overcome)
            &[keys::U, keys::N, keys::D, keys::E], // unde- (under, understand)
        ];

        if keys.len() >= 3 {
            for prefix in PREFIXES_3 {
                if keys[0].0 == prefix[0] && keys[1].0 == prefix[1] && keys[2].0 == prefix[2] {
                    return 95;
                }
            }
        }

        if keys.len() >= 4 {
            for prefix in PREFIXES_4 {
                if keys[0].0 == prefix[0]
                    && keys[1].0 == prefix[1]
                    && keys[2].0 == prefix[2]
                    && keys[3].0 == prefix[3]
                {
                    return 95; // Very high confidence for 4-char prefixes (rest-)
                }
            }
        }

        0
    }

    /// L7: Check for English vowel patterns (ea, ou, oo, ai, oi, etc.)
    fn check_vowel_patterns(keys: &[(u16, bool)]) -> u8 {
        const VOWEL_PATTERNS: &[&[u16; 2]] = &[
            &[keys::E, keys::A], // ea
            &[keys::O, keys::U], // ou
                                 // Removed: oo, ee, ai, oi, ue, au (Vietnamese/Telex ambiguity)
        ];

        for i in 0..keys.len().saturating_sub(1) {
            let curr = keys[i].0;
            let next = keys[i + 1].0;

            for pattern in VOWEL_PATTERNS {
                if curr == pattern[0] && next == pattern[1] {
                    return 85;
                }
            }
        }

        0
    }

    /// L8: Check for impossible bigrams in Vietnamese
    fn check_impossible_bigrams(keys: &[(u16, bool)]) -> u8 {
        // These bigrams don't exist in Vietnamese
        const IMPOSSIBLE: &[&[u16; 2]] = &[
            &[keys::Q, keys::B], // qb
            &[keys::Q, keys::D], // qd
            &[keys::Q, keys::F], // qf
            &[keys::Z, keys::S], // zs
            &[keys::Z, keys::N], // zn
            &[keys::J, keys::M], // jm
            &[keys::F, keys::N], // fn
            &[keys::W, keys::G], // wg
            &[keys::X, keys::B], // xb
            &[keys::V, keys::T], // vt
        ];

        for i in 0..keys.len().saturating_sub(1) {
            let curr = keys[i].0;
            let next = keys[i + 1].0;

            for pair in IMPOSSIBLE {
                if curr == pair[0] && next == pair[1] {
                    return 80;
                }
            }
        }

        0
    }
}

/// Auto-restore decision logic
pub struct AutoRestoreDecider;

pub use crate::infrastructure::adapters::validation::vietnamese_validator::{ValidationResult, VietnameseSyllableValidator};

impl AutoRestoreDecider {
    /// Decide whether to restore English word
    ///
    /// # Returns
    /// - true: restore to English (high confidence)
    /// - false: keep Vietnamese transforms
    pub fn should_restore(
        phonotactic: &PhonotacticResult,
        vietnamese_validation: &ValidationResult,
        has_transforms: bool,
    ) -> bool {
        if !has_transforms {
            return false;
        }

        // CRITICAL FIX: If Vietnamese validation shows valid output, NEVER restore
        // This fixes "trường" + space being restored to "truowfng"
        // The raw input looks unusual due to Telex modifiers, but output is valid Vietnamese
        if vietnamese_validation.is_valid {
            return false; // Trust the valid Vietnamese output
        }

        // Strong signal: multiple English layers detected
        let english_layers = (phonotactic.matched_layers.count_ones() as u8).max(1);

        // Vietnamese validation: confidence (0-100)
        let viet_confidence = vietnamese_validation.confidence;

        // Decision matrix:
        // 1. If 3+ English layers AND Vietnamese validates poorly -> RESTORE
        // 2. If 5+ English layers -> RESTORE (very strong signal)
        // 3. If high confidence English AND invalid Vietnamese -> RESTORE

        let english_confidence = phonotactic.english_confidence;

        // High English confidence (>75%) AND Vietnamese validation fails (low confidence)
        if english_confidence > 75 && viet_confidence < 50 {
            return true;
        }

        // Multiple English layers (>2) AND very low Vietnamese confidence
        if english_layers >= 3 && viet_confidence < 30 {
            return true;
        }

        // Exceptional case: phonotactic confidence > 80% and Vietnamese invalid
        if english_confidence >= 80 && !vietnamese_validation.is_valid {
            return true;
        }

        false
    }

    /// Get restore confidence (0-100%)
    pub fn confidence(
        phonotactic: &PhonotacticResult,
        vietnamese_validation: &ValidationResult,
    ) -> u8 {
        let english = phonotactic.english_confidence as i32;
        let viet_invalid_score = (100 - vietnamese_validation.confidence as i32).max(0);

        // Combine signals: English high + Vietnamese low = high restore confidence
        (((english + viet_invalid_score) / 2).min(100)) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layer1_invalid_initials() {
        let keys = vec![(keys::F, false), (keys::O, false), (keys::R, false)];
        let result = PhonotacticEngine::analyze(&keys);
        assert!(
            result.layer_scores[0] > 0,
            "Should detect F as invalid initial"
        );
    }

    #[test]
    fn test_layer2_onset_clusters() {
        let keys = vec![(keys::B, false), (keys::L, false), (keys::U, false)];
        let result = PhonotacticEngine::analyze(&keys);
        assert!(result.layer_scores[1] > 0, "Should detect BL cluster");
    }

    #[test]
    fn test_vietnamese_valid_syllable() {
        let keys = vec![keys::T, keys::O, keys::A, keys::N];
        let result = VietnameseSyllableValidator::validate(&keys);
        assert!(result.is_valid, "TOAN should be valid Vietnamese");
    }

    #[test]
    fn test_vietnamese_invalid_initials() {
        let keys = vec![keys::F, keys::O, keys::R];
        let result = VietnameseSyllableValidator::validate(&keys);
        assert!(!result.is_valid, "FOR should fail (invalid initial)");
    }

    #[test]
    fn test_auto_restore_strong_english() {
        let phonotactic = PhonotacticResult {
            english_confidence: 95,
            layer_scores: [100, 98, 0, 0, 0, 0, 0, 0],
            matched_layers: 0b11,
        };
        let viet = ValidationResult {
            is_valid: false,
            confidence: 0,
        };
        assert!(
            AutoRestoreDecider::should_restore(&phonotactic, &viet, true),
            "Should restore high-confidence English"
        );
    }
}
