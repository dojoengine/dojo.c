use std::collections::HashMap;
use std::str::FromStr;

use crypto_bigint::U256;
use dojo_types::schema::Struct;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use starknet::core::types::FunctionCall;
use starknet::core::utils::get_selector_from_name;
use starknet_crypto::Felt;
use tsify_next::{declare, Tsify};
use wasm_bindgen::prelude::*;

use super::utils::parse_ty_as_json_str;

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Controllers(pub Vec<Controller>);

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Controller {
    pub address: String,
    pub username: String,
    pub deployed_at_timestamp: u64,
}

impl From<&torii_grpc::types::Controller> for Controller {
    fn from(value: &torii_grpc::types::Controller) -> Self {
        Self {
            address: format!("{:#x}", value.address),
            username: value.username.clone(),
            deployed_at_timestamp: value.deployed_at,
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Tokens(pub Vec<Token>);

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TokenBalances(pub Vec<TokenBalance>);

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Token {
    pub id: String,
    pub contract_address: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub metadata: String,
}

impl From<&torii_grpc::types::Token> for Token {
    fn from(value: &torii_grpc::types::Token) -> Self {
        Self {
            id: format!("{:#x}", value.contract_address),
            contract_address: format!("{:#x}", value.contract_address),
            name: value.name.clone(),
            symbol: value.symbol.clone(),
            decimals: value.decimals,
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

impl From<&torii_grpc::types::TokenBalance> for TokenBalance {
    fn from(value: &torii_grpc::types::TokenBalance) -> Self {
        Self {
            balance: format!("0x{:x}", value.balance),
            account_address: format!("{:#x}", value.account_address),
            contract_address: format!("{:#x}", value.contract_address),
            token_id: value.token_id.to_string(),
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

impl From<&IndexerUpdate> for torii_grpc::types::IndexerUpdate {
    fn from(value: &IndexerUpdate) -> Self {
        Self {
            head: value.head,
            tps: value.tps,
            last_block_timestamp: value.last_block_timestamp,
            contract_address: Felt::from_str(value.contract_address.as_str()).unwrap(),
        }
    }
}

impl From<&torii_grpc::types::IndexerUpdate> for IndexerUpdate {
    fn from(value: &torii_grpc::types::IndexerUpdate) -> Self {
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
    #[serde(rename = "rpcUrl")]
    pub rpc_url: String,
    #[serde(rename = "toriiUrl")]
    pub torii_url: String,
    #[serde(rename = "relayUrl")]
    pub relay_url: String,
    #[serde(rename = "worldAddress")]
    pub world_address: String,
}

#[wasm_bindgen]
impl ClientConfig {
    #[wasm_bindgen(constructor)]
    pub fn new(
        rpc_url: String,
        torii_url: String,
        relay_url: String,
        world_address: String,
    ) -> Self {
        Self { rpc_url, torii_url, relay_url, world_address }
    }
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

impl From<&starknet::core::crypto::Signature> for Signature {
    fn from(value: &starknet::core::crypto::Signature) -> Self {
        Self { r: format!("{:#x}", value.r), s: format!("{:#x}", value.s) }
    }
}

impl From<&Signature> for starknet::core::crypto::Signature {
    fn from(value: &Signature) -> Self {
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

impl From<&Struct> for Model {
    fn from(value: &Struct) -> Self {
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
pub struct Entity(pub HashMap<String, Model>);

impl From<&torii_grpc::types::schema::Entity> for Entity {
    fn from(value: &torii_grpc::types::schema::Entity) -> Self {
        let mut seen_models = HashMap::new();
        Self(
            value
                .models
                .iter()
                .map(|m| {
                    let count = seen_models.entry(m.name.clone()).or_insert(0);
                    let name =
                        if *count == 0 { m.name.clone() } else { format!("{}-{}", m.name, count) };
                    *count += 1;
                    (name, m.into())
                })
                .collect(),
        )
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi, hashmap_as_object)]
pub struct Entities(pub HashMap<String, Entity>);

impl From<&Vec<torii_grpc::types::schema::Entity>> for Entities {
    fn from(value: &Vec<torii_grpc::types::schema::Entity>) -> Self {
        Self(
            value
                .iter()
                .enumerate()
                .map(|(i, e)| {
                    (
                        if e.hashed_keys != Felt::ZERO {
                            format!("{:#x}", e.hashed_keys)
                        } else {
                            format!("{:#x}", i)
                        },
                        e.into(),
                    )
                })
                .collect(),
        )
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Call {
    pub to: String,
    pub selector: String,
    pub calldata: Vec<String>,
}

impl From<&Call> for starknet::core::types::Call {
    fn from(value: &Call) -> Self {
        Self {
            to: Felt::from_str(value.to.as_str()).unwrap(),
            selector: get_selector_from_name(value.selector.as_str()).unwrap(),
            calldata: value.calldata.iter().map(|c| Felt::from_str(c.as_str()).unwrap()).collect(),
        }
    }
}

impl From<&Call> for FunctionCall {
    fn from(value: &Call) -> Self {
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

impl From<&BlockTag> for starknet::core::types::BlockTag {
    fn from(value: &BlockTag) -> Self {
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

impl From<&BlockId> for starknet::core::types::BlockId {
    fn from(value: &BlockId) -> Self {
        match value {
            BlockId::Hash(hash) => {
                starknet::core::types::BlockId::Hash(Felt::from_str(hash.as_str()).unwrap())
            }
            BlockId::Number(number) => starknet::core::types::BlockId::Number(*number),
            BlockId::BlockTag(tag) => starknet::core::types::BlockId::Tag(tag.into()),
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Query {
    pub limit: u32,
    pub offset: u32,
    pub clause: Option<Clause>,
    pub dont_include_hashed_keys: bool,
    pub order_by: Vec<OrderBy>,
    pub entity_models: Vec<String>,
    pub entity_updated_after: u64,
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct OrderBy {
    pub model: String,
    pub member: String,
    pub direction: OrderDirection,
}

impl From<&OrderBy> for torii_grpc::types::OrderBy {
    fn from(value: &OrderBy) -> Self {
        Self {
            model: value.model.clone(),
            member: value.member.clone(),
            direction: (&value.direction).into(),
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum OrderDirection {
    Asc,
    Desc,
}

impl From<&OrderDirection> for torii_grpc::types::OrderDirection {
    fn from(value: &OrderDirection) -> Self {
        match value {
            OrderDirection::Asc => Self::Asc,
            OrderDirection::Desc => Self::Desc,
        }
    }
}

impl From<&Query> for torii_grpc::types::Query {
    fn from(value: &Query) -> Self {
        Self {
            limit: value.limit,
            offset: value.offset,
            clause: value.clause.as_ref().map(|c| c.into()),
            dont_include_hashed_keys: value.dont_include_hashed_keys,
            order_by: value.order_by.iter().map(|o| o.into()).collect(),
            entity_models: value.entity_models.iter().map(|m| m.to_string()).collect(),
            entity_updated_after: value.entity_updated_after,
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Clause {
    Keys(KeysClause),
    Member(MemberClause),
    Composite(CompositeClause),
}

#[declare]
pub type KeysClauses = Vec<EntityKeysClause>;

#[declare]
pub type ModelKeysClauses = Vec<ModelKeysClause>;

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ModelKeysClause {
    pub model: String,
    pub keys: Vec<String>,
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum PatternMatching {
    FixedLen = 0,
    VariableLen = 1,
}

impl From<&PatternMatching> for torii_grpc::types::PatternMatching {
    fn from(value: &PatternMatching) -> Self {
        match value {
            PatternMatching::FixedLen => Self::FixedLen,
            PatternMatching::VariableLen => Self::VariableLen,
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum EntityKeysClause {
    HashedKeys(Vec<String>),
    Keys(KeysClause),
}

impl From<&EntityKeysClause> for torii_grpc::types::EntityKeysClause {
    fn from(value: &EntityKeysClause) -> Self {
        match value {
            EntityKeysClause::HashedKeys(keys) => {
                Self::HashedKeys(keys.iter().map(|k| Felt::from_str(k.as_str()).unwrap()).collect())
            }
            EntityKeysClause::Keys(keys) => Self::Keys(keys.into()),
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

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum MemberValue {
    Primitive(Primitive),
    String(String),
    List(Vec<MemberValue>),
}

impl From<&MemberValue> for torii_grpc::types::MemberValue {
    fn from(value: &MemberValue) -> Self {
        match value {
            MemberValue::Primitive(primitive) => {
                torii_grpc::types::MemberValue::Primitive(primitive.into())
            }
            MemberValue::String(string) => torii_grpc::types::MemberValue::String(string.clone()),
            MemberValue::List(list) => {
                let values =
                    list.iter().map(|v| v.into()).collect::<Vec<torii_grpc::types::MemberValue>>();
                torii_grpc::types::MemberValue::List(values)
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

impl From<&ModelKeysClause> for torii_grpc::types::ModelKeysClause {
    fn from(value: &ModelKeysClause) -> Self {
        Self {
            model: value.model.to_string(),
            keys: value.keys.iter().map(|k| Felt::from_str(k.as_str()).unwrap()).collect(),
        }
    }
}

impl From<&KeysClause> for torii_grpc::types::KeysClause {
    fn from(value: &KeysClause) -> Self {
        Self {
            keys: value
                .keys
                .iter()
                .map(|o| o.as_ref().map(|k| Felt::from_str(k.as_str()).unwrap()))
                .collect(),
            models: value.models.iter().map(|m| m.to_string()).collect(),
            pattern_matching: (&value.pattern_matching).into(),
        }
    }
}

impl From<&MemberClause> for torii_grpc::types::MemberClause {
    fn from(value: &MemberClause) -> Self {
        Self {
            model: value.model.to_string(),
            member: value.member.to_string(),
            operator: (&value.operator).into(),
            value: (&value.value).into(),
        }
    }
}

impl From<&CompositeClause> for torii_grpc::types::CompositeClause {
    fn from(value: &CompositeClause) -> Self {
        Self {
            operator: (&value.operator).into(),
            clauses: value.clauses.iter().map(|c| c.into()).collect(),
        }
    }
}

impl From<&Clause> for torii_grpc::types::Clause {
    fn from(value: &Clause) -> Self {
        match value {
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

impl From<&LogicalOperator> for torii_grpc::types::LogicalOperator {
    fn from(value: &LogicalOperator) -> Self {
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

impl From<&ComparisonOperator> for torii_grpc::types::ComparisonOperator {
    fn from(value: &ComparisonOperator) -> Self {
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

impl From<&Value> for torii_grpc::types::Value {
    fn from(value: &Value) -> Self {
        Self {
            primitive_type: (&value.primitive_type).into(),
            value_type: (&value.value_type).into(),
        }
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

impl From<&ValueType> for torii_grpc::types::ValueType {
    fn from(value: &ValueType) -> Self {
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

impl From<&Primitive> for dojo_types::primitive::Primitive {
    fn from(value: &Primitive) -> Self {
        match value {
            Primitive::I8(Some(value)) => Self::I8(Some(*value)),
            Primitive::I16(Some(value)) => Self::I16(Some(*value)),
            Primitive::I32(Some(value)) => Self::I32(Some(*value)),
            Primitive::I64(Some(value)) => Self::I64(Some(*value)),
            Primitive::I128(Some(value)) => Self::I128(Some(i128::from_str(value).unwrap())),
            Primitive::U8(Some(value)) => Self::U8(Some(*value)),
            Primitive::U16(Some(value)) => Self::U16(Some(*value)),
            Primitive::U32(Some(value)) => Self::U32(Some(*value)),
            Primitive::U64(Some(value)) => Self::U64(Some(*value)),
            Primitive::U128(Some(value)) => Self::U128(Some(u128::from_str(value).unwrap())),
            Primitive::U256(Some(value)) => Self::U256(Some(U256::from_be_hex(value.as_str()))),
            Primitive::Bool(Some(value)) => Self::Bool(Some(*value)),
            Primitive::Felt252(Some(value)) => Self::Felt252(Some(Felt::from_str(value).unwrap())),
            Primitive::ClassHash(Some(value)) => {
                Self::ClassHash(Some(Felt::from_str(value).unwrap()))
            }
            Primitive::ContractAddress(Some(value)) => {
                Self::ContractAddress(Some(Felt::from_str(value).unwrap()))
            }
            Primitive::EthAddress(Some(value)) => {
                Self::EthAddress(Some(Felt::from_str(value).unwrap()))
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
