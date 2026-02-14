//! Repositories
//!
//! Data access layer for dictionaries, shortcuts, etc.

pub mod dictionary_repo;
pub mod shortcut_repo;

pub use dictionary_repo::DictionaryRepo;
pub use shortcut_repo::ShortcutRepo;
