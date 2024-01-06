mod constants;
mod types;
mod utils;

#[cfg(not(target_arch = "wasm32"))]
mod libc;
#[cfg(target_arch = "wasm32")]
/// cbindgen:ignore
mod libwasm;
