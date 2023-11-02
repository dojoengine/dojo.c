use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use torii_client::client::Client;

pub struct ToriiClient(Client);

#[repr(C)]
pub struct Error {
    message: *const c_char,
}

#[derive(Clone)]
#[repr(C)]
pub struct FieldElement {
    data: [u8; 32],
}

impl From<&FieldElement> for starknet::core::types::FieldElement {
    fn from(val: &FieldElement) -> Self {
        starknet::core::types::FieldElement::from_bytes_be(&val.data).unwrap()
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct EntityModel {
    pub model: *const c_char,
    pub keys: *const FieldElement,
    pub keys_len: usize,
}

impl From<&EntityModel> for dojo_types::schema::EntityModel {
    fn from(val: &EntityModel) -> Self {
        let model = unsafe { CStr::from_ptr(val.model).to_string_lossy().into_owned() };
        let keys = unsafe { std::slice::from_raw_parts(val.keys, val.keys_len).to_vec() };

        dojo_types::schema::EntityModel {
            model,
            keys: keys.iter().map(|e| e.into()).collect(),
        }
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_new(
    torii_url: *const c_char,
    rpc_url: *const c_char,
    world: &FieldElement,
    entities: *const EntityModel,
    entities_len: usize,
    error: *mut Error,
) -> *mut ToriiClient {
    let torii_url = unsafe { CStr::from_ptr(torii_url).to_string_lossy().into_owned() };
    let rpc_url = unsafe { CStr::from_ptr(rpc_url).to_string_lossy().into_owned() };
    let entities = unsafe { std::slice::from_raw_parts(entities, entities_len).to_vec() };

    let client_future = Client::new(
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
pub unsafe extern "C" fn client_add_entities_to_sync(
    client: *mut ToriiClient,
    entities: *const EntityModel,
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
    entities: *const EntityModel,
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
