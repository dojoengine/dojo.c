mod constants;
mod utils;
mod global_types;

#[cfg(not(target_arch = "wasm32"))]
mod c;
#[cfg(target_arch = "wasm32")]
/// cbindgen:ignore
mod wasm;

// UniFFI support - multi-language bindings
#[cfg(not(target_arch = "wasm32"))]
pub mod uniffi;

// Re-export all UniFFI types at crate root for scaffolding
#[cfg(not(target_arch = "wasm32"))]
pub use uniffi::*;

// Include the generated UniFFI scaffolding
#[cfg(not(target_arch = "wasm32"))]
include!(concat!(env!("OUT_DIR"), "/dojo.uniffi.rs"));
