mod types;

use starknet::accounts::{Account as StarknetAccount, ExecutionEncoding, SingleOwnerAccount};
use starknet::core::utils::cairo_short_string_to_felt;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::{JsonRpcClient, Provider};
use starknet::signers::{LocalWallet, SigningKey};
use starknet_crypto::FieldElement;
use std::ffi::{c_void, CStr, CString};
use std::os::raw::c_char;
use std::thread;
use torii_client::client::Client as TClient;
use types::{
    Account, CArray, Call, Entity, Error, KeysClause, Query, ToriiClient, Ty, WorldMetadata,
};

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_new(
    torii_url: *const c_char,
    rpc_url: *const c_char,
    world: *const c_char,
    entities: *const KeysClause,
    entities_len: usize,
    error: *mut Error,
) -> *mut ToriiClient {
    let torii_url = unsafe { CStr::from_ptr(torii_url).to_string_lossy().into_owned() };
    let rpc_url = unsafe { CStr::from_ptr(rpc_url).to_string_lossy().into_owned() };
    let world = unsafe { CStr::from_ptr(world).to_string_lossy().into_owned() };
    let entities = unsafe { std::slice::from_raw_parts(entities, entities_len) };

    let world = FieldElement::from_hex_be(world.as_str());
    if let Err(e) = world {
        unsafe {
            *error = Error {
                message: CString::new(e.to_string()).unwrap().into_raw(),
            };
        }
        return std::ptr::null_mut();
    }
    let world = world.unwrap();

    let client_future = TClient::new(
        torii_url,
        rpc_url.clone(),
        world,
        Some(entities.iter().map(|e| (&e.clone()).into()).collect()),
    );

    let runtime = tokio::runtime::Runtime::new().unwrap();
    let client = runtime.block_on(client_future);

    match client {
        Ok(client) => Box::into_raw(Box::new(ToriiClient {
            inner: client,
            rpc_url,
            runtime,
        })),
        Err(e) => {
            unsafe {
                *error = Error {
                    message: CString::new(e.to_string()).unwrap().into_raw(),
                };
            }
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_entity(
    client: *mut ToriiClient,
    keys: &KeysClause,
    error: *mut Error,
) -> *mut Ty {
    let keys = (&keys.clone()).into();
    let entity_future = unsafe { (*client).inner.entity(&keys) };

    let result = (*client).runtime.block_on(entity_future);

    match result {
        Ok(ty) => {
            if let Some(ty) = ty {
                Box::into_raw(Box::new((&ty).into()))
            } else {
                std::ptr::null_mut()
            }
        }
        Err(e) => {
            unsafe {
                *error = Error {
                    message: CString::new(e.to_string()).unwrap().into_raw(),
                };
            }
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_entities(
    client: *mut ToriiClient,
    query: &Query,
    error: *mut Error,
) -> CArray<Entity> {
    let query = (&query.clone()).into();

    let entities_future = unsafe { (*client).inner.entities(query) };

    let result = (*client).runtime.block_on(entities_future);

    match result {
        Ok(entities) => {
            let entities: Vec<Entity> = entities.into_iter().map(|e| (&e).into()).collect();

            entities.into()
        }
        Err(e) => {
            unsafe {
                *error = Error {
                    message: CString::new(e.to_string()).unwrap().into_raw(),
                };
            }

            CArray {
                data: std::ptr::null_mut(),
                data_len: 0,
            }
        }
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
pub unsafe extern "C" fn client_start_subscription(client: *mut ToriiClient, error: *mut Error) {
    let client_future = unsafe { (*client).inner.start_subscription() };
    let result = (*client).runtime.block_on(client_future);

    if let Err(e) = result {
        unsafe {
            *error = Error {
                message: CString::new(e.to_string()).unwrap().into_raw(),
            };
        }

        return;
    }

    (*client).runtime.spawn(result.unwrap());
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
    error: *mut Error,
) {
    let entities = unsafe { std::slice::from_raw_parts(entities, entities_len).to_vec() };

    let client_future = unsafe {
        (*client)
            .inner
            .add_entities_to_sync(entities.iter().map(|e| e.into()).collect())
    };

    let result = (*client).runtime.block_on(client_future);

    if let Err(e) = result {
        unsafe {
            *error = Error {
                message: CString::new(e.to_string()).unwrap().into_raw(),
            };
        }
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
    error: *mut Error,
) {
    let entities = unsafe { std::slice::from_raw_parts(entities, entities_len).to_vec() };

    let client_future = unsafe {
        (*client)
            .inner
            .remove_entities_to_sync(entities.iter().map(|e| e.into()).collect())
    };

    let result = (*client).runtime.block_on(client_future);

    if let Err(e) = result {
        unsafe {
            *error = Error {
                message: CString::new(e.to_string()).unwrap().into_raw(),
            };
        }
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn account_new(
    client: *mut ToriiClient,
    private_key: *const c_char,
    address: *const c_char,
    error: *mut Error,
) -> *mut Account {
    let address = unsafe { CStr::from_ptr(address).to_string_lossy().into_owned() };
    let address = FieldElement::from_hex_be(address.as_str());
    if let Err(e) = address {
        unsafe {
            *error = Error {
                message: CString::new(e.to_string()).unwrap().into_raw(),
            };
        }
        return std::ptr::null_mut();
    }
    let address = address.unwrap();

    let rpc = JsonRpcClient::new(HttpTransport::new(
        url::Url::parse(&unsafe { (*client).rpc_url.clone() }).unwrap(),
    ));

    let chain_id = (*client).runtime.block_on(rpc.chain_id()).unwrap();

    let signer = LocalWallet::from_signing_key(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be(
            unsafe { CStr::from_ptr(private_key).to_string_lossy().into_owned() }.as_str(),
        )
        .unwrap(),
    ));
    let account =
        SingleOwnerAccount::new(rpc, signer, address, chain_id, ExecutionEncoding::Legacy);

    Box::into_raw(Box::new(Account(account)))
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
pub unsafe extern "C" fn account_execute_raw(
    account: *mut Account,
    calldata: *const Call,
    calldata_len: usize,
    error: *mut Error,
) {
    let calldata = unsafe { std::slice::from_raw_parts(calldata, calldata_len).to_vec() };
    let calldata = calldata
        .into_iter()
        .map(|c| (&c).into())
        .collect::<Vec<starknet::accounts::Call>>();

    let call = (*account).0.execute(calldata);

    let result = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(call.send());

    if let Err(e) = result {
        unsafe {
            *error = Error {
                message: CString::new(e.to_string()).unwrap().into_raw(),
            };
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
            let _ = Box::from_raw(t);
        }
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
        let _: torii_grpc::types::Entity = (&*Box::<Entity>::from_raw(entity)).into();
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
