//! External Integrations — re-exported from adapters/validation/
//!
//! Original implementations moved to `crate::infrastructure::adapters::validation`.

pub mod updater;

// Re-exports pointing to new locations
pub use crate::infrastructure::adapters::validation::english;
pub use crate::infrastructure::adapters::validation::fsm;
pub use crate::infrastructure::adapters::validation::vietnamese_validator;
pub use crate::infrastructure::adapters::validation::diacritical_validator;

pub use updater::UpdaterAdapter;
