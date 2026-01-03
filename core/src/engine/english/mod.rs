//! English detection and processing
//!
//! Multi-layer English word detection using phonotactic patterns.

pub mod phonotactic;
pub mod english_detection;

pub use phonotactic::PhonotacticEngine;
