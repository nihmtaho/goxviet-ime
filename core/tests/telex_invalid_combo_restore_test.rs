//! Regression tests for invalid Vietnamese Telex/VNI combination handling.
//!
//! Covers three sub-features:
//! - Sub-A: Horn/Breve skips glide vowels ("hoacwj" → "hoặc")
//! - Sub-B: Tone mark validates vowel cluster ("yas" → "yas", not "yá")
//! - Sub-C: NA-PAC compatibility check on coda extension ("hoawjch" → raw "hoawjch")
//! - Sub-D: NA-PAC unknown cluster regression (gi+ă, qu+ă, êô combos must NOT restore)

use goxviet_core::utils::{telex, vni};

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

// ── Sub-D: Unknown vowel cluster regression ───────────────────────────────────
// gi+ă, qu+ă, and multi-vowel sequences (êô) are not in the NA phonotactic model
// but are valid Vietnamese syllables — they must NOT trigger the coda-restore.

#[test]
fn test_giawng_not_restored() {
    // gi (initial) + ă (breve a) + ng: vowel cluster "iă" is unknown to NA model
    // → must allow through (not restore to raw)
    telex(&[("giawng", "giăng")]);
}

#[test]
fn test_giuwong_not_restored() {
    // gi (initial) + ươ + ng: vowel cluster is unknown to NA model → allow through
    telex(&[("giuwong", "giương")]);
}

#[test]
fn test_giawngf_tone_correct() {
    // gi + ă + ng + huyền: tone mark should land on ă (diacritic priority, Rule 1)
    telex(&[("giawngf", "giằng")]);
}

#[test]
fn test_quawng_not_restored() {
    // qu (initial) + ă + ng: vowel cluster "uă" unknown to NA model → allow through
    telex(&[("quawng", "quăng")]);
}

#[test]
fn test_neeoong_not_restored() {
    // nê + ô + ng: multi-circumflex vowel cluster "êô" unknown to NA model → allow through
    telex(&[("neeoong", "nêông")]);
}

#[test]
fn test_gia8ng_vni_not_restored() {
    // VNI: gi + ă (a8) + ng → giăng
    vni(&[("gia8ng", "giăng")]);
}

#[test]
fn test_qua8ng_vni_not_restored() {
    // VNI: qu + ă (a8) + ng → quăng
    vni(&[("qua8ng", "quăng")]);
}

#[test]
fn test_ne6o6ng_vni_not_restored() {
    // VNI: nê (e6) + ô (o6) + ng → nêông
    vni(&[("ne6o6ng", "nêông")]);
}
