mod types;

use std::ffi::{c_void, CStr, CString};
use std::ops::Deref;
use std::os::raw::c_char;
use std::sync::Arc;

use cainome::cairo_serde::{self, ByteArray, CairoSerde};
use starknet::accounts::{Account as StarknetAccount, ExecutionEncoding, SingleOwnerAccount};
use starknet::core::types::FunctionCall;
use starknet::core::utils::{
    cairo_short_string_to_felt, get_contract_address, get_selector_from_name,
};
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::{JsonRpcClient, Provider as _};
use starknet::signers::{LocalWallet, SigningKey, VerifyingKey};
use starknet_crypto::{poseidon_hash_many, Felt};
use stream_cancel::{StreamExt as _, Tripwire};
use tokio_stream::StreamExt;
use torii_client::client::Client as TClient;
use torii_relay::typed_data::TypedData;
use torii_relay::types::Message;
use types::{EntityKeysClause, ModelKeysClause, Struct};

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

    // Start subscription
    let result = runtime.block_on(client.start_subscription());
    match result {
        Ok(sub) => {
            runtime.spawn(sub);
        }
        Err(e) => return Result::Err(e.into()),
    }

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
    signature: types::Signature,
) -> Result<CArray<u8>> {
    let message = unsafe { CStr::from_ptr(message).to_string_lossy().into_owned() };
    let message = match serde_json::from_str::<TypedData>(message.as_str()) {
        Ok(message) => message,
        Err(e) => return Result::Err(e.into()),
    };

    let client_future = unsafe {
        (*client).inner.publish_message(Message {
            message,
            signature_r: (&signature.r).into(),
            signature_s: (&signature.s).into(),
        })
    };

    match (*client).runtime.block_on(client_future) {
        Ok(data) => Result::Ok(data.into()),
        Err(e) => Result::Err(e.into()),
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_model(
    client: *mut ToriiClient,
    keys: &ModelKeysClause,
) -> Result<*mut Ty> {
    let keys = (&keys.clone()).into();
    let entity_future = unsafe { (*client).inner.model(&keys) };

    match (*client).runtime.block_on(entity_future) {
        Ok(ty) => {
            if let Some(ty) = ty {
                Result::Ok(Box::into_raw(Box::new((&ty).into())))
            } else {
                Result::Ok(std::ptr::null_mut())
            }
        }
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
pub unsafe extern "C" fn client_subscribed_models(
    client: *mut ToriiClient,
) -> CArray<ModelKeysClause> {
    let entities = unsafe { (*client).inner.subscribed_models().clone() };
    let entities = entities.into_iter().map(|e| (&e).into()).collect::<Vec<ModelKeysClause>>();

    entities.into()
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_metadata(client: *mut ToriiClient) -> WorldMetadata {
    unsafe { (&(*client).inner.metadata().clone()).into() }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_add_models_to_sync(
    client: *mut ToriiClient,
    models: *const ModelKeysClause,
    models_len: usize,
) -> Result<bool> {
    let models = unsafe { std::slice::from_raw_parts(models, models_len).to_vec() };

    let client_future =
        unsafe { (*client).inner.add_models_to_sync(models.iter().map(|e| e.into()).collect()) };

    match (*client).runtime.block_on(client_future) {
        Ok(_) => Result::Ok(true),
        Err(e) => Result::Err(e.into()),
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_on_sync_model_update(
    client: *mut ToriiClient,
    model: ModelKeysClause,
    callback: unsafe extern "C" fn(),
) -> Result<*mut Subscription> {
    let model: torii_grpc::types::ModelKeysClause = (&model).into();
    let storage = (*client).inner.storage();

    let rcv = match storage.add_listener(
        cairo_short_string_to_felt(model.model.as_str()).unwrap(),
        model.keys.as_slice(),
    ) {
        Ok(rcv) => rcv,
        Err(e) => return Result::Err(e.into()),
    };

    let (trigger, tripwire) = Tripwire::new();
    (*client).runtime.spawn(async move {
        let mut rcv = rcv.take_until_if(tripwire);

        while rcv.next().await.is_some() {
            callback();
        }
    });

    Result::Ok(Box::into_raw(Box::new(Subscription { id: 0, trigger })))
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_on_entity_state_update(
    client: *mut ToriiClient,
    clauses: *const EntityKeysClause,
    clauses_len: usize,
    callback: unsafe extern "C" fn(types::FieldElement, CArray<Struct>),
) -> Result<*mut Subscription> {
    let clauses = unsafe { std::slice::from_raw_parts(clauses, clauses_len) };
    let clauses = clauses.iter().map(|c| c.into()).collect::<Vec<_>>();

    let entity_stream = unsafe { (*client).inner.on_entity_updated(clauses) };
    let mut rcv = match (*client).runtime.block_on(entity_stream) {
        Ok(rcv) => rcv,
        Err(e) => return Result::Err(e.into()),
    };

    let subscription_id = match (*client).runtime.block_on(rcv.next()) {
        Some(Ok((subscription_id, _))) => subscription_id,
        _ => {
            return Result::Err(Error {
                message: CString::new("failed to get subscription id").unwrap().into_raw(),
            });
        }
    };

    let (trigger, tripwire) = Tripwire::new();
    (*client).runtime.spawn(async move {
        let mut rcv = rcv.take_until_if(tripwire);

        while let Some(Ok((_, entity))) = rcv.next().await {
            let key: types::FieldElement = (&entity.hashed_keys).into();
            let models: Vec<Struct> = entity.models.into_iter().map(|e| (&e).into()).collect();
            callback(key, models.into());
        }
    });

    Result::Ok(Box::into_raw(Box::new(Subscription { id: subscription_id, trigger })))
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

    match (*client)
        .runtime
        .block_on((*client).inner.update_entity_subscription((*subscription).id, clauses))
    {
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
    let clauses = unsafe { std::slice::from_raw_parts(clauses, clauses_len) };
    let clauses = clauses.iter().map(|c| c.into()).collect::<Vec<_>>();

    let entity_stream = unsafe { (*client).inner.on_event_message_updated(clauses) };
    let mut rcv = match (*client).runtime.block_on(entity_stream) {
        Ok(rcv) => rcv,
        Err(e) => return Result::Err(e.into()),
    };

    let subscription_id = match (*client).runtime.block_on(rcv.next()) {
        Some(Ok((subscription_id, _))) => subscription_id,
        _ => {
            return Result::Err(Error {
                message: CString::new("faild to get subscription id").unwrap().into_raw(),
            });
        }
    };

    let (trigger, tripwire) = Tripwire::new();
    (*client).runtime.spawn(async move {
        let mut rcv = rcv.take_until_if(tripwire);

        while let Some(Ok((_, entity))) = rcv.next().await {
            let key: types::FieldElement = (&entity.hashed_keys).into();
            let models: Vec<Struct> = entity.models.into_iter().map(|e| (&e).into()).collect();
            callback(key, models.into());
        }
    });

    Result::Ok(Box::into_raw(Box::new(Subscription { id: subscription_id, trigger })))
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

    match (*client)
        .runtime
        .block_on((*client).inner.update_event_message_subscription((*subscription).id, clauses))
    {
        Ok(_) => Result::Ok(true),
        Err(e) => Result::Err(e.into()),
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_remove_models_to_sync(
    client: *mut ToriiClient,
    models: *const ModelKeysClause,
    models_len: usize,
) -> Result<bool> {
    let models = unsafe { std::slice::from_raw_parts(models, models_len).to_vec() };

    let client_future =
        unsafe { (*client).inner.remove_models_to_sync(models.iter().map(|e| e.into()).collect()) };

    match (*client).runtime.block_on(client_future) {
        Ok(_) => Result::Ok(true),
        Err(e) => Result::Err(e.into()),
    }
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
    let exec = (*master_account).0.execute_v3(vec![starknet::accounts::Call {
        to: constants::UDC_ADDRESS,
        calldata: vec![
            constants::KATANA_ACCOUNT_CLASS_HASH, // class_hash
            verifying_key.scalar(),               // salt
            Felt::ZERO,                           // deployer_address
            Felt::ONE,                            // constructor calldata length (1)
            verifying_key.scalar(),               // constructor calldata
        ],
        selector: get_selector_from_name("deployContract").unwrap(),
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
pub unsafe extern "C" fn account_execute_raw(
    account: *mut Account,
    calldata: *const Call,
    calldata_len: usize,
) -> Result<types::FieldElement> {
    let calldata = unsafe { std::slice::from_raw_parts(calldata, calldata_len).to_vec() };
    let calldata =
        calldata.into_iter().map(|c| (&c).into()).collect::<Vec<starknet::accounts::Call>>();
    let call = (*account).0.execute_v3(calldata);

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
