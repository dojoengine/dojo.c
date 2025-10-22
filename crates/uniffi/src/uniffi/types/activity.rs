// Activity types
use chrono::DateTime;

use super::core::*;

#[derive(Debug, Clone)]
pub struct ActionCount {
    pub action_name: String,
    pub count: u32,
}

#[derive(Debug, Clone)]
pub struct Activity {
    pub id: String,
    pub world_address: FieldElement,
    pub namespace: String,
    pub caller_address: FieldElement,
    pub session_start: u64,
    pub session_end: u64,
    pub action_count: u32,
    pub actions: Vec<ActionCount>,
    pub updated_at: u64,
}

impl From<torii_proto::Activity> for Activity {
    fn from(val: torii_proto::Activity) -> Self {
        let actions: Vec<ActionCount> = val
            .actions
            .into_iter()
            .map(|(name, count)| ActionCount { action_name: name, count })
            .collect();

        Activity {
            id: val.id,
            world_address: felt_to_field_element(val.world_address),
            namespace: val.namespace,
            caller_address: felt_to_field_element(val.caller_address),
            session_start: val.session_start.timestamp() as u64,
            session_end: val.session_end.timestamp() as u64,
            action_count: val.action_count,
            actions,
            updated_at: val.updated_at.timestamp() as u64,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ActivityQuery {
    pub world_addresses: Vec<FieldElement>,
    pub namespaces: Vec<String>,
    pub caller_addresses: Vec<FieldElement>,
    pub from_time: Option<u64>,
    pub to_time: Option<u64>,
    pub pagination: Pagination,
}

impl From<ActivityQuery> for torii_proto::ActivityQuery {
    fn from(val: ActivityQuery) -> Self {
        torii_proto::ActivityQuery {
            world_addresses: val
                .world_addresses
                .into_iter()
                .map(|a| field_element_to_felt(&a).unwrap())
                .collect(),
            namespaces: val.namespaces,
            caller_addresses: val
                .caller_addresses
                .into_iter()
                .map(|a| field_element_to_felt(&a).unwrap())
                .collect(),
            from_time: val.from_time.map(|t| DateTime::from_timestamp(t as i64, 0).unwrap()),
            to_time: val.to_time.map(|t| DateTime::from_timestamp(t as i64, 0).unwrap()),
            pagination: val.pagination.into(),
        }
    }
}
