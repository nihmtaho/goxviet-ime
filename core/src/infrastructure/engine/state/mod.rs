//! State management for Vietnamese IME
//!
//! Handles word history and raw input restoration.

pub mod history;
pub mod restore;

pub use history::WordHistory;
// restore module provides functions for raw input restoration
