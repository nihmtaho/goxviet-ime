//! Domain Layer - Core Business Logic
//!
//! This layer contains:
//! - **Entities**: Core business objects with identity
//! - **Value Objects**: Immutable data structures
//! - **Ports**: Interfaces/traits that define contracts
//!
//! ## SOLID Principle: Dependency Rule
//! Domain layer has ZERO dependencies on outer layers.
//! It defines interfaces that outer layers must implement.

pub mod entities;
pub mod value_objects;
pub mod ports;
