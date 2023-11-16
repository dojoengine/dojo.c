mod storage;
mod types;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use torii_client::client::Client as TClient;
use types::{CArray, EntityQuery, Error, FieldElement, ToriiClient, Ty, WorldMetadata};

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_new(
    torii_url: *const c_char,
    rpc_url: *const c_char,
    world: &FieldElement,
    entities: *const EntityQuery,
    entities_len: usize,
    error: *mut Error,
) -> *mut ToriiClient {
    let torii_url = unsafe { CStr::from_ptr(torii_url).to_string_lossy().into_owned() };
    let rpc_url = unsafe { CStr::from_ptr(rpc_url).to_string_lossy().into_owned() };
    let entities = unsafe { std::slice::from_raw_parts(entities, entities_len).to_vec() };

    let client_future = TClient::new(
        torii_url,
        rpc_url,
        world.into(),
        Some(entities.iter().map(|e| e.into()).collect()),
    );

    let client = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(client_future);

    match client {
        Ok(client) => Box::into_raw(Box::new(ToriiClient(client))),
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
    entity: &EntityQuery,
    error: *mut Error,
) -> *mut Ty {
    let entity: dojo_types::schema::EntityQuery = (&entity.clone()).into();
    let entity_future = unsafe { (*client).0.entity(&entity) };

    let result = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(entity_future);

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
pub unsafe extern "C" fn client_metadata(client: *mut ToriiClient) -> WorldMetadata {
    unsafe { (&(*client).0.metadata().clone()).into() }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_subscribed_entities(
    client: *mut ToriiClient,
) -> *const CArray<EntityQuery> {
    let entities = unsafe { (*client).0.subscribed_entities().clone() };
    let entities: Vec<EntityQuery> = entities.into_iter().map(|e| (&e).into()).collect();

    &CArray {
        data: entities.as_ptr(),
        data_len: entities.len(),
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_start_subscription(client: *mut ToriiClient, error: *mut Error) {
    let client_future = unsafe { (*client).0.start_subscription() };

    let result = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(client_future);

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
pub unsafe extern "C" fn client_add_entities_to_sync(
    client: *mut ToriiClient,
    entities: *const EntityQuery,
    entities_len: usize,
    error: *mut Error,
) {
    let entities = unsafe { std::slice::from_raw_parts(entities, entities_len).to_vec() };

    let client_future = unsafe {
        (*client)
            .0
            .add_entities_to_sync(entities.iter().map(|e| e.into()).collect())
    };

    let result = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(client_future);

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
pub unsafe extern "C" fn client_remove_entities_to_sync(
    client: *mut ToriiClient,
    entities: *const EntityQuery,
    entities_len: usize,
    error: *mut Error,
) {
    let entities = unsafe { std::slice::from_raw_parts(entities, entities_len).to_vec() };

    let client_future = unsafe {
        (*client)
            .0
            .remove_entities_to_sync(entities.iter().map(|e| e.into()).collect())
    };

    let result = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(client_future);

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
