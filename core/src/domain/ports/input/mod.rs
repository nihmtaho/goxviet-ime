//! Input method port (interfaces)
//!
//! Defines abstractions for Vietnamese input methods following the Strategy pattern.

pub mod input_method;

// Re-export main types
pub use input_method::{DiacriticType, InputMethod, InputMethodId};
