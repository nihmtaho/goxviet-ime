//! Application Use Cases
//!
//! Command/Query implementations for business operations.

pub mod manage_shortcuts;
pub mod process_keystroke;
pub mod transform_text;
pub mod validate_input;

// Re-export
pub use manage_shortcuts::{ManageShortcutsUseCase, Shortcut, ShortcutResult};
pub use process_keystroke::ProcessKeystrokeUseCase;
pub use transform_text::{TransformRequest, TransformTextUseCase, TransformationType};
pub use validate_input::{ValidateInputUseCase, ValidationRequest};
