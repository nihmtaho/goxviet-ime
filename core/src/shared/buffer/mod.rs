//! Buffer management for Vietnamese IME
//!
//! Handles typing buffer, raw input tracking, and output generation.

pub mod buffer;
pub mod raw_input_buffer;
pub mod rebuild;

pub use buffer::{Buffer, Char, MAX};
pub use raw_input_buffer::RawInputBuffer;
