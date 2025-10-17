use std::ffi::{c_char, CStr, CString};

use chrono::DateTime;
use crypto_bigint::Encoding;
use dojo_types::naming::compute_selector_from_names;
use starknet::core::utils::get_selector_from_name;
use torii_client::Client;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Page<T> {
    pub items: CArray<T>,
    pub next_cursor: COption<*const c_char>,
}

impl<T, U> From<torii_proto::Page<T>> for Page<U>
where
    U: From<T>,
{
    fn from(val: torii_proto::Page<T>) -> Self {
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
pub struct Controller {
    pub address: FieldElement,
    pub username: *const c_char,
    pub deployed_at_timestamp: u64,
}

impl From<torii_proto::Controller> for Controller {
    fn from(val: torii_proto::Controller) -> Self {
        Controller {
            address: val.address.into(),
            username: CString::new(val.username.clone()).unwrap().into_raw(),
            deployed_at_timestamp: val.deployed_at.timestamp() as u64,
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Token {
    pub contract_address: FieldElement,
    pub token_id: COption<U256>,
    pub name: *const c_char,
    pub symbol: *const c_char,
    pub decimals: u8,
    pub metadata: *const c_char,
    pub total_supply: COption<U256>,
}

impl From<torii_proto::Token> for Token {
    fn from(val: torii_proto::Token) -> Self {
        Token {
            token_id: val.token_id.into(),
            contract_address: val.contract_address.into(),
            name: CString::new(val.name.clone()).unwrap().into_raw(),
            symbol: CString::new(val.symbol.clone()).unwrap().into_raw(),
            decimals: val.decimals,
            metadata: CString::new(val.metadata.clone()).unwrap().into_raw(),
            total_supply: val.total_supply.into(),
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct TokenBalance {
    pub balance: U256,
    pub account_address: FieldElement,
    pub contract_address: FieldElement,
    pub token_id: COption<U256>,
}

impl From<torii_proto::TokenBalance> for TokenBalance {
    fn from(val: torii_proto::TokenBalance) -> Self {
        TokenBalance {
            balance: val.balance.into(),
            account_address: val.account_address.into(),
            contract_address: val.contract_address.into(),
            token_id: val.token_id.into(),
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct TokenContract {
    pub contract_address: FieldElement,
    pub name: *const c_char,
    pub symbol: *const c_char,
    pub decimals: u8,
    pub metadata: *const c_char,
    pub token_metadata: *const c_char,
    pub total_supply: COption<U256>,
}

impl From<torii_proto::TokenContract> for TokenContract {
    fn from(value: torii_proto::TokenContract) -> Self {
        Self {
            contract_address: value.contract_address.into(),
            name: CString::new(value.name.clone()).unwrap().into_raw(),
            symbol: CString::new(value.symbol.clone()).unwrap().into_raw(),
            decimals: value.decimals,
            token_metadata: CString::new(value.token_metadata.clone()).unwrap().into_raw(),
            total_supply: value.total_supply.into(),
            metadata: CString::new(value.metadata.clone()).unwrap().into_raw(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub enum ContractType {
    WORLD,
    ERC20,
    ERC721,
    ERC1155,
    UDC,
    OTHER,
}

impl From<torii_proto::ContractType> for ContractType {
    fn from(val: torii_proto::ContractType) -> Self {
        match val {
            torii_proto::ContractType::WORLD => ContractType::WORLD,
            torii_proto::ContractType::ERC20 => ContractType::ERC20,
            torii_proto::ContractType::ERC721 => ContractType::ERC721,
            torii_proto::ContractType::ERC1155 => ContractType::ERC1155,
            torii_proto::ContractType::UDC => ContractType::UDC,
            torii_proto::ContractType::OTHER => ContractType::OTHER,
        }
    }
}

impl From<ContractType> for torii_proto::ContractType {
    fn from(val: ContractType) -> Self {
        match val {
            ContractType::WORLD => torii_proto::ContractType::WORLD,
            ContractType::ERC20 => torii_proto::ContractType::ERC20,
            ContractType::ERC721 => torii_proto::ContractType::ERC721,
            ContractType::ERC1155 => torii_proto::ContractType::ERC1155,
            ContractType::UDC => torii_proto::ContractType::UDC,
            ContractType::OTHER => torii_proto::ContractType::OTHER,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Contract {
    pub contract_address: FieldElement,
    pub contract_type: ContractType,
    pub head: COption<u64>,
    pub tps: COption<u64>,
    pub last_block_timestamp: COption<u64>,
    pub last_pending_block_tx: COption<FieldElement>,
    pub updated_at: u64,
    pub created_at: u64,
}

impl From<torii_proto::Contract> for Contract {
    fn from(val: torii_proto::Contract) -> Self {
        Contract {
            contract_type: val.contract_type.into(),
            head: val.head.into(),
            tps: val.tps.into(),
            last_block_timestamp: val.last_block_timestamp.into(),
            last_pending_block_tx: val.last_pending_block_tx.into(),
            updated_at: val.updated_at.timestamp() as u64,
            created_at: val.created_at.timestamp() as u64,
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
    PreConfirmed,
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
            BlockTag::PreConfirmed => starknet::core::types::BlockTag::PreConfirmed,
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
pub struct ControllerQuery {
    pub pagination: Pagination,
    pub contract_addresses: CArray<FieldElement>,
    pub usernames: CArray<*const c_char>,
}

impl From<ControllerQuery> for torii_proto::ControllerQuery {
    fn from(val: ControllerQuery) -> Self {
        let usernames: Vec<*const c_char> = val.usernames.into();
        let usernames = usernames
            .into_iter()
            .map(|u| unsafe { CStr::from_ptr(u).to_string_lossy().to_string() })
            .collect::<Vec<String>>();
        torii_proto::ControllerQuery {
            pagination: val.pagination.into(),
            contract_addresses: val.contract_addresses.into(),
            usernames,
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct AttributeFilter {
    pub trait_name: *const c_char,
    pub trait_value: *const c_char,
}

impl From<AttributeFilter> for torii_proto::TokenAttributeFilter {
    fn from(val: AttributeFilter) -> Self {
        torii_proto::TokenAttributeFilter {
            trait_name: unsafe { CStr::from_ptr(val.trait_name).to_string_lossy().to_string() },
            trait_value: unsafe { CStr::from_ptr(val.trait_value).to_string_lossy().to_string() },
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct TokenQuery {
    pub contract_addresses: CArray<FieldElement>,
    pub token_ids: CArray<U256>,
    pub attribute_filters: CArray<AttributeFilter>,
    pub pagination: Pagination,
}

impl From<TokenQuery> for torii_proto::TokenQuery {
    fn from(val: TokenQuery) -> Self {
        torii_proto::TokenQuery {
            contract_addresses: val.contract_addresses.into(),
            token_ids: val.token_ids.into(),
            attribute_filters: val.attribute_filters.into(),
            pagination: val.pagination.into(),
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct TokenBalanceQuery {
    pub contract_addresses: CArray<FieldElement>,
    pub account_addresses: CArray<FieldElement>,
    pub token_ids: CArray<U256>,
    pub pagination: Pagination,
}

impl From<TokenBalanceQuery> for torii_proto::TokenBalanceQuery {
    fn from(val: TokenBalanceQuery) -> Self {
        torii_proto::TokenBalanceQuery {
            contract_addresses: val.contract_addresses.into(),
            account_addresses: val.account_addresses.into(),
            token_ids: val.token_ids.into(),
            pagination: val.pagination.into(),
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct TokenContractQuery {
    pub contract_addresses: CArray<FieldElement>,
    pub contract_types: CArray<ContractType>,
    pub pagination: Pagination,
}

impl From<TokenContractQuery> for torii_proto::TokenContractQuery {
    fn from(val: TokenContractQuery) -> Self {
        torii_proto::TokenContractQuery {
            contract_addresses: val.contract_addresses.into(),
            contract_types: val.contract_types.into(),
            pagination: val.pagination.into(),
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct TokenTransfer {
    pub id: *const c_char,
    pub contract_address: FieldElement,
    pub from_address: FieldElement,
    pub to_address: FieldElement,
    pub amount: U256,
    pub token_id: COption<U256>,
    pub executed_at: u64,
    pub event_id: COption<*const c_char>,
}

impl From<torii_proto::TokenTransfer> for TokenTransfer {
    fn from(val: torii_proto::TokenTransfer) -> Self {
        TokenTransfer {
            id: CString::new(val.id.clone()).unwrap().into_raw(),
            contract_address: val.contract_address.into(),
            from_address: val.from_address.into(),
            to_address: val.to_address.into(),
            amount: val.amount.into(),
            token_id: val.token_id.into(),
            executed_at: val.executed_at.timestamp() as u64,
            event_id: val
                .event_id
                .map(|e| CString::new(e).unwrap().into_raw() as *const c_char)
                .into(),
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct TokenTransferQuery {
    pub contract_addresses: CArray<FieldElement>,
    pub account_addresses: CArray<FieldElement>,
    pub token_ids: CArray<U256>,
    pub pagination: Pagination,
}

impl From<TokenTransferQuery> for torii_proto::TokenTransferQuery {
    fn from(val: TokenTransferQuery) -> Self {
        torii_proto::TokenTransferQuery {
            contract_addresses: val.contract_addresses.into(),
            account_addresses: val.account_addresses.into(),
            token_ids: val.token_ids.into(),
            pagination: val.pagination.into(),
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct TransactionFilter {
    pub transaction_hashes: CArray<FieldElement>,
    pub caller_addresses: CArray<FieldElement>,
    pub contract_addresses: CArray<FieldElement>,
    pub entrypoints: CArray<*const c_char>,
    pub model_selectors: CArray<FieldElement>,
    pub from_block: COption<u64>,
    pub to_block: COption<u64>,
}

impl From<TransactionFilter> for torii_proto::TransactionFilter {
    fn from(val: TransactionFilter) -> Self {
        let entrypoints: Vec<*const c_char> = val.entrypoints.into();
        let entrypoints = entrypoints
            .into_iter()
            .map(|e| unsafe { CStr::from_ptr(e).to_string_lossy().to_string() })
            .collect::<Vec<String>>();

        torii_proto::TransactionFilter {
            transaction_hashes: val.transaction_hashes.into(),
            caller_addresses: val.caller_addresses.into(),
            contract_addresses: val.contract_addresses.into(),
            entrypoints,
            model_selectors: val.model_selectors.into(),
            from_block: val.from_block.into(),
            to_block: val.to_block.into(),
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct TransactionQuery {
    pub filter: COption<TransactionFilter>,
    pub pagination: Pagination,
}

impl From<TransactionQuery> for torii_proto::TransactionQuery {
    fn from(val: TransactionQuery) -> Self {
        torii_proto::TransactionQuery {
            filter: val.filter.map(|f| f.into()).into(),
            pagination: val.pagination.into(),
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct ContractQuery {
    pub contract_addresses: CArray<FieldElement>,
    pub contract_types: CArray<ContractType>,
}

impl From<ContractQuery> for torii_proto::ContractQuery {
    fn from(val: ContractQuery) -> Self {
        torii_proto::ContractQuery {
            contract_addresses: val.contract_addresses.into(),
            contract_types: val.contract_types.into(),
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Transaction {
    pub transaction_hash: FieldElement,
    pub sender_address: FieldElement,
    pub calldata: CArray<FieldElement>,
    pub max_fee: FieldElement,
    pub signature: CArray<FieldElement>,
    pub nonce: FieldElement,
    pub block_number: u64,
    pub transaction_type: *const c_char,
    pub block_timestamp: u64,
    pub calls: CArray<TransactionCall>,
    pub unique_models: CArray<FieldElement>,
}

impl From<torii_proto::Transaction> for Transaction {
    fn from(val: torii_proto::Transaction) -> Self {
        Transaction {
            transaction_hash: val.transaction_hash.into(),
            sender_address: val.sender_address.into(),
            calldata: val.calldata.into(),
            max_fee: val.max_fee.into(),
            signature: val.signature.into(),
            nonce: val.nonce.into(),
            block_number: val.block_number,
            transaction_type: CString::new(val.transaction_type).unwrap().into_raw(),
            block_timestamp: val.block_timestamp.timestamp() as u64,
            calls: val.calls.into(),
            unique_models: val.unique_models.into(),
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub enum CallType {
    Execute,
    ExecuteFromOutside,
}

impl From<torii_proto::CallType> for CallType {
    fn from(val: torii_proto::CallType) -> Self {
        match val {
            torii_proto::CallType::Execute => CallType::Execute,
            torii_proto::CallType::ExecuteFromOutside => CallType::ExecuteFromOutside,
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct TransactionCall {
    pub contract_address: FieldElement,
    pub entrypoint: *const c_char,
    pub calldata: CArray<FieldElement>,
    pub call_type: CallType,
    pub caller_address: FieldElement,
}

impl From<torii_proto::TransactionCall> for TransactionCall {
    fn from(val: torii_proto::TransactionCall) -> Self {
        TransactionCall {
            contract_address: val.contract_address.into(),
            entrypoint: CString::new(val.entrypoint).unwrap().into_raw(),
            calldata: val.calldata.into(),
            call_type: val.call_type.into(),
            caller_address: val.caller_address.into(),
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Query {
    pub world_addresses: CArray<FieldElement>,
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
    pub limit: COption<u32>,
    pub direction: PaginationDirection,
    pub order_by: CArray<OrderBy>,
}

impl From<Pagination> for torii_proto::Pagination {
    fn from(val: Pagination) -> Self {
        torii_proto::Pagination {
            cursor: val
                .cursor
                .map(|c| unsafe { CStr::from_ptr(c).to_string_lossy().to_string() })
                .into(),
            limit: val.limit.into(),
            direction: val.direction.into(),
            order_by: val.order_by.into(),
        }
    }
}

impl From<torii_proto::Pagination> for Pagination {
    fn from(val: torii_proto::Pagination) -> Self {
        Pagination {
            cursor: val.cursor.map(|c| CString::new(c).unwrap().into_raw() as *const c_char).into(),
            limit: val.limit.into(),
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

impl From<PaginationDirection> for torii_proto::PaginationDirection {
    fn from(val: PaginationDirection) -> Self {
        match val {
            PaginationDirection::Forward => torii_proto::PaginationDirection::Forward,
            PaginationDirection::Backward => torii_proto::PaginationDirection::Backward,
        }
    }
}

impl From<torii_proto::PaginationDirection> for PaginationDirection {
    fn from(val: torii_proto::PaginationDirection) -> Self {
        match val {
            torii_proto::PaginationDirection::Forward => PaginationDirection::Forward,
            torii_proto::PaginationDirection::Backward => PaginationDirection::Backward,
        }
    }
}
#[derive(Clone, Debug)]
#[repr(C)]
pub struct OrderBy {
    pub field: *const c_char,
    pub direction: OrderDirection,
}

impl From<OrderBy> for torii_proto::OrderBy {
    fn from(val: OrderBy) -> Self {
        torii_proto::OrderBy {
            field: unsafe { CStr::from_ptr(val.field).to_string_lossy().to_string() },
            direction: val.direction.into(),
        }
    }
}

impl From<torii_proto::OrderBy> for OrderBy {
    fn from(val: torii_proto::OrderBy) -> Self {
        OrderBy {
            field: CString::new(val.field.clone()).unwrap().into_raw(),
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

impl From<OrderDirection> for torii_proto::OrderDirection {
    fn from(val: OrderDirection) -> Self {
        match val {
            OrderDirection::Asc => torii_proto::OrderDirection::Asc,
            OrderDirection::Desc => torii_proto::OrderDirection::Desc,
        }
    }
}

impl From<torii_proto::OrderDirection> for OrderDirection {
    fn from(val: torii_proto::OrderDirection) -> Self {
        match val {
            torii_proto::OrderDirection::Asc => OrderDirection::Asc,
            torii_proto::OrderDirection::Desc => OrderDirection::Desc,
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub enum Clause {
    HashedKeys(CArray<FieldElement>),
    Keys(KeysClause),
    CMember(MemberClause),
    Composite(CompositeClause),
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
pub enum MemberValue {
    PrimitiveValue(Primitive),
    String(*const c_char),
    List(CArray<MemberValue>),
}

impl From<MemberValue> for torii_proto::MemberValue {
    fn from(val: MemberValue) -> Self {
        match val {
            MemberValue::PrimitiveValue(primitive) => {
                torii_proto::MemberValue::Primitive(primitive.into())
            }
            MemberValue::String(string) => torii_proto::MemberValue::String(unsafe {
                CStr::from_ptr(string).to_string_lossy().to_string()
            }),
            MemberValue::List(list) => {
                let values: Vec<MemberValue> = list.into();
                let values =
                    values.into_iter().map(|v| v.into()).collect::<Vec<torii_proto::MemberValue>>();
                torii_proto::MemberValue::List(values)
            }
        }
    }
}

impl From<torii_proto::MemberValue> for MemberValue {
    fn from(val: torii_proto::MemberValue) -> Self {
        match val {
            torii_proto::MemberValue::Primitive(primitive) => {
                MemberValue::PrimitiveValue(primitive.into())
            }
            torii_proto::MemberValue::String(string) => {
                MemberValue::String(CString::new(string.clone()).unwrap().into_raw())
            }
            torii_proto::MemberValue::List(list) => {
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
    // Array-specific operators
    Contains,      // Array contains value
    ContainsAll,   // Array contains all values
    ContainsAny,   // Array contains any of the values
    ArrayLengthEq, // Array length equals
    ArrayLengthGt, // Array length greater than
    ArrayLengthLt, // Array length less than
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
    pub world_address: FieldElement,
    pub hashed_keys: FieldElement,
    pub models: CArray<Struct>,
    pub created_at: u64,
    pub updated_at: u64,
    pub executed_at: u64,
}

impl From<Entity> for torii_proto::schema::Entity {
    fn from(val: Entity) -> Self {
        let models: Vec<Struct> = val.models.into();
        let models = models.into_iter().map(|m| m.into()).collect();

        torii_proto::schema::Entity {
            world_address: val.world_address.into(),
            hashed_keys: val.hashed_keys.into(),
            models,
            created_at: DateTime::from_timestamp(val.created_at as i64, 0).unwrap(),
            updated_at: DateTime::from_timestamp(val.updated_at as i64, 0).unwrap(),
            executed_at: DateTime::from_timestamp(val.executed_at as i64, 0).unwrap(),
        }
    }
}

impl From<torii_proto::schema::Entity> for Entity {
    fn from(val: torii_proto::schema::Entity) -> Self {
        let models = val.models.into_iter().map(|m| m.into()).collect::<Vec<Struct>>();

        Entity {
            world_address: val.world_address.into(),
            hashed_keys: val.hashed_keys.into(),
            models: models.into(),
            created_at: val.created_at.timestamp() as u64,
            updated_at: val.updated_at.timestamp() as u64,
            executed_at: val.executed_at.timestamp() as u64,
        }
    }
}

impl From<ValueType> for torii_proto::ValueType {
    fn from(value: ValueType) -> Self {
        match value {
            ValueType::String(v) => {
                let v = unsafe { CStr::from_ptr(v).to_string_lossy().into_owned() };
                torii_proto::ValueType::String(v)
            }
            ValueType::Int(v) => torii_proto::ValueType::Int(v),
            ValueType::UInt(v) => torii_proto::ValueType::UInt(v),
            ValueType::VBool(v) => torii_proto::ValueType::Bool(v),
            ValueType::Bytes(v) => {
                let v = v.into();
                torii_proto::ValueType::Bytes(v)
            }
        }
    }
}

impl From<torii_proto::ValueType> for ValueType {
    fn from(value: torii_proto::ValueType) -> Self {
        match value {
            torii_proto::ValueType::String(v) => {
                let v = CString::new(v.clone()).unwrap().into_raw();
                ValueType::String(v)
            }
            torii_proto::ValueType::Int(v) => ValueType::Int(v),
            torii_proto::ValueType::UInt(v) => ValueType::UInt(v),
            torii_proto::ValueType::Bool(v) => ValueType::VBool(v),
            torii_proto::ValueType::Bytes(v) => {
                let v = v.clone().into();
                ValueType::Bytes(v)
            }
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct FixedSizeArray {
    pub array: CArray<Ty>,
    pub size: u32,
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
    FixedSizeArray_(FixedSizeArray),
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
            dojo_types::schema::Ty::FixedSizeArray((ty, size)) => {
                Ty::FixedSizeArray_(FixedSizeArray { array: ty.into(), size })
            }
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
            Ty::FixedSizeArray_(fixed_size_array) => dojo_types::schema::Ty::FixedSizeArray((
                fixed_size_array.array.into(),
                fixed_size_array.size,
            )),
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

impl From<Query> for torii_proto::Query {
    fn from(val: Query) -> Self {
        let models: Vec<String> = CStringArray(val.models).into();
        let clause = val.clause.map(|c| c.into()).into();

        torii_proto::Query {
            world_addresses: val.world_addresses.into(),
            pagination: val.pagination.into(),
            clause,
            models,
            no_hashed_keys: val.no_hashed_keys,
            historical: val.historical,
        }
    }
}

impl From<torii_proto::Query> for Query {
    fn from(val: torii_proto::Query) -> Self {
        let models = StringVec(val.models).into();

        Query {
            world_addresses: val.world_addresses.into(),
            pagination: val.pagination.into(),
            clause: val.clause.into(),
            models,
            no_hashed_keys: val.no_hashed_keys,
            historical: val.historical,
        }
    }
}

impl From<Clause> for torii_proto::Clause {
    fn from(val: Clause) -> Self {
        match val {
            Clause::HashedKeys(keys) => torii_proto::Clause::HashedKeys(keys.into()),
            Clause::Keys(keys) => torii_proto::Clause::Keys(keys.into()),
            Clause::CMember(member) => torii_proto::Clause::Member(member.into()),
            Clause::Composite(composite) => torii_proto::Clause::Composite(composite.into()),
        }
    }
}

impl From<torii_proto::Clause> for Clause {
    fn from(val: torii_proto::Clause) -> Self {
        match val {
            torii_proto::Clause::HashedKeys(keys) => Clause::HashedKeys(keys.into()),
            torii_proto::Clause::Keys(keys) => Clause::Keys(keys.into()),
            torii_proto::Clause::Member(member) => Clause::CMember(member.into()),
            torii_proto::Clause::Composite(composite) => Clause::Composite(composite.into()),
        }
    }
}

impl From<PatternMatching> for torii_proto::PatternMatching {
    fn from(val: PatternMatching) -> Self {
        match val {
            PatternMatching::FixedLen => torii_proto::PatternMatching::FixedLen,
            PatternMatching::VariableLen => torii_proto::PatternMatching::VariableLen,
        }
    }
}

impl From<torii_proto::PatternMatching> for PatternMatching {
    fn from(val: torii_proto::PatternMatching) -> Self {
        match val {
            torii_proto::PatternMatching::FixedLen => PatternMatching::FixedLen,
            torii_proto::PatternMatching::VariableLen => PatternMatching::VariableLen,
        }
    }
}

impl From<KeysClause> for torii_proto::KeysClause {
    fn from(val: KeysClause) -> Self {
        let keys: Vec<Option<starknet_crypto::Felt>> = COptionArray(val.keys).into();
        let models: Vec<String> = CStringArray(val.models).into();

        torii_proto::KeysClause { keys, pattern_matching: val.pattern_matching.into(), models }
    }
}

impl From<torii_proto::KeysClause> for KeysClause {
    fn from(val: torii_proto::KeysClause) -> Self {
        KeysClause {
            models: StringVec(val.models).into(),
            keys: val.keys.into(),
            pattern_matching: val.pattern_matching.into(),
        }
    }
}

impl From<MemberClause> for torii_proto::MemberClause {
    fn from(val: MemberClause) -> Self {
        torii_proto::MemberClause {
            member: unsafe { CString::from_raw(val.member as *mut c_char).into_string().unwrap() },
            model: unsafe { CString::from_raw(val.model as *mut c_char).into_string().unwrap() },
            operator: val.operator.into(),
            value: val.value.into(),
        }
    }
}

impl From<torii_proto::MemberClause> for MemberClause {
    fn from(val: torii_proto::MemberClause) -> Self {
        MemberClause {
            model: CString::new(val.model.clone()).unwrap().into_raw(),
            member: CString::new(val.member.clone()).unwrap().into_raw(),
            operator: val.operator.into(),
            value: val.value.into(),
        }
    }
}

impl From<CompositeClause> for torii_proto::CompositeClause {
    fn from(val: CompositeClause) -> Self {
        let operator = val.operator.into();
        let clauses = val.clauses.into();

        torii_proto::CompositeClause { operator, clauses }
    }
}

impl From<torii_proto::CompositeClause> for CompositeClause {
    fn from(val: torii_proto::CompositeClause) -> Self {
        let operator = val.operator.into();
        let clauses = val.clauses.into();

        CompositeClause { operator, clauses }
    }
}

impl From<LogicalOperator> for torii_proto::LogicalOperator {
    fn from(val: LogicalOperator) -> Self {
        match val {
            LogicalOperator::And => torii_proto::LogicalOperator::And,
            LogicalOperator::Or => torii_proto::LogicalOperator::Or,
        }
    }
}

impl From<torii_proto::LogicalOperator> for LogicalOperator {
    fn from(val: torii_proto::LogicalOperator) -> Self {
        match val {
            torii_proto::LogicalOperator::And => LogicalOperator::And,
            torii_proto::LogicalOperator::Or => LogicalOperator::Or,
        }
    }
}

impl From<ComparisonOperator> for torii_proto::ComparisonOperator {
    fn from(val: ComparisonOperator) -> Self {
        match val {
            ComparisonOperator::Eq => torii_proto::ComparisonOperator::Eq,
            ComparisonOperator::Neq => torii_proto::ComparisonOperator::Neq,
            ComparisonOperator::Gt => torii_proto::ComparisonOperator::Gt,
            ComparisonOperator::Gte => torii_proto::ComparisonOperator::Gte,
            ComparisonOperator::Lt => torii_proto::ComparisonOperator::Lt,
            ComparisonOperator::Lte => torii_proto::ComparisonOperator::Lte,
            ComparisonOperator::In => torii_proto::ComparisonOperator::In,
            ComparisonOperator::NotIn => torii_proto::ComparisonOperator::NotIn,
            ComparisonOperator::Contains => torii_proto::ComparisonOperator::Contains,
            ComparisonOperator::ContainsAll => torii_proto::ComparisonOperator::ContainsAll,
            ComparisonOperator::ContainsAny => torii_proto::ComparisonOperator::ContainsAny,
            ComparisonOperator::ArrayLengthEq => torii_proto::ComparisonOperator::ArrayLengthEq,
            ComparisonOperator::ArrayLengthGt => torii_proto::ComparisonOperator::ArrayLengthGt,
            ComparisonOperator::ArrayLengthLt => torii_proto::ComparisonOperator::ArrayLengthLt,
        }
    }
}

impl From<torii_proto::ComparisonOperator> for ComparisonOperator {
    fn from(val: torii_proto::ComparisonOperator) -> Self {
        match val {
            torii_proto::ComparisonOperator::Eq => ComparisonOperator::Eq,
            torii_proto::ComparisonOperator::Neq => ComparisonOperator::Neq,
            torii_proto::ComparisonOperator::Gt => ComparisonOperator::Gt,
            torii_proto::ComparisonOperator::Gte => ComparisonOperator::Gte,
            torii_proto::ComparisonOperator::Lt => ComparisonOperator::Lt,
            torii_proto::ComparisonOperator::Lte => ComparisonOperator::Lte,
            torii_proto::ComparisonOperator::In => ComparisonOperator::In,
            torii_proto::ComparisonOperator::NotIn => ComparisonOperator::NotIn,
            torii_proto::ComparisonOperator::Contains => ComparisonOperator::Contains,
            torii_proto::ComparisonOperator::ContainsAll => ComparisonOperator::ContainsAll,
            torii_proto::ComparisonOperator::ContainsAny => ComparisonOperator::ContainsAny,
            torii_proto::ComparisonOperator::ArrayLengthEq => ComparisonOperator::ArrayLengthEq,
            torii_proto::ComparisonOperator::ArrayLengthGt => ComparisonOperator::ArrayLengthGt,
            torii_proto::ComparisonOperator::ArrayLengthLt => ComparisonOperator::ArrayLengthLt,
        }
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

impl From<torii_proto::Value> for Value {
    fn from(val: torii_proto::Value) -> Self {
        Value { primitive_type: val.primitive_type.into(), value_type: val.value_type.into() }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct World {
    pub world_address: FieldElement,
    pub models: CArray<Model>,
}

impl From<torii_proto::World> for World {
    fn from(value: torii_proto::World) -> Self {
        let models: Vec<Model> = value.models.into_values().map(|v| v.into()).collect();

        World { world_address: value.world_address.into(), models: models.into() }
    }
}

impl From<World> for torii_proto::World {
    fn from(value: World) -> Self {
        let models: Vec<torii_proto::Model> = value.models.into();
        let models = models
            .into_iter()
            .map(|m| (compute_selector_from_names(&m.namespace, &m.name), m))
            .collect();

        torii_proto::World { world_address: value.world_address.into(), models }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Model {
    pub world_address: FieldElement,
    pub schema: Ty,
    pub namespace: *const c_char,
    pub name: *const c_char,
    pub selector: FieldElement,
    pub packed_size: u32,
    pub unpacked_size: u32,
    pub class_hash: FieldElement,
    pub contract_address: FieldElement,
    pub layout: *const c_char,
    pub use_legacy_store: bool,
}

impl From<torii_proto::Model> for Model {
    fn from(value: torii_proto::Model) -> Self {
        let layout = serde_json::to_string(&value.layout).unwrap();
        let layout = CString::new(layout).unwrap().into_raw();

        Model {
            world_address: value.world_address.into(),
            schema: value.schema.into(),
            name: CString::new(value.name.clone()).unwrap().into_raw(),
            namespace: CString::new(value.namespace.clone()).unwrap().into_raw(),
            selector: value.selector.into(),
            packed_size: value.packed_size,
            unpacked_size: value.unpacked_size,
            class_hash: value.class_hash.into(),
            contract_address: value.contract_address.into(),
            layout,
            use_legacy_store: value.use_legacy_store,
        }
    }
}

impl From<Model> for torii_proto::Model {
    fn from(value: Model) -> Self {
        let layout = unsafe { CStr::from_ptr(value.layout).to_string_lossy().into_owned() };
        let layout = serde_json::from_str(&layout).unwrap();

        torii_proto::Model {
            world_address: value.world_address.into(),
            schema: value.schema.into(),
            namespace: unsafe {
                CString::from_raw(value.namespace as *mut c_char).into_string().unwrap()
            },
            name: unsafe { CString::from_raw(value.name as *mut c_char).into_string().unwrap() },
            selector: value.selector.into(),
            packed_size: value.packed_size,
            unpacked_size: value.unpacked_size,
            class_hash: value.class_hash.into(),
            contract_address: value.contract_address.into(),
            layout,
            use_legacy_store: value.use_legacy_store,
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

impl From<Event> for torii_proto::Event {
    fn from(val: Event) -> Self {
        let keys: Vec<_> = val.keys.into();
        let data: Vec<_> = val.data.into();

        torii_proto::Event { keys, data, transaction_hash: val.transaction_hash.into() }
    }
}

impl From<torii_proto::Event> for Event {
    fn from(val: torii_proto::Event) -> Self {
        let keys = val.keys.into_iter().map(|k| k.into()).collect::<Vec<FieldElement>>();
        let data = val.data.into_iter().map(|k| k.into()).collect::<Vec<FieldElement>>();

        Event {
            keys: keys.into(),
            data: data.into(),
            transaction_hash: val.transaction_hash.into(),
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Message {
    pub message: *const c_char,
    pub signature: CArray<FieldElement>,
    pub world_address: FieldElement,
}

impl From<Message> for torii_proto::Message {
    fn from(val: Message) -> Self {
        let message = unsafe { CStr::from_ptr(val.message).to_string_lossy().into_owned() };
        let signature_slice =
            unsafe { std::slice::from_raw_parts(val.signature.data, val.signature.data_len) };
        let signature = signature_slice.iter().map(|f| f.clone().into()).collect();

        torii_proto::Message { message, signature, world_address: val.world_address.into() }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct AggregationQuery {
    pub aggregator_ids: CArray<*const c_char>,
    pub entity_ids: CArray<*const c_char>,
    pub pagination: Pagination,
}

impl From<AggregationQuery> for torii_proto::AggregationQuery {
    fn from(val: AggregationQuery) -> Self {
        let aggregator_ids: Vec<*const c_char> = val.aggregator_ids.into();
        let aggregator_ids = aggregator_ids
            .into_iter()
            .map(|id| unsafe { CStr::from_ptr(id).to_string_lossy().to_string() })
            .collect::<Vec<String>>();

        let entity_ids: Vec<*const c_char> = val.entity_ids.into();
        let entity_ids = entity_ids
            .into_iter()
            .map(|id| unsafe { CStr::from_ptr(id).to_string_lossy().to_string() })
            .collect::<Vec<String>>();

        torii_proto::AggregationQuery {
            aggregator_ids,
            entity_ids,
            pagination: val.pagination.into(),
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct ActivityQuery {
    pub world_addresses: CArray<FieldElement>,
    pub namespaces: CArray<*const c_char>,
    pub caller_addresses: CArray<FieldElement>,
    pub from_time: COption<u64>,
    pub to_time: COption<u64>,
    pub pagination: Pagination,
}

impl From<ActivityQuery> for torii_proto::ActivityQuery {
    fn from(val: ActivityQuery) -> Self {
        let namespaces: Vec<*const c_char> = val.namespaces.into();
        let namespaces = namespaces
            .into_iter()
            .map(|ns| unsafe { CStr::from_ptr(ns).to_string_lossy().to_string() })
            .collect::<Vec<String>>();

        torii_proto::ActivityQuery {
            world_addresses: val.world_addresses.into(),
            namespaces,
            caller_addresses: val.caller_addresses.into(),
            from_time: val.from_time.map(|t| DateTime::from_timestamp(t as i64, 0).unwrap()).into(),
            to_time: val.to_time.map(|t| DateTime::from_timestamp(t as i64, 0).unwrap()).into(),
            pagination: val.pagination.into(),
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct AggregationEntry {
    pub id: *const c_char,
    pub aggregator_id: *const c_char,
    pub entity_id: *const c_char,
    pub value: U256,
    pub display_value: *const c_char,
    pub position: u64,
    pub model_id: *const c_char,
    pub created_at: u64,
    pub updated_at: u64,
}

impl From<torii_proto::AggregationEntry> for AggregationEntry {
    fn from(val: torii_proto::AggregationEntry) -> Self {
        AggregationEntry {
            id: CString::new(val.id.clone()).unwrap().into_raw(),
            aggregator_id: CString::new(val.aggregator_id.clone()).unwrap().into_raw(),
            entity_id: CString::new(val.entity_id.clone()).unwrap().into_raw(),
            value: val.value.into(),
            display_value: CString::new(val.display_value.clone()).unwrap().into_raw(),
            position: val.position,
            model_id: CString::new(val.model_id.clone()).unwrap().into_raw(),
            created_at: val.created_at.timestamp() as u64,
            updated_at: val.updated_at.timestamp() as u64,
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Activity {
    pub id: *const c_char,
    pub world_address: FieldElement,
    pub namespace: *const c_char,
    pub caller_address: FieldElement,
    pub session_start: u64,
    pub session_end: u64,
    pub action_count: u32,
    pub actions: CArray<ActionCount>,
    pub updated_at: u64,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct ActionCount {
    pub action_name: *const c_char,
    pub count: u32,
}

impl From<torii_proto::Activity> for Activity {
    fn from(val: torii_proto::Activity) -> Self {
        let actions: Vec<ActionCount> = val
            .actions
            .into_iter()
            .map(|(name, count)| ActionCount {
                action_name: CString::new(name).unwrap().into_raw(),
                count,
            })
            .collect();

        Activity {
            id: CString::new(val.id).unwrap().into_raw(),
            world_address: val.world_address.into(),
            namespace: CString::new(val.namespace).unwrap().into_raw(),
            caller_address: val.caller_address.into(),
            session_start: val.session_start.timestamp() as u64,
            session_end: val.session_end.timestamp() as u64,
            action_count: val.action_count,
            actions: actions.into(),
            updated_at: val.updated_at.timestamp() as u64,
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct AchievementQuery {
    pub world_addresses: CArray<FieldElement>,
    pub namespaces: CArray<*const c_char>,
    pub hidden: COption<bool>,
    pub pagination: Pagination,
}

impl From<AchievementQuery> for torii_proto::AchievementQuery {
    fn from(val: AchievementQuery) -> Self {
        let namespaces: Vec<*const c_char> = val.namespaces.into();
        let namespaces = namespaces
            .into_iter()
            .map(|ns| unsafe { CStr::from_ptr(ns).to_string_lossy().to_string() })
            .collect::<Vec<String>>();

        torii_proto::AchievementQuery {
            world_addresses: val.world_addresses.into(),
            namespaces,
            hidden: val.hidden.into(),
            pagination: val.pagination.into(),
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct PlayerAchievementQuery {
    pub world_addresses: CArray<FieldElement>,
    pub namespaces: CArray<*const c_char>,
    pub player_addresses: CArray<FieldElement>,
    pub pagination: Pagination,
}

impl From<PlayerAchievementQuery> for torii_proto::PlayerAchievementQuery {
    fn from(val: PlayerAchievementQuery) -> Self {
        let namespaces: Vec<*const c_char> = val.namespaces.into();
        let namespaces = namespaces
            .into_iter()
            .map(|ns| unsafe { CStr::from_ptr(ns).to_string_lossy().to_string() })
            .collect::<Vec<String>>();

        torii_proto::PlayerAchievementQuery {
            world_addresses: val.world_addresses.into(),
            namespaces,
            player_addresses: val.player_addresses.into(),
            pagination: val.pagination.into(),
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Achievement {
    pub id: *const c_char,
    pub world_address: FieldElement,
    pub namespace: *const c_char,
    pub entity_id: *const c_char,
    pub hidden: bool,
    pub index: u32,
    pub points: u32,
    pub start: *const c_char,
    pub end: *const c_char,
    pub group: *const c_char,
    pub icon: *const c_char,
    pub title: *const c_char,
    pub description: *const c_char,
    pub tasks: CArray<AchievementTask>,
    pub data: *const c_char,
    pub total_completions: u32,
    pub completion_rate: f64,
    pub created_at: u64,
    pub updated_at: u64,
}

impl From<torii_proto::Achievement> for Achievement {
    fn from(val: torii_proto::Achievement) -> Self {
        let tasks: Vec<AchievementTask> = val.tasks.into_iter().map(|t| t.into()).collect();

        Achievement {
            id: CString::new(val.id).unwrap().into_raw(),
            world_address: val.world_address.into(),
            namespace: CString::new(val.namespace).unwrap().into_raw(),
            entity_id: CString::new(val.entity_id).unwrap().into_raw(),
            hidden: val.hidden,
            index: val.index,
            points: val.points,
            start: CString::new(val.start).unwrap().into_raw(),
            end: CString::new(val.end).unwrap().into_raw(),
            group: CString::new(val.group).unwrap().into_raw(),
            icon: CString::new(val.icon).unwrap().into_raw(),
            title: CString::new(val.title).unwrap().into_raw(),
            description: CString::new(val.description).unwrap().into_raw(),
            tasks: tasks.into(),
            data: CString::new(val.data.unwrap_or_default()).unwrap().into_raw(),
            total_completions: val.total_completions,
            completion_rate: val.completion_rate,
            created_at: val.created_at.timestamp() as u64,
            updated_at: val.updated_at.timestamp() as u64,
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct AchievementTask {
    pub task_id: *const c_char,
    pub description: *const c_char,
    pub total: u32,
    pub total_completions: u32,
    pub completion_rate: f64,
    pub created_at: u64,
}

impl From<torii_proto::AchievementTask> for AchievementTask {
    fn from(val: torii_proto::AchievementTask) -> Self {
        AchievementTask {
            task_id: CString::new(val.task_id).unwrap().into_raw(),
            description: CString::new(val.description).unwrap().into_raw(),
            total: val.total,
            total_completions: val.total_completions,
            completion_rate: val.completion_rate,
            created_at: val.created_at.timestamp() as u64,
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct PlayerAchievementEntry {
    pub player_address: FieldElement,
    pub stats: PlayerAchievementStats,
    pub achievements: CArray<PlayerAchievementProgress>,
}

impl From<torii_proto::PlayerAchievementEntry> for PlayerAchievementEntry {
    fn from(val: torii_proto::PlayerAchievementEntry) -> Self {
        let achievements: Vec<PlayerAchievementProgress> =
            val.achievements.into_iter().map(|a| a.into()).collect();

        PlayerAchievementEntry {
            player_address: val.player_address.into(),
            stats: val.stats.into(),
            achievements: achievements.into(),
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct PlayerAchievementStats {
    pub total_points: u32,
    pub completed_achievements: u32,
    pub total_achievements: u32,
    pub completion_percentage: f64,
    pub last_achievement_at: COption<u64>,
    pub created_at: u64,
    pub updated_at: u64,
}

impl From<torii_proto::PlayerAchievementStats> for PlayerAchievementStats {
    fn from(val: torii_proto::PlayerAchievementStats) -> Self {
        PlayerAchievementStats {
            total_points: val.total_points,
            completed_achievements: val.completed_achievements,
            total_achievements: val.total_achievements,
            completion_percentage: val.completion_percentage,
            last_achievement_at: val.last_achievement_at.map(|t| t.timestamp() as u64).into(),
            created_at: val.created_at.timestamp() as u64,
            updated_at: val.updated_at.timestamp() as u64,
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct PlayerAchievementProgress {
    pub achievement: Achievement,
    pub task_progress: CArray<TaskProgress>,
    pub completed: bool,
    pub progress_percentage: f64,
}

impl From<torii_proto::PlayerAchievementProgress> for PlayerAchievementProgress {
    fn from(val: torii_proto::PlayerAchievementProgress) -> Self {
        let task_progress: Vec<TaskProgress> =
            val.task_progress.into_iter().map(|t| t.into()).collect();

        PlayerAchievementProgress {
            achievement: val.achievement.into(),
            task_progress: task_progress.into(),
            completed: val.completed,
            progress_percentage: val.progress_percentage,
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct TaskProgress {
    pub task_id: *const c_char,
    pub count: u32,
    pub completed: bool,
}

impl From<torii_proto::TaskProgress> for TaskProgress {
    fn from(val: torii_proto::TaskProgress) -> Self {
        TaskProgress {
            task_id: CString::new(val.task_id).unwrap().into_raw(),
            count: val.count,
            completed: val.completed,
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct AchievementProgression {
    pub id: *const c_char,
    pub achievement_id: *const c_char,
    pub task_id: *const c_char,
    pub world_address: FieldElement,
    pub namespace: *const c_char,
    pub player_id: FieldElement,
    pub count: u32,
    pub completed: bool,
    pub completed_at: COption<u64>,
    pub created_at: u64,
    pub updated_at: u64,
}

impl From<torii_proto::AchievementProgression> for AchievementProgression {
    fn from(val: torii_proto::AchievementProgression) -> Self {
        AchievementProgression {
            id: CString::new(val.id).unwrap().into_raw(),
            achievement_id: CString::new(val.achievement_id).unwrap().into_raw(),
            task_id: CString::new(val.task_id).unwrap().into_raw(),
            world_address: val.world_address.into(),
            namespace: CString::new(val.namespace).unwrap().into_raw(),
            player_id: val.player_id.into(),
            count: val.count,
            completed: val.completed,
            completed_at: val.completed_at.map(|t| t.timestamp() as u64).into(),
            created_at: val.created_at.timestamp() as u64,
            updated_at: val.updated_at.timestamp() as u64,
        }
    }
}

// C-specific types for accounts and providers
pub struct Provider(
    pub(crate)  std::sync::Arc<
        starknet::providers::JsonRpcClient<starknet::providers::jsonrpc::HttpTransport>,
    >,
);

pub struct Account(
    pub(crate)  starknet::accounts::SingleOwnerAccount<
        std::sync::Arc<
            starknet::providers::JsonRpcClient<starknet::providers::jsonrpc::HttpTransport>,
        >,
        starknet::signers::LocalWallet,
    >,
);

pub struct Subscription {
    pub id: u64,
    pub(crate) trigger: stream_cancel::Trigger,
}
