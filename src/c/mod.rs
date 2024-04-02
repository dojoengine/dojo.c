mod types;

use self::types::{
    BlockId, CArray, COption, Call, Entity, Error, KeysClause, Model, Query, Result, Signature,
    ToriiClient, Ty, WorldMetadata,
};
use crate::constants;
use crate::types::{Account, Provider};
use crate::utils::watch_tx;
use starknet::accounts::{Account as StarknetAccount, ExecutionEncoding, SingleOwnerAccount};
use starknet::core::types::FunctionCall;
use starknet::core::utils::{
    cairo_short_string_to_felt, get_contract_address, get_selector_from_name,
};
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::{JsonRpcClient, Provider as _};
use starknet::signers::{LocalWallet, SigningKey, VerifyingKey};
use starknet_crypto::FieldElement;
use std::ffi::{c_void, CStr, CString};
use std::ops::Deref;
use std::os::raw::c_char;
use std::sync::Arc;
use tokio_stream::StreamExt;
use torii_client::client::Client as TClient;
use torii_relay::typed_data::TypedData;
use torii_relay::types::Message;

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_new(
    torii_url: *const c_char,
    rpc_url: *const c_char,
    libp2p_relay_url: *const c_char,
    world: *const c_char,
    // entities is optional
    entities: *const KeysClause,
    entities_len: usize,
) -> Result<*mut ToriiClient> {
    let torii_url = unsafe { CStr::from_ptr(torii_url).to_string_lossy().into_owned() };
    let rpc_url = unsafe { CStr::from_ptr(rpc_url).to_string_lossy().into_owned() };
    let libp2p_relay_url = unsafe {
        CStr::from_ptr(libp2p_relay_url)
            .to_string_lossy()
            .into_owned()
    };
    let world = unsafe { CStr::from_ptr(world).to_string_lossy().into_owned() };
    let some_entities = if entities.is_null() || entities_len == 0 {
        None
    } else {
        let entities = unsafe { std::slice::from_raw_parts(entities, entities_len) };
        let entities = entities.iter().map(|e| (&e.clone()).into()).collect();
        Some(entities)
    };

    let world = match FieldElement::from_hex_be(world.as_str()) {
        Ok(world) => world,
        Err(e) => return Result::Err(e.into()),
    };

    let client_future = TClient::new(torii_url, rpc_url, libp2p_relay_url, world, some_entities);

    let runtime = tokio::runtime::Runtime::new().unwrap();
    let mut client = match runtime.block_on(client_future) {
        Ok(client) => client,
        Err(e) => return Result::Err(e.into()),
    };

    // Run relay
    match runtime.block_on(client.wait_for_relay()) {
        Ok(_) => {}
        Err(e) => return Result::Err(e.into()),
    }

    // Start subscription
    let result = runtime.block_on(client.start_subscription());
    match result {
        Ok(sub) => {
            runtime.spawn(sub);
        }
        Err(e) => return Result::Err(e.into()),
    }

    Result::Ok(Box::into_raw(Box::new(ToriiClient {
        inner: client,
        runtime,
    })))
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
    keys: &KeysClause,
) -> Result<COption<*mut Ty>> {
    let keys = (&keys.clone()).into();
    let entity_future = unsafe { (*client).inner.model(&keys) };

    match (*client).runtime.block_on(entity_future) {
        Ok(ty) => {
            if let Some(ty) = ty {
                Result::Ok(COption::Some(Box::into_raw(Box::new((&ty).into()))))
            } else {
                Result::Ok(COption::None)
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
    let query = (&query.clone()).into();

    let entities_future = unsafe { (*client).inner.entities(query) };

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
pub unsafe extern "C" fn client_subscribed_models(client: *mut ToriiClient) -> CArray<KeysClause> {
    let entities = unsafe { (*client).inner.subscribed_models().clone() };
    let entities = entities
        .into_iter()
        .map(|e| (&e).into())
        .collect::<Vec<KeysClause>>();

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
    models: *const KeysClause,
    models_len: usize,
) -> Result<bool> {
    let models = unsafe { std::slice::from_raw_parts(models, models_len).to_vec() };

    let client_future = unsafe {
        (*client)
            .inner
            .add_models_to_sync(models.iter().map(|e| e.into()).collect())
    };

    match (*client).runtime.block_on(client_future) {
        Ok(_) => Result::Ok(true),
        Err(e) => Result::Err(e.into()),
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_on_sync_model_update(
    client: *mut ToriiClient,
    model: KeysClause,
    callback: unsafe extern "C" fn(),
) -> Result<bool> {
    let model: torii_grpc::types::KeysClause = (&model).into();
    let storage = (*client).inner.storage();

    let mut rcv = match storage.add_listener(
        cairo_short_string_to_felt(model.model.as_str()).unwrap(),
        model.keys.as_slice(),
    ) {
        Ok(rcv) => rcv,
        Err(e) => return Result::Err(e.into()),
    };

    (*client).runtime.spawn(async move {
        if let Ok(Some(_)) = rcv.try_next() {
            callback();
        }
    });

    Result::Ok(true)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_on_entity_state_update(
    client: *mut ToriiClient,
    entities: *mut types::FieldElement,
    entities_len: usize,
    callback: unsafe extern "C" fn(types::FieldElement, CArray<Model>),
) -> Result<bool> {
    let entities = unsafe { std::slice::from_raw_parts(entities, entities_len) };
    // to vec of fieldleemnt
    let entities = entities.iter().map(|e| (&e.clone()).into()).collect();

    let entity_stream = unsafe { (*client).inner.on_entity_updated(entities) };
    let mut rcv = match (*client).runtime.block_on(entity_stream) {
        Ok(rcv) => rcv,
        Err(e) => return Result::Err(e.into()),
    };

    (*client).runtime.spawn(async move {
        while let Some(Ok(entity)) = rcv.next().await {
            let key: types::FieldElement = (&entity.hashed_keys).into();
            let models: Vec<Model> = entity.models.into_iter().map(|e| (&e).into()).collect();
            callback(key, models.into());
        }
    });

    Result::Ok(true)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_remove_models_to_sync(
    client: *mut ToriiClient,
    models: *const KeysClause,
    models_len: usize,
) -> Result<bool> {
    let models = unsafe { std::slice::from_raw_parts(models, models_len).to_vec() };

    let client_future = unsafe {
        (*client)
            .inner
            .remove_models_to_sync(models.iter().map(|e| e.into()).collect())
    };

    match (*client).runtime.block_on(client_future) {
        Ok(_) => Result::Ok(true),
        Err(e) => Result::Err(e.into()),
    }
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
        Err(err) => return Result::Err(Error { message: CString::new(format!("Invalid typed data: {}", err)).unwrap().into_raw() }),
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
    let address = match FieldElement::from_hex_be(address.deref()) {
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
            (*provider)
                .0
                .call::<FunctionCall, starknet::core::types::BlockId>(
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
) -> Result<*mut Account> {
    let signing_key = SigningKey::from_random();
    let verifying_key = signing_key.verifying_key();
    let address = get_contract_address(
        verifying_key.scalar(),
        constants::KATANA_ACCOUNT_CLASS_HASH,
        &[verifying_key.scalar()],
        FieldElement::ZERO,
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
    let exec = (*master_account).0.execute(vec![starknet::accounts::Call {
        to: constants::UDC_ADDRESS,
        calldata: vec![
            constants::KATANA_ACCOUNT_CLASS_HASH, // class_hash
            verifying_key.scalar(),               // salt
            FieldElement::ZERO,                   // deployer_address
            FieldElement::ONE,                    // constructor calldata length (1)
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
        Err(e) => Result::Err(Error {
            message: CString::new(e.to_string()).unwrap().into_raw(),
        }),
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
    let calldata = calldata
        .into_iter()
        .map(|c| (&c).into())
        .collect::<Vec<starknet::accounts::Call>>();
    let call = (*account).0.execute(calldata);

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
            Err(e) => Result::Err(Error {
                message: CString::new(e.to_string()).unwrap().into_raw(),
            }),
        },
        Err(e) => Result::Err(e.into()),
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn hash_get_contract_address(
    class_hash: types::FieldElement,
    salt: types::FieldElement,
    constructor_calldata: *const FieldElement,
    constructor_calldata_len: usize,
    deployer_address: types::FieldElement,
) -> types::FieldElement {
    let class_hash = (&class_hash).into();
    let salt = (&salt).into();
    let constructor_calldata = unsafe {
        std::slice::from_raw_parts(constructor_calldata, constructor_calldata_len).to_vec()
    };
    let deployer_address = (&deployer_address).into();

    let address = get_contract_address(salt, class_hash, &constructor_calldata, deployer_address);

    (&address).into()
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
pub unsafe extern "C" fn model_free(model: *mut Model) {
    if !model.is_null() {
        let _: torii_grpc::types::schema::Model = (&*Box::<Model>::from_raw(model)).into();
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
