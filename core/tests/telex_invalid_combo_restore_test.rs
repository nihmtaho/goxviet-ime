//! Regression tests for invalid Vietnamese Telex/VNI combination handling.
//!
//! Covers three sub-features:
//! - Sub-A: Horn/Breve skips glide vowels ("hoacwj" → "hoặc")
//! - Sub-B: Tone mark validates vowel cluster ("yas" → "yas", not "yá")
//! - Sub-C: NA-PAC compatibility check on coda extension ("hoawjch" → raw "hoawjch")

use goxviet_core::utils::telex;

// ── Sub-B: Tone mark + invalid vowel cluster ──────────────────────────────────

#[test]
fn test_yas_outputs_yas() {
    // "ya" is not a valid Vietnamese vowel nucleus → 's' should NOT apply sắc
    telex(&[("yas", "yas")]);
}

#[test]
fn test_hoas_still_works() {
    // "oa" is NA.1 → sắc applies normally → "hoá"
    telex(&[("hoas", "hoá")]);
}

#[test]
fn test_ans_still_works() {
    // "a" is NA.1 → sắc on 'a' → "án"
    telex(&[("ans", "án")]);
}

#[test]
fn test_ys_still_works() {
    // "y" alone is NA.0 → sắc applies → "ý"
    telex(&[("ys", "ý")]);
}

// ── Sub-A: Horn/Breve skips glide vowels ─────────────────────────────────────

#[test]
fn test_hoacwj_outputs_hoac_nang() {
    // "hoac" + w applies horn to 'a' (not glide 'o') → "hoặc" + j = nặng
    telex(&[("hoacwj", "hoặc")]);
}

#[test]
fn test_duocwj_still_works() {
    // "duoc" + w: 'o' is not a glide (no 'a'/'e' after 'o') → horn on 'o', normalize_uo_compound
    // also gives horn to 'u' → "dược" (d without stroke; 'dd' would be needed for đ)
    telex(&[("duocwj", "dược")]);
}

// ── Sub-C: NA-PAC coda extension validation ───────────────────────────────────

#[test]
fn test_hoawjch_restores_raw() {
    // "hoặc" (NA.3=oă, PAC.1=c) + 'h' would form "ch" (PAC.0) → invalid → restore raw
    telex(&[("hoawjch", "hoawjch")]);
}

#[test]
fn test_hoach_still_valid() {
    // "hoac" + 'h': "oa" is NA.1 which allows PAC.0 (ch) → "hoach" stays as typed
    // (no Vietnamese transforms applied before 'h', so no NA-PAC check triggers)
    telex(&[("hoach", "hoach")]);
}
