//! Regression tests for invalid Vietnamese Telex/VNI combination handling.
//!
//! Covers three sub-features:
//! - Sub-A: Horn/Breve skips glide vowels ("hoacwj" → "hoặc")
//! - Sub-B: Tone mark validates vowel cluster ("yas" → "yas", not "yá")
//! - Sub-C: NA-PAC compatibility check on coda extension ("hoawjch" → raw "hoawjch")
//! - Sub-D: NA-PAC unknown cluster regression (gi+ă, qu+ă, êô combos must NOT restore)
//! - Sub-E: NA.5 diphthong + coda → immediate restore ("voiwsc" → "voiwsc", "voicws" → "voicws")

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

// ── Sub-E: NA.5 diphthong + coda → immediate restore ─────────────────────────
// NA.5 vowel clusters (oi, ơi, ai, ay, etc.) allow NO final consonants.
// When a coda is added to a diphthong that has Vietnamese transforms,
// the engine must immediately restore to raw (not wait for SPACE).

#[test]
fn test_voiwsc_restores_raw() {
    // v + oi + w(horn on o → ơi) + s(sắc) + c(coda on NA.5 → invalid)
    // "ơi" is NA.5 (open only) — adding 'c' must restore immediately
    telex(&[("voiwsc", "voiwsc")]);
}

#[test]
fn test_voicws_restores_raw() {
    // v + oi + c(added first, no transform yet) + w(horn on o → ơi, NA.5 + existing coda → invalid)
    // After horn, vowel cluster becomes "ơi" (NA.5) while coda 'c' already exists → restore
    telex(&[("voicws", "voicws")]);
}

#[test]
fn test_voif_no_coda_stays_viet() {
    // v + oi + f(huyền) → "vòi" — NA.5 open syllable with tone is valid
    telex(&[("voif", "vòi")]);
}

#[test]
fn test_oic_restores_raw() {
    // oi (NA.5) + c → no transforms, but to verify boundary check doesn't break things
    // Without Vietnamese transforms, check_first_coda_validity doesn't fire
    // The boundary check (at SPACE) may or may not restore — just ensure no crash
    // (No assertion on exact output — just ensure it doesn't panic)
    telex(&[("oic", "oic")]);
}

#[test]
fn test_uowc_valid_uo_compound() {
    // u + o + w → "ươ" compound (NA.2, allows PAC.1) + c → "ươc" is valid Vietnamese
    // The engine normalizes "uow" to "ươ" (NA.2), NOT "uơ" (NA.4 open-only)
    telex(&[("uowc", "ươc")]);
}

// ── Sub-F: User reported failure cases from failures_english_telex.txt ────────

// Fix 2 (vowel cluster check): horn/marked vowel followed by incompatible vowel
#[test]
fn test_vowel_stays_raw() {
    // "vowel": v + o + w(horn → ơ) + e → "ơe" is NOT in any NA group → restore immediately
    // Result: "voe" at that point, then 'l' appended → "vowel" (or restore to full raw)
    telex(&[("vowel", "vowel")]);
}

#[test]
fn test_vison_stays_raw() {
    // "vison": v + i + s(sắc on i → í) + o → "io" is NOT in any NA group → restore immediately
    telex(&[("vison", "vison")]);
}

// Fix 1 (coda extension check): invalid 2-char coda with pp / dd / etc.
#[test]
fn test_tropophyte_stays_raw() {
    // "tropophyte": trô (oo→ô) + p + p → "pp" is NOT a Vietnamese digraph coda → restore
    telex(&[("tropophyte", "tropophyte")]);
}

// Fix 3 (dd-stroke guard): 'dd' in English context without prior transforms
#[test]
fn test_trodden_stays_raw() {
    // "trodden": t + r + o + d + d → no Vietnamese transforms when second 'd' typed → no đ stroke
    telex(&[("trodden", "trodden")]);
}

// ── Sub-G: ươu triphthong (ơ + u whitelist) ──────────────────────────────────

#[test]
fn test_ruowuj_ruou_nang() {
    // r + ươ (uow) + u + j(nặng) → rượu
    telex(&[("ruowuj", "rượu")]);
}

#[test]
fn test_huowu_ruou_open() {
    // h + ươ (uow) + u → hươu
    telex(&[("huowu", "hươu")]);
}

#[test]
fn test_ruo7u5_vni_ruou_nang() {
    // VNI: r + u + o + 7(horn) + u + 5(nặng) → rượu
    vni(&[("ruo7u5", "rượu")]);
}

#[test]
fn test_huo7u_vni_ruou_open() {
    // VNI: h + u + o + 7(horn) + u → hươu
    vni(&[("huo7u", "hươu")]);
}
