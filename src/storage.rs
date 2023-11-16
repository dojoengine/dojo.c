use std::ffi::CString;

use torii_client::client::storage::ModelStorage;

use crate::types::{CArray, Error, FieldElement};

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn storage_get_entity_storage(
    storage: *mut ModelStorage,
    model: FieldElement,
    raw_keys: CArray<FieldElement>,
    error: *mut Error,
) -> *mut CArray<FieldElement> {
    let raw_keys: Vec<starknet_crypto::FieldElement> = unsafe {
        std::slice::from_raw_parts(raw_keys.data, raw_keys.data_len)
            .to_vec()
            .iter()
            .map(|f| (&f.clone()).into())
            .collect()
    };
    let result = unsafe { (*storage).get_entity_storage((&model).into(), raw_keys.as_slice()) };

    match result {
        Ok(ty) => {
            if let Some(ty) = ty {
                let ty = ty
                    .iter()
                    .map(|f| (&f.clone()).into())
                    .collect::<Vec<FieldElement>>();
                Box::into_raw(Box::new(CArray {
                    data: ty.as_ptr(),
                    data_len: ty.len(),
                }))
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
pub unsafe extern "C" fn storage_set_entity_storage(
    storage: *mut ModelStorage,
    model: FieldElement,
    raw_keys: CArray<FieldElement>,
    raw_values: CArray<FieldElement>,
    error: *mut Error,
) {
    let raw_keys: Vec<starknet_crypto::FieldElement> = unsafe {
        std::slice::from_raw_parts(raw_keys.data, raw_keys.data_len)
            .to_vec()
            .iter()
            .map(|f| (&f.clone()).into())
            .collect()
    };
    let raw_values: Vec<starknet_crypto::FieldElement> = unsafe {
        std::slice::from_raw_parts(raw_values.data, raw_values.data_len)
            .to_vec()
            .iter()
            .map(|f| (&f.clone()).into())
            .collect()
    };
    let result = unsafe { (*storage).set_entity_storage((&model).into(), raw_keys, raw_values) };

    match result {
        Ok(_) => {}
        Err(e) => unsafe {
            *error = Error {
                message: CString::new(e.to_string()).unwrap().into_raw(),
            };
        },
    }
}
