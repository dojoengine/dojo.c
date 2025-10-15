// Token types
use super::core::*;

#[derive(Debug, Clone)]
pub struct Token {
    pub contract_address: FieldElement,
    pub token_id: Option<U256>,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub metadata: String,
    pub total_supply: Option<U256>,
}

impl From<torii_proto::Token> for Token {
    fn from(val: torii_proto::Token) -> Self {
        Token {
            token_id: val.token_id.map(u256_to_uniffi),
            contract_address: felt_to_field_element(val.contract_address),
            name: val.name,
            symbol: val.symbol,
            decimals: val.decimals,
            metadata: val.metadata,
            total_supply: val.total_supply.map(u256_to_uniffi),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TokenBalance {
    pub balance: U256,
    pub account_address: FieldElement,
    pub contract_address: FieldElement,
    pub token_id: Option<U256>,
}

impl From<torii_proto::TokenBalance> for TokenBalance {
    fn from(val: torii_proto::TokenBalance) -> Self {
        TokenBalance {
            balance: u256_to_uniffi(val.balance),
            account_address: felt_to_field_element(val.account_address),
            contract_address: felt_to_field_element(val.contract_address),
            token_id: val.token_id.map(u256_to_uniffi),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TokenContract {
    pub contract_address: FieldElement,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub metadata: String,
    pub token_metadata: String,
    pub total_supply: Option<U256>,
}

impl From<torii_proto::TokenContract> for TokenContract {
    fn from(val: torii_proto::TokenContract) -> Self {
        Self {
            contract_address: felt_to_field_element(val.contract_address),
            name: val.name,
            symbol: val.symbol,
            decimals: val.decimals,
            token_metadata: val.token_metadata,
            total_supply: val.total_supply.map(u256_to_uniffi),
            metadata: val.metadata,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AttributeFilter {
    pub trait_name: String,
    pub trait_value: String,
}

impl From<AttributeFilter> for torii_proto::TokenAttributeFilter {
    fn from(val: AttributeFilter) -> Self {
        torii_proto::TokenAttributeFilter { trait_name: val.trait_name, trait_value: val.trait_value }
    }
}

#[derive(Debug, Clone)]
pub struct TokenQuery {
    pub contract_addresses: Vec<FieldElement>,
    pub token_ids: Vec<U256>,
    pub attribute_filters: Vec<AttributeFilter>,
    pub pagination: Pagination,
}

impl From<TokenQuery> for torii_proto::TokenQuery {
    fn from(val: TokenQuery) -> Self {
        torii_proto::TokenQuery {
            contract_addresses: val
                .contract_addresses
                .into_iter()
                .map(|a| field_element_to_felt(&a).unwrap())
                .collect(),
            token_ids: val.token_ids.into_iter().map(|t| uniffi_to_u256(&t).unwrap()).collect(),
            attribute_filters: val.attribute_filters.into_iter().map(|f| f.into()).collect(),
            pagination: val.pagination.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TokenBalanceQuery {
    pub contract_addresses: Vec<FieldElement>,
    pub account_addresses: Vec<FieldElement>,
    pub token_ids: Vec<U256>,
    pub pagination: Pagination,
}

impl From<TokenBalanceQuery> for torii_proto::TokenBalanceQuery {
    fn from(val: TokenBalanceQuery) -> Self {
        torii_proto::TokenBalanceQuery {
            contract_addresses: val
                .contract_addresses
                .into_iter()
                .map(|a| field_element_to_felt(&a).unwrap())
                .collect(),
            account_addresses: val
                .account_addresses
                .into_iter()
                .map(|a| field_element_to_felt(&a).unwrap())
                .collect(),
            token_ids: val.token_ids.into_iter().map(|t| uniffi_to_u256(&t).unwrap()).collect(),
            pagination: val.pagination.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TokenContractQuery {
    pub contract_addresses: Vec<FieldElement>,
    pub contract_types: Vec<super::contract::ContractType>,
    pub pagination: Pagination,
}

impl From<TokenContractQuery> for torii_proto::TokenContractQuery {
    fn from(val: TokenContractQuery) -> Self {
        torii_proto::TokenContractQuery {
            contract_addresses: val
                .contract_addresses
                .into_iter()
                .map(|a| field_element_to_felt(&a).unwrap())
                .collect(),
            contract_types: val.contract_types.into_iter().map(|t| t.into()).collect(),
            pagination: val.pagination.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TokenTransfer {
    pub id: String,
    pub contract_address: FieldElement,
    pub from_address: FieldElement,
    pub to_address: FieldElement,
    pub amount: U256,
    pub token_id: Option<U256>,
    pub executed_at: u64,
    pub event_id: Option<String>,
}

impl From<torii_proto::TokenTransfer> for TokenTransfer {
    fn from(val: torii_proto::TokenTransfer) -> Self {
        TokenTransfer {
            id: val.id,
            contract_address: felt_to_field_element(val.contract_address),
            from_address: felt_to_field_element(val.from_address),
            to_address: felt_to_field_element(val.to_address),
            amount: u256_to_uniffi(val.amount),
            token_id: val.token_id.map(u256_to_uniffi),
            executed_at: val.executed_at.timestamp() as u64,
            event_id: val.event_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TokenTransferQuery {
    pub contract_addresses: Vec<FieldElement>,
    pub account_addresses: Vec<FieldElement>,
    pub token_ids: Vec<U256>,
    pub pagination: Pagination,
}

impl From<TokenTransferQuery> for torii_proto::TokenTransferQuery {
    fn from(val: TokenTransferQuery) -> Self {
        torii_proto::TokenTransferQuery {
            contract_addresses: val
                .contract_addresses
                .into_iter()
                .map(|a| field_element_to_felt(&a).unwrap())
                .collect(),
            account_addresses: val
                .account_addresses
                .into_iter()
                .map(|a| field_element_to_felt(&a).unwrap())
                .collect(),
            token_ids: val.token_ids.into_iter().map(|t| uniffi_to_u256(&t).unwrap()).collect(),
            pagination: val.pagination.into(),
        }
    }
}

