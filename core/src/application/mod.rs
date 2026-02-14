//! Application Layer - Use Cases & Orchestration
//!
//! This layer:
//! - Implements business use cases
//! - Orchestrates domain objects
//! - Depends ONLY on domain layer
//!
//! ## SOLID Principle: Single Responsibility
//! Each use case has ONE responsibility - ONE business operation.

pub mod use_cases;
pub mod services;
pub mod dto;
