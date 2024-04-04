mod utils;

use std::str::FromStr;
use std::sync::Arc;

use crypto_bigint::U256;
use futures::StreamExt;
use js_sys::Array;
use serde::{Deserialize, Serialize};
use starknet::accounts::{
    Account as _, ConnectedAccount as _, ExecutionEncoding, SingleOwnerAccount,
};
use starknet::core::types::{FieldElement, FunctionCall};
use starknet::core::utils::{
    cairo_short_string_to_felt, get_contract_address, get_selector_from_name,
};
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::{JsonRpcClient, Provider as _};
use starknet::signers::{LocalWallet, SigningKey, VerifyingKey};
use starknet_crypto::Signature;
use torii_relay::typed_data::TypedData;
use torii_relay::types::Message;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use crate::constants;
use crate::types::{Account, Provider};
use crate::utils::watch_tx;
use crate::wasm::utils::{parse_entities_as_json_str, parse_ty_as_json_str};

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
pub struct Calls(Vec<Call>);

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Call {
    pub to: String,
    pub selector: String,
    pub calldata: Vec<String>,
}

impl From<&Call> for starknet::accounts::Call {
    fn from(value: &Call) -> Self {
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

impl From<&Call> for FunctionCall {
    fn from(value: &Call) -> Self {
        Self {
            contract_address: FieldElement::from_str(value.to.as_str()).unwrap(),
            entry_point_selector: get_selector_from_name(value.selector.as_str()).unwrap(),
            calldata: value
                .calldata
                .iter()
                .map(|c| FieldElement::from_str(c.as_str()).unwrap())
                .collect(),
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum BlockTag {
    Latest,
    Pending,
}

impl From<&BlockTag> for starknet::core::types::BlockTag {
    fn from(value: &BlockTag) -> Self {
        match value {
            BlockTag::Latest => starknet::core::types::BlockTag::Latest,
            BlockTag::Pending => starknet::core::types::BlockTag::Pending,
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

impl From<&BlockId> for starknet::core::types::BlockId {
    fn from(value: &BlockId) -> Self {
        match value {
            BlockId::Hash(hash) => {
                starknet::core::types::BlockId::Hash(FieldElement::from_str(hash.as_str()).unwrap())
            }
            BlockId::Number(number) => starknet::core::types::BlockId::Number(*number),
            BlockId::BlockTag(tag) => starknet::core::types::BlockId::Tag(tag.into()),
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Query {
    pub limit: u32,
    pub offset: u32,
    pub clause: Option<Clause>,
}

impl From<&Query> for torii_grpc::types::Query {
    fn from(value: &Query) -> Self {
        Self {
            limit: value.limit,
            offset: value.offset,
            clause: value.clause.as_ref().map(|c| c.into()),
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Clause {
    Keys(KeysClause),
    Member(MemberClause),
    Composite(CompositeClause),
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct KeysClauses(pub Vec<KeysClause>);

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct KeysClause {
    pub model: String,
    pub keys: Vec<String>,
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct MemberClause {
    pub model: String,
    pub member: String,
    pub operator: ComparisonOperator,
    pub value: Value,
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct CompositeClause {
    pub model: String,
    pub operator: LogicalOperator,
    pub clauses: Vec<Clause>,
}

impl From<&KeysClause> for torii_grpc::types::KeysClause {
    fn from(value: &KeysClause) -> Self {
        Self {
            model: value.model.to_string(),
            keys: value
                .keys
                .iter()
                .map(|k| FieldElement::from_str(k.as_str()).unwrap())
                .collect(),
        }
    }
}

impl From<&MemberClause> for torii_grpc::types::MemberClause {
    fn from(value: &MemberClause) -> Self {
        Self {
            model: value.model.to_string(),
            member: value.member.to_string(),
            operator: (&value.operator).into(),
            value: (&value.value).into(),
        }
    }
}

impl From<&CompositeClause> for torii_grpc::types::CompositeClause {
    fn from(value: &CompositeClause) -> Self {
        Self {
            model: value.model.to_string(),
            operator: (&value.operator).into(),
            clauses: value.clauses.iter().map(|c| c.into()).collect(),
        }
    }
}

impl From<&Clause> for torii_grpc::types::Clause {
    fn from(value: &Clause) -> Self {
        match value {
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

impl From<&LogicalOperator> for torii_grpc::types::LogicalOperator {
    fn from(value: &LogicalOperator) -> Self {
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
}

impl From<&ComparisonOperator> for torii_grpc::types::ComparisonOperator {
    fn from(value: &ComparisonOperator) -> Self {
        match value {
            ComparisonOperator::Eq => Self::Eq,
            ComparisonOperator::Neq => Self::Neq,
            ComparisonOperator::Gt => Self::Gt,
            ComparisonOperator::Gte => Self::Gte,
            ComparisonOperator::Lt => Self::Lt,
            ComparisonOperator::Lte => Self::Lte,
        }
    }
}

#[derive(Tsify, Serialize, Deserialize, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Value {
    pub primitive_type: Primitive,
    pub value_type: ValueType,
}

impl From<&Value> for torii_grpc::types::Value {
    fn from(value: &Value) -> Self {
        Self {
            primitive_type: (&value.primitive_type).into(),
            value_type: (&value.value_type).into(),
        }
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

impl From<&ValueType> for torii_grpc::types::ValueType {
    fn from(value: &ValueType) -> Self {
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
    U8(Option<u8>),
    U16(Option<u16>),
    U32(Option<u32>),
    U64(Option<u64>),
    U128(Option<String>),
    U256(Option<String>),
    USize(Option<u32>),
    Bool(Option<bool>),
    Felt252(Option<String>),
    ClassHash(Option<String>),
    ContractAddress(Option<String>),
}

impl From<&Primitive> for dojo_types::primitive::Primitive {
    fn from(value: &Primitive) -> Self {
        match value {
            Primitive::U8(Some(value)) => Self::U8(Some(*value)),
            Primitive::U16(Some(value)) => Self::U16(Some(*value)),
            Primitive::U32(Some(value)) => Self::U32(Some(*value)),
            Primitive::U64(Some(value)) => Self::U64(Some(*value)),
            Primitive::U128(Some(value)) => Self::U128(Some(u128::from_str(value).unwrap())),
            Primitive::U256(Some(value)) => Self::U256(Some(U256::from_be_hex(value.as_str()))),
            Primitive::USize(Some(value)) => Self::USize(Some(*value)),
            Primitive::Bool(Some(value)) => Self::Bool(Some(*value)),
            Primitive::Felt252(Some(value)) => {
                Self::Felt252(Some(FieldElement::from_str(value).unwrap()))
            }
            Primitive::ClassHash(Some(value)) => {
                Self::ClassHash(Some(FieldElement::from_str(value).unwrap()))
            }
            Primitive::ContractAddress(Some(value)) => {
                Self::ContractAddress(Some(FieldElement::from_str(value).unwrap()))
            }
            _ => unimplemented!(),
        }
    }
}

#[wasm_bindgen(js_name = typedDataEncode)]
pub fn typed_data_encode(typed_data: &str, address: &str) -> Result<String, JsValue> {
    let typed_data = serde_json::from_str::<TypedData>(&typed_data)
        .map_err(|err| JsValue::from(format!("failed to parse typed data: {err}")))?;

    let address = FieldElement::from_str(&address)
        .map_err(|err| JsValue::from(format!("failed to parse address: {err}")))?;

    typed_data
        .encode(address)
        .map(|felt| format!("{:#x}", felt))
        .map_err(|err| JsValue::from(err.to_string()))
}

#[wasm_bindgen(js_name = signingKeyNew)]
pub fn signing_key_new() -> String {
    let private_key: SigningKey = SigningKey::from_random();

    format!("{:#x}", private_key.secret_scalar())
}

#[wasm_bindgen(js_name = signingKeySign)]
pub fn signing_key_sign(private_key: &str, hash: &str) -> Result<JsSignature, JsValue> {
    let private_key = FieldElement::from_str(private_key);
    if let Err(e) = private_key {
        return Err(JsValue::from(format!("failed to parse private key: {e}")));
    }

    let hash = FieldElement::from_str(hash);
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
    let signing_key = FieldElement::from_str(signing_key);
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
    let verifying_key = FieldElement::from_str(verifying_key);
    if let Err(e) = verifying_key {
        return Err(JsValue::from(format!("failed to parse verifying key: {e}")));
    }

    let verifying_key = VerifyingKey::from_scalar(verifying_key.unwrap());

    let hash = FieldElement::from_str(hash);
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

#[wasm_bindgen(js_name = createProvider)]
pub unsafe fn create_provider(rpc_url: &str) -> Result<Provider, JsValue> {
    let rpc_url = url::Url::parse(rpc_url);
    if let Err(e) = rpc_url {
        return Err(JsValue::from(format!("failed to parse rpc url: {e}")));
    }
    let rpc_url = rpc_url.unwrap();

    let rpc = JsonRpcClient::new(HttpTransport::new(rpc_url));

    Result::Ok(Provider(Arc::new(rpc)))
}

#[wasm_bindgen]
impl Provider {
    #[wasm_bindgen(js_name = createAccount)]
    pub async unsafe fn create_account(
        &self,
        private_key: &str,
        address: &str,
    ) -> Result<Account, JsValue> {
        let private_key = FieldElement::from_str(private_key);
        if let Err(e) = private_key {
            return Err(JsValue::from(format!("failed to parse private key: {e}")));
        }

        let private_key = private_key.unwrap();

        let address = FieldElement::from_str(address);
        if let Err(e) = address {
            return Err(JsValue::from(format!("failed to parse address: {e}")));
        }

        let address = address.unwrap();

        let chain_id = self.0.chain_id().await;
        if let Err(e) = chain_id {
            return Err(JsValue::from(format!("failed to get chain id: {e}")));
        }

        let chain_id = chain_id.unwrap();

        let signer = LocalWallet::from_signing_key(SigningKey::from_secret_scalar(private_key));
        let account = SingleOwnerAccount::new(
            self.0.clone(),
            signer,
            address,
            chain_id,
            ExecutionEncoding::New,
        );

        Result::Ok(Account(account))
    }

    #[wasm_bindgen(js_name = call)]
    pub async unsafe fn call(&self, call: Call, block_id: BlockId) -> Result<Array, JsValue> {
        let result = self
            .0
            .call::<FunctionCall, starknet::core::types::BlockId>(
                (&call).into(),
                (&block_id).into(),
            )
            .await;

        match result {
            Ok(res) => Ok(res
                .iter()
                .map(|f| JsValue::from(format!("{:#x}", f)))
                .collect()),
            Err(e) => Err(JsValue::from_str(&e.to_string())),
        }
    }

    #[wasm_bindgen(js_name = waitForTransaction)]
    pub async unsafe fn wait_for_transaction(&self, txn_hash: &str) -> Result<bool, JsValue> {
        let txn_hash = FieldElement::from_str(txn_hash)
            .map_err(|err| JsValue::from(format!("failed to parse transaction hash: {err}")))?;
        let result: Result<(), anyhow::Error> = watch_tx(&self.0, txn_hash).await;

        match result {
            Ok(_) => Result::Ok(true),
            Err(e) => Err(JsValue::from_str(&e.to_string())),
        }
    }
}

#[wasm_bindgen]
impl Account {
    #[wasm_bindgen(js_name = address)]
    pub unsafe fn address(&self) -> Result<String, JsValue> {
        let address = self.0.address();
        Ok(format!("{:#x}", address))
    }

    #[wasm_bindgen(js_name = chainId)]
    pub unsafe fn chain_id(&self) -> Result<String, JsValue> {
        let chain_id = self.0.chain_id();
        Ok(format!("{:#x}", chain_id))
    }

    #[wasm_bindgen(js_name = setBlockId)]
    pub unsafe fn set_block_id(&mut self, block_id: String) -> Result<(), JsValue> {
        let block_id = FieldElement::from_str(&block_id)
            .map_err(|err| JsValue::from(format!("failed to parse block id: {err}")))?;
        self.0
            .set_block_id(starknet::core::types::BlockId::Hash(block_id));
        Ok(())
    }

    #[wasm_bindgen(js_name = executeRaw)]
    pub async unsafe fn execute_raw(&self, calldata: Calls) -> Result<String, JsValue> {
        let calldata = calldata.0.iter().map(|c| c.into()).collect();

        let call = self.0.execute(calldata);

        let result = call.send().await;

        match result {
            Ok(res) => Ok(format!("{:#x}", res.transaction_hash)),
            Err(e) => Err(JsValue::from_str(&e.to_string())),
        }
    }

    #[wasm_bindgen(js_name = deployBurner)]
    pub async unsafe fn deploy_burner(&self) -> Result<Account, JsValue> {
        let signing_key = SigningKey::from_random();
        let verifying_key = signing_key.verifying_key();
        let address = get_contract_address(
            verifying_key.scalar(),
            constants::KATANA_ACCOUNT_CLASS_HASH,
            &[verifying_key.scalar()],
            FieldElement::ZERO,
        );
        let signer = LocalWallet::from_signing_key(signing_key);

        let chain_id = self.0.chain_id();

        let provider = self.0.provider().clone();
        let account =
            SingleOwnerAccount::new(provider, signer, address, chain_id, ExecutionEncoding::New);

        // deploy the burner
        let exec = self.0.execute(vec![starknet::accounts::Call {
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

        let _ = watch_tx(self.0.provider(), result.transaction_hash).await;

        Result::Ok(Account(account))
    }
}

#[wasm_bindgen(js_name = hashGetContractAddress)]
pub fn hash_get_contract_address(
    class_hash: &str,
    salt: &str,
    constructor_calldata: Vec<String>,
    deployer_address: &str,
) -> Result<String, JsValue> {
    let class_hash = FieldElement::from_str(class_hash)
        .map_err(|err| JsValue::from(format!("failed to parse class hash: {err}")))?;
    let salt = FieldElement::from_str(salt)
        .map_err(|err| JsValue::from(format!("failed to parse salt: {err}")))?;
    let deployer_address = FieldElement::from_str(deployer_address)
        .map_err(|err| JsValue::from(format!("failed to parse deployer address: {err}")))?;

    let constructor_calldata = constructor_calldata
        .into_iter()
        .map(|c| {
            FieldElement::from_str(c.as_str()).map_err(|err| {
                JsValue::from(format!("failed to parse constructor calldata: {err}"))
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let address = get_contract_address(salt, class_hash, &constructor_calldata, deployer_address);

    Ok(format!("{:#x}", address))
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(js_name = getEntities)]
    pub async fn get_entities(&self, query: Query) -> Result<JsValue, JsValue> {
        #[cfg(feature = "console-error-panic")]
        console_error_panic_hook::set_once();

        let results = self.inner.entities((&query).into()).await;

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
        keys: Vec<String>,
    ) -> Result<JsValue, JsValue> {
        #[cfg(feature = "console-error-panic")]
        console_error_panic_hook::set_once();

        let keys = keys
            .into_iter()
            .map(|k| FieldElement::from_str(k.as_str()))
            .collect::<Result<Vec<FieldElement>, _>>()
            .map_err(|err| JsValue::from(format!("failed to parse entity keys: {err}")))?;

        match self
            .inner
            .model(&torii_grpc::types::KeysClause {
                model: model.to_string(),
                keys,
            })
            .await
        {
            Ok(Some(ty)) => Ok(js_sys::JSON::parse(&parse_ty_as_json_str(&ty, false).to_string())?),
            Ok(None) => Ok(JsValue::NULL),

            Err(err) => Err(JsValue::from(format!("failed to get entity: {err}"))),
        }
    }

    /// Register new entities to be synced.
    #[wasm_bindgen(js_name = addModelsToSync)]
    pub async unsafe fn add_models_to_sync(&self, models: KeysClauses) -> Result<(), JsValue> {
        log("adding models to sync...");

        #[cfg(feature = "console-error-panic")]
        console_error_panic_hook::set_once();

        let models = models.0.iter().map(|e| e.into()).collect();

        self.inner
            .add_models_to_sync(models)
            .await
            .map_err(|err| JsValue::from(err.to_string()))
    }

    /// Remove the entities from being synced.
    #[wasm_bindgen(js_name = removeModelsToSync)]
    pub async unsafe fn remove_models_to_sync(&self, models: KeysClauses) -> Result<(), JsValue> {
        log("removing models to sync...");

        #[cfg(feature = "console-error-panic")]
        console_error_panic_hook::set_once();

        let models = models.0.iter().map(|e| e.into()).collect();

        self.inner
            .remove_models_to_sync(models)
            .await
            .map_err(|err| JsValue::from(err.to_string()))
    }

    /// Register a callback to be called every time the specified synced entity's value changes.
    #[wasm_bindgen(js_name = onSyncModelChange)]
    pub async fn on_sync_model_change(
        &self,
        model: KeysClause,
        callback: js_sys::Function,
    ) -> Result<(), JsValue> {
        #[cfg(feature = "console-error-panic")]
        console_error_panic_hook::set_once();

        let name = cairo_short_string_to_felt(&model.model).expect("invalid model name");
        let mut rcv = self
            .inner
            .storage()
            .add_listener(
                name,
                &model
                    .keys
                    .iter()
                    .map(|k| FieldElement::from_str(k.as_str()))
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap(),
            )
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
            .unwrap_or_default()
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

    #[wasm_bindgen(js_name = publishMessage)]
    pub async fn publish_message(
        &mut self,
        message: &str,
        signature: JsSignature,
    ) -> Result<js_sys::Uint8Array, JsValue> {
        #[cfg(feature = "console-error-panic")]
        console_error_panic_hook::set_once();

        let message = serde_json::from_str(message)
            .map_err(|err| JsValue::from(format!("failed to parse message: {err}")))?;

        let message_id = self
            .inner
            .publish_message(Message {
                message,
                signature_r: FieldElement::from_str(signature.r.as_str())
                    .map_err(|err| JsValue::from(err.to_string()))?,
                signature_s: FieldElement::from_str(signature.s.as_str())
                    .map_err(|err| JsValue::from(err.to_string()))?,
            })
            .await
            .map_err(|err| JsValue::from(err.to_string()))?;

        Ok(message_id.as_slice().into())
    }
}

/// Create the a client with the given configurations.
#[wasm_bindgen(js_name = createClient)]
#[allow(non_snake_case)]
pub async fn create_client(
    initialModelsToSync: KeysClauses,
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

    let models = initialModelsToSync.0.iter().map(|e| e.into()).collect();

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

    let relay_runner = client.relay_runner();
    wasm_bindgen_futures::spawn_local(async move {
        relay_runner.lock().await.run().await;
    });

    Ok(Client { inner: client })
}
