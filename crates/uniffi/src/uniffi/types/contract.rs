// Contract types
use super::core::*;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Contract {
    pub contract_address: FieldElement,
    pub contract_type: ContractType,
    pub head: Option<u64>,
    pub tps: Option<u64>,
    pub last_block_timestamp: Option<u64>,
    pub last_pending_block_tx: Option<FieldElement>,
    pub updated_at: u64,
    pub created_at: u64,
}

impl From<torii_proto::Contract> for Contract {
    fn from(val: torii_proto::Contract) -> Self {
        Contract {
            contract_type: val.contract_type.into(),
            head: val.head,
            tps: val.tps,
            last_block_timestamp: val.last_block_timestamp,
            last_pending_block_tx: val.last_pending_block_tx.map(felt_to_field_element),
            updated_at: val.updated_at.timestamp() as u64,
            created_at: val.created_at.timestamp() as u64,
            contract_address: felt_to_field_element(val.contract_address),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ContractQuery {
    pub contract_addresses: Vec<FieldElement>,
    pub contract_types: Vec<ContractType>,
}

impl From<ContractQuery> for torii_proto::ContractQuery {
    fn from(val: ContractQuery) -> Self {
        torii_proto::ContractQuery {
            contract_addresses: val
                .contract_addresses
                .into_iter()
                .map(|a| field_element_to_felt(&a).unwrap())
                .collect(),
            contract_types: val.contract_types.into_iter().map(|t| t.into()).collect(),
        }
    }
}
