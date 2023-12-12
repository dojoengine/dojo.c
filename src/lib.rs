mod types;

use starknet::accounts::{Account as StarknetAccount, ExecutionEncoding, SingleOwnerAccount};
use starknet::core::utils::cairo_short_string_to_felt;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::{JsonRpcClient, Provider};
use starknet::signers::{LocalWallet, SigningKey, VerifyingKey};
use starknet_crypto::FieldElement;
use std::ffi::{c_void, CStr, CString};
use std::ops::Deref;
use std::os::raw::c_char;
use std::thread;
use torii_client::client::Client as TClient;
use types::{
    Account, BlockId, CArray, CJsonRpcClient, COption, Call, Entity, Error, KeysClause, Query,
    Result, Signature, ToriiClient, Ty, WorldMetadata,
};

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_new(
    torii_url: *const c_char,
    rpc_url: *const c_char,
    world: *const c_char,
    // entities is optional
    entities: *const KeysClause,
    entities_len: usize,
) -> Result<*mut ToriiClient> {
    let torii_url = unsafe { CStr::from_ptr(torii_url).to_string_lossy().into_owned() };
    let rpc_url = unsafe { CStr::from_ptr(rpc_url).to_string_lossy().into_owned() };
    let world = unsafe { CStr::from_ptr(world).to_string_lossy().into_owned() };
    let some_entities = if entities.is_null() || entities_len == 0 {
        None
    } else {
        let entities = unsafe { std::slice::from_raw_parts(entities, entities_len) };
        let entities = entities.iter().map(|e| (&e.clone()).into()).collect();
        Some(entities)
    };

    let world = FieldElement::from_hex_be(world.as_str());

    if let Err(e) = world {
        return Result::Err(Error {
            message: CString::new(e.to_string()).unwrap().into_raw(),
        });
    }
    let world = world.unwrap();

    let client_future = TClient::new(torii_url, rpc_url, world, some_entities);

    let runtime = tokio::runtime::Runtime::new().unwrap();
    let client = runtime.block_on(client_future);

    match client {
        Ok(client) => Result::Ok(Box::into_raw(Box::new(ToriiClient {
            inner: client,
            runtime,
        }))),
        Err(e) => Result::Err(Error {
            message: CString::new(e.to_string()).unwrap().into_raw(),
        }),
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_entity(
    client: *mut ToriiClient,
    keys: &KeysClause,
) -> Result<COption<*mut Ty>> {
    let keys = (&keys.clone()).into();
    let entity_future = unsafe { (*client).inner.entity(&keys) };

    let result = (*client).runtime.block_on(entity_future);

    match result {
        Ok(ty) => {
            if let Some(ty) = ty {
                Result::Ok(COption::Some(Box::into_raw(Box::new((&ty).into()))))
            } else {
                Result::Ok(COption::None)
            }
        }
        Err(e) => Result::Err(Error {
            message: CString::new(e.to_string()).unwrap().into_raw(),
        }),
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

    let result = (*client).runtime.block_on(entities_future);

    match result {
        Ok(entities) => {
            let entities: Vec<Entity> = entities.into_iter().map(|e| (&e).into()).collect();

            Result::Ok(entities.into())
        }
        Err(e) => Result::Err(Error {
            message: CString::new(e.to_string()).unwrap().into_raw(),
        }),
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_subscribed_entities(
    client: *mut ToriiClient,
) -> CArray<KeysClause> {
    let entities = unsafe { (*client).inner.subscribed_entities().clone() };
    let entities = entities
        .into_iter()
        .map(|e| (&e).into())
        .collect::<Vec<KeysClause>>();

    entities.into()
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_start_subscription(client: *mut ToriiClient) -> Result<bool> {
    let client_future = unsafe { (*client).inner.start_subscription() };
    let result = (*client).runtime.block_on(client_future);

    match result {
        Ok(sub) => {
            (*client).runtime.spawn(sub);
            Result::Ok(true)
        }
        Err(e) => Result::Err(Error {
            message: CString::new(e.to_string()).unwrap().into_raw(),
        }),
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_metadata(client: *mut ToriiClient) -> WorldMetadata {
    unsafe { (&(*client).inner.metadata().clone()).into() }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_add_entities_to_sync(
    client: *mut ToriiClient,
    entities: *const KeysClause,
    entities_len: usize,
) -> Result<bool> {
    let entities = unsafe { std::slice::from_raw_parts(entities, entities_len).to_vec() };

    let client_future = unsafe {
        (*client)
            .inner
            .add_entities_to_sync(entities.iter().map(|e| e.into()).collect())
    };

    let result = (*client).runtime.block_on(client_future);

    match result {
        Ok(_) => Result::Ok(true),
        Err(e) => Result::Err(Error {
            message: CString::new(e.to_string()).unwrap().into_raw(),
        }),
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_on_entity_state_update(
    client: *mut ToriiClient,
    entity: &KeysClause,
    callback: unsafe extern "C" fn(),
) {
    let entity: torii_grpc::types::KeysClause = entity.into();
    let model = cairo_short_string_to_felt(&entity.model).unwrap();
    let mut rcv = (*client)
        .inner
        .storage()
        .add_listener(model, entity.keys.as_slice())
        .unwrap();

    thread::spawn(move || loop {
        if let Ok(Some(_)) = rcv.try_next() {
            callback();
        }
    });
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_remove_entities_to_sync(
    client: *mut ToriiClient,
    entities: *const KeysClause,
    entities_len: usize,
) -> Result<bool> {
    let entities = unsafe { std::slice::from_raw_parts(entities, entities_len).to_vec() };

    let client_future = unsafe {
        (*client)
            .inner
            .remove_entities_to_sync(entities.iter().map(|e| e.into()).collect())
    };

    let result = (*client).runtime.block_on(client_future);

    match result {
        Ok(_) => Result::Ok(true),
        Err(e) => Result::Err(Error {
            message: CString::new(e.to_string()).unwrap().into_raw(),
        }),
    }
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
        Err(e) => Result::Err(Error {
            message: CString::new(e.to_string()).unwrap().into_raw(),
        }),
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn felt_from_hex_be(hex: *const c_char) -> Result<types::FieldElement> {
    let hex = unsafe { CStr::from_ptr(hex).to_string_lossy() };
    let hex = FieldElement::from_hex_be(hex.deref());

    match hex {
        Ok(hex) => Result::Ok((&hex).into()),
        Err(e) => Result::Err(Error {
            message: CString::new(e.to_string()).unwrap().into_raw(),
        }),
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
        Err(e) => Result::Err(Error {
            message: CString::new(e.to_string()).unwrap().into_raw(),
        }),
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn jsonrpc_client_new(rpc_url: *const c_char) -> Result<*mut CJsonRpcClient> {
    let rpc_url = unsafe { CStr::from_ptr(rpc_url).to_string_lossy() };
    let rpc_url = url::Url::parse(rpc_url.deref());
    if let Err(e) = rpc_url {
        return Result::Err(Error {
            message: CString::new(e.to_string()).unwrap().into_raw(),
        });
    }
    let rpc_url = rpc_url.unwrap();

    let rpc = JsonRpcClient::new(HttpTransport::new(rpc_url));

    Result::Ok(Box::into_raw(Box::new(CJsonRpcClient(rpc))))
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn account_new(
    rpc: *mut CJsonRpcClient,
    private_key: types::FieldElement,
    address: *const c_char,
) -> Result<*mut Account<'static>> {
    let address = unsafe { CStr::from_ptr(address).to_string_lossy() };
    let address = FieldElement::from_hex_be(address.deref());
    if let Err(e) = address {
        return Result::Err(Error {
            message: CString::new(e.to_string()).unwrap().into_raw(),
        });
    }

    let address = address.unwrap();

    let chain_id = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on((*rpc).0.chain_id())
        .unwrap();

    let signer =
        LocalWallet::from_signing_key(SigningKey::from_secret_scalar((&private_key).into()));
    let account = SingleOwnerAccount::new(
        &(*rpc).0,
        signer,
        address,
        chain_id,
        ExecutionEncoding::Legacy,
    );

    Result::Ok(Box::into_raw(Box::new(Account(account))))
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn account_address(account: *mut Account<'static>) -> types::FieldElement {
    (&(*account).0.address()).into()
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn account_chain_id(account: *mut Account<'static>) -> types::FieldElement {
    (&(*account).0.chain_id()).into()
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn account_set_block_id(account: *mut Account<'static>, block_id: BlockId) {
    let block_id = (&block_id).into();
    (*account).0.set_block_id(block_id);
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn account_execute_raw(
    account: *mut Account<'static>,
    calldata: *const Call,
    calldata_len: usize,
) -> Result<bool> {
    let calldata = unsafe { std::slice::from_raw_parts(calldata, calldata_len).to_vec() };
    let calldata = calldata
        .into_iter()
        .map(|c| (&c).into())
        .collect::<Vec<starknet::accounts::Call>>();

    let call = (*account).0.execute(calldata);

    let result = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(call.send());

    match result {
        Ok(_) => Result::Ok(true),
        Err(e) => Result::Err(Error {
            message: CString::new(e.to_string()).unwrap().into_raw(),
        }),
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
            let _ = Box::from_raw(t);
        }
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn account_free(account: *mut Account<'static>) {
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
        let _: String = CString::from_raw((*error).message as *mut i8)
            .into_string()
            .unwrap();
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
