// UniFFI bindings for Dojo
// Multi-language bindings (Swift, Kotlin, Python) using UniFFI

pub mod uniffi;

// Re-export all UniFFI types at crate root for scaffolding
pub use uniffi::*;

// Include the generated UniFFI scaffolding
::uniffi::include_scaffolding!("dojo");
