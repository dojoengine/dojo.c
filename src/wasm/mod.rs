/// WASM bindings for Dojo client functionality
///
/// # Description
/// Provides interfaces for Starknet operations, cryptographic functions,
/// and Torii client interactions
mod utils;

use std::collections::HashMap;
use std::str::FromStr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use cainome::cairo_serde::{self, ByteArray, CairoSerde};
use crypto_bigint::U256;
use dojo_world::contracts::naming::compute_selector_from_tag;
use futures::{FutureExt, StreamExt};
use js_sys::Array;
use serde::{Deserialize, Serialize};
use serde_json::json;
use starknet::accounts::{
    Account as _, ConnectedAccount as _, ExecutionEncoding, SingleOwnerAccount,
};
use starknet::core::types::{Felt, FunctionCall};
use starknet::core::utils::get_contract_address;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::{JsonRpcClient, Provider as _};
use starknet::signers::{LocalWallet, SigningKey, VerifyingKey};
use starknet_crypto::poseidon_hash_many;
use stream_cancel::{StreamExt as _, Tripwire};
use torii_relay::types::Message;
use torii_typed_data::TypedData;
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

use crate::constants;
use crate::types::{Account, Provider, Subscription, ToriiClient};
use crate::utils::watch_tx;

mod types;

use types::{
    BlockId, Call, Calls, ClientConfig, Controller, Controllers, Entities, Entity, IndexerUpdate,
    KeysClause, KeysClauses, Model, Query, Signature, Token, TokenBalance, TokenBalances, Tokens,
    WasmU256,
};

const JSON_COMPAT_SERIALIZER: serde_wasm_bindgen::Serializer =
    serde_wasm_bindgen::Serializer::json_compatible();

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

/// Encodes typed data according to Starknet's typed data specification
///
/// # Parameters
/// * `typed_data` - JSON string containing the typed data
/// * `address` - Address as hex string
///
/// # Returns
/// Result containing encoded data as hex string or error
#[wasm_bindgen(js_name = typedDataEncode)]
pub fn typed_data_encode(typed_data: &str, address: &str) -> Result<String, JsValue> {
    let typed_data = serde_json::from_str::<TypedData>(&typed_data)
        .map_err(|err| JsValue::from(format!("failed to parse typed data: {err}")))?;

    let address = Felt::from_str(&address)
        .map_err(|err| JsValue::from(format!("failed to parse address: {err}")))?;

    typed_data
        .encode(address)
        .map(|felt| format!("{:#x}", felt))
        .map_err(|err| JsValue::from(err.to_string()))
}

/// Generates a new random signing key
///
/// # Returns
/// Private key as hex string
#[wasm_bindgen(js_name = signingKeyNew)]
pub fn signing_key_new() -> String {
    let private_key: SigningKey = SigningKey::from_random();

    format!("{:#x}", private_key.secret_scalar())
}

/// Signs a message hash with a private key
///
/// # Parameters
/// * `private_key` - Private key as hex string
/// * `hash` - Message hash as hex string
///
/// # Returns
/// Result containing signature or error
#[wasm_bindgen(js_name = signingKeySign)]
pub fn signing_key_sign(private_key: &str, hash: &str) -> Result<Signature, JsValue> {
    let private_key = Felt::from_str(private_key);
    if let Err(e) = private_key {
        return Err(JsValue::from(format!("failed to parse private key: {e}")));
    }

    let hash = Felt::from_str(hash);
    if let Err(e) = hash {
        return Err(JsValue::from(format!("failed to parse hash: {e}")));
    }

    let private_key = SigningKey::from_secret_scalar(private_key.unwrap());
    let sig = private_key.sign(&hash.unwrap());

    match sig {
        Ok(sig) => Result::Ok(Signature::from(&sig)),
        Err(e) => Err(JsValue::from(format!("failed to sign: {e}"))),
    }
}

/// Derives a verifying (public) key from a signing (private) key
///
/// # Parameters
/// * `signing_key` - Signing key as hex string
///
/// # Returns
/// Result containing verifying key as hex string or error
#[wasm_bindgen(js_name = verifyingKeyNew)]
pub fn verifying_key_new(signing_key: &str) -> Result<String, JsValue> {
    let signing_key = Felt::from_str(signing_key);
    if let Err(e) = signing_key {
        return Err(JsValue::from(format!("failed to parse signing key: {e}")));
    }

    let verifying_key = starknet_crypto::get_public_key(&signing_key.unwrap());

    Ok(format!("{:#x}", verifying_key))
}

/// Verifies a signature against a message hash using a verifying key
///
/// # Parameters
/// * `verifying_key` - Verifying key as hex string
/// * `hash` - Message hash as hex string
/// * `signature` - Signature to verify
///
/// # Returns
/// Result containing verification success boolean or error
#[wasm_bindgen(js_name = verifyingKeyVerify)]
pub fn verifying_key_verify(
    verifying_key: &str,
    hash: &str,
    signature: Signature,
) -> Result<bool, JsValue> {
    let verifying_key = Felt::from_str(verifying_key);
    if let Err(e) = verifying_key {
        return Err(JsValue::from(format!("failed to parse verifying key: {e}")));
    }

    let verifying_key = VerifyingKey::from_scalar(verifying_key.unwrap());

    let hash = Felt::from_str(hash);
    if let Err(e) = hash {
        return Err(JsValue::from(format!("failed to parse hash: {e}")));
    }

    let hash = &hash.unwrap();

    let signature = &starknet::core::crypto::Signature::from(&signature);

    match verifying_key.verify(hash, signature) {
        Ok(result) => Result::Ok(result),
        Err(e) => Err(JsValue::from(format!("failed to verify: {e}"))),
    }
}

/// Creates a new Starknet provider instance for a given RPC URL
///
/// # Parameters
/// * `rpc_url` - URL of the RPC endpoint
///
/// # Returns
/// Result containing Provider instance or error
#[wasm_bindgen(js_name = createProvider)]
pub unsafe fn create_provider(rpc_url: &str) -> Result<Provider, JsValue> {
    let rpc_url = url::Url::parse(rpc_url);
    if let Err(e) = rpc_url {
        return Err(JsValue::from(format!("failed to parse rpc url: {e}")));
    }
    let rpc_url = rpc_url.unwrap();

    let rpc = JsonRpcClient::new(HttpTransport::new(rpc_url));

    Result::Ok(Provider(Arc::new(rpc)))
}

#[wasm_bindgen]
impl Provider {
    /// Creates a new account instance with the given private key and address
    ///
    /// # Parameters
    /// * `private_key` - Private key as hex string
    /// * `address` - Account address as hex string
    ///
    /// # Returns
    /// Result containing Account instance or error
    #[wasm_bindgen(js_name = createAccount)]
    pub async unsafe fn create_account(
        &self,
        private_key: &str,
        address: &str,
    ) -> Result<Account, JsValue> {
        let private_key = Felt::from_str(private_key);
        if let Err(e) = private_key {
            return Err(JsValue::from(format!("failed to parse private key: {e}")));
        }

        let private_key = private_key.unwrap();

        let address = Felt::from_str(address);
        if let Err(e) = address {
            return Err(JsValue::from(format!("failed to parse address: {e}")));
        }

        let address = address.unwrap();

        let chain_id = self.0.chain_id().await;
        if let Err(e) = chain_id {
            return Err(JsValue::from(format!("failed to get chain id: {e}")));
        }

        let chain_id = chain_id.unwrap();

        let signer = LocalWallet::from_signing_key(SigningKey::from_secret_scalar(private_key));
        let account = SingleOwnerAccount::new(
            self.0.clone(),
            signer,
            address,
            chain_id,
            ExecutionEncoding::New,
        );

        Result::Ok(Account(account))
    }

    /// Calls a Starknet contract view function
    ///
    /// # Parameters
    /// * `call` - Call parameters including contract address and function
    /// * `block_id` - Block identifier for the call
    ///
    /// # Returns
    /// Result containing array of field elements or error
    #[wasm_bindgen(js_name = call)]
    pub async unsafe fn call(&self, call: Call, block_id: BlockId) -> Result<Array, JsValue> {
        let result = self
            .0
            .call::<FunctionCall, starknet::core::types::BlockId>(
                (&call).into(),
                (&block_id).into(),
            )
            .await;

        match result {
            Ok(res) => Ok(res.iter().map(|f| JsValue::from(format!("{:#x}", f))).collect()),
            Err(e) => Err(JsValue::from_str(&e.to_string())),
        }
    }

    /// Waits for a transaction to be confirmed
    ///
    /// # Parameters
    /// * `txn_hash` - Transaction hash as hex string
    ///
    /// # Returns
    /// Result containing success boolean or error
    #[wasm_bindgen(js_name = waitForTransaction)]
    pub async unsafe fn wait_for_transaction(&self, txn_hash: &str) -> Result<bool, JsValue> {
        let txn_hash = Felt::from_str(txn_hash)
            .map_err(|err| JsValue::from(format!("failed to parse transaction hash: {err}")))?;
        let result: Result<(), anyhow::Error> = watch_tx(&self.0, txn_hash).await;

        match result {
            Ok(_) => Result::Ok(true),
            Err(e) => Err(JsValue::from_str(&e.to_string())),
        }
    }
}

#[wasm_bindgen]
impl Account {
    /// Returns the account's address
    ///
    /// # Returns
    /// Result containing address as hex string or error
    #[wasm_bindgen(js_name = address)]
    pub unsafe fn address(&self) -> Result<String, JsValue> {
        let address = self.0.address();
        Ok(format!("{:#x}", address))
    }

    /// Returns the account's chain ID
    ///
    /// # Returns
    /// Result containing chain ID as hex string or error
    #[wasm_bindgen(js_name = chainId)]
    pub unsafe fn chain_id(&self) -> Result<String, JsValue> {
        let chain_id = self.0.chain_id();
        Ok(format!("{:#x}", chain_id))
    }

    /// Sets the block ID for subsequent operations
    ///
    /// # Parameters
    /// * `block_id` - Block ID as hex string
    ///
    /// # Returns
    /// Result containing unit or error
    #[wasm_bindgen(js_name = setBlockId)]
    pub unsafe fn set_block_id(&mut self, block_id: String) -> Result<(), JsValue> {
        let block_id = Felt::from_str(&block_id)
            .map_err(|err| JsValue::from(format!("failed to parse block id: {err}")))?;
        self.0.set_block_id(starknet::core::types::BlockId::Hash(block_id));
        Ok(())
    }

    /// Executes a raw transaction
    ///
    /// # Parameters
    /// * `calldata` - Array of contract calls to execute
    ///
    /// # Returns
    /// Result containing transaction hash as hex string or error
    #[wasm_bindgen(js_name = executeRaw)]
    pub async unsafe fn execute_raw(&self, calldata: Calls) -> Result<String, JsValue> {
        let calldata = calldata.iter().map(|c| c.into()).collect();

        let call = self.0.execute_v1(calldata);

        let result = call.send().await;

        match result {
            Ok(res) => Ok(format!("{:#x}", res.transaction_hash)),
            Err(e) => Err(JsValue::from_str(&e.to_string())),
        }
    }

    /// Deploys a burner wallet
    ///
    /// # Parameters
    /// * `private_key` - Private key for the burner wallet as hex string
    ///
    /// # Returns
    /// Result containing new Account instance or error
    #[wasm_bindgen(js_name = deployBurner)]
    pub async unsafe fn deploy_burner(&self, private_key: &str) -> Result<Account, JsValue> {
        let private_key = match Felt::from_str(private_key) {
            Ok(key) => key,
            Err(e) => return Err(JsValue::from(format!("failed to parse private key: {e}"))),
        };

        let signing_key = SigningKey::from_secret_scalar(private_key);
        let verifying_key = signing_key.verifying_key();
        let address = get_contract_address(
            verifying_key.scalar(),
            constants::KATANA_ACCOUNT_CLASS_HASH,
            &[verifying_key.scalar()],
            Felt::ZERO,
        );
        let signer = LocalWallet::from_signing_key(signing_key);

        let chain_id = self.0.chain_id();

        let provider = self.0.provider().clone();
        let account =
            SingleOwnerAccount::new(provider, signer, address, chain_id, ExecutionEncoding::New);

        // deploy the burner
        let exec = self.0.execute_v1(vec![starknet::core::types::Call {
            to: constants::UDC_ADDRESS,
            calldata: vec![
                constants::KATANA_ACCOUNT_CLASS_HASH, // class_hash
                verifying_key.scalar(),               // salt
                Felt::ZERO,                           // deployer_address
                Felt::ONE,                            // constructor calldata length (1)
                verifying_key.scalar(),               // constructor calldata
            ],
            selector: starknet::core::utils::get_selector_from_name("deployContract").unwrap(),
        }]);

        let result = exec.send().await;

        if let Err(e) = result {
            return Err(JsValue::from(format!("failed to deploy burner: {e}",)));
        }

        let result = result.unwrap();

        let _ = watch_tx(self.0.provider(), result.transaction_hash).await;

        Result::Ok(Account(account))
    }

    /// Gets the current nonce for the account
    ///
    /// # Returns
    /// Result containing nonce as hex string or error
    #[wasm_bindgen(js_name = nonce)]
    pub async unsafe fn nonce(&self) -> Result<String, JsValue> {
        let nonce = self.0.get_nonce().await.map_err(|e| JsValue::from(e.to_string()))?;
        Ok(format!("{:#x}", nonce))
    }
}

/// Computes a contract address from deployment parameters
///
/// # Parameters
/// * `class_hash` - Contract class hash as hex string
/// * `salt` - Salt value as hex string
/// * `constructor_calldata` - Array of constructor parameters as hex strings
/// * `deployer_address` - Address of deployer as hex string
///
/// # Returns
/// Result containing computed contract address as hex string or error
#[wasm_bindgen(js_name = hashGetContractAddress)]
pub fn hash_get_contract_address(
    class_hash: &str,
    salt: &str,
    constructor_calldata: Vec<String>,
    deployer_address: &str,
) -> Result<String, JsValue> {
    let class_hash = Felt::from_str(class_hash)
        .map_err(|err| JsValue::from(format!("failed to parse class hash: {err}")))?;
    let salt = Felt::from_str(salt)
        .map_err(|err| JsValue::from(format!("failed to parse salt: {err}")))?;
    let deployer_address = Felt::from_str(deployer_address)
        .map_err(|err| JsValue::from(format!("failed to parse deployer address: {err}")))?;

    let constructor_calldata = constructor_calldata
        .iter()
        .map(|c| {
            Felt::from_str(c).map_err(|err| {
                JsValue::from(format!("failed to parse constructor calldata: {err}"))
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let address = get_contract_address(salt, class_hash, &constructor_calldata, deployer_address);

    Ok(format!("{:#x}", address))
}

/// Computes a selector from a tag string
///
/// # Parameters
/// * `tag` - Tag string to compute selector from
///
/// # Returns
/// Selector as hex string
#[wasm_bindgen(js_name = getSelectorFromTag)]
pub fn get_selector_from_tag(tag: &str) -> String {
    let selector = compute_selector_from_tag(tag);
    format!("{:#x}", selector)
}

/// Serializes a string into a Cairo byte array
///
/// # Parameters
/// * `str` - String to serialize
///
/// # Returns
/// Result containing array of field elements as hex strings or error
#[wasm_bindgen(js_name = byteArraySerialize)]
pub fn bytearray_serialize(str: &str) -> Result<Vec<String>, JsValue> {
    let bytearray = match ByteArray::from_string(str) {
        Ok(bytearray) => bytearray,
        Err(e) => return Err(JsValue::from(format!("failed to parse bytearray: {e}"))),
    };
    let felts = cairo_serde::ByteArray::cairo_serialize(&bytearray);

    Ok(felts.iter().map(|f| format!("{:#x}", f)).collect())
}

/// Deserializes a Cairo byte array into a string
///
/// # Parameters
/// * `felts` - Array of field elements as hex strings
///
/// # Returns
/// Result containing deserialized string or error
#[wasm_bindgen(js_name = byteArrayDeserialize)]
pub fn bytearray_deserialize(felts: Vec<String>) -> Result<String, JsValue> {
    let felts = felts
        .iter()
        .map(|f| Felt::from_str(f))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| JsValue::from(format!("failed to parse felts: {e}")))?;

    let bytearray = match cairo_serde::ByteArray::cairo_deserialize(&felts, 0) {
        Ok(bytearray) => bytearray,
        Err(e) => return Err(JsValue::from(format!("failed to deserialize bytearray: {e}"))),
    };

    match bytearray.to_string() {
        Ok(s) => Ok(s),
        Err(e) => Err(JsValue::from(format!("failed to serialize bytearray: {e}"))),
    }
}

/// Computes a Poseidon hash of the inputs
///
/// # Parameters
/// * `inputs` - Array of field elements as hex strings
///
/// # Returns
/// Result containing hash as hex string or error
#[wasm_bindgen(js_name = poseidonHash)]
pub fn poseidon_hash(inputs: Vec<String>) -> Result<String, JsValue> {
    let inputs = inputs
        .iter()
        .map(|i| Felt::from_str(i))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| JsValue::from(format!("failed to parse inputs: {e}")))?;

    Ok(format!("{:#x}", poseidon_hash_many(&inputs)))
}

/// Gets a selector from a function name
///
/// # Parameters
/// * `name` - Function name to compute selector from
///
/// # Returns
/// Result containing selector as hex string or error
#[wasm_bindgen(js_name = getSelectorFromName)]
pub fn get_selector_from_name(name: &str) -> Result<String, JsValue> {
    let selector = starknet::core::utils::get_selector_from_name(name)
        .map_err(|e| JsValue::from(e.to_string()))?;
    Ok(format!("{:#x}", selector))
}

/// Computes the Starknet variant of Keccak hash
///
/// # Parameters
/// * `inputs` - Byte array to hash
///
/// # Returns
/// Result containing hash as hex string or error
#[wasm_bindgen(js_name = starknetKeccak)]
pub fn starknet_keccak(inputs: js_sys::Uint8Array) -> Result<String, JsValue> {
    let inputs = inputs.to_vec();

    let hash = starknet::core::utils::starknet_keccak(&inputs);
    Ok(format!("{:#x}", hash))
}

/// Converts a short string to a Cairo field element
///
/// # Parameters
/// * `str` - String to convert
///
/// # Returns
/// Result containing field element as hex string or error
#[wasm_bindgen(js_name = cairoShortStringToFelt)]
pub fn cairo_short_string_to_felt(str: &str) -> Result<String, JsValue> {
    let felt = starknet::core::utils::cairo_short_string_to_felt(str)
        .map_err(|e| JsValue::from(e.to_string()))?;

    Ok(format!("{:#x}", felt))
}

/// Parses a Cairo field element into a short string
///
/// # Parameters
/// * `str` - Field element as hex string
///
/// # Returns
/// Result containing parsed string or error
#[wasm_bindgen(js_name = parseCairoShortString)]
pub fn parse_cairo_short_string(str: &str) -> Result<String, JsValue> {
    let felt =
        Felt::from_str(str).map_err(|e| JsValue::from(format!("failed to parse felt: {e}")))?;
    let string = starknet::core::utils::parse_cairo_short_string(&felt)
        .map_err(|e| JsValue::from(format!("failed to parse cairo short string: {e}")))?;

    Ok(string)
}

#[wasm_bindgen]
impl ToriiClient {
    /// Gets controllers along with their usernames for the given contract addresses
    ///
    /// # Parameters
    /// * `contract_addresses` - Array of contract addresses as hex strings. If empty, all
    ///   controllers will be returned.
    ///
    /// # Returns
    /// Result containing controllers or error
    #[wasm_bindgen(js_name = getControllers)]
    pub async fn get_controllers(
        &self,
        contract_addresses: Vec<String>,
    ) -> Result<Controllers, JsValue> {
        let contract_addresses = contract_addresses
            .iter()
            .map(|c| Felt::from_str(c))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| JsValue::from(format!("failed to parse contract addresses: {e}")))?;

        let controllers = self
            .inner
            .controllers(contract_addresses)
            .await
            .map_err(|e| JsValue::from(format!("failed to get controllers: {e}")))?;

        Ok(Controllers(controllers.iter().map(|c| c.into()).collect()))
    }

    /// Gets token information for the given contract addresses
    ///
    /// # Parameters
    /// * `contract_addresses` - Array of contract addresses as hex strings
    ///
    /// # Returns
    /// Result containing token information or error
    #[wasm_bindgen(js_name = getTokens)]
    pub async fn get_tokens(
        &self,
        contract_addresses: Vec<String>,
        token_ids: Vec<WasmU256>,
    ) -> Result<Tokens, JsValue> {
        let contract_addresses = contract_addresses
            .iter()
            .map(|c| Felt::from_str(c))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| JsValue::from(format!("failed to parse contract addresses: {e}")))?;

        let token_ids = token_ids.iter().map(|t| t.into()).collect::<Vec<_>>();

        let tokens = self
            .inner
            .tokens(contract_addresses, token_ids)
            .await
            .map_err(|e| JsValue::from(format!("failed to get tokens: {e}")))?;

        Ok(Tokens(tokens.iter().map(|t| t.into()).collect()))
    }

    /// Subscribes to token updates
    ///
    /// # Parameters
    /// * `contract_addresses` - Array of contract addresses as hex strings
    /// * `callback` - JavaScript function to call on updates
    ///
    /// # Returns
    /// Result containing subscription handle or error
    #[wasm_bindgen(js_name = onTokenUpdated)]
    pub fn on_token_updated(
        &self,
        contract_addresses: Vec<String>,
        token_ids: Vec<WasmU256>,
        callback: js_sys::Function,
    ) -> Result<Subscription, JsValue> {
        #[cfg(feature = "console-error-panic")]
        console_error_panic_hook::set_once();

        let contract_addresses = contract_addresses
            .iter()
            .map(|addr| {
                Felt::from_str(addr).map_err(|err| {
                    JsValue::from(format!("failed to parse contract address: {err}"))
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        let token_ids = token_ids.iter().map(|t| t.into()).collect::<Vec<_>>();

        let subscription_id = Arc::new(AtomicU64::new(0));
        let (trigger, tripwire) = Tripwire::new();

        let subscription = Subscription { id: Arc::clone(&subscription_id), trigger };

        // Spawn a new task to handle the stream and reconnections
        let client = self.inner.clone();
        let subscription_id_clone = Arc::clone(&subscription_id);
        wasm_bindgen_futures::spawn_local(async move {
            let mut backoff = 1000;
            let max_backoff = 60000;

            loop {
                if let Ok(stream) =
                    client.on_token_updated(contract_addresses.clone(), token_ids.clone()).await
                {
                    backoff = 1000; // Reset backoff on successful connection

                    let mut stream = stream.take_until_if(tripwire.clone());

                    while let Some(Ok((id, token))) = stream.next().await {
                        subscription_id_clone.store(id, Ordering::SeqCst);
                        let token: Token = (&token).into();

                        let _ = callback.call1(
                            &JsValue::null(),
                            &token.serialize(&JSON_COMPAT_SERIALIZER).unwrap(),
                        );
                    }
                }

                // If we've reached this point, the stream has ended (possibly due to disconnection)
                // We'll try to reconnect after a delay, unless the tripwire has been triggered
                if tripwire.clone().now_or_never().unwrap_or_default() {
                    break; // Exit the loop if the subscription has been cancelled
                }
                gloo_timers::future::TimeoutFuture::new(backoff).await;
                backoff = std::cmp::min(backoff * 2, max_backoff);
            }
        });

        Ok(subscription)
    }

    /// Gets token balances for given accounts and contracts
    ///
    /// # Parameters
    /// * `contract_addresses` - Array of contract addresses as hex strings
    /// * `account_addresses` - Array of account addresses as hex strings
    ///
    /// # Returns
    /// Result containing token balances or error
    #[wasm_bindgen(js_name = getTokenBalances)]
    pub async fn get_token_balances(
        &self,
        contract_addresses: Vec<String>,
        account_addresses: Vec<String>,
        token_ids: Vec<WasmU256>,
    ) -> Result<TokenBalances, JsValue> {
        let account_addresses = account_addresses
            .iter()
            .map(|a| Felt::from_str(a))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| JsValue::from(format!("failed to parse account addresses: {e}")))?;

        let contract_addresses = contract_addresses
            .iter()
            .map(|c| Felt::from_str(c))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| JsValue::from(format!("failed to parse contract addresses: {e}")))?;

        let token_ids = token_ids.iter().map(|t| t.into()).collect::<Vec<_>>();

        let token_balances = self
            .inner
            .token_balances(account_addresses, contract_addresses, token_ids)
            .await
            .map_err(|e| JsValue::from(format!("failed to get token balances: {e}")))?;

        Ok(TokenBalances(token_balances.iter().map(|t| t.into()).collect()))
    }

    /// Queries entities based on the provided query parameters
    ///
    /// # Parameters
    /// * `query` - Query parameters for filtering entities
    ///
    /// # Returns
    /// Result containing matching entities or error
    #[wasm_bindgen(js_name = getEntities)]
    pub async fn get_entities(&self, query: Query) -> Result<Entities, JsValue> {
        #[cfg(feature = "console-error-panic")]
        console_error_panic_hook::set_once();

        let results = self.inner.entities((&query).into()).await;

        match results {
            Ok(entities) => Ok((&entities).into()),
            Err(err) => Err(JsValue::from(format!("failed to get entities: {err}"))),
        }
    }

    /// Gets all entities with pagination
    ///
    /// # Parameters
    /// * `limit` - Maximum number of entities to return
    /// * `offset` - Number of entities to skip
    ///
    /// # Returns
    /// Result containing paginated entities or error
    #[wasm_bindgen(js_name = getAllEntities)]
    pub async fn get_all_entities(&self, limit: u32, offset: u32) -> Result<Entities, JsValue> {
        #[cfg(feature = "console-error-panic")]
        console_error_panic_hook::set_once();

        let results = self
            .inner
            .entities(torii_grpc::types::Query {
                limit,
                offset,
                clause: None,
                dont_include_hashed_keys: false,
                order_by: vec![],
                entity_models: vec![],
                entity_updated_after: 0,
            })
            .await;

        match results {
            Ok(entities) => Ok((&entities).into()),
            Err(err) => Err(JsValue::from(format!("failed to get entities: {err}"))),
        }
    }

    /// Gets event messages based on query parameters
    ///
    /// # Parameters
    /// * `query` - Query parameters for filtering messages
    /// * `historical` - Whether to include historical messages
    ///
    /// # Returns
    /// Result containing matching event messages or error
    #[wasm_bindgen(js_name = getEventMessages)]
    pub async fn get_event_messages(
        &self,
        query: Query,
        historical: bool,
    ) -> Result<Entities, JsValue> {
        #[cfg(feature = "console-error-panic")]
        console_error_panic_hook::set_once();

        let results = self.inner.event_messages((&query).into(), historical).await;

        match results {
            Ok(event_messages) => Ok((&event_messages).into()),
            Err(err) => Err(JsValue::from(format!("failed to get event_messages: {err}"))),
        }
    }

    /// Subscribes to entity updates
    ///
    /// # Parameters
    /// * `clauses` - Array of key clauses for filtering updates
    /// * `callback` - JavaScript function to call on updates
    ///
    /// # Returns
    /// Result containing subscription handle or error
    #[wasm_bindgen(js_name = onEntityUpdated)]
    pub fn on_entity_updated(
        &self,
        clauses: KeysClauses,
        callback: js_sys::Function,
    ) -> Result<Subscription, JsValue> {
        #[cfg(feature = "console-error-panic")]
        console_error_panic_hook::set_once();

        let clauses: Vec<_> = clauses.iter().map(|c| c.into()).collect();
        let subscription_id = Arc::new(AtomicU64::new(0));
        let (trigger, tripwire) = Tripwire::new();

        let subscription = Subscription { id: Arc::clone(&subscription_id), trigger };

        // Spawn a new task to handle the stream and reconnections
        let client = self.inner.clone();
        let subscription_id_clone = Arc::clone(&subscription_id);
        wasm_bindgen_futures::spawn_local(async move {
            let mut backoff = 1000;
            let max_backoff = 60000;

            loop {
                if let Ok(stream) = client.on_entity_updated(clauses.clone()).await {
                    backoff = 1000; // Reset backoff on successful connection

                    let mut stream = stream.take_until_if(tripwire.clone());

                    while let Some(Ok((id, entity))) = stream.next().await {
                        subscription_id_clone.store(id, Ordering::SeqCst);
                        let models: Entity = (&entity).into();

                        let _ = callback.call2(
                            &JsValue::null(),
                            &JsValue::from_str(&format!("{:#x}", entity.hashed_keys)),
                            &models.serialize(&JSON_COMPAT_SERIALIZER).unwrap(),
                        );
                    }
                }

                // If we've reached this point, the stream has ended (possibly due to disconnection)
                // We'll try to reconnect after a delay, unless the tripwire has been triggered
                if tripwire.clone().now_or_never().unwrap_or_default() {
                    break; // Exit the loop if the subscription has been cancelled
                }
                gloo_timers::future::TimeoutFuture::new(backoff).await;
                backoff = std::cmp::min(backoff * 2, max_backoff);
            }
        });

        Ok(subscription)
    }

    /// Updates an existing entity subscription
    ///
    /// # Parameters
    /// * `subscription` - Existing subscription to update
    /// * `clauses` - New array of key clauses for filtering
    ///
    /// # Returns
    /// Result containing unit or error
    #[wasm_bindgen(js_name = updateEntitySubscription)]
    pub async fn update_entity_subscription(
        &self,
        subscription: &Subscription,
        clauses: KeysClauses,
    ) -> Result<(), JsValue> {
        let clauses = clauses.iter().map(|c| c.into()).collect();
        self.inner
            .update_entity_subscription(subscription.id.load(Ordering::SeqCst), clauses)
            .await
            .map_err(|err| JsValue::from(format!("failed to update subscription: {err}")))
    }

    /// Subscribes to event message updates
    ///
    /// # Parameters
    /// * `clauses` - Array of key clauses for filtering updates
    /// * `historical` - Whether to include historical messages
    /// * `callback` - JavaScript function to call on updates
    ///
    /// # Returns
    /// Result containing subscription handle or error
    #[wasm_bindgen(js_name = onEventMessageUpdated)]
    pub fn on_event_message_updated(
        &self,
        clauses: KeysClauses,
        historical: bool,
        callback: js_sys::Function,
    ) -> Result<Subscription, JsValue> {
        #[cfg(feature = "console-error-panic")]
        console_error_panic_hook::set_once();

        let clauses: Vec<_> = clauses.iter().map(|c| c.into()).collect();
        let subscription_id = Arc::new(AtomicU64::new(0));
        let (trigger, tripwire) = Tripwire::new();

        let subscription = Subscription { id: Arc::clone(&subscription_id), trigger };

        // Spawn a new task to handle the stream and reconnections
        let client = self.inner.clone();
        let subscription_id_clone = Arc::clone(&subscription_id);
        wasm_bindgen_futures::spawn_local(async move {
            let mut backoff = 1000;
            let max_backoff = 60000;

            loop {
                if let Ok(stream) =
                    client.on_event_message_updated(clauses.clone(), historical).await
                {
                    backoff = 1000; // Reset backoff on successful connection

                    let mut stream = stream.take_until_if(tripwire.clone());

                    while let Some(Ok((id, entity))) = stream.next().await {
                        subscription_id_clone.store(id, Ordering::SeqCst);
                        let models: Entity = (&entity).into();

                        let _ = callback.call2(
                            &JsValue::null(),
                            &JsValue::from_str(&format!("{:#x}", entity.hashed_keys)),
                            &models.serialize(&JSON_COMPAT_SERIALIZER).unwrap(),
                        );
                    }
                }

                // If we've reached this point, the stream has ended (possibly due to disconnection)
                // We'll try to reconnect after a delay, unless the tripwire has been triggered
                if tripwire.clone().now_or_never().unwrap_or_default() {
                    break; // Exit the loop if the subscription has been cancelled
                }
                gloo_timers::future::TimeoutFuture::new(backoff).await;
                backoff = std::cmp::min(backoff * 2, max_backoff);
            }
        });

        Ok(subscription)
    }

    /// Updates an existing event message subscription
    ///
    /// # Parameters
    /// * `subscription` - Existing subscription to update
    /// * `clauses` - New array of key clauses for filtering
    /// * `historical` - Whether to include historical messages
    ///
    /// # Returns
    /// Result containing unit or error
    #[wasm_bindgen(js_name = updateEventMessageSubscription)]
    pub async fn update_event_message_subscription(
        &self,
        subscription: &Subscription,
        clauses: KeysClauses,
        historical: bool,
    ) -> Result<(), JsValue> {
        let clauses = clauses.iter().map(|c| c.into()).collect();
        self.inner
            .update_event_message_subscription(
                subscription.id.load(Ordering::SeqCst),
                clauses,
                historical,
            )
            .await
            .map_err(|err| JsValue::from(format!("failed to update subscription: {err}")))
    }

    /// Subscribes to Starknet events
    ///
    /// # Parameters
    /// * `clauses` - Array of key clauses for filtering events
    /// * `callback` - JavaScript function to call on events
    ///
    /// # Returns
    /// Result containing subscription handle or error
    #[wasm_bindgen(js_name = onStarknetEvent)]
    pub fn on_starknet_event(
        &self,
        clauses: KeysClauses,
        callback: js_sys::Function,
    ) -> Result<Subscription, JsValue> {
        #[cfg(feature = "console-error-panic")]
        console_error_panic_hook::set_once();

        let clauses: Vec<_> = clauses.iter().map(|c| c.into()).collect();
        let subscription_id = Arc::new(AtomicU64::new(0));
        let (trigger, tripwire) = Tripwire::new();

        let subscription = Subscription { id: Arc::clone(&subscription_id), trigger };

        // Spawn a new task to handle the stream and reconnections
        let client = self.inner.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let mut backoff = 1000;
            let max_backoff = 60000;

            loop {
                if let Ok(stream) = client.on_starknet_event(clauses.clone()).await {
                    backoff = 1000; // Reset backoff on successful connection

                    let mut stream = stream.take_until_if(tripwire.clone());

                    while let Some(Ok(event)) = stream.next().await {
                        let _ = callback.call1(
                            &JsValue::null(),
                            &event.serialize(&JSON_COMPAT_SERIALIZER).unwrap(),
                        );
                    }
                }

                // If we've reached this point, the stream has ended (possibly due to disconnection)
                // We'll try to reconnect after a delay, unless the tripwire has been triggered
                if tripwire.clone().now_or_never().unwrap_or_default() {
                    break; // Exit the loop if the subscription has been cancelled
                }
                gloo_timers::future::TimeoutFuture::new(backoff).await;
                backoff = std::cmp::min(backoff * 2, max_backoff);
            }
        });

        Ok(subscription)
    }

    /// Subscribes to indexer updates
    ///
    /// # Parameters
    /// * `contract_address` - Optional contract address to filter updates
    /// * `callback` - JavaScript function to call on updates
    ///
    /// # Returns
    /// Result containing subscription handle or error
    #[wasm_bindgen(js_name = onIndexerUpdated)]
    pub fn on_indexer_updated(
        &self,
        contract_address: Option<String>,
        callback: js_sys::Function,
    ) -> Result<Subscription, JsValue> {
        #[cfg(feature = "console-error-panic")]
        console_error_panic_hook::set_once();

        let contract_address = contract_address
            .map(|c| {
                Felt::from_str(c.as_str()).map_err(|err| {
                    JsValue::from(format!("failed to parse contract address: {err}"))
                })
            })
            .transpose()?;
        let subscription_id = Arc::new(AtomicU64::new(0));
        let (trigger, tripwire) = Tripwire::new();

        let subscription = Subscription { id: Arc::clone(&subscription_id), trigger };

        // Spawn a new task to handle the stream and reconnections
        let client = self.inner.clone();
        let subscription_id_clone = Arc::clone(&subscription_id);
        wasm_bindgen_futures::spawn_local(async move {
            let mut backoff = 1000;
            let max_backoff = 60000;

            loop {
                if let Ok(stream) = client.on_indexer_updated(contract_address).await {
                    backoff = 1000; // Reset backoff on successful connection

                    let mut stream = stream.take_until_if(tripwire.clone());

                    while let Some(Ok(update)) = stream.next().await {
                        let update: IndexerUpdate = (&update).into();

                        let _ = callback.call1(
                            &JsValue::null(),
                            &update.serialize(&JSON_COMPAT_SERIALIZER).unwrap(),
                        );
                    }
                }

                // If we've reached this point, the stream has ended (possibly due to disconnection)
                // We'll try to reconnect after a delay, unless the tripwire has been triggered
                if tripwire.clone().now_or_never().unwrap_or_default() {
                    break; // Exit the loop if the subscription has been cancelled
                }
                gloo_timers::future::TimeoutFuture::new(backoff).await;
                backoff = std::cmp::min(backoff * 2, max_backoff);
            }
        });

        Ok(subscription)
    }

    /// Subscribes to token balance updates
    ///
    /// # Parameters
    /// * `contract_addresses` - Array of contract addresses to filter (empty for all)
    /// * `account_addresses` - Array of account addresses to filter (empty for all)
    /// * `callback` - JavaScript function to call on updates
    ///
    /// # Returns
    /// Result containing subscription handle or error
    #[wasm_bindgen(js_name = onTokenBalanceUpdated)]
    pub fn on_token_balance_updated(
        &self,
        contract_addresses: Vec<String>,
        account_addresses: Vec<String>,
        token_ids: Vec<WasmU256>,
        callback: js_sys::Function,
    ) -> Result<Subscription, JsValue> {
        #[cfg(feature = "console-error-panic")]
        console_error_panic_hook::set_once();

        let account_addresses = account_addresses
            .iter()
            .map(|addr| {
                Felt::from_str(addr)
                    .map_err(|err| JsValue::from(format!("failed to parse account address: {err}")))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let contract_addresses = contract_addresses
            .iter()
            .map(|addr| {
                Felt::from_str(addr).map_err(|err| {
                    JsValue::from(format!("failed to parse contract address: {err}"))
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        let token_ids = token_ids.iter().map(|t| t.into()).collect::<Vec<_>>();

        let subscription_id = Arc::new(AtomicU64::new(0));
        let (trigger, tripwire) = Tripwire::new();

        let subscription = Subscription { id: Arc::clone(&subscription_id), trigger };

        // Spawn a new task to handle the stream and reconnections
        let client = self.inner.clone();
        let subscription_id_clone = Arc::clone(&subscription_id);
        wasm_bindgen_futures::spawn_local(async move {
            let mut backoff = 1000;
            let max_backoff = 60000;

            loop {
                if let Ok(stream) = client
                    .on_token_balance_updated(
                        contract_addresses.clone(),
                        account_addresses.clone(),
                        token_ids.clone(),
                    )
                    .await
                {
                    backoff = 1000; // Reset backoff on successful connection

                    let mut stream = stream.take_until_if(tripwire.clone());

                    while let Some(Ok((id, balance))) = stream.next().await {
                        subscription_id_clone.store(id, Ordering::SeqCst);
                        let balance: TokenBalance = (&balance).into();

                        let _ = callback.call1(
                            &JsValue::null(),
                            &balance.serialize(&JSON_COMPAT_SERIALIZER).unwrap(),
                        );
                    }
                }

                // If we've reached this point, the stream has ended (possibly due to disconnection)
                // We'll try to reconnect after a delay, unless the tripwire has been triggered
                if tripwire.clone().now_or_never().unwrap_or_default() {
                    break; // Exit the loop if the subscription has been cancelled
                }
                gloo_timers::future::TimeoutFuture::new(backoff).await;
                backoff = std::cmp::min(backoff * 2, max_backoff);
            }
        });

        Ok(subscription)
    }

    /// Updates an existing token balance subscription
    ///
    /// # Parameters
    /// * `subscription` - Existing subscription to update
    /// * `contract_addresses` - New array of contract addresses to filter
    /// * `account_addresses` - New array of account addresses to filter
    ///
    /// # Returns
    /// Result containing unit or error
    #[wasm_bindgen(js_name = updateTokenBalanceSubscription)]
    pub async fn update_token_balance_subscription(
        &self,
        subscription: &Subscription,
        contract_addresses: Vec<String>,
        account_addresses: Vec<String>,
        token_ids: Vec<WasmU256>,
    ) -> Result<(), JsValue> {
        let account_addresses = account_addresses
            .iter()
            .map(|addr| {
                Felt::from_str(addr)
                    .map_err(|err| JsValue::from(format!("failed to parse account address: {err}")))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let contract_addresses = contract_addresses
            .iter()
            .map(|addr| {
                Felt::from_str(addr).map_err(|err| {
                    JsValue::from(format!("failed to parse contract address: {err}"))
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        let token_ids = token_ids.iter().map(|t| t.into()).collect::<Vec<_>>();

        self.inner
            .update_token_balance_subscription(
                subscription.id.load(Ordering::SeqCst),
                contract_addresses,
                account_addresses,
                token_ids,
            )
            .await
            .map_err(|err| JsValue::from(format!("failed to update subscription: {err}")))
    }

    /// Publishes a message to the network
    ///
    /// # Parameters
    /// * `message` - Message to publish as JSON string
    /// * `signature` - Array of signature field elements as hex strings
    ///
    /// # Returns
    /// Result containing message ID as byte array or error
    #[wasm_bindgen(js_name = publishMessage)]
    pub async fn publish_message(
        &mut self,
        message: &str,
        signature: Vec<String>,
    ) -> Result<js_sys::Uint8Array, JsValue> {
        #[cfg(feature = "console-error-panic")]
        console_error_panic_hook::set_once();

        let message = serde_json::from_str(message)
            .map_err(|err| JsValue::from(format!("failed to parse message: {err}")))?;

        let signature = signature
            .iter()
            .map(|s| Felt::from_str(s.as_str()))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|err| JsValue::from(format!("failed to parse signature: {err}")))?;

        let message_id = self
            .inner
            .publish_message(Message { message, signature })
            .await
            .map_err(|err| JsValue::from(err.to_string()))?;

        Ok(message_id.as_slice().into())
    }
}

#[wasm_bindgen]
impl Subscription {
    /// Cancels an active subscription
    pub fn cancel(self) {
        self.trigger.cancel();
    }
}

/// Creates a new Torii client with the given configuration
///
/// # Parameters
/// * `config` - Client configuration including URLs and world address
///
/// # Returns
/// Result containing ToriiClient instance or error
#[wasm_bindgen(js_name = createClient)]
#[allow(non_snake_case)]
pub async fn create_client(config: ClientConfig) -> Result<ToriiClient, JsValue> {
    #[cfg(feature = "console-error-panic")]
    console_error_panic_hook::set_once();

    let ClientConfig { torii_url, relay_url, world_address } = config;

    let world_address = Felt::from_str(&world_address)
        .map_err(|err| JsValue::from(format!("failed to parse world address: {err}")))?;

    let client = torii_client::client::Client::new(torii_url, relay_url, world_address)
        .await
        .map_err(|err| JsValue::from(format!("failed to build client: {err}")))?;

    let relay_runner = client.relay_runner();
    wasm_bindgen_futures::spawn_local(async move {
        relay_runner.lock().await.run().await;
    });

    Ok(ToriiClient { inner: Arc::new(client) })
}
