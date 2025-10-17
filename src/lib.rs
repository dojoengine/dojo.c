// Re-export the appropriate crate based on compilation target

#[cfg(not(target_arch = "wasm32"))]
pub use c::*;

#[cfg(target_arch = "wasm32")]
pub use wasm::*;
