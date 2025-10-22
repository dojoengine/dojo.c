// Schema types - Primitive, Ty, Struct, Enum, Member
use super::core::*;

#[derive(Debug, Clone)]
pub enum Primitive {
    I8 { value: i8 },
    I16 { value: i16 },
    I32 { value: i32 },
    I64 { value: i64 },
    I128 { value: Vec<u8> }, // 16 bytes
    U8 { value: u8 },
    U16 { value: u16 },
    U32 { value: u32 },
    U64 { value: u64 },
    U128 { value: Vec<u8> }, // 16 bytes
    U256 { value: U256 },
    Bool { value: bool },
    Felt252 { value: FieldElement },
    ClassHash { value: FieldElement },
    ContractAddress { value: FieldElement },
    EthAddress { value: FieldElement },
}

impl From<Primitive> for dojo_types::primitive::Primitive {
    fn from(value: Primitive) -> Self {
        match value {
            Primitive::I8 { value: v } => dojo_types::primitive::Primitive::I8(Some(v)),
            Primitive::I16 { value: v } => dojo_types::primitive::Primitive::I16(Some(v)),
            Primitive::I32 { value: v } => dojo_types::primitive::Primitive::I32(Some(v)),
            Primitive::I64 { value: v } => dojo_types::primitive::Primitive::I64(Some(v)),
            Primitive::I128 { value: v } => {
                let mut bytes = [0u8; 16];
                bytes.copy_from_slice(&v);
                dojo_types::primitive::Primitive::I128(Some(i128::from_be_bytes(bytes)))
            }
            Primitive::U8 { value: v } => dojo_types::primitive::Primitive::U8(Some(v)),
            Primitive::U16 { value: v } => dojo_types::primitive::Primitive::U16(Some(v)),
            Primitive::U32 { value: v } => dojo_types::primitive::Primitive::U32(Some(v)),
            Primitive::U64 { value: v } => dojo_types::primitive::Primitive::U64(Some(v)),
            Primitive::U128 { value: v } => {
                let mut bytes = [0u8; 16];
                bytes.copy_from_slice(&v);
                dojo_types::primitive::Primitive::U128(Some(u128::from_be_bytes(bytes)))
            }
            Primitive::U256 { value: v } => {
                dojo_types::primitive::Primitive::U256(Some(uniffi_to_u256(&v).unwrap()))
            }
            Primitive::Bool { value: v } => dojo_types::primitive::Primitive::Bool(Some(v)),
            Primitive::Felt252 { value: v } => {
                dojo_types::primitive::Primitive::Felt252(Some(field_element_to_felt(&v).unwrap()))
            }
            Primitive::ClassHash { value: v } => dojo_types::primitive::Primitive::ClassHash(Some(
                field_element_to_felt(&v).unwrap(),
            )),
            Primitive::ContractAddress { value: v } => {
                dojo_types::primitive::Primitive::ContractAddress(Some(
                    field_element_to_felt(&v).unwrap(),
                ))
            }
            Primitive::EthAddress { value: v } => dojo_types::primitive::Primitive::EthAddress(
                Some(field_element_to_felt(&v).unwrap()),
            ),
        }
    }
}

impl From<dojo_types::primitive::Primitive> for Primitive {
    fn from(value: dojo_types::primitive::Primitive) -> Self {
        match value {
            dojo_types::primitive::Primitive::I8(v) => Primitive::I8 { value: v.unwrap_or(0) },
            dojo_types::primitive::Primitive::I16(v) => Primitive::I16 { value: v.unwrap_or(0) },
            dojo_types::primitive::Primitive::I32(v) => Primitive::I32 { value: v.unwrap_or(0) },
            dojo_types::primitive::Primitive::I64(v) => Primitive::I64 { value: v.unwrap_or(0) },
            dojo_types::primitive::Primitive::I128(v) => {
                Primitive::I128 { value: v.unwrap_or(0).to_be_bytes().to_vec() }
            }
            dojo_types::primitive::Primitive::U8(v) => Primitive::U8 { value: v.unwrap_or(0) },
            dojo_types::primitive::Primitive::U16(v) => Primitive::U16 { value: v.unwrap_or(0) },
            dojo_types::primitive::Primitive::U32(v) => Primitive::U32 { value: v.unwrap_or(0) },
            dojo_types::primitive::Primitive::U64(v) => Primitive::U64 { value: v.unwrap_or(0) },
            dojo_types::primitive::Primitive::U128(v) => {
                Primitive::U128 { value: v.unwrap_or(0).to_be_bytes().to_vec() }
            }
            dojo_types::primitive::Primitive::U256(v) => Primitive::U256 {
                value: v.map(u256_to_uniffi).unwrap_or_else(|| U256("0x0".to_string())),
            },
            dojo_types::primitive::Primitive::Bool(v) => {
                Primitive::Bool { value: v.unwrap_or(false) }
            }
            dojo_types::primitive::Primitive::Felt252(v) => Primitive::Felt252 {
                value: v
                    .map(felt_to_field_element)
                    .unwrap_or_else(|| FieldElement("0x0".to_string())),
            },
            dojo_types::primitive::Primitive::ClassHash(v) => Primitive::ClassHash {
                value: v
                    .map(felt_to_field_element)
                    .unwrap_or_else(|| FieldElement("0x0".to_string())),
            },
            dojo_types::primitive::Primitive::ContractAddress(v) => Primitive::ContractAddress {
                value: v
                    .map(felt_to_field_element)
                    .unwrap_or_else(|| FieldElement("0x0".to_string())),
            },
            dojo_types::primitive::Primitive::EthAddress(v) => Primitive::EthAddress {
                value: v
                    .map(felt_to_field_element)
                    .unwrap_or_else(|| FieldElement("0x0".to_string())),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum MemberValue {
    Primitive { value: Primitive },
    String { value: String },
    List { values: Vec<MemberValue> },
}

impl From<MemberValue> for torii_proto::MemberValue {
    fn from(val: MemberValue) -> Self {
        match val {
            MemberValue::Primitive { value } => torii_proto::MemberValue::Primitive(value.into()),
            MemberValue::String { value } => torii_proto::MemberValue::String(value),
            MemberValue::List { values } => {
                torii_proto::MemberValue::List(values.into_iter().map(|v| v.into()).collect())
            }
        }
    }
}

impl From<torii_proto::MemberValue> for MemberValue {
    fn from(val: torii_proto::MemberValue) -> Self {
        match val {
            torii_proto::MemberValue::Primitive(value) => {
                MemberValue::Primitive { value: value.into() }
            }
            torii_proto::MemberValue::String(value) => MemberValue::String { value },
            torii_proto::MemberValue::List(values) => {
                MemberValue::List { values: values.into_iter().map(|v| v.into()).collect() }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct FixedSizeArray {
    pub array: Vec<Ty>,
    pub size: u32,
}

#[derive(Debug, Clone)]
pub enum Ty {
    Primitive { value: Primitive },
    Struct { value: Struct },
    Enum { value: EnumType },
    Tuple { values: Vec<Ty> },
    Array { values: Vec<Ty> },
    FixedSizeArray { value: FixedSizeArray },
    ByteArray { value: String },
}

impl From<dojo_types::schema::Ty> for Ty {
    fn from(val: dojo_types::schema::Ty) -> Self {
        match val {
            dojo_types::schema::Ty::Primitive(primitive) => {
                Ty::Primitive { value: primitive.into() }
            }
            dojo_types::schema::Ty::Struct(struct_) => Ty::Struct { value: struct_.into() },
            dojo_types::schema::Ty::Enum(enum_) => Ty::Enum { value: enum_.into() },
            dojo_types::schema::Ty::Tuple(tuple) => {
                Ty::Tuple { values: tuple.into_iter().map(|t| t.into()).collect() }
            }
            dojo_types::schema::Ty::Array(array) => {
                Ty::Array { values: array.into_iter().map(|t| t.into()).collect() }
            }
            dojo_types::schema::Ty::FixedSizeArray((ty, size)) => Ty::FixedSizeArray {
                value: FixedSizeArray { array: ty.into_iter().map(|t| t.into()).collect(), size },
            },
            dojo_types::schema::Ty::ByteArray(array) => Ty::ByteArray { value: array },
        }
    }
}

impl From<Ty> for dojo_types::schema::Ty {
    fn from(val: Ty) -> Self {
        match val {
            Ty::Primitive { value } => dojo_types::schema::Ty::Primitive(value.into()),
            Ty::Struct { value } => dojo_types::schema::Ty::Struct(value.into()),
            Ty::Enum { value } => dojo_types::schema::Ty::Enum(value.into()),
            Ty::Tuple { values } => {
                dojo_types::schema::Ty::Tuple(values.into_iter().map(|t| t.into()).collect())
            }
            Ty::Array { values } => {
                dojo_types::schema::Ty::Array(values.into_iter().map(|t| t.into()).collect())
            }
            Ty::FixedSizeArray { value: fixed_size_array } => {
                dojo_types::schema::Ty::FixedSizeArray((
                    fixed_size_array.array.into_iter().map(|t| t.into()).collect(),
                    fixed_size_array.size,
                ))
            }
            Ty::ByteArray { value } => dojo_types::schema::Ty::ByteArray(value),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Member {
    pub name: String,
    pub ty: Ty,
    pub key: bool,
}

impl From<dojo_types::schema::Member> for Member {
    fn from(value: dojo_types::schema::Member) -> Self {
        Member { name: value.name, ty: value.ty.into(), key: value.key }
    }
}

impl From<Member> for dojo_types::schema::Member {
    fn from(value: Member) -> Self {
        dojo_types::schema::Member { name: value.name, ty: value.ty.into(), key: value.key }
    }
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub name: String,
    pub children: Vec<Member>,
}

impl From<Struct> for dojo_types::schema::Struct {
    fn from(value: Struct) -> Self {
        dojo_types::schema::Struct {
            name: value.name,
            children: value.children.into_iter().map(|c| c.into()).collect(),
        }
    }
}

impl From<dojo_types::schema::Struct> for Struct {
    fn from(value: dojo_types::schema::Struct) -> Self {
        Struct {
            name: value.name,
            children: value.children.into_iter().map(|c| c.into()).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EnumOption {
    pub name: String,
    pub ty: Ty,
}

impl From<dojo_types::schema::EnumOption> for EnumOption {
    fn from(value: dojo_types::schema::EnumOption) -> Self {
        EnumOption { name: value.name, ty: value.ty.into() }
    }
}

impl From<EnumOption> for dojo_types::schema::EnumOption {
    fn from(value: EnumOption) -> Self {
        dojo_types::schema::EnumOption { name: value.name, ty: value.ty.into() }
    }
}

#[derive(Debug, Clone)]
pub struct EnumType {
    pub name: String,
    pub option: u8,
    pub options: Vec<EnumOption>,
}

impl From<dojo_types::schema::Enum> for EnumType {
    fn from(value: dojo_types::schema::Enum) -> Self {
        EnumType {
            name: value.name,
            option: value.option.unwrap_or(0),
            options: value.options.into_iter().map(|o| o.into()).collect(),
        }
    }
}

impl From<EnumType> for dojo_types::schema::Enum {
    fn from(value: EnumType) -> Self {
        dojo_types::schema::Enum {
            name: value.name,
            option: Some(value.option),
            options: value.options.into_iter().map(|o| o.into()).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ValueType {
    String { value: String },
    Int { value: i64 },
    UInt { value: u64 },
    Bool { value: bool },
    Bytes { value: Vec<u8> },
}

impl From<ValueType> for torii_proto::ValueType {
    fn from(val: ValueType) -> Self {
        match val {
            ValueType::String { value: v } => torii_proto::ValueType::String(v),
            ValueType::Int { value: v } => torii_proto::ValueType::Int(v),
            ValueType::UInt { value: v } => torii_proto::ValueType::UInt(v),
            ValueType::Bool { value: v } => torii_proto::ValueType::Bool(v),
            ValueType::Bytes { value: v } => torii_proto::ValueType::Bytes(v),
        }
    }
}

impl From<torii_proto::ValueType> for ValueType {
    fn from(val: torii_proto::ValueType) -> Self {
        match val {
            torii_proto::ValueType::String(v) => ValueType::String { value: v },
            torii_proto::ValueType::Int(v) => ValueType::Int { value: v },
            torii_proto::ValueType::UInt(v) => ValueType::UInt { value: v },
            torii_proto::ValueType::Bool(v) => ValueType::Bool { value: v },
            torii_proto::ValueType::Bytes(v) => ValueType::Bytes { value: v },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Value {
    pub primitive_type: Primitive,
    pub value_type: ValueType,
}

impl From<torii_proto::Value> for Value {
    fn from(val: torii_proto::Value) -> Self {
        Value { primitive_type: val.primitive_type.into(), value_type: val.value_type.into() }
    }
}

impl From<Value> for torii_proto::Value {
    fn from(val: Value) -> Self {
        torii_proto::Value {
            primitive_type: val.primitive_type.into(),
            value_type: val.value_type.into(),
        }
    }
}
