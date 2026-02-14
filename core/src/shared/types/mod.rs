//! Core types and configuration for Vietnamese IME engine

pub mod config;

pub use config::EngineConfig;

// Re-export types from types.rs
pub use types::{Action, Result, Transform};

mod types;
