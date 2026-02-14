//! Engine Buffer - Domain Entities
//!
//! Re-exports buffer types from the legacy engine.
//! These are core domain entities (Buffer, Char, RawInputBuffer) used throughout
//! the Vietnamese input processing pipeline.
//!
//! # SOLID Layer: Domain / Entities
//!
//! Buffer and Char represent the fundamental data structures for managing
//! user input in the IME engine.

pub use crate::infrastructure::engine::buffer::{Buffer, Char, RawInputBuffer, MAX};
