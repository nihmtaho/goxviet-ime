//! Input Method Adapters
//!
//! Concrete implementations of the `InputMethod` port.

pub mod telex_adapter;
pub mod vni_adapter;

// Re-export
pub use telex_adapter::TelexAdapter;
pub use vni_adapter::VniAdapter;

