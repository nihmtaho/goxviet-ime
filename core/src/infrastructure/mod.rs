//! Infrastructure Layer - Adapters & External Integrations
//!
//! This layer:
//! - Implements domain ports (adapters)
//! - Integrates with external systems
//! - Contains ALL concrete implementations
//!
//! ## SOLID Principle: Dependency Inversion
//! Infrastructure depends on domain abstractions, not vice versa.

pub mod adapters;
pub mod repositories;
pub mod external;
pub mod engine;
