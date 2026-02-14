//! Application Services
//!
//! Coordinate operations across domain boundaries.

pub mod config_service;
pub mod processor_service;

// Re-export
pub use config_service::ConfigService;
pub use processor_service::{ProcessorService, ProcessingOutput, ProcessorError};
