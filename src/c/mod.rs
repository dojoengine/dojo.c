mod types;

use std::ffi::{CStr, CString, c_void};
use std::ops::Deref;
use std::os::raw::c_char;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use cainome::cairo_serde::{self, ByteArray, CairoSerde};
use dojo_world::contracts::naming::compute_selector_from_tag;
use starknet::accounts::{
    Account as StarknetAccount, ConnectedAccount, ExecutionEncoding, SingleOwnerAccount,
};
use starknet::core::types::FunctionCall;
use starknet::core::utils::get_contract_address;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::{JsonRpcClient, Provider as _};
use starknet::signers::{LocalWallet, SigningKey, VerifyingKey};
use starknet_crypto::{Felt, poseidon_hash_many};
use stream_cancel::{StreamExt as _, Tripwire};
use tokio::time::sleep;
use tokio_stream::StreamExt;
use torii_client::client::Client as TClient;
use torii_relay::typed_data::TypedData;
use torii_relay::types::Message;
use types::{EntityKeysClause, Event, IndexerUpdate, Struct};

use self::types::{
    BlockId, CArray, Call, Entity, Error, Query, Result, Signature, ToriiClient, Ty, WorldMetadata,
};
use crate::constants;
use crate::types::{Account, Provider, Subscription};
use crate::utils::watch_tx;

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_new(
    torii_url: *const c_char,
    rpc_url: *const c_char,
    libp2p_relay_url: *const c_char,
    world: types::FieldElement,
) -> Result<*mut ToriiClient> {
    let torii_url = unsafe { CStr::from_ptr(torii_url).to_string_lossy().into_owned() };
    let rpc_url = unsafe { CStr::from_ptr(rpc_url).to_string_lossy().into_owned() };
    let libp2p_relay_url =
        unsafe { CStr::from_ptr(libp2p_relay_url).to_string_lossy().into_owned() };

    let client_future = TClient::new(torii_url, rpc_url, libp2p_relay_url, (&world).into());

    let runtime = tokio::runtime::Runtime::new().unwrap();
    let client = match runtime.block_on(client_future) {
        Ok(client) => client,
        Err(e) => return Result::Err(e.into()),
    };

    let relay_runner = client.relay_runner();
    runtime.spawn(async move {
        relay_runner.lock().await.run().await;
    });

    Result::Ok(Box::into_raw(Box::new(ToriiClient { inner: client, runtime, logger: None })))
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_set_logger(
    client: *mut ToriiClient,
    logger: extern "C" fn(*const c_char),
) {
    unsafe {
        (*client).logger = Some(logger);
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
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
    let signature = signature.iter().map(|f| (&f.clone()).into()).collect::<Vec<Felt>>();

    let client_future = unsafe { (*client).inner.publish_message(Message { message, signature }) };

    match (*client).runtime.block_on(client_future) {
        Ok(data) => Result::Ok(data.into()),
        Err(e) => Result::Err(e.into()),
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_entities(
    client: *mut ToriiClient,
    query: &Query,
) -> Result<CArray<Entity>> {
    let entities_future = unsafe { (*client).inner.entities(query.into()) };

    match (*client).runtime.block_on(entities_future) {
        Ok(entities) => {
            let entities: Vec<Entity> = entities.into_iter().map(|e| (&e).into()).collect();

            Result::Ok(entities.into())
        }
        Err(e) => Result::Err(e.into()),
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_event_messages(
    client: *mut ToriiClient,
    query: &Query,
) -> Result<CArray<Entity>> {
    let event_messages_future = unsafe { (*client).inner.event_messages(query.into()) };

    match (*client).runtime.block_on(event_messages_future) {
        Ok(event_messages) => {
            let event_messages: Vec<Entity> =
                event_messages.into_iter().map(|e| (&e).into()).collect();

            Result::Ok(event_messages.into())
        }
        Err(e) => Result::Err(e.into()),
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_metadata(client: *mut ToriiClient) -> WorldMetadata {
    unsafe { (&(*client).inner.metadata().clone()).into() }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_on_entity_state_update(
    client: *mut ToriiClient,
    clauses: *const EntityKeysClause,
    clauses_len: usize,
    callback: unsafe extern "C" fn(types::FieldElement, CArray<Struct>),
) -> Result<*mut Subscription> {
    let client = Arc::from_raw(client);
    let clauses = unsafe { std::slice::from_raw_parts(clauses, clauses_len) };
    let clauses = clauses.iter().map(|c| c.into()).collect::<Vec<_>>();

    let subscription_id = Arc::new(AtomicU64::new(0));
    let (trigger, tripwire) = Tripwire::new();

    let subscription = Subscription { id: Arc::clone(&subscription_id), trigger };

    // Spawn a new thread to handle the stream and reconnections
    let client_clone = client.clone();
    let subscription_id_clone = Arc::clone(&subscription_id);
    client.runtime.spawn(async move {
        let mut backoff = Duration::from_secs(1);
        let max_backoff = Duration::from_secs(60);

        loop {
            let rcv = client_clone.inner.on_entity_updated(clauses.clone()).await;

            match rcv {
                Ok(rcv) => {
                    backoff = Duration::from_secs(1); // Reset backoff on successful connection

                    let mut rcv = rcv.take_until_if(tripwire.clone());

                    while let Some(Ok((id, entity))) = rcv.next().await {
                        subscription_id_clone.store(id, Ordering::SeqCst);
                        let key: types::FieldElement = (&entity.hashed_keys).into();
                        let models: Vec<Struct> =
                            entity.models.into_iter().map(|e| (&e).into()).collect();
                        callback(key, models.into());
                    }
                }
                Err(_) => {
                    // Check if the tripwire has been triggered before attempting to reconnect
                    if tripwire.clone().await {
                        break; // Exit the loop if the subscription has been cancelled
                    }
                }
            }

            // If we've reached this point, the stream has ended (possibly due to disconnection)
            // We'll try to reconnect after a delay, unless the tripwire has been triggered
            if tripwire.clone().await {
                break; // Exit the loop if the subscription has been cancelled
            }
            sleep(backoff).await;
            backoff = std::cmp::min(backoff * 2, max_backoff);
        }
    });

    Result::Ok(Box::into_raw(Box::new(subscription)))
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_update_entity_subscription(
    client: *mut ToriiClient,
    subscription: *mut Subscription,
    clauses: *const EntityKeysClause,
    clauses_len: usize,
) -> Result<bool> {
    let clauses = unsafe { std::slice::from_raw_parts(clauses, clauses_len) };
    let clauses = clauses.iter().map(|c| c.into()).collect::<Vec<_>>();

    match (*client).runtime.block_on(
        (*client)
            .inner
            .update_entity_subscription((*subscription).id.load(Ordering::SeqCst), clauses),
    ) {
        Ok(_) => Result::Ok(true),
        Err(e) => Result::Err(e.into()),
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_on_event_message_update(
    client: *mut ToriiClient,
    clauses: *const EntityKeysClause,
    clauses_len: usize,
    callback: unsafe extern "C" fn(types::FieldElement, CArray<Struct>),
) -> Result<*mut Subscription> {
    let client = Arc::from_raw(client);
    let clauses = unsafe { std::slice::from_raw_parts(clauses, clauses_len) };
    let clauses = clauses.iter().map(|c| c.into()).collect::<Vec<_>>();

    let subscription_id = Arc::new(AtomicU64::new(0));
    let (trigger, tripwire) = Tripwire::new();

    let subscription = Subscription { id: Arc::clone(&subscription_id), trigger };

    // Spawn a new thread to handle the stream and reconnections
    let client_clone = client.clone();
    let subscription_id_clone = Arc::clone(&subscription_id);
    client.runtime.spawn(async move {
        let mut backoff = Duration::from_secs(1);
        let max_backoff = Duration::from_secs(60);

        loop {
            let rcv = client_clone.inner.on_event_message_updated(clauses.clone()).await;

            match rcv {
                Ok(rcv) => {
                    backoff = Duration::from_secs(1); // Reset backoff on successful connection

                    let mut rcv = rcv.take_until_if(tripwire.clone());

                    while let Some(Ok((id, entity))) = rcv.next().await {
                        subscription_id_clone.store(id, Ordering::SeqCst);
                        let key: types::FieldElement = (&entity.hashed_keys).into();
                        let models: Vec<Struct> =
                            entity.models.into_iter().map(|e| (&e).into()).collect();
                        callback(key, models.into());
                    }
                }
                Err(_) => {
                    // Check if the tripwire has been triggered before attempting to reconnect
                    if tripwire.clone().await {
                        break; // Exit the loop if the subscription has been cancelled
                    }
                }
            }

            // If we've reached this point, the stream has ended (possibly due to disconnection)
            // We'll try to reconnect after a delay, unless the tripwire has been triggered
            if tripwire.clone().await {
                break; // Exit the loop if the subscription has been cancelled
            }
            sleep(backoff).await;
            backoff = std::cmp::min(backoff * 2, max_backoff);
        }
    });

    Result::Ok(Box::into_raw(Box::new(subscription)))
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_update_event_message_subscription(
    client: *mut ToriiClient,
    subscription: *mut Subscription,
    clauses: *const EntityKeysClause,
    clauses_len: usize,
) -> Result<bool> {
    let clauses = unsafe { std::slice::from_raw_parts(clauses, clauses_len) };
    let clauses = clauses.iter().map(|c| c.into()).collect::<Vec<_>>();

    match (*client).runtime.block_on(
        (*client)
            .inner
            .update_event_message_subscription((*subscription).id.load(Ordering::SeqCst), clauses),
    ) {
        Ok(_) => Result::Ok(true),
        Err(e) => Result::Err(e.into()),
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_on_starknet_event(
    client: *mut ToriiClient,
    clauses: *const EntityKeysClause,
    clauses_len: usize,
    callback: unsafe extern "C" fn(CArray<Event>),
) -> Result<*mut Subscription> {
    let client = Arc::from_raw(client);
    let clauses = unsafe { std::slice::from_raw_parts(clauses, clauses_len) };
    let clauses = clauses.iter().map(|c| c.into()).collect::<Vec<_>>();

    let subscription_id = Arc::new(AtomicU64::new(0));
    let (trigger, tripwire) = Tripwire::new();

    let subscription = Subscription { id: Arc::clone(&subscription_id), trigger };

    // Spawn a new thread to handle the stream and reconnections
    let client_clone = client.clone();
    client.runtime.spawn(async move {
        let mut backoff = Duration::from_secs(1);
        let max_backoff = Duration::from_secs(60);

        loop {
            let rcv = client_clone.inner.on_starknet_event(clauses.clone()).await;

            match rcv {
                Ok(rcv) => {
                    backoff = Duration::from_secs(1); // Reset backoff on successful connection

                    let mut rcv = rcv.take_until_if(tripwire.clone());

                    while let Some(Ok(event)) = rcv.next().await {
                        let events: Vec<Event> =
                            vec![event].into_iter().map(|e| (&e).into()).collect();
                        callback(events.into());
                    }
                }
                Err(_) => {
                    // Check if the tripwire has been triggered before attempting to reconnect
                    if tripwire.clone().await {
                        break; // Exit the loop if the subscription has been cancelled
                    }
                }
            }

            // If we've reached this point, the stream has ended (possibly due to disconnection)
            // We'll try to reconnect after a delay, unless the tripwire has been triggered
            if tripwire.clone().await {
                break; // Exit the loop if the subscription has been cancelled
            }
            sleep(backoff).await;
            backoff = std::cmp::min(backoff * 2, max_backoff);
        }
    });

    Result::Ok(Box::into_raw(Box::new(subscription)))
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn on_indexer_update(
    client: *mut ToriiClient,
    contract_address: *const types::FieldElement,
    callback: unsafe extern "C" fn(IndexerUpdate),
) -> Result<*mut Subscription> {
    let client = Arc::from_raw(client);
    let contract_address = if contract_address.is_null() {
        None
    } else {
        Some(unsafe { (&*contract_address).into() })
    };

    let subscription_id = Arc::new(AtomicU64::new(0));
    let (trigger, tripwire) = Tripwire::new();

    let subscription = Subscription { id: Arc::clone(&subscription_id), trigger };

    // Spawn a new thread to handle the stream and reconnections
    let client_clone = client.clone();
    client.runtime.spawn(async move {
        let mut backoff = Duration::from_secs(1);
        let max_backoff = Duration::from_secs(60);

        loop {
            let rcv = client_clone.inner.on_indexer_updated(contract_address).await;

            match rcv {
                Ok(rcv) => {
                    backoff = Duration::from_secs(1); // Reset backoff on successful connection

                    let mut rcv = rcv.take_until_if(tripwire.clone());

                    while let Some(Ok(update)) = rcv.next().await {
                        callback((&update).into());
                    }
                }
                Err(_) => {
                    // Check if the tripwire has been triggered before attempting to reconnect
                    if tripwire.clone().await {
                        break; // Exit the loop if the subscription has been cancelled
                    }
                }
            }

            // If we've reached this point, the stream has ended (possibly due to disconnection)
            // We'll try to reconnect after a delay, unless the tripwire has been triggered
            if tripwire.clone().await {
                break; // Exit the loop if the subscription has been cancelled
            }
            sleep(backoff).await;
            backoff = std::cmp::min(backoff * 2, max_backoff);
        }
    });

    Result::Ok(Box::into_raw(Box::new(subscription)))
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn bytearray_serialize(
    str: *const c_char,
) -> Result<CArray<types::FieldElement>> {
    let str = unsafe { CStr::from_ptr(str).to_string_lossy().into_owned() };
    let bytearray = match ByteArray::from_string(str.as_str()) {
        Ok(bytearray) => bytearray,
        Err(e) => return Result::Err(e.into()),
    };

    let felts = cairo_serde::ByteArray::cairo_serialize(&bytearray);
    let felts = felts.iter().map(|f| f.into()).collect::<Vec<types::FieldElement>>();
    Result::Ok(felts.into())
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn bytearray_deserialize(
    felts: *const types::FieldElement,
    felts_len: usize,
) -> Result<*const c_char> {
    let felts = unsafe { std::slice::from_raw_parts(felts, felts_len) };
    let felts = felts.iter().map(|f| (&f.clone()).into()).collect::<Vec<Felt>>();
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

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn poseidon_hash(
    felts: *const types::FieldElement,
    felts_len: usize,
) -> types::FieldElement {
    let felts = unsafe { std::slice::from_raw_parts(felts, felts_len) };
    let felts = felts.iter().map(|f| (&f.clone()).into()).collect::<Vec<Felt>>();

    (&poseidon_hash_many(&felts)).into()
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn get_selector_from_name(
    name: *const c_char,
) -> Result<types::FieldElement> {
    let name = unsafe { CStr::from_ptr(name).to_string_lossy().into_owned() };
    let selector = match starknet::core::utils::get_selector_from_name(name.as_str()) {
        Ok(selector) => selector,
        Err(e) => return Result::Err(e.into()),
    };

    Result::Ok((&selector).into())
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn get_selector_from_tag(tag: *const c_char) -> types::FieldElement {
    let tag = unsafe { CStr::from_ptr(tag).to_string_lossy().into_owned() };
    let selector = compute_selector_from_tag(tag.as_str());

    (&selector).into()
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn starknet_keccak(
    bytes: *const u8,
    bytes_len: usize,
) -> types::FieldElement {
    let bytes = unsafe { std::slice::from_raw_parts(bytes, bytes_len) };
    let hash = starknet::core::utils::starknet_keccak(bytes);

    (&hash).into()
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn cairo_short_string_to_felt(
    str: *const c_char,
) -> Result<types::FieldElement> {
    let str = unsafe { CStr::from_ptr(str).to_string_lossy().into_owned() };
    let felt = match starknet::core::utils::cairo_short_string_to_felt(str.as_str()) {
        Ok(felt) => felt,
        Err(e) => return Result::Err(e.into()),
    };

    Result::Ok((&felt).into())
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn parse_cairo_short_string(
    felt: types::FieldElement,
) -> Result<*const c_char> {
    let felt = (&felt).into();
    let str = match starknet::core::utils::parse_cairo_short_string(&felt) {
        Ok(str) => str,
        Err(e) => return Result::Err(e.into()),
    };

    Result::Ok(CString::new(str).unwrap().into_raw())
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
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

    let address = (&address).into();
    let encoded = match typed_data.encode(address) {
        Ok(encoded) => encoded,
        Err(err) => return Result::Err(err.into()),
    };

    Result::Ok((&encoded).into())
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn signing_key_new() -> types::FieldElement {
    let private_key = SigningKey::from_random();
    (&private_key.secret_scalar()).into()
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn signing_key_sign(
    private_key: types::FieldElement,
    hash: types::FieldElement,
) -> Result<Signature> {
    let private_key = SigningKey::from_secret_scalar((&private_key).into());
    let sig = private_key.sign(&(&hash).into());

    match sig {
        Ok(sig) => Result::Ok((&sig).into()),
        Err(e) => Result::Err(e.into()),
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn verifying_key_new(
    signing_key: types::FieldElement,
) -> types::FieldElement {
    let signing_key = (&signing_key).into();
    let verifying_key = starknet_crypto::get_public_key(&signing_key);

    (&verifying_key).into()
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn verifying_key_verify(
    verifying_key: types::FieldElement,
    hash: types::FieldElement,
    signature: types::Signature,
) -> Result<bool> {
    let verifying_key = VerifyingKey::from_scalar((&verifying_key).into());
    let signature = &(&signature).into();
    let hash = &(&hash).into();

    match verifying_key.verify(hash, signature) {
        Ok(result) => Result::Ok(result),
        Err(e) => Result::Err(e.into()),
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn provider_new(rpc_url: *const c_char) -> Result<*mut Provider> {
    let rpc_url = unsafe { CStr::from_ptr(rpc_url).to_string_lossy() };
    let rpc_url = match url::Url::parse(rpc_url.deref()) {
        Ok(url) => url,
        Err(e) => return Result::Err(e.into()),
    };

    let rpc = JsonRpcClient::new(HttpTransport::new(rpc_url));

    Result::Ok(Box::into_raw(Box::new(Provider(Arc::new(rpc)))))
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
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

    let chain_id = match tokio::runtime::Runtime::new() {
        Ok(runtime) => match runtime.block_on((*rpc).0.chain_id()) {
            Ok(chain_id) => chain_id,
            Err(e) => return Result::Err(e.into()),
        },
        Err(e) => return Result::Err(e.into()),
    };

    let signer =
        LocalWallet::from_signing_key(SigningKey::from_secret_scalar((&private_key).into()));
    let account = SingleOwnerAccount::new(
        (*rpc).0.clone(),
        signer,
        address,
        chain_id,
        ExecutionEncoding::New,
    );

    Result::Ok(Box::into_raw(Box::new(Account(account))))
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn starknet_call(
    provider: *mut Provider,
    call: Call,
    block_id: BlockId,
) -> Result<CArray<types::FieldElement>> {
    let res = match tokio::runtime::Runtime::new() {
        Ok(runtime) => match runtime.block_on(
            (*provider).0.call::<FunctionCall, starknet::core::types::BlockId>(
                (&call).into(),
                (&block_id).into(),
            ),
        ) {
            Ok(res) => res,
            Err(e) => return Result::Err(e.into()),
        },
        Err(e) => return Result::Err(e.into()),
    };

    let res: Vec<_> = res.iter().map(|f| f.into()).collect::<Vec<_>>();
    Result::Ok(res.into())
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn account_deploy_burner(
    provider: *mut Provider,
    master_account: *mut Account,
    signing_key: types::FieldElement,
) -> Result<*mut Account> {
    let signing_key = SigningKey::from_secret_scalar((&signing_key).into());
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
    let exec = (*master_account).0.execute_v1(vec![starknet::core::types::Call {
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

    let runtime = match tokio::runtime::Runtime::new() {
        Ok(runtime) => runtime,
        Err(e) => return Result::Err(e.into()),
    };

    let result = match runtime.block_on(exec.send()) {
        Ok(result) => result,
        Err(e) => return Result::Err(e.into()),
    };

    match runtime.block_on(watch_tx(&(*provider).0, result.transaction_hash)) {
        Ok(_) => Result::Ok(Box::into_raw(Box::new(Account(account)))),
        Err(e) => Result::Err(Error { message: CString::new(e.to_string()).unwrap().into_raw() }),
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn account_address(account: *mut Account) -> types::FieldElement {
    (&(*account).0.address()).into()
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn account_chain_id(account: *mut Account) -> types::FieldElement {
    (&(*account).0.chain_id()).into()
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn account_set_block_id(account: *mut Account, block_id: BlockId) {
    let block_id = (&block_id).into();
    (*account).0.set_block_id(block_id);
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn account_nonce(account: *mut Account) -> Result<types::FieldElement> {
    let nonce = match tokio::runtime::Runtime::new().unwrap().block_on((*account).0.get_nonce()) {
        Ok(nonce) => nonce,
        Err(e) => return Result::Err(e.into()),
    };

    Result::Ok((&nonce).into())
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn account_execute_raw(
    account: *mut Account,
    calldata: *const Call,
    calldata_len: usize,
) -> Result<types::FieldElement> {
    let calldata = unsafe { std::slice::from_raw_parts(calldata, calldata_len).to_vec() };
    let calldata =
        calldata.into_iter().map(|c| (&c).into()).collect::<Vec<starknet::core::types::Call>>();
    let call = (*account).0.execute_v1(calldata);

    match tokio::runtime::Runtime::new() {
        Ok(runtime) => match runtime.block_on(call.send()) {
            Ok(result) => Result::Ok((&result.transaction_hash).into()),
            Err(e) => Result::Err(e.into()),
        },
        Err(e) => Result::Err(e.into()),
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn wait_for_transaction(
    rpc: *mut Provider,
    txn_hash: types::FieldElement,
) -> Result<bool> {
    let txn_hash = (&txn_hash).into();
    match tokio::runtime::Runtime::new() {
        Ok(runtime) => match runtime.block_on(watch_tx(&(*rpc).0, txn_hash)) {
            Ok(_) => Result::Ok(true),
            Err(e) => {
                Result::Err(Error { message: CString::new(e.to_string()).unwrap().into_raw() })
            }
        },
        Err(e) => Result::Err(e.into()),
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn hash_get_contract_address(
    class_hash: types::FieldElement,
    salt: types::FieldElement,
    constructor_calldata: *const types::FieldElement,
    constructor_calldata_len: usize,
    deployer_address: types::FieldElement,
) -> types::FieldElement {
    let class_hash = (&class_hash).into();
    let salt = (&salt).into();
    let constructor_calldata = unsafe {
        std::slice::from_raw_parts(constructor_calldata, constructor_calldata_len).to_vec()
    };
    let constructor_calldata =
        constructor_calldata.iter().map(|f| (&f.clone()).into()).collect::<Vec<Felt>>();
    let deployer_address = (&deployer_address).into();

    let address = get_contract_address(salt, class_hash, &constructor_calldata, deployer_address);

    (&address).into()
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn subscription_cancel(subscription: *mut Subscription) {
    if !subscription.is_null() {
        unsafe {
            let subscription = Box::from_raw(subscription);
            subscription.trigger.cancel();
        }
    }
}

// This function takes a raw pointer to ToriiClient as an argument.
// It checks if the pointer is not null. If it's not, it converts the raw pointer
// back into a Box<ToriiClient>, which gets dropped at the end of the scope,
// deallocating the memory.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_free(t: *mut ToriiClient) {
    if !t.is_null() {
        unsafe {
            let client = Box::from_raw(t);
            client.runtime.shutdown_background();
        }
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn provider_free(rpc: *mut Provider) {
    if !rpc.is_null() {
        unsafe {
            let _ = Box::from_raw(rpc);
        }
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn model_free(model: *mut Struct) {
    if !model.is_null() {
        let _: dojo_types::schema::Struct = (&*Box::<Struct>::from_raw(model)).into();
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn account_free(account: *mut Account) {
    if !account.is_null() {
        unsafe {
            let _ = Box::from_raw(account);
        }
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn ty_free(ty: *mut Ty) {
    if !ty.is_null() {
        let _: dojo_types::schema::Ty = (&*Box::<Ty>::from_raw(ty)).into();
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn entity_free(entity: *mut Entity) {
    if !entity.is_null() {
        let _: torii_grpc::types::schema::Entity = (&*Box::<Entity>::from_raw(entity)).into();
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn error_free(error: *mut Error) {
    if !error.is_null() {
        let _: String = CString::from_raw((*error).message).into_string().unwrap();
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn world_metadata_free(metadata: *mut WorldMetadata) {
    if !metadata.is_null() {
        let _: dojo_types::WorldMetadata = (&*Box::<WorldMetadata>::from_raw(metadata)).into();
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn carray_free(data: *mut c_void, data_len: usize) {
    if !data.is_null() {
        let _: Vec<c_void> = Vec::from_raw_parts(data, data_len, data_len);
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn string_free(string: *mut c_char) {
    if !string.is_null() {
        let _: String = CString::from_raw(string).into_string().unwrap();
    }
}

// TODO: free keys clause
