use goxviet_core::data::{
    chars::{mark, tone},
    keys,
};
use goxviet_core::engine::buffer::{Buffer, Char};
use goxviet_core::engine::vietnamese::transform;

fn setup_buffer(s: &str) -> Buffer {
    let mut buf = Buffer::new();
    for ch in s.chars() {
        let key = match ch.to_ascii_lowercase() {
            'a' => keys::A,
            'e' => keys::E,
            'i' => keys::I,
            'o' => keys::O,
            'u' => keys::U,
            'y' => keys::Y,
            'v' => keys::V,
            's' => keys::S,
            _ => continue,
        };
        buf.push(Char::new(key, ch.is_uppercase()));
    }
    buf
}

#[test]
fn test_missing_modified_positions_on_reposition() {
    // Setup buffer: "uí" (u, i with mark SAC)
    let mut buf = setup_buffer("ui");
    // Add mark to I (index 1) manually
    buf.get_mut(1).unwrap().mark = mark::SAC;

    // Verify initial state
    assert_eq!(buf.get(1).unwrap().mark, mark::SAC);
    assert_eq!(buf.get(0).unwrap().tone, tone::NONE);

    // Apply tone W (Horn) to U (index 0) -> creates Ư
    // "uí" + w -> "ưi"
    // Rule 1: Diacritic priority -> mark should move to Ư (index 0)
    // Old mark on I (index 1) should be removed.

    // transform::apply_tone(buf, key, tone_value, method)
    // key: W (for horn), tone: HORN, method: 0 (Telex)
    let result = transform::apply_tone(&mut buf, keys::W, tone::HORN, 0);

    assert!(result.applied, "Tone should be applied (u -> ư)");

    // Initial Verification: Mark should have moved
    assert_eq!(
        buf.get(1).unwrap().mark,
        mark::NONE,
        "Mark should be removed from I"
    );
    assert_eq!(buf.get(0).unwrap().mark, mark::SAC, "Mark should be on Ư");
    assert_eq!(buf.get(0).unwrap().tone, tone::HORN, "Ư should have horn");

    // The Issue: modified_positions should include ALL changed indices
    // 0 (U -> Ư, mark added)
    // 1 (I -> I, mark removed)
    // So indices 0 and 1 should be in modified_positions.

    let modified = result.modified_positions;
    println!("Modified positions: {:?}", modified);

    // If the bug exists, this assertion will fail because it likely only contains [0]
    assert!(
        modified.contains(&0),
        "Index 0 (target of mark move) must be in modified positions. Actual: {:?}",
        modified
    );
    assert!(
        modified.contains(&1),
        "Index 1 (source of mark move) must be in modified positions. Actual: {:?}",
        modified
    );
}
