use starknet::{
    accounts::SingleOwnerAccount,
    core::utils::get_selector_from_name,
    providers::{jsonrpc::HttpTransport, JsonRpcClient},
    signers::LocalWallet,
};
use std::{
    ffi::{c_char, CStr, CString},
    fmt::Write,
};
use torii_client::client::Client;

pub struct Account(pub SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>);

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Call {
    pub to: *const c_char,
    pub selector: *const c_char,
    pub calldata: CArray<FieldElement>,
}

impl From<&Call> for starknet::accounts::Call {
    fn from(val: &Call) -> Self {
        let to = unsafe { CStr::from_ptr(val.to).to_string_lossy().to_string() };
        let selector = unsafe { CStr::from_ptr(val.selector).to_string_lossy().to_string() };

        let calldata: Vec<FieldElement> = (&val.calldata).into();
        let calldata = calldata.iter().map(|c| (&c.clone()).into()).collect();

        starknet::accounts::Call {
            to: starknet_crypto::FieldElement::from_hex_be(&to).unwrap(),
            selector: get_selector_from_name(&selector).unwrap(),
            calldata,
        }
    }
}

pub struct ToriiClient {
    pub inner: Client,
    pub rpc_url: String,
    pub runtime: tokio::runtime::Runtime,
}

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
    CMember(MemberClause),
    Composite(CompositeClause),
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct KeysClause {
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
pub struct Value {
    pub primitive_type: Primitive,
    pub value_type: ValueType,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub enum ValueType {
    VString(*const c_char),
    Int(i64),
    UInt(u64),
    VBool(bool),
    Bytes(CArray<u8>),
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Entity {
    pub key: FieldElement,
    pub models: CArray<Model>,
}

impl From<&Entity> for torii_grpc::types::Entity {
    fn from(val: &Entity) -> Self {
        let models: Vec<Model> = (&val.models).into();
        let models = models.iter().map(|m| (&m.clone()).into()).collect();

        torii_grpc::types::Entity {
            key: (&val.key.clone()).into(),
            models,
        }
    }
}

impl From<&torii_grpc::types::Entity> for Entity {
    fn from(val: &torii_grpc::types::Entity) -> Self {
        let models = val
            .models
            .iter()
            .map(|m| (&m.clone()).into())
            .collect::<Vec<Model>>();

        Entity {
            key: (&val.key.clone()).into(),
            models: models.into(),
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Model {
    pub name: *const c_char,
    pub members: CArray<Member>,
}

impl From<&Model> for torii_grpc::types::Model {
    fn from(val: &Model) -> Self {
        let members: Vec<Member> = (&val.members).into();

        torii_grpc::types::Model {
            name: unsafe {
                CString::from_raw(val.name as *mut c_char)
                    .into_string()
                    .unwrap()
            },
            members: members.iter().map(|m| m.into()).collect(),
        }
    }
}

impl From<&torii_grpc::types::Model> for Model {
    fn from(val: &torii_grpc::types::Model) -> Self {
        let members = val
            .members
            .iter()
            .map(|m| (&m.clone()).into())
            .collect::<Vec<Member>>();

        Model {
            name: CString::new(val.name.clone()).unwrap().into_raw(),
            members: members.into(),
        }
    }
}

impl From<&ValueType> for torii_grpc::types::ValueType {
    fn from(value: &ValueType) -> Self {
        match value {
            ValueType::VString(v) => {
                let v = unsafe { CStr::from_ptr(*v).to_string_lossy().into_owned() };
                torii_grpc::types::ValueType::String(v)
            }
            ValueType::Int(v) => torii_grpc::types::ValueType::Int(*v),
            ValueType::UInt(v) => torii_grpc::types::ValueType::UInt(*v),
            ValueType::VBool(v) => torii_grpc::types::ValueType::Bool(*v),
            ValueType::Bytes(v) => {
                let v = v.into();
                torii_grpc::types::ValueType::Bytes(v)
            }
        }
    }
}

impl From<&torii_grpc::types::ValueType> for ValueType {
    fn from(value: &torii_grpc::types::ValueType) -> Self {
        match value {
            torii_grpc::types::ValueType::String(v) => {
                let v = CString::new(v.clone()).unwrap().into_raw();
                ValueType::VString(v)
            }
            torii_grpc::types::ValueType::Int(v) => ValueType::Int(*v),
            torii_grpc::types::ValueType::UInt(v) => ValueType::UInt(*v),
            torii_grpc::types::ValueType::Bool(v) => ValueType::VBool(*v),
            torii_grpc::types::ValueType::Bytes(v) => {
                let v = v.clone().into();
                ValueType::Bytes(v)
            }
        }
    }
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
                let primitive = primitive.into();

                Ty::TyPrimitive(primitive)
            }
            dojo_types::schema::Ty::Struct(struct_) => Ty::TyStruct((&struct_.clone()).into()),
            dojo_types::schema::Ty::Enum(enum_) => Ty::TyEnum((&enum_.clone()).into()),
            dojo_types::schema::Ty::Tuple(tuple) => {
                let children = tuple
                    .iter()
                    .map(|c| (&c.clone()).into())
                    .collect::<Vec<_>>();

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
                dojo_types::schema::Ty::Primitive((&primitive.clone()).into())
            }
            Ty::TyStruct(struct_) => dojo_types::schema::Ty::Struct((&struct_.clone()).into()),
            Ty::TyEnum(enum_) => dojo_types::schema::Ty::Enum((&enum_.clone()).into()),
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

impl From<&Enum> for dojo_types::schema::Enum {
    fn from(value: &Enum) -> Self {
        let options: Vec<EnumOption> = (&value.options).into();
        let options = options.iter().map(|o| (&o.clone()).into()).collect();

        dojo_types::schema::Enum {
            name: unsafe {
                CString::from_raw(value.name as *mut c_char)
                    .into_string()
                    .unwrap()
            },
            option: Some(value.option),
            options,
        }
    }
}

impl From<&dojo_types::schema::Enum> for Enum {
    fn from(value: &dojo_types::schema::Enum) -> Self {
        let options = value
            .options
            .iter()
            .map(|o| (&o.clone()).into())
            .collect::<Vec<EnumOption>>();

        Enum {
            name: CString::new(value.name.clone()).unwrap().into_raw(),
            option: value.option.unwrap_or(0),
            options: options.into(),
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct EnumOption {
    pub name: *const c_char,
    pub ty: *mut Ty,
}

impl From<&EnumOption> for dojo_types::schema::EnumOption {
    fn from(value: &EnumOption) -> Self {
        dojo_types::schema::EnumOption {
            name: unsafe {
                CString::from_raw(value.name as *mut c_char)
                    .into_string()
                    .unwrap()
            },
            ty: unsafe { (&*Box::<Ty>::from_raw(value.ty)).into() },
        }
    }
}

impl From<&dojo_types::schema::EnumOption> for EnumOption {
    fn from(value: &dojo_types::schema::EnumOption) -> Self {
        EnumOption {
            name: CString::new(value.name.clone()).unwrap().into_raw(),
            ty: Box::into_raw(Box::new((&value.ty.clone()).into())),
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Struct {
    pub name: *const c_char,
    pub children: CArray<Member>,
}

impl From<&Struct> for dojo_types::schema::Struct {
    fn from(value: &Struct) -> Self {
        let children: Vec<Member> = (&value.children).into();
        let children = children.iter().map(|c| (&c.clone()).into()).collect();

        dojo_types::schema::Struct {
            name: unsafe {
                CString::from_raw(value.name as *mut c_char)
                    .into_string()
                    .unwrap()
            },
            children,
        }
    }
}

impl From<&dojo_types::schema::Struct> for Struct {
    fn from(value: &dojo_types::schema::Struct) -> Self {
        let children = value
            .children
            .iter()
            .map(|c| (&c.clone()).into())
            .collect::<Vec<Member>>();

        Struct {
            name: CString::new(value.name.clone()).unwrap().into_raw(),
            children: children.into(),
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Member {
    pub name: *const c_char,
    pub ty: *mut Ty,
    pub key: bool,
}

impl From<&Member> for dojo_types::schema::Member {
    fn from(value: &Member) -> Self {
        dojo_types::schema::Member {
            name: unsafe {
                CString::from_raw(value.name as *mut c_char)
                    .into_string()
                    .unwrap()
            },
            ty: unsafe { (&*Box::<Ty>::from_raw(value.ty)).into() },
            key: value.key,
        }
    }
}

impl From<&dojo_types::schema::Member> for Member {
    fn from(value: &dojo_types::schema::Member) -> Self {
        Member {
            name: CString::new(value.name.clone()).unwrap().into_raw(),
            ty: Box::into_raw(Box::new((&value.ty.clone()).into())),
            key: value.key,
        }
    }
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

impl From<&Primitive> for dojo_types::primitive::Primitive {
    fn from(value: &Primitive) -> Self {
        match value {
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
        }
    }
}

impl From<&dojo_types::primitive::Primitive> for Primitive {
    fn from(value: &dojo_types::primitive::Primitive) -> Self {
        match value {
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
        }
    }
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
            Clause::CMember(member) => torii_grpc::types::Clause::Member((&member.clone()).into()),
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
            torii_grpc::types::Clause::Member(member) => Clause::CMember((&member.clone()).into()),
            torii_grpc::types::Clause::Composite(composite) => {
                Clause::Composite((&composite.clone()).into())
            }
        }
    }
}

impl From<&KeysClause> for torii_grpc::types::KeysClause {
    fn from(val: &KeysClause) -> Self {
        let keys: Vec<*const c_char> = (&val.keys).into();
        let keys = std::mem::ManuallyDrop::new(keys);

        torii_grpc::types::KeysClause {
            model: unsafe { CStr::from_ptr(val.model).to_string_lossy().to_string() },
            keys: keys
                .iter()
                .map(|k| {
                    let k = unsafe { CStr::from_ptr(*k).to_string_lossy().to_string() };
                    starknet::core::types::FieldElement::from_hex_be(k.as_str()).unwrap()
                })
                .collect(),
        }
    }
}

impl From<&torii_grpc::types::KeysClause> for KeysClause {
    fn from(val: &torii_grpc::types::KeysClause) -> Self {
        let keys = val
            .keys
            .iter()
            .map(|k| {
                // convert bytes to hex string
                let str = k.to_bytes_be().iter().fold("0x".to_string(), |mut acc, b| {
                    write!(acc, "{:02x}", b).unwrap();
                    acc
                });
                CString::new(str).unwrap().into_raw() as *const c_char
            })
            .collect::<Vec<*const c_char>>();
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
        torii_grpc::types::Value {
            primitive_type: (&val.primitive_type).into(),
            value_type: (&val.value_type).into(),
        }
    }
}

impl From<&torii_grpc::types::Value> for Value {
    fn from(val: &torii_grpc::types::Value) -> Self {
        Value {
            primitive_type: (&val.primitive_type).into(),
            value_type: (&val.value_type).into(),
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

impl From<&WorldMetadata> for dojo_types::WorldMetadata {
    fn from(value: &WorldMetadata) -> Self {
        let models: Vec<CHashItem<*const c_char, ModelMetadata>> = (&value.models).into();
        let models = models
            .iter()
            .map(|m| {
                let key = unsafe {
                    CString::from_raw(m.key as *mut c_char)
                        .into_string()
                        .unwrap()
                };
                let value: dojo_types::schema::ModelMetadata = (&m.value).into();

                (key, value)
            })
            .collect();

        dojo_types::WorldMetadata {
            world_address: (&value.world_address.clone()).into(),
            world_class_hash: (&value.world_class_hash.clone()).into(),
            executor_address: (&value.executor_address.clone()).into(),
            executor_class_hash: (&value.executor_class_hash.clone()).into(),
            models,
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

impl From<&ModelMetadata> for dojo_types::schema::ModelMetadata {
    fn from(value: &ModelMetadata) -> Self {
        let layout: Vec<FieldElement> = (&value.layout).into();

        let layout: Vec<starknet::core::types::FieldElement> =
            layout.iter().map(|v| (&v.clone()).into()).collect();

        dojo_types::schema::ModelMetadata {
            schema: (&value.schema).into(),
            name: unsafe {
                CString::from_raw(value.name as *mut c_char)
                    .into_string()
                    .unwrap()
            },
            packed_size: value.packed_size,
            unpacked_size: value.unpacked_size,
            class_hash: (&value.class_hash).into(),
            layout,
        }
    }
}
