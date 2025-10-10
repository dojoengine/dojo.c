mod constants;
mod types;
mod utils;

// Diplomat FFI bridge modules
mod ffi;

// Re-export diplomat-generated FFI
pub use ffi::*;

// Keep old modules for reference during migration
#[cfg(not(target_arch = "wasm32"))]
mod c;
#[cfg(target_arch = "wasm32")]
/// cbindgen:ignore
mod wasm;
