//! Transformation Ports (Interfaces)
//!
//! Defines abstractions for text transformation following SOLID principles.

pub mod mark_transformer;
pub mod tone_transformer;

// Re-export main types
pub use mark_transformer::{MarkTransformer, MarkType};
pub use tone_transformer::{ToneStrategy, ToneTransformer};
