use std::collections::HashMap;

use num_bigint::BigUint;
use num_traits::Num;
use wasm_bindgen::JsValue;

use super::types::{EnumValue, Ty};
use crate::wasm::types::FixedSizeArray;

pub fn parse_ty_as_json_str(ty: &dojo_types::schema::Ty, key: bool) -> Ty {
    match ty {
        dojo_types::schema::Ty::Primitive(primitive) => Ty {
            r#type: "primitive".to_string(),
            type_name: ty.name(),
            value: serde_wasm_bindgen::to_value(&primitive.to_json_value().unwrap()).unwrap(),
            key,
        },
        dojo_types::schema::Ty::Struct(struct_ty) => Ty {
            r#type: "struct".to_string(),
            type_name: ty.name(),
            value: {
                let map = struct_ty
                    .children
                    .iter()
                    .map(|child| {
                        (child.name.to_owned(), parse_ty_as_json_str(&child.ty, child.key))
                    })
                    .collect::<HashMap<String, Ty>>();

                // Convert to JsValue as an object instead of a Map
                let js_obj = js_sys::Object::new();
                for (key, value) in map {
                    js_sys::Reflect::set(
                        &js_obj,
                        &JsValue::from_str(&key),
                        &serde_wasm_bindgen::to_value(&value).unwrap(),
                    )
                    .unwrap();
                }
                JsValue::from(js_obj)
            },
            key,
        },

        dojo_types::schema::Ty::Enum(enum_ty) => Ty {
            r#type: "enum".to_string(),
            type_name: ty.name(),
            value: if let Some(option) = enum_ty.option {
                let option = &enum_ty.options[option as usize];
                serde_wasm_bindgen::to_value(&EnumValue {
                    option: option.name.clone(),
                    // should we hardcode key to always be false for inners of enum?
                    value: parse_ty_as_json_str(&option.ty, false),
                })
                .unwrap()
            } else {
                JsValue::NULL
            },
            key,
        },

        dojo_types::schema::Ty::Tuple(tuple) => Ty {
            r#type: "tuple".to_string(),
            type_name: ty.name(),
            value: serde_wasm_bindgen::to_value(
                &tuple
                .iter()
                // should we hardcode key to always be false for inners of tuple?
                .map(|child| parse_ty_as_json_str(child, false))
                .collect::<Vec<Ty>>(),
            )
            .unwrap(),
            key,
        },
        dojo_types::schema::Ty::Array(array) => Ty {
            r#type: "array".to_string(),
            type_name: ty.name(),
            // shoud we hardcode key to always be false for inners of array?
            value: serde_wasm_bindgen::to_value(
                &array.iter().map(|child| parse_ty_as_json_str(child, false)).collect::<Vec<Ty>>(),
            )
            .unwrap(),
            key,
        },
        dojo_types::schema::Ty::ByteArray(bytearray) => Ty {
            r#type: "bytearray".to_string(),
            type_name: ty.name(),
            value: serde_wasm_bindgen::to_value(bytearray.as_str()).unwrap(),
            key,
        },
        dojo_types::schema::Ty::FixedSizeArray((array, size)) => Ty {
            r#type: "fixed_size_array".to_string(),
            type_name: ty.name(),
            value: serde_wasm_bindgen::to_value(&FixedSizeArray {
                array: array.iter().map(|ty| parse_ty_as_json_str(ty, false)).collect(),
                size: *size,
            })
            .unwrap(),
            key,
        },
    }
}

pub fn pad_to_hex(input: &str) -> Result<String, String> {
    // Process the input to determine format and parse accordingly
    let big_value = if input.starts_with("0x") || input.starts_with("0X") {
        // Parse hexadecimal with prefix
        match BigUint::from_str_radix(&input[2..], 16) {
            Ok(v) => v,
            Err(_) => return Err(format!("Invalid hexadecimal input: {}", input)),
        }
    } else if input.chars().all(|c| c.is_digit(16)) && input.chars().any(|c| !c.is_digit(10)) {
        // Input contains non-decimal digits (a-f, A-F) without 0x prefix, assume hex
        match BigUint::from_str_radix(input, 16) {
            Ok(v) => v,
            Err(_) => return Err(format!("Invalid hexadecimal input: {}", input)),
        }
    } else {
        // Assume decimal otherwise
        match input.parse::<BigUint>() {
            Ok(v) => v,
            Err(_) => return Err(format!("Invalid numeric input: {}", input)),
        }
    };

    // Convert to hex string without 0x prefix
    let hex_string = big_value.to_str_radix(16);

    // Pad to 64 characters
    let padded_hex = format!("{:0>64}", hex_string);

    Ok(padded_hex)
}
