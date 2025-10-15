// Core types - FieldElement, U256, Error, Pagination, Signature, Call, BlockId

// Newtype wrappers for custom type conversions
// These will be represented as strings in the UDL via [Custom] attribute

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldElement(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct U256(pub String);

// Custom type implementations for UniFFI
// These newtype wrappers will be represented as their inner type (String) in foreign languages
uniffi::custom_newtype!(FieldElement, String);
uniffi::custom_newtype!(U256, String);

// Helper functions to convert between internal types and UniFFI types
pub fn felt_to_field_element(felt: starknet::core::types::Felt) -> FieldElement {
    FieldElement(format!("0x{:064x}", felt))
}

pub fn field_element_to_felt(fe: &FieldElement) -> Result<starknet::core::types::Felt, DojoError> {
    starknet::core::types::Felt::from_hex(&fe.0).map_err(|_| DojoError::InvalidInput)
}

pub fn u256_to_uniffi(u: crypto_bigint::U256) -> U256 {
    U256(format!("0x{:064x}", u))
}

pub fn uniffi_to_u256(u: &U256) -> Result<crypto_bigint::U256, DojoError> {
    let s = u.0.strip_prefix("0x").unwrap_or(&u.0);
    let bytes = hex::decode(s).map_err(|_| DojoError::InvalidInput)?;
    Ok(crypto_bigint::U256::from_be_slice(&bytes))
}

// Error types
#[derive(Debug, thiserror::Error)]
pub enum DojoError {
    #[error("Client error: {0}")]
    ClientError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Invalid input")]
    InvalidInput,
    #[error("Connection error")]
    ConnectionError,
    #[error("Publish error")]
    PublishError,
    #[error("Query error")]
    QueryError,
    #[error("Subscription error")]
    SubscriptionError,
}

impl From<anyhow::Error> for DojoError {
    fn from(e: anyhow::Error) -> Self {
        DojoError::ClientError(e.to_string())
    }
}

// Pagination
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct OrderBy {
    pub field: String,
    pub direction: OrderDirection,
}

impl From<OrderBy> for torii_proto::OrderBy {
    fn from(val: OrderBy) -> Self {
        torii_proto::OrderBy { field: val.field, direction: val.direction.into() }
    }
}

impl From<torii_proto::OrderBy> for OrderBy {
    fn from(val: torii_proto::OrderBy) -> Self {
        OrderBy { field: val.field, direction: val.direction.into() }
    }
}

#[derive(Debug, Clone)]
pub struct Pagination {
    pub cursor: Option<String>,
    pub limit: Option<u32>,
    pub direction: PaginationDirection,
    pub order_by: Vec<OrderBy>,
}

impl From<Pagination> for torii_proto::Pagination {
    fn from(val: Pagination) -> Self {
        torii_proto::Pagination {
            cursor: val.cursor,
            limit: val.limit,
            direction: val.direction.into(),
            order_by: val.order_by.into_iter().map(|o| o.into()).collect(),
        }
    }
}

impl From<torii_proto::Pagination> for Pagination {
    fn from(val: torii_proto::Pagination) -> Self {
        Pagination {
            cursor: val.cursor,
            limit: val.limit,
            direction: val.direction.into(),
            order_by: val.order_by.into_iter().map(|o| o.into()).collect(),
        }
    }
}

// Note: Page<T> is not included in UniFFI as it doesn't support generics.
// If you need paginated results, use the specific query methods that return
// collections directly (e.g., Vec<Token>, Vec<Entity>, etc.)

// Signature
#[derive(Debug, Clone)]
pub struct Signature {
    pub r: FieldElement,
    pub s: FieldElement,
}

impl From<Signature> for starknet::core::crypto::Signature {
    fn from(val: Signature) -> Self {
        Self {
            r: field_element_to_felt(&val.r).unwrap(),
            s: field_element_to_felt(&val.s).unwrap(),
        }
    }
}

impl From<starknet::core::crypto::Signature> for Signature {
    fn from(val: starknet::core::crypto::Signature) -> Self {
        Signature { r: felt_to_field_element(val.r), s: felt_to_field_element(val.s) }
    }
}

// Call
#[derive(Debug, Clone)]
pub struct Call {
    pub to: FieldElement,
    pub selector: String,
    pub calldata: Vec<FieldElement>,
}

impl From<Call> for starknet::core::types::Call {
    fn from(val: Call) -> Self {
        starknet::core::types::Call {
            to: field_element_to_felt(&val.to).unwrap(),
            selector: starknet::core::utils::get_selector_from_name(&val.selector).unwrap(),
            calldata: val.calldata.into_iter().map(|f| field_element_to_felt(&f).unwrap()).collect(),
        }
    }
}

impl From<Call> for starknet::core::types::FunctionCall {
    fn from(val: Call) -> Self {
        starknet::core::types::FunctionCall {
            contract_address: field_element_to_felt(&val.to).unwrap(),
            entry_point_selector: starknet::core::utils::get_selector_from_name(&val.selector).unwrap(),
            calldata: val.calldata.into_iter().map(|f| field_element_to_felt(&f).unwrap()).collect(),
        }
    }
}

// BlockId and BlockTag
#[derive(Debug, Clone)]
pub enum BlockTag {
    Latest,
    PreConfirmed,
}

impl From<BlockTag> for starknet::core::types::BlockTag {
    fn from(val: BlockTag) -> Self {
        match val {
            BlockTag::Latest => starknet::core::types::BlockTag::Latest,
            BlockTag::PreConfirmed => starknet::core::types::BlockTag::PreConfirmed,
        }
    }
}

#[derive(Debug, Clone)]
pub enum BlockId {
    Hash(FieldElement),
    Number(u64),
    Tag(BlockTag),
}

impl From<BlockId> for starknet::core::types::BlockId {
    fn from(val: BlockId) -> Self {
        match val {
            BlockId::Hash(hash) => starknet::core::types::BlockId::Hash(field_element_to_felt(&hash).unwrap()),
            BlockId::Number(number) => starknet::core::types::BlockId::Number(number),
            BlockId::Tag(tag) => starknet::core::types::BlockId::Tag(tag.into()),
        }
    }
}

// Pagination wrapper types for different result types
// These are used by the client but defined here for availability

use super::controller::Controller;
use super::token::{Token, TokenBalance, TokenContract, TokenTransfer};
use super::transaction::Transaction;
use super::aggregation::AggregationEntry;
use super::activity::Activity;
use super::achievement::{Achievement, PlayerAchievementEntry};
use super::entity::Entity;
use super::event::Event;

#[derive(Debug, Clone)]
pub struct PageController {
    pub items: Vec<Controller>,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PageToken {
    pub items: Vec<Token>,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PageTokenBalance {
    pub items: Vec<TokenBalance>,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PageTokenContract {
    pub items: Vec<TokenContract>,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PageTokenTransfer {
    pub items: Vec<TokenTransfer>,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PageTransaction {
    pub items: Vec<Transaction>,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PageAggregationEntry {
    pub items: Vec<AggregationEntry>,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PageActivity {
    pub items: Vec<Activity>,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PageAchievement {
    pub items: Vec<Achievement>,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PagePlayerAchievement {
    pub items: Vec<PlayerAchievementEntry>,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PageEntity {
    pub items: Vec<Entity>,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PageEvent {
    pub items: Vec<Event>,
    pub next_cursor: Option<String>,
}
