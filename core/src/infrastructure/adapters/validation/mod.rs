//! Validator Adapters
//!
//! Implementations of validation traits + Vietnamese/English validation modules.

pub mod fsm_validator_adapter;
pub mod language_detector_adapter;
pub mod phonotactic_adapter;

// Validation modules (moved from infrastructure/external/)
pub mod english;
pub mod fsm;
pub mod vietnamese_validator;
pub mod diacritical_validator;

// Re-exports for convenience
pub use fsm_validator_adapter::FsmValidatorAdapter;
pub use language_detector_adapter::LanguageDetectorAdapter;
pub use phonotactic_adapter::PhonotacticAdapter;
