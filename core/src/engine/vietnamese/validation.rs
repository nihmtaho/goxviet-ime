//! Vietnamese Syllable Validation
//!
//! Whitelist-based validation for Vietnamese syllables.
//! Uses valid patterns from docs/vietnamese-language-system.md Section 7.6.1

use crate::engine::vietnamese::syllable::{parse, Syllable};
use crate::data::chars::tone;
use crate::data::constants;
use crate::data::keys;

/// Validation result
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationResult {
    Valid,
    InvalidInitial,
    InvalidFinal,
    InvalidSpelling,
    InvalidVowelPattern,
    NoVowel,
}

impl ValidationResult {
    pub fn is_valid(&self) -> bool {
        matches!(self, ValidationResult::Valid)
    }
}

// =============================================================================
// BUFFER SNAPSHOT - Keys + Modifiers for validation
// =============================================================================

/// Snapshot of buffer state for validation
/// Contains both keys and their modifiers (tones)
pub struct BufferSnapshot {
    pub keys: Vec<u16>,
    pub tones: Vec<u8>,
    /// True when tones were explicitly provided (validate modifier requirements)
    /// False when created from keys-only (legacy, skip modifier checks)
    pub has_tone_info: bool,
}

impl BufferSnapshot {
    /// Create from keys only (no modifier info - legacy compatibility)
    /// Modifier requirements will NOT be enforced
    pub fn from_keys(keys: Vec<u16>) -> Self {
        let len = keys.len();
        Self {
            keys,
            tones: vec![0; len],
            has_tone_info: false,
        }
    }
}

// =============================================================================
// VALIDATION RULES
// =============================================================================

/// Rule type: takes buffer snapshot and parsed syllable, returns error or None
type Rule = fn(&BufferSnapshot, &Syllable) -> Option<ValidationResult>;

/// All validation rules in order of priority
const RULES: &[Rule] = &[
    rule_has_vowel,
    rule_valid_initial,
    rule_all_chars_parsed,
    rule_spelling,
    rule_valid_final,
    rule_valid_vowel_pattern,
];

/// Rule 1: Must have at least one vowel
fn rule_has_vowel(_snap: &BufferSnapshot, syllable: &Syllable) -> Option<ValidationResult> {
    if syllable.is_empty() {
        return Some(ValidationResult::NoVowel);
    }
    None
}

/// Rule 2: Initial consonant must be valid Vietnamese
fn rule_valid_initial(snap: &BufferSnapshot, syllable: &Syllable) -> Option<ValidationResult> {
    if syllable.initial.is_empty() {
        return None;
    }

    let initial: Vec<u16> = syllable.initial.iter().map(|&i| snap.keys[i]).collect();

    let is_valid = match initial.len() {
        1 => constants::VALID_INITIALS_1.contains(&initial[0]),
        2 => constants::VALID_INITIALS_2
            .iter()
            .any(|p| p[0] == initial[0] && p[1] == initial[1]),
        3 => initial[0] == keys::N && initial[1] == keys::G && initial[2] == keys::H,
        _ => false,
    };

    if !is_valid {
        return Some(ValidationResult::InvalidInitial);
    }
    None
}

/// Rule 3: All characters must be parsed into syllable structure
fn rule_all_chars_parsed(snap: &BufferSnapshot, syllable: &Syllable) -> Option<ValidationResult> {
    let parsed = syllable.initial.len()
        + syllable.glide.map_or(0, |_| 1)
        + syllable.vowel.len()
        + syllable.final_c.len();

    if parsed != snap.keys.len() {
        return Some(ValidationResult::InvalidFinal);
    }
    None
}

/// Rule 4: Vietnamese spelling rules (c/k, g/gh, ng/ngh)
fn rule_spelling(snap: &BufferSnapshot, syllable: &Syllable) -> Option<ValidationResult> {
    if syllable.initial.is_empty() || syllable.vowel.is_empty() {
        return None;
    }

    let initial: Vec<u16> = syllable.initial.iter().map(|&i| snap.keys[i]).collect();
    let first_vowel = snap.keys[syllable.glide.unwrap_or(syllable.vowel[0])];

    for &(consonant, vowels, _msg) in constants::SPELLING_RULES {
        if initial == consonant && vowels.contains(&first_vowel) {
            return Some(ValidationResult::InvalidSpelling);
        }
    }

    None
}

/// Rule 5: Final consonant must be valid
fn rule_valid_final(snap: &BufferSnapshot, syllable: &Syllable) -> Option<ValidationResult> {
    if syllable.final_c.is_empty() {
        return None;
    }

    let final_c: Vec<u16> = syllable.final_c.iter().map(|&i| snap.keys[i]).collect();

    let is_valid = match final_c.len() {
        1 => constants::VALID_FINALS_1.contains(&final_c[0]),
        2 => constants::VALID_FINALS_2
            .iter()
            .any(|p| p[0] == final_c[0] && p[1] == final_c[1]),
        _ => false,
    };

    if !is_valid {
        return Some(ValidationResult::InvalidFinal);
    }
    None
}

/// Rule 6: Vowel patterns must be valid Vietnamese (WHITELIST approach)
///
/// Validates against 40 valid patterns from docs 7.6.1:
/// - 29 diphthongs (nguyên âm đôi)
/// - 11 triphthongs (nguyên âm ba)
///
/// Modifier requirements (circumflex checks) are ONLY enforced when tone info
/// is available (tones not all zeros). This allows legacy is_valid() to work
/// while is_valid_with_tones() can do full validation.
fn rule_valid_vowel_pattern(
    snap: &BufferSnapshot,
    syllable: &Syllable,
) -> Option<ValidationResult> {
    if syllable.vowel.len() < 2 {
        return None; // Single vowel always valid
    }

    let vowel_indices: &[usize] = &syllable.vowel;
    let vowel_keys: Vec<u16> = vowel_indices.iter().map(|&i| snap.keys[i]).collect();
    let vowel_tones: Vec<u8> = vowel_indices.iter().map(|&i| snap.tones[i]).collect();

    match vowel_keys.len() {
        2 => {
            let pair = [vowel_keys[0], vowel_keys[1]];

            // Check if base pattern is in whitelist
            if !constants::VALID_DIPHTHONGS.contains(&pair) {
                return Some(ValidationResult::InvalidVowelPattern);
            }

            // Only check modifier requirements when tone info was explicitly provided
            // This is the key fix for "new" → "neư" bug
            // E+U requires circumflex on E (êu valid, eu/eư invalid)
            if snap.has_tone_info
                && constants::V1_CIRCUMFLEX_REQUIRED.contains(&pair)
                && vowel_tones[0] != tone::CIRCUMFLEX
            {
                return Some(ValidationResult::InvalidVowelPattern);
            }

            // V2 circumflex requirements (I+E → iê, U+E → uê, Y+E → yê)
            // Only check when tone info provided AND V2 has wrong modifier
            if snap.has_tone_info && constants::V2_CIRCUMFLEX_REQUIRED.contains(&pair) {
                // If V2 has horn modifier instead of circumflex, it's invalid
                // But if V2 has no modifier yet, allow it (modifier may come later)
                if vowel_tones[1] == tone::HORN {
                    return Some(ValidationResult::InvalidVowelPattern);
                }
            }

            // Breve (ă) restrictions: 'ă' cannot be followed by another vowel
            // Valid: ăm, ăn, ăng, ănh, ăp, ăt, ăc (consonant endings)
            // Valid: oă (in "xoăn" etc.)
            // Invalid: ăi, ăo, ău, ăy (breve + vowel)
            // In Vietnamese, horn tone on 'a' creates breve 'ă'
            if snap.has_tone_info && vowel_keys[0] == keys::A && vowel_tones[0] == tone::HORN {
                // A with breve followed by vowel is invalid
                // (V2 in diphthong is always a vowel, so this is always invalid)
                return Some(ValidationResult::InvalidVowelPattern);
            }
        }
        3 => {
            let triple = [vowel_keys[0], vowel_keys[1], vowel_keys[2]];

            // Check if base pattern is in whitelist
            if !constants::VALID_TRIPHTHONGS.contains(&triple) {
                return Some(ValidationResult::InvalidVowelPattern);
            }

            // Triphthong modifier checks only when tone info provided
            if snap.has_tone_info {
                // uyê requires circumflex on E (last vowel)
                if triple == [keys::U, keys::Y, keys::E] && vowel_tones[2] == tone::HORN {
                    return Some(ValidationResult::InvalidVowelPattern);
                }
            }
        }
        _ => {
            // More than 3 vowels is always invalid
            return Some(ValidationResult::InvalidVowelPattern);
        }
    }

    None
}

// =============================================================================
// PUBLIC API
// =============================================================================

/// Validate buffer as Vietnamese syllable - runs all rules
pub fn validate(snap: &BufferSnapshot) -> ValidationResult {
    if snap.keys.is_empty() {
        return ValidationResult::NoVowel;
    }

    let syllable = parse(&snap.keys);

    for rule in RULES {
        if let Some(error) = rule(snap, &syllable) {
            return error;
        }
    }

    ValidationResult::Valid
}

/// Quick check if buffer could be valid Vietnamese (with modifier info)
/// This will fully validate modifier requirements (e.g., E+U requires circumflex)
pub fn is_valid_with_tones(keys: &[u16], tones: &[u8]) -> bool {
    let snap = BufferSnapshot {
        keys: keys.to_vec(),
        tones: tones.to_vec(),
        has_tone_info: true, // Enforce modifier requirements
    };
    validate(&snap).is_valid()
}

/// Quick check if buffer could be valid Vietnamese (keys only - legacy)
///
/// NOTE: This cannot fully validate modifier requirements.
/// Use is_valid_with_tones() for complete validation.
pub fn is_valid(buffer_keys: &[u16]) -> bool {
    let snap = BufferSnapshot::from_keys(buffer_keys.to_vec());
    validate(&snap).is_valid()
}

/// Zero-allocation validation using iterator over buffer
/// Returns true if buffer structure is valid for transformation.
/// Does NOT check vowel patterns since intermediate states like "aa" → "â" are valid.
#[inline]
pub fn is_valid_for_transform_iter<'a, I>(buffer_iter: I) -> bool
where
    I: Iterator<Item = &'a u16> + Clone,
{
    // Quick check: must have at least one element
    let mut iter_clone = buffer_iter.clone();
    if iter_clone.next().is_none() {
        return false;
    }

    // Fast path: if buffer is small (1-3 chars), skip expensive validation
    let count = buffer_iter.clone().count();
    if count <= 3 {
        // Most 1-3 char sequences are valid intermediate states
        // Only check for obvious invalid patterns
        let keys: Vec<u16> = buffer_iter.copied().collect();
        let syllable = parse(&keys);
        
        // Must have at least one vowel
        if syllable.vowel.is_empty() {
            return false;
        }
        
        return true;
    }

    // Slow path: full validation for longer buffers
    let keys: Vec<u16> = buffer_iter.copied().collect();
    let snap = BufferSnapshot::from_keys(keys);
    let syllable = parse(&snap.keys);

    for rule in RULES_FOR_TRANSFORM {
        if rule(&snap, &syllable).is_some() {
            return false;
        }
    }

    true
}

/// Rules for pre-transformation validation (excludes vowel pattern check)
/// Used to validate buffer structure before applying tone/mark transformations.
/// Allows intermediate states like "aa" that become valid after transformation.
const RULES_FOR_TRANSFORM: &[Rule] = &[
    rule_has_vowel,
    rule_valid_initial,
    rule_all_chars_parsed,
    rule_spelling,
    rule_valid_final,
    // NOTE: rule_valid_vowel_pattern is excluded - applied only to final results
];

/// Pre-transformation validation (allows intermediate vowel patterns)
///
/// Used by try_tone/try_stroke to validate buffer structure before transformation.
/// Does NOT check vowel patterns since intermediate states like "aa" → "â" are valid.
pub fn is_valid_for_transform(buffer_keys: &[u16]) -> bool {
    is_valid_for_transform_iter(buffer_keys.iter())
}

/// Zero-allocation foreign word pattern check using iterator
/// Returns true if vowel pattern is NOT in valid Vietnamese whitelist.
#[inline]
pub fn is_foreign_word_pattern_iter<'a, I>(buffer_iter: I, modifier_key: u16) -> bool
where
    I: Iterator<Item = &'a u16> + Clone,
{
    // Collect keys only once
    let buffer_keys: Vec<u16> = buffer_iter.copied().collect();
    is_foreign_word_pattern(&buffer_keys, modifier_key)
}

/// Check if the buffer shows patterns that suggest foreign word input.
///
/// Returns true if vowel pattern is NOT in valid Vietnamese whitelist.
/// 
/// NOTE: This function only checks raw keys without tone/modifier information.
/// It may incorrectly flag valid Vietnamese patterns that use diacritics.
/// Callers should check for Vietnamese transforms before using this.
pub fn is_foreign_word_pattern(buffer_keys: &[u16], modifier_key: u16) -> bool {
    let syllable = parse(buffer_keys);

    // Check 1: Invalid vowel patterns (not in whitelist)
    if syllable.vowel.len() >= 2 {
        let vowels: Vec<u16> = syllable.vowel.iter().map(|&i| buffer_keys[i]).collect();

        // Check consecutive pairs for common foreign patterns
        // IMPORTANT: This check only looks at raw keys, not modifiers.
        // Vietnamese "ươ" (u+o with horn) appears as [U, O] in raw keys,
        // same as English "ou". This is why we need to check for Vietnamese
        // transforms in the caller before rejecting.
        for window in vowels.windows(2) {
            let pair = [window[0], window[1]];
            // "ou" and "yo" are common in English but never valid in Vietnamese
            // EXCEPT: "uo" with horn modifiers (ư + ơ) is valid as ươ compound
            // This cannot be detected here - caller must check for horn transforms
            if pair == [keys::O, keys::U] || pair == [keys::Y, keys::O] {
                return true;
            }
            // "oo" is common in English (look, book, took) but never valid in Vietnamese
            if pair == [keys::O, keys::O] {
                return true;
            }
        }

        let is_valid_pattern = match vowels.len() {
            2 => {
                let pair = [vowels[0], vowels[1]];
                constants::VALID_DIPHTHONGS.contains(&pair)
            }
            3 => {
                let triple = [vowels[0], vowels[1], vowels[2]];
                constants::VALID_TRIPHTHONGS.contains(&triple)
            }
            _ => false,
        };

        if !is_valid_pattern {
            return true;
        }
    }

    // Check 2: Consonant clusters common in foreign words (T+R, P+R, C+R)
    if modifier_key == keys::R && syllable.final_c.len() == 1 && !syllable.initial.is_empty() {
        let final_key = buffer_keys[syllable.final_c[0]];
        if matches!(final_key, keys::T | keys::P | keys::C) {
            return true;
        }
    }

    // Check 3: Common English prefix patterns (de + s → describe, design)
    if modifier_key == keys::S
        && syllable.initial.len() == 1
        && syllable.vowel.len() == 1
        && syllable.final_c.is_empty()
    {
        let initial = buffer_keys[syllable.initial[0]];
        let vowel = buffer_keys[syllable.vowel[0]];

        if initial == keys::D && vowel == keys::E {
            return true;
        }
    }

    // Check 4: English words ending with vowel + 'x' modifier (ONLY specific English words)
    // Pattern: specific_initial + vowel + 'x'
    // Note: 'x' in Telex is BOTH a final consonant AND a tone modifier (ngã mark).
    // 
    // MUST BE SPECIFIC: Only block known English patterns, not Vietnamese!
    // - Block: "fix" (f+i+x), "hex" (h+e+x) - ONLY with X modifier
    // - Allow: "hej" (h+e+j) → "hệ" is valid Vietnamese (Issue #27)
    // - Allow: "mix" (m+i+x) → "mĩ" is valid Vietnamese
    // - Allow: "six" (s+i+x) → "sĩ" is valid Vietnamese
    // - Allow: "tax" (t+a+x) → "tã" is valid Vietnamese
    // - Allow: "tex" (t+e+x) → "tẽ" is valid Vietnamese (will be detected as English later if "text" typed)
    // - Allow: "sex" (s+e+x) → "sẽ" is valid Vietnamese
    //
    // Strategy: Only block very specific English patterns that are NEVER Vietnamese
    // CRITICAL: Only block X modifier, NOT J modifier - J is for nặng tone which is valid Vietnamese
    // Issue #27: "heej" → "hệ" was incorrectly blocked because J was included in this check
    if modifier_key == keys::X
        && syllable.vowel.len() == 1 
        && syllable.final_c.is_empty()
        && syllable.initial.len() == 1  // Must be single consonant
    {
        let vowel = buffer_keys[syllable.vowel[0]];
        let initial = buffer_keys[syllable.initial[0]];
        
        // VERY SPECIFIC: Only block F+I+X (fix) and H+E+X (hex)
        // These are common English words but rare Vietnamese syllables
        // Note: Do NOT block J modifier here - "hệ" (h+e+j), "fị" (f+i+j) are valid Vietnamese
        if (initial == keys::F && vowel == keys::I)  // fix
            || (initial == keys::H && vowel == keys::E)  // hex
        {
            return true;
        }
        
        // Note: Do NOT block other combinations:
        // - m+i+x → "mĩ" is valid (mì, mĩ)
        // - s+i+x → "sĩ" is valid (sĩ, sì)
        // - t+a+x → "tã" is valid (tã, tà)
        // - t+e+x → "tẽ" is valid (will detect "text" pattern later)
        // - s+e+x → "sẽ" is valid (will detect "sex" as English in full context)
    }

    // Check 5: English words like "text", "next", "sexy" - detect after tone mark applied
    // Pattern: buffer has tone mark on vowel (e.g., tẽ) and user types additional letter
    // This catches "text" where "tex" → "tẽ" transform happened, then 't' is typed
    // 
    // Strategy: Detect when buffer has transformed syllable + additional letter that makes it English
    // Common patterns:
    // - tẽ + t → text
    // - sẽ + x → sexy  
    // - nẽ + x → next
    // - rẽ + x → relax (partial)
    //
    // Note: We check this in handle_normal_letter when the additional letter is typed,
    // not here in try_mark. This check is for old logic compatibility.
    if syllable.final_c.len() == 1 {
        let final_key = buffer_keys[syllable.final_c[0]];
        
        // Check if final is 'x' and modifier is tone mark
        // This means X was already typed and accepted as final consonant,
        // now user is typing another tone modifier
        if final_key == keys::X && matches!(modifier_key, keys::S | keys::F | keys::R | keys::X | keys::J | keys::Z) {
            if syllable.vowel.len() == 1 && syllable.initial.len() == 1 {
                let vowel = buffer_keys[syllable.vowel[0]];
                let initial = buffer_keys[syllable.initial[0]];
                
                // Only block specific English patterns: t+e+x+t (text)
                if initial == keys::T && vowel == keys::E {
                    return true;
                }
            }
        }
    }

    // Check 6: Pattern with 't' final + tone modifier - ONLY for specific English words
    // When buffer is [consonant, e, t] and modifier is tone mark
    // MUST BE SPECIFIC: "tet" with various tones is valid Vietnamese (tết, tét, tẹt, tẻt, tẽt)
    if syllable.final_c.len() == 1 && syllable.vowel.len() == 1 && syllable.initial.len() >= 2 {
        let final_key = buffer_keys[syllable.final_c[0]];
        let vowel = buffer_keys[syllable.vowel[0]];
        
        // ONLY block when initial has 2+ consonants (invalid in Vietnamese)
        // Examples: test (t+e+s+t where "tes" parses as te+s), best, rest
        // Vietnamese "tet" patterns have single initial: t+e+t, d+e+t, etc.
        if final_key == keys::T 
            && vowel == keys::E 
            && matches!(modifier_key, keys::S | keys::F | keys::R | keys::X | keys::J)
        {
            // Multi-consonant initial = English
            return true;
        }
    }

    // Check 7: Words ending with 's' + tone modifier (class, pass, miss, boss, less)
    // ONLY when initial is invalid Vietnamese (multi-consonant cluster)
    if syllable.final_c.len() == 1 && syllable.initial.len() >= 2 {
        let final_key = buffer_keys[syllable.final_c[0]];
        
        // Multi-consonant initial + s final = English (class, press, etc.)
        if final_key == keys::S && matches!(modifier_key, keys::S | keys::F | keys::R | keys::X | keys::J) {
            return true;
        }
    }

    // Check 8: English words ending with -isk, -usk patterns (risk, disk, dusk, tusk)
    // Pattern: consonant + i/u + s + k modifier
    if modifier_key == keys::K 
        && syllable.final_c.len() == 1 
        && syllable.vowel.len() == 1
        && !syllable.initial.is_empty()
    {
        let final_key = buffer_keys[syllable.final_c[0]];
        let vowel = buffer_keys[syllable.vowel[0]];
        
        // -isk pattern (risk, disk, brisk)
        if final_key == keys::S && vowel == keys::I {
            return true;
        }
        // -usk pattern (dusk, tusk, musk, husk)
        if final_key == keys::S && vowel == keys::U {
            return true;
        }
    }

    // Check 9: Double vowel + K patterns (look, book, took, cook, hook)
    // Pattern: consonant + oo + k modifier
    if modifier_key == keys::K 
        && syllable.vowel.len() == 2
        && !syllable.initial.is_empty()
        && syllable.final_c.is_empty()
    {
        let vowel1 = buffer_keys[syllable.vowel[0]];
        let vowel2 = buffer_keys[syllable.vowel[1]];
        
        // oo + k pattern (common English, never Vietnamese)
        if vowel1 == keys::O && vowel2 == keys::O {
            return true;
        }
    }

    // Check 10: REMOVED - Too broad, blocks valid Vietnamese
    // Vietnamese has many valid C+E+tone patterns: te, de, le, me, ne, etc.
    // Only specific multi-word English patterns should be blocked, handled by other checks

    false
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::keys_from_str;

    /// Valid Vietnamese syllables
    const VALID: &[&str] = &[
        "ba", "ca", "an", "em", "gi", "gia", "giau", "ke", "ki", "ky", "nghe", "nghi", "nghieng",
        "truong", "nguoi", "duoc",
    ];

    /// Invalid: no vowel
    const INVALID_NO_VOWEL: &[&str] = &["bcd", "bcdfgh"];

    /// Invalid: bad initial
    const INVALID_INITIAL: &[&str] = &["clau", "john", "bla", "string", "chrome"];

    /// Invalid: spelling violations
    const INVALID_SPELLING: &[&str] = &["ci", "ce", "cy", "ka", "ko", "ku", "ngi", "nge", "ge"];

    /// Invalid: foreign words
    const INVALID_FOREIGN: &[&str] = &["exp", "expect", "test", "claudeco", "claus"];

    fn assert_all_valid(words: &[&str]) {
        for w in words {
            assert!(is_valid(&keys_from_str(w)), "'{}' should be valid", w);
        }
    }

    fn assert_all_invalid(words: &[&str]) {
        for w in words {
            assert!(!is_valid(&keys_from_str(w)), "'{}' should be invalid", w);
        }
    }

    #[test]
    fn test_valid() {
        assert_all_valid(VALID);
    }

    #[test]
    fn test_invalid_no_vowel() {
        assert_all_invalid(INVALID_NO_VOWEL);
    }

    #[test]
    fn test_invalid_initial() {
        assert_all_invalid(INVALID_INITIAL);
    }

    #[test]
    fn test_invalid_spelling() {
        assert_all_invalid(INVALID_SPELLING);
    }

    #[test]
    fn test_invalid_foreign() {
        assert_all_invalid(INVALID_FOREIGN);
    }

    // New tests for whitelist validation
    #[test]
    fn test_eu_invalid_without_circumflex() {
        // "eu" without circumflex should be invalid
        let keys = keys_from_str("neu");
        let tones = vec![0, 0, 0]; // no modifiers
        assert!(
            !is_valid_with_tones(&keys, &tones),
            "'neu' without circumflex should be invalid"
        );
    }

    #[test]
    fn test_eu_valid_with_circumflex() {
        // "êu" with circumflex should be valid
        let keys = keys_from_str("neu");
        let tones = vec![0, tone::CIRCUMFLEX, 0]; // circumflex on E
        assert!(
            is_valid_with_tones(&keys, &tones),
            "'nêu' with circumflex should be valid"
        );
    }

    #[test]
    fn test_valid_diphthongs() {
        // Test some valid diphthong patterns
        let valid_patterns = [
            "ai", "ao", "au", "eo", "ia", "iu", "oa", "oe", "oi", "ui", "uy",
        ];
        for pattern in valid_patterns {
            let keys = keys_from_str(pattern);
            assert!(is_valid(&keys), "'{}' should be valid diphthong", pattern);
        }
    }

    #[test]
    fn test_invalid_diphthongs() {
        // Test some invalid diphthong patterns (not in whitelist)
        let invalid_patterns = ["ou", "yo", "ae", "yi"];
        for pattern in invalid_patterns {
            let keys = keys_from_str(pattern);
            assert!(
                !is_valid(&keys),
                "'{}' should be invalid diphthong",
                pattern
            );
        }
    }

    #[test]
    fn test_breve_followed_by_vowel_invalid() {
        // Issue #44: "taiw" → "tăi" should be invalid
        // Breve (ă) cannot be followed by another vowel in Vietnamese
        // Valid: ăm, ăn, ăng (consonant endings), oă (xoăn)
        // Invalid: ăi, ăo, ău, ăy
        let keys = keys_from_str("tai");
        let tones = vec![0, tone::HORN, 0]; // breve on 'a'
        assert!(
            !is_valid_with_tones(&keys, &tones),
            "'tăi' (breve + vowel) should be invalid"
        );

        // Also test standalone ăi
        let keys = keys_from_str("ai");
        let tones = vec![tone::HORN, 0]; // breve on 'a'
        assert!(
            !is_valid_with_tones(&keys, &tones),
            "'ăi' should be invalid"
        );
    }
}
