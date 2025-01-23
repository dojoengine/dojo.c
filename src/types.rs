use std::collections::HashMap;
use std::ffi::c_char;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::atomic::AtomicU64;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use starknet::accounts::SingleOwnerAccount;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::JsonRpcClient;
use starknet::signers::LocalWallet;
use starknet_crypto::Felt;
use stream_cancel::Trigger;
use torii_client::client::Client;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub target: Felt,
    pub method: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterSessionResponse {
    pub username: String,
    pub address: String,
    pub expires_at: String,
    pub owner_guid: String,
    #[serde(default)]
    pub transaction_hash: Option<String>,
    #[serde(default)]
    pub already_registered: Option<bool>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SessionsStorage {
    pub active: String,
    pub sessions: HashMap<String, Vec<RegisteredSession>>,
    pub accounts: HashMap<String, RegisteredAccount>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisteredSession {
    pub public_key: Felt,
    pub expires_at: u64,
    pub policies: Vec<account_sdk::account::session::hash::Policy>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisteredAccount {
    pub username: String,
    pub address: Felt,
    pub owner_guid: Felt,
    pub chain_id: Felt,
    pub rpc_url: String
}

impl SessionsStorage {
    pub fn from_file(file: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(file)?;
        let reader = BufReader::new(file);
        let account_storage: SessionsStorage = serde_json::from_reader(reader)?;
        Ok(account_storage)
    }
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
pub struct SessionAccount(pub(crate) account_sdk::account::session::SessionAccount<Arc<JsonRpcClient<HttpTransport>>>);
#[wasm_bindgen]
pub struct Subscription {
    pub(crate) id: Arc<AtomicU64>,
    pub(crate) trigger: Trigger,
}
