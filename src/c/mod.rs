mod types;

use std::ffi::{c_void, CStr, CString};
use std::fs;
use std::net::SocketAddr;
use std::ops::Deref;
use std::os::raw::c_char;
use std::str::FromStr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use account_sdk::abigen::controller::OutsideExecutionV3;
use account_sdk::account::outside_execution::{
    OutsideExecution, OutsideExecutionAccount, OutsideExecutionCaller,
};
use account_sdk::account::session::account::SessionAccount;
use account_sdk::account::session::hash::Session;
use account_sdk::provider::{CartridgeJsonRpcProvider, CartridgeProvider};
use account_sdk::signers::Signer;
use account_sdk::utils::time::get_current_timestamp;
use axum::extract::State;
use axum::http::{header, HeaderValue, Method, StatusCode};
use axum::response::IntoResponse;
use axum::routing::post;
use axum::Router;
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine as _;
use cainome::cairo_serde::{self, ByteArray, CairoSerde};
use crypto_bigint::U256;
use directories::ProjectDirs;
use dojo_world::contracts::naming::compute_selector_from_tag;
use futures::FutureExt;
use keyring::Entry;
use lazy_static::lazy_static;
use starknet::accounts::{
    Account as StarknetAccount, ConnectedAccount, ExecutionEncoding, SingleOwnerAccount,
};
use starknet::core::types::FunctionCall;
use starknet::core::utils::get_contract_address;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::{JsonRpcClient, Provider as _};
use starknet::signers::{LocalWallet, SigningKey, VerifyingKey};
use starknet_crypto::{poseidon_hash_many, Felt};
use stream_cancel::{StreamExt as _, Tripwire};
use tokio::net::TcpListener;
use tokio::runtime::Runtime;
use tokio::time::sleep;
use tokio_stream::StreamExt;
use torii_client::client::Client as TClient;
use torii_relay::types::Message;
use torii_typed_data::TypedData;
use tower_http::cors::{AllowOrigin, CorsLayer};
use types::{
    BlockId, CArray, COption, Call, Controller, Entity, EntityKeysClause, Error, Event,
    IndexerUpdate, Page, Policy, Query, Result, Signature, Struct, Token, TokenBalance,
    ToriiClient, Ty, WorldMetadata,
};
use url::Url;

use crate::constants;
use crate::types::{
    Account, ControllerAccount, Provider, RegisterSessionResponse, RegisteredAccount,
    RegisteredSession, SessionsStorage, Subscription,
};
use crate::utils::watch_tx;

lazy_static! {
    static ref RUNTIME: Arc<Runtime> =
        Arc::new(Runtime::new().expect("Failed to create Tokio runtime"));
}

/// Creates a new Torii client instance
///
/// # Parameters
/// * `torii_url` - URL of the Torii server
/// * `libp2p_relay_url` - URL of the libp2p relay server
/// * `world` - World address as a FieldElement
///
/// # Returns
/// Result containing pointer to new ToriiClient instance or error
#[no_mangle]
pub unsafe extern "C" fn client_new(
    torii_url: *const c_char,
    libp2p_relay_url: *const c_char,
    world: types::FieldElement,
) -> Result<*mut ToriiClient> {
    let torii_url = unsafe { CStr::from_ptr(torii_url).to_string_lossy().into_owned() };
    let libp2p_relay_url =
        unsafe { CStr::from_ptr(libp2p_relay_url).to_string_lossy().into_owned() };

    let client_future = TClient::new(torii_url, libp2p_relay_url, world.into());

    let client = match RUNTIME.block_on(client_future) {
        Ok(client) => client,
        Err(e) => return Result::Err(e.into()),
    };

    let relay_runner = client.relay_runner();
    RUNTIME.spawn(async move {
        relay_runner.lock().await.run().await;
    });

    Result::Ok(Box::into_raw(Box::new(ToriiClient { inner: client, logger: None })))
}

// State struct to share data with callback handler
#[derive(Clone)]
struct CallbackState {
    shutdown_tx: tokio::sync::mpsc::Sender<()>,
    rpc_url: String,
    policies: Vec<account_sdk::account::session::policy::Policy>,
    private_key: SigningKey,
    public_key: Felt,
    account_callback: extern "C" fn(*mut ControllerAccount),
}

// Modify handle_callback to call the callback
async fn handle_callback(State(state): State<CallbackState>, body: String) -> impl IntoResponse {
    // Decode base64 payload
    let padded = match body.len() % 4 {
        0 => body,
        n => body + &"=".repeat(4 - n),
    };
    let decoded = match BASE64.decode(padded) {
        Ok(d) => d,
        Err(e) => {
            println!("Failed to decode payload: {}", e);
            return StatusCode::BAD_REQUEST;
        }
    };

    // Parse JSON from decoded bytes
    let payload: RegisterSessionResponse = match serde_json::from_slice(&decoded) {
        Ok(p) => p,
        Err(e) => {
            println!("Failed to deserialize payload: {}", e);
            return StatusCode::BAD_REQUEST;
        }
    };

    let provider = CartridgeJsonRpcProvider::new(Url::from_str(&state.rpc_url).unwrap());
    let chain_id = provider.chain_id().await.unwrap();

    let project_dirs = ProjectDirs::from("org", "dojoengine", "dojo");
    if let Some(proj_dirs) = project_dirs {
        let data_dir = proj_dirs.data_dir();
        fs::create_dir_all(data_dir).unwrap();

        let account_file = data_dir.join("sessions.json");
        let mut sessions_storage =
            SessionsStorage::from_file(account_file.clone()).unwrap_or_default();

        sessions_storage.active = format!("{:#x}/{:#x}", payload.address, chain_id);
        sessions_storage.sessions.entry(sessions_storage.active.clone()).or_default().push(
            RegisteredSession {
                public_key: state.public_key,
                expires_at: payload.expires_at,
                policies: state.policies.clone(),
            },
        );
        sessions_storage.accounts.insert(
            sessions_storage.active.clone(),
            RegisteredAccount {
                username: payload.username.clone(),
                address: payload.address,
                owner_guid: payload.owner_guid,
                chain_id,
                rpc_url: state.rpc_url,
            },
        );

        fs::write(account_file.clone(), serde_json::to_string_pretty(&sessions_storage).unwrap())
            .unwrap();
        println!("Account data saved to {}", account_file.display());
    }

    let signer = Signer::Starknet(state.private_key);
    let session = Session::new(
        state.policies.clone(),
        payload.expires_at,
        &signer.clone().into(),
        Felt::ZERO,
    )
    .unwrap();

    let session_account = SessionAccount::new_as_registered(
        provider,
        signer,
        payload.address,
        chain_id,
        payload.owner_guid,
        session,
    );

    // Call the callback with the new account
    (state.account_callback)(Box::into_raw(Box::new(ControllerAccount {
        account: session_account,
        username: payload.username,
    })));

    // Signal shutdown after handling callback
    state.shutdown_tx.send(()).await.unwrap();
    StatusCode::OK
}

/// Initiates a connection to establish a new session account
///
/// This function:
/// 1. Generates a new signing key pair
/// 2. Starts a local HTTP server to receive the callback
/// 3. Opens the keychain session URL in browser
/// 4. Waits for callback with session details
/// 5. Creates and stores the session
/// 6. Calls the provided callback with the new session account
///
/// # Safety
/// This function is marked as unsafe because it:
/// - Handles raw C pointers
/// - Performs FFI operations
/// - Creates system-level resources (HTTP server, keyring entries)
///
/// # Parameters
/// * `rpc_url` - Pointer to null-terminated string containing the RPC endpoint URL
/// * `policies` - Pointer to array of Policy structs defining session permissions
/// * `policies_len` - Length of the policies array
/// * `account_callback` - Function pointer called with the new session account when ready
///
/// # Example
/// ```c
/// void on_account(SessionAccount* account) {
///     // Handle new session account
/// }
///
/// controller_connect(
///     "https://rpc.example.com",
///     policies,
///     policies_length,
///     on_account
/// );
/// ```
#[no_mangle]
pub unsafe extern "C" fn controller_connect(
    rpc_url: *const c_char,
    policies: *const Policy,
    policies_len: usize,
    account_callback: extern "C" fn(*mut ControllerAccount),
) {
    let rpc_url = unsafe { CStr::from_ptr(rpc_url).to_string_lossy().into_owned() };
    let policies = unsafe { std::slice::from_raw_parts(policies, policies_len) };
    let account_policies = policies
        .iter()
        .map(|p| account_sdk::account::session::policy::Policy::Call(p.clone().into()))
        .collect::<Vec<account_sdk::account::session::policy::Policy>>();
    let policies = policies.iter().map(|p| p.clone().into()).collect::<Vec<crate::types::Policy>>();

    // Generate new random signing key
    let signing_key = SigningKey::from_random();
    let verifying_key = signing_key.verifying_key().scalar();

    // Store signing key in system keyring
    let keyring = Entry::new("dojo-keyring", &format!("{:#x}", verifying_key)).unwrap();
    keyring.set_password(&format!("{:#x}", signing_key.secret_scalar())).unwrap();

    // Create shutdown channel
    let (shutdown_tx, mut shutdown_rx) = tokio::sync::mpsc::channel(1);

    // Create state with RPC URL and shutdown sender
    let state = CallbackState {
        shutdown_tx,
        rpc_url: rpc_url.clone(),
        policies: account_policies,
        private_key: signing_key,
        public_key: verifying_key,
        account_callback,
    };

    // Set up the HTTP callback server with state and CORS
    let app = Router::new()
        .route("/callback", post(handle_callback))
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::exact(HeaderValue::from_static(
                    "https://x.cartridge.gg",
                )))
                .allow_methods([Method::POST])
                .allow_headers([header::CONTENT_TYPE]),
        )
        .with_state(state);

    // Find an available port by trying to bind to port 0
    let addr = SocketAddr::from(([127, 0, 0, 1], 0));
    let listener = RUNTIME.block_on(TcpListener::bind(addr)).unwrap();
    let bound_addr = listener.local_addr().unwrap();

    let server = axum::serve(listener, app);

    // Spawn server with graceful shutdown
    RUNTIME.spawn(async move {
        server
            .with_graceful_shutdown(async move {
                shutdown_rx.recv().await;
                println!("Shutting down server");
            })
            .await
            .unwrap();
    });

    println!("Listening on {}", bound_addr);

    let callback_url = format!("http://{}/callback", bound_addr).replace("127.0.0.1", "localhost");
    let mut url = url::Url::parse(constants::KEYCHAIN_SESSION_URL).unwrap();
    url.query_pairs_mut()
        .append_pair("callback_uri", &callback_url)
        .append_pair("public_key", &format!("{:#x}", verifying_key))
        .append_pair("rpc_url", &rpc_url)
        .append_pair("policies", &serde_json::to_string(&policies).unwrap());

    open::that(url.as_str()).unwrap();
}

/// Retrieves a stored session account if one exists and is valid
///
/// # Parameters
/// * `policies` - Array of policies to match the session
/// * `policies_len` - Length of policies array
/// * `chain_id` - Chain ID to verify against
///
/// # Returns
/// Result containing pointer to SessionAccount or error if no valid account exists
#[no_mangle]
pub unsafe extern "C" fn controller_account(
    policies: *const Policy,
    policies_len: usize,
    chain_id: types::FieldElement,
) -> Result<*mut ControllerAccount> {
    let policies = unsafe { std::slice::from_raw_parts(policies, policies_len) };
    let account_policies: Vec<account_sdk::account::session::policy::Policy> = policies
        .iter()
        .map(|p| account_sdk::account::session::policy::Policy::Call(p.clone().into()))
        .collect();

    let chain_id: Felt = chain_id.into();

    // Get project directories
    let project_dirs = match ProjectDirs::from("org", "dojoengine", "dojo") {
        Some(dirs) => dirs,
        None => {
            return Result::Err(Error {
                message: CString::new("Could not determine project directories")
                    .unwrap()
                    .into_raw(),
            });
        }
    };

    // Load sessions storage
    let account_file = project_dirs.data_dir().join("sessions.json");
    let sessions_storage = match SessionsStorage::from_file(account_file) {
        Ok(storage) => storage,
        Err(_) => {
            return Result::Err(Error {
                message: CString::new("No stored session found").unwrap().into_raw(),
            });
        }
    };

    // Helper function to try creating a session account
    let try_create_session_account = |account: &RegisteredAccount,
                                      session: &RegisteredSession|
     -> Option<ControllerAccount> {
        // Check chain ID
        if account.chain_id != chain_id {
            return None;
        }

        // Check expiration
        let current_time =
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();

        if current_time > session.expires_at {
            return None;
        }

        // Check if policies match
        if session.policies != account_policies {
            return None;
        }

        // Get the signing key from keyring
        let keyring = Entry::new("dojo-keyring", &format!("{:#x}", session.public_key)).ok()?;
        let signing_key_hex = keyring.get_password().ok()?;

        // Initialize provider and signer
        let provider = CartridgeJsonRpcProvider::new(Url::from_str(&account.rpc_url).unwrap());
        let signing_key = SigningKey::from_secret_scalar(Felt::from_hex(&signing_key_hex).unwrap());
        let signer = Signer::Starknet(signing_key);

        // Create new session
        let session = Session::new(
            session.policies.clone(),
            session.expires_at,
            &signer.clone().into(),
            Felt::ZERO,
        )
        .ok()?;

        // Create session account
        let session_account = SessionAccount::new_as_registered(
            provider,
            signer,
            account.address,
            account.chain_id,
            account.owner_guid,
            session,
        );

        Some(ControllerAccount { account: session_account, username: account.username.clone() })
    };

    // First try the active account if it exists
    if !sessions_storage.active.is_empty() {
        if let Some(account) = sessions_storage.accounts.get(&sessions_storage.active) {
            if let Some(sessions) = sessions_storage.sessions.get(&sessions_storage.active) {
                for session in sessions {
                    if let Some(controller) = try_create_session_account(account, session) {
                        return Result::Ok(Box::into_raw(Box::new(controller)));
                    }
                }
            }
        }
    }

    // If active account didn't work, try all accounts
    for (account_key, account) in sessions_storage.accounts.iter() {
        if let Some(sessions) = sessions_storage.sessions.get(account_key) {
            for session in sessions {
                if let Some(controller) = try_create_session_account(account, session) {
                    return Result::Ok(Box::into_raw(Box::new(controller)));
                }
            }
        }
    }

    // No valid session found
    Result::Err(Error {
        message: CString::new("No valid session found with matching chain ID and policies")
            .unwrap()
            .into_raw(),
    })
}

/// Clears sessions matching the specified policies and chain ID
///
/// # Parameters
/// * `policies` - Array of policies to match
/// * `policies_len` - Length of policies array
/// * `chain_id` - Chain ID to match
///
/// # Returns
/// Result containing success boolean or error
#[no_mangle]
pub unsafe extern "C" fn controller_clear(
    policies: *const Policy,
    policies_len: usize,
    chain_id: types::FieldElement,
) -> Result<bool> {
    let policies = unsafe { std::slice::from_raw_parts(policies, policies_len) };
    let account_policies: Vec<account_sdk::account::session::policy::Policy> = policies
        .iter()
        .map(|p| account_sdk::account::session::policy::Policy::Call(p.clone().into()))
        .collect();

    let chain_id: Felt = chain_id.into();

    // Get project directories
    let project_dirs = match ProjectDirs::from("org", "dojoengine", "dojo") {
        Some(dirs) => dirs,
        None => {
            return Result::Err(Error {
                message: CString::new("Could not determine project directories")
                    .unwrap()
                    .into_raw(),
            });
        }
    };

    // Load sessions storage
    let account_file = project_dirs.data_dir().join("sessions.json");
    let mut sessions_storage = match SessionsStorage::from_file(account_file.clone()) {
        Ok(storage) => storage,
        Err(_) => {
            return Result::Err(Error {
                message: CString::new("No stored session found").unwrap().into_raw(),
            });
        }
    };

    let mut modified = false;

    // Get the active account
    if let Some(account) = sessions_storage.accounts.get(&sessions_storage.active) {
        // Only process if chain ID matches
        if account.chain_id == chain_id {
            if let Some(sessions) = sessions_storage.sessions.get_mut(&sessions_storage.active) {
                // Find and remove sessions with matching policies
                let initial_len = sessions.len();
                sessions.retain(|session| {
                    if session.policies == account_policies {
                        // Remove the signing key from keyring
                        if let Ok(keyring) =
                            Entry::new("dojo-keyring", &format!("{:#x}", session.public_key))
                        {
                            let _ = keyring.delete_credential();
                        }
                        false // Remove this session
                    } else {
                        true // Keep this session
                    }
                });
                modified = initial_len != sessions.len();
            }
        }
    }

    // Save changes if any modifications were made
    if modified && sessions_storage.write_to_file(account_file).is_err() {
        return Result::Err(Error {
            message: CString::new("Failed to save updated sessions").unwrap().into_raw(),
        });
    }

    Result::Ok(true)
}

/// Gets the username of controller
///
/// # Parameters
/// * `account` - Pointer to Account
///
/// # Returns
/// CString containing the username
#[no_mangle]
pub unsafe extern "C" fn controller_username(controller: *mut ControllerAccount) -> *const c_char {
    CString::new((*controller).username.to_string()).unwrap().into_raw()
}

/// Gets account address
///
/// # Parameters
/// * `account` - Pointer to Account
///
/// # Returns
/// FieldElement containing the account address
#[no_mangle]
pub unsafe extern "C" fn controller_address(
    controller: *mut ControllerAccount,
) -> types::FieldElement {
    (*controller).account.address().into()
}

/// Gets account chain ID
///
/// # Parameters
/// * `account` - Pointer to Account
///
/// # Returns
/// FieldElement containing the chain ID
#[no_mangle]
pub unsafe extern "C" fn controller_chain_id(
    controller: *mut ControllerAccount,
) -> types::FieldElement {
    (*controller).account.chain_id().into()
}

/// Gets account nonce
///
/// # Parameters
/// * `account` - Pointer to Account
///
/// # Returns
/// Result containing FieldElement nonce or error
#[no_mangle]
pub unsafe extern "C" fn controller_nonce(
    controller: *mut ControllerAccount,
) -> Result<types::FieldElement> {
    let nonce = match RUNTIME.block_on((*controller).account.get_nonce()) {
        Ok(nonce) => nonce,
        Err(e) => return Result::Err(e.into()),
    };

    Result::Ok(nonce.into())
}

/// Executes raw transaction
///
/// # Parameters
/// * `account` - Pointer to Account
/// * `calldata` - Array of Call structs
/// * `calldata_len` - Length of calldata array
///
/// # Returns
/// Result containing transaction hash as FieldElement or error
#[no_mangle]
pub unsafe extern "C" fn controller_execute_raw(
    controller: *mut ControllerAccount,
    calldata: *const Call,
    calldata_len: usize,
) -> Result<types::FieldElement> {
    let calldata = unsafe { std::slice::from_raw_parts(calldata, calldata_len).to_vec() };
    let calldata =
        calldata.into_iter().map(|c| c.into()).collect::<Vec<starknet::core::types::Call>>();
    let call = (*controller).account.execute_v3(calldata);

    match RUNTIME.block_on(call.send()) {
        Ok(result) => Result::Ok(result.transaction_hash.into()),
        Err(e) => {
            println!("Error executing call: {:?}", e);
            Result::Err(e.into())
        }
    }
}

/// Executes a transaction from outside (paymaster)
///
/// # Parameters
/// * `account` - Pointer to Account
/// * `calldata` - Array of Call structs
/// * `calldata_len` - Length of calldata array
///
/// # Returns
/// Result containing transaction hash as FieldElement or error
#[no_mangle]
pub unsafe extern "C" fn controller_execute_from_outside(
    controller: *mut ControllerAccount,
    calldata: *const Call,
    calldata_len: usize,
) -> Result<types::FieldElement> {
    let caller = OutsideExecutionCaller::Any;
    let calls = unsafe { std::slice::from_raw_parts(calldata, calldata_len).to_vec() };
    let calls = calls.into_iter().map(|c| c.into()).collect::<Vec<starknet::core::types::Call>>();
    let now = get_current_timestamp();
    let outside_execution = OutsideExecutionV3 {
        caller: caller.into(),
        execute_after: 0_u64,
        execute_before: now + 600,
        calls: calls.into_iter().map(|c| c.into()).collect(),
        nonce: (SigningKey::from_random().secret_scalar(), 1),
    };

    let signed = match RUNTIME.block_on(
        (*controller)
            .account
            .sign_outside_execution(OutsideExecution::V3(outside_execution.clone())),
    ) {
        Ok(signed) => signed,
        Err(e) => return Result::Err(e.into()),
    };

    let res =
        match RUNTIME.block_on((*controller).account.provider().add_execute_outside_transaction(
            OutsideExecution::V3(outside_execution),
            (*controller).account.address(),
            signed.signature,
        )) {
            Ok(res) => res,
            Err(e) => {
                println!("Error executing call: {:?}", e);
                return Result::Err(e.into());
            }
        };

    Result::Ok(res.transaction_hash.into())
}

/// Sets a logger callback function for the client
///
/// # Parameters
/// * `client` - Pointer to ToriiClient instance
/// * `logger` - Callback function that takes a C string parameter
#[no_mangle]
pub unsafe extern "C" fn client_set_logger(
    client: *mut ToriiClient,
    logger: extern "C" fn(*const c_char),
) {
    unsafe {
        (*client).logger = Some(logger);
    }
}

/// Publishes a message to the network
///
/// # Parameters
/// * `client` - Pointer to ToriiClient instance
/// * `message` - JSON string containing typed data message
/// * `signature_felts` - Array of field elements containing signature
/// * `signature_felts_len` - Length of signature array
///
/// # Returns
/// Result containing byte array or error
#[no_mangle]
pub unsafe extern "C" fn client_publish_message(
    client: *mut ToriiClient,
    // A json string representing the typed data message
    message: *const c_char,
    signature_felts: *const types::FieldElement,
    signature_felts_len: usize,
) -> Result<CArray<u8>> {
    let message = unsafe { CStr::from_ptr(message).to_string_lossy().into_owned() };
    let message = match serde_json::from_str::<TypedData>(message.as_str()) {
        Ok(message) => message,
        Err(e) => return Result::Err(e.into()),
    };

    let signature = unsafe { std::slice::from_raw_parts(signature_felts, signature_felts_len) };
    let signature = signature.iter().map(|f| f.clone().into()).collect::<Vec<Felt>>();

    let client_future = unsafe { (*client).inner.publish_message(Message { message, signature }) };

    match RUNTIME.block_on(client_future) {
        Ok(data) => Result::Ok(data.into()),
        Err(e) => Result::Err(e.into()),
    }
}

/// Retrieves controllers for the given contract addresses
///
/// # Parameters
/// * `client` - Pointer to ToriiClient instance
/// * `contract_addresses` - Array of contract addresses. If empty, all controllers will be
///   returned.
///
/// # Returns
/// Result containing controllers or error
#[no_mangle]
pub unsafe extern "C" fn client_controllers(
    client: *mut ToriiClient,
    contract_addresses: *const types::FieldElement,
    contract_addresses_len: usize,
) -> Result<CArray<Controller>> {
    let contract_addresses = if contract_addresses.is_null() || contract_addresses_len == 0 {
        Vec::new()
    } else {
        let addresses =
            unsafe { std::slice::from_raw_parts(contract_addresses, contract_addresses_len) };
        addresses.iter().map(|f| f.clone().into()).collect::<Vec<Felt>>()
    };

    let controllers_future = unsafe { (*client).inner.controllers(contract_addresses) };

    match RUNTIME.block_on(controllers_future) {
        Ok(controllers) => {
            let controllers: Vec<Controller> = controllers.into_iter().map(|c| c.into()).collect();
            Result::Ok(controllers.into())
        }
        Err(e) => Result::Err(e.into()),
    }
}

/// Queries entities matching given criteria
///
/// # Parameters
/// * `client` - Pointer to ToriiClient instance
/// * `query` - Query parameters
///
/// # Returns
/// Result containing array of matching entities or error
#[no_mangle]
pub unsafe extern "C" fn client_entities(
    client: *mut ToriiClient,
    query: Query,
) -> Result<Page<Entity>> {
    let query = query.clone().into();
    let entities_future = unsafe { (*client).inner.entities(query) };

    match RUNTIME.block_on(entities_future) {
        Ok(entities) => Result::Ok(entities.into()),
        Err(e) => Result::Err(e.into()),
    }
}

/// Retrieves event messages matching the given query
///
/// # Parameters
/// * `client` - Pointer to ToriiClient instance
/// * `query` - Query parameters
/// * `historical` - Whether to include historical messages
///
/// # Returns
/// Result containing array of matching event message entities or error
#[no_mangle]
pub unsafe extern "C" fn client_event_messages(
    client: *mut ToriiClient,
    query: Query,
) -> Result<Page<Entity>> {
    let query = query.clone().into();
    let event_messages_future = unsafe { (*client).inner.event_messages(query) };

    match RUNTIME.block_on(event_messages_future) {
        Ok(event_messages) => Result::Ok(event_messages.into()),
        Err(e) => Result::Err(e.into()),
    }
}

/// Gets the world metadata for the client
///
/// # Parameters
/// * `client` - Pointer to ToriiClient instance
///
/// # Returns
/// WorldMetadata structure containing world information
#[no_mangle]
pub unsafe extern "C" fn client_metadata(client: *mut ToriiClient) -> Result<WorldMetadata> {
    let metadata_future = unsafe { (*client).inner.metadata() };
    match RUNTIME.block_on(metadata_future) {
        Ok(metadata) => Result::Ok(metadata.into()),
        Err(e) => Result::Err(e.into()),
    }
}

/// Subscribes to entity state updates
///
/// # Parameters
/// * `client` - Pointer to ToriiClient instance
/// * `clauses` - Array of entity key clauses to filter updates
/// * `clauses_len` - Length of clauses array
/// * `callback` - Function called when updates occur
///
/// # Returns
/// Result containing pointer to Subscription or error
#[no_mangle]
pub unsafe extern "C" fn client_on_entity_state_update(
    client: *mut ToriiClient,
    clauses: *const EntityKeysClause,
    clauses_len: usize,
    callback: unsafe extern "C" fn(types::FieldElement, CArray<Struct>),
) -> Result<*mut Subscription> {
    let client = Arc::new(unsafe { &*client });
    let clauses = if clauses.is_null() || clauses_len == 0 {
        Vec::new()
    } else {
        let clauses = unsafe { std::slice::from_raw_parts(clauses, clauses_len) };
        clauses.iter().map(|c| c.clone().into()).collect::<Vec<_>>()
    };

    let subscription_id = Arc::new(AtomicU64::new(0));
    let (trigger, tripwire) = Tripwire::new();

    let subscription = Subscription { id: Arc::clone(&subscription_id), trigger };

    // Spawn a new thread to handle the stream and reconnections
    let client_clone = client.clone();
    let subscription_id_clone = Arc::clone(&subscription_id);
    RUNTIME.spawn(async move {
        let mut backoff = Duration::from_secs(1);
        let max_backoff = Duration::from_secs(60);

        loop {
            let rcv = client_clone.inner.on_entity_updated(clauses.clone()).await;
            if let Ok(rcv) = rcv {
                backoff = Duration::from_secs(1); // Reset backoff on successful connection

                let mut rcv = rcv.take_until_if(tripwire.clone());

                while let Some(Ok((id, entity))) = rcv.next().await {
                    subscription_id_clone.store(id, Ordering::SeqCst);
                    let key: types::FieldElement = entity.hashed_keys.into();
                    let models: Vec<Struct> = entity.models.into_iter().map(|e| e.into()).collect();
                    callback(key, models.into());
                }
            }

            // If we've reached this point, the stream has ended (possibly due to disconnection)
            // We'll try to reconnect after a delay, unless the tripwire has been triggered
            if tripwire.clone().now_or_never().unwrap_or_default() {
                break; // Exit the loop if the subscription has been cancelled
            }
            sleep(backoff).await;
            backoff = std::cmp::min(backoff * 2, max_backoff);
        }
    });

    Result::Ok(Box::into_raw(Box::new(subscription)))
}

/// Updates an existing entity subscription with new clauses
///
/// # Parameters
/// * `client` - Pointer to ToriiClient instance
/// * `subscription` - Pointer to existing Subscription
/// * `clauses` - New array of entity key clauses
/// * `clauses_len` - Length of new clauses array
///
/// # Returns
/// Result containing success boolean or error
#[no_mangle]
pub unsafe extern "C" fn client_update_entity_subscription(
    client: *mut ToriiClient,
    subscription: *mut Subscription,
    clauses: *const EntityKeysClause,
    clauses_len: usize,
) -> Result<bool> {
    let clauses = if clauses.is_null() || clauses_len == 0 {
        Vec::new()
    } else {
        let clauses = unsafe { std::slice::from_raw_parts(clauses, clauses_len) };
        clauses.iter().map(|c| c.clone().into()).collect::<Vec<_>>()
    };

    match RUNTIME.block_on(
        (*client)
            .inner
            .update_entity_subscription((*subscription).id.load(Ordering::SeqCst), clauses),
    ) {
        Ok(_) => Result::Ok(true),
        Err(e) => Result::Err(e.into()),
    }
}

/// Subscribes to event message updates
///
/// # Parameters
/// * `client` - Pointer to ToriiClient instance
/// * `clauses` - Array of entity key clauses to filter updates
/// * `clauses_len` - Length of clauses array
/// * `callback` - Function called when updates occur
///
/// # Returns
/// Result containing pointer to Subscription or error
#[no_mangle]
pub unsafe extern "C" fn client_on_event_message_update(
    client: *mut ToriiClient,
    clauses: *const EntityKeysClause,
    clauses_len: usize,
    callback: unsafe extern "C" fn(types::FieldElement, CArray<Struct>),
) -> Result<*mut Subscription> {
    let client = Arc::new(unsafe { &*client });
    let clauses = if clauses.is_null() || clauses_len == 0 {
        Vec::new()
    } else {
        let clauses = unsafe { std::slice::from_raw_parts(clauses, clauses_len) };
        clauses.iter().map(|c| c.clone().into()).collect::<Vec<_>>()
    };

    let subscription_id = Arc::new(AtomicU64::new(0));
    let (trigger, tripwire) = Tripwire::new();

    let subscription = Subscription { id: Arc::clone(&subscription_id), trigger };

    // Spawn a new thread to handle the stream and reconnections
    let client_clone = client.clone();
    let subscription_id_clone = Arc::clone(&subscription_id);
    RUNTIME.spawn(async move {
        let mut backoff = Duration::from_secs(1);
        let max_backoff = Duration::from_secs(60);

        loop {
            let rcv = client_clone.inner.on_event_message_updated(clauses.clone()).await;
            if let Ok(rcv) = rcv {
                backoff = Duration::from_secs(1); // Reset backoff on successful connection

                let mut rcv = rcv.take_until_if(tripwire.clone());

                while let Some(Ok((id, entity))) = rcv.next().await {
                    subscription_id_clone.store(id, Ordering::SeqCst);
                    let key: types::FieldElement = entity.hashed_keys.into();
                    let models: Vec<Struct> = entity.models.into_iter().map(|e| e.into()).collect();
                    callback(key, models.into());
                }
            }

            // If we've reached this point, the stream has ended (possibly due to disconnection)
            // We'll try to reconnect after a delay, unless the tripwire has been triggered
            if tripwire.clone().now_or_never().unwrap_or_default() {
                break; // Exit the loop if the subscription has been cancelled
            }
            sleep(backoff).await;
            backoff = std::cmp::min(backoff * 2, max_backoff);
        }
    });

    Result::Ok(Box::into_raw(Box::new(subscription)))
}

/// Updates an existing event message subscription
///
/// # Parameters
/// * `client` - Pointer to ToriiClient instance
/// * `subscription` - Pointer to existing Subscription
/// * `clauses` - New array of entity key clauses
/// * `clauses_len` - Length of new clauses array
///
/// # Returns
/// Result containing success boolean or error
#[no_mangle]
pub unsafe extern "C" fn client_update_event_message_subscription(
    client: *mut ToriiClient,
    subscription: *mut Subscription,
    clauses: *const EntityKeysClause,
    clauses_len: usize,
) -> Result<bool> {
    let clauses = if clauses.is_null() || clauses_len == 0 {
        Vec::new()
    } else {
        let clauses = unsafe { std::slice::from_raw_parts(clauses, clauses_len) };
        clauses.iter().map(|c| c.clone().into()).collect::<Vec<_>>()
    };

    match RUNTIME.block_on(
        (*client)
            .inner
            .update_event_message_subscription((*subscription).id.load(Ordering::SeqCst), clauses),
    ) {
        Ok(_) => Result::Ok(true),
        Err(e) => Result::Err(e.into()),
    }
}

/// Subscribes to Starknet events
///
/// # Parameters
/// * `client` - Pointer to ToriiClient instance
/// * `clauses` - Array of entity key clauses to filter events
/// * `clauses_len` - Length of clauses array
/// * `callback` - Function called when events occur
///
/// # Returns
/// Result containing pointer to Subscription or error
#[no_mangle]
pub unsafe extern "C" fn client_on_starknet_event(
    client: *mut ToriiClient,
    clauses: *const EntityKeysClause,
    clauses_len: usize,
    callback: unsafe extern "C" fn(Event),
) -> Result<*mut Subscription> {
    let client = Arc::new(unsafe { &*client });
    let clauses = if clauses.is_null() || clauses_len == 0 {
        Vec::new()
    } else {
        let clauses = unsafe { std::slice::from_raw_parts(clauses, clauses_len) };
        clauses.iter().map(|c| c.clone().into()).collect::<Vec<_>>()
    };

    let subscription_id = Arc::new(AtomicU64::new(0));
    let (trigger, tripwire) = Tripwire::new();

    let subscription = Subscription { id: Arc::clone(&subscription_id), trigger };

    // Spawn a new thread to handle the stream and reconnections
    let client_clone = client.clone();
    RUNTIME.spawn(async move {
        let mut backoff = Duration::from_secs(1);
        let max_backoff = Duration::from_secs(60);

        loop {
            let rcv = client_clone.inner.on_starknet_event(clauses.clone()).await;

            if let Ok(rcv) = rcv {
                backoff = Duration::from_secs(1); // Reset backoff on successful connection

                let mut rcv = rcv.take_until_if(tripwire.clone());

                while let Some(Ok(event)) = rcv.next().await {
                    callback(event.into());
                }
            }

            // If we've reached this point, the stream has ended (possibly due to disconnection)
            // We'll try to reconnect after a delay, unless the tripwire has been triggered
            if tripwire.clone().now_or_never().unwrap_or_default() {
                break; // Exit the loop if the subscription has been cancelled
            }
            sleep(backoff).await;
            backoff = std::cmp::min(backoff * 2, max_backoff);
        }
    });

    Result::Ok(Box::into_raw(Box::new(subscription)))
}

/// Retrieves token information for given contract addresses
///
/// # Parameters
/// * `client` - Pointer to ToriiClient instance
/// * `contract_addresses` - Array of contract addresses
/// * `contract_addresses_len` - Length of addresses array
/// * `token_ids` - Array of token ids
/// * `token_ids_len` - Length of token ids array
/// * `limit` - Maximum number of tokens to return
/// * `cursor` - Cursor to start from
///
/// # Returns
/// Result containing array of Token information or error
#[no_mangle]
pub unsafe extern "C" fn client_tokens(
    client: *mut ToriiClient,
    contract_addresses: *const types::FieldElement,
    contract_addresses_len: usize,
    token_ids: *const types::U256,
    token_ids_len: usize,
    limit: u32,
    cursor: COption<*const c_char>,
) -> Result<Page<Token>> {
    let contract_addresses = if contract_addresses.is_null() || contract_addresses_len == 0 {
        Vec::new()
    } else {
        let addresses =
            unsafe { std::slice::from_raw_parts(contract_addresses, contract_addresses_len) };
        addresses.iter().map(|f| f.clone().into()).collect::<Vec<Felt>>()
    };
    let token_ids = if token_ids.is_null() || token_ids_len == 0 {
        Vec::new()
    } else {
        let ids = unsafe { std::slice::from_raw_parts(token_ids, token_ids_len) };
        ids.iter().map(|f| f.clone().into()).collect::<Vec<U256>>()
    };

    let limit = if limit == 0 { None } else { Some(limit) };
    let tokens = match RUNTIME.block_on(
        (*client).inner.tokens(
            contract_addresses,
            token_ids,
            limit,
            cursor
                .map(|c| unsafe { std::ffi::CStr::from_ptr(c).to_string_lossy().into_owned() })
                .into(),
        ),
    ) {
        Ok(tokens) => tokens,
        Err(e) => return Result::Err(e.into()),
    };

    Result::Ok(tokens.into())
}

/// Subscribes to token updates
///
/// # Parameters
/// * `client` - Pointer to ToriiClient instance
/// * `contract_addresses` - Array of contract addresses
/// * `callback` - Function called when updates occur
///
/// # Returns
/// Result containing pointer to Subscription or error
#[no_mangle]
pub unsafe extern "C" fn client_on_token_update(
    client: *mut ToriiClient,
    contract_addresses: *const types::FieldElement,
    contract_addresses_len: usize,
    token_ids: *const types::U256,
    token_ids_len: usize,
    callback: unsafe extern "C" fn(Token),
) -> Result<*mut Subscription> {
    let client = Arc::new(unsafe { &*client });

    // Convert contract addresses array to Vec<Felt> if not empty
    let contract_addresses = if contract_addresses.is_null() || contract_addresses_len == 0 {
        Vec::new()
    } else {
        let addresses =
            unsafe { std::slice::from_raw_parts(contract_addresses, contract_addresses_len) };
        addresses.iter().map(|f| f.clone().into()).collect::<Vec<Felt>>()
    };

    let token_ids = if token_ids.is_null() || token_ids_len == 0 {
        Vec::new()
    } else {
        let ids = unsafe { std::slice::from_raw_parts(token_ids, token_ids_len) };
        ids.iter().map(|f| f.clone().into()).collect::<Vec<U256>>()
    };

    let subscription_id = Arc::new(AtomicU64::new(0));
    let (trigger, tripwire) = Tripwire::new();

    let subscription = Subscription { id: Arc::clone(&subscription_id), trigger };

    // Spawn a new thread to handle the stream and reconnections
    let client_clone = client.clone();
    RUNTIME.spawn(async move {
        let mut backoff = Duration::from_secs(1);
        let max_backoff = Duration::from_secs(60);

        loop {
            let rcv = client_clone
                .inner
                .on_token_updated(contract_addresses.clone(), token_ids.clone())
                .await;

            if let Ok(rcv) = rcv {
                backoff = Duration::from_secs(1); // Reset backoff on successful connection

                let mut rcv = rcv.take_until_if(tripwire.clone());

                while let Some(Ok((id, token))) = rcv.next().await {
                    subscription_id.store(id, Ordering::SeqCst);
                    let token: Token = token.into();
                    callback(token);
                }
            }

            // If we've reached this point, the stream has ended (possibly due to disconnection)
            // We'll try to reconnect after a delay, unless the tripwire has been triggered
            if tripwire.clone().now_or_never().unwrap_or_default() {
                break; // Exit the loop if the subscription has been cancelled
            }
            sleep(backoff).await;
            backoff = std::cmp::min(backoff * 2, max_backoff);
        }
    });

    Result::Ok(Box::into_raw(Box::new(subscription)))
}

/// Gets token balances for given accounts and contracts
///
/// # Parameters
/// * `client` - Pointer to ToriiClient instance
/// * `contract_addresses` - Array of contract addresses
/// * `contract_addresses_len` - Length of contract addresses array
/// * `account_addresses` - Array of account addresses
/// * `account_addresses_len` - Length of account addresses array
/// * `token_ids` - Array of token ids
/// * `token_ids_len` - Length of token ids array
/// * `limit` - Maximum number of token balances to return
/// * `cursor` - Cursor to start from
///
/// # Returns
/// Result containing array of TokenBalance information or error
#[no_mangle]
pub unsafe extern "C" fn client_token_balances(
    client: *mut ToriiClient,
    contract_addresses: *const types::FieldElement,
    contract_addresses_len: usize,
    account_addresses: *const types::FieldElement,
    account_addresses_len: usize,
    token_ids: *const types::U256,
    token_ids_len: usize,
    limit: u32,
    cursor: COption<*const c_char>,
) -> Result<Page<TokenBalance>> {
    let account_addresses = if account_addresses.is_null() || account_addresses_len == 0 {
        Vec::new()
    } else {
        let addresses =
            unsafe { std::slice::from_raw_parts(account_addresses, account_addresses_len) };
        addresses.iter().map(|f| f.clone().into()).collect::<Vec<Felt>>()
    };

    let contract_addresses = if contract_addresses.is_null() || contract_addresses_len == 0 {
        Vec::new()
    } else {
        let addresses =
            unsafe { std::slice::from_raw_parts(contract_addresses, contract_addresses_len) };
        addresses.iter().map(|f| f.clone().into()).collect::<Vec<Felt>>()
    };

    let token_ids = if token_ids.is_null() || token_ids_len == 0 {
        Vec::new()
    } else {
        let ids = unsafe { std::slice::from_raw_parts(token_ids, token_ids_len) };
        ids.iter().map(|f| f.clone().into()).collect::<Vec<U256>>()
    };

    let token_balances = match RUNTIME.block_on(
        (*client).inner.token_balances(
            account_addresses,
            contract_addresses,
            token_ids,
            if limit == 0 { None } else { Some(limit) },
            cursor
                .map(|c| unsafe { std::ffi::CStr::from_ptr(c).to_string_lossy().into_owned() })
                .into(),
        ),
    ) {
        Ok(balances) => balances,
        Err(e) => return Result::Err(e.into()),
    };

    Result::Ok(token_balances.into())
}

/// Subscribes to indexer updates
///
/// # Parameters
/// * `client` - Pointer to ToriiClient instance
/// * `contract_address` - Optional contract address to filter updates
/// * `callback` - Function called when updates occur
///
/// # Returns
/// Result containing pointer to Subscription or error
#[no_mangle]
pub unsafe extern "C" fn on_indexer_update(
    client: *mut ToriiClient,
    contract_address: *const types::FieldElement,
    callback: unsafe extern "C" fn(IndexerUpdate),
) -> Result<*mut Subscription> {
    let client = Arc::new(unsafe { &*client });
    let contract_address = if contract_address.is_null() {
        None
    } else {
        Some(unsafe { (*contract_address).clone().into() })
    };

    let subscription_id = Arc::new(AtomicU64::new(0));
    let (trigger, tripwire) = Tripwire::new();

    let subscription = Subscription { id: Arc::clone(&subscription_id), trigger };

    // Spawn a new thread to handle the stream and reconnections
    let client_clone = client.clone();
    RUNTIME.spawn(async move {
        let mut backoff = Duration::from_secs(1);
        let max_backoff = Duration::from_secs(60);

        loop {
            let rcv = client_clone.inner.on_indexer_updated(contract_address).await;
            if let Ok(rcv) = rcv {
                backoff = Duration::from_secs(1); // Reset backoff on successful connection

                let mut rcv = rcv.take_until_if(tripwire.clone());

                while let Some(Ok(update)) = rcv.next().await {
                    callback(update.into());
                }
            }

            // If we've reached this point, the stream has ended (possibly due to disconnection)
            // We'll try to reconnect after a delay, unless the tripwire has been triggered
            if tripwire.clone().now_or_never().unwrap_or_default() {
                break; // Exit the loop if the subscription has been cancelled
            }
            sleep(backoff).await;
            backoff = std::cmp::min(backoff * 2, max_backoff);
        }
    });

    Result::Ok(Box::into_raw(Box::new(subscription)))
}

/// Subscribes to token balance updates
///
/// # Parameters
/// * `client` - Pointer to ToriiClient instance
/// * `contract_addresses` - Array of contract addresses to filter (empty for all)
/// * `contract_addresses_len` - Length of contract addresses array
/// * `account_addresses` - Array of account addresses to filter (empty for all)
/// * `account_addresses_len` - Length of account addresses array
/// * `callback` - Function called when updates occur
///
/// # Returns
/// Result containing pointer to Subscription or error
#[no_mangle]
pub unsafe extern "C" fn client_on_token_balance_update(
    client: *mut ToriiClient,
    contract_addresses: *const types::FieldElement,
    contract_addresses_len: usize,
    account_addresses: *const types::FieldElement,
    account_addresses_len: usize,
    token_ids: *const types::U256,
    token_ids_len: usize,
    callback: unsafe extern "C" fn(TokenBalance),
) -> Result<*mut Subscription> {
    let client = Arc::new(unsafe { &*client });

    // Convert account addresses array to Vec<Felt> if not empty
    let account_addresses = if account_addresses.is_null() || account_addresses_len == 0 {
        Vec::new()
    } else {
        let addresses =
            unsafe { std::slice::from_raw_parts(account_addresses, account_addresses_len) };
        addresses.iter().map(|f| f.clone().into()).collect::<Vec<Felt>>()
    };

    // Convert contract addresses array to Vec<Felt> if not empty
    let contract_addresses = if contract_addresses.is_null() || contract_addresses_len == 0 {
        Vec::new()
    } else {
        let addresses =
            unsafe { std::slice::from_raw_parts(contract_addresses, contract_addresses_len) };
        addresses.iter().map(|f| f.clone().into()).collect::<Vec<Felt>>()
    };

    let token_ids = if token_ids.is_null() || token_ids_len == 0 {
        Vec::new()
    } else {
        let ids = unsafe { std::slice::from_raw_parts(token_ids, token_ids_len) };
        ids.iter().map(|f| f.clone().into()).collect::<Vec<U256>>()
    };

    let subscription_id = Arc::new(AtomicU64::new(0));
    let (trigger, tripwire) = Tripwire::new();

    let subscription = Subscription { id: Arc::clone(&subscription_id), trigger };

    // Spawn a new thread to handle the stream and reconnections
    let client_clone = client.clone();
    RUNTIME.spawn(async move {
        let mut backoff = Duration::from_secs(1);
        let max_backoff = Duration::from_secs(60);

        loop {
            let rcv = client_clone
                .inner
                .on_token_balance_updated(
                    contract_addresses.clone(),
                    account_addresses.clone(),
                    token_ids.clone(),
                )
                .await;

            if let Ok(rcv) = rcv {
                backoff = Duration::from_secs(1); // Reset backoff on successful connection

                let mut rcv = rcv.take_until_if(tripwire.clone());

                while let Some(Ok((id, balance))) = rcv.next().await {
                    subscription_id.store(id, Ordering::SeqCst);
                    let balance: TokenBalance = balance.into();
                    callback(balance);
                }
            }

            // If we've reached this point, the stream has ended (possibly due to disconnection)
            // We'll try to reconnect after a delay, unless the tripwire has been triggered
            if tripwire.clone().now_or_never().unwrap_or_default() {
                break; // Exit the loop if the subscription has been cancelled
            }
            sleep(backoff).await;
            backoff = std::cmp::min(backoff * 2, max_backoff);
        }
    });

    Result::Ok(Box::into_raw(Box::new(subscription)))
}

/// Updates an existing token balance subscription
///
/// # Parameters
/// * `client` - Pointer to ToriiClient instance
/// * `subscription` - Pointer to existing Subscription
/// * `contract_addresses` - Array of contract addresses to filter (empty for all)
/// * `contract_addresses_len` - Length of contract addresses array
/// * `account_addresses` - Array of account addresses to filter (empty for all)
/// * `account_addresses_len` - Length of account addresses array
///
/// # Returns
/// Result containing success boolean or error
#[no_mangle]
pub unsafe extern "C" fn client_update_token_balance_subscription(
    client: *mut ToriiClient,
    subscription: *mut Subscription,
    contract_addresses: *const types::FieldElement,
    contract_addresses_len: usize,
    account_addresses: *const types::FieldElement,
    account_addresses_len: usize,
    token_ids: *const types::U256,
    token_ids_len: usize,
) -> Result<bool> {
    let contract_addresses = if contract_addresses.is_null() || contract_addresses_len == 0 {
        Vec::new()
    } else {
        let addresses =
            unsafe { std::slice::from_raw_parts(contract_addresses, contract_addresses_len) };
        addresses.iter().map(|f| f.clone().into()).collect::<Vec<Felt>>()
    };

    let account_addresses = if account_addresses.is_null() || account_addresses_len == 0 {
        Vec::new()
    } else {
        let addresses =
            unsafe { std::slice::from_raw_parts(account_addresses, account_addresses_len) };
        addresses.iter().map(|f| f.clone().into()).collect::<Vec<Felt>>()
    };

    let token_ids = if token_ids.is_null() || token_ids_len == 0 {
        Vec::new()
    } else {
        let ids = unsafe { std::slice::from_raw_parts(token_ids, token_ids_len) };
        ids.iter().map(|f| f.clone().into()).collect::<Vec<U256>>()
    };

    match RUNTIME.block_on((*client).inner.update_token_balance_subscription(
        (*subscription).id.load(Ordering::SeqCst),
        contract_addresses,
        account_addresses,
        token_ids,
    )) {
        Ok(_) => Result::Ok(true),
        Err(e) => Result::Err(e.into()),
    }
}

/// Serializes a string into a byte array
///
/// # Parameters
/// * `str` - String to serialize
///
/// # Returns
/// Result containing array of FieldElements or error
#[no_mangle]
pub unsafe extern "C" fn bytearray_serialize(
    str: *const c_char,
) -> Result<CArray<types::FieldElement>> {
    let str = unsafe { CStr::from_ptr(str).to_string_lossy().into_owned() };
    let bytearray = match ByteArray::from_string(str.as_str()) {
        Ok(bytearray) => bytearray,
        Err(e) => return Result::Err(e.into()),
    };

    let felts = cairo_serde::ByteArray::cairo_serialize(&bytearray);
    let felts = felts.into_iter().map(|f| f.into()).collect::<Vec<types::FieldElement>>();
    Result::Ok(felts.into())
}

/// Deserializes field elements into a string
///
/// # Parameters
/// * `felts` - Array of field elements
/// * `felts_len` - Length of field elements array
///
/// # Returns
/// Result containing pointer to C string or error
#[no_mangle]
pub unsafe extern "C" fn bytearray_deserialize(
    felts: *const types::FieldElement,
    felts_len: usize,
) -> Result<*const c_char> {
    let felts = unsafe { std::slice::from_raw_parts(felts, felts_len) };
    let felts = felts.iter().map(|f| f.clone().into()).collect::<Vec<Felt>>();
    let bytearray = match cairo_serde::ByteArray::cairo_deserialize(&felts, 0) {
        Ok(bytearray) => bytearray,
        Err(e) => return Result::Err(e.into()),
    };

    let bytearray = match bytearray.to_string() {
        Ok(bytearray) => bytearray,
        Err(e) => return Result::Err(e.into()),
    };

    Result::Ok(CString::new(bytearray).unwrap().into_raw())
}

/// Computes Poseidon hash of field elements
///
/// # Parameters
/// * `felts` - Array of field elements
/// * `felts_len` - Length of array
///
/// # Returns
/// FieldElement containing the hash result
#[no_mangle]
pub unsafe extern "C" fn poseidon_hash(
    felts: *const types::FieldElement,
    felts_len: usize,
) -> types::FieldElement {
    let felts = unsafe { std::slice::from_raw_parts(felts, felts_len) };
    let felts = felts.iter().map(|f| f.clone().into()).collect::<Vec<Felt>>();

    poseidon_hash_many(&felts).into()
}

/// Gets selector from name string
///
/// # Parameters
/// * `name` - Name to compute selector from
///
/// # Returns
/// Result containing FieldElement selector or error
#[no_mangle]
pub unsafe extern "C" fn get_selector_from_name(
    name: *const c_char,
) -> Result<types::FieldElement> {
    let name = unsafe { CStr::from_ptr(name).to_string_lossy().into_owned() };
    let selector = match starknet::core::utils::get_selector_from_name(name.as_str()) {
        Ok(selector) => selector,
        Err(e) => return Result::Err(e.into()),
    };

    Result::Ok(selector.into())
}

/// Gets selector from tag string
///
/// # Parameters
/// * `tag` - Tag to compute selector from
///
/// # Returns
/// FieldElement containing the computed selector
#[no_mangle]
pub unsafe extern "C" fn get_selector_from_tag(tag: *const c_char) -> types::FieldElement {
    let tag = unsafe { CStr::from_ptr(tag).to_string_lossy().into_owned() };
    let selector = compute_selector_from_tag(tag.as_str());

    selector.into()
}

/// Computes Starknet keccak hash of bytes
///
/// # Parameters
/// * `bytes` - Byte array to hash
/// * `bytes_len` - Length of byte array
///
/// # Returns
/// FieldElement containing the hash result
#[no_mangle]
pub unsafe extern "C" fn starknet_keccak(
    bytes: *const u8,
    bytes_len: usize,
) -> types::FieldElement {
    let bytes = unsafe { std::slice::from_raw_parts(bytes, bytes_len) };
    let hash = starknet::core::utils::starknet_keccak(bytes);

    hash.into()
}

/// Converts a short string to field element
///
/// # Parameters
/// * `str` - String to convert
///
/// # Returns
/// Result containing FieldElement or error
#[no_mangle]
pub unsafe extern "C" fn cairo_short_string_to_felt(
    str: *const c_char,
) -> Result<types::FieldElement> {
    let str = unsafe { CStr::from_ptr(str).to_string_lossy().into_owned() };
    let felt = match starknet::core::utils::cairo_short_string_to_felt(str.as_str()) {
        Ok(felt) => felt,
        Err(e) => return Result::Err(e.into()),
    };

    Result::Ok(felt.into())
}

/// Parses a field element into a short string
///
/// # Parameters
/// * `felt` - FieldElement to parse
///
/// # Returns
/// Result containing pointer to C string or error
#[no_mangle]
pub unsafe extern "C" fn parse_cairo_short_string(
    felt: types::FieldElement,
) -> Result<*const c_char> {
    let felt = felt.into();
    let str = match starknet::core::utils::parse_cairo_short_string(&felt) {
        Ok(str) => str,
        Err(e) => return Result::Err(e.into()),
    };

    Result::Ok(CString::new(str).unwrap().into_raw())
}

/// Encodes typed data
///
/// # Parameters
/// * `typed_data` - JSON string of typed data
/// * `address` - Address as FieldElement
///
/// # Returns
/// Result containing encoded FieldElement or error
#[no_mangle]
pub unsafe extern "C" fn typed_data_encode(
    typed_data: *const c_char,
    address: types::FieldElement,
) -> Result<types::FieldElement> {
    let typed_data = unsafe { CStr::from_ptr(typed_data).to_string_lossy().into_owned() };
    let typed_data = match serde_json::from_str::<TypedData>(typed_data.as_str()) {
        Ok(typed_data) => typed_data,
        Err(err) => {
            return Result::Err(Error {
                message: CString::new(format!("Invalid typed data: {}", err)).unwrap().into_raw(),
            });
        }
    };

    let address = address.into();
    let encoded = match typed_data.encode(address) {
        Ok(encoded) => encoded,
        Err(err) => return Result::Err(err.into()),
    };

    Result::Ok(encoded.into())
}

/// Generates a new signing key
///
/// # Returns
/// FieldElement containing the new private key
#[no_mangle]
pub unsafe extern "C" fn signing_key_new() -> types::FieldElement {
    let private_key = SigningKey::from_random();
    private_key.secret_scalar().into()
}

/// Signs a hash with a private key
///
/// # Parameters
/// * `private_key` - Private key as FieldElement
/// * `hash` - Hash to sign as FieldElement
///
/// # Returns
/// Result containing Signature or error
#[no_mangle]
pub unsafe extern "C" fn signing_key_sign(
    private_key: types::FieldElement,
    hash: types::FieldElement,
) -> Result<Signature> {
    let private_key = SigningKey::from_secret_scalar(private_key.into());
    let sig = private_key.sign(&hash.into());

    match sig {
        Ok(sig) => Result::Ok(sig.into()),
        Err(e) => Result::Err(e.into()),
    }
}

/// Creates a verifying key from a signing key
///
/// # Parameters
/// * `signing_key` - Signing key as FieldElement
///
/// # Returns
/// FieldElement containing the verifying key
#[no_mangle]
pub unsafe extern "C" fn verifying_key_new(
    signing_key: types::FieldElement,
) -> types::FieldElement {
    let signing_key = signing_key.into();
    let verifying_key = starknet_crypto::get_public_key(&signing_key);

    verifying_key.into()
}

/// Verifies a signature
///
/// # Parameters
/// * `verifying_key` - Verifying key as FieldElement
/// * `hash` - Hash that was signed
/// * `signature` - Signature to verify
///
/// # Returns
/// Result containing verification success boolean or error
#[no_mangle]
pub unsafe extern "C" fn verifying_key_verify(
    verifying_key: types::FieldElement,
    hash: types::FieldElement,
    signature: types::Signature,
) -> Result<bool> {
    let verifying_key = VerifyingKey::from_scalar(verifying_key.into());
    let signature = &signature.into();
    let hash = &hash.into();

    match verifying_key.verify(hash, signature) {
        Ok(result) => Result::Ok(result),
        Err(e) => Result::Err(e.into()),
    }
}

/// Creates a new provider instance
///
/// # Parameters
/// * `rpc_url` - URL of the RPC endpoint
///
/// # Returns
/// Result containing pointer to Provider or error
#[no_mangle]
pub unsafe extern "C" fn provider_new(rpc_url: *const c_char) -> Result<*mut Provider> {
    let rpc_url = unsafe { CStr::from_ptr(rpc_url).to_string_lossy() };
    let rpc_url = match url::Url::parse(rpc_url.deref()) {
        Ok(url) => url,
        Err(e) => return Result::Err(e.into()),
    };

    let rpc = JsonRpcClient::new(HttpTransport::new(rpc_url));

    Result::Ok(Box::into_raw(Box::new(Provider(Arc::new(rpc)))))
}

/// Creates a new account instance
///
/// # Parameters
/// * `rpc` - Pointer to Provider
/// * `private_key` - Private key as FieldElement
/// * `address` - Account address as string
///
/// # Returns
/// Result containing pointer to Account or error
#[no_mangle]
pub unsafe extern "C" fn account_new(
    rpc: *mut Provider,
    private_key: types::FieldElement,
    address: *const c_char,
) -> Result<*mut Account> {
    let address = unsafe { CStr::from_ptr(address).to_string_lossy() };
    let address = match Felt::from_hex(address.deref()) {
        Ok(address) => address,
        Err(e) => return Result::Err(e.into()),
    };

    let chain_id = match RUNTIME.block_on((*rpc).0.chain_id()) {
        Ok(chain_id) => chain_id,
        Err(e) => return Result::Err(e.into()),
    };

    let signer = LocalWallet::from_signing_key(SigningKey::from_secret_scalar(private_key.into()));
    let account = SingleOwnerAccount::new(
        (*rpc).0.clone(),
        signer,
        address,
        chain_id,
        ExecutionEncoding::New,
    );

    Result::Ok(Box::into_raw(Box::new(Account(account))))
}

/// Makes a Starknet call
///
/// # Parameters
/// * `provider` - Pointer to Provider
/// * `call` - Call parameters
/// * `block_id` - Block identifier
///
/// # Returns
/// Result containing array of FieldElements or error
#[no_mangle]
pub unsafe extern "C" fn starknet_call(
    provider: *mut Provider,
    call: Call,
    block_id: BlockId,
) -> Result<CArray<types::FieldElement>> {
    let res = match RUNTIME.block_on(
        (*provider)
            .0
            .call::<FunctionCall, starknet::core::types::BlockId>(call.into(), block_id.into()),
    ) {
        Ok(res) => res,
        Err(e) => return Result::Err(e.into()),
    };

    Result::Ok(res.into())
}

/// Deploys a burner account
///
/// # Parameters
/// * `provider` - Pointer to Provider
/// * `master_account` - Pointer to master Account
/// * `signing_key` - Signing key for new account
///
/// # Returns
/// Result containing pointer to new Account or error
#[no_mangle]
pub unsafe extern "C" fn account_deploy_burner(
    provider: *mut Provider,
    master_account: *mut Account,
    signing_key: types::FieldElement,
) -> Result<*mut Account> {
    let signing_key = SigningKey::from_secret_scalar(signing_key.into());
    let verifying_key = signing_key.verifying_key();
    let address = get_contract_address(
        verifying_key.scalar(),
        constants::KATANA_ACCOUNT_CLASS_HASH,
        &[verifying_key.scalar()],
        Felt::ZERO,
    );
    let signer = LocalWallet::from_signing_key(signing_key);

    let chain_id = (*master_account).0.chain_id();

    let account = SingleOwnerAccount::new(
        (*provider).0.clone(),
        signer,
        address,
        chain_id,
        ExecutionEncoding::New,
    );

    // deploy the burner
    let exec = (*master_account).0.execute_v3(vec![starknet::core::types::Call {
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

    let result = match RUNTIME.block_on(exec.send()) {
        Ok(result) => result,
        Err(e) => return Result::Err(e.into()),
    };

    match RUNTIME.block_on(watch_tx(&(*provider).0, result.transaction_hash)) {
        Ok(_) => Result::Ok(Box::into_raw(Box::new(Account(account)))),
        Err(e) => Result::Err(Error { message: CString::new(e.to_string()).unwrap().into_raw() }),
    }
}

/// Gets account address
///
/// # Parameters
/// * `account` - Pointer to Account
///
/// # Returns
/// FieldElement containing the account address
#[no_mangle]
pub unsafe extern "C" fn account_address(account: *mut Account) -> types::FieldElement {
    (*account).0.address().into()
}

/// Gets account chain ID
///
/// # Parameters
/// * `account` - Pointer to Account
///
/// # Returns
/// FieldElement containing the chain ID
#[no_mangle]
pub unsafe extern "C" fn account_chain_id(account: *mut Account) -> types::FieldElement {
    (*account).0.chain_id().into()
}

/// Sets block ID for account
///
/// # Parameters
/// * `account` - Pointer to Account
/// * `block_id` - New block ID
#[no_mangle]
pub unsafe extern "C" fn account_set_block_id(account: *mut Account, block_id: BlockId) {
    let block_id = block_id.into();
    (*account).0.set_block_id(block_id);
}

/// Gets account nonce
///
/// # Parameters
/// * `account` - Pointer to Account
///
/// # Returns
/// Result containing FieldElement nonce or error
#[no_mangle]
pub unsafe extern "C" fn account_nonce(account: *mut Account) -> Result<types::FieldElement> {
    let nonce = match RUNTIME.block_on((*account).0.get_nonce()) {
        Ok(nonce) => nonce,
        Err(e) => return Result::Err(e.into()),
    };

    Result::Ok(nonce.into())
}

/// Executes raw transaction
///
/// # Parameters
/// * `account` - Pointer to Account
/// * `calldata` - Array of Call structs
/// * `calldata_len` - Length of calldata array
///
/// # Returns
/// Result containing transaction hash as FieldElement or error
#[no_mangle]
pub unsafe extern "C" fn account_execute_raw(
    account: *mut Account,
    calldata: *const Call,
    calldata_len: usize,
) -> Result<types::FieldElement> {
    let calldata = unsafe { std::slice::from_raw_parts(calldata, calldata_len).to_vec() };
    let calldata =
        calldata.into_iter().map(|c| c.into()).collect::<Vec<starknet::core::types::Call>>();
    let call = (*account).0.execute_v3(calldata);

    match RUNTIME.block_on(call.send()) {
        Ok(result) => Result::Ok(result.transaction_hash.into()),
        Err(e) => Result::Err(e.into()),
    }
}

/// Waits for transaction completion
///
/// # Parameters
/// * `rpc` - Pointer to Provider
/// * `txn_hash` - Transaction hash as FieldElement
///
/// # Returns
/// Result containing success boolean or error
#[no_mangle]
pub unsafe extern "C" fn wait_for_transaction(
    rpc: *mut Provider,
    txn_hash: types::FieldElement,
) -> Result<bool> {
    let txn_hash = txn_hash.into();
    match RUNTIME.block_on(watch_tx(&(*rpc).0, txn_hash)) {
        Ok(_) => Result::Ok(true),
        Err(e) => Result::Err(Error { message: CString::new(e.to_string()).unwrap().into_raw() }),
    }
}

/// Computes contract address
///
/// # Parameters
/// * `class_hash` - Class hash as FieldElement
/// * `salt` - Salt as FieldElement
/// * `constructor_calldata` - Array of constructor parameters
/// * `constructor_calldata_len` - Length of constructor parameters
/// * `deployer_address` - Deployer address as FieldElement
///
/// # Returns
/// FieldElement containing computed contract address
#[no_mangle]
pub unsafe extern "C" fn hash_get_contract_address(
    class_hash: types::FieldElement,
    salt: types::FieldElement,
    constructor_calldata: *const types::FieldElement,
    constructor_calldata_len: usize,
    deployer_address: types::FieldElement,
) -> types::FieldElement {
    let class_hash = class_hash.into();
    let salt = salt.into();
    let constructor_calldata = unsafe {
        std::slice::from_raw_parts(constructor_calldata, constructor_calldata_len).to_vec()
    };
    let constructor_calldata =
        constructor_calldata.iter().map(|f| f.clone().into()).collect::<Vec<Felt>>();
    let deployer_address = deployer_address.into();

    let address = get_contract_address(salt, class_hash, &constructor_calldata, deployer_address);

    address.into()
}

/// Cancels a subscription
///
/// # Parameters
/// * `subscription` - Pointer to Subscription to cancel
#[no_mangle]
pub unsafe extern "C" fn subscription_cancel(subscription: *mut Subscription) {
    if !subscription.is_null() {
        unsafe {
            let subscription = Box::from_raw(subscription);
            subscription.trigger.cancel();
        }
    }
}

/// Frees a ToriiClient instance
///
/// # Parameters
/// * `t` - Pointer to ToriiClient to free
#[no_mangle]
pub unsafe extern "C" fn client_free(t: *mut ToriiClient) {
    if !t.is_null() {
        unsafe {
            let _ = Box::from_raw(t);
        }
    }
}

/// Frees a Provider instance
///
/// # Parameters
/// * `rpc` - Pointer to Provider to free
#[no_mangle]
pub unsafe extern "C" fn provider_free(rpc: *mut Provider) {
    if !rpc.is_null() {
        unsafe {
            let _ = Box::from_raw(rpc);
        }
    }
}

/// Frees a Model instance
///
/// # Parameters
/// * `model` - Pointer to Model to free
#[no_mangle]
pub unsafe extern "C" fn model_free(model: *mut Struct) {
    if !model.is_null() {
        let _: dojo_types::schema::Struct = (*Box::<Struct>::from_raw(model)).into();
    }
}

/// Frees an Account instance
///
/// # Parameters
/// * `account` - Pointer to Account to free
#[no_mangle]
pub unsafe extern "C" fn account_free(account: *mut Account) {
    if !account.is_null() {
        unsafe {
            let _ = Box::from_raw(account);
        }
    }
}

/// Frees a Type instance
///
/// # Parameters
/// * `ty` - Pointer to Type to free
#[no_mangle]
pub unsafe extern "C" fn ty_free(ty: *mut Ty) {
    if !ty.is_null() {
        let _: dojo_types::schema::Ty = (*Box::<Ty>::from_raw(ty)).into();
    }
}

/// Frees an Entity instance
///
/// # Parameters
/// * `entity` - Pointer to Entity to free
#[no_mangle]
pub unsafe extern "C" fn entity_free(entity: *mut Entity) {
    if !entity.is_null() {
        let _: torii_grpc::types::schema::Entity = (*Box::<Entity>::from_raw(entity)).into();
    }
}

/// Frees an Error instance
///
/// # Parameters
/// * `error` - Pointer to Error to free
#[no_mangle]
pub unsafe extern "C" fn error_free(error: *mut Error) {
    if !error.is_null() {
        let _: String = CString::from_raw((*error).message).into_string().unwrap();
    }
}

/// Frees a WorldMetadata instance
///
/// # Parameters
/// * `metadata` - Pointer to WorldMetadata to free
#[no_mangle]
pub unsafe extern "C" fn world_metadata_free(metadata: *mut WorldMetadata) {
    if !metadata.is_null() {
        let _: dojo_types::WorldMetadata = (*Box::<WorldMetadata>::from_raw(metadata)).into();
    }
}

/// Frees a CArray instance
///
/// # Parameters
/// * `data` - Pointer to array data
/// * `data_len` - Length of array
#[no_mangle]
pub unsafe extern "C" fn carray_free(data: *mut c_void, data_len: usize) {
    if !data.is_null() {
        let _: Vec<c_void> = Vec::from_raw_parts(data, data_len, data_len);
    }
}

/// Frees a string
///
/// # Parameters
/// * `string` - Pointer to string to free
#[no_mangle]
pub unsafe extern "C" fn string_free(string: *mut c_char) {
    if !string.is_null() {
        let _: String = CString::from_raw(string).into_string().unwrap();
    }
}
