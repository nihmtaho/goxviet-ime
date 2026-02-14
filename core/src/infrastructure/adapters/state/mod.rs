//! State Management Adapters
//!
//! Implementations of state management traits

pub mod memory_buffer_adapter;
pub mod simple_history_adapter;

// Re-export main types
pub use memory_buffer_adapter::MemoryBufferAdapter;
pub use simple_history_adapter::SimpleHistoryAdapter;
