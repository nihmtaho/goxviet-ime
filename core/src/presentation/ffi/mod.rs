//! C Foreign Function Interface
//!
//! Exposes C-compatible API for platform integrations

pub mod types;
pub mod conversions;
pub mod api;

pub use types::*;
pub use conversions::*;
