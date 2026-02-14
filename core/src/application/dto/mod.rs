//! Data Transfer Objects (DTOs)
//!
//! Plain data structures for transferring information between layers.

pub mod engine_config;
pub mod processing_context;

// Re-export main types
pub use engine_config::EngineConfig;
pub use processing_context::ProcessingContext;
