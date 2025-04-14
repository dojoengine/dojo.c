use std::ffi::{c_char, CStr, CString};

use crypto_bigint::Encoding;
use starknet::core::utils::get_selector_from_name;
use torii_client::client::Client;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Page<T> {
    pub items: CArray<T>,
    pub next_cursor: COption<*const c_char>,
}

impl<T, U> From<torii_grpc::types::Page<T>> for Page<U>
where
    U: From<T>,
{
    fn from(val: torii_grpc::types::Page<T>) -> Self {
        let items = val.items.into_iter().map(|t| t.into()).collect::<Vec<U>>();
        Page {
            items: items.into(),
            next_cursor: val
                .next_cursor
                .map(|c| CString::new(c).unwrap().into_raw() as *const c_char)
                .into(),
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub enum Result<T> {
    Ok(T),
    #[allow(dead_code)]
    Err(Error),
}
#[derive(Debug, Clone)]
#[repr(C)]
pub enum COption<T> {
    Some(T),
    None,
}

impl<T> COption<T> {
    pub fn as_ref(&self) -> COption<&T> {
        match self {
            COption::Some(x) => COption::Some(x),
            COption::None => COption::None,
        }
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> COption<U> {
        match self {
            COption::Some(x) => COption::Some(f(x)),
            COption::None => COption::None,
        }
    }
}

impl<T, U> From<Option<T>> for COption<U>
where
    U: From<T>,
{
    fn from(val: Option<T>) -> Self {
        match val {
            Some(v) => COption::Some(v.into()),
            None => COption::None,
        }
    }
}

impl<T> From<COption<T>> for Option<T> {
    fn from(val: COption<T>) -> Self {
        match val {
            COption::Some(v) => Some(v),
            COption::None => None,
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Policy {
    pub target: FieldElement,
    pub method: *const c_char,
    pub description: *const c_char,
}

impl From<Policy> for crate::types::Policy {
    fn from(val: Policy) -> Self {
        crate::types::Policy {
            target: val.target.into(),
            method: unsafe { CStr::from_ptr(val.method).to_string_lossy().to_string() },
            description: unsafe { CStr::from_ptr(val.description).to_string_lossy().to_string() },
        }
    }
}

impl From<Policy> for account_sdk::account::session::policy::CallPolicy {
    fn from(val: Policy) -> Self {
        account_sdk::account::session::policy::CallPolicy {
            contract_address: val.target.into(),
            selector: get_selector_from_name(&unsafe {
                CStr::from_ptr(val.method).to_string_lossy().to_string()
            })
            .unwrap(),
            authorized: Some(true),
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Controller {
    pub address: FieldElement,
    pub username: *const c_char,
    pub deployed_at_timestamp: u64,
}

impl From<torii_grpc::types::Controller> for Controller {
    fn from(val: torii_grpc::types::Controller) -> Self {
        Controller {
            address: val.address.into(),
            username: CString::new(val.username.clone()).unwrap().into_raw(),
            deployed_at_timestamp: val.deployed_at,
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Token {
    pub contract_address: FieldElement,
    pub token_id: U256,
    pub name: *const c_char,
    pub symbol: *const c_char,
    pub decimals: u8,
    pub metadata: *const c_char,
}

impl From<torii_grpc::types::Token> for Token {
    fn from(val: torii_grpc::types::Token) -> Self {
        Token {
            token_id: val.token_id.into(),
            contract_address: val.contract_address.into(),
            name: CString::new(val.name.clone()).unwrap().into_raw(),
            symbol: CString::new(val.symbol.clone()).unwrap().into_raw(),
            decimals: val.decimals,
            metadata: CString::new(val.metadata.clone()).unwrap().into_raw(),
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct TokenBalance {
    pub balance: U256,
    pub account_address: FieldElement,
    pub contract_address: FieldElement,
    pub token_id: U256,
}

impl From<torii_grpc::types::TokenBalance> for TokenBalance {
    fn from(val: torii_grpc::types::TokenBalance) -> Self {
        TokenBalance {
            balance: val.balance.into(),
            account_address: val.account_address.into(),
            contract_address: val.contract_address.into(),
            token_id: val.token_id.into(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct IndexerUpdate {
    pub head: i64,
    pub tps: i64,
    pub last_block_timestamp: i64,
    pub contract_address: FieldElement,
}

impl From<IndexerUpdate> for torii_grpc::types::IndexerUpdate {
    fn from(val: IndexerUpdate) -> Self {
        torii_grpc::types::IndexerUpdate {
            head: val.head,
            tps: val.tps,
            last_block_timestamp: val.last_block_timestamp,
            contract_address: val.contract_address.into(),
        }
    }
}

impl From<torii_grpc::types::IndexerUpdate> for IndexerUpdate {
    fn from(val: torii_grpc::types::IndexerUpdate) -> Self {
        IndexerUpdate {
            head: val.head,
            tps: val.tps,
            last_block_timestamp: val.last_block_timestamp,
            contract_address: val.contract_address.into(),
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Signature {
    /// The `r` value of a signature
    pub r: FieldElement,
    /// The `s` value of a signature
    pub s: FieldElement,
}

impl From<Signature> for starknet::core::crypto::Signature {
    fn from(val: Signature) -> Self {
        Self { r: val.r.into(), s: val.s.into() }
    }
}

impl From<starknet::core::crypto::Signature> for Signature {
    fn from(val: starknet::core::crypto::Signature) -> Self {
        Signature { r: val.r.into(), s: val.s.into() }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Call {
    pub to: FieldElement,
    pub selector: *const c_char,
    pub calldata: CArray<FieldElement>,
}

/// Block hash, number or tag
#[derive(Debug, Clone)]
#[repr(C)]
pub enum BlockId {
    Hash(FieldElement),
    Number(u64),
    BlockTag_(BlockTag),
}

#[derive(Debug, Clone)]
#[repr(C)]
pub enum BlockTag {
    Latest,
    Pending,
}

impl From<BlockId> for starknet::core::types::BlockId {
    fn from(val: BlockId) -> Self {
        match val {
            BlockId::Hash(hash) => starknet::core::types::BlockId::Hash(hash.into()),
            BlockId::Number(number) => starknet::core::types::BlockId::Number(number),
            BlockId::BlockTag_(tag) => starknet::core::types::BlockId::Tag(tag.into()),
        }
    }
}

impl From<BlockTag> for starknet::core::types::BlockTag {
    fn from(val: BlockTag) -> Self {
        match val {
            BlockTag::Latest => starknet::core::types::BlockTag::Latest,
            BlockTag::Pending => starknet::core::types::BlockTag::Pending,
        }
    }
}

impl From<Call> for starknet::core::types::Call {
    fn from(val: Call) -> Self {
        let selector = unsafe { CStr::from_ptr(val.selector).to_string_lossy().to_string() };
        let calldata: Vec<_> = val.calldata.into();

        starknet::core::types::Call {
            to: val.to.into(),
            selector: get_selector_from_name(&selector).unwrap(),
            calldata,
        }
    }
}

impl From<Call> for starknet::core::types::FunctionCall {
    fn from(val: Call) -> Self {
        let selector = unsafe { CStr::from_ptr(val.selector).to_string_lossy().to_string() };
        let calldata: Vec<_> = val.calldata.into();

        starknet::core::types::FunctionCall {
            contract_address: val.to.into(),
            entry_point_selector: get_selector_from_name(&selector).unwrap(),
            calldata,
        }
    }
}

pub struct ToriiClient {
    pub inner: Client,
    pub logger: Option<extern "C" fn(*const c_char)>,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct CArray<T> {
    pub data: *mut T,
    pub data_len: usize,
}

impl<T, U> From<Vec<T>> for CArray<U>
where
    U: From<T>,
{
    fn from(val: Vec<T>) -> Self {
        let val = val.into_iter().map(|v| v.into()).collect::<Vec<U>>();
        let mut val = std::mem::ManuallyDrop::new(val);
        val.shrink_to_fit();

        CArray { data: val.as_mut_ptr(), data_len: val.len() }
    }
}

impl<T: Clone, U: From<T>> From<CArray<T>> for Vec<U> {
    fn from(val: CArray<T>) -> Self {
        let mut vec = Vec::with_capacity(val.data_len);
        unsafe {
            for i in 0..val.data_len {
                vec.push((*val.data.add(i)).clone().into());
            }
        }
        vec
    }
}

#[derive(Clone, Debug)]
pub struct COptionArray<T>(CArray<COption<T>>);

impl<T: Clone, U: From<T>> From<COptionArray<T>> for Vec<Option<U>> {
    fn from(val: COptionArray<T>) -> Self {
        let mut vec = Vec::with_capacity(val.0.data_len);
        unsafe {
            for i in 0..val.0.data_len {
                vec.push((*val.0.data.add(i)).clone().map(|v| v.into()).into());
            }
        }
        vec
    }
}

#[derive(Clone, Debug)]
pub struct StringVec(Vec<String>);

impl From<StringVec> for CArray<*const c_char> {
    fn from(val: StringVec) -> Self {
        let vec = val
            .0
            .into_iter()
            .map(|s| CString::new(s).unwrap().into_raw() as *const c_char)
            .collect::<Vec<_>>();

        vec.into()
    }
}

#[derive(Clone, Debug)]
pub struct CStringArray(CArray<*const c_char>);

impl From<CStringArray> for Vec<String> {
    fn from(val: CStringArray) -> Self {
        let mut vec = Vec::with_capacity(val.0.data_len);
        for i in 0..val.0.data_len {
            vec.push(unsafe { CStr::from_ptr(*val.0.data.add(i)).to_string_lossy().into_owned() });
        }
        vec
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct CHashItem<K, V> {
    pub key: K,
    pub value: V,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Error {
    pub message: *mut c_char,
}

// Implement conversion from std::error::Error to Error
impl<T> From<T> for Error
where
    T: std::error::Error,
{
    fn from(val: T) -> Self {
        Error { message: CString::new(val.to_string()).unwrap().into_raw() }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct U256 {
    data: [u8; 32],
}

impl From<U256> for crypto_bigint::U256 {
    fn from(val: U256) -> Self {
        crypto_bigint::U256::from_be_slice(&val.data)
    }
}

impl From<crypto_bigint::U256> for U256 {
    fn from(val: crypto_bigint::U256) -> Self {
        U256 { data: val.to_be_bytes() }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct FieldElement {
    data: [u8; 32],
}

impl From<FieldElement> for starknet::core::types::Felt {
    fn from(val: FieldElement) -> Self {
        starknet::core::types::Felt::from_bytes_be(&val.data)
    }
}

impl From<starknet::core::types::Felt> for FieldElement {
    fn from(val: starknet::core::types::Felt) -> Self {
        FieldElement { data: val.to_bytes_be() }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Query {
    pub pagination: Pagination,
    pub clause: COption<Clause>,
    pub no_hashed_keys: bool,
    pub models: CArray<*const c_char>,
    pub historical: bool,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Pagination {
    pub cursor: COption<*const c_char>,
    pub limit: u32,
    pub direction: PaginationDirection,
    pub order_by: CArray<OrderBy>,
}

impl From<Pagination> for torii_grpc::types::Pagination {
    fn from(val: Pagination) -> Self {
        torii_grpc::types::Pagination {
            cursor: val
                .cursor
                .map(|c| unsafe { CStr::from_ptr(c).to_string_lossy().to_string() })
                .into(),
            limit: val.limit,
            direction: val.direction.into(),
            order_by: val.order_by.into(),
        }
    }
}

impl From<torii_grpc::types::Pagination> for Pagination {
    fn from(val: torii_grpc::types::Pagination) -> Self {
        Pagination {
            cursor: val.cursor.map(|c| CString::new(c).unwrap().into_raw() as *const c_char).into(),
            limit: val.limit,
            direction: val.direction.into(),
            order_by: val.order_by.into(),
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub enum PaginationDirection {
    Forward,
    Backward,
}

impl From<PaginationDirection> for torii_grpc::types::PaginationDirection {
    fn from(val: PaginationDirection) -> Self {
        match val {
            PaginationDirection::Forward => torii_grpc::types::PaginationDirection::Forward,
            PaginationDirection::Backward => torii_grpc::types::PaginationDirection::Backward,
        }
    }
}

impl From<torii_grpc::types::PaginationDirection> for PaginationDirection {
    fn from(val: torii_grpc::types::PaginationDirection) -> Self {
        match val {
            torii_grpc::types::PaginationDirection::Forward => PaginationDirection::Forward,
            torii_grpc::types::PaginationDirection::Backward => PaginationDirection::Backward,
        }
    }
}
#[derive(Clone, Debug)]
#[repr(C)]
pub struct OrderBy {
    pub model: *const c_char,
    pub member: *const c_char,
    pub direction: OrderDirection,
}

impl From<OrderBy> for torii_grpc::types::OrderBy {
    fn from(val: OrderBy) -> Self {
        torii_grpc::types::OrderBy {
            model: unsafe { CStr::from_ptr(val.model).to_string_lossy().to_string() },
            member: unsafe { CStr::from_ptr(val.member).to_string_lossy().to_string() },
            direction: val.direction.into(),
        }
    }
}

impl From<torii_grpc::types::OrderBy> for OrderBy {
    fn from(val: torii_grpc::types::OrderBy) -> Self {
        OrderBy {
            model: CString::new(val.model.clone()).unwrap().into_raw(),
            member: CString::new(val.member.clone()).unwrap().into_raw(),
            direction: val.direction.into(),
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub enum OrderDirection {
    Asc,
    Desc,
}

impl From<OrderDirection> for torii_grpc::types::OrderDirection {
    fn from(val: OrderDirection) -> Self {
        match val {
            OrderDirection::Asc => torii_grpc::types::OrderDirection::Asc,
            OrderDirection::Desc => torii_grpc::types::OrderDirection::Desc,
        }
    }
}

impl From<torii_grpc::types::OrderDirection> for OrderDirection {
    fn from(val: torii_grpc::types::OrderDirection) -> Self {
        match val {
            torii_grpc::types::OrderDirection::Asc => OrderDirection::Asc,
            torii_grpc::types::OrderDirection::Desc => OrderDirection::Desc,
        }
    }
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
pub enum EntityKeysClause {
    HashedKeys(CArray<FieldElement>),
    EntityKeys(KeysClause),
}

#[derive(Clone, Debug)]
#[repr(C)]
pub enum PatternMatching {
    FixedLen = 0,
    VariableLen = 1,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct KeysClause {
    pub keys: CArray<COption<FieldElement>>,
    pub pattern_matching: PatternMatching,
    pub models: CArray<*const c_char>,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct ModelKeysClause {
    pub keys: CArray<FieldElement>,
    pub model: *const c_char,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub enum MemberValue {
    PrimitiveValue(Primitive),
    String(*const c_char),
    List(CArray<MemberValue>),
}

impl From<MemberValue> for torii_grpc::types::MemberValue {
    fn from(val: MemberValue) -> Self {
        match val {
            MemberValue::PrimitiveValue(primitive) => {
                torii_grpc::types::MemberValue::Primitive(primitive.into())
            }
            MemberValue::String(string) => torii_grpc::types::MemberValue::String(unsafe {
                CStr::from_ptr(string).to_string_lossy().to_string()
            }),
            MemberValue::List(list) => {
                let values: Vec<MemberValue> = list.into();
                let values = values
                    .into_iter()
                    .map(|v| v.into())
                    .collect::<Vec<torii_grpc::types::MemberValue>>();
                torii_grpc::types::MemberValue::List(values)
            }
        }
    }
}

impl From<torii_grpc::types::MemberValue> for MemberValue {
    fn from(val: torii_grpc::types::MemberValue) -> Self {
        match val {
            torii_grpc::types::MemberValue::Primitive(primitive) => {
                MemberValue::PrimitiveValue(primitive.into())
            }
            torii_grpc::types::MemberValue::String(string) => {
                MemberValue::String(CString::new(string.clone()).unwrap().into_raw())
            }
            torii_grpc::types::MemberValue::List(list) => {
                let values = list.into_iter().map(|v| v.into()).collect::<Vec<MemberValue>>();
                MemberValue::List(values.into())
            }
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct MemberClause {
    pub model: *const c_char,
    pub member: *const c_char,
    pub operator: ComparisonOperator,
    pub value: MemberValue,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct CompositeClause {
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
    In,
    NotIn,
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
    String(*const c_char),
    Int(i64),
    UInt(u64),
    VBool(bool),
    Bytes(CArray<u8>),
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Entity {
    pub hashed_keys: FieldElement,
    pub models: CArray<Struct>,
}

impl From<Entity> for torii_grpc::types::schema::Entity {
    fn from(val: Entity) -> Self {
        let models: Vec<Struct> = val.models.into();
        let models = models.into_iter().map(|m| m.into()).collect();

        torii_grpc::types::schema::Entity { hashed_keys: val.hashed_keys.into(), models }
    }
}

impl From<torii_grpc::types::schema::Entity> for Entity {
    fn from(val: torii_grpc::types::schema::Entity) -> Self {
        let models = val.models.into_iter().map(|m| m.into()).collect::<Vec<Struct>>();

        Entity { hashed_keys: val.hashed_keys.into(), models: models.into() }
    }
}

impl From<ValueType> for torii_grpc::types::ValueType {
    fn from(value: ValueType) -> Self {
        match value {
            ValueType::String(v) => {
                let v = unsafe { CStr::from_ptr(v).to_string_lossy().into_owned() };
                torii_grpc::types::ValueType::String(v)
            }
            ValueType::Int(v) => torii_grpc::types::ValueType::Int(v),
            ValueType::UInt(v) => torii_grpc::types::ValueType::UInt(v),
            ValueType::VBool(v) => torii_grpc::types::ValueType::Bool(v),
            ValueType::Bytes(v) => {
                let v = v.into();
                torii_grpc::types::ValueType::Bytes(v)
            }
        }
    }
}

impl From<torii_grpc::types::ValueType> for ValueType {
    fn from(value: torii_grpc::types::ValueType) -> Self {
        match value {
            torii_grpc::types::ValueType::String(v) => {
                let v = CString::new(v.clone()).unwrap().into_raw();
                ValueType::String(v)
            }
            torii_grpc::types::ValueType::Int(v) => ValueType::Int(v),
            torii_grpc::types::ValueType::UInt(v) => ValueType::UInt(v),
            torii_grpc::types::ValueType::Bool(v) => ValueType::VBool(v),
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
    Primitive_(Primitive),
    Struct_(Struct),
    Enum_(Enum),
    Tuple_(CArray<Ty>),
    Array_(CArray<Ty>),
    ByteArray(*const c_char),
}

impl From<dojo_types::schema::Ty> for Ty {
    fn from(value: dojo_types::schema::Ty) -> Self {
        match value {
            dojo_types::schema::Ty::Primitive(primitive) => {
                let primitive = primitive.into();

                Ty::Primitive_(primitive)
            }
            dojo_types::schema::Ty::Struct(struct_) => Ty::Struct_(struct_.into()),
            dojo_types::schema::Ty::Enum(enum_) => Ty::Enum_(enum_.into()),
            dojo_types::schema::Ty::Tuple(tuple) => Ty::Tuple_(tuple.into()),
            dojo_types::schema::Ty::Array(array) => Ty::Array_(array.into()),
            dojo_types::schema::Ty::ByteArray(array) => {
                let array = CString::new(array.clone()).unwrap().into_raw();
                Ty::ByteArray(array)
            }
        }
    }
}

// Implement opposite conversion
// use CString and other alike types to destruct the data
impl From<Ty> for dojo_types::schema::Ty {
    fn from(value: Ty) -> Self {
        match value {
            Ty::Primitive_(primitive) => dojo_types::schema::Ty::Primitive(primitive.into()),
            Ty::Struct_(struct_) => dojo_types::schema::Ty::Struct(struct_.into()),
            Ty::Enum_(enum_) => dojo_types::schema::Ty::Enum(enum_.into()),
            Ty::Tuple_(tuple) => dojo_types::schema::Ty::Tuple(tuple.into()),
            Ty::Array_(array) => dojo_types::schema::Ty::Array(array.into()),
            Ty::ByteArray(array) => {
                let array = unsafe { CStr::from_ptr(array).to_string_lossy().to_string() };
                dojo_types::schema::Ty::ByteArray(array)
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

impl From<Enum> for dojo_types::schema::Enum {
    fn from(value: Enum) -> Self {
        let options: Vec<EnumOption> = value.options.into();
        let options = options.into_iter().map(|o| o.into()).collect();

        dojo_types::schema::Enum {
            name: unsafe { CString::from_raw(value.name as *mut c_char).into_string().unwrap() },
            option: Some(value.option),
            options,
        }
    }
}

impl From<dojo_types::schema::Enum> for Enum {
    fn from(value: dojo_types::schema::Enum) -> Self {
        let options = value.options.into_iter().map(|o| o.into()).collect::<Vec<EnumOption>>();

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

impl From<EnumOption> for dojo_types::schema::EnumOption {
    fn from(value: EnumOption) -> Self {
        dojo_types::schema::EnumOption {
            name: unsafe { CString::from_raw(value.name as *mut c_char).into_string().unwrap() },
            ty: unsafe { (*Box::<Ty>::from_raw(value.ty)).into() },
        }
    }
}

impl From<dojo_types::schema::EnumOption> for EnumOption {
    fn from(value: dojo_types::schema::EnumOption) -> Self {
        EnumOption {
            name: CString::new(value.name.clone()).unwrap().into_raw(),
            ty: Box::into_raw(Box::new(value.ty.into())),
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Struct {
    pub name: *const c_char,
    pub children: CArray<Member>,
}

impl From<Struct> for dojo_types::schema::Struct {
    fn from(value: Struct) -> Self {
        let children: Vec<Member> = value.children.into();
        let children = children.into_iter().map(|c| c.into()).collect();

        dojo_types::schema::Struct {
            name: unsafe { CString::from_raw(value.name as *mut c_char).into_string().unwrap() },
            children,
        }
    }
}

impl From<dojo_types::schema::Struct> for Struct {
    fn from(value: dojo_types::schema::Struct) -> Self {
        let children = value.children.into_iter().map(|c| c.into()).collect::<Vec<Member>>();

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

impl From<Member> for dojo_types::schema::Member {
    fn from(value: Member) -> Self {
        dojo_types::schema::Member {
            name: unsafe { CString::from_raw(value.name as *mut c_char).into_string().unwrap() },
            ty: unsafe { (*Box::<Ty>::from_raw(value.ty)).into() },
            key: value.key,
        }
    }
}

impl From<dojo_types::schema::Member> for Member {
    fn from(value: dojo_types::schema::Member) -> Self {
        Member {
            name: CString::new(value.name.clone()).unwrap().into_raw(),
            ty: Box::into_raw(Box::new(value.ty.into())),
            key: value.key,
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub enum Primitive {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    // TODO: better way?
    I128([u8; 16]),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    // TODO: better way?
    U128([u8; 16]),
    U256_(U256),
    Bool(bool),
    Felt252(FieldElement),
    ClassHash(FieldElement),
    ContractAddress(FieldElement),
    EthAddress(FieldElement),
}

impl From<Primitive> for dojo_types::primitive::Primitive {
    fn from(value: Primitive) -> Self {
        match value {
            Primitive::I8(v) => dojo_types::primitive::Primitive::I8(Some(v)),
            Primitive::I16(v) => dojo_types::primitive::Primitive::I16(Some(v)),
            Primitive::I32(v) => dojo_types::primitive::Primitive::I32(Some(v)),
            Primitive::I64(v) => dojo_types::primitive::Primitive::I64(Some(v)),
            Primitive::I128(v) => {
                dojo_types::primitive::Primitive::I128(Some(i128::from_be_bytes(v)))
            }
            Primitive::U8(v) => dojo_types::primitive::Primitive::U8(Some(v)),
            Primitive::U16(v) => dojo_types::primitive::Primitive::U16(Some(v)),
            Primitive::U32(v) => dojo_types::primitive::Primitive::U32(Some(v)),
            Primitive::U64(v) => dojo_types::primitive::Primitive::U64(Some(v)),
            Primitive::U128(v) => {
                dojo_types::primitive::Primitive::U128(Some(u128::from_be_bytes(v)))
            }
            Primitive::U256_(v) => dojo_types::primitive::Primitive::U256(Some(v.into())),
            Primitive::Bool(v) => dojo_types::primitive::Primitive::Bool(Some(v)),
            Primitive::Felt252(v) => dojo_types::primitive::Primitive::Felt252(Some(v.into())),
            Primitive::ClassHash(v) => dojo_types::primitive::Primitive::ClassHash(Some(v.into())),
            Primitive::ContractAddress(v) => {
                dojo_types::primitive::Primitive::ContractAddress(Some(v.into()))
            }
            Primitive::EthAddress(v) => {
                dojo_types::primitive::Primitive::EthAddress(Some(v.into()))
            }
        }
    }
}

impl From<dojo_types::primitive::Primitive> for Primitive {
    fn from(value: dojo_types::primitive::Primitive) -> Self {
        match value {
            dojo_types::primitive::Primitive::I8(v) => Primitive::I8(v.unwrap_or(0)),
            dojo_types::primitive::Primitive::I16(v) => Primitive::I16(v.unwrap_or(0)),
            dojo_types::primitive::Primitive::I32(v) => Primitive::I32(v.unwrap_or(0)),
            dojo_types::primitive::Primitive::I64(v) => Primitive::I64(v.unwrap_or(0)),
            dojo_types::primitive::Primitive::I128(v) => {
                if let Some(v) = v {
                    Primitive::I128(v.to_be_bytes())
                } else {
                    Primitive::I128([0; 16])
                }
            }
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
                    Primitive::U256_(v.into())
                } else {
                    Primitive::U256_(U256 { data: [0; 32] })
                }
            }
            dojo_types::primitive::Primitive::Bool(v) => Primitive::Bool(v.unwrap_or(false)),
            dojo_types::primitive::Primitive::Felt252(v) => {
                if let Some(v) = v {
                    Primitive::Felt252(v.into())
                } else {
                    Primitive::Felt252(FieldElement { data: [0; 32] })
                }
            }
            dojo_types::primitive::Primitive::ClassHash(v) => {
                if let Some(v) = v {
                    Primitive::Felt252(v.into())
                } else {
                    Primitive::Felt252(FieldElement { data: [0; 32] })
                }
            }
            dojo_types::primitive::Primitive::ContractAddress(v) => {
                if let Some(v) = v {
                    Primitive::Felt252(v.into())
                } else {
                    Primitive::Felt252(FieldElement { data: [0; 32] })
                }
            }
            dojo_types::primitive::Primitive::EthAddress(v) => {
                if let Some(v) = v {
                    Primitive::EthAddress(v.into())
                } else {
                    Primitive::EthAddress(FieldElement { data: [0; 32] })
                }
            }
        }
    }
}

impl From<Query> for torii_grpc::types::Query {
    fn from(val: Query) -> Self {
        let models: Vec<String> = CStringArray(val.models).into();
        let clause = val.clause.map(|c| c.into()).into();

        torii_grpc::types::Query {
            pagination: val.pagination.into(),
            clause,
            models,
            no_hashed_keys: val.no_hashed_keys,
            historical: val.historical,
        }
    }
}

impl From<torii_grpc::types::Query> for Query {
    fn from(val: torii_grpc::types::Query) -> Self {
        let models = StringVec(val.models).into();

        Query {
            pagination: val.pagination.into(),
            clause: val.clause.into(),
            models,
            no_hashed_keys: val.no_hashed_keys,
            historical: val.historical,
        }
    }
}

impl From<Clause> for torii_grpc::types::Clause {
    fn from(val: Clause) -> Self {
        match val {
            Clause::Keys(keys) => torii_grpc::types::Clause::Keys(keys.into()),
            Clause::CMember(member) => torii_grpc::types::Clause::Member(member.into()),
            Clause::Composite(composite) => torii_grpc::types::Clause::Composite(composite.into()),
        }
    }
}

impl From<torii_grpc::types::Clause> for Clause {
    fn from(val: torii_grpc::types::Clause) -> Self {
        match val {
            torii_grpc::types::Clause::Keys(keys) => Clause::Keys(keys.into()),
            torii_grpc::types::Clause::Member(member) => Clause::CMember(member.into()),
            torii_grpc::types::Clause::Composite(composite) => Clause::Composite(composite.into()),
        }
    }
}

impl From<PatternMatching> for torii_grpc::types::PatternMatching {
    fn from(val: PatternMatching) -> Self {
        match val {
            PatternMatching::FixedLen => torii_grpc::types::PatternMatching::FixedLen,
            PatternMatching::VariableLen => torii_grpc::types::PatternMatching::VariableLen,
        }
    }
}

impl From<torii_grpc::types::PatternMatching> for PatternMatching {
    fn from(val: torii_grpc::types::PatternMatching) -> Self {
        match val {
            torii_grpc::types::PatternMatching::FixedLen => PatternMatching::FixedLen,
            torii_grpc::types::PatternMatching::VariableLen => PatternMatching::VariableLen,
        }
    }
}

impl From<EntityKeysClause> for torii_grpc::types::EntityKeysClause {
    fn from(val: EntityKeysClause) -> Self {
        match val {
            EntityKeysClause::HashedKeys(keys) => {
                let keys: Vec<FieldElement> = keys.into();
                let keys = keys.into_iter().map(|k| k.into()).collect();

                torii_grpc::types::EntityKeysClause::HashedKeys(keys)
            }
            EntityKeysClause::EntityKeys(keys) => {
                torii_grpc::types::EntityKeysClause::Keys(keys.into())
            }
        }
    }
}

impl From<torii_grpc::types::EntityKeysClause> for EntityKeysClause {
    fn from(val: torii_grpc::types::EntityKeysClause) -> Self {
        match val {
            torii_grpc::types::EntityKeysClause::HashedKeys(keys) => {
                let keys = keys.into_iter().map(|k| k.into()).collect::<Vec<FieldElement>>();
                EntityKeysClause::HashedKeys(keys.into())
            }
            torii_grpc::types::EntityKeysClause::Keys(keys) => {
                EntityKeysClause::EntityKeys(keys.into())
            }
        }
    }
}

impl From<KeysClause> for torii_grpc::types::KeysClause {
    fn from(val: KeysClause) -> Self {
        let keys: Vec<Option<starknet_crypto::Felt>> = COptionArray(val.keys).into();
        let models: Vec<String> = CStringArray(val.models).into();

        torii_grpc::types::KeysClause {
            keys,
            pattern_matching: val.pattern_matching.into(),
            models,
        }
    }
}

impl From<torii_grpc::types::KeysClause> for KeysClause {
    fn from(val: torii_grpc::types::KeysClause) -> Self {
        KeysClause {
            models: StringVec(val.models).into(),
            keys: val.keys.into(),
            pattern_matching: val.pattern_matching.into(),
        }
    }
}

impl From<ModelKeysClause> for torii_grpc::types::ModelKeysClause {
    fn from(val: ModelKeysClause) -> Self {
        let keys: Vec<_> = val.keys.into();

        torii_grpc::types::ModelKeysClause {
            model: unsafe { CStr::from_ptr(val.model).to_string_lossy().to_string() },
            keys,
        }
    }
}

impl From<torii_grpc::types::ModelKeysClause> for ModelKeysClause {
    fn from(val: torii_grpc::types::ModelKeysClause) -> Self {
        let keys = val.keys.into_iter().map(|k| k.into()).collect::<Vec<FieldElement>>();

        ModelKeysClause {
            model: CString::new(val.model.clone()).unwrap().into_raw(),
            keys: keys.into(),
        }
    }
}

impl From<MemberClause> for torii_grpc::types::MemberClause {
    fn from(val: MemberClause) -> Self {
        torii_grpc::types::MemberClause {
            member: unsafe { CString::from_raw(val.member as *mut c_char).into_string().unwrap() },
            model: unsafe { CString::from_raw(val.model as *mut c_char).into_string().unwrap() },
            operator: val.operator.into(),
            value: val.value.into(),
        }
    }
}

impl From<torii_grpc::types::MemberClause> for MemberClause {
    fn from(val: torii_grpc::types::MemberClause) -> Self {
        MemberClause {
            model: CString::new(val.model.clone()).unwrap().into_raw(),
            member: CString::new(val.member.clone()).unwrap().into_raw(),
            operator: val.operator.into(),
            value: val.value.into(),
        }
    }
}

impl From<CompositeClause> for torii_grpc::types::CompositeClause {
    fn from(val: CompositeClause) -> Self {
        let operator = val.operator.into();
        let clauses = val.clauses.into();

        torii_grpc::types::CompositeClause { operator, clauses }
    }
}

impl From<torii_grpc::types::CompositeClause> for CompositeClause {
    fn from(val: torii_grpc::types::CompositeClause) -> Self {
        let operator = val.operator.into();
        let clauses = val.clauses.into();

        CompositeClause { operator, clauses }
    }
}

impl From<LogicalOperator> for torii_grpc::types::LogicalOperator {
    fn from(val: LogicalOperator) -> Self {
        match val {
            LogicalOperator::And => torii_grpc::types::LogicalOperator::And,
            LogicalOperator::Or => torii_grpc::types::LogicalOperator::Or,
        }
    }
}

impl From<torii_grpc::types::LogicalOperator> for LogicalOperator {
    fn from(val: torii_grpc::types::LogicalOperator) -> Self {
        match val {
            torii_grpc::types::LogicalOperator::And => LogicalOperator::And,
            torii_grpc::types::LogicalOperator::Or => LogicalOperator::Or,
        }
    }
}

impl From<ComparisonOperator> for torii_grpc::types::ComparisonOperator {
    fn from(val: ComparisonOperator) -> Self {
        match val {
            ComparisonOperator::Eq => torii_grpc::types::ComparisonOperator::Eq,
            ComparisonOperator::Neq => torii_grpc::types::ComparisonOperator::Neq,
            ComparisonOperator::Gt => torii_grpc::types::ComparisonOperator::Gt,
            ComparisonOperator::Gte => torii_grpc::types::ComparisonOperator::Gte,
            ComparisonOperator::Lt => torii_grpc::types::ComparisonOperator::Lt,
            ComparisonOperator::Lte => torii_grpc::types::ComparisonOperator::Lte,
            ComparisonOperator::In => torii_grpc::types::ComparisonOperator::In,
            ComparisonOperator::NotIn => torii_grpc::types::ComparisonOperator::NotIn,
        }
    }
}

impl From<torii_grpc::types::ComparisonOperator> for ComparisonOperator {
    fn from(val: torii_grpc::types::ComparisonOperator) -> Self {
        match val {
            torii_grpc::types::ComparisonOperator::Eq => ComparisonOperator::Eq,
            torii_grpc::types::ComparisonOperator::Neq => ComparisonOperator::Neq,
            torii_grpc::types::ComparisonOperator::Gt => ComparisonOperator::Gt,
            torii_grpc::types::ComparisonOperator::Gte => ComparisonOperator::Gte,
            torii_grpc::types::ComparisonOperator::Lt => ComparisonOperator::Lt,
            torii_grpc::types::ComparisonOperator::Lte => ComparisonOperator::Lte,
            torii_grpc::types::ComparisonOperator::In => ComparisonOperator::In,
            torii_grpc::types::ComparisonOperator::NotIn => ComparisonOperator::NotIn,
        }
    }
}

impl From<Value> for torii_grpc::types::Value {
    fn from(val: Value) -> Self {
        torii_grpc::types::Value {
            primitive_type: val.primitive_type.into(),
            value_type: val.value_type.into(),
        }
    }
}

impl From<torii_grpc::types::Value> for Value {
    fn from(val: torii_grpc::types::Value) -> Self {
        Value { primitive_type: val.primitive_type.into(), value_type: val.value_type.into() }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct WorldMetadata {
    pub world_address: FieldElement,
    pub models: CArray<CHashItem<FieldElement, ModelMetadata>>,
}

impl From<dojo_types::WorldMetadata> for WorldMetadata {
    fn from(value: dojo_types::WorldMetadata) -> Self {
        let models = value
            .models
            .into_iter()
            .map(|(k, v)| CHashItem { key: k.into(), value: v.into() })
            .collect::<Vec<CHashItem<FieldElement, ModelMetadata>>>();

        WorldMetadata { world_address: value.world_address.into(), models: models.into() }
    }
}

impl From<WorldMetadata> for dojo_types::WorldMetadata {
    fn from(value: WorldMetadata) -> Self {
        let models: Vec<CHashItem<FieldElement, ModelMetadata>> = value.models.into();
        let models = models
            .into_iter()
            .map(|m| {
                let k = m.key.into();
                let v: dojo_types::schema::ModelMetadata = m.value.into();

                (k, v)
            })
            .collect();

        dojo_types::WorldMetadata { world_address: value.world_address.into(), models }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct ModelMetadata {
    pub schema: Ty,
    pub namespace: *const c_char,
    pub name: *const c_char,
    pub packed_size: u32,
    pub unpacked_size: u32,
    pub class_hash: FieldElement,
    pub contract_address: FieldElement,
    pub layout: CArray<FieldElement>,
}

impl From<dojo_types::schema::ModelMetadata> for ModelMetadata {
    fn from(value: dojo_types::schema::ModelMetadata) -> Self {
        let layout = value.layout.into_iter().map(|v| v.into()).collect::<Vec<FieldElement>>();

        ModelMetadata {
            schema: value.schema.into(),
            name: CString::new(value.name.clone()).unwrap().into_raw(),
            namespace: CString::new(value.namespace.clone()).unwrap().into_raw(),
            packed_size: value.packed_size,
            unpacked_size: value.unpacked_size,
            class_hash: value.class_hash.into(),
            contract_address: value.contract_address.into(),
            layout: layout.into(),
        }
    }
}

impl From<ModelMetadata> for dojo_types::schema::ModelMetadata {
    fn from(value: ModelMetadata) -> Self {
        let layout: Vec<FieldElement> = value.layout.into();

        let layout: Vec<starknet::core::types::Felt> =
            layout.into_iter().map(|v| v.into()).collect();

        dojo_types::schema::ModelMetadata {
            schema: value.schema.into(),
            namespace: unsafe {
                CString::from_raw(value.namespace as *mut c_char).into_string().unwrap()
            },
            name: unsafe { CString::from_raw(value.name as *mut c_char).into_string().unwrap() },
            packed_size: value.packed_size,
            unpacked_size: value.unpacked_size,
            class_hash: value.class_hash.into(),
            contract_address: value.contract_address.into(),
            layout,
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Event {
    pub keys: CArray<FieldElement>,
    pub data: CArray<FieldElement>,
    pub transaction_hash: FieldElement,
}

impl From<Event> for torii_grpc::types::Event {
    fn from(val: Event) -> Self {
        let keys: Vec<_> = val.keys.into();
        let data: Vec<_> = val.data.into();

        torii_grpc::types::Event { keys, data, transaction_hash: val.transaction_hash.into() }
    }
}

impl From<torii_grpc::types::Event> for Event {
    fn from(val: torii_grpc::types::Event) -> Self {
        let keys = val.keys.into_iter().map(|k| k.into()).collect::<Vec<FieldElement>>();
        let data = val.data.into_iter().map(|k| k.into()).collect::<Vec<FieldElement>>();

        Event {
            keys: keys.into(),
            data: data.into(),
            transaction_hash: val.transaction_hash.into(),
        }
    }
}
