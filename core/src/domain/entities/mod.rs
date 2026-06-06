//! Domain Entities
//!
//! Entities are objects with identity that represent core business concepts.
//! They contain business logic and maintain invariants.

pub mod buffer;
pub mod input_method_config;
pub mod key_event;
pub mod syllable;
pub mod tone;

pub use input_method_config::{InputAction, InputMethodConfig};
