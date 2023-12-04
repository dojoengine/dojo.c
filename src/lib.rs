mod types;

use starknet::core::utils::cairo_short_string_to_felt;
use starknet_crypto::FieldElement;
use std::ffi::{CStr, CString, c_void};
use std::os::raw::c_char;
use std::thread;
use torii_client::client::Client as TClient;
use types::{CArray, Entity, Error, KeysClause, Query, ToriiClient, Ty, WorldMetadata};

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
        rpc_url,
        world,
        Some(entities.iter().map(|e| (&e.clone()).into()).collect()),
    );

    let runtime = tokio::runtime::Runtime::new().unwrap();
    let client = runtime.block_on(client_future);

    match client {
        Ok(client) => Box::into_raw(Box::new(ToriiClient {
            inner: client,
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
    println!("{:?}", *keys.keys.data);
    let keys = (&keys.clone()).into();
    let entity_future = unsafe { (*client).inner.entity(&keys) };

    let result = (*client).runtime.block_on(entity_future);

    match result {
        Ok(ty) => {
            if let Some(ty) = ty {
                println!("{:?}", ty);
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
) -> *const CArray<Entity> {
    let query = (&query.clone()).into();
    println!("{:?}", query);

    let entities_future = unsafe { (*client).inner.entities(query) };

    let result = (*client).runtime.block_on(entities_future);

    match result {
        Ok(entities) => {
            let entities: Vec<Entity> = entities.into_iter().map(|e| (&e.clone()).into()).collect();

            Box::into_raw(Box::new(entities.into()))
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
pub unsafe extern "C" fn client_subscribed_entities(
    client: *mut ToriiClient,
) -> *const CArray<KeysClause> {
    let entities = unsafe { (*client).inner.subscribed_entities().clone() };
    let entities = entities
        .into_iter()
        .map(|e| (&e).into())
        .collect::<Vec<KeysClause>>();

    Box::into_raw(Box::new(entities.into()))
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
            println!("Received update");
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

// This function takes a raw pointer to ToriiClient as an argument.
// It checks if the pointer is not null. If it's not, it converts the raw pointer
// back into a Box<ToriiClient>, which gets dropped at the end of the scope,
// deallocating the memory.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_free(client: *mut ToriiClient) {
    if !client.is_null() {
        unsafe {
            let _ = Box::from_raw(client);
        }
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn carray_free(array: *mut c_void, len: usize) {
    if !array.is_null() {
        let _ = Vec::from_raw_parts(array as *mut c_void, len, len);
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn ty_free(ty: *mut Ty) {
    if !ty.is_null() {
        let _: dojo_types::schema::Ty = (&*Box::<Ty>::from_raw(ty)).into();
    }
}
