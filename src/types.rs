use std::ffi::{c_char, CStr, CString};
use torii_client::client::Client as TClient;

pub struct ToriiClient(pub TClient);

#[derive(Clone)]
#[repr(C)]
pub struct CArray<T> {
    pub data: *mut T,
    pub data_len: usize,
}

impl<T> From<&mut Vec<T>> for CArray<T> {
    fn from(val: &mut Vec<T>) -> Self {
        val.shrink_to_fit();

        let mut val = std::mem::ManuallyDrop::new(val);
        CArray {
            data: val.as_mut_ptr(),
            data_len: val.len(),
        }
    }
}

impl<T> From<&CArray<T>> for Vec<T> {
    fn from(val: &CArray<T>) -> Self {
        unsafe { Vec::from_raw_parts(val.data, val.data_len, val.data_len) }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct CHashItem<K, V> {
    pub key: K,
    pub value: V,
}

#[repr(C)]
pub struct Error {
    pub message: *const c_char,
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

impl From<&starknet::core::types::FieldElement> for FieldElement {
    fn from(val: &starknet::core::types::FieldElement) -> Self {
        FieldElement {
            data: val.to_bytes_be(),
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct EntityQuery {
    pub model: *const c_char,
    pub clause: Clause,
}

#[derive(Clone)]
#[repr(C)]
pub enum Clause {
    Keys(KeysClause),
    Attribute(AttributeClause),
    Composite(CompositeClause),
}

pub type KeysClause = CArray<FieldElement>;

#[derive(Clone)]
#[repr(C)]
pub struct AttributeClause {
    pub attribute: *const c_char,
    pub operator: ComparisonOperator,
    pub value: Value,
}

#[derive(Clone)]
#[repr(C)]
pub struct CompositeClause {
    pub operator: LogicalOperator,
    pub clauses: CArray<Clause>,
}

#[derive(Clone)]
#[repr(C)]
pub enum LogicalOperator {
    And,
    Or,
}

#[derive(Clone)]
#[repr(C)]
pub enum ComparisonOperator {
    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
}

#[derive(Clone)]
#[repr(C)]
pub enum Value {
    VString(*const c_char),
    Int(i64),
    UInt(u64),
    VBool(bool),
    Bytes(CArray<u8>),
}

#[derive(Clone)]
#[repr(C)]
#[allow(clippy::enum_variant_names)]
pub enum Ty {
    TyPrimitive(Primitive),
    TyStruct(Struct),
    TyEnum(Enum),
    TyTuple(CArray<Ty>),
}

impl From<&dojo_types::schema::Ty> for Ty {
    fn from(value: &dojo_types::schema::Ty) -> Self {
        match value {
            dojo_types::schema::Ty::Primitive(primitive) => {
                let primitive = match primitive {
                    dojo_types::primitive::Primitive::U8(v) => Primitive::U8(v.unwrap_or(0)),
                    dojo_types::primitive::Primitive::U16(v) => Primitive::U16(v.unwrap_or(0)),
                    dojo_types::primitive::Primitive::U32(v) => Primitive::U32(v.unwrap_or(0)),
                    dojo_types::primitive::Primitive::U64(v) => Primitive::U64(v.unwrap_or(0)),
                    dojo_types::primitive::Primitive::U128(v) => {
                        if let Some(v) = v {
                            Primitive::U128(v.to_be_bytes())
                        } else {
                            Primitive::U128([0; 16])
                        }
                    }
                    dojo_types::primitive::Primitive::U256(v) => {
                        if let Some(v) = v {
                            Primitive::U256(v.to_words())
                        } else {
                            Primitive::U256([0; 4])
                        }
                    }
                    dojo_types::primitive::Primitive::USize(v) => Primitive::USize(v.unwrap_or(0)),
                    dojo_types::primitive::Primitive::Bool(v) => Primitive::PBool(v.unwrap_or(false)),
                    dojo_types::primitive::Primitive::Felt252(v) => {
                        if let Some(v) = v {
                            Primitive::Felt252((&v.clone()).into())
                        } else {
                            Primitive::Felt252(FieldElement{data: [0; 32]})
                        }
                    }
                    dojo_types::primitive::Primitive::ClassHash(v) => {
                        if let Some(v) = v {
                            Primitive::Felt252((&v.clone()).into())
                        } else {
                            Primitive::Felt252(FieldElement{data: [0; 32]})
                        }
                    }
                    dojo_types::primitive::Primitive::ContractAddress(v) => {
                        if let Some(v) = v {
                            Primitive::Felt252((&v.clone()).into())
                        } else {
                            Primitive::Felt252(FieldElement{data: [0; 32]})
                        }
                    }
                };

                Ty::TyPrimitive(primitive)
            }
            dojo_types::schema::Ty::Struct(struct_) => {
                let children = &mut struct_
                    .children
                    .iter()
                    .map(|c| Member {
                        name: CString::new(c.name.clone()).unwrap().into_raw(),
                        ty: &((&c.ty.clone()).into()),
                        key: c.key,
                    })
                    .collect::<Vec<Member>>();

                Ty::TyStruct(Struct {
                    name: CString::new(struct_.name.clone()).unwrap().into_raw(),
                    children: children.into(),
                })
            }
            dojo_types::schema::Ty::Enum(enum_) => {
                let options = &mut enum_
                    .options
                    .iter()
                    .map(|o| EnumOption {
                        name: CString::new(o.name.clone()).unwrap().into_raw(),
                        ty: &((&o.ty.clone()).into()),
                    })
                    .collect::<Vec<EnumOption>>();

                Ty::TyEnum(Enum {
                    name: CString::new(enum_.name.clone()).unwrap().into_raw(),
                    option: enum_.option.unwrap_or(0),
                    options: (options).into(),
                })
            }
            dojo_types::schema::Ty::Tuple(tuple) => {
                let children = &mut tuple
                    .iter()
                    .map(|c| (&c.clone()).into())
                    .collect::<Vec<Ty>>();

                Ty::TyTuple((children).into())
            }
        }
    }
}

// Implement opposite conversion
// use CString and other alike types to destruct the data
impl From<&Ty> for dojo_types::schema::Ty {
    fn from(value: &Ty) -> Self {
        match value {
            Ty::TyPrimitive(primitive) => {
                let primitive = match primitive {
                    Primitive::U8(v) => dojo_types::primitive::Primitive::U8(Some(*v)),
                    Primitive::U16(v) => dojo_types::primitive::Primitive::U16(Some(*v)),
                    Primitive::U32(v) => dojo_types::primitive::Primitive::U32(Some(*v)),
                    Primitive::U64(v) => dojo_types::primitive::Primitive::U64(Some(*v)),
                    Primitive::U128(v) => {
                        dojo_types::primitive::Primitive::U128(Some(u128::from_be_bytes(*v)))
                    }
                    Primitive::U256(v) => dojo_types::primitive::Primitive::U256(Some((*v).into())),
                    Primitive::USize(v) => dojo_types::primitive::Primitive::USize(Some(*v)),
                    Primitive::PBool(v) => dojo_types::primitive::Primitive::Bool(Some(*v)),
                    Primitive::Felt252(v) => {
                        dojo_types::primitive::Primitive::Felt252(Some((&v.clone()).into()))
                    }
                    Primitive::ClassHash(v) => {
                        dojo_types::primitive::Primitive::ClassHash(Some((&v.clone()).into()))
                    }
                    Primitive::ContractAddress(v) => {
                        dojo_types::primitive::Primitive::ContractAddress(Some((&v.clone()).into()))
                    }
                };

                dojo_types::schema::Ty::Primitive(primitive)
            }
            Ty::TyStruct(struct_) => {
                let children = unsafe {
                    Vec::from_raw_parts(
                        struct_.children.data,
                        struct_.children.data_len,
                        struct_.children.data_len,
                    )
                    .iter()
                    .map(|c| dojo_types::schema::Member {
                        name: CString::from_raw(c.name as *mut c_char)
                            .into_string()
                            .unwrap(),
                        ty: (&*c.ty.clone()).into(),
                        key: c.key,
                    })
                    .collect::<Vec<_>>()
                };

                dojo_types::schema::Ty::Struct(dojo_types::schema::Struct {
                    name: unsafe {
                        CString::from_raw(struct_.name as *mut c_char)
                            .into_string()
                            .unwrap()
                    },
                    children,
                })
            }
            Ty::TyEnum(enum_) => {
                let options = unsafe {
                    Vec::from_raw_parts(
                        enum_.options.data,
                        enum_.options.data_len,
                        enum_.options.data_len,
                    )
                    .iter()
                    .map(|o| dojo_types::schema::EnumOption {
                        name: CString::from_raw(o.name as *mut c_char)
                            .into_string()
                            .unwrap(),
                        ty: (&*o.ty.clone()).into(),
                    })
                    .collect::<Vec<_>>()
                };

                dojo_types::schema::Ty::Enum(dojo_types::schema::Enum {
                    name: unsafe {
                        CString::from_raw(enum_.name as *mut c_char)
                            .into_string()
                            .unwrap()
                    },
                    option: Some(enum_.option),
                    options,
                })
            }
            Ty::TyTuple(tuple) => {
                let children = unsafe {
                    Vec::from_raw_parts(tuple.data, tuple.data_len, tuple.data_len)
                        .iter()
                        .map(|c| (&c.clone()).into())
                        .collect::<Vec<_>>()
                };

                dojo_types::schema::Ty::Tuple(children)
            }
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Enum {
    pub name: *const c_char,
    pub option: u8,
    pub options: CArray<EnumOption>,
}

#[derive(Clone)]
#[repr(C)]
pub struct EnumOption {
    pub name: *const c_char,
    pub ty: *const Ty,
}

#[derive(Clone)]
#[repr(C)]
pub struct Struct {
    pub name: *const c_char,
    pub children: CArray<Member>,
}

#[derive(Clone)]
#[repr(C)]
pub struct Member {
    pub name: *const c_char,
    pub ty: *const Ty,
    pub key: bool,
}

#[derive(Clone)]
#[repr(C)]
pub enum Primitive {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    // TODO: better way?
    U128([u8; 16]),
    U256([u64; 4]),
    USize(u32),
    PBool(bool),
    Felt252(FieldElement),
    ClassHash(FieldElement),
    ContractAddress(FieldElement),
}

impl From<&EntityQuery> for dojo_types::schema::EntityQuery {
    fn from(val: &EntityQuery) -> Self {
        dojo_types::schema::EntityQuery {
            model: unsafe { CStr::from_ptr(val.model).to_string_lossy().into_owned() },
            clause: (&val.clause.clone()).into(),
        }
    }
}

impl From<&dojo_types::schema::EntityQuery> for EntityQuery {
    fn from(val: &dojo_types::schema::EntityQuery) -> Self {
        EntityQuery {
            model: CString::new(val.model.clone()).unwrap().into_raw(),
            clause: (&val.clause.clone()).into(),
        }
    }
}

impl From<&Clause> for dojo_types::schema::Clause {
    fn from(val: &Clause) -> Self {
        match val {
            Clause::Keys(keys) => dojo_types::schema::Clause::Keys((&keys.clone()).into()),
            Clause::Attribute(attribute) => {
                dojo_types::schema::Clause::Attribute((&attribute.clone()).into())
            }
            Clause::Composite(composite) => {
                dojo_types::schema::Clause::Composite((&composite.clone()).into())
            }
        }
    }
}

impl From<&dojo_types::schema::Clause> for Clause {
    fn from(val: &dojo_types::schema::Clause) -> Self {
        match val {
            dojo_types::schema::Clause::Keys(keys) => Clause::Keys((&keys.clone()).into()),
            dojo_types::schema::Clause::Attribute(attribute) => {
                Clause::Attribute((&attribute.clone()).into())
            }
            dojo_types::schema::Clause::Composite(composite) => {
                Clause::Composite((&composite.clone()).into())
            }
        }
    }
}

impl From<&KeysClause> for dojo_types::schema::KeysClause {
    fn from(val: &KeysClause) -> Self {
        let keys = unsafe { std::slice::from_raw_parts(val.data, val.data_len).to_vec() };

        dojo_types::schema::KeysClause {
            keys: keys.iter().map(|k| k.into()).collect(),
        }
    }
}

impl From<&dojo_types::schema::KeysClause> for KeysClause {
    fn from(val: &dojo_types::schema::KeysClause) -> Self {
        let keys = &mut val
            .keys
            .iter()
            .map(|k| (&k.clone()).into())
            .collect::<Vec<FieldElement>>();

        (keys).into()
    }
}

impl From<&AttributeClause> for dojo_types::schema::AttributeClause {
    fn from(val: &AttributeClause) -> Self {
        dojo_types::schema::AttributeClause {
            attribute: unsafe { CStr::from_ptr(val.attribute).to_string_lossy().into_owned() },
            operator: (&val.operator.clone()).into(),
            value: (&val.value.clone()).into(),
        }
    }
}

impl From<&dojo_types::schema::AttributeClause> for AttributeClause {
    fn from(val: &dojo_types::schema::AttributeClause) -> Self {
        AttributeClause {
            attribute: CString::new(val.attribute.clone()).unwrap().into_raw(),
            operator: (&val.operator.clone()).into(),
            value: (&val.value.clone()).into(),
        }
    }
}

impl From<&CompositeClause> for dojo_types::schema::CompositeClause {
    fn from(val: &CompositeClause) -> Self {
        let operator = &val.operator.clone();
        let clauses = unsafe {
            Vec::from_raw_parts(val.clauses.data, val.clauses.data_len, val.clauses.data_len)
        };

        dojo_types::schema::CompositeClause {
            operator: operator.into(),
            clauses: clauses.iter().map(|c| c.into()).collect(),
        }
    }
}

impl From<&dojo_types::schema::CompositeClause> for CompositeClause {
    fn from(val: &dojo_types::schema::CompositeClause) -> Self {
        let operator = &val.operator.clone();
        let clauses = &mut val
            .clauses
            .iter()
            .map(|c| (&c.clone()).into())
            .collect::<Vec<Clause>>();

        CompositeClause {
            operator: operator.into(),
            clauses: (clauses).into(),
        }
    }
}

impl From<&LogicalOperator> for dojo_types::schema::LogicalOperator {
    fn from(val: &LogicalOperator) -> Self {
        match val {
            LogicalOperator::And => dojo_types::schema::LogicalOperator::And,
            LogicalOperator::Or => dojo_types::schema::LogicalOperator::Or,
        }
    }
}

impl From<&dojo_types::schema::LogicalOperator> for LogicalOperator {
    fn from(val: &dojo_types::schema::LogicalOperator) -> Self {
        match val {
            dojo_types::schema::LogicalOperator::And => LogicalOperator::And,
            dojo_types::schema::LogicalOperator::Or => LogicalOperator::Or,
        }
    }
}

impl From<&ComparisonOperator> for dojo_types::schema::ComparisonOperator {
    fn from(val: &ComparisonOperator) -> Self {
        match val {
            ComparisonOperator::Eq => dojo_types::schema::ComparisonOperator::Eq,
            ComparisonOperator::Neq => dojo_types::schema::ComparisonOperator::Neq,
            ComparisonOperator::Gt => dojo_types::schema::ComparisonOperator::Gt,
            ComparisonOperator::Gte => dojo_types::schema::ComparisonOperator::Gte,
            ComparisonOperator::Lt => dojo_types::schema::ComparisonOperator::Lt,
            ComparisonOperator::Lte => dojo_types::schema::ComparisonOperator::Lte,
        }
    }
}

impl From<&dojo_types::schema::ComparisonOperator> for ComparisonOperator {
    fn from(val: &dojo_types::schema::ComparisonOperator) -> Self {
        match val {
            dojo_types::schema::ComparisonOperator::Eq => ComparisonOperator::Eq,
            dojo_types::schema::ComparisonOperator::Neq => ComparisonOperator::Neq,
            dojo_types::schema::ComparisonOperator::Gt => ComparisonOperator::Gt,
            dojo_types::schema::ComparisonOperator::Gte => ComparisonOperator::Gte,
            dojo_types::schema::ComparisonOperator::Lt => ComparisonOperator::Lt,
            dojo_types::schema::ComparisonOperator::Lte => ComparisonOperator::Lte,
        }
    }
}

impl From<&Value> for dojo_types::schema::Value {
    fn from(val: &Value) -> Self {
        match val {
            Value::VString(string) => dojo_types::schema::Value::String(unsafe {
                CStr::from_ptr(*string).to_string_lossy().into_owned()
            }),
            Value::Int(int) => dojo_types::schema::Value::Int(*int),
            Value::UInt(uint) => dojo_types::schema::Value::UInt(*uint),
            Value::VBool(bool) => dojo_types::schema::Value::Bool(*bool),
            Value::Bytes(bytes) => unsafe {
                dojo_types::schema::Value::Bytes(Vec::from_raw_parts(
                    bytes.data,
                    bytes.data_len,
                    bytes.data_len,
                ))
            },
        }
    }
}

impl From<&dojo_types::schema::Value> for Value {
    fn from(val: &dojo_types::schema::Value) -> Self {
        match val {
            dojo_types::schema::Value::String(string) => {
                Value::VString(CString::new(string.clone()).unwrap().into_raw())
            }
            dojo_types::schema::Value::Int(int) => Value::Int(*int),
            dojo_types::schema::Value::UInt(uint) => Value::UInt(*uint),
            dojo_types::schema::Value::Bool(bool) => Value::VBool(*bool),
            dojo_types::schema::Value::Bytes(bytes) => Value::Bytes((&mut bytes.clone()).into()),
        }
    }
}

pub type EntityKeys = CArray<FieldElement>;
pub type StorageKey = FieldElement;
pub type StorageValue = FieldElement;

#[derive(Clone)]
#[repr(C)]
pub struct ModelIndex {
    model: FieldElement,
    keys: CArray<EntityKeys>,
}

#[derive(Clone)]
#[repr(C)]
pub struct ModelStorage {
    metadata: WorldMetadata,
    storage: CArray<CHashItem<StorageKey, StorageValue>>,
    // a map of model name to a set of entity keys.
    model_index: CArray<CHashItem<FieldElement, CArray<EntityKeys>>>,
    // listener for storage updates.
    // senders: Mutex<HashMap<u8, Sender<()>>>,
    // listeners: Mutex<HashMap<StorageKey, Vec<u8>>>,
}

// impl From<&torii_client::client::storage::ModelStorage> for ModelStorage {
//     fn from(value: &torii_client::client::storage::ModelStorage) -> Self {
//         let metadata = value.metadata;
//         let storage = value.storage.clone();
//         let model_index = value.model_index.clone();

//         Self {
//             metadata: (&metadata).into(),
//             storage: (&storage).into(),
//             model_index: (&model_index).into(),
//         }
//     }
// }

#[derive(Clone)]
#[repr(C)]
pub struct WorldMetadata {
    pub world_address: FieldElement,
    pub world_class_hash: FieldElement,
    pub executor_address: FieldElement,
    pub executor_class_hash: FieldElement,
    pub models: CArray<CHashItem<*const c_char, ModelMetadata>>,
}

impl From<&dojo_types::WorldMetadata> for WorldMetadata {
    fn from(value: &dojo_types::WorldMetadata) -> Self {
        let models = &mut value
            .models
            .iter()
            .map(|(k, v)| CHashItem {
                key: CString::new(k.clone()).unwrap().into_raw() as *const c_char,
                value: (&v.clone()).into(),
            })
            .collect::<Vec<CHashItem<*const c_char, ModelMetadata>>>();

        WorldMetadata {
            world_address: (&value.world_address.clone()).into(),
            world_class_hash: (&value.world_class_hash.clone()).into(),
            executor_address: (&value.executor_address.clone()).into(),
            executor_class_hash: (&value.executor_class_hash.clone()).into(),
            models: (models).into(),
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct ModelMetadata {
    pub schema: Ty,
    pub name: *const c_char,
    pub packed_size: u32,
    pub unpacked_size: u32,
    pub class_hash: FieldElement,
    pub layout: CArray<FieldElement>,
}

impl From<&dojo_types::schema::ModelMetadata> for ModelMetadata {
    fn from(value: &dojo_types::schema::ModelMetadata) -> Self {
        let layout = &mut value
            .layout
            .iter()
            .map(|v| (&v.clone()).into())
            .collect::<Vec<FieldElement>>();

        ModelMetadata {
            schema: (&value.schema.clone()).into(),
            name: CString::new(value.name.clone()).unwrap().into_raw(),
            packed_size: value.packed_size,
            unpacked_size: value.unpacked_size,
            class_hash: (&value.class_hash.clone()).into(),
            layout: (layout).into(),
        }
    }
}
