//! English language detection — phonotactic analysis and Vietnamese-first language decision.
//!
//! Language detection pipeline (see `language_decision.rs`):
//! - Priority 1 — Vietnamese TuDien dictionary (Vietnamese-first policy)
//! - Priority 2 — English dictionary (18k words via `EnglishDictAdapter`)
//! - Priority 3 — Vietnamese structure validator
//! - Priority 4 — Phonotactic analysis
//! - Priority 5 — Diacritics penalty

pub mod english_dict_adapter;
pub mod language_decision;
pub mod phonotactic;
pub use english_dict_adapter::EnglishDictAdapter;
