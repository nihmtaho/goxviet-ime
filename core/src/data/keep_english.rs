//! Short English words that Telex/VNI tone modifiers corrupt.
//! "as"→"á", "is"→"í", "of"→"ò", "has"→"hás" etc.
//! At SPACE boundary, check_and_restore_english_at_boundary restores these without phonotactic.

include!(concat!(env!("OUT_DIR"), "/keep_english.rs"));

/// Returns true if `word` is in the keep-English exception list.
pub fn is_keep_english(word: &str) -> bool {
    KEEP_ENGLISH.contains(&word.to_lowercase().as_str())
}
