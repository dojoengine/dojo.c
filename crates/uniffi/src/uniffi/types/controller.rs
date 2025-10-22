// Controller types
use super::core::*;

#[derive(Debug, Clone)]
pub struct Controller {
    pub address: FieldElement,
    pub username: String,
    pub deployed_at_timestamp: u64,
}

impl From<torii_proto::Controller> for Controller {
    fn from(val: torii_proto::Controller) -> Self {
        Controller {
            address: felt_to_field_element(val.address),
            username: val.username,
            deployed_at_timestamp: val.deployed_at.timestamp() as u64,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ControllerQuery {
    pub pagination: Pagination,
    pub contract_addresses: Vec<FieldElement>,
    pub usernames: Vec<String>,
}

impl From<ControllerQuery> for torii_proto::ControllerQuery {
    fn from(val: ControllerQuery) -> Self {
        torii_proto::ControllerQuery {
            pagination: val.pagination.into(),
            contract_addresses: val
                .contract_addresses
                .into_iter()
                .map(|a| field_element_to_felt(&a).unwrap())
                .collect(),
            usernames: val.usernames,
        }
    }
}
