//! Minimal JS bindings for the torii client.

use std::cell::{OnceCell, RefCell};
use std::str::FromStr;
use std::sync::Arc;

use futures::lock::Mutex;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use starknet::accounts::{
    Account as _, Call, ConnectedAccount as _, ExecutionEncoding, SingleOwnerAccount,
};
use starknet::core::types::{BlockId, FieldElement};
use starknet::core::utils::{
    cairo_short_string_to_felt, get_contract_address, get_selector_from_name,
};
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::{JsonRpcClient, Provider as _};
use starknet::signers::{LocalWallet, SigningKey, VerifyingKey};
use starknet_crypto::Signature;
use torii_grpc::types::{Clause, KeysClause, Query};
use tsify::Tsify;
use wasm_bindgen::{prelude::*, JsObject};

use dojo_types::primitive::Primitive;
use dojo_types::schema::Ty;
use serde_json::Value;
use torii_grpc::types::schema::Entity;

use crate::constants;
use crate::types::{Account, CJsonRpcClient};
use crate::utils::watch_tx;

pub fn parse_entities_as_json_str(entities: Vec<Entity>) -> Value {
    entities
        .into_iter()
        .map(|entity| {
            let entity_key = format!("{:#x}", entity.hashed_keys);
            let models_map = entity
                .models
                .into_iter()
                .map(|model| {
                    let model_map = model
                        .members
                        .iter()
                        .map(|member| (member.name.to_owned(), parse_ty_as_json_str(&member.ty)))
                        .collect::<serde_json::Map<String, Value>>();

                    (model.name, model_map.into())
                })
                .collect::<serde_json::Map<String, Value>>();

            (entity_key, models_map.into())
        })
        .collect::<serde_json::Map<String, Value>>()
        .into()
}

pub fn parse_ty_as_json_str(ty: &Ty) -> Value {
    match ty {
        Ty::Primitive(primitive) => serde_json::json!({
            "type": primitive.to_string(),
            "value": primitive_value_json(*primitive)
        }),

        Ty::Struct(struct_ty) => serde_json::json!({
            "type": "struct",
            "value": struct_ty
            .children
            .iter()
            .map(|child| (child.name.to_owned(), parse_ty_as_json_str(&child.ty)))
            .collect::<serde_json::Map<String, Value>>()
        }),

        Ty::Enum(enum_ty) => serde_json::json!({
            "type": "enum",
            "value": if let Some(option) = enum_ty.option {
                option.into()
            } else {
                Value::Null
            }
        }),

        Ty::Tuple(_) => unimplemented!("tuple not supported"),
    }
}

fn primitive_value_json(primitive: Primitive) -> Value {
    match primitive {
        Primitive::Bool(Some(value)) => Value::Bool(value),
        Primitive::U8(Some(value)) => Value::Number(value.into()),
        Primitive::U16(Some(value)) => Value::Number(value.into()),
        Primitive::U32(Some(value)) => Value::Number(value.into()),
        Primitive::U64(Some(value)) => Value::Number(value.into()),
        Primitive::USize(Some(value)) => Value::Number(value.into()),
        Primitive::U128(Some(value)) => Value::String(format!("{value:#x}")),
        Primitive::U256(Some(value)) => Value::String(format!("{value:#x}")),
        Primitive::Felt252(Some(value)) => Value::String(format!("{value:#x}")),
        Primitive::ClassHash(Some(value)) => Value::String(format!("{value:#x}")),
        Primitive::ContractAddress(Some(value)) => Value::String(format!("{value:#x}")),
        _ => Value::Null,
    }
}

type JsFieldElement = JsValue;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EntityQuery {
    pub model: String,
    pub keys: Vec<FieldElement>,
}

impl From<EntityQuery> for KeysClause {
    fn from(value: EntityQuery) -> Self {
        Self {
            model: value.model,
            keys: value.keys,
        }
    }
}

#[wasm_bindgen(typescript_custom_section)]
pub const ENTITY_QUERY_TS: &'static str = r#"
export interface EntityQuery {
    model: string;
    keys: string[];
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "EntityQuery")]
    pub type IEntityQuery;
}

impl TryFrom<IEntityQuery> for EntityQuery {
    type Error = serde_wasm_bindgen::Error;

    fn try_from(value: IEntityQuery) -> Result<Self, Self::Error> {
        serde_wasm_bindgen::from_value(value.into())
    }
}

#[derive(Tsify, Serialize, Deserialize)]
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
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

#[wasm_bindgen]
pub struct Client {
    inner: torii_client::client::Client,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct JsSignature {
    pub r: String,
    pub s: String,
}

impl From<&Signature> for JsSignature {
    fn from(value: &Signature) -> Self {
        Self {
            r: format!("{:#x}", value.r),
            s: format!("{:#x}", value.s),
        }
    }
}

impl From<&JsSignature> for Signature {
    fn from(value: &JsSignature) -> Self {
        Self {
            r: FieldElement::from_str(value.r.as_str()).unwrap(),
            s: FieldElement::from_str(value.s.as_str()).unwrap(),
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct JsCalls {
    pub calls: Vec<JsCall>,
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct JsCall {
    pub to: String,
    pub selector: String,
    pub calldata: Vec<String>,
}

impl From<&JsCall> for starknet::accounts::Call {
    fn from(value: &JsCall) -> Self {
        Self {
            to: FieldElement::from_str(value.to.as_str()).unwrap(),
            selector: get_selector_from_name(value.selector.as_str()).unwrap(),
            calldata: value
                .calldata
                .iter()
                .map(|c| FieldElement::from_str(c.as_str()).unwrap())
                .collect(),
        }
    }
}

#[wasm_bindgen(js_name = signingKeyNew)]
pub fn signing_key_new() -> String {
    let private_key: SigningKey = SigningKey::from_random();

    format!("{:#x}", private_key.secret_scalar())
}

#[wasm_bindgen(js_name = signingKeySign)]
pub fn signing_key_sign(private_key: &str, hash: &str) -> Result<JsSignature, JsValue> {
    let private_key = FieldElement::from_hex_be(private_key);
    if let Err(e) = private_key {
        return Err(JsValue::from(format!("failed to parse private key: {e}")));
    }

    let hash = FieldElement::from_hex_be(hash);
    if let Err(e) = hash {
        return Err(JsValue::from(format!("failed to parse hash: {e}")));
    }

    let private_key = SigningKey::from_secret_scalar(private_key.unwrap());
    let sig = private_key.sign(&hash.unwrap());

    match sig {
        Ok(sig) => Result::Ok(JsSignature::from(&sig)),
        Err(e) => Err(JsValue::from(format!("failed to sign: {e}"))),
    }
}

#[wasm_bindgen(js_name = verifyingKeyNew)]
pub fn verifying_key_new(signing_key: &str) -> Result<String, JsValue> {
    let signing_key = FieldElement::from_hex_be(signing_key);
    if let Err(e) = signing_key {
        return Err(JsValue::from(format!("failed to parse signing key: {e}")));
    }

    let verifying_key = starknet_crypto::get_public_key(&signing_key.unwrap());

    Ok(format!("{:#x}", verifying_key))
}

#[wasm_bindgen(js_name = verifyingKeyVerify)]
pub fn verifying_key_verify(
    verifying_key: &str,
    hash: &str,
    signature: JsSignature,
) -> Result<bool, JsValue> {
    let verifying_key = FieldElement::from_hex_be(verifying_key);
    if let Err(e) = verifying_key {
        return Err(JsValue::from(format!("failed to parse verifying key: {e}")));
    }

    let verifying_key = VerifyingKey::from_scalar(verifying_key.unwrap());

    let hash = FieldElement::from_hex_be(hash);
    if let Err(e) = hash {
        return Err(JsValue::from(format!("failed to parse hash: {e}")));
    }

    let hash = &hash.unwrap();

    let signature = &Signature::from(&signature);

    match verifying_key.verify(hash, signature) {
        Ok(result) => Result::Ok(result),
        Err(e) => Err(JsValue::from(format!("failed to verify: {e}"))),
    }
}

#[wasm_bindgen(js_name = jsonrpcClientNew)]
pub unsafe fn jsonrpc_client_new(rpc_url: &str) -> Result<*mut CJsonRpcClient, JsValue> {
    let rpc_url = url::Url::parse(rpc_url);
    if let Err(e) = rpc_url {
        return Err(JsValue::from(format!("failed to parse rpc url: {e}")));
    }
    let rpc_url = rpc_url.unwrap();

    let rpc = JsonRpcClient::new(HttpTransport::new(rpc_url));

    Result::Ok(Box::into_raw(Box::new(CJsonRpcClient(rpc))))
}

#[wasm_bindgen(js_name = accountNew)]
pub async unsafe fn account_new(
    rpc: *mut CJsonRpcClient,
    private_key: &str,
    address: &str,
) -> Result<*mut Account, JsValue> {
    let private_key = FieldElement::from_hex_be(private_key);
    if let Err(e) = private_key {
        return Err(JsValue::from(format!("failed to parse private key: {e}")));
    }

    let private_key = private_key.unwrap();

    let address = FieldElement::from_hex_be(address);
    if let Err(e) = address {
        return Err(JsValue::from(format!("failed to parse address: {e}")));
    }

    let address = address.unwrap();

    let chain_id = (*rpc).0.chain_id().await;
    if let Err(e) = chain_id {
        return Err(JsValue::from(format!("failed to get chain id: {e}")));
    }

    let chain_id = chain_id.unwrap();

    let signer = LocalWallet::from_signing_key(SigningKey::from_secret_scalar(private_key));
    let account =
        SingleOwnerAccount::new(&(*rpc).0, signer, address, chain_id, ExecutionEncoding::New);

    Result::Ok(Box::into_raw(Box::new(Account(account))))
}

#[wasm_bindgen(js_name = accountAddress)]
pub unsafe fn account_address(account: *mut Account) -> Result<String, JsValue> {
    let address = (*account).0.address();
    Ok(format!("{:#x}", address))
}

#[wasm_bindgen(js_name = accountChainId)]
pub unsafe fn account_chain_id(account: *mut Account) -> Result<String, JsValue> {
    let chain_id = (*account).0.chain_id();
    Ok(format!("{:#x}", chain_id))
}

#[wasm_bindgen(js_name = accountSetBlockId)]
pub unsafe fn account_set_block_id(account: *mut Account, block_id: String) -> Result<(), JsValue> {
    let block_id = FieldElement::from_hex_be(&block_id)
        .map_err(|err| JsValue::from(format!("failed to parse block id: {err}")))?;
    (*account).0.set_block_id(BlockId::Hash(block_id));
    Ok(())
}

#[wasm_bindgen(js_name = accountExecuteRaw)]
pub async unsafe fn account_execute_raw(
    account: *mut Account,
    calldata: JsCalls,
) -> Result<String, JsValue> {
    let calldata = calldata
        .calls
        .iter()
        .map(|c| c.into())
        .collect::<Vec<Call>>();

    let call = (*account).0.execute(calldata);

    let result = call.send().await;

    match result {
        Ok(res) => Ok(format!("{:#x}", res.transaction_hash)),
        Err(e) => Err(JsValue::from_str(&e.to_string())),
    }
}

#[wasm_bindgen(js_name = waitForTransaction)]
pub async unsafe fn wait_for_transaction(
    rpc: *mut CJsonRpcClient,
    txn_hash: &str,
) -> Result<bool, JsValue> {
    let txn_hash = FieldElement::from_hex_be(txn_hash)
        .map_err(|err| JsValue::from(format!("failed to parse transaction hash: {err}")))?;
    let result: Result<(), anyhow::Error> = watch_tx(&(*rpc).0, txn_hash).await;

    match result {
        Ok(_) => Result::Ok(true),
        Err(e) => Err(JsValue::from_str(&e.to_string())),
    }
}

#[wasm_bindgen(js_name = hashGetContractAddress)]
pub fn hash_get_contract_address(
    class_hash: &str,
    salt: &str,
    constructor_calldata: Vec<String>,
    deployer_address: &str,
) -> Result<String, JsValue> {
    let class_hash = FieldElement::from_hex_be(class_hash)
        .map_err(|err| JsValue::from(format!("failed to parse class hash: {err}")))?;
    let salt = FieldElement::from_hex_be(salt)
        .map_err(|err| JsValue::from(format!("failed to parse salt: {err}")))?;
    let deployer_address = FieldElement::from_hex_be(deployer_address)
        .map_err(|err| JsValue::from(format!("failed to parse deployer address: {err}")))?;

    let constructor_calldata = constructor_calldata
        .into_iter()
        .map(|c| {
            FieldElement::from_hex_be(c.as_str()).map_err(|err| {
                JsValue::from(format!("failed to parse constructor calldata: {err}"))
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let address = get_contract_address(salt, class_hash, &constructor_calldata, deployer_address);

    Ok(format!("{:#x}", address))
}

#[wasm_bindgen(js_name = accountDeployBurner)]
pub async unsafe fn account_deploy_burner(
    master_account: *mut Account,
) -> Result<*mut Account, JsValue> {
    let signing_key = SigningKey::from_random();
    let verifying_key = signing_key.verifying_key();
    let address = get_contract_address(
        verifying_key.scalar(),
        constants::KATANA_ACCOUNT_CLASS_HASH,
        &[verifying_key.scalar()],
        FieldElement::ZERO,
    );
    let signer = LocalWallet::from_signing_key(signing_key);

    let chain_id = (*master_account).0.chain_id();

    let account = SingleOwnerAccount::new(
        *(*master_account).0.provider(),
        signer,
        address,
        chain_id,
        ExecutionEncoding::New,
    );

    // deploy the burner
    let exec = (*master_account).0.execute(vec![starknet::accounts::Call {
        to: constants::UDC_ADDRESS,
        calldata: vec![
            constants::KATANA_ACCOUNT_CLASS_HASH, // class_hash
            verifying_key.scalar(),               // salt
            FieldElement::ZERO,                   // deployer_address
            FieldElement::ONE,                    // constructor calldata length (1)
            verifying_key.scalar(),               // constructor calldata
        ],
        selector: get_selector_from_name("deployContract").unwrap(),
    }]);

    let result = exec.send().await;

    if let Err(e) = result {
        return Err(JsValue::from(format!(
            "failed to start torii client subscription service: {e}"
        )));
    }

    let result = result.unwrap();

    let _ = watch_tx((*master_account).0.provider(), result.transaction_hash).await;

    Result::Ok(Box::into_raw(Box::new(Account(account))))
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(js_name = getEntities)]
    pub async fn get_entities(&self, limit: u32, offset: u32) -> Result<JsValue, JsValue> {
        #[cfg(feature = "console-error-panic")]
        console_error_panic_hook::set_once();

        let results = self
            .inner
            .entities(Query {
                clause: None,
                limit,
                offset,
            })
            .await;

        match results {
            Ok(entities) => Ok(js_sys::JSON::parse(
                &parse_entities_as_json_str(entities).to_string(),
            )?),
            Err(err) => Err(JsValue::from(format!("failed to get entities: {err}"))),
        }
    }

    #[wasm_bindgen(js_name = getEntitiesByKeys)]
    pub async fn get_entities_by_keys(
        &self,
        model: &str,
        keys: Vec<JsFieldElement>,
        limit: u32,
        offset: u32,
    ) -> Result<JsValue, JsValue> {
        #[cfg(feature = "console-error-panic")]
        console_error_panic_hook::set_once();

        let keys = keys
            .into_iter()
            .map(serde_wasm_bindgen::from_value::<FieldElement>)
            .collect::<Result<Vec<FieldElement>, _>>()
            .map_err(|err| JsValue::from(format!("failed to parse entity keys: {err}")))?;

        let results = self
            .inner
            .entities(Query {
                clause: Some(Clause::Keys(KeysClause {
                    model: model.to_string(),
                    keys,
                })),
                limit,
                offset,
            })
            .await;

        match results {
            Ok(entities) => Ok(js_sys::JSON::parse(
                &parse_entities_as_json_str(entities).to_string(),
            )?),
            Err(err) => Err(JsValue::from(format!("failed to get entities: {err}"))),
        }
    }

    /// Retrieves the model value of an entity. Will fetch from remote if the requested entity is not one of the entities that are being synced.
    #[wasm_bindgen(js_name = getModelValue)]
    pub async fn get_model_value(
        &self,
        model: &str,
        keys: Vec<JsFieldElement>,
    ) -> Result<JsValue, JsValue> {
        #[cfg(feature = "console-error-panic")]
        console_error_panic_hook::set_once();

        let keys = keys
            .into_iter()
            .map(serde_wasm_bindgen::from_value::<FieldElement>)
            .collect::<Result<Vec<FieldElement>, _>>()
            .map_err(|err| JsValue::from(format!("failed to parse entity keys: {err}")))?;

        match self
            .inner
            .model(&KeysClause {
                model: model.to_string(),
                keys,
            })
            .await
        {
            Ok(Some(ty)) => Ok(js_sys::JSON::parse(&parse_ty_as_json_str(&ty).to_string())?),
            Ok(None) => Ok(JsValue::NULL),

            Err(err) => Err(JsValue::from(format!("failed to get entity: {err}"))),
        }
    }

    /// Register new entities to be synced.
    #[wasm_bindgen(js_name = addModelsToSync)]
    pub async unsafe fn add_models_to_sync(
        &self,
        models: Vec<IEntityQuery>,
    ) -> Result<(), JsValue> {
        log("adding models to sync...");

        #[cfg(feature = "console-error-panic")]
        console_error_panic_hook::set_once();

        let models = models
            .into_iter()
            .map(|e| TryInto::<KeysClause>::try_into(e))
            .collect::<Result<Vec<_>, _>>()?;

        self.inner
            .add_models_to_sync(models)
            .await
            .map_err(|err| JsValue::from(err.to_string()))
    }

    /// Remove the entities from being synced.
    #[wasm_bindgen(js_name = removeModelsToSync)]
    pub async unsafe fn remove_models_to_sync(
        &self,
        models: Vec<IEntityQuery>,
    ) -> Result<(), JsValue> {
        log("removing models to sync...");

        #[cfg(feature = "console-error-panic")]
        console_error_panic_hook::set_once();

        let models = models
            .into_iter()
            .map(|e| TryInto::<KeysClause>::try_into(e))
            .collect::<Result<Vec<_>, _>>()?;

        self.inner
            .remove_models_to_sync(models)
            .await
            .map_err(|err| JsValue::from(err.to_string()))
    }

    /// Register a callback to be called every time the specified synced entity's value changes.
    #[wasm_bindgen(js_name = onSyncModelChange)]
    pub async fn on_sync_model_change(
        &self,
        model: IEntityQuery,
        callback: js_sys::Function,
    ) -> Result<(), JsValue> {
        #[cfg(feature = "console-error-panic")]
        console_error_panic_hook::set_once();

        let model = serde_wasm_bindgen::from_value::<EntityQuery>(model.into())?;
        let name = cairo_short_string_to_felt(&model.model).expect("invalid model name");
        let mut rcv = self
            .inner
            .storage()
            .add_listener(name, &model.keys)
            .unwrap();

        wasm_bindgen_futures::spawn_local(async move {
            while rcv.next().await.is_some() {
                let _ = callback.call0(&JsValue::null());
            }
        });

        Ok(())
    }

    #[wasm_bindgen(js_name = onEntityUpdated)]
    pub async fn on_entity_updated(
        &self,
        ids: Option<Vec<String>>,
        callback: js_sys::Function,
    ) -> Result<(), JsValue> {
        #[cfg(feature = "console-error-panic")]
        console_error_panic_hook::set_once();

        let ids = ids
            .unwrap_or(vec![])
            .into_iter()
            .map(|id| {
                FieldElement::from_str(&id)
                    .map_err(|err| JsValue::from(format!("failed to parse entity id: {err}")))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let mut stream = self.inner.on_entity_updated(ids).await.unwrap();

        wasm_bindgen_futures::spawn_local(async move {
            while let Some(update) = stream.next().await {
                let entity = update.expect("no updated entity");
                let json_str = parse_entities_as_json_str(vec![entity]).to_string();
                let _ = callback.call1(
                    &JsValue::null(),
                    &js_sys::JSON::parse(&json_str).expect("json parse failed"),
                );
            }
        });

        Ok(())
    }

    #[wasm_bindgen(js_name = subscribeTopic)]
    pub async fn subscribe_topic(&mut self, topic: String) -> Result<bool, JsValue> {
        #[cfg(feature = "console-error-panic")]
        console_error_panic_hook::set_once();

        let sub = self
            .inner
            .subscribe_topic(topic)
            .await
            .map_err(|err| JsValue::from(err.to_string()))?;

        Ok(sub)
    }

    #[wasm_bindgen(js_name = unsubscribeTopic)]
    pub async fn unsubscribe_topic(&mut self, topic: String) -> Result<bool, JsValue> {
        #[cfg(feature = "console-error-panic")]
        console_error_panic_hook::set_once();

        let sub = self
            .inner
            .unsubscribe_topic(topic)
            .await
            .map_err(|err| JsValue::from(err.to_string()))?;

        Ok(sub)
    }

    #[wasm_bindgen(js_name = publishMessage)]
    pub async fn publish_message(
        &mut self,
        topic: &str,
        message: &[u8],
    ) -> Result<js_sys::Uint8Array, JsValue> {
        #[cfg(feature = "console-error-panic")]
        console_error_panic_hook::set_once();

        let message_id = self
            .inner
            .publish_message(topic, message)
            .await
            .map_err(|err| JsValue::from(err.to_string()))?;

        Ok(message_id.as_slice().into())
    }

    #[wasm_bindgen(js_name = onMessage)]
    pub async fn on_message(&self, callback: js_sys::Function) -> Result<(), JsValue> {
        #[cfg(feature = "console-error-panic")]
        console_error_panic_hook::set_once();

        let stream = self.inner.relay_client_stream();

        wasm_bindgen_futures::spawn_local(async move {
            while let Some(message) = stream.lock().await.next().await {
                let array = &js_sys::Array::new_with_length(5);
                array.set(
                    0,
                    JsValue::from_str(message.propagation_source.to_string().as_str()),
                );
                array.set(1, JsValue::from_str(message.source.to_string().as_str()));
                array.set(
                    2,
                    JsValue::from_str(message.message_id.to_string().as_str()),
                );
                array.set(3, JsValue::from_str(message.topic.as_str()));
                array.set(4, js_sys::Uint8Array::from(message.data.as_slice()).into());

                let _ = callback.apply(&JsValue::null(), array);
            }
        });

        Ok(())
    }
}

/// Create the a client with the given configurations.
#[wasm_bindgen(js_name = createClient)]
#[allow(non_snake_case)]
pub async fn create_client(
    initialModelsToSync: Vec<IEntityQuery>,
    config: ClientConfig,
) -> Result<Client, JsValue> {
    #[cfg(feature = "console-error-panic")]
    console_error_panic_hook::set_once();

    let ClientConfig {
        rpc_url,
        torii_url,
        relay_url,
        world_address,
    } = config;

    let models = initialModelsToSync
        .into_iter()
        .map(|e| TryInto::<KeysClause>::try_into(e))
        .collect::<Result<Vec<_>, _>>()?;

    let world_address = FieldElement::from_str(&world_address)
        .map_err(|err| JsValue::from(format!("failed to parse world address: {err}")))?;

    let client = torii_client::client::Client::new(
        torii_url,
        rpc_url,
        relay_url,
        world_address,
        Some(models),
    )
    .await
    .map_err(|err| JsValue::from(format!("failed to build client: {err}")))?;

    wasm_bindgen_futures::spawn_local(client.start_subscription().await.map_err(|err| {
        JsValue::from(format!(
            "failed to start torii client subscription service: {err}"
        ))
    })?);

    let relay_client_runner = client.relay_client_runner();
    wasm_bindgen_futures::spawn_local(async move {
        relay_client_runner.lock().await.run().await;
    });

    Ok(Client { inner: client })
}
