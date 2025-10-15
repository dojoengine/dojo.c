// UniFFI module - exposes Dojo types to multiple languages
// This module provides a cleaner, idiomatic API compared to C FFI

// Core types and utilities
pub mod core;

// Domain-specific type modules
pub mod achievement;
pub mod activity;
pub mod aggregation;
pub mod client;
pub mod contract;
pub mod controller;
pub mod entity;
pub mod event;
pub mod query;
pub mod schema;
pub mod token;
pub mod transaction;

// Re-export all public types for convenience
pub use achievement::*;
pub use activity::*;
pub use aggregation::*;
pub use client::*;
pub use contract::*;
pub use controller::*;
pub use core::*;
pub use entity::*;
pub use event::*;
pub use query::*;
pub use schema::*;
pub use token::*;
pub use transaction::*;
