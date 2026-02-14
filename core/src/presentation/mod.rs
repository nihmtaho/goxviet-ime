//! Presentation Layer - FFI & Dependency Injection
//!
//! This layer:
//! - Exposes C FFI API
//! - Wires up dependencies (IoC container)
//! - Converts between C and Rust types
//!
//! ## SOLID Principle: Open/Closed
//! Add new features by registering new implementations in DI container,
//! without modifying existing code.

pub mod ffi;
pub mod di;
