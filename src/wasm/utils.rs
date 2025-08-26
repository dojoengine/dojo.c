use std::collections::HashMap;

use gloo_utils::format::JsValueSerdeExt;
use num_bigint::BigUint;
use num_traits::Num;
use serde_json::Value as JsonValue;
use wasm_bindgen::JsValue;

use super::types::{EnumValue, Ty};
use crate::wasm::types::FixedSizeArray;

fn json_value_to_js_value(json_value: &JsonValue) -> JsValue {
    JsValue::from_serde(json_value).unwrap()
}

pub fn parse_ty_as_json_str(ty: &dojo_types::schema::Ty, key: bool) -> Ty {
    match ty {
        dojo_types::schema::Ty::Primitive(primitive) => Ty {
            r#type: "primitive".to_string(),
            type_name: ty.name(),
            value: json_value_to_js_value(&primitive.to_json_value().unwrap()),
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

#[cfg(test)]
mod tests {
    use dojo_types::primitive::Primitive;
    use serde_json::json;
    use wasm_bindgen_test::*;

    use super::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_json_value_to_js_value_primitives() {
        // Test null
        let json_val = json!(null);
        let js_val = json_value_to_js_value(&json_val);
        assert!(js_val.is_null());

        // Test boolean
        let json_val = json!(true);
        let js_val = json_value_to_js_value(&json_val);
        assert!(js_val.is_truthy());

        let json_val = json!(false);
        let js_val = json_value_to_js_value(&json_val);
        assert!(!js_val.is_truthy());

        // Test number
        let json_val = json!(42);
        let js_val = json_value_to_js_value(&json_val);
        assert!(js_val.is_f64());
        assert_eq!(js_val.as_f64().unwrap() as i32, 42);

        // Test string
        let json_val = json!("hello world");
        let js_val = json_value_to_js_value(&json_val);
        assert!(js_val.is_string());
        assert_eq!(js_val.as_string().unwrap(), "hello world");

        // Test hex string (like what primitives produce)
        let json_val = json!("0x000000000000000000000000000000000000000000000000000000000000002a");
        let js_val = json_value_to_js_value(&json_val);
        assert!(js_val.is_string());
        assert_eq!(
            js_val.as_string().unwrap(),
            "0x000000000000000000000000000000000000000000000000000000000000002a"
        );
    }

    #[wasm_bindgen_test]
    fn test_primitive_to_js_value_conversions() {
        // Test small integers
        let primitive = Primitive::I8(Some(42));
        let json_val = primitive.to_json_value().unwrap();
        let js_val = json_value_to_js_value(&json_val);
        assert!(js_val.is_f64());
        assert_eq!(js_val.as_f64().unwrap() as i8, 42);

        let primitive = Primitive::U32(Some(1234567));
        let json_val = primitive.to_json_value().unwrap();
        let js_val = json_value_to_js_value(&json_val);
        assert!(js_val.is_f64());
        assert_eq!(js_val.as_f64().unwrap() as u32, 1234567);

        // Test boolean
        let primitive = Primitive::Bool(Some(true));
        let json_val = primitive.to_json_value().unwrap();
        let js_val = json_value_to_js_value(&json_val);
        assert!(js_val.is_truthy());

        // Test large integers (should be strings)
        let primitive = Primitive::U64(Some(18446744073709551615u64));
        let json_val = primitive.to_json_value().unwrap();
        let js_val = json_value_to_js_value(&json_val);
        assert!(js_val.is_string());
        assert_eq!(js_val.as_string().unwrap(), "18446744073709551615");

        // Test U256 (should be hex string)
        let primitive = Primitive::U256(Some(starknet_types_core::felt::Felt::from(42u32)));
        let json_val = primitive.to_json_value().unwrap();
        let js_val = json_value_to_js_value(&json_val);
        assert!(js_val.is_string());
        assert!(js_val.as_string().unwrap().starts_with("0x"));

        // Test ContractAddress (should be hex string)
        let primitive =
            Primitive::ContractAddress(Some(starknet_types_core::felt::Felt::from(42u32)));
        let json_val = primitive.to_json_value().unwrap();
        let js_val = json_value_to_js_value(&json_val);
        assert!(js_val.is_string());
        assert!(js_val.as_string().unwrap().starts_with("0x"));
    }

    #[wasm_bindgen_test]
    fn test_json_array_conversion() {
        let json_val = json!([1, 2, 3, "test", true]);
        let js_val = json_value_to_js_value(&json_val);

        // Check if it's an array
        assert!(js_val.is_object());

        // Convert to js_sys::Array to test further
        let js_array = js_sys::Array::from(&js_val);
        assert_eq!(js_array.length(), 5);
    }

    #[wasm_bindgen_test]
    fn test_json_object_conversion() {
        let json_val = json!({
            "name": "test",
            "value": 42,
            "active": true
        });
        let js_val = json_value_to_js_value(&json_val);

        // Check if it's an object
        assert!(js_val.is_object());

        // Test that we can access properties
        let js_obj = js_sys::Object::from(js_val);
        let name_val = js_sys::Reflect::get(&js_obj, &JsValue::from("name")).unwrap();
        assert!(name_val.is_string());
        assert_eq!(name_val.as_string().unwrap(), "test");
    }
}
