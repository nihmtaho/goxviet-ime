//! Unified Vietnamese IME Engine
//!
//! SOLID-compliant facade for the Vietnamese input method engine.
//! This module re-exports and organizes the engine components following SOLID principles.

// Re-export core engine implementation
pub use crate::infrastructure::engine::{Action, Engine, EngineConfig, Result, Transform, WordHistory};

// Re-export InputMethod from features::shortcut
pub use crate::infrastructure::engine::features::shortcut::InputMethod;

// Re-export buffer components
pub use crate::infrastructure::engine::buffer::MAX as BUFFER_MAX;
pub use crate::infrastructure::engine::buffer::{Buffer, Char, RawInputBuffer};

// Re-export features
pub use crate::infrastructure::engine::features::{
    encoding::{EncodingConverter, OutputEncoding},
    shortcut::{Shortcut, ShortcutTable},
};

// Re-export validation from external infrastructure
pub use crate::infrastructure::external::diacritical_validator::DiacriticalValidator;
pub use crate::infrastructure::external::vietnamese_validator::{
    ValidationResult as FsmValidationResult,
    VietnameseSyllableValidator as FsmSyllableValidator,
};

// Re-export english detection (uses infrastructure::external::english)
pub use crate::infrastructure::external::english::{
    dictionary::Dictionary,
    language_decision::{DecisionResult, LanguageDecisionEngine},
    phonotactic::{AutoRestoreDecider, PhonotacticEngine, PhonotacticResult},
};

// Re-export FSM tables
pub use crate::infrastructure::external::fsm::tables;

/// Initialize the unified engine
pub fn init() -> Engine {
    Engine::new()
}

/// Get default engine configuration
pub fn default_config() -> EngineConfig {
    EngineConfig::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_engine_creation() {
        let _engine = init();
    }
}
