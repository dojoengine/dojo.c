// UniFFI bindings for Dojo
//
// This module provides foreign function interface bindings for multiple languages
// (Swift, Kotlin, Python) using Mozilla's UniFFI framework.

// Type definitions organized by domain
pub mod types;

// Client implementation
pub mod client;

// Re-export everything for convenience
pub use client::*;
pub use types::*;
