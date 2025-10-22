// Entity, Model, and World types
use chrono::DateTime;

use super::core::*;
use super::schema::{Struct, Ty};

#[derive(Debug, Clone)]
pub struct Entity {
    pub world_address: FieldElement,
    pub hashed_keys: FieldElement,
    pub models: Vec<Struct>,
    pub created_at: u64,
    pub updated_at: u64,
    pub executed_at: u64,
}

impl From<Entity> for torii_proto::schema::Entity {
    fn from(val: Entity) -> Self {
        torii_proto::schema::Entity {
            world_address: field_element_to_felt(&val.world_address).unwrap(),
            hashed_keys: field_element_to_felt(&val.hashed_keys).unwrap(),
            models: val.models.into_iter().map(|m| m.into()).collect(),
            created_at: DateTime::from_timestamp(val.created_at as i64, 0).unwrap(),
            updated_at: DateTime::from_timestamp(val.updated_at as i64, 0).unwrap(),
            executed_at: DateTime::from_timestamp(val.executed_at as i64, 0).unwrap(),
        }
    }
}

impl From<torii_proto::schema::Entity> for Entity {
    fn from(val: torii_proto::schema::Entity) -> Self {
        Entity {
            world_address: felt_to_field_element(val.world_address),
            hashed_keys: felt_to_field_element(val.hashed_keys),
            models: val.models.into_iter().map(|m| m.into()).collect(),
            created_at: val.created_at.timestamp() as u64,
            updated_at: val.updated_at.timestamp() as u64,
            executed_at: val.executed_at.timestamp() as u64,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Model {
    pub world_address: FieldElement,
    pub schema: Ty,
    pub namespace: String,
    pub name: String,
    pub selector: FieldElement,
    pub packed_size: u32,
    pub unpacked_size: u32,
    pub class_hash: FieldElement,
    pub contract_address: FieldElement,
    pub layout: String,
    pub use_legacy_store: bool,
}

impl From<torii_proto::Model> for Model {
    fn from(value: torii_proto::Model) -> Self {
        let layout = serde_json::to_string(&value.layout).unwrap();

        Model {
            world_address: felt_to_field_element(value.world_address),
            schema: value.schema.into(),
            name: value.name,
            namespace: value.namespace,
            selector: felt_to_field_element(value.selector),
            packed_size: value.packed_size,
            unpacked_size: value.unpacked_size,
            class_hash: felt_to_field_element(value.class_hash),
            contract_address: felt_to_field_element(value.contract_address),
            layout,
            use_legacy_store: value.use_legacy_store,
        }
    }
}

impl From<Model> for torii_proto::Model {
    fn from(value: Model) -> Self {
        let layout = serde_json::from_str(&value.layout).unwrap();

        torii_proto::Model {
            world_address: field_element_to_felt(&value.world_address).unwrap(),
            schema: value.schema.into(),
            namespace: value.namespace,
            name: value.name,
            selector: field_element_to_felt(&value.selector).unwrap(),
            packed_size: value.packed_size,
            unpacked_size: value.unpacked_size,
            class_hash: field_element_to_felt(&value.class_hash).unwrap(),
            contract_address: field_element_to_felt(&value.contract_address).unwrap(),
            layout,
            use_legacy_store: value.use_legacy_store,
        }
    }
}

#[derive(Debug, Clone)]
pub struct World {
    pub world_address: FieldElement,
    pub models: Vec<Model>,
}

impl From<torii_proto::World> for World {
    fn from(value: torii_proto::World) -> Self {
        let models: Vec<Model> = value.models.into_values().map(|v| v.into()).collect();

        World { world_address: felt_to_field_element(value.world_address), models }
    }
}

impl From<World> for torii_proto::World {
    fn from(value: World) -> Self {
        let models: Vec<torii_proto::Model> = value.models.into_iter().map(|m| m.into()).collect();
        let models = models
            .into_iter()
            .map(|m| (dojo_types::naming::compute_selector_from_names(&m.namespace, &m.name), m))
            .collect();

        torii_proto::World {
            world_address: field_element_to_felt(&value.world_address).unwrap(),
            models,
        }
    }
}
