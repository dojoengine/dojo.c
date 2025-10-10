
#[diplomat::bridge]
pub mod ffi {
    use diplomat_runtime::{DiplomatStr, DiplomatWrite};
    use std::fmt::Write;
    use crate::ffi::error::ffi::DojoError;

    /// Type of primitive
    pub enum PrimitiveType {
        I8,
        I16,
        I32,
        I64,
        I128,
        U8,
        U16,
        U32,
        U64,
        U128,
        U256,
        Bool,
        Felt252,
        ClassHash,
        ContractAddress,
        EthAddress,
    }

    /// Represents a primitive Cairo type with its value
    #[diplomat::opaque]
    pub struct Primitive {
        pub(crate) inner: dojo_types::primitive::Primitive,
    }

    impl Primitive {
        /// Gets the primitive type
        pub fn primitive_type(&self) -> PrimitiveType {
            match &self.inner {
                dojo_types::primitive::Primitive::I8(_) => PrimitiveType::I8,
                dojo_types::primitive::Primitive::I16(_) => PrimitiveType::I16,
                dojo_types::primitive::Primitive::I32(_) => PrimitiveType::I32,
                dojo_types::primitive::Primitive::I64(_) => PrimitiveType::I64,
                dojo_types::primitive::Primitive::I128(_) => PrimitiveType::I128,
                dojo_types::primitive::Primitive::U8(_) => PrimitiveType::U8,
                dojo_types::primitive::Primitive::U16(_) => PrimitiveType::U16,
                dojo_types::primitive::Primitive::U32(_) => PrimitiveType::U32,
                dojo_types::primitive::Primitive::U64(_) => PrimitiveType::U64,
                dojo_types::primitive::Primitive::U128(_) => PrimitiveType::U128,
                dojo_types::primitive::Primitive::U256(_) => PrimitiveType::U256,
                dojo_types::primitive::Primitive::Bool(_) => PrimitiveType::Bool,
                dojo_types::primitive::Primitive::Felt252(_) => PrimitiveType::Felt252,
                dojo_types::primitive::Primitive::ClassHash(_) => PrimitiveType::ClassHash,
                dojo_types::primitive::Primitive::ContractAddress(_) => PrimitiveType::ContractAddress,
                dojo_types::primitive::Primitive::EthAddress(_) => PrimitiveType::EthAddress,
            }
        }

        /// Creates a primitive from JSON
        pub fn from_json(json: &DiplomatStr) -> Result<Box<Primitive>, Box<DojoError>> {
            let s = std::str::from_utf8(json)?;
            let inner: dojo_types::primitive::Primitive = serde_json::from_str(s)?;
            Ok(Box::new(Primitive { inner }))
        }

        /// Serializes the primitive to JSON
        pub fn to_json(&self, write: &mut DiplomatWrite) -> Result<(), Box<DojoError>> {
            let json = serde_json::to_string(&self.inner)?;
            write!(write, "{}", json).unwrap();
            Ok(())
        }
    }

    /// Represents a Dojo struct member (field)
    #[diplomat::opaque]
    pub struct Member {
        pub(crate) inner: dojo_types::schema::Member,
    }

    impl Member {
        /// Gets the member name
        pub fn name(&self, write: &mut DiplomatWrite) {
            write!(write, "{}", self.inner.name).unwrap();
        }

        /// Returns true if this member is a key
        pub fn is_key(&self) -> bool {
            self.inner.key
        }
    }

    /// Represents a Dojo struct
    #[diplomat::opaque]
    pub struct Struct {
        pub(crate) inner: dojo_types::schema::Struct,
    }

    impl Struct {
        /// Gets the struct name
        pub fn name(&self, write: &mut DiplomatWrite) {
            write!(write, "{}", self.inner.name).unwrap();
        }

        /// Gets the number of children (members)
        pub fn children_count(&self) -> u32 {
            self.inner.children.len() as u32
        }

        /// Creates a new struct from JSON schema
        pub fn from_json(json: &DiplomatStr) -> Result<Box<Struct>, Box<DojoError>> {
            let s = std::str::from_utf8(json)?;
            let inner: dojo_types::schema::Struct = serde_json::from_str(s)?;
            Ok(Box::new(Struct { inner }))
        }

        /// Serializes the struct to JSON
        pub fn to_json(&self, write: &mut DiplomatWrite) -> Result<(), Box<DojoError>> {
            let json = serde_json::to_string(&self.inner)?;
            write!(write, "{}", json).unwrap();
            Ok(())
        }
    }

    /// Represents a Dojo enum option (variant)
    #[diplomat::opaque]
    pub struct EnumOption {
        pub(crate) inner: dojo_types::schema::EnumOption,
    }

    impl EnumOption {
        /// Gets the option name
        pub fn name(&self, write: &mut DiplomatWrite) {
            write!(write, "{}", self.inner.name).unwrap();
        }
    }

    /// Represents a Dojo enum
    #[diplomat::opaque]
    pub struct Enum {
        pub(crate) inner: dojo_types::schema::Enum,
    }

    impl Enum {
        /// Gets the enum name
        pub fn name(&self, write: &mut DiplomatWrite) {
            write!(write, "{}", self.inner.name).unwrap();
        }

        /// Gets the current option (selected variant) index
        pub fn option(&self) -> u8 {
            self.inner.option.unwrap_or(0)
        }

        /// Gets the number of options (variants)
        pub fn options_count(&self) -> u32 {
            self.inner.options.len() as u32
        }

        /// Creates a new enum from JSON schema
        pub fn from_json(json: &DiplomatStr) -> Result<Box<Enum>, Box<DojoError>> {
            let s = std::str::from_utf8(json)?;
            let inner: dojo_types::schema::Enum = serde_json::from_str(s)?;
            Ok(Box::new(Enum { inner }))
        }

        /// Serializes the enum to JSON
        pub fn to_json(&self, write: &mut DiplomatWrite) -> Result<(), Box<DojoError>> {
            let json = serde_json::to_string(&self.inner)?;
            write!(write, "{}", json).unwrap();
            Ok(())
        }
    }

    /// Represents a Dojo model
    #[diplomat::opaque]
    pub struct Model {
        pub(crate) inner: torii_proto::Model,
    }

    impl Model {
        /// Gets the model name
        pub fn name(&self, write: &mut DiplomatWrite) {
            write!(write, "{}", self.inner.name).unwrap();
        }

        /// Gets the model namespace
        pub fn namespace(&self, write: &mut DiplomatWrite) {
            write!(write, "{}", self.inner.namespace).unwrap();
        }

        /// Gets the model selector (hex)
        pub fn selector(&self, write: &mut DiplomatWrite) {
            write!(write, "{:#x}", self.inner.selector).unwrap();
        }

        /// Gets the model class hash (hex)
        pub fn class_hash(&self, write: &mut DiplomatWrite) {
            write!(write, "{:#x}", self.inner.class_hash).unwrap();
        }

        /// Gets the model contract address (hex)
        pub fn contract_address(&self, write: &mut DiplomatWrite) {
            write!(write, "{:#x}", self.inner.contract_address).unwrap();
        }

        /// Gets the packed size
        pub fn packed_size(&self) -> u32 {
            self.inner.packed_size
        }

        /// Gets the unpacked size
        pub fn unpacked_size(&self) -> u32 {
            self.inner.unpacked_size
        }

        /// Returns true if the model uses legacy store
        pub fn use_legacy_store(&self) -> bool {
            self.inner.use_legacy_store
        }

        /// Creates a new model from JSON
        pub fn from_json(json: &DiplomatStr) -> Result<Box<Model>, Box<DojoError>> {
            let s = std::str::from_utf8(json)?;
            let inner: torii_proto::Model = serde_json::from_str(s)?;
            Ok(Box::new(Model { inner }))
        }

        /// Serializes the model to JSON
        pub fn to_json(&self, write: &mut DiplomatWrite) -> Result<(), Box<DojoError>> {
            let json = serde_json::to_string(&self.inner)?;
            write!(write, "{}", json).unwrap();
            Ok(())
        }
    }

    /// Represents a Dojo entity
    #[diplomat::opaque]
    pub struct Entity {
        pub(crate) inner: torii_proto::schema::Entity,
    }

    impl Entity {
        /// Gets the hashed keys (hex)
        pub fn hashed_keys(&self, write: &mut DiplomatWrite) {
            write!(write, "{:#x}", self.inner.hashed_keys).unwrap();
        }

        /// Gets the number of models in this entity
        pub fn models_count(&self) -> u32 {
            self.inner.models.len() as u32
        }

        /// Gets the created_at timestamp
        pub fn created_at(&self) -> u64 {
            self.inner.created_at.timestamp() as u64
        }

        /// Gets the updated_at timestamp
        pub fn updated_at(&self) -> u64 {
            self.inner.updated_at.timestamp() as u64
        }

        /// Gets the executed_at timestamp
        pub fn executed_at(&self) -> u64 {
            self.inner.executed_at.timestamp() as u64
        }

        /// Creates a new entity from JSON
        pub fn from_json(json: &DiplomatStr) -> Result<Box<Entity>, Box<DojoError>> {
            let s = std::str::from_utf8(json)?;
            let inner: torii_proto::schema::Entity = serde_json::from_str(s)?;
            Ok(Box::new(Entity { inner }))
        }

        /// Serializes the entity to JSON
        pub fn to_json(&self, write: &mut DiplomatWrite) -> Result<(), Box<DojoError>> {
            let json = serde_json::to_string(&self.inner)?;
            write!(write, "{}", json).unwrap();
            Ok(())
        }
    }

    /// Represents a Dojo world
    #[diplomat::opaque]
    pub struct World {
        pub(crate) inner: torii_proto::World,
    }

    impl World {
        /// Gets the world address (hex)
        pub fn world_address(&self, write: &mut DiplomatWrite) {
            write!(write, "{:#x}", self.inner.world_address).unwrap();
        }

        /// Gets the number of models in this world
        pub fn models_count(&self) -> u32 {
            self.inner.models.len() as u32
        }

        /// Creates a new world from JSON
        pub fn from_json(json: &DiplomatStr) -> Result<Box<World>, Box<DojoError>> {
            let s = std::str::from_utf8(json)?;
            let inner: torii_proto::World = serde_json::from_str(s)?;
            Ok(Box::new(World { inner }))
        }

        /// Serializes the world to JSON
        pub fn to_json(&self, write: &mut DiplomatWrite) -> Result<(), Box<DojoError>> {
            let json = serde_json::to_string(&self.inner)?;
            write!(write, "{}", json).unwrap();
            Ok(())
        }
    }
}

