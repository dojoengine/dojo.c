use std::ffi::c_char;
use std::sync::atomic::AtomicU64;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use starknet::accounts::SingleOwnerAccount;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::JsonRpcClient;
use starknet::signers::LocalWallet;
use stream_cancel::Trigger;
use torii_client::client::Client;
use wasm_bindgen::prelude::*;

#[derive(Debug, Serialize)]
pub struct Policy {
    pub target: String,
    pub method: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterSessionResponse {
    pub username: String,
    pub address: String,
    pub expires_at: String,
    pub owner_guid: String,
    pub transaction_hash: Option<String>,
    pub already_registered: Option<bool>,
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
