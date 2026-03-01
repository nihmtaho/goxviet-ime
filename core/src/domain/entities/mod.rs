//! Domain Entities
//!
//! Entities are objects with identity that represent core business concepts.
//! They contain business logic and maintain invariants.

pub mod tone;
pub mod key_event;
pub mod buffer;
pub mod syllable;
pub mod engine_buffer;
pub mod input_method_config;

pub use input_method_config::{InputAction, InputMethodConfig};
