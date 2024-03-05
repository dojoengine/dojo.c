use dojo_types::{primitive::Primitive, schema::Ty};
use serde_json::Value;
use torii_grpc::types::schema::Entity;

pub fn parse_entities_as_json_str(entities: Vec<Entity>) -> Value {
    entities
        .into_iter()
        .map(|entity| {
            let entity_key = format!("{:#x}", entity.hashed_keys);
            let models_map = entity
                .models
                .into_iter()
                .map(|model| {
                    let model_map = model
                        .members
                        .iter()
                        .map(|member| (member.name.to_owned(), parse_ty_as_json_str(&member.ty)))
                        .collect::<serde_json::Map<String, Value>>();

                    (model.name, model_map.into())
                })
                .collect::<serde_json::Map<String, Value>>();

            (entity_key, models_map.into())
        })
        .collect::<serde_json::Map<String, Value>>()
        .into()
}

pub fn parse_ty_as_json_str(ty: &Ty) -> Value {
    match ty {
        Ty::Primitive(primitive) => serde_json::json!({
            "type": primitive.to_string(),
            "value": primitive_value_json(*primitive)
        }),

        Ty::Struct(struct_ty) => serde_json::json!({
            "type": "struct",
            "value": struct_ty
            .children
            .iter()
            .map(|child| (child.name.to_owned(), parse_ty_as_json_str(&child.ty)))
            .collect::<serde_json::Map<String, Value>>()
        }),

        Ty::Enum(enum_ty) => serde_json::json!({
            "type": "enum",
            "value": if let Some(option) = enum_ty.option {
                option.into()
            } else {
                Value::Null
            }
        }),

        Ty::Tuple(_) => unimplemented!("tuple not supported"),
    }
}

fn primitive_value_json(primitive: Primitive) -> Value {
    match primitive {
        Primitive::Bool(Some(value)) => Value::Bool(value),
        Primitive::U8(Some(value)) => Value::Number(value.into()),
        Primitive::U16(Some(value)) => Value::Number(value.into()),
        Primitive::U32(Some(value)) => Value::Number(value.into()),
        Primitive::U64(Some(value)) => Value::Number(value.into()),
        Primitive::USize(Some(value)) => Value::Number(value.into()),
        Primitive::U128(Some(value)) => Value::String(format!("{value:#x}")),
        Primitive::U256(Some(value)) => Value::String(format!("{value:#x}")),
        Primitive::Felt252(Some(value)) => Value::String(format!("{value:#x}")),
        Primitive::ClassHash(Some(value)) => Value::String(format!("{value:#x}")),
        Primitive::ContractAddress(Some(value)) => Value::String(format!("{value:#x}")),
        _ => Value::Null,
    }
}
