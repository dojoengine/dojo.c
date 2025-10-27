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
pub struct Contracts(pub Vec<Contract>);

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
pub struct TokenContracts(pub Page<TokenContract>);

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TokenTransfers(pub Page<TokenTransfer>);

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Token {
    pub contract_address: String,
    pub token_id: Option<String>,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub metadata: String,
    pub total_supply: Option<String>,
}

impl From<torii_proto::Token> for Token {
    fn from(value: torii_proto::Token) -> Self {
        Self {
            contract_address: format!("{:#x}", value.contract_address),
            token_id: value.token_id.map(|t| format!("0x{:x}", t)),
            name: value.name.clone(),
            symbol: value.symbol.clone(),
            decimals: value.decimals,
            metadata: value.metadata.clone(),
            total_supply: value.total_supply.map(|t| format!("0x{:x}", t)),
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TokenContract {
    pub contract_address: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub metadata: String,
    pub token_metadata: String,
    pub total_supply: Option<String>,
}

impl From<torii_proto::TokenContract> for TokenContract {
    fn from(value: torii_proto::TokenContract) -> Self {
        Self {
            contract_address: format!("{:#x}", value.contract_address),
            name: value.name.clone(),
            symbol: value.symbol.clone(),
            decimals: value.decimals,
            token_metadata: value.token_metadata.clone(),
            total_supply: value.total_supply.map(|t| format!("0x{:x}", t)),
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
    pub token_id: Option<String>,
}

impl From<torii_proto::TokenBalance> for TokenBalance {
    fn from(value: torii_proto::TokenBalance) -> Self {
        Self {
            balance: format!("0x{:x}", value.balance),
            account_address: format!("{:#x}", value.account_address),
            contract_address: format!("{:#x}", value.contract_address),
            token_id: value.token_id.map(|t| format!("0x{:x}", t)),
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TokenTransfer {
    pub id: String,
    pub contract_address: String,
    pub from_address: String,
    pub to_address: String,
    pub amount: String,
    pub token_id: Option<String>,
    pub executed_at: u64,
    pub event_id: Option<String>,
}

impl From<torii_proto::TokenTransfer> for TokenTransfer {
    fn from(value: torii_proto::TokenTransfer) -> Self {
        Self {
            id: value.id,
            contract_address: format!("{:#x}", value.contract_address),
            from_address: format!("{:#x}", value.from_address),
            to_address: format!("{:#x}", value.to_address),
            amount: format!("0x{:x}", value.amount),
            token_id: value.token_id.map(|t| format!("0x{:x}", t)),
            executed_at: value.executed_at.timestamp() as u64,
            event_id: value.event_id,
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TokenTransferQuery {
    pub contract_addresses: Vec<String>,
    pub account_addresses: Vec<String>,
    pub token_ids: Vec<String>,
    pub pagination: Pagination,
}

impl From<TokenTransferQuery> for torii_proto::TokenTransferQuery {
    fn from(value: TokenTransferQuery) -> Self {
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
            from_block: val.from_block,
            to_block: val.to_block,
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
pub struct AttributeFilter {
    pub trait_name: String,
    pub trait_value: String,
}

impl From<AttributeFilter> for torii_proto::TokenAttributeFilter {
    fn from(value: AttributeFilter) -> Self {
        Self { trait_name: value.trait_name, trait_value: value.trait_value }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TokenQuery {
    pub contract_addresses: Vec<String>,
    pub token_ids: Vec<String>,
    pub attribute_filters: Vec<AttributeFilter>,
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
            attribute_filters: value.attribute_filters.into_iter().map(|a| a.into()).collect(),
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
pub struct TokenContractQuery {
    pub contract_addresses: Vec<String>,
    pub contract_types: Vec<ContractType>,
    pub pagination: Pagination,
}

impl From<TokenContractQuery> for torii_proto::TokenContractQuery {
    fn from(value: TokenContractQuery) -> Self {
        Self {
            contract_addresses: value
                .contract_addresses
                .into_iter()
                .map(|c| Felt::from_str(c.as_str()).unwrap())
                .collect(),
            contract_types: value.contract_types.into_iter().map(|a| a.into()).collect(),
            pagination: value.pagination.into(),
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[allow(clippy::upper_case_acronyms)]
pub enum ContractType {
    WORLD,
    ERC20,
    ERC721,
    ERC1155,
    UDC,
    OTHER,
}

impl From<ContractType> for torii_proto::ContractType {
    fn from(value: ContractType) -> Self {
        match value {
            ContractType::WORLD => torii_proto::ContractType::WORLD,
            ContractType::ERC20 => torii_proto::ContractType::ERC20,
            ContractType::ERC721 => torii_proto::ContractType::ERC721,
            ContractType::ERC1155 => torii_proto::ContractType::ERC1155,
            ContractType::UDC => torii_proto::ContractType::UDC,
            ContractType::OTHER => torii_proto::ContractType::OTHER,
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ContractQuery {
    pub contract_addresses: Vec<String>,
    pub contract_types: Vec<ContractType>,
}

impl From<ContractQuery> for torii_proto::ContractQuery {
    fn from(value: ContractQuery) -> Self {
        Self {
            contract_addresses: value
                .contract_addresses
                .into_iter()
                .map(|c| Felt::from_str(c.as_str()).unwrap())
                .collect(),
            contract_types: value.contract_types.into_iter().map(|t| t.into()).collect(),
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Contract {
    pub contract_address: String,
    pub contract_type: ContractType,
    pub head: Option<u64>,
    pub tps: Option<u64>,
    pub last_block_timestamp: Option<u64>,
    pub last_pending_block_tx: Option<String>,
    pub updated_at: u64,
    pub created_at: u64,
}

impl From<torii_proto::Contract> for Contract {
    fn from(value: torii_proto::Contract) -> Self {
        Self {
            contract_address: format!("{:#x}", value.contract_address),
            contract_type: value.contract_type.into(),
            head: value.head,
            tps: value.tps,
            last_block_timestamp: value.last_block_timestamp,
            last_pending_block_tx: value.last_pending_block_tx.map(|tx| format!("{:#x}", tx)),
            updated_at: value.updated_at.timestamp() as u64,
            created_at: value.created_at.timestamp() as u64,
        }
    }
}

impl From<torii_proto::ContractType> for ContractType {
    fn from(value: torii_proto::ContractType) -> Self {
        match value {
            torii_proto::ContractType::WORLD => ContractType::WORLD,
            torii_proto::ContractType::ERC20 => ContractType::ERC20,
            torii_proto::ContractType::ERC721 => ContractType::ERC721,
            torii_proto::ContractType::ERC1155 => ContractType::ERC1155,
            torii_proto::ContractType::UDC => ContractType::UDC,
            torii_proto::ContractType::OTHER => ContractType::OTHER,
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
    #[tsify(
        type = r#""primitive" | "struct" | "enum" | "array" | "tuple" | "bytearray" | "fixed_size_array""#
    )]
    pub r#type: String,
    pub type_name: String,
    #[serde(with = "serde_wasm_bindgen::preserve")]
    #[tsify(type = "boolean | number | string | Ty | Record<string, Ty> | Array<Ty> | EnumValue \
                    | FixedSizeArray | null")]
    pub value: JsValue,
    pub key: bool,
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct FixedSizeArray {
    pub array: Vec<Ty>,
    pub size: u32,
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
    pub world_address: String,
    pub hashed_keys: String,
    pub models: HashMap<String, Model>,
    pub created_at: u64,
    pub updated_at: u64,
    pub executed_at: u64,
}

impl From<torii_proto::schema::Entity> for Entity {
    fn from(value: torii_proto::schema::Entity) -> Self {
        Self {
            world_address: format!("{:#x}", value.world_address),
            hashed_keys: format!("{:#x}", value.hashed_keys),
            models: value.models.into_iter().map(|m| (m.name.clone(), m.into())).collect(),
            created_at: value.created_at.timestamp() as u64,
            updated_at: value.updated_at.timestamp() as u64,
            executed_at: value.executed_at.timestamp() as u64,
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
    PreConfirmed,
}

impl From<BlockTag> for starknet::core::types::BlockTag {
    fn from(value: BlockTag) -> Self {
        match value {
            BlockTag::Latest => starknet::core::types::BlockTag::Latest,
            BlockTag::PreConfirmed => starknet::core::types::BlockTag::PreConfirmed,
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
    pub world_addresses: Vec<String>,
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
            limit: value.limit,
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
            world_addresses: value
                .world_addresses
                .into_iter()
                .map(|addr| Felt::from_hex(&addr).unwrap())
                .collect(),
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
    pub world_address: String,
}

impl From<Message> for torii_proto::Message {
    fn from(val: Message) -> Self {
        torii_proto::Message {
            message: val.message,
            signature: val.signature.iter().map(|s| Felt::from_str(s).unwrap()).collect(),
            world_address: Felt::from_hex(&val.world_address).unwrap(),
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct AggregationQuery {
    pub aggregator_ids: Vec<String>,
    pub entity_ids: Vec<String>,
    pub pagination: Pagination,
}

impl From<AggregationQuery> for torii_proto::AggregationQuery {
    fn from(value: AggregationQuery) -> Self {
        Self {
            aggregator_ids: value.aggregator_ids,
            entity_ids: value.entity_ids,
            pagination: value.pagination.into(),
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ActivityQuery {
    pub world_addresses: Vec<String>,
    pub namespaces: Vec<String>,
    pub caller_addresses: Vec<String>,
    pub from_time: Option<String>,
    pub to_time: Option<String>,
    pub pagination: Pagination,
}

impl From<ActivityQuery> for torii_proto::ActivityQuery {
    fn from(value: ActivityQuery) -> Self {
        Self {
            world_addresses: value
                .world_addresses
                .into_iter()
                .map(|addr| Felt::from_str(&addr).unwrap())
                .collect(),
            namespaces: value.namespaces,
            caller_addresses: value
                .caller_addresses
                .into_iter()
                .map(|addr| Felt::from_str(&addr).unwrap())
                .collect(),
            from_time: value.from_time.and_then(|t| t.parse().ok()),
            to_time: value.to_time.and_then(|t| t.parse().ok()),
            pagination: value.pagination.into(),
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct AggregationEntry {
    pub id: String,
    pub aggregator_id: String,
    pub entity_id: String,
    pub value: String,
    pub display_value: String,
    pub position: u64,
    pub model_id: String,
    pub created_at: u64,
    pub updated_at: u64,
}

impl From<torii_proto::AggregationEntry> for AggregationEntry {
    fn from(value: torii_proto::AggregationEntry) -> Self {
        Self {
            id: value.id,
            aggregator_id: value.aggregator_id,
            entity_id: value.entity_id,
            value: format!("0x{:x}", value.value),
            display_value: value.display_value,
            position: value.position,
            model_id: value.model_id,
            created_at: value.created_at.timestamp() as u64,
            updated_at: value.updated_at.timestamp() as u64,
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi, hashmap_as_object)]
pub struct Activity {
    pub id: String,
    pub world_address: String,
    pub namespace: String,
    pub caller_address: String,
    pub session_start: u64,
    pub session_end: u64,
    pub action_count: u32,
    pub actions: HashMap<String, u32>,
    pub updated_at: u64,
}

impl From<torii_proto::Activity> for Activity {
    fn from(value: torii_proto::Activity) -> Self {
        Self {
            id: value.id,
            world_address: format!("{:#x}", value.world_address),
            namespace: value.namespace,
            caller_address: format!("{:#x}", value.caller_address),
            session_start: value.session_start.timestamp() as u64,
            session_end: value.session_end.timestamp() as u64,
            action_count: value.action_count,
            actions: value.actions,
            updated_at: value.updated_at.timestamp() as u64,
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Aggregations(pub Page<AggregationEntry>);

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Activities(pub Page<Activity>);

// ===== Achievement Types =====

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct AchievementQuery {
    pub world_addresses: Vec<String>,
    pub namespaces: Vec<String>,
    pub hidden: Option<bool>,
    pub pagination: Pagination,
}

impl From<AchievementQuery> for torii_proto::AchievementQuery {
    fn from(value: AchievementQuery) -> Self {
        Self {
            world_addresses: value
                .world_addresses
                .into_iter()
                .map(|addr| Felt::from_hex(&addr).unwrap())
                .collect(),
            namespaces: value.namespaces,
            hidden: value.hidden,
            pagination: value.pagination.into(),
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct PlayerAchievementQuery {
    pub world_addresses: Vec<String>,
    pub namespaces: Vec<String>,
    pub player_addresses: Vec<String>,
    pub pagination: Pagination,
}

impl From<PlayerAchievementQuery> for torii_proto::PlayerAchievementQuery {
    fn from(value: PlayerAchievementQuery) -> Self {
        Self {
            world_addresses: value
                .world_addresses
                .into_iter()
                .map(|addr| Felt::from_hex(&addr).unwrap())
                .collect(),
            namespaces: value.namespaces,
            player_addresses: value
                .player_addresses
                .into_iter()
                .map(|addr| Felt::from_hex(&addr).unwrap())
                .collect(),
            pagination: value.pagination.into(),
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Achievement {
    pub id: String,
    pub world_address: String,
    pub namespace: String,
    pub entity_id: String,
    pub hidden: bool,
    pub index: u32,
    pub points: u32,
    pub start: String,
    pub end: String,
    pub group: String,
    pub icon: String,
    pub title: String,
    pub description: String,
    pub tasks: Vec<AchievementTask>,
    pub data: String,
    pub total_completions: u32,
    pub completion_rate: f64,
    pub created_at: u64,
    pub updated_at: u64,
}

impl From<torii_proto::Achievement> for Achievement {
    fn from(value: torii_proto::Achievement) -> Self {
        Self {
            id: value.id,
            world_address: format!("{:#x}", value.world_address),
            namespace: value.namespace,
            entity_id: value.entity_id,
            hidden: value.hidden,
            index: value.index,
            points: value.points,
            start: value.start,
            end: value.end,
            group: value.group,
            icon: value.icon,
            title: value.title,
            description: value.description,
            tasks: value.tasks.into_iter().map(|t| t.into()).collect(),
            data: value.data.unwrap_or_default(),
            total_completions: value.total_completions,
            completion_rate: value.completion_rate,
            created_at: value.created_at.timestamp() as u64,
            updated_at: value.updated_at.timestamp() as u64,
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct AchievementTask {
    pub task_id: String,
    pub description: String,
    pub total: u32,
    pub total_completions: u32,
    pub completion_rate: f64,
    pub created_at: u64,
}

impl From<torii_proto::AchievementTask> for AchievementTask {
    fn from(value: torii_proto::AchievementTask) -> Self {
        Self {
            task_id: value.task_id,
            description: value.description,
            total: value.total,
            total_completions: value.total_completions,
            completion_rate: value.completion_rate,
            created_at: value.created_at.timestamp() as u64,
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct PlayerAchievementEntry {
    pub player_address: String,
    pub stats: PlayerAchievementStats,
    pub achievements: Vec<PlayerAchievementProgress>,
}

impl From<torii_proto::PlayerAchievementEntry> for PlayerAchievementEntry {
    fn from(value: torii_proto::PlayerAchievementEntry) -> Self {
        Self {
            player_address: format!("{:#x}", value.player_address),
            stats: value.stats.into(),
            achievements: value.achievements.into_iter().map(|a| a.into()).collect(),
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct PlayerAchievementStats {
    pub total_points: u32,
    pub completed_achievements: u32,
    pub total_achievements: u32,
    pub completion_percentage: f64,
    pub last_achievement_at: Option<u64>,
    pub created_at: u64,
    pub updated_at: u64,
}

impl From<torii_proto::PlayerAchievementStats> for PlayerAchievementStats {
    fn from(value: torii_proto::PlayerAchievementStats) -> Self {
        Self {
            total_points: value.total_points,
            completed_achievements: value.completed_achievements,
            total_achievements: value.total_achievements,
            completion_percentage: value.completion_percentage,
            last_achievement_at: value.last_achievement_at.map(|t| t.timestamp() as u64),
            created_at: value.created_at.timestamp() as u64,
            updated_at: value.updated_at.timestamp() as u64,
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct PlayerAchievementProgress {
    pub achievement: Achievement,
    pub task_progress: Vec<TaskProgress>,
    pub completed: bool,
    pub progress_percentage: f64,
}

impl From<torii_proto::PlayerAchievementProgress> for PlayerAchievementProgress {
    fn from(value: torii_proto::PlayerAchievementProgress) -> Self {
        Self {
            achievement: value.achievement.into(),
            task_progress: value.task_progress.into_iter().map(|t| t.into()).collect(),
            completed: value.completed,
            progress_percentage: value.progress_percentage,
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TaskProgress {
    pub task_id: String,
    pub count: u32,
    pub completed: bool,
}

impl From<torii_proto::TaskProgress> for TaskProgress {
    fn from(value: torii_proto::TaskProgress) -> Self {
        Self { task_id: value.task_id, count: value.count, completed: value.completed }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct AchievementProgression {
    pub id: String,
    pub achievement_id: String,
    pub task_id: String,
    pub world_address: String,
    pub namespace: String,
    pub player_id: String,
    pub count: u32,
    pub completed: bool,
    pub completed_at: Option<u64>,
    pub created_at: u64,
    pub updated_at: u64,
}

impl From<torii_proto::AchievementProgression> for AchievementProgression {
    fn from(value: torii_proto::AchievementProgression) -> Self {
        Self {
            id: value.id,
            achievement_id: value.achievement_id,
            task_id: value.task_id,
            world_address: format!("{:#x}", value.world_address),
            namespace: value.namespace,
            player_id: format!("{:#x}", value.player_id),
            count: value.count,
            completed: value.completed,
            completed_at: value.completed_at.map(|t| t.timestamp() as u64),
            created_at: value.created_at.timestamp() as u64,
            updated_at: value.updated_at.timestamp() as u64,
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Achievements(pub Page<Achievement>);

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct PlayerAchievements(pub Page<PlayerAchievementEntry>);

// Search types
#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct SearchQuery {
    pub query: String,
    pub limit: u32,
}

impl From<SearchQuery> for torii_proto::SearchQuery {
    fn from(value: SearchQuery) -> Self {
        Self { query: value.query, limit: value.limit }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi, hashmap_as_object)]
pub struct SearchMatch {
    pub id: String,
    pub fields: HashMap<String, String>,
    pub score: Option<f64>,
}

impl From<torii_proto::SearchMatch> for SearchMatch {
    fn from(value: torii_proto::SearchMatch) -> Self {
        Self { id: value.id, fields: value.fields, score: value.score }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TableSearchResults {
    pub table: String,
    pub count: u32,
    pub matches: Vec<SearchMatch>,
}

impl From<torii_proto::TableSearchResults> for TableSearchResults {
    fn from(value: torii_proto::TableSearchResults) -> Self {
        Self {
            table: value.table,
            count: value.count,
            matches: value.matches.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct SearchResponse {
    pub total: u32,
    pub results: Vec<TableSearchResults>,
}

impl From<torii_proto::SearchResponse> for SearchResponse {
    fn from(value: torii_proto::SearchResponse) -> Self {
        Self { total: value.total, results: value.results.into_iter().map(Into::into).collect() }
    }
}

// WASM-specific client types
#[wasm_bindgen]
pub struct ToriiClient {
    #[wasm_bindgen(skip)]
    pub inner: std::sync::Arc<torii_client::Client>,
}

#[wasm_bindgen]
pub struct Provider(
    pub(crate)  std::sync::Arc<
        starknet::providers::JsonRpcClient<starknet::providers::jsonrpc::HttpTransport>,
    >,
);

#[wasm_bindgen]
pub struct Account(
    pub(crate)  starknet::accounts::SingleOwnerAccount<
        std::sync::Arc<
            starknet::providers::JsonRpcClient<starknet::providers::jsonrpc::HttpTransport>,
        >,
        starknet::signers::LocalWallet,
    >,
);

#[wasm_bindgen]
pub struct Subscription {
    pub id: u64,
    pub(crate) trigger: stream_cancel::Trigger,
}
