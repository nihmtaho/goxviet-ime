//! Vietnamese IME Core - Clean Architecture (v3.0.0)
//!
//! High-performance Vietnamese input method engine following SOLID principles.
//!
//! # Architecture
//!
//! This library implements Clean Architecture with clear separation:
//!
//! - **domain/**: Core business logic (entities, value objects, ports)
//! - **application/**: Use cases and services
//! - **infrastructure/**: Adapters (Telex, VNI, validators)
//! - **presentation/**: FFI API and dependency injection
//!
//! # FFI API v2 Usage
//!
//! ```c
//! // Initialize engine with config
//! void* engine = ime_create_engine_v2(NULL);  // NULL for default config
//!
//! // Process keystrokes
//! FfiProcessResult_v2 result;
//! FfiStatusCode status = ime_process_key_v2(engine, 'a', &result);
//! if (status == FFI_STATUS_OK && result.consumed) {
//!     // Apply text changes
//!     apply_backspaces(result.backspace_count);
//!     insert_text(result.text);
//! }
//! ime_free_string_v2(result.text);
//!
//! // Cleanup
//! ime_destroy_engine_v2(engine);
//! ```
//!
//! See `presentation/ffi/api.rs` for full API documentation.

// ============================================================
// CLEAN ARCHITECTURE (v3.0.0)
// ============================================================
pub mod domain;          // Core business logic (entities, ports, value objects)
pub mod application;     // Use cases & services (orchestration)
pub mod infrastructure;  // Adapters & implementations (Telex, VNI, validators)
pub mod presentation;    // FFI API & DI container
pub mod shared;          // Shared utilities (buffer, types)
pub mod features;        // Feature modules (shortcuts, encoding)
pub mod unified_engine;  // SOLID facade for engine components

// ============================================================
// UTILITIES & DATA
// ============================================================
pub mod data;            // Constants, keys, dictionaries, FSM tables
pub mod utils;           // Helper functions

// ============================================================
// LEGACY CODE REMOVED (v3.1.0)
// ============================================================
// engine/ and engine_v2/ have been fully migrated:
//   - engine/     → infrastructure/engine/  (core Vietnamese processing)
//   - engine_v2/  → infrastructure/external/        (validation, FSM, English detection)

// Backward-compatible re-exports for integration tests and external consumers
pub mod engine {
    //! Re-export of infrastructure::engine for backward compatibility
    pub use crate::infrastructure::engine::*;
}
pub mod engine_v2 {
    //! Re-export of infrastructure::external for backward compatibility
    pub use crate::infrastructure::external::*;
}

// Other utilities
pub mod input;           // Input utilities
pub mod updater;         // Auto-update functionality

// ============================================================
// OBSOLETE MODULES DELETED (v3.0.0)
// ============================================================
// The following modules have been removed as they were unused/obsolete:
// - processors/  (44KB) - Old processor implementation
// - state/       (24KB) - Old state management
// - traits/      (28KB) - Old trait definitions  
// - types.rs     (310 lines) - Obsolete type definitions
// - transformers/ (empty)
// - validators/   (empty)
// - features/     (empty)
// - ffi/          (empty)

// Re-export main types from presentation layer
pub use presentation::ffi::types::{
    FfiConfig_v2,
    FfiProcessResult_v2,
    FfiStatusCode,
    FfiVersionInfo,
};

// Re-export v2 FFI API
pub use presentation::ffi::api::{
    ime_create_engine_v2,
    ime_destroy_engine_v2,
    ime_process_key_v2,
    ime_get_config_v2,
    ime_set_config_v2,
    ime_get_version_v2,
    ime_free_string_v2,
    // Shortcut API
    ime_add_shortcut_v2,
    ime_remove_shortcut_v2,
    ime_clear_shortcuts_v2,
    ime_shortcuts_count_v2,
    ime_set_shortcuts_enabled_v2,
};

// ============================================================
// v1 FFI API REMOVED (v3.0.0)
// ============================================================
//
// Legacy v1 FFI functions have been removed in v3.0.0.
//
// Migration:
//   v1: ime_init() / ime_key() / ime_free()
//   v2: ime_create_engine_v2() / ime_process_key_v2() / ime_destroy_engine_v2()
//
// Key differences:
//   - v2 uses out parameters to avoid Swift ABI issues
//   - v2 returns explicit status codes
//   - v2 supports per-engine config (no global state)
//
// See MIGRATION_GUIDE.md for detailed migration instructions.
//
// Removed functions:
//   - ime_init(), ime_key(), ime_key_ext()
//   - ime_method(), ime_enabled(), ime_skip_w_shortcut()
//   - ime_esc_restore(), ime_free_tone(), ime_modern()
//   - ime_instant_restore(), ime_get_buffer()
//   - ime_clear(), ime_clear_all(), ime_free()
//   - ime_add_shortcut(), ime_remove_shortcut(), ime_clear_shortcuts()
//   - ime_shortcuts_count(), ime_shortcuts_capacity()
//   - ime_shortcuts_is_at_capacity(), ime_export_shortcuts_json()
//   - ime_import_shortcuts_json(), ime_free_string()
//   - ime_set_shortcuts_enabled(), ime_set_encoding()
//   - ime_get_encoding(), ime_convert_encoding()
//   - ime_free_bytes(), ime_restore_word()
//
