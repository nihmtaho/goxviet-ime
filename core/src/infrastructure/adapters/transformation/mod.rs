//! Transformer Adapters
//!
//! Implementations of transformation traits + Vietnamese transformation modules.

pub mod vietnamese_mark_adapter;
pub mod vietnamese_tone_adapter;

// Vietnamese transformation modules (moved from infrastructure/engine/transform/)
pub mod syllable;
pub mod tone_positioning;
pub mod transform;
pub mod validation;
pub mod vowel_compound;

// Re-exports for convenience
pub use vietnamese_mark_adapter::VietnameseMarkAdapter;
pub use vietnamese_tone_adapter::VietnameseToneAdapter;
pub use syllable::Syllable;
pub use transform::{ModifierType, TransformResult};
