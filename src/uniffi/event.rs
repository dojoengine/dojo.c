// Event and Message types
use super::core::*;

#[derive(Debug, Clone)]
pub struct Event {
    pub keys: Vec<FieldElement>,
    pub data: Vec<FieldElement>,
    pub transaction_hash: FieldElement,
}

impl From<Event> for torii_proto::Event {
    fn from(val: Event) -> Self {
        torii_proto::Event {
            keys: val.keys.into_iter().map(|k| field_element_to_felt(&k).unwrap()).collect(),
            data: val.data.into_iter().map(|d| field_element_to_felt(&d).unwrap()).collect(),
            transaction_hash: field_element_to_felt(&val.transaction_hash).unwrap(),
        }
    }
}

impl From<torii_proto::Event> for Event {
    fn from(val: torii_proto::Event) -> Self {
        Event {
            keys: val.keys.into_iter().map(felt_to_field_element).collect(),
            data: val.data.into_iter().map(felt_to_field_element).collect(),
            transaction_hash: felt_to_field_element(val.transaction_hash),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Message {
    pub message: String,
    pub signature: Vec<FieldElement>,
    pub world_address: FieldElement,
}

impl From<Message> for torii_proto::Message {
    fn from(val: Message) -> Self {
        torii_proto::Message {
            message: val.message,
            signature: val.signature.into_iter().map(|s| field_element_to_felt(&s).unwrap()).collect(),
            world_address: field_element_to_felt(&val.world_address).unwrap(),
        }
    }
}

