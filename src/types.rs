use std::ffi::c_char;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;

use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use starknet::accounts::SingleOwnerAccount;
use starknet::providers::JsonRpcClient;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::signers::LocalWallet;
use starknet_crypto::Felt;
use stream_cancel::Trigger;
use torii_client::client::Client;
use wasm_bindgen::prelude::*;
use starknet::core::serde::unsigned_field_element::UfeHex;

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct Policy {
    #[serde_as(as = "UfeHex")]
    pub target: Felt,
    pub method: String,
    pub description: String,
}

#[wasm_bindgen]
pub struct ToriiClient {
    #[wasm_bindgen(skip)]
    #[cfg(not(target_arch = "wasm32"))]
    pub inner: Client,
    #[wasm_bindgen(skip)]
    #[cfg(target_arch = "wasm32")]
    pub inner: Arc<Client>,
    #[cfg(not(target_arch = "wasm32"))]
    #[wasm_bindgen(skip)]
    pub runtime: tokio::runtime::Runtime,
    #[cfg(not(target_arch = "wasm32"))]
    #[wasm_bindgen(skip)]
    pub logger: Option<extern "C" fn(*const c_char)>,
}
#[wasm_bindgen]
pub struct Provider(pub(crate) Arc<JsonRpcClient<HttpTransport>>);
#[wasm_bindgen]
pub struct Account(pub(crate) SingleOwnerAccount<Arc<JsonRpcClient<HttpTransport>>, LocalWallet>);
#[wasm_bindgen]
pub struct Subscription {
    pub(crate) id: Arc<AtomicU64>,
    pub(crate) trigger: Trigger,
}
