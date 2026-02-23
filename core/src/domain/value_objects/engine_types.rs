//! Engine Types - Domain Value Objects
//!
//! Re-exports core types from the engine.
//! These are value objects representing actions, results, and transformations.
//!
//! # SOLID Layer: Domain / Value Objects

pub use crate::infrastructure::engine::core_types::{Action, Result, Transform};
pub use crate::infrastructure::engine::core_types::config::EngineConfig as LegacyEngineConfig;
