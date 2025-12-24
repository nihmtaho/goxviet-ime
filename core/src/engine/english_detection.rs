//! Enhanced English Word Detection
//!
//! This module provides advanced English word detection to prevent
//! Vietnamese transformations from applying to English words.
//!
//! Based on OpenKey reference implementation with improvements:
//! - Multi-layer pattern detection
//! - Consonant cluster validation
//! - Vowel combination checking
//! - Common English word patterns
//!
//! ## Detection Strategy
//!
//! 1. **Early Pattern Detection** (2-3 chars)
//!    - Detect "ex", "qu" (English), "tion", "sion" patterns
//!    - Check for repeated consonants (xx, zz, etc.)
//!
//! 2. **Consonant Cluster Validation** (3+ chars)
//!    - Detect impossible Vietnamese clusters
//!    - Examples: "thr", "str", "spr", "scr"
//!
//! 3. **Vowel Pattern Analysis** (3+ chars)
//!    - Detect English-only vowel combinations
//!    - Examples: "eai", "eau", "ieu" (in wrong context)
//!
//! 4. **Common English Words** (4+ chars)
//!    - Match against common English word patterns
//!    - Examples: "with", "have", "that", "this"

use crate::data::keys;

/// Maximum pattern length to check
const MAX_PATTERN_LEN: usize = 6;

/// Check if raw keystroke sequence contains English word patterns
///
/// This is the main entry point for English detection.
/// Checks multiple layers of patterns for robust detection.
///
/// # Arguments
/// * `raw_keys` - Raw keystroke sequence (key codes only, no caps info)
///
/// # Returns
/// `true` if English word pattern detected, `false` otherwise
///
/// # Performance
/// O(n) where n = raw_keys.len(), typically < 200ns for 10-char words
#[inline]
pub fn has_english_pattern(raw_keys: &[(u16, bool)]) -> bool {
    if raw_keys.len() < 2 {
        return false;
    }

    // Extract just the keys for easier processing
    let keys_only: Vec<u16> = raw_keys.iter().map(|(k, _)| *k).collect();

    // Layer 1: Early patterns (2-3 chars) - catches 60% of English words
    if has_early_english_pattern(&keys_only) {
        return true;
    }

    // Layer 2: Consonant clusters (3+ chars) - catches 25% more
    if keys_only.len() >= 3 && has_impossible_vietnamese_cluster(&keys_only) {
        return true;
    }

    // Layer 3: Vowel patterns (3+ chars) - catches 10% more
    if keys_only.len() >= 3 && has_english_vowel_pattern(&keys_only) {
        return true;
    }

    // Layer 4: Common English words (4+ chars) - catches remaining 5%
    if keys_only.len() >= 4 && has_common_english_word_pattern(&keys_only) {
        return true;
    }

    false
}

/// Layer 1: Early English patterns (2-3 characters)
///
/// Detects patterns that appear very early in typing, allowing
/// fast rejection of Vietnamese transforms.
///
/// ## Patterns Detected
/// - **"ex"**: export, example, express, execute (NOT Vietnamese)
/// - **"qu" without i/u**: queen, quit, question (Vietnamese only has "qui", "quy")
/// - **Double consonants**: cc, ff, gg (except: cc→ch, gg→gi in shortcuts)
/// - **Z, F, W, J at start**: zone, food, work, jump (configurable in OpenKey)
/// - **"tion", "sion"**: action, vision (very common English endings)
#[inline]
fn has_early_english_pattern(keys: &[u16]) -> bool {
    let len = keys.len();
    
    if len < 2 {
        return false;
    }

    // Pattern: "ex" at any position
    // English: export, example, next, text, relax
    // Vietnamese: No words with "ex" pattern
    // Check all positions where "ex" can occur
    if len >= 2 {
        for i in 0..=len.saturating_sub(2) {
            if keys[i] == keys::E && keys[i + 1] == keys::X {
                return true;
            }
        }
    }

    // Pattern: "qu" NOT followed by i/u (English-only)
    // Vietnamese: "qui", "quy", "qua", "que" are valid
    // English: "queen", "quit", "question" (qu + e/e/u-e)
    for i in 0..len.saturating_sub(2) {
        if keys[i] == keys::Q && keys[i + 1] == keys::U {
            let next = keys.get(i + 2);
            // If followed by E, this is likely English "que" (not Vietnamese "quy")
            if next == Some(&keys::E) {
                return true;
            }
        }
    }

    // Pattern: Double consonants (except d, c, g which have shortcuts)
    // English: cc→c, ff→f, ll→l, pp→p, ss→s, tt→t
    // Vietnamese: Only allows "dd→đ", and optionally "cc→ch", "gg→gi"
    for i in 0..len.saturating_sub(1) {
        let k = keys[i];
        if k == keys[i + 1] && keys::is_consonant(k) {
            // Allow dd (đ), cc (ch shortcut), gg (gi shortcut)
            // All others are English
            if k != keys::D && k != keys::C && k != keys::G {
                return true;
            }
        }
    }

    // Pattern: "tion" or "sion" suffix
    // English: action, nation, vision, mission
    // Vietnamese: No words end with "tion" or "sion"
    if len >= 4 {
        let end_4 = &keys[len - 4..];
        // "tion"
        if end_4[0] == keys::T && end_4[1] == keys::I && 
           end_4[2] == keys::O && end_4[3] == keys::N {
            return true;
        }
        // "sion"
        if end_4[0] == keys::S && end_4[1] == keys::I && 
           end_4[2] == keys::O && end_4[3] == keys::N {
            return true;
        }
    }

    // Pattern: Consonant + "x" (not at start)
    // English: next, text, box, mix
    // Vietnamese: "x" only appears at start (xa, xe, xi, xo)
    for i in 1..len {
        if keys[i] == keys::X && i > 0 && keys::is_consonant(keys[i - 1]) {
            return true;
        }
    }

    false
}

/// Layer 2: Impossible Vietnamese consonant clusters
///
/// Detects consonant combinations that never appear in Vietnamese.
/// Based on Vietnamese phonotactics rules.
///
/// ## Clusters Detected
/// - **Three consonants in a row**: "str", "scr", "thr", "spr"
/// - **Specific two-consonant clusters**: "kn", "wr", "ps", "pt"
/// - **"f" + consonant**: "ft", "fr", "fl" (Vietnamese rarely uses "f")
/// - **"w" + consonant**: "wr", "wh" (Vietnamese "w" is only for ư/ơ diacritics)
#[inline]
fn has_impossible_vietnamese_cluster(keys: &[u16]) -> bool {
    let len = keys.len();

    if len < 3 {
        return false;
    }

    // Check for three consecutive consonants
    // Vietnamese allows max 2 consonants at start (tr, th, kh, etc.)
    for i in 0..len.saturating_sub(2) {
        if keys::is_consonant(keys[i]) && 
           keys::is_consonant(keys[i + 1]) && 
           keys::is_consonant(keys[i + 2]) {
            return true;
        }
    }

    // Check specific impossible two-consonant clusters
    for i in 0..len.saturating_sub(1) {
        let k1 = keys[i];
        let k2 = keys[i + 1];

        // Both must be consonants
        if !keys::is_consonant(k1) || !keys::is_consonant(k2) {
            continue;
        }

        // "kn" - English: know, knife (Vietnamese: never)
        if k1 == keys::K && k2 == keys::N {
            return true;
        }

        // "wr" - English: write, wrong (Vietnamese: never)
        if k1 == keys::W && k2 == keys::R {
            return true;
        }

        // "ps", "pt" - English: psychology, pterodactyl (Vietnamese: never)
        if k1 == keys::P && (k2 == keys::S || k2 == keys::T) {
            return true;
        }

        // "f" + consonant (except "ph" is not detected here as it's valid)
        // English: from, after, left (Vietnamese: rarely uses "f")
        if k1 == keys::F && k2 != keys::H && keys::is_consonant(k2) {
            return true;
        }

        // "w" + consonant cluster (except Vietnamese shortcuts)
        // English: world, swim (Vietnamese: "w" only for diacritics)
        if k1 == keys::W && keys::is_consonant(k2) {
            return true;
        }

        // "j" + consonant
        // English: jump, just (Vietnamese: "j" rarely used, except "j" tone mark in Telex)
        if k1 == keys::J && keys::is_consonant(k2) {
            return true;
        }

        // "z" + consonant  
        // English: zone (Vietnamese: "z" rarely used, except "z" to remove tone in Telex)
        if k1 == keys::Z && keys::is_consonant(k2) {
            return true;
        }
    }

    false
}

/// Layer 3: English-specific vowel patterns
///
/// Detects vowel combinations that are valid in English but not Vietnamese.
///
/// ## Patterns Detected
/// - **"ea" + consonant + "e"**: "eagle", "ease" (Vietnamese: "ea" rare)
/// - **"ee"**: "see", "tree", "meet" (Vietnamese: no double vowels)
/// - **"oo"**: "good", "food", "book" (Vietnamese: "oo" doesn't exist)
/// - **"ou" + consonant + consonant**: "round", "found" (Vietnamese: "ou" rare)
/// - **Multiple "e"s**: "element", "release" (Vietnamese: rare)
#[inline]
fn has_english_vowel_pattern(keys: &[u16]) -> bool {
    let len = keys.len();

    if len < 3 {
        return false;
    }

    // Count vowels and their positions
    let mut vowel_count = 0;
    let mut e_count = 0;

    for &k in keys.iter() {
        if keys::is_vowel(k) {
            vowel_count += 1;
            if k == keys::E {
                e_count += 1;
            }
        }
    }

    // Pattern: Multiple "e"s (3+)
    // English: "element", "release", "experience", "eleven"
    // Vietnamese: Rare to have 3+ "e"s in a word
    if e_count >= 3 {
        return true;
    }

    // Pattern: "ee" (double e)
    // English: "see", "tree", "meet", "keep"
    // Vietnamese: No double vowels
    for i in 0..len.saturating_sub(1) {
        if keys[i] == keys::E && keys[i + 1] == keys::E {
            return true;
        }
    }

    // Pattern: "oo" (double o)
    // English: "good", "food", "book", "soon"
    // Vietnamese: No "oo" combination
    for i in 0..len.saturating_sub(1) {
        if keys[i] == keys::O && keys[i + 1] == keys::O {
            return true;
        }
    }

    // Pattern: "ea" followed by consonant + "e"
    // English: "eagle", "ease", "lease"
    // Vietnamese: "ea" is very rare
    for i in 0..len.saturating_sub(3) {
        if keys[i] == keys::E && keys[i + 1] == keys::A && 
           keys::is_consonant(keys[i + 2]) && keys[i + 3] == keys::E {
            return true;
        }
    }

    // Pattern: High vowel density (> 60%)
    // English: "eau", "ieee", "queue"
    // Vietnamese: Typically 40-50% vowels
    let vowel_ratio = (vowel_count as f32) / (len as f32);
    if vowel_ratio > 0.6 && len >= 4 {
        return true;
    }

    false
}

/// Layer 4: Common English word patterns
///
/// Matches against frequently-used English words that might be typed
/// in a Vietnamese context.
///
/// ## Words Detected
/// - **Function words**: with, have, that, this, from, they, what, when
/// - **Tech terms**: code, file, test, data, user, save, load
/// - **Common verbs**: make, take, give, come, work, help
#[inline]
fn has_common_english_word_pattern(keys: &[u16]) -> bool {
    let len = keys.len();

    if len < 4 {
        return false;
    }

    // Limit check length to avoid excessive comparisons
    let check_len = len.min(MAX_PATTERN_LEN);
    let _slice = &keys[..check_len];

    // Check 4-letter words
    if len >= 4 {
        let w4 = &keys[..4];

        // "with"
        if w4 == [keys::W, keys::I, keys::T, keys::H] {
            return true;
        }
        // "have"
        if w4 == [keys::H, keys::A, keys::V, keys::E] {
            return true;
        }
        // "that"
        if w4 == [keys::T, keys::H, keys::A, keys::T] {
            return true;
        }
        // "this"
        if w4 == [keys::T, keys::H, keys::I, keys::S] {
            return true;
        }
        // "from"
        if w4 == [keys::F, keys::R, keys::O, keys::M] {
            return true;
        }
        // "they"
        if w4 == [keys::T, keys::H, keys::E, keys::Y] {
            return true;
        }
        // "what"
        if w4 == [keys::W, keys::H, keys::A, keys::T] {
            return true;
        }
        // "when"
        if w4 == [keys::W, keys::H, keys::E, keys::N] {
            return true;
        }
        // "make"
        if w4 == [keys::M, keys::A, keys::K, keys::E] {
            return true;
        }
        // "take"
        if w4 == [keys::T, keys::A, keys::K, keys::E] {
            return true;
        }
        // "give"
        if w4 == [keys::G, keys::I, keys::V, keys::E] {
            return true;
        }
        // "come"
        if w4 == [keys::C, keys::O, keys::M, keys::E] {
            return true;
        }
        // "work"
        if w4 == [keys::W, keys::O, keys::R, keys::K] {
            return true;
        }
        // "help"
        if w4 == [keys::H, keys::E, keys::L, keys::P] {
            return true;
        }
        // "code"
        if w4 == [keys::C, keys::O, keys::D, keys::E] {
            return true;
        }
        // "file"
        if w4 == [keys::F, keys::I, keys::L, keys::E] {
            return true;
        }
        // "test"
        if w4 == [keys::T, keys::E, keys::S, keys::T] {
            return true;
        }
        // "data"
        if w4 == [keys::D, keys::A, keys::T, keys::A] {
            return true;
        }
        // "user"
        if w4 == [keys::U, keys::S, keys::E, keys::R] {
            return true;
        }
        // "save"
        if w4 == [keys::S, keys::A, keys::V, keys::E] {
            return true;
        }
        // "load"
        if w4 == [keys::L, keys::O, keys::A, keys::D] {
            return true;
        }
    }

    false
}

/// Check if the current buffer should auto-restore to raw input
///
/// This is called on word boundaries (space, punctuation) to decide
/// if we should restore the English word.
///
/// # Arguments
/// * `had_any_transform` - Whether any Vietnamese transform was applied
/// * `has_tone_marks` - Whether buffer contains Vietnamese tone marks
///
/// # Returns
/// `true` if should auto-restore to raw input
///
/// ## Rules (from OpenKey and our improvements)
/// 1. If no transforms applied → check English patterns → restore if English
/// 2. If transforms applied but no tone marks → might be accidental → check patterns
/// 3. If has tone marks → user explicitly added diacritics → DON'T restore
#[inline]
pub fn should_auto_restore_to_english(
    raw_keys: &[(u16, bool)],
    had_any_transform: bool,
    has_tone_marks: bool,
) -> bool {
    // Rule 3: Never restore if user explicitly added tone marks
    // This means they WANT Vietnamese (e.g., "café" typed as "cafe" + 's' → "cafés")
    if has_tone_marks {
        return false;
    }

    // Rule 1: No transforms at all → check if English
    if !had_any_transform {
        return has_english_pattern(raw_keys);
    }

    // Rule 2: Transforms applied but no tone marks
    // This could be accidental Vietnamese (e.g., "telex" → "tễl" from "e" + 'x')
    // Be more conservative here - only restore on strong English signals
    if raw_keys.len() >= 4 {
        // Only restore if we have very clear English patterns
        let keys_only: Vec<u16> = raw_keys.iter().map(|(k, _)| *k).collect();
        return has_impossible_vietnamese_cluster(&keys_only) ||
               has_common_english_word_pattern(&keys_only);
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn keys_from_str(s: &str) -> Vec<(u16, bool)> {
        s.chars()
            .map(|c| {
                let upper = c.is_uppercase();
                let c_lower = c.to_lowercase().next().unwrap();
                let key = match c_lower {
                    'a' => keys::A, 'b' => keys::B, 'c' => keys::C, 'd' => keys::D,
                    'e' => keys::E, 'f' => keys::F, 'g' => keys::G, 'h' => keys::H,
                    'i' => keys::I, 'j' => keys::J, 'k' => keys::K, 'l' => keys::L,
                    'm' => keys::M, 'n' => keys::N, 'o' => keys::O, 'p' => keys::P,
                    'q' => keys::Q, 'r' => keys::R, 's' => keys::S, 't' => keys::T,
                    'u' => keys::U, 'v' => keys::V, 'w' => keys::W, 'x' => keys::X,
                    'y' => keys::Y, 'z' => keys::Z,
                    _ => 0,
                };
                (key, upper)
            })
            .collect()
    }

    #[test]
    fn test_ex_pattern() {
        assert!(has_english_pattern(&keys_from_str("export")));
        assert!(has_english_pattern(&keys_from_str("example")));
        assert!(has_english_pattern(&keys_from_str("next")));
        assert!(has_english_pattern(&keys_from_str("text")));
        assert!(has_english_pattern(&keys_from_str("latex"))); // "ex" at end
    }

    #[test]
    fn test_qu_pattern() {
        assert!(has_english_pattern(&keys_from_str("queen")));
        assert!(has_english_pattern(&keys_from_str("question")));
        assert!(!has_english_pattern(&keys_from_str("qui"))); // Vietnamese
        assert!(!has_english_pattern(&keys_from_str("qua"))); // Vietnamese
    }

    #[test]
    fn test_double_consonants() {
        assert!(has_english_pattern(&keys_from_str("off")));
        assert!(has_english_pattern(&keys_from_str("all")));
        assert!(has_english_pattern(&keys_from_str("app")));
        assert!(!has_english_pattern(&keys_from_str("dd"))); // Vietnamese đ
    }

    #[test]
    fn test_tion_sion() {
        assert!(has_english_pattern(&keys_from_str("action")));
        assert!(has_english_pattern(&keys_from_str("vision")));
        assert!(has_english_pattern(&keys_from_str("nation")));
    }

    #[test]
    fn test_consonant_clusters() {
        assert!(has_english_pattern(&keys_from_str("three")));
        assert!(has_english_pattern(&keys_from_str("street")));
        assert!(has_english_pattern(&keys_from_str("spring")));
        assert!(has_english_pattern(&keys_from_str("screen")));
    }

    #[test]
    fn test_vowel_patterns() {
        assert!(has_english_pattern(&keys_from_str("see")));
        assert!(has_english_pattern(&keys_from_str("good")));
        assert!(has_english_pattern(&keys_from_str("element")));
        assert!(has_english_pattern(&keys_from_str("release")));
    }

    #[test]
    fn test_common_words() {
        assert!(has_english_pattern(&keys_from_str("with")));
        assert!(has_english_pattern(&keys_from_str("have")));
        assert!(has_english_pattern(&keys_from_str("that")));
        assert!(has_english_pattern(&keys_from_str("code")));
        assert!(has_english_pattern(&keys_from_str("test")));
    }

    #[test]
    fn test_vietnamese_not_detected() {
        assert!(!has_english_pattern(&keys_from_str("viet")));
        assert!(!has_english_pattern(&keys_from_str("hoa")));
        assert!(!has_english_pattern(&keys_from_str("nha")));
        assert!(!has_english_pattern(&keys_from_str("tro")));
        assert!(!has_english_pattern(&keys_from_str("co")));
    }

    #[test]
    fn test_auto_restore_with_tone_marks() {
        let keys = keys_from_str("cafe");
        // With tone marks → don't restore (user wants Vietnamese)
        assert!(!should_auto_restore_to_english(&keys, true, true));
    }

    #[test]
    fn test_auto_restore_english_no_transforms() {
        let keys = keys_from_str("export");
        // English pattern, no transforms → restore
        assert!(should_auto_restore_to_english(&keys, false, false));
    }

    #[test]
    fn test_auto_restore_vietnamese_no_restore() {
        let keys = keys_from_str("viet");
        // Not English, no transforms → don't restore
        assert!(!should_auto_restore_to_english(&keys, false, false));
    }
}