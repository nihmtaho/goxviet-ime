//! Core types — re-exported from shared/types/
//!
//! Original implementation moved to `crate::shared::types`.

pub use crate::shared::types::config;

pub use crate::shared::types::EngineConfig;
pub use crate::shared::types::{Action, Result, Transform};

// Keep backward compat for `types` sub-module access
pub mod types {
    pub use crate::shared::types::*;
}
