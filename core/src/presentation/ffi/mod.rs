//! C Foreign Function Interface
//!
//! Exposes C-compatible API for platform integrations

pub mod api;
pub mod conversions;
pub mod types;

pub use conversions::*;
pub use types::*;
