use dojo_types::schema::Ty;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use torii_client::client::Client as TClient;

pub struct ToriiClient(TClient);

#[derive(Clone)]
#[repr(C)]
pub struct CArray<T> {
    data: *const T,
    data_len: usize,
}

#[repr(C)]
pub struct Error {
    message: *const c_char,
}

#[derive(Clone)]
#[repr(C)]
pub struct FieldElement {
    data: [u8; 32],
}

impl From<&FieldElement> for starknet::core::types::FieldElement {
    fn from(val: &FieldElement) -> Self {
        starknet::core::types::FieldElement::from_bytes_be(&val.data).unwrap()
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct EntityQuery {
    pub model: *const c_char,
    pub clause: Clause,
}

#[derive(Clone)]
#[repr(C)]
pub enum Clause {
    Keys(KeysClause),
    Attribute(AttributeClause),
    Composite(CompositeClause),
}

type KeysClause = CArray<FieldElement>;

#[derive(Clone)]
#[repr(C)]
pub struct AttributeClause {
    pub attribute: *const c_char,
    pub operator: ComparisonOperator,
    pub value: Value,
}

#[derive(Clone)]
#[repr(C)]
pub struct CompositeClause {
    pub operator: LogicalOperator,
    pub clauses: *const Clause,
    pub clauses_len: usize,
}

#[derive(Clone)]
#[repr(C)]
pub enum LogicalOperator {
    And,
    Or,
}

#[derive(Clone)]
#[repr(C)]
pub enum ComparisonOperator {
    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
}

#[derive(Clone)]
#[repr(C)]
pub enum Value {
    String(*const c_char),
    Int(i64),
    UInt(u64),
    Bool(bool),
    Bytes(CArray<u8>),
}

impl From<&EntityQuery> for dojo_types::schema::EntityQuery {
    fn from(val: &EntityQuery) -> Self {
        dojo_types::schema::EntityQuery {
            model: unsafe { CStr::from_ptr(val.model).to_string_lossy().into_owned() },
            clause: (&val.clause.clone()).into(),
        }
    }
}

impl From<&Clause> for dojo_types::schema::Clause {
    fn from(val: &Clause) -> Self {
        match val {
            Clause::Keys(keys) => dojo_types::schema::Clause::Keys((&keys.clone()).into()),
            Clause::Attribute(attribute) => {
                dojo_types::schema::Clause::Attribute((&attribute.clone()).into())
            }
            Clause::Composite(composite) => {
                dojo_types::schema::Clause::Composite((&composite.clone()).into())
            }
        }
    }
}

impl From<&KeysClause> for dojo_types::schema::KeysClause {
    fn from(val: &KeysClause) -> Self {
        let keys = unsafe { std::slice::from_raw_parts(val.data, val.data_len).to_vec() };

        dojo_types::schema::KeysClause {
            keys: keys.iter().map(|k| k.into()).collect(),
        }
    }
}

impl From<&AttributeClause> for dojo_types::schema::AttributeClause {
    fn from(val: &AttributeClause) -> Self {
        dojo_types::schema::AttributeClause {
            attribute: unsafe { CStr::from_ptr(val.attribute).to_string_lossy().into_owned() },
            operator: (&val.operator.clone()).into(),
            value: (&val.value.clone()).into(),
        }
    }
}

impl From<&CompositeClause> for dojo_types::schema::CompositeClause {
    fn from(val: &CompositeClause) -> Self {
        let operator = &val.operator.clone();
        let clauses = unsafe { std::slice::from_raw_parts(val.clauses, val.clauses_len).to_vec() };

        dojo_types::schema::CompositeClause {
            operator: operator.into(),
            clauses: clauses.iter().map(|c| c.into()).collect(),
        }
    }
}

impl From<&LogicalOperator> for dojo_types::schema::LogicalOperator {
    fn from(val: &LogicalOperator) -> Self {
        match val {
            LogicalOperator::And => dojo_types::schema::LogicalOperator::And,
            LogicalOperator::Or => dojo_types::schema::LogicalOperator::Or,
        }
    }
}

impl From<&ComparisonOperator> for dojo_types::schema::ComparisonOperator {
    fn from(val: &ComparisonOperator) -> Self {
        match val {
            ComparisonOperator::Eq => dojo_types::schema::ComparisonOperator::Eq,
            ComparisonOperator::Neq => dojo_types::schema::ComparisonOperator::Neq,
            ComparisonOperator::Gt => dojo_types::schema::ComparisonOperator::Gt,
            ComparisonOperator::Gte => dojo_types::schema::ComparisonOperator::Gte,
            ComparisonOperator::Lt => dojo_types::schema::ComparisonOperator::Lt,
            ComparisonOperator::Lte => dojo_types::schema::ComparisonOperator::Lte,
        }
    }
}

impl From<&Value> for dojo_types::schema::Value {
    fn from(val: &Value) -> Self {
        match val {
            Value::String(string) => dojo_types::schema::Value::String(unsafe {
                CStr::from_ptr(*string).to_string_lossy().into_owned()
            }),
            Value::Int(int) => dojo_types::schema::Value::Int(*int),
            Value::UInt(uint) => dojo_types::schema::Value::UInt(*uint),
            Value::Bool(bool) => dojo_types::schema::Value::Bool(*bool),
            Value::Bytes(bytes) => unsafe {
                dojo_types::schema::Value::Bytes(
                    std::slice::from_raw_parts(bytes.data, bytes.data_len).to_vec(),
                )
            },
        }
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_new(
    torii_url: *const c_char,
    rpc_url: *const c_char,
    world: &FieldElement,
    entities: *const EntityQuery,
    entities_len: usize,
    error: *mut Error,
) -> *mut ToriiClient {
    let torii_url = unsafe { CStr::from_ptr(torii_url).to_string_lossy().into_owned() };
    let rpc_url = unsafe { CStr::from_ptr(rpc_url).to_string_lossy().into_owned() };
    let entities = unsafe { std::slice::from_raw_parts(entities, entities_len).to_vec() };

    let client_future = TClient::new(
        torii_url,
        rpc_url,
        world.into(),
        Some(entities.iter().map(|e| e.into()).collect()),
    );

    let client = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(client_future);

    match client {
        Ok(client) => Box::into_raw(Box::new(ToriiClient(client))),
        Err(e) => {
            unsafe {
                *error = Error {
                    message: CString::new(e.to_string()).unwrap().into_raw(),
                };
            }
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_entity(
    client: *mut ToriiClient,
    entity: &EntityQuery,
    error: *mut Error,
) -> *mut Ty {
    let entity: dojo_types::schema::EntityQuery = (&entity.clone()).into();
    let entity_future = unsafe { (*client).0.entity(&entity) };


    let result = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(entity_future);

    match result {
        Ok(entity) => {
            if let Some(entity) = entity {
                let entity = entity.into();
                Box::into_raw(Box::new(entity))
            } else {
                std::ptr::null_mut()
            }
        },
        Err(e) => {
            unsafe {
                *error = Error {
                    message: CString::new(e.to_string()).unwrap().into_raw(),
                };
            }
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_add_entities_to_sync(
    client: *mut ToriiClient,
    entities: *const EntityQuery,
    entities_len: usize,
    error: *mut Error,
) {
    let entities = unsafe { std::slice::from_raw_parts(entities, entities_len).to_vec() };

    let client_future = unsafe {
        (*client)
            .0
            .add_entities_to_sync(entities.iter().map(|e| e.into()).collect())
    };

    let result = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(client_future);

    if let Err(e) = result {
        unsafe {
            *error = Error {
                message: CString::new(e.to_string()).unwrap().into_raw(),
            };
        }
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_remove_entities_to_sync(
    client: *mut ToriiClient,
    entities: *const EntityQuery,
    entities_len: usize,
    error: *mut Error,
) {
    let entities = unsafe { std::slice::from_raw_parts(entities, entities_len).to_vec() };

    let client_future = unsafe {
        (*client)
            .0
            .remove_entities_to_sync(entities.iter().map(|e| e.into()).collect())
    };

    let result = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(client_future);

    if let Err(e) = result {
        unsafe {
            *error = Error {
                message: CString::new(e.to_string()).unwrap().into_raw(),
            };
        }
    }
}

// This function takes a raw pointer to ToriiClient as an argument.
// It checks if the pointer is not null. If it's not, it converts the raw pointer
// back into a Box<ToriiClient>, which gets dropped at the end of the scope,
// deallocating the memory.
#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn client_free(client: *mut ToriiClient) {
    if !client.is_null() {
        unsafe {
            let _ = Box::from_raw(client);
        }
    }
}
