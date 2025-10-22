// Transaction types
use super::core::*;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct TransactionCall {
    pub contract_address: FieldElement,
    pub entrypoint: String,
    pub calldata: Vec<FieldElement>,
    pub call_type: CallType,
    pub caller_address: FieldElement,
}

impl From<torii_proto::TransactionCall> for TransactionCall {
    fn from(val: torii_proto::TransactionCall) -> Self {
        TransactionCall {
            contract_address: felt_to_field_element(val.contract_address),
            entrypoint: val.entrypoint,
            calldata: val.calldata.into_iter().map(felt_to_field_element).collect(),
            call_type: val.call_type.into(),
            caller_address: felt_to_field_element(val.caller_address),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub transaction_hash: FieldElement,
    pub sender_address: FieldElement,
    pub calldata: Vec<FieldElement>,
    pub max_fee: FieldElement,
    pub signature: Vec<FieldElement>,
    pub nonce: FieldElement,
    pub block_number: u64,
    pub transaction_type: String,
    pub block_timestamp: u64,
    pub calls: Vec<TransactionCall>,
    pub unique_models: Vec<FieldElement>,
}

impl From<torii_proto::Transaction> for Transaction {
    fn from(val: torii_proto::Transaction) -> Self {
        Transaction {
            transaction_hash: felt_to_field_element(val.transaction_hash),
            sender_address: felt_to_field_element(val.sender_address),
            calldata: val.calldata.into_iter().map(felt_to_field_element).collect(),
            max_fee: felt_to_field_element(val.max_fee),
            signature: val.signature.into_iter().map(felt_to_field_element).collect(),
            nonce: felt_to_field_element(val.nonce),
            block_number: val.block_number,
            transaction_type: val.transaction_type,
            block_timestamp: val.block_timestamp.timestamp() as u64,
            calls: val.calls.into_iter().map(|c| c.into()).collect(),
            unique_models: val.unique_models.into_iter().map(felt_to_field_element).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TransactionFilter {
    pub transaction_hashes: Vec<FieldElement>,
    pub caller_addresses: Vec<FieldElement>,
    pub contract_addresses: Vec<FieldElement>,
    pub entrypoints: Vec<String>,
    pub model_selectors: Vec<FieldElement>,
    pub from_block: Option<u64>,
    pub to_block: Option<u64>,
}

impl From<TransactionFilter> for torii_proto::TransactionFilter {
    fn from(val: TransactionFilter) -> Self {
        torii_proto::TransactionFilter {
            transaction_hashes: val
                .transaction_hashes
                .into_iter()
                .map(|h| field_element_to_felt(&h).unwrap())
                .collect(),
            caller_addresses: val
                .caller_addresses
                .into_iter()
                .map(|a| field_element_to_felt(&a).unwrap())
                .collect(),
            contract_addresses: val
                .contract_addresses
                .into_iter()
                .map(|a| field_element_to_felt(&a).unwrap())
                .collect(),
            entrypoints: val.entrypoints,
            model_selectors: val
                .model_selectors
                .into_iter()
                .map(|s| field_element_to_felt(&s).unwrap())
                .collect(),
            from_block: val.from_block,
            to_block: val.to_block,
        }
    }
}

#[derive(Debug, Clone)]
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
