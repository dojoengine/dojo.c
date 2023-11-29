use std::ffi::{c_char, CStr, CString};
use torii_client::client::Client as TClient;

pub struct ToriiClient(pub TClient);

#[derive(Clone, Debug)]
#[repr(C)]
pub struct CArray<T> {
    pub data: *mut T,
    pub data_len: usize,
}

impl<T> From<Vec<T>> for CArray<T> {
    fn from(val: Vec<T>) -> Self {
        let mut val = std::mem::ManuallyDrop::new(val);
        val.shrink_to_fit();

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

#[derive(Clone, Debug)]
#[repr(C)]
pub struct CHashItem<K, V> {
    pub key: K,
    pub value: V,
}

#[repr(C)]
pub struct Error {
    pub message: *const c_char,
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Query {
    pub limit: u32,
    pub offset: u32,
    pub clause: Clause,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub enum Clause {
    Keys(KeysClause),
    Member(MemberClause),
    Composite(CompositeClause),
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct KeysClause {
    pub model: *const c_char,
    pub keys: CArray<FieldElement>,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Keys {
    pub model: *const c_char,
    pub keys: CArray<*const c_char>,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct MemberClause {
    pub model: *const c_char,
    pub member: *const c_char,
    pub operator: ComparisonOperator,
    pub value: Value,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct CompositeClause {
    pub model: *const c_char,
    pub operator: LogicalOperator,
    pub clauses: CArray<Clause>,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub enum LogicalOperator {
    And,
    Or,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub enum ComparisonOperator {
    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub enum Value {
    VString(*const c_char),
    Int(i64),
    UInt(u64),
    VBool(bool),
    Bytes(CArray<u8>),
}

#[derive(Clone, Debug)]
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
                    dojo_types::primitive::Primitive::Bool(v) => {
                        Primitive::PBool(v.unwrap_or(false))
                    }
                    dojo_types::primitive::Primitive::Felt252(v) => {
                        if let Some(v) = v {
                            Primitive::Felt252((&v.clone()).into())
                        } else {
                            Primitive::Felt252(FieldElement { data: [0; 32] })
                        }
                    }
                    dojo_types::primitive::Primitive::ClassHash(v) => {
                        if let Some(v) = v {
                            Primitive::Felt252((&v.clone()).into())
                        } else {
                            Primitive::Felt252(FieldElement { data: [0; 32] })
                        }
                    }
                    dojo_types::primitive::Primitive::ContractAddress(v) => {
                        if let Some(v) = v {
                            Primitive::Felt252((&v.clone()).into())
                        } else {
                            Primitive::Felt252(FieldElement { data: [0; 32] })
                        }
                    }
                };

                Ty::TyPrimitive(primitive)
            }
            dojo_types::schema::Ty::Struct(struct_) => {
                let children = struct_
                    .children
                    .iter()
                    .map(|c| Member {
                        name: CString::new(c.name.clone()).unwrap().into_raw(),
                        ty: Box::into_raw(Box::new((&c.ty.clone()).into())),
                        key: c.key,
                    })
                    .collect::<Vec<Member>>();

                Ty::TyStruct(Struct {
                    name: CString::new(struct_.name.clone()).unwrap().into_raw(),
                    children: children.into(),
                })
            }
            dojo_types::schema::Ty::Enum(enum_) => {
                let options = enum_
                    .options
                    .iter()
                    .map(|o| EnumOption {
                        name: CString::new(o.name.clone()).unwrap().into_raw(),
                        ty: Box::into_raw(Box::new((&o.ty.clone()).into())),
                    })
                    .collect::<Vec<EnumOption>>();

                Ty::TyEnum(Enum {
                    name: CString::new(enum_.name.clone()).unwrap().into_raw(),
                    option: enum_.option.unwrap_or(0),
                    options: options.into(),
                })
            }
            dojo_types::schema::Ty::Tuple(tuple) => {
                let children = tuple
                    .iter()
                    .map(|c| (&c.clone()).into())
                    .collect::<Vec<Ty>>();

                Ty::TyTuple(children.into())
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
                        ty: (&*Box::<Ty>::from_raw(c.ty)).into(),
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
                    .map(|o: &EnumOption| dojo_types::schema::EnumOption {
                        name: CString::from_raw(o.name as *mut c_char)
                            .into_string()
                            .unwrap(),
                        ty: (&*Box::<Ty>::from_raw(o.ty)).into(),
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

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Enum {
    pub name: *const c_char,
    pub option: u8,
    pub options: CArray<EnumOption>,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct EnumOption {
    pub name: *const c_char,
    pub ty: *mut Ty,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Struct {
    pub name: *const c_char,
    pub children: CArray<Member>,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Member {
    pub name: *const c_char,
    pub ty: *mut Ty,
    pub key: bool,
}

#[derive(Clone, Debug)]
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

impl From<&Query> for torii_grpc::types::Query {
    fn from(val: &Query) -> Self {
        torii_grpc::types::Query {
            limit: val.limit,
            offset: val.offset,
            clause: (&val.clause.clone()).into(),
        }
    }
}

impl From<&torii_grpc::types::Query> for Query {
    fn from(val: &torii_grpc::types::Query) -> Self {
        Query {
            limit: val.limit,
            offset: val.offset,
            clause: (&val.clause.clone()).into(),
        }
    }
}

impl From<&Clause> for torii_grpc::types::Clause {
    fn from(val: &Clause) -> Self {
        match val {
            Clause::Keys(keys) => torii_grpc::types::Clause::Keys((&keys.clone()).into()),
            Clause::Member(member) => torii_grpc::types::Clause::Member((&member.clone()).into()),
            Clause::Composite(composite) => {
                torii_grpc::types::Clause::Composite((&composite.clone()).into())
            }
        }
    }
}

impl From<&torii_grpc::types::Clause> for Clause {
    fn from(val: &torii_grpc::types::Clause) -> Self {
        match val {
            torii_grpc::types::Clause::Keys(keys) => Clause::Keys((&keys.clone()).into()),
            torii_grpc::types::Clause::Member(member) => Clause::Member((&member.clone()).into()),
            torii_grpc::types::Clause::Composite(composite) => {
                Clause::Composite((&composite.clone()).into())
            }
        }
    }
}

impl From<&Keys> for torii_grpc::types::KeysClause {
    fn from(val: &Keys) -> Self {
        let keys: Vec<*const i8> = (&val.keys).into();
        let keys = std::mem::ManuallyDrop::new(keys);

        torii_grpc::types::KeysClause {
            model: unsafe { CStr::from_ptr(val.model).to_string_lossy().to_string() },
            keys: keys.iter().map(|k| {
                let k = unsafe { CStr::from_ptr(*k).to_string_lossy().to_string() };
                starknet::core::types::FieldElement::from_hex_be(k.as_str()).unwrap()
            }).collect(),
        }
    }
}

impl From<&KeysClause> for torii_grpc::types::KeysClause {
    fn from(val: &KeysClause) -> Self {
        let keys: Vec<FieldElement> = (&val.keys).into();

        torii_grpc::types::KeysClause {
            model: unsafe {
                CString::from_raw(val.model as *mut c_char)
                    .into_string()
                    .unwrap()
            },
            keys: keys.iter().map(|k| k.into()).collect(),
        }
    }
}

impl From<&torii_grpc::types::KeysClause> for KeysClause {
    fn from(val: &torii_grpc::types::KeysClause) -> Self {
        let keys = val
            .keys
            .iter()
            .map(|k| (&k.clone()).into())
            .collect::<Vec<FieldElement>>()
            .to_owned();

        KeysClause {
            model: CString::new(val.model.clone()).unwrap().into_raw(),
            keys: keys.into(),
        }
    }
}

impl From<&MemberClause> for torii_grpc::types::MemberClause {
    fn from(val: &MemberClause) -> Self {
        torii_grpc::types::MemberClause {
            member: unsafe {
                CString::from_raw(val.member as *mut c_char)
                    .into_string()
                    .unwrap()
            },
            model: unsafe {
                CString::from_raw(val.model as *mut c_char)
                    .into_string()
                    .unwrap()
            },
            operator: (&val.operator.clone()).into(),
            value: (&val.value.clone()).into(),
        }
    }
}

impl From<&torii_grpc::types::MemberClause> for MemberClause {
    fn from(val: &torii_grpc::types::MemberClause) -> Self {
        MemberClause {
            model: CString::new(val.model.clone()).unwrap().into_raw(),
            member: CString::new(val.member.clone()).unwrap().into_raw(),
            operator: (&val.operator.clone()).into(),
            value: (&val.value.clone()).into(),
        }
    }
}

impl From<&CompositeClause> for torii_grpc::types::CompositeClause {
    fn from(val: &CompositeClause) -> Self {
        let operator = &val.operator.clone();
        let clauses = unsafe {
            Vec::from_raw_parts(val.clauses.data, val.clauses.data_len, val.clauses.data_len)
        };

        torii_grpc::types::CompositeClause {
            model: unsafe {
                CString::from_raw(val.model as *mut c_char)
                    .into_string()
                    .unwrap()
            },
            operator: operator.into(),
            clauses: clauses.iter().map(|c| c.into()).collect(),
        }
    }
}

impl From<&torii_grpc::types::CompositeClause> for CompositeClause {
    fn from(val: &torii_grpc::types::CompositeClause) -> Self {
        let operator = &val.operator.clone();
        let clauses = val
            .clauses
            .iter()
            .map(|c| (&c.clone()).into())
            .collect::<Vec<Clause>>();

        CompositeClause {
            model: CString::new(val.model.clone()).unwrap().into_raw(),
            operator: operator.into(),
            clauses: clauses.into(),
        }
    }
}

impl From<&LogicalOperator> for torii_grpc::types::LogicalOperator {
    fn from(val: &LogicalOperator) -> Self {
        match val {
            LogicalOperator::And => torii_grpc::types::LogicalOperator::And,
            LogicalOperator::Or => torii_grpc::types::LogicalOperator::Or,
        }
    }
}

impl From<&torii_grpc::types::LogicalOperator> for LogicalOperator {
    fn from(val: &torii_grpc::types::LogicalOperator) -> Self {
        match val {
            torii_grpc::types::LogicalOperator::And => LogicalOperator::And,
            torii_grpc::types::LogicalOperator::Or => LogicalOperator::Or,
        }
    }
}

impl From<&ComparisonOperator> for torii_grpc::types::ComparisonOperator {
    fn from(val: &ComparisonOperator) -> Self {
        match val {
            ComparisonOperator::Eq => torii_grpc::types::ComparisonOperator::Eq,
            ComparisonOperator::Neq => torii_grpc::types::ComparisonOperator::Neq,
            ComparisonOperator::Gt => torii_grpc::types::ComparisonOperator::Gt,
            ComparisonOperator::Gte => torii_grpc::types::ComparisonOperator::Gte,
            ComparisonOperator::Lt => torii_grpc::types::ComparisonOperator::Lt,
            ComparisonOperator::Lte => torii_grpc::types::ComparisonOperator::Lte,
        }
    }
}

impl From<&torii_grpc::types::ComparisonOperator> for ComparisonOperator {
    fn from(val: &torii_grpc::types::ComparisonOperator) -> Self {
        match val {
            torii_grpc::types::ComparisonOperator::Eq => ComparisonOperator::Eq,
            torii_grpc::types::ComparisonOperator::Neq => ComparisonOperator::Neq,
            torii_grpc::types::ComparisonOperator::Gt => ComparisonOperator::Gt,
            torii_grpc::types::ComparisonOperator::Gte => ComparisonOperator::Gte,
            torii_grpc::types::ComparisonOperator::Lt => ComparisonOperator::Lt,
            torii_grpc::types::ComparisonOperator::Lte => ComparisonOperator::Lte,
        }
    }
}

impl From<&Value> for torii_grpc::types::Value {
    fn from(val: &Value) -> Self {
        match val {
            Value::VString(string) => torii_grpc::types::Value::String(unsafe {
                CStr::from_ptr(*string).to_string_lossy().into_owned()
            }),
            Value::Int(int) => torii_grpc::types::Value::Int(*int),
            Value::UInt(uint) => torii_grpc::types::Value::UInt(*uint),
            Value::VBool(bool) => torii_grpc::types::Value::Bool(*bool),
            Value::Bytes(bytes) => unsafe {
                torii_grpc::types::Value::Bytes(Vec::from_raw_parts(
                    bytes.data,
                    bytes.data_len,
                    bytes.data_len,
                ))
            },
        }
    }
}

impl From<&torii_grpc::types::Value> for Value {
    fn from(val: &torii_grpc::types::Value) -> Self {
        match val {
            torii_grpc::types::Value::String(string) => {
                Value::VString(CString::new(string.clone()).unwrap().into_raw())
            }
            torii_grpc::types::Value::Int(int) => Value::Int(*int),
            torii_grpc::types::Value::UInt(uint) => Value::UInt(*uint),
            torii_grpc::types::Value::Bool(bool) => Value::VBool(*bool),
            torii_grpc::types::Value::Bytes(bytes) => Value::Bytes(bytes.to_owned().into()),
        }
    }
}

pub type EntityKeys = CArray<FieldElement>;
pub type StorageKey = FieldElement;
pub type StorageValue = FieldElement;

#[derive(Clone, Debug)]
#[repr(C)]
pub struct ModelIndex {
    model: FieldElement,
    keys: CArray<EntityKeys>,
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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
        let models = value
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
            models: models.into(),
        }
    }
}

#[derive(Clone, Debug)]
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
        let layout = value
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
            layout: layout.into(),
        }
    }
}
