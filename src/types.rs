use std::collections::HashMap;
use std::ffi::c_char;
use std::fs::File;
use std::io::{BufReader, BufWriter};
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

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterSessionResponse {
    pub username: String,
    pub address: Felt,
    #[serde(deserialize_with = "deserialize_expires_at", serialize_with = "serialize_expires_at")]
    pub expires_at: u64,
    pub owner_guid: Felt,
    pub _transaction_hash: Option<Felt>,
    pub _already_registered: Option<bool>,
}

fn deserialize_expires_at<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let expires_at_str = String::deserialize(deserializer)?;
    expires_at_str.parse::<u64>().map_err(serde::de::Error::custom)
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
    pub policies: Vec<account_sdk::account::session::policy::Policy>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisteredAccount {
    pub username: String,
    pub address: Felt,
    pub owner_guid: Felt,
    pub chain_id: Felt,
    pub rpc_url: String,
}

impl SessionsStorage {
    pub fn from_file(file: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(file)?;
        let reader = BufReader::new(file);
        let account_storage: SessionsStorage = serde_json::from_reader(reader)?;
        Ok(account_storage)
    }

    pub fn write_to_file(&self, file: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::create(file)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self)?;
        Ok(())
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
pub struct ControllerAccount {
    pub(crate) account: account_sdk::account::session::account::SessionAccount,
    pub(crate) username: String,
}
#[wasm_bindgen]
pub struct Subscription {
    pub(crate) id: Arc<AtomicU64>,
    pub(crate) trigger: Trigger,
}
