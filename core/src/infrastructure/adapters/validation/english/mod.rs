//! English language detection — phonotactic analysis and Vietnamese-first language decision.
//!
//! The English dictionary (dictionary.rs / dictionary_data.rs) was removed in Sprint C.
//! Language detection now uses the Vietnamese TuDien dictionary (Priority 1) and the
//! PhonotacticEngine (Priority 2). See `language_decision.rs`.

pub mod language_decision;
pub mod phonotactic;
