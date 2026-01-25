//! Feature modules for Vietnamese IME
//!
//! User-defined shortcuts and abbreviations.
//! Multi-encoding output support.

pub mod encoding;
pub mod shortcut;

pub use encoding::{EncodingConverter, OutputEncoding};
pub use shortcut::Shortcut;
