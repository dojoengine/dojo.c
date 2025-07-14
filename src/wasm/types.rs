use std::collections::HashMap;
use std::str::FromStr;

use crypto_bigint::U256;
use dojo_types::schema::Struct;
use serde::{Deserialize, Serialize};
use starknet::core::types::FunctionCall;
use starknet::core::utils::get_selector_from_name;
use starknet_crypto::Felt;
use tsify_next::{declare, Tsify};
use wasm_bindgen::prelude::*;

use super::utils::{pad_to_hex, parse_ty_as_json_str};

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub next_cursor: Option<String>,
}

impl<T, U> From<torii_proto::Page<T>> for Page<U>
where
    U: From<T>,
{
    fn from(value: torii_proto::Page<T>) -> Self {
        Self {
            items: value.items.into_iter().map(|t| t.into()).collect(),
            next_cursor: value.next_cursor,
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct WasmU256(pub String);

impl From<WasmU256> for U256 {
    fn from(value: WasmU256) -> Self {
        U256::from_be_hex(pad_to_hex(value.0.as_str()).unwrap().as_str())
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Controllers(pub Page<Controller>);

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Controller {
    pub address: String,
    pub username: String,
    pub deployed_at_timestamp: u64,
}

impl From<torii_proto::Controller> for Controller {
    fn from(value: torii_proto::Controller) -> Self {
        Self {
            address: format!("{:#x}", value.address),
            username: value.username.clone(),
            deployed_at_timestamp: value.deployed_at.timestamp() as u64,
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Tokens(pub Page<Token>);

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TokenBalances(pub Page<TokenBalance>);

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TokenCollections(pub Page<TokenCollection>);

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Token {
    pub contract_address: String,
    pub token_id: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub metadata: String,
}

impl From<torii_proto::Token> for Token {
    fn from(value: torii_proto::Token) -> Self {
        Self {
            contract_address: format!("{:#x}", value.contract_address),
            token_id: format!("0x{:x}", value.token_id),
            name: value.name.clone(),
            symbol: value.symbol.clone(),
            decimals: value.decimals,
            metadata: value.metadata.clone(),
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TokenCollection {
    pub contract_address: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub count: u32,
    pub metadata: String,
}

impl From<torii_proto::TokenCollection> for TokenCollection {
    fn from(value: torii_proto::TokenCollection) -> Self {
        Self {
            contract_address: format!("{:#x}", value.contract_address),
            name: value.name.clone(),
            symbol: value.symbol.clone(),
            decimals: value.decimals,
            count: value.count,
            metadata: value.metadata.clone(),
        }
    }
}
impl From<torii_proto::Token> for TokenCollection {
    fn from(value: torii_proto::Token) -> Self {
        Self {
            contract_address: format!("{:#x}", value.contract_address),
            name: value.name.clone(),
            symbol: value.symbol.clone(),
            decimals: value.decimals,
            count: 0,
            metadata: value.metadata.clone(),
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TokenBalance {
    pub balance: String,
    pub account_address: String,
    pub contract_address: String,
    pub token_id: String,
}

impl From<torii_proto::TokenBalance> for TokenBalance {
    fn from(value: torii_proto::TokenBalance) -> Self {
        Self {
            balance: format!("0x{:x}", value.balance),
            account_address: format!("{:#x}", value.account_address),
            contract_address: format!("{:#x}", value.contract_address),
            token_id: format!("0x{:x}", value.token_id),
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TransactionFilter {
    pub transaction_hashes: Vec<String>,
    pub caller_addresses: Vec<String>,
    pub contract_addresses: Vec<String>,
    pub entrypoints: Vec<String>,
    pub model_selectors: Vec<String>,
    pub from_block: Option<u64>,
    pub to_block: Option<u64>,
}

impl From<TransactionFilter> for torii_proto::TransactionFilter {
    fn from(val: TransactionFilter) -> Self {
        torii_proto::TransactionFilter {
            transaction_hashes: val
                .transaction_hashes
                .into_iter()
                .map(|h| Felt::from_str(h.as_str()).unwrap())
                .collect(),
            caller_addresses: val
                .caller_addresses
                .into_iter()
                .map(|h| Felt::from_str(h.as_str()).unwrap())
                .collect(),
            contract_addresses: val
                .contract_addresses
                .into_iter()
                .map(|h| Felt::from_str(h.as_str()).unwrap())
                .collect(),
            entrypoints: val.entrypoints,
            model_selectors: val
                .model_selectors
                .into_iter()
                .map(|h| Felt::from_str(h.as_str()).unwrap())
                .collect(),
            from_block: val.from_block.into(),
            to_block: val.to_block.into(),
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TransactionQuery {
    pub filter: Option<TransactionFilter>,
    pub pagination: Pagination,
}

impl From<TransactionQuery> for torii_proto::TransactionQuery {
    fn from(val: TransactionQuery) -> Self {
        torii_proto::TransactionQuery {
            filter: val.filter.map(|f| f.into()),
            pagination: val.pagination.into(),
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Transactions(pub Page<Transaction>);

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Transaction {
    pub transaction_hash: String,
    pub sender_address: String,
    pub calldata: Vec<String>,
    pub max_fee: String,
    pub signature: Vec<String>,
    pub nonce: String,
    pub block_number: u64,
    pub transaction_type: String,
    pub block_timestamp: u64,
    pub calls: Vec<TransactionCall>,
    pub unique_models: Vec<String>,
}

impl From<torii_proto::Transaction> for Transaction {
    fn from(val: torii_proto::Transaction) -> Self {
        Transaction {
            transaction_hash: format!("{:#x}", val.transaction_hash),
            sender_address: format!("{:#x}", val.sender_address),
            calldata: val.calldata.into_iter().map(|c| format!("{:#x}", c)).collect(),
            max_fee: format!("{:#x}", val.max_fee),
            signature: val.signature.into_iter().map(|s| format!("{:#x}", s)).collect(),
            nonce: format!("{:#x}", val.nonce),
            block_number: val.block_number,
            transaction_type: val.transaction_type,
            block_timestamp: val.block_timestamp.timestamp() as u64,
            calls: val.calls.into_iter().map(|c| c.into()).collect(),
            unique_models: val.unique_models.into_iter().map(|m| format!("{:#x}", m)).collect(),
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
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

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TransactionCall {
    pub contract_address: String,
    pub entrypoint: String,
    pub calldata: Vec<String>,
    pub call_type: CallType,
    pub caller_address: String,
}

impl From<torii_proto::TransactionCall> for TransactionCall {
    fn from(val: torii_proto::TransactionCall) -> Self {
        TransactionCall {
            contract_address: format!("{:#x}", val.contract_address),
            entrypoint: val.entrypoint,
            calldata: val.calldata.into_iter().map(|c| format!("{:#x}", c)).collect(),
            call_type: val.call_type.into(),
            caller_address: format!("{:#x}", val.caller_address),
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ControllerQuery {
    pub contract_addresses: Vec<String>,
    pub usernames: Vec<String>,
    pub pagination: Pagination,
}

impl From<ControllerQuery> for torii_proto::ControllerQuery {
    fn from(value: ControllerQuery) -> Self {
        Self {
            contract_addresses: value
                .contract_addresses
                .into_iter()
                .map(|c| Felt::from_str(c.as_str()).unwrap())
                .collect(),
            usernames: value.usernames,
            pagination: value.pagination.into(),
        }
    }
}
#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TokenQuery {
    pub contract_addresses: Vec<String>,
    pub token_ids: Vec<String>,
    pub pagination: Pagination,
}

impl From<TokenQuery> for torii_proto::TokenQuery {
    fn from(value: TokenQuery) -> Self {
        Self {
            contract_addresses: value
                .contract_addresses
                .into_iter()
                .map(|c| Felt::from_str(c.as_str()).unwrap())
                .collect(),
            token_ids: value.token_ids.into_iter().map(|t| U256::from_be_hex(t.as_str())).collect(),
            pagination: value.pagination.into(),
        }
    }
}
#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TokenBalanceQuery {
    pub contract_addresses: Vec<String>,
    pub account_addresses: Vec<String>,
    pub token_ids: Vec<String>,
    pub pagination: Pagination,
}

impl From<TokenBalanceQuery> for torii_proto::TokenBalanceQuery {
    fn from(value: TokenBalanceQuery) -> Self {
        Self {
            contract_addresses: value
                .contract_addresses
                .into_iter()
                .map(|c| Felt::from_str(c.as_str()).unwrap())
                .collect(),
            account_addresses: value
                .account_addresses
                .into_iter()
                .map(|a| Felt::from_str(a.as_str()).unwrap())
                .collect(),
            token_ids: value.token_ids.into_iter().map(|t| U256::from_be_hex(t.as_str())).collect(),
            pagination: value.pagination.into(),
        }
    }
}
#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct IndexerUpdate {
    pub head: i64,
    pub tps: i64,
    pub last_block_timestamp: i64,
    pub contract_address: String,
}

impl From<IndexerUpdate> for torii_proto::IndexerUpdate {
    fn from(value: IndexerUpdate) -> Self {
        Self {
            head: value.head,
            tps: value.tps,
            last_block_timestamp: value.last_block_timestamp,
            contract_address: Felt::from_str(value.contract_address.as_str()).unwrap(),
        }
    }
}

impl From<torii_proto::IndexerUpdate> for IndexerUpdate {
    fn from(value: torii_proto::IndexerUpdate) -> Self {
        Self {
            head: value.head,
            tps: value.tps,
            last_block_timestamp: value.last_block_timestamp,
            contract_address: format!("{:#x}", value.contract_address),
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ClientConfig {
    #[serde(rename = "toriiUrl")]
    pub torii_url: String,
    #[serde(rename = "worldAddress")]
    pub world_address: String,
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi, hashmap_as_object)]
pub struct Ty {
    #[tsify(type = r#""primitive" | "struct" | "enum" | "array" | "tuple" | "bytearray""#)]
    pub r#type: String,
    pub type_name: String,
    #[serde(with = "serde_wasm_bindgen::preserve")]
    #[tsify(type = "boolean | number | string | Ty | Record<string, Ty> | Array<Ty> | { option: \
                    string, value: Ty } | null")]
    pub value: JsValue,
    pub key: bool,
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct EnumValue {
    pub option: String,
    pub value: Ty,
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Signature {
    pub r: String,
    pub s: String,
}

impl From<starknet::core::crypto::Signature> for Signature {
    fn from(value: starknet::core::crypto::Signature) -> Self {
        Self { r: format!("{:#x}", value.r), s: format!("{:#x}", value.s) }
    }
}

impl From<Signature> for starknet::core::crypto::Signature {
    fn from(value: Signature) -> Self {
        Self {
            r: Felt::from_str(value.r.as_str()).unwrap(),
            s: Felt::from_str(value.s.as_str()).unwrap(),
        }
    }
}

#[declare]
pub type Calls = Vec<Call>;

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi, hashmap_as_object)]
pub struct Model(pub HashMap<String, Ty>);

impl From<Struct> for Model {
    fn from(value: Struct) -> Self {
        Self(
            value
                .children
                .iter()
                .map(|c| (c.name.clone(), parse_ty_as_json_str(&c.ty, c.key)))
                .collect(),
        )
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi, hashmap_as_object)]
pub struct Entity {
    pub hashed_keys: String,
    pub models: HashMap<String, Model>,
}

impl From<torii_proto::schema::Entity> for Entity {
    fn from(value: torii_proto::schema::Entity) -> Self {
        Self {
            hashed_keys: format!("{:#x}", value.hashed_keys),
            models: value.models.into_iter().map(|m| (m.name.clone(), m.into())).collect(),
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi, hashmap_as_object)]
pub struct Entities(pub Page<Entity>);

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Call {
    pub to: String,
    pub selector: String,
    pub calldata: Vec<String>,
}

impl From<Call> for starknet::core::types::Call {
    fn from(value: Call) -> Self {
        Self {
            to: Felt::from_str(value.to.as_str()).unwrap(),
            selector: get_selector_from_name(value.selector.as_str()).unwrap(),
            calldata: value.calldata.iter().map(|c| Felt::from_str(c.as_str()).unwrap()).collect(),
        }
    }
}

impl From<Call> for FunctionCall {
    fn from(value: Call) -> Self {
        Self {
            contract_address: Felt::from_str(value.to.as_str()).unwrap(),
            entry_point_selector: get_selector_from_name(value.selector.as_str()).unwrap(),
            calldata: value.calldata.iter().map(|c| Felt::from_str(c.as_str()).unwrap()).collect(),
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum BlockTag {
    Latest,
    Pending,
}

impl From<BlockTag> for starknet::core::types::BlockTag {
    fn from(value: BlockTag) -> Self {
        match value {
            BlockTag::Latest => starknet::core::types::BlockTag::Latest,
            BlockTag::Pending => starknet::core::types::BlockTag::Pending,
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum BlockId {
    Hash(String),
    Number(u64),
    BlockTag(BlockTag),
}

impl From<BlockId> for starknet::core::types::BlockId {
    fn from(value: BlockId) -> Self {
        match value {
            BlockId::Hash(hash) => {
                starknet::core::types::BlockId::Hash(Felt::from_str(hash.as_str()).unwrap())
            }
            BlockId::Number(number) => starknet::core::types::BlockId::Number(number),
            BlockId::BlockTag(tag) => starknet::core::types::BlockId::Tag(tag.into()),
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Query {
    pub pagination: Pagination,
    pub clause: Option<Clause>,
    pub no_hashed_keys: bool,
    pub models: Vec<String>,
    pub historical: bool,
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Pagination {
    pub limit: Option<u32>,
    pub cursor: Option<String>,
    pub direction: PaginationDirection,
    pub order_by: Vec<OrderBy>,
}

impl From<Pagination> for torii_proto::Pagination {
    fn from(value: Pagination) -> Self {
        Self {
            limit: value.limit.map(|l| l as u32),
            cursor: value.cursor.map(|c| c.to_string()),
            direction: value.direction.into(),
            order_by: value.order_by.into_iter().map(|o| o.into()).collect(),
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum PaginationDirection {
    Forward,
    Backward,
}

impl From<PaginationDirection> for torii_proto::PaginationDirection {
    fn from(value: PaginationDirection) -> Self {
        match value {
            PaginationDirection::Forward => Self::Forward,
            PaginationDirection::Backward => Self::Backward,
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct OrderBy {
    pub field: String,
    pub direction: OrderDirection,
}

impl From<OrderBy> for torii_proto::OrderBy {
    fn from(value: OrderBy) -> Self {
        Self { field: value.field.clone(), direction: value.direction.into() }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum OrderDirection {
    Asc,
    Desc,
}

impl From<OrderDirection> for torii_proto::OrderDirection {
    fn from(value: OrderDirection) -> Self {
        match value {
            OrderDirection::Asc => Self::Asc,
            OrderDirection::Desc => Self::Desc,
        }
    }
}

impl From<Query> for torii_proto::Query {
    fn from(value: Query) -> Self {
        Self {
            pagination: value.pagination.into(),
            clause: value.clause.map(|c| c.into()),
            no_hashed_keys: value.no_hashed_keys,
            models: value.models,
            historical: value.historical,
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Clause {
    HashedKeys(Vec<String>),
    Keys(KeysClause),
    Member(MemberClause),
    Composite(CompositeClause),
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum PatternMatching {
    FixedLen = 0,
    VariableLen = 1,
}

impl From<PatternMatching> for torii_proto::PatternMatching {
    fn from(value: PatternMatching) -> Self {
        match value {
            PatternMatching::FixedLen => Self::FixedLen,
            PatternMatching::VariableLen => Self::VariableLen,
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct KeysClause {
    pub keys: Vec<Option<String>>,
    pub pattern_matching: PatternMatching,
    pub models: Vec<String>,
}

#[declare]
pub type KeysClauses = Vec<KeysClause>;

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum MemberValue {
    Primitive(Primitive),
    String(String),
    List(Vec<MemberValue>),
}

impl From<MemberValue> for torii_proto::MemberValue {
    fn from(value: MemberValue) -> Self {
        match value {
            MemberValue::Primitive(primitive) => {
                torii_proto::MemberValue::Primitive(primitive.into())
            }
            MemberValue::String(string) => torii_proto::MemberValue::String(string.clone()),
            MemberValue::List(list) => {
                let values =
                    list.into_iter().map(|v| v.into()).collect::<Vec<torii_proto::MemberValue>>();
                torii_proto::MemberValue::List(values)
            }
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct MemberClause {
    pub model: String,
    pub member: String,
    pub operator: ComparisonOperator,
    pub value: MemberValue,
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct CompositeClause {
    pub operator: LogicalOperator,
    pub clauses: Vec<Clause>,
}

impl From<KeysClause> for torii_proto::KeysClause {
    fn from(value: KeysClause) -> Self {
        Self {
            keys: value
                .keys
                .iter()
                .map(|o| o.as_ref().map(|k| Felt::from_str(k.as_str()).unwrap()))
                .collect(),
            models: value.models.iter().map(|m| m.to_string()).collect(),
            pattern_matching: value.pattern_matching.into(),
        }
    }
}

impl From<MemberClause> for torii_proto::MemberClause {
    fn from(value: MemberClause) -> Self {
        Self {
            model: value.model.to_string(),
            member: value.member.to_string(),
            operator: value.operator.into(),
            value: value.value.into(),
        }
    }
}

impl From<CompositeClause> for torii_proto::CompositeClause {
    fn from(value: CompositeClause) -> Self {
        Self {
            operator: value.operator.into(),
            clauses: value.clauses.into_iter().map(|c| c.into()).collect(),
        }
    }
}

impl From<Clause> for torii_proto::Clause {
    fn from(value: Clause) -> Self {
        match value {
            Clause::HashedKeys(keys) => {
                Self::HashedKeys(keys.iter().map(|k| Felt::from_str(k.as_str()).unwrap()).collect())
            }
            Clause::Keys(keys) => Self::Keys(keys.into()),
            Clause::Member(member) => Self::Member(member.into()),
            Clause::Composite(composite) => Self::Composite(composite.into()),
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum LogicalOperator {
    And,
    Or,
}

impl From<LogicalOperator> for torii_proto::LogicalOperator {
    fn from(value: LogicalOperator) -> Self {
        match value {
            LogicalOperator::And => Self::And,
            LogicalOperator::Or => Self::Or,
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
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

impl From<ComparisonOperator> for torii_proto::ComparisonOperator {
    fn from(value: ComparisonOperator) -> Self {
        match value {
            ComparisonOperator::Eq => Self::Eq,
            ComparisonOperator::Neq => Self::Neq,
            ComparisonOperator::Gt => Self::Gt,
            ComparisonOperator::Gte => Self::Gte,
            ComparisonOperator::Lt => Self::Lt,
            ComparisonOperator::Lte => Self::Lte,
            ComparisonOperator::In => Self::In,
            ComparisonOperator::NotIn => Self::NotIn,
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Value {
    pub primitive_type: Primitive,
    pub value_type: ValueType,
}

impl From<Value> for torii_proto::Value {
    fn from(value: Value) -> Self {
        Self { primitive_type: value.primitive_type.into(), value_type: value.value_type.into() }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum ValueType {
    String(String),
    Int(i64),
    UInt(u64),
    VBool(bool),
    Bytes(Vec<u8>),
}

impl From<ValueType> for torii_proto::ValueType {
    fn from(value: ValueType) -> Self {
        match &value {
            ValueType::String(s) => Self::String(s.to_string()),
            ValueType::Int(i) => Self::Int(*i),
            ValueType::UInt(u) => Self::UInt(*u),
            ValueType::VBool(b) => Self::Bool(*b),
            ValueType::Bytes(b) => Self::Bytes(b.to_vec()),
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Primitive {
    I8(Option<i8>),
    I16(Option<i16>),
    I32(Option<i32>),
    I64(Option<i64>),
    I128(Option<String>),
    U8(Option<u8>),
    U16(Option<u16>),
    U32(Option<u32>),
    U64(Option<u64>),
    U128(Option<String>),
    U256(Option<String>),
    Bool(Option<bool>),
    Felt252(Option<String>),
    ClassHash(Option<String>),
    ContractAddress(Option<String>),
    EthAddress(Option<String>),
}

impl From<Primitive> for dojo_types::primitive::Primitive {
    fn from(value: Primitive) -> Self {
        match value {
            Primitive::I8(Some(value)) => Self::I8(Some(value)),
            Primitive::I16(Some(value)) => Self::I16(Some(value)),
            Primitive::I32(Some(value)) => Self::I32(Some(value)),
            Primitive::I64(Some(value)) => Self::I64(Some(value)),
            Primitive::I128(Some(value)) => {
                Self::I128(Some(i128::from_str(value.as_str()).unwrap()))
            }
            Primitive::U8(Some(value)) => Self::U8(Some(value)),
            Primitive::U16(Some(value)) => Self::U16(Some(value)),
            Primitive::U32(Some(value)) => Self::U32(Some(value)),
            Primitive::U64(Some(value)) => Self::U64(Some(value)),
            Primitive::U128(Some(value)) => {
                Self::U128(Some(u128::from_str(value.as_str()).unwrap()))
            }
            Primitive::U256(Some(value)) => Self::U256(Some(U256::from_be_hex(value.as_str()))),
            Primitive::Bool(Some(value)) => Self::Bool(Some(value)),
            Primitive::Felt252(Some(value)) => {
                Self::Felt252(Some(Felt::from_str(value.as_str()).unwrap()))
            }
            Primitive::ClassHash(Some(value)) => {
                Self::ClassHash(Some(Felt::from_str(value.as_str()).unwrap()))
            }
            Primitive::ContractAddress(Some(value)) => {
                Self::ContractAddress(Some(Felt::from_str(value.as_str()).unwrap()))
            }
            Primitive::EthAddress(Some(value)) => {
                Self::EthAddress(Some(Felt::from_str(value.as_str()).unwrap()))
            }
            _ => unimplemented!(),
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Event {
    pub keys: Vec<String>,
    pub data: Vec<String>,
    pub transaction_hash: String,
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Message {
    pub message: String,
    pub signature: Vec<String>,
}

impl From<Message> for torii_proto::Message {
    fn from(val: Message) -> Self {
        torii_proto::Message {
            message: val.message,
            signature: val.signature.iter().map(|s| Felt::from_str(s).unwrap()).collect(),
        }
    }
}
