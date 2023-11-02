use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use torii_client::client::Client;

#[repr(C)]
pub struct Error {
    message: *const c_char,
}

#[derive(Clone)]
#[repr(C)]
pub struct FieldElement {
    data: [u8; 32],
}

impl Into<starknet::core::types::FieldElement> for &FieldElement {
    fn into(self) -> starknet::core::types::FieldElement {
        starknet::core::types::FieldElement::from_bytes_be(&self.data).unwrap()
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct EntityModel {
    pub model: *const c_char,
    pub keys: *const FieldElement,
    pub keys_len: usize,
}

impl Into<dojo_types::schema::EntityModel> for &EntityModel {
    fn into(self) -> dojo_types::schema::EntityModel {
        let model = unsafe { CStr::from_ptr(self.model).to_string_lossy().into_owned() };
        let keys = unsafe { std::slice::from_raw_parts(self.keys, self.keys_len).to_vec() };

        dojo_types::schema::EntityModel {
            model,
            keys: keys.iter().map(|e| e.into()).collect(),
        }
    }
}

#[no_mangle]
pub extern "C" fn client_new(
    torii_url: *const c_char,
    rpc_url: *const c_char,
    world: &FieldElement,
    entities: *const EntityModel,
    entities_len: usize,
    error: *mut Error,
) -> *mut Client {
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
        Ok(client) => Box::into_raw(Box::new(client)),
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
