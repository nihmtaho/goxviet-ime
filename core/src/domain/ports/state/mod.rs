//! State Management Ports (Interfaces)
//!
//! Defines abstractions for state management following SOLID principles.

pub mod buffer_manager;
pub mod history_tracker;

// Re-export main types
pub use buffer_manager::BufferManager;
pub use history_tracker::{HistoryEntry, HistoryTracker};
