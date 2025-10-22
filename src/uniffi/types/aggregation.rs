// Aggregation types
use super::core::*;

#[derive(Debug, Clone)]
pub struct AggregationQuery {
    pub aggregator_ids: Vec<String>,
    pub entity_ids: Vec<String>,
    pub pagination: Pagination,
}

impl From<AggregationQuery> for torii_proto::AggregationQuery {
    fn from(val: AggregationQuery) -> Self {
        torii_proto::AggregationQuery {
            aggregator_ids: val.aggregator_ids,
            entity_ids: val.entity_ids,
            pagination: val.pagination.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AggregationEntry {
    pub id: String,
    pub aggregator_id: String,
    pub entity_id: String,
    pub value: U256,
    pub display_value: String,
    pub position: u64,
    pub model_id: String,
    pub created_at: u64,
    pub updated_at: u64,
}

impl From<torii_proto::AggregationEntry> for AggregationEntry {
    fn from(val: torii_proto::AggregationEntry) -> Self {
        AggregationEntry {
            id: val.id,
            aggregator_id: val.aggregator_id,
            entity_id: val.entity_id,
            value: u256_to_uniffi(val.value),
            display_value: val.display_value,
            position: val.position,
            model_id: val.model_id,
            created_at: val.created_at.timestamp() as u64,
            updated_at: val.updated_at.timestamp() as u64,
        }
    }
}

