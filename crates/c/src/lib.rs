mod types;

use std::ffi::{c_void, CStr, CString};
use std::ops::Deref;
use std::os::raw::c_char;
use std::sync::Arc;
use std::time::Duration;

use cainome::cairo_serde::{self, ByteArray, CairoSerde};
use crypto_bigint::U256;
use dojo_core::constants;
use dojo_core::utils::watch_tx;
use dojo_world::contracts::naming::compute_selector_from_tag;
use futures::FutureExt;
use lazy_static::lazy_static;
use starknet::accounts::{
    Account as StarknetAccount, ConnectedAccount, ExecutionEncoding, SingleOwnerAccount,
};
use starknet::core::types::{FunctionCall, TypedData};
use starknet::core::utils::get_contract_address;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::{JsonRpcClient, Provider as _};
use starknet::signers::{LocalWallet, SigningKey, VerifyingKey};
use starknet_crypto::{poseidon_hash_many, Felt};
use stream_cancel::{StreamExt as _, Tripwire};
use tokio::runtime::Runtime;
use tokio::sync::oneshot;
use tokio::time::sleep;
use tokio_stream::StreamExt;
use torii_client::Client as TClient;
use torii_proto::Message;
use types::{
    Achievement, AchievementProgression, Activity, AggregationEntry, BlockId, CArray, COption,
    Call, Clause, Contract, Controller, Entity, Error, Event, KeysClause, Page,
    PlayerAchievementEntry, Query, Result, Signature, Struct, Token, TokenBalance, TokenContract,
    TokenTransfer, TokenTransferQuery, ToriiClient, Ty, World,
};

use crate::types::{
    Account, AchievementQuery, ActivityQuery, AggregationQuery, ContractQuery, ControllerQuery,
    PlayerAchievementQuery, Provider, Subscription, TokenBalanceQuery, TokenContractQuery,
    TokenQuery, Transaction, TransactionFilter, TransactionQuery,
};

lazy_static! {
    static ref RUNTIME: Arc<Runtime> =
        Arc::new(Runtime::new().expect("Failed to create Tokio runtime"));
}

#[allow(clippy::missing_safety_doc)]
mod ffi {
    use super::*;
    /// Creates a new Torii client instance
    ///
    /// # Parameters
    /// * `torii_url` - URL of the Torii server
    /// * `libp2p_relay_url` - URL of the libp2p relay server
    ///
    /// # Returns
    /// Result containing pointer to new ToriiClient instance or error
    #[no_mangle]
    pub unsafe extern "C" fn client_new(torii_url: *const c_char) -> Result<*mut ToriiClient> {
        let torii_url = unsafe { CStr::from_ptr(torii_url).to_string_lossy().into_owned() };
        let client_future = TClient::new(torii_url);

        let client = match RUNTIME.block_on(client_future) {
            Ok(client) => client,
            Err(e) => return Result::Err(e.into()),
        };

        Result::Ok(Box::into_raw(Box::new(ToriiClient { inner: client, logger: None })))
    }

    /// Sets a logger callback function for the client
    ///
    /// # Parameters
    /// * `client` - Pointer to ToriiClient instance
    /// * `logger` - Callback function that takes a C string parameter
    #[no_mangle]
    pub unsafe extern "C" fn client_set_logger(
        client: *mut ToriiClient,
        logger: extern "C" fn(*const c_char),
    ) {
        unsafe {
            (*client).logger = Some(logger);
        }
    }

    /// Publishes a message to the network
    ///
    /// # Parameters
    /// * `client` - Pointer to ToriiClient instance
    /// * `message` - JSON string containing typed data message
    /// * `signature_felts` - Array of field elements containing signature
    /// * `signature_felts_len` - Length of signature array
    ///
    /// # Returns
    /// Result containing byte array or error
    #[no_mangle]
    pub unsafe extern "C" fn client_publish_message(
        client: *mut ToriiClient,
        message: types::Message,
    ) -> Result<*const c_char> {
        let client_future = unsafe { (*client).inner.publish_message(message.into()) };

        match RUNTIME.block_on(client_future) {
            Ok(data) => Result::Ok(CString::new(data).unwrap().into_raw() as *const c_char),
            Err(e) => Result::Err(e.into()),
        }
    }

    /// Publishes multiple messages to the network
    ///
    /// # Parameters
    /// * `client` - Pointer to ToriiClient instance
    /// * `messages` - Array of Message structs
    /// * `messages_len` - Length of messages array
    ///
    /// # Returns
    /// Result containing array of message IDs or error
    #[no_mangle]
    pub unsafe extern "C" fn client_publish_message_batch(
        client: *mut ToriiClient,
        messages: *const types::Message,
        messages_len: usize,
    ) -> Result<CArray<*const c_char>> {
        let messages = unsafe { std::slice::from_raw_parts(messages, messages_len) };
        let messages: Vec<Message> = messages.iter().cloned().map(|msg| msg.into()).collect();
        let client_future = unsafe { (*client).inner.publish_message_batch(messages) };

        match RUNTIME.block_on(client_future) {
            Ok(message_ids) => {
                let ids: Vec<*const c_char> = message_ids
                    .into_iter()
                    .map(|id| CString::new(id).unwrap().into_raw() as *const c_char)
                    .collect();
                Result::Ok(ids.into())
            }
            Err(e) => Result::Err(e.into()),
        }
    }

    /// Retrieves controllers for the given contract addresses
    ///
    /// # Parameters
    /// * `client` - Pointer to ToriiClient instance
    /// * `contract_addresses` - Array of contract addresses. If empty, all controllers will be
    ///   returned.
    ///
    /// # Returns
    /// Result containing controllers or error
    #[no_mangle]
    pub unsafe extern "C" fn client_controllers(
        client: *mut ToriiClient,
        query: ControllerQuery,
    ) -> Result<Page<Controller>> {
        let query = query.into();
        let controllers_future = unsafe { (*client).inner.controllers(query) };

        match RUNTIME.block_on(controllers_future) {
            Ok(controllers) => Result::Ok(controllers.into()),
            Err(e) => Result::Err(e.into()),
        }
    }

    /// Queries entities matching given criteria
    ///
    /// # Parameters
    /// * `client` - Pointer to ToriiClient instance
    /// * `query` - Query parameters
    ///
    /// # Returns
    /// Result containing array of matching entities or error
    #[no_mangle]
    pub unsafe extern "C" fn client_entities(
        client: *mut ToriiClient,
        query: Query,
    ) -> Result<Page<Entity>> {
        let query = query.clone().into();
        let entities_future = unsafe { (*client).inner.entities(query) };

        match RUNTIME.block_on(entities_future) {
            Ok(entities) => Result::Ok(entities.into()),
            Err(e) => Result::Err(e.into()),
        }
    }

    /// Retrieves event messages matching the given query
    ///
    /// # Parameters
    /// * `client` - Pointer to ToriiClient instance
    /// * `query` - Query parameters
    /// * `historical` - Whether to include historical messages
    ///
    /// # Returns
    /// Result containing array of matching event message entities or error
    #[no_mangle]
    pub unsafe extern "C" fn client_event_messages(
        client: *mut ToriiClient,
        query: Query,
    ) -> Result<Page<Entity>> {
        let query = query.clone().into();
        let event_messages_future = unsafe { (*client).inner.event_messages(query) };

        match RUNTIME.block_on(event_messages_future) {
            Ok(event_messages) => Result::Ok(event_messages.into()),
            Err(e) => Result::Err(e.into()),
        }
    }

    /// Gets the world metadata for the client
    ///
    /// # Parameters
    /// * `client` - Pointer to ToriiClient instance
    ///
    /// # Returns
    /// World structure containing world information
    #[no_mangle]
    pub unsafe extern "C" fn client_worlds(
        client: *mut ToriiClient,
        world_addresses: *const types::FieldElement,
        world_addresses_len: usize,
    ) -> Result<CArray<World>> {
        let world_addresses =
            unsafe { std::slice::from_raw_parts(world_addresses, world_addresses_len) };
        let world_addresses = world_addresses.iter().map(|addr| addr.clone().into()).collect();
        let metadata_future = unsafe { (*client).inner.worlds(world_addresses) };
        match RUNTIME.block_on(metadata_future) {
            Ok(metadata) => Result::Ok(metadata.into()),
            Err(e) => Result::Err(e.into()),
        }
    }

    /// Retrieves transactions matching the given query
    ///
    /// # Parameters
    /// * `client` - Pointer to ToriiClient instance
    /// * `query` - Query parameters
    ///
    /// # Returns
    /// Result containing array of matching transactions or error
    #[no_mangle]
    pub unsafe extern "C" fn client_transactions(
        client: *mut ToriiClient,
        query: TransactionQuery,
    ) -> Result<Page<Transaction>> {
        let query = query.into();
        let transactions_future = unsafe { (*client).inner.transactions(query) };

        match RUNTIME.block_on(transactions_future) {
            Ok(transactions) => Result::Ok(transactions.into()),
            Err(e) => Result::Err(e.into()),
        }
    }

    /// Subscribes to transaction updates
    ///
    /// # Parameters
    /// * `client` - Pointer to ToriiClient instance
    /// * `filter` - Filter parameters
    /// * `callback` - Function called when updates occur
    ///
    /// # Returns
    /// Result containing pointer to Subscription or error
    #[no_mangle]
    pub unsafe extern "C" fn client_on_transaction(
        client: *mut ToriiClient,
        filter: COption<TransactionFilter>,
        callback: unsafe extern "C" fn(Transaction),
    ) -> Result<*mut Subscription> {
        let client = Arc::new(unsafe { &*client });
        let (trigger, tripwire) = Tripwire::new();

        let (sub_id_tx, sub_id_rx) = oneshot::channel();
        let mut sub_id_tx = Some(sub_id_tx);

        let filter: Option<torii_proto::TransactionFilter> = filter.map(|f| f.into()).into();

        // Spawn a new thread to handle the stream and reconnections
        let client_clone = client.clone();
        RUNTIME.spawn(async move {
            let mut backoff = Duration::from_secs(1);
            let max_backoff = Duration::from_secs(60);

            loop {
                let rcv = client_clone.inner.on_transaction(filter.clone()).await;
                if let Ok(rcv) = rcv {
                    backoff = Duration::from_secs(1); // Reset backoff on successful connection

                    let mut rcv = rcv.take_until_if(tripwire.clone());

                    while let Some(Ok(transaction)) = rcv.next().await {
                        if let Some(tx) = sub_id_tx.take() {
                            tx.send(0).expect("Failed to send subscription ID");
                        } else {
                            callback(transaction.into());
                        }
                    }
                }

                // If we've reached this point, the stream has ended (possibly due to disconnection)
                // We'll try to reconnect after a delay, unless the tripwire has been triggered
                if tripwire.clone().now_or_never().unwrap_or_default() {
                    break; // Exit the loop if the subscription has been cancelled
                }
                sleep(backoff).await;
                backoff = std::cmp::min(backoff * 2, max_backoff);
            }
        });

        let subscription_id = match RUNTIME.block_on(sub_id_rx) {
            Ok(id) => id,
            Err(_) => {
                return Result::Err(Error {
                    message: CString::new("Failed to establish entity subscription")
                        .unwrap()
                        .into_raw(),
                });
            }
        };

        let subscription = Subscription { id: subscription_id, trigger };
        Result::Ok(Box::into_raw(Box::new(subscription)))
    }

    /// Subscribes to entity state updates
    ///
    /// # Parameters
    /// * `client` - Pointer to ToriiClient instance
    /// * `clauses` - Array of entity key clauses to filter updates
    /// * `clauses_len` - Length of clauses array
    /// * `callback` - Function called when updates occur
    ///
    /// # Returns
    /// Result containing pointer to Subscription or error
    #[no_mangle]
    pub unsafe extern "C" fn client_on_entity_state_update(
        client: *mut ToriiClient,
        clause: COption<Clause>,
        world_addresses: *const types::FieldElement,
        world_addresses_len: usize,
        callback: unsafe extern "C" fn(Entity),
    ) -> Result<*mut Subscription> {
        let client = Arc::new(unsafe { &*client });
        let (trigger, tripwire) = Tripwire::new();

        let (sub_id_tx, sub_id_rx) = oneshot::channel();
        let mut sub_id_tx = Some(sub_id_tx);

        let clause: Option<torii_proto::Clause> = clause.map(|c| c.into()).into();
        let world_addresses =
            unsafe { std::slice::from_raw_parts(world_addresses, world_addresses_len) };
        let world_addresses =
            world_addresses.iter().map(|addr| addr.clone().into()).collect::<Vec<Felt>>();
        // Spawn a new thread to handle the stream and reconnections
        let client_clone = client.clone();
        RUNTIME.spawn(async move {
            let mut backoff = Duration::from_secs(1);
            let max_backoff = Duration::from_secs(60);

            loop {
                let rcv = client_clone
                    .inner
                    .on_entity_updated(clause.clone(), world_addresses.clone())
                    .await;
                if let Ok(rcv) = rcv {
                    backoff = Duration::from_secs(1); // Reset backoff on successful connection

                    let mut rcv = rcv.take_until_if(tripwire.clone());

                    while let Some(Ok((id, entity))) = rcv.next().await {
                        if let Some(tx) = sub_id_tx.take() {
                            tx.send(id).expect("Failed to send subscription ID");
                        } else {
                            callback(entity.into());
                        }
                    }
                }

                // If we've reached this point, the stream has ended (possibly due to disconnection)
                // We'll try to reconnect after a delay, unless the tripwire has been triggered
                if tripwire.clone().now_or_never().unwrap_or_default() {
                    break; // Exit the loop if the subscription has been cancelled
                }
                sleep(backoff).await;
                backoff = std::cmp::min(backoff * 2, max_backoff);
            }
        });

        let subscription_id = match RUNTIME.block_on(sub_id_rx) {
            Ok(id) => id,
            Err(_) => {
                return Result::Err(Error {
                    message: CString::new("Failed to establish entity subscription")
                        .unwrap()
                        .into_raw(),
                });
            }
        };

        let subscription = Subscription { id: subscription_id, trigger };
        Result::Ok(Box::into_raw(Box::new(subscription)))
    }

    /// Updates an existing entity subscription with new clauses
    ///
    /// # Parameters
    /// * `client` - Pointer to ToriiClient instance
    /// * `subscription` - Pointer to existing Subscription
    /// * `clauses` - New array of entity key clauses
    /// * `clauses_len` - Length of new clauses array
    ///
    /// # Returns
    /// Result containing success boolean or error
    #[no_mangle]
    pub unsafe extern "C" fn client_update_entity_subscription(
        client: *mut ToriiClient,
        subscription: *mut Subscription,
        clause: COption<Clause>,
        world_addresses: *const types::FieldElement,
        world_addresses_len: usize,
    ) -> Result<bool> {
        let clause: Option<torii_proto::Clause> = clause.map(|c| c.into()).into();
        let world_addresses =
            unsafe { std::slice::from_raw_parts(world_addresses, world_addresses_len) };
        let world_addresses = world_addresses.iter().map(|addr| addr.clone().into()).collect();
        match RUNTIME.block_on((*client).inner.update_entity_subscription(
            (*subscription).id,
            clause,
            world_addresses,
        )) {
            Ok(_) => Result::Ok(true),
            Err(e) => Result::Err(e.into()),
        }
    }

    /// Retrieves aggregations (leaderboards, stats, rankings) matching query parameter
    ///
    /// # Parameters
    /// * `client` - Pointer to ToriiClient instance
    /// * `query` - AggregationQuery containing aggregator_ids, entity_ids, and pagination
    ///
    /// # Returns
    /// Result containing Page of AggregationEntry or error
    #[no_mangle]
    pub unsafe extern "C" fn client_aggregations(
        client: *mut ToriiClient,
        query: AggregationQuery,
    ) -> Result<Page<AggregationEntry>> {
        let query = query.into();
        let aggregations_future = unsafe { (*client).inner.aggregations(query) };

        match RUNTIME.block_on(aggregations_future) {
            Ok(aggregations) => Result::Ok(aggregations.into()),
            Err(e) => Result::Err(e.into()),
        }
    }

    /// Subscribes to aggregation updates (leaderboards, stats, rankings)
    ///
    /// # Parameters
    /// * `client` - Pointer to ToriiClient instance
    /// * `aggregator_ids` - Array of aggregator IDs to subscribe to
    /// * `aggregator_ids_len` - Length of aggregator_ids array
    /// * `entity_ids` - Array of entity IDs to subscribe to
    /// * `entity_ids_len` - Length of entity_ids array
    /// * `callback` - Function called when updates occur
    ///
    /// # Returns
    /// Result containing pointer to Subscription or error
    #[no_mangle]
    pub unsafe extern "C" fn client_on_aggregation_update(
        client: *mut ToriiClient,
        aggregator_ids: *const *const c_char,
        aggregator_ids_len: usize,
        entity_ids: *const *const c_char,
        entity_ids_len: usize,
        callback: unsafe extern "C" fn(AggregationEntry),
    ) -> Result<*mut Subscription> {
        let client = Arc::new(unsafe { &*client });

        // Convert aggregator_ids array to Vec<String> if not empty
        let aggregator_ids = if aggregator_ids.is_null() || aggregator_ids_len == 0 {
            Vec::new()
        } else {
            let ids = unsafe { std::slice::from_raw_parts(aggregator_ids, aggregator_ids_len) };
            ids.iter()
                .map(|id| unsafe { CStr::from_ptr(*id).to_string_lossy().to_string() })
                .collect::<Vec<String>>()
        };

        let entity_ids = if entity_ids.is_null() || entity_ids_len == 0 {
            Vec::new()
        } else {
            let ids = unsafe { std::slice::from_raw_parts(entity_ids, entity_ids_len) };
            ids.iter()
                .map(|id| unsafe { CStr::from_ptr(*id).to_string_lossy().to_string() })
                .collect::<Vec<String>>()
        };

        let (sub_id_tx, sub_id_rx) = oneshot::channel();
        let mut sub_id_tx = Some(sub_id_tx);
        let (trigger, tripwire) = Tripwire::new();

        // Spawn a new thread to handle the stream and reconnections
        let client_clone = client.clone();
        RUNTIME.spawn(async move {
            let mut backoff = Duration::from_secs(1);
            let max_backoff = Duration::from_secs(60);

            loop {
                let rcv = client_clone
                    .inner
                    .on_aggregation_updated(aggregator_ids.clone(), entity_ids.clone())
                    .await;

                if let Ok(rcv) = rcv {
                    backoff = Duration::from_secs(1); // Reset backoff on successful connection

                    let mut rcv = rcv.take_until_if(tripwire.clone());

                    while let Some(Ok((id, aggregation_entry))) = rcv.next().await {
                        // Our first message will be the subscription ID
                        if let Some(tx) = sub_id_tx.take() {
                            tx.send(id).expect("Failed to send subscription ID");
                        } else {
                            let aggregation_entry: AggregationEntry = aggregation_entry.into();
                            callback(aggregation_entry);
                        }
                    }
                }

                // If we've reached this point, the stream has ended (possibly due to disconnection)
                // We'll try to reconnect after a delay, unless the tripwire has been triggered
                if tripwire.clone().now_or_never().unwrap_or_default() {
                    break; // Exit the loop if the subscription has been cancelled
                }
                sleep(backoff).await;
                backoff = std::cmp::min(backoff * 2, max_backoff);
            }
        });

        let subscription_id = match RUNTIME.block_on(sub_id_rx) {
            Ok(id) => id,
            Err(_) => {
                return Result::Err(Error {
                    message: CString::new("Failed to establish aggregation subscription")
                        .unwrap()
                        .into_raw(),
                });
            }
        };

        let subscription = Subscription { id: subscription_id, trigger };

        Result::Ok(Box::into_raw(Box::new(subscription)))
    }

    /// Updates an existing aggregation subscription with new parameters
    ///
    /// # Parameters
    /// * `client` - Pointer to ToriiClient instance
    /// * `subscription` - Pointer to existing Subscription
    /// * `aggregator_ids` - Array of aggregator IDs to subscribe to
    /// * `aggregator_ids_len` - Length of aggregator_ids array
    /// * `entity_ids` - Array of entity IDs to subscribe to
    /// * `entity_ids_len` - Length of entity_ids array
    ///
    /// # Returns
    /// Result containing success boolean or error
    #[no_mangle]
    pub unsafe extern "C" fn client_update_aggregation_subscription(
        client: *mut ToriiClient,
        subscription: *mut Subscription,
        aggregator_ids: *const *const c_char,
        aggregator_ids_len: usize,
        entity_ids: *const *const c_char,
        entity_ids_len: usize,
    ) -> Result<bool> {
        // Convert aggregator_ids array to Vec<String> if not empty
        let aggregator_ids = if aggregator_ids.is_null() || aggregator_ids_len == 0 {
            Vec::new()
        } else {
            let ids = unsafe { std::slice::from_raw_parts(aggregator_ids, aggregator_ids_len) };
            ids.iter()
                .map(|id| unsafe { CStr::from_ptr(*id).to_string_lossy().to_string() })
                .collect::<Vec<String>>()
        };

        let entity_ids = if entity_ids.is_null() || entity_ids_len == 0 {
            Vec::new()
        } else {
            let ids = unsafe { std::slice::from_raw_parts(entity_ids, entity_ids_len) };
            ids.iter()
                .map(|id| unsafe { CStr::from_ptr(*id).to_string_lossy().to_string() })
                .collect::<Vec<String>>()
        };

        match RUNTIME.block_on((*client).inner.update_aggregation_subscription(
            (*subscription).id,
            aggregator_ids,
            entity_ids,
        )) {
            Ok(_) => Result::Ok(true),
            Err(e) => Result::Err(e.into()),
        }
    }

    /// Retrieves achievements matching query parameter
    ///
    /// # Parameters
    /// * `client` - Pointer to ToriiClient instance
    /// * `query` - AchievementQuery containing world_addresses, namespaces, hidden filter, and
    ///   pagination
    ///
    /// # Returns
    /// Result containing Page of Achievement or error
    #[no_mangle]
    pub unsafe extern "C" fn client_achievements(
        client: *mut ToriiClient,
        query: AchievementQuery,
    ) -> Result<Page<Achievement>> {
        let query = query.into();
        let achievements_future = unsafe { (*client).inner.achievements(query) };

        match RUNTIME.block_on(achievements_future) {
            Ok(achievements) => Result::Ok(achievements.into()),
            Err(e) => Result::Err(e.into()),
        }
    }

    /// Retrieves player achievement data matching query parameter
    ///
    /// # Parameters
    /// * `client` - Pointer to ToriiClient instance
    /// * `query` - PlayerAchievementQuery containing world_addresses, namespaces, player_addresses,
    ///   and pagination
    ///
    /// # Returns
    /// Result containing Page of PlayerAchievementEntry or error
    #[no_mangle]
    pub unsafe extern "C" fn client_player_achievements(
        client: *mut ToriiClient,
        query: PlayerAchievementQuery,
    ) -> Result<Page<PlayerAchievementEntry>> {
        let query = query.into();
        let player_achievements_future = unsafe { (*client).inner.player_achievements(query) };

        match RUNTIME.block_on(player_achievements_future) {
            Ok(player_achievements) => Result::Ok(player_achievements.into()),
            Err(e) => Result::Err(e.into()),
        }
    }

    /// Subscribes to achievement progression updates
    ///
    /// # Parameters
    /// * `client` - Pointer to ToriiClient instance
    /// * `world_addresses` - Array of world addresses to subscribe to
    /// * `world_addresses_len` - Length of world_addresses array
    /// * `namespaces` - Array of namespaces to subscribe to
    /// * `namespaces_len` - Length of namespaces array
    /// * `player_addresses` - Array of player addresses to subscribe to
    /// * `player_addresses_len` - Length of player_addresses array
    /// * `achievement_ids` - Array of achievement IDs to subscribe to
    /// * `achievement_ids_len` - Length of achievement_ids array
    /// * `callback` - Function called when updates occur
    ///
    /// # Returns
    /// Result containing pointer to Subscription or error
    #[no_mangle]
    pub unsafe extern "C" fn client_on_achievement_progression_update(
        client: *mut ToriiClient,
        world_addresses: *const types::FieldElement,
        world_addresses_len: usize,
        namespaces: *const *const c_char,
        namespaces_len: usize,
        player_addresses: *const types::FieldElement,
        player_addresses_len: usize,
        achievement_ids: *const *const c_char,
        achievement_ids_len: usize,
        callback: unsafe extern "C" fn(AchievementProgression),
    ) -> Result<*mut Subscription> {
        let client = Arc::new(unsafe { &*client });

        // Convert world_addresses array to Vec<Felt> if not empty
        let world_addresses = if world_addresses.is_null() || world_addresses_len == 0 {
            Vec::new()
        } else {
            let addrs = unsafe { std::slice::from_raw_parts(world_addresses, world_addresses_len) };
            addrs.iter().map(|addr| addr.clone().into()).collect::<Vec<Felt>>()
        };

        // Convert namespaces array to Vec<String> if not empty
        let namespaces = if namespaces.is_null() || namespaces_len == 0 {
            Vec::new()
        } else {
            let ns = unsafe { std::slice::from_raw_parts(namespaces, namespaces_len) };
            ns.iter()
                .map(|n| unsafe { CStr::from_ptr(*n).to_string_lossy().to_string() })
                .collect::<Vec<String>>()
        };

        // Convert player_addresses array to Vec<Felt> if not empty
        let player_addresses = if player_addresses.is_null() || player_addresses_len == 0 {
            Vec::new()
        } else {
            let addrs =
                unsafe { std::slice::from_raw_parts(player_addresses, player_addresses_len) };
            addrs.iter().map(|addr| addr.clone().into()).collect::<Vec<Felt>>()
        };

        // Convert achievement_ids array to Vec<String> if not empty
        let achievement_ids = if achievement_ids.is_null() || achievement_ids_len == 0 {
            Vec::new()
        } else {
            let ids = unsafe { std::slice::from_raw_parts(achievement_ids, achievement_ids_len) };
            ids.iter()
                .map(|id| unsafe { CStr::from_ptr(*id).to_string_lossy().to_string() })
                .collect::<Vec<String>>()
        };

        let (sub_id_tx, sub_id_rx) = oneshot::channel();
        let mut sub_id_tx = Some(sub_id_tx);
        let (trigger, tripwire) = Tripwire::new();

        // Spawn a new thread to handle the stream and reconnections
        let client_clone = client.clone();
        RUNTIME.spawn(async move {
            let mut backoff = Duration::from_secs(1);
            let max_backoff = Duration::from_secs(60);

            loop {
                let rcv = client_clone
                    .inner
                    .on_achievement_progression_updated(
                        world_addresses.clone(),
                        namespaces.clone(),
                        player_addresses.clone(),
                        achievement_ids.clone(),
                    )
                    .await;

                if let Ok(rcv) = rcv {
                    backoff = Duration::from_secs(1); // Reset backoff on successful connection

                    let mut rcv = rcv.take_until_if(tripwire.clone());

                    while let Some(Ok((id, progression))) = rcv.next().await {
                        // Our first message will be the subscription ID
                        if let Some(tx) = sub_id_tx.take() {
                            tx.send(id).expect("Failed to send subscription ID");
                        } else {
                            let progression: AchievementProgression = progression.into();
                            callback(progression);
                        }
                    }
                }

                // If we've reached this point, the stream has ended (possibly due to disconnection)
                // We'll try to reconnect after a delay, unless the tripwire has been triggered
                if tripwire.clone().now_or_never().unwrap_or_default() {
                    break; // Exit the loop if the subscription has been cancelled
                }
                sleep(backoff).await;
                backoff = std::cmp::min(backoff * 2, max_backoff);
            }
        });

        let subscription_id = match RUNTIME.block_on(sub_id_rx) {
            Ok(id) => id,
            Err(_) => {
                return Result::Err(Error {
                    message: CString::new(
                        "Failed to establish achievement progression subscription",
                    )
                    .unwrap()
                    .into_raw(),
                });
            }
        };

        let subscription = Subscription { id: subscription_id, trigger };

        Result::Ok(Box::into_raw(Box::new(subscription)))
    }

    /// Updates an existing achievement progression subscription with new parameters
    ///
    /// # Parameters
    /// * `client` - Pointer to ToriiClient instance
    /// * `subscription` - Pointer to existing Subscription
    /// * `world_addresses` - Array of world addresses to subscribe to
    /// * `world_addresses_len` - Length of world_addresses array
    /// * `namespaces` - Array of namespaces to subscribe to
    /// * `namespaces_len` - Length of namespaces array
    /// * `player_addresses` - Array of player addresses to subscribe to
    /// * `player_addresses_len` - Length of player_addresses array
    /// * `achievement_ids` - Array of achievement IDs to subscribe to
    /// * `achievement_ids_len` - Length of achievement_ids array
    ///
    /// # Returns
    /// Result containing success boolean or error
    #[no_mangle]
    pub unsafe extern "C" fn client_update_achievement_progression_subscription(
        client: *mut ToriiClient,
        subscription: *mut Subscription,
        world_addresses: *const types::FieldElement,
        world_addresses_len: usize,
        namespaces: *const *const c_char,
        namespaces_len: usize,
        player_addresses: *const types::FieldElement,
        player_addresses_len: usize,
        achievement_ids: *const *const c_char,
        achievement_ids_len: usize,
    ) -> Result<bool> {
        // Convert world_addresses array to Vec<Felt> if not empty
        let world_addresses = if world_addresses.is_null() || world_addresses_len == 0 {
            Vec::new()
        } else {
            let addrs = unsafe { std::slice::from_raw_parts(world_addresses, world_addresses_len) };
            addrs.iter().map(|addr| addr.clone().into()).collect::<Vec<Felt>>()
        };

        // Convert namespaces array to Vec<String> if not empty
        let namespaces = if namespaces.is_null() || namespaces_len == 0 {
            Vec::new()
        } else {
            let ns = unsafe { std::slice::from_raw_parts(namespaces, namespaces_len) };
            ns.iter()
                .map(|n| unsafe { CStr::from_ptr(*n).to_string_lossy().to_string() })
                .collect::<Vec<String>>()
        };

        // Convert player_addresses array to Vec<Felt> if not empty
        let player_addresses = if player_addresses.is_null() || player_addresses_len == 0 {
            Vec::new()
        } else {
            let addrs =
                unsafe { std::slice::from_raw_parts(player_addresses, player_addresses_len) };
            addrs.iter().map(|addr| addr.clone().into()).collect::<Vec<Felt>>()
        };

        // Convert achievement_ids array to Vec<String> if not empty
        let achievement_ids = if achievement_ids.is_null() || achievement_ids_len == 0 {
            Vec::new()
        } else {
            let ids = unsafe { std::slice::from_raw_parts(achievement_ids, achievement_ids_len) };
            ids.iter()
                .map(|id| unsafe { CStr::from_ptr(*id).to_string_lossy().to_string() })
                .collect::<Vec<String>>()
        };

        match RUNTIME.block_on((*client).inner.update_achievement_progression_subscription(
            (*subscription).id,
            world_addresses,
            namespaces,
            player_addresses,
            achievement_ids,
        )) {
            Ok(_) => Result::Ok(true),
            Err(e) => Result::Err(e.into()),
        }
    }

    /// Retrieves activities (user session tracking) matching query parameter
    ///
    /// # Parameters
    /// * `client` - Pointer to ToriiClient instance
    /// * `query` - ActivityQuery containing world_addresses, namespaces, caller_addresses, and
    ///   pagination
    ///
    /// # Returns
    /// Result containing Page of Activity or error
    #[no_mangle]
    pub unsafe extern "C" fn client_activities(
        client: *mut ToriiClient,
        query: ActivityQuery,
    ) -> Result<Page<Activity>> {
        let query = query.into();
        let activities_future = unsafe { (*client).inner.activities(query) };

        match RUNTIME.block_on(activities_future) {
            Ok(activities) => Result::Ok(activities.into()),
            Err(e) => Result::Err(e.into()),
        }
    }

    /// Subscribes to activity updates (user session tracking)
    ///
    /// # Parameters
    /// * `client` - Pointer to ToriiClient instance
    /// * `world_addresses` - Array of world addresses to subscribe to
    /// * `world_addresses_len` - Length of world_addresses array
    /// * `namespaces` - Array of namespaces to subscribe to
    /// * `namespaces_len` - Length of namespaces array
    /// * `caller_addresses` - Array of caller addresses to subscribe to
    /// * `caller_addresses_len` - Length of caller_addresses array
    /// * `callback` - Function called when updates occur
    ///
    /// # Returns
    /// Result containing pointer to Subscription or error
    #[no_mangle]
    pub unsafe extern "C" fn client_on_activity_update(
        client: *mut ToriiClient,
        world_addresses: *const types::FieldElement,
        world_addresses_len: usize,
        namespaces: *const *const c_char,
        namespaces_len: usize,
        caller_addresses: *const types::FieldElement,
        caller_addresses_len: usize,
        callback: unsafe extern "C" fn(Activity),
    ) -> Result<*mut Subscription> {
        let client = Arc::new(unsafe { &*client });

        // Convert world addresses array to Vec<Felt> if not empty
        let world_addresses = if world_addresses.is_null() || world_addresses_len == 0 {
            Vec::new()
        } else {
            let addresses =
                unsafe { std::slice::from_raw_parts(world_addresses, world_addresses_len) };
            addresses.iter().map(|f| f.clone().into()).collect::<Vec<Felt>>()
        };

        // Convert namespaces array to Vec<String> if not empty
        let namespaces = if namespaces.is_null() || namespaces_len == 0 {
            Vec::new()
        } else {
            let ns = unsafe { std::slice::from_raw_parts(namespaces, namespaces_len) };
            ns.iter()
                .map(|ns| unsafe { CStr::from_ptr(*ns).to_string_lossy().to_string() })
                .collect::<Vec<String>>()
        };

        // Convert caller addresses array to Vec<Felt> if not empty
        let caller_addresses = if caller_addresses.is_null() || caller_addresses_len == 0 {
            Vec::new()
        } else {
            let addresses =
                unsafe { std::slice::from_raw_parts(caller_addresses, caller_addresses_len) };
            addresses.iter().map(|f| f.clone().into()).collect::<Vec<Felt>>()
        };

        let (sub_id_tx, sub_id_rx) = oneshot::channel();
        let mut sub_id_tx = Some(sub_id_tx);
        let (trigger, tripwire) = Tripwire::new();

        // Spawn a new thread to handle the stream and reconnections
        let client_clone = client.clone();
        RUNTIME.spawn(async move {
            let mut backoff = Duration::from_secs(1);
            let max_backoff = Duration::from_secs(60);

            loop {
                let rcv = client_clone
                    .inner
                    .on_activity_updated(
                        world_addresses.clone(),
                        namespaces.clone(),
                        caller_addresses.clone(),
                    )
                    .await;

                if let Ok(rcv) = rcv {
                    backoff = Duration::from_secs(1); // Reset backoff on successful connection

                    let mut rcv = rcv.take_until_if(tripwire.clone());

                    while let Some(Ok((id, activity))) = rcv.next().await {
                        // Our first message will be the subscription ID
                        if let Some(tx) = sub_id_tx.take() {
                            tx.send(id).expect("Failed to send subscription ID");
                        } else {
                            let activity: Activity = activity.into();
                            callback(activity);
                        }
                    }
                }

                // If we've reached this point, the stream has ended (possibly due to disconnection)
                // We'll try to reconnect after a delay, unless the tripwire has been triggered
                if tripwire.clone().now_or_never().unwrap_or_default() {
                    break; // Exit the loop if the subscription has been cancelled
                }
                sleep(backoff).await;
                backoff = std::cmp::min(backoff * 2, max_backoff);
            }
        });

        let subscription_id = match RUNTIME.block_on(sub_id_rx) {
            Ok(id) => id,
            Err(_) => {
                return Result::Err(Error {
                    message: CString::new("Failed to establish activity subscription")
                        .unwrap()
                        .into_raw(),
                });
            }
        };

        let subscription = Subscription { id: subscription_id, trigger };

        Result::Ok(Box::into_raw(Box::new(subscription)))
    }

    /// Updates an existing activity subscription with new parameters
    ///
    /// # Parameters
    /// * `client` - Pointer to ToriiClient instance
    /// * `subscription` - Pointer to existing Subscription
    /// * `world_addresses` - Array of world addresses to subscribe to
    /// * `world_addresses_len` - Length of world_addresses array
    /// * `namespaces` - Array of namespaces to subscribe to
    /// * `namespaces_len` - Length of namespaces array
    /// * `caller_addresses` - Array of caller addresses to subscribe to
    /// * `caller_addresses_len` - Length of caller_addresses array
    ///
    /// # Returns
    /// Result containing success boolean or error
    #[no_mangle]
    pub unsafe extern "C" fn client_update_activity_subscription(
        client: *mut ToriiClient,
        subscription: *mut Subscription,
        world_addresses: *const types::FieldElement,
        world_addresses_len: usize,
        namespaces: *const *const c_char,
        namespaces_len: usize,
        caller_addresses: *const types::FieldElement,
        caller_addresses_len: usize,
    ) -> Result<bool> {
        // Convert world addresses array to Vec<Felt> if not empty
        let world_addresses = if world_addresses.is_null() || world_addresses_len == 0 {
            Vec::new()
        } else {
            let addresses =
                unsafe { std::slice::from_raw_parts(world_addresses, world_addresses_len) };
            addresses.iter().map(|f| f.clone().into()).collect::<Vec<Felt>>()
        };

        // Convert namespaces array to Vec<String> if not empty
        let namespaces = if namespaces.is_null() || namespaces_len == 0 {
            Vec::new()
        } else {
            let ns = unsafe { std::slice::from_raw_parts(namespaces, namespaces_len) };
            ns.iter()
                .map(|ns| unsafe { CStr::from_ptr(*ns).to_string_lossy().to_string() })
                .collect::<Vec<String>>()
        };

        // Convert caller addresses array to Vec<Felt> if not empty
        let caller_addresses = if caller_addresses.is_null() || caller_addresses_len == 0 {
            Vec::new()
        } else {
            let addresses =
                unsafe { std::slice::from_raw_parts(caller_addresses, caller_addresses_len) };
            addresses.iter().map(|f| f.clone().into()).collect::<Vec<Felt>>()
        };

        match RUNTIME.block_on((*client).inner.update_activity_subscription(
            (*subscription).id,
            world_addresses,
            namespaces,
            caller_addresses,
        )) {
            Ok(_) => Result::Ok(true),
            Err(e) => Result::Err(e.into()),
        }
    }

    /// Subscribes to event message updates
    ///
    /// # Parameters
    /// * `client` - Pointer to ToriiClient instance
    /// * `clauses` - Array of entity key clauses to filter updates
    /// * `clauses_len` - Length of clauses array
    /// * `callback` - Function called when updates occur
    ///
    /// # Returns
    /// Result containing pointer to Subscription or error
    #[no_mangle]
    pub unsafe extern "C" fn client_on_event_message_update(
        client: *mut ToriiClient,
        clause: COption<Clause>,
        world_addresses: *const types::FieldElement,
        world_addresses_len: usize,
        callback: unsafe extern "C" fn(Entity),
    ) -> Result<*mut Subscription> {
        let client = Arc::new(unsafe { &*client });
        let clause: Option<torii_proto::Clause> = clause.map(|c| c.into()).into();
        let world_addresses =
            unsafe { std::slice::from_raw_parts(world_addresses, world_addresses_len) };
        let world_addresses =
            world_addresses.iter().map(|addr| addr.clone().into()).collect::<Vec<Felt>>();
        let (trigger, tripwire) = Tripwire::new();
        let (sub_id_tx, sub_id_rx) = oneshot::channel();
        let mut sub_id_tx = Some(sub_id_tx);

        // Spawn a new thread to handle the stream and reconnections
        let client_clone = client.clone();
        RUNTIME.spawn(async move {
            let mut backoff = Duration::from_secs(1);
            let max_backoff = Duration::from_secs(60);

            loop {
                let rcv = client_clone
                    .inner
                    .on_event_message_updated(clause.clone(), world_addresses.clone())
                    .await;
                if let Ok(rcv) = rcv {
                    backoff = Duration::from_secs(1); // Reset backoff on successful connection

                    let mut rcv = rcv.take_until_if(tripwire.clone());

                    while let Some(Ok((id, entity))) = rcv.next().await {
                        if let Some(tx) = sub_id_tx.take() {
                            tx.send(id).expect("Failed to send subscription ID");
                        } else {
                            callback(entity.into());
                        }
                    }
                }

                // If we've reached this point, the stream has ended (possibly due to disconnection)
                // We'll try to reconnect after a delay, unless the tripwire has been triggered
                if tripwire.clone().now_or_never().unwrap_or_default() {
                    break; // Exit the loop if the subscription has been cancelled
                }
                sleep(backoff).await;
                backoff = std::cmp::min(backoff * 2, max_backoff);
            }
        });

        let subscription_id = match RUNTIME.block_on(sub_id_rx) {
            Ok(id) => id,
            Err(_) => {
                return Result::Err(Error {
                    message: CString::new("Failed to establish event message subscription")
                        .unwrap()
                        .into_raw(),
                });
            }
        };

        let subscription = Subscription { id: subscription_id, trigger };
        Result::Ok(Box::into_raw(Box::new(subscription)))
    }

    /// Updates an existing event message subscription
    ///
    /// # Parameters
    /// * `client` - Pointer to ToriiClient instance
    /// * `subscription` - Pointer to existing Subscription
    /// * `clauses` - New array of entity key clauses
    /// * `clauses_len` - Length of new clauses array
    ///
    /// # Returns
    /// Result containing success boolean or error
    #[no_mangle]
    pub unsafe extern "C" fn client_update_event_message_subscription(
        client: *mut ToriiClient,
        subscription: *mut Subscription,
        clause: COption<Clause>,
        world_addresses: *const types::FieldElement,
        world_addresses_len: usize,
    ) -> Result<bool> {
        let clause: Option<torii_proto::Clause> = clause.map(|c| c.into()).into();
        let world_addresses =
            unsafe { std::slice::from_raw_parts(world_addresses, world_addresses_len) };
        let world_addresses = world_addresses.iter().map(|addr| addr.clone().into()).collect();
        match RUNTIME.block_on((*client).inner.update_event_message_subscription(
            (*subscription).id,
            clause,
            world_addresses,
        )) {
            Ok(_) => Result::Ok(true),
            Err(e) => Result::Err(e.into()),
        }
    }

    /// Subscribes to Starknet events
    ///
    /// # Parameters
    /// * `client` - Pointer to ToriiClient instance
    /// * `clauses` - Array of entity key clauses to filter events
    /// * `clauses_len` - Length of clauses array
    /// * `callback` - Function called when events occur
    ///
    /// # Returns
    /// Result containing pointer to Subscription or error
    #[no_mangle]
    pub unsafe extern "C" fn client_on_starknet_event(
        client: *mut ToriiClient,
        clauses: *const KeysClause,
        clauses_len: usize,
        callback: unsafe extern "C" fn(Event),
    ) -> Result<*mut Subscription> {
        let client = Arc::new(unsafe { &*client });
        let clauses = if clauses.is_null() || clauses_len == 0 {
            Vec::new()
        } else {
            let clauses = unsafe { std::slice::from_raw_parts(clauses, clauses_len) };
            clauses.iter().map(|c| c.clone().into()).collect::<Vec<_>>()
        };

        let (sub_id_tx, sub_id_rx) = oneshot::channel();
        let mut sub_id_tx = Some(sub_id_tx);
        let (trigger, tripwire) = Tripwire::new();

        // Spawn a new thread to handle the stream and reconnections
        let client_clone = client.clone();
        RUNTIME.spawn(async move {
            let mut backoff = Duration::from_secs(1);
            let max_backoff = Duration::from_secs(60);

            loop {
                let rcv = client_clone.inner.on_starknet_event(clauses.clone()).await;

                if let Ok(rcv) = rcv {
                    backoff = Duration::from_secs(1); // Reset backoff on successful connection

                    let mut rcv = rcv.take_until_if(tripwire.clone());

                    while let Some(Ok(event)) = rcv.next().await {
                        if let Some(tx) = sub_id_tx.take() {
                            tx.send(0).expect("Failed to send subscription ID");
                        } else {
                            let event: Event = event.into();
                            callback(event);
                        }
                    }
                }

                // If we've reached this point, the stream has ended (possibly due to disconnection)
                // We'll try to reconnect after a delay, unless the tripwire has been triggered
                if tripwire.clone().now_or_never().unwrap_or_default() {
                    break; // Exit the loop if the subscription has been cancelled
                }
                sleep(backoff).await;
                backoff = std::cmp::min(backoff * 2, max_backoff);
            }
        });

        let subscription_id = match RUNTIME.block_on(sub_id_rx) {
            Ok(id) => id,
            Err(_) => {
                return Result::Err(Error {
                    message: CString::new("Failed to establish event subscription")
                        .unwrap()
                        .into_raw(),
                });
            }
        };

        let subscription = Subscription { id: subscription_id, trigger };

        Result::Ok(Box::into_raw(Box::new(subscription)))
    }

    /// Retrieves token information for given contract addresses
    ///
    /// # Parameters
    /// * `client` - Pointer to ToriiClient instance
    /// * `contract_addresses` - Array of contract addresses
    /// * `contract_addresses_len` - Length of addresses array
    /// * `token_ids` - Array of token ids
    /// * `token_ids_len` - Length of token ids array
    /// * `limit` - Maximum number of tokens to return
    /// * `cursor` - Cursor to start from
    ///
    /// # Returns
    /// Result containing array of Token information or error
    #[no_mangle]
    pub unsafe extern "C" fn client_tokens(
        client: *mut ToriiClient,
        query: TokenQuery,
    ) -> Result<Page<Token>> {
        let query = query.into();
        let tokens_future = unsafe { (*client).inner.tokens(query) };

        match RUNTIME.block_on(tokens_future) {
            Ok(tokens) => Result::Ok(tokens.into()),
            Err(e) => Result::Err(e.into()),
        }
    }

    /// Subscribes to token updates
    ///
    /// # Parameters
    /// * `client` - Pointer to ToriiClient instance
    /// * `contract_addresses` - Array of contract addresses
    /// * `callback` - Function called when updates occur
    ///
    /// # Returns
    /// Result containing pointer to Subscription or error
    #[no_mangle]
    pub unsafe extern "C" fn client_on_token_update(
        client: *mut ToriiClient,
        contract_addresses: *const types::FieldElement,
        contract_addresses_len: usize,
        token_ids: *const types::U256,
        token_ids_len: usize,
        callback: unsafe extern "C" fn(Token),
    ) -> Result<*mut Subscription> {
        let client = Arc::new(unsafe { &*client });

        // Convert contract addresses array to Vec<Felt> if not empty
        let contract_addresses = if contract_addresses.is_null() || contract_addresses_len == 0 {
            Vec::new()
        } else {
            let addresses =
                unsafe { std::slice::from_raw_parts(contract_addresses, contract_addresses_len) };
            addresses.iter().map(|f| f.clone().into()).collect::<Vec<Felt>>()
        };

        let token_ids = if token_ids.is_null() || token_ids_len == 0 {
            Vec::new()
        } else {
            let ids = unsafe { std::slice::from_raw_parts(token_ids, token_ids_len) };
            ids.iter().map(|f| f.clone().into()).collect::<Vec<U256>>()
        };

        let (sub_id_tx, sub_id_rx) = oneshot::channel();
        let mut sub_id_tx = Some(sub_id_tx);
        let (trigger, tripwire) = Tripwire::new();

        // Spawn a new thread to handle the stream and reconnections
        let client_clone = client.clone();
        RUNTIME.spawn(async move {
            let mut backoff = Duration::from_secs(1);
            let max_backoff = Duration::from_secs(60);

            loop {
                let rcv = client_clone
                    .inner
                    .on_token_updated(contract_addresses.clone(), token_ids.clone())
                    .await;

                if let Ok(rcv) = rcv {
                    backoff = Duration::from_secs(1); // Reset backoff on successful connection

                    let mut rcv = rcv.take_until_if(tripwire.clone());

                    while let Some(Ok((id, token))) = rcv.next().await {
                        // Our first message will be the subscription ID
                        if let Some(tx) = sub_id_tx.take() {
                            tx.send(id).expect("Failed to send subscription ID");
                        } else {
                            let token: Token = token.into();
                            callback(token);
                        }
                    }
                }

                // If we've reached this point, the stream has ended (possibly due to disconnection)
                // We'll try to reconnect after a delay, unless the tripwire has been triggered
                if tripwire.clone().now_or_never().unwrap_or_default() {
                    break; // Exit the loop if the subscription has been cancelled
                }
                sleep(backoff).await;
                backoff = std::cmp::min(backoff * 2, max_backoff);
            }
        });

        let subscription_id = match RUNTIME.block_on(sub_id_rx) {
            Ok(id) => id,
            Err(_) => {
                return Result::Err(Error {
                    message: CString::new("Failed to establish token subscription")
                        .unwrap()
                        .into_raw(),
                });
            }
        };

        let subscription = Subscription { id: subscription_id, trigger };

        Result::Ok(Box::into_raw(Box::new(subscription)))
    }

    /// Gets token balances for given accounts and contracts
    ///
    /// # Parameters
    /// * `client` - Pointer to ToriiClient instance
    /// * `contract_addresses` - Array of contract addresses
    /// * `contract_addresses_len` - Length of contract addresses array
    /// * `account_addresses` - Array of account addresses
    /// * `account_addresses_len` - Length of account addresses array
    /// * `token_ids` - Array of token ids
    /// * `token_ids_len` - Length of token ids array
    /// * `limit` - Maximum number of token balances to return
    /// * `cursor` - Cursor to start from
    ///
    /// # Returns
    /// Result containing array of TokenBalance information or error
    #[no_mangle]
    pub unsafe extern "C" fn client_token_balances(
        client: *mut ToriiClient,
        query: TokenBalanceQuery,
    ) -> Result<Page<TokenBalance>> {
        let query = query.into();
        let token_balances_future = unsafe { (*client).inner.token_balances(query) };

        match RUNTIME.block_on(token_balances_future) {
            Ok(token_balances) => Result::Ok(token_balances.into()),
            Err(e) => Result::Err(e.into()),
        }
    }

    /// Gets token collections for given accounts and contracts
    ///
    /// # Parameters
    /// * `client` - Pointer to ToriiClient instance
    /// * `contract_addresses` - Array of contract addresses
    /// * `contract_addresses_len` - Length of contract addresses array
    /// * `account_addresses` - Array of account addresses
    /// * `account_addresses_len` - Length of account addresses array
    /// * `token_ids` - Array of token ids
    /// * `token_ids_len` - Length of token ids array
    /// * `limit` - Maximum number of token balances to return
    /// * `cursor` - Cursor to start from
    ///
    /// # Returns
    /// Result containing array of TokenBalance information or error
    #[no_mangle]
    pub unsafe extern "C" fn client_token_contracts(
        client: *mut ToriiClient,
        query: TokenContractQuery,
    ) -> Result<Page<TokenContract>> {
        let query = query.into();
        let token_contracts_future = unsafe { (*client).inner.token_contracts(query) };

        match RUNTIME.block_on(token_contracts_future) {
            Ok(contracts) => Result::Ok(contracts.into()),
            Err(e) => Result::Err(e.into()),
        }
    }

    /// Gets contracts matching the given query
    ///
    /// # Parameters
    /// * `client` - Pointer to ToriiClient instance
    /// * `query` - ContractQuery parameters
    ///
    /// # Returns
    /// Result containing array of Contract information or error
    #[no_mangle]
    pub unsafe extern "C" fn client_contracts(
        client: *mut ToriiClient,
        query: ContractQuery,
    ) -> Result<CArray<Contract>> {
        let query: torii_proto::ContractQuery = query.into();
        let contracts_future = unsafe { (*client).inner.contracts(query) };

        match RUNTIME.block_on(contracts_future) {
            Ok(contracts) => Result::Ok(contracts.into()),
            Err(e) => Result::Err(e.into()),
        }
    }

    /// Retrieves token transfers matching the given query
    ///
    /// # Parameters
    /// * `client` - Pointer to ToriiClient instance
    /// * `query` - TokenTransferQuery parameters
    ///
    /// # Returns
    /// Result containing array of TokenTransfer information or error
    #[no_mangle]
    pub unsafe extern "C" fn client_token_transfers(
        client: *mut ToriiClient,
        query: TokenTransferQuery,
    ) -> Result<Page<TokenTransfer>> {
        let query = query.into();
        let token_transfers_future = unsafe { (*client).inner.token_transfers(query) };

        match RUNTIME.block_on(token_transfers_future) {
            Ok(transfers) => Result::Ok(transfers.into()),
            Err(e) => Result::Err(e.into()),
        }
    }

    /// Subscribes to contract updates
    ///
    /// # Parameters
    /// * `client` - Pointer to ToriiClient instance
    /// * `contract_address` - Optional contract address to filter updates
    /// * `callback` - Function called when updates occur
    ///
    /// # Returns
    /// Result containing pointer to Subscription or error
    #[no_mangle]
    pub unsafe extern "C" fn on_contract_update(
        client: *mut ToriiClient,
        contract_address: *const types::FieldElement,
        callback: unsafe extern "C" fn(Contract),
    ) -> Result<*mut Subscription> {
        let client = Arc::new(unsafe { &*client });
        let contract_address = if contract_address.is_null() {
            None
        } else {
            Some(unsafe { (*contract_address).clone().into() })
        };

        let (sub_id_tx, sub_id_rx) = oneshot::channel();
        let mut sub_id_tx = Some(sub_id_tx);
        let (trigger, tripwire) = Tripwire::new();

        // Spawn a new thread to handle the stream and reconnections
        let client_clone = client.clone();
        RUNTIME.spawn(async move {
            let mut backoff = Duration::from_secs(1);
            let max_backoff = Duration::from_secs(60);

            loop {
                let rcv = client_clone.inner.on_contract_updated(contract_address).await;
                if let Ok(rcv) = rcv {
                    backoff = Duration::from_secs(1); // Reset backoff on successful connection

                    let mut rcv = rcv.take_until_if(tripwire.clone());

                    while let Some(Ok(update)) = rcv.next().await {
                        if let Some(tx) = sub_id_tx.take() {
                            tx.send(0).expect("Failed to send subscription ID");
                        } else {
                            callback(update.into());
                        }
                    }
                }

                // If we've reached this point, the stream has ended (possibly due to disconnection)
                // We'll try to reconnect after a delay, unless the tripwire has been triggered
                if tripwire.clone().now_or_never().unwrap_or_default() {
                    break; // Exit the loop if the subscription has been cancelled
                }
                sleep(backoff).await;
                backoff = std::cmp::min(backoff * 2, max_backoff);
            }
        });

        let subscription_id = match RUNTIME.block_on(sub_id_rx) {
            Ok(id) => id,
            Err(_) => {
                return Result::Err(Error {
                    message: CString::new("Failed to establish contract subscription")
                        .unwrap()
                        .into_raw(),
                });
            }
        };

        let subscription = Subscription { id: subscription_id, trigger };

        Result::Ok(Box::into_raw(Box::new(subscription)))
    }

    /// Subscribes to token balance updates
    ///
    /// # Parameters
    /// * `client` - Pointer to ToriiClient instance
    /// * `contract_addresses` - Array of contract addresses to filter (empty for all)
    /// * `contract_addresses_len` - Length of contract addresses array
    /// * `account_addresses` - Array of account addresses to filter (empty for all)
    /// * `account_addresses_len` - Length of account addresses array
    /// * `callback` - Function called when updates occur
    ///
    /// # Returns
    /// Result containing pointer to Subscription or error
    #[no_mangle]
    pub unsafe extern "C" fn client_on_token_balance_update(
        client: *mut ToriiClient,
        contract_addresses: *const types::FieldElement,
        contract_addresses_len: usize,
        account_addresses: *const types::FieldElement,
        account_addresses_len: usize,
        token_ids: *const types::U256,
        token_ids_len: usize,
        callback: unsafe extern "C" fn(TokenBalance),
    ) -> Result<*mut Subscription> {
        let client = Arc::new(unsafe { &*client });

        // Convert account addresses array to Vec<Felt> if not empty
        let account_addresses = if account_addresses.is_null() || account_addresses_len == 0 {
            Vec::new()
        } else {
            let addresses =
                unsafe { std::slice::from_raw_parts(account_addresses, account_addresses_len) };
            addresses.iter().map(|f| f.clone().into()).collect::<Vec<Felt>>()
        };

        // Convert contract addresses array to Vec<Felt> if not empty
        let contract_addresses = if contract_addresses.is_null() || contract_addresses_len == 0 {
            Vec::new()
        } else {
            let addresses =
                unsafe { std::slice::from_raw_parts(contract_addresses, contract_addresses_len) };
            addresses.iter().map(|f| f.clone().into()).collect::<Vec<Felt>>()
        };

        let token_ids = if token_ids.is_null() || token_ids_len == 0 {
            Vec::new()
        } else {
            let ids = unsafe { std::slice::from_raw_parts(token_ids, token_ids_len) };
            ids.iter().map(|f| f.clone().into()).collect::<Vec<U256>>()
        };

        let (sub_id_tx, sub_id_rx) = oneshot::channel();
        let mut sub_id_tx = Some(sub_id_tx);
        let (trigger, tripwire) = Tripwire::new();

        // Spawn a new thread to handle the stream and reconnections
        let client_clone = client.clone();
        RUNTIME.spawn(async move {
            let mut backoff = Duration::from_secs(1);
            let max_backoff = Duration::from_secs(60);

            loop {
                let rcv = client_clone
                    .inner
                    .on_token_balance_updated(
                        contract_addresses.clone(),
                        account_addresses.clone(),
                        token_ids.clone(),
                    )
                    .await;

                if let Ok(rcv) = rcv {
                    backoff = Duration::from_secs(1); // Reset backoff on successful connection

                    let mut rcv = rcv.take_until_if(tripwire.clone());

                    while let Some(Ok((id, balance))) = rcv.next().await {
                        if let Some(tx) = sub_id_tx.take() {
                            tx.send(id).expect("Failed to send subscription ID");
                        } else {
                            let balance: TokenBalance = balance.into();
                            callback(balance);
                        }
                    }
                }

                // If we've reached this point, the stream has ended (possibly due to disconnection)
                // We'll try to reconnect after a delay, unless the tripwire has been triggered
                if tripwire.clone().now_or_never().unwrap_or_default() {
                    break; // Exit the loop if the subscription has been cancelled
                }
                sleep(backoff).await;
                backoff = std::cmp::min(backoff * 2, max_backoff);
            }
        });

        let subscription_id = match RUNTIME.block_on(sub_id_rx) {
            Ok(id) => id,
            Err(_) => {
                return Result::Err(Error {
                    message: CString::new("Failed to establish token balance subscription")
                        .unwrap()
                        .into_raw(),
                });
            }
        };

        let subscription = Subscription { id: subscription_id, trigger };

        Result::Ok(Box::into_raw(Box::new(subscription)))
    }

    /// Updates an existing token balance subscription
    ///
    /// # Parameters
    /// * `client` - Pointer to ToriiClient instance
    /// * `subscription` - Pointer to existing Subscription
    /// * `contract_addresses` - Array of contract addresses to filter (empty for all)
    /// * `contract_addresses_len` - Length of contract addresses array
    /// * `account_addresses` - Array of account addresses to filter (empty for all)
    /// * `account_addresses_len` - Length of account addresses array
    ///
    /// # Returns
    /// Result containing success boolean or error
    #[no_mangle]
    pub unsafe extern "C" fn client_update_token_balance_subscription(
        client: *mut ToriiClient,
        subscription: *mut Subscription,
        contract_addresses: *const types::FieldElement,
        contract_addresses_len: usize,
        account_addresses: *const types::FieldElement,
        account_addresses_len: usize,
        token_ids: *const types::U256,
        token_ids_len: usize,
    ) -> Result<bool> {
        let contract_addresses = if contract_addresses.is_null() || contract_addresses_len == 0 {
            Vec::new()
        } else {
            let addresses =
                unsafe { std::slice::from_raw_parts(contract_addresses, contract_addresses_len) };
            addresses.iter().map(|f| f.clone().into()).collect::<Vec<Felt>>()
        };

        let account_addresses = if account_addresses.is_null() || account_addresses_len == 0 {
            Vec::new()
        } else {
            let addresses =
                unsafe { std::slice::from_raw_parts(account_addresses, account_addresses_len) };
            addresses.iter().map(|f| f.clone().into()).collect::<Vec<Felt>>()
        };

        let token_ids = if token_ids.is_null() || token_ids_len == 0 {
            Vec::new()
        } else {
            let ids = unsafe { std::slice::from_raw_parts(token_ids, token_ids_len) };
            ids.iter().map(|f| f.clone().into()).collect::<Vec<U256>>()
        };

        match RUNTIME.block_on((*client).inner.update_token_balance_subscription(
            (*subscription).id,
            contract_addresses,
            account_addresses,
            token_ids,
        )) {
            Ok(_) => Result::Ok(true),
            Err(e) => Result::Err(e.into()),
        }
    }

    /// Subscribes to token transfer updates
    ///
    /// # Parameters
    /// * `client` - Pointer to ToriiClient instance
    /// * `contract_addresses` - Array of contract addresses to filter (empty for all)
    /// * `contract_addresses_len` - Length of contract addresses array
    /// * `account_addresses` - Array of account addresses to filter (empty for all)
    /// * `account_addresses_len` - Length of account addresses array
    /// * `token_ids` - Array of token IDs to filter (empty for all)
    /// * `token_ids_len` - Length of token IDs array
    /// * `callback` - Function called when updates occur
    ///
    /// # Returns
    /// Result containing pointer to Subscription or error
    #[no_mangle]
    pub unsafe extern "C" fn client_on_token_transfer_update(
        client: *mut ToriiClient,
        contract_addresses: *const types::FieldElement,
        contract_addresses_len: usize,
        account_addresses: *const types::FieldElement,
        account_addresses_len: usize,
        token_ids: *const types::U256,
        token_ids_len: usize,
        callback: unsafe extern "C" fn(TokenTransfer),
    ) -> Result<*mut Subscription> {
        let client = Arc::new(unsafe { &*client });

        // Convert account addresses array to Vec<Felt> if not empty
        let account_addresses = if account_addresses.is_null() || account_addresses_len == 0 {
            Vec::new()
        } else {
            let addresses =
                unsafe { std::slice::from_raw_parts(account_addresses, account_addresses_len) };
            addresses.iter().map(|f| f.clone().into()).collect::<Vec<Felt>>()
        };

        // Convert contract addresses array to Vec<Felt> if not empty
        let contract_addresses = if contract_addresses.is_null() || contract_addresses_len == 0 {
            Vec::new()
        } else {
            let addresses =
                unsafe { std::slice::from_raw_parts(contract_addresses, contract_addresses_len) };
            addresses.iter().map(|f| f.clone().into()).collect::<Vec<Felt>>()
        };

        let token_ids = if token_ids.is_null() || token_ids_len == 0 {
            Vec::new()
        } else {
            let ids = unsafe { std::slice::from_raw_parts(token_ids, token_ids_len) };
            ids.iter().map(|f| f.clone().into()).collect::<Vec<U256>>()
        };

        let (sub_id_tx, sub_id_rx) = oneshot::channel();
        let mut sub_id_tx = Some(sub_id_tx);
        let (trigger, tripwire) = Tripwire::new();

        // Spawn a new thread to handle the stream and reconnections
        let client_clone = client.clone();
        RUNTIME.spawn(async move {
            let mut backoff = Duration::from_secs(1);
            let max_backoff = Duration::from_secs(60);

            loop {
                let rcv = client_clone
                    .inner
                    .on_token_transfer_updated(
                        contract_addresses.clone(),
                        account_addresses.clone(),
                        token_ids.clone(),
                    )
                    .await;

                if let Ok(rcv) = rcv {
                    backoff = Duration::from_secs(1); // Reset backoff on successful connection

                    let mut rcv = rcv.take_until_if(tripwire.clone());

                    while let Some(Ok(transfer)) = rcv.next().await {
                        if let Some(tx) = sub_id_tx.take() {
                            tx.send(0).expect("Failed to send subscription ID");
                        } else {
                            let transfer: types::TokenTransfer = transfer.into();
                            callback(transfer);
                        }
                    }
                }

                // If we've reached this point, the stream has ended (possibly due to disconnection)
                // We'll try to reconnect after a delay, unless the tripwire has been triggered
                if tripwire.clone().now_or_never().unwrap_or_default() {
                    break; // Exit the loop if the subscription has been cancelled
                }
                sleep(backoff).await;
                backoff = std::cmp::min(backoff * 2, max_backoff);
            }
        });

        let subscription_id = match RUNTIME.block_on(sub_id_rx) {
            Ok(id) => id,
            Err(_) => {
                return Result::Err(Error {
                    message: CString::new("Failed to establish token transfer subscription")
                        .unwrap()
                        .into_raw(),
                });
            }
        };

        let subscription = Subscription { id: subscription_id, trigger };

        Result::Ok(Box::into_raw(Box::new(subscription)))
    }

    /// Updates an existing token transfer subscription
    ///
    /// # Parameters
    /// * `client` - Pointer to ToriiClient instance
    /// * `subscription` - Pointer to existing Subscription
    /// * `contract_addresses` - Array of contract addresses to filter (empty for all)
    /// * `contract_addresses_len` - Length of contract addresses array
    /// * `account_addresses` - Array of account addresses to filter (empty for all)
    /// * `account_addresses_len` - Length of account addresses array
    /// * `token_ids` - Array of token IDs to filter (empty for all)
    /// * `token_ids_len` - Length of token IDs array
    ///
    /// # Returns
    /// Result containing success boolean or error
    #[no_mangle]
    pub unsafe extern "C" fn client_update_token_transfer_subscription(
        client: *mut ToriiClient,
        subscription: *mut Subscription,
        contract_addresses: *const types::FieldElement,
        contract_addresses_len: usize,
        account_addresses: *const types::FieldElement,
        account_addresses_len: usize,
        token_ids: *const types::U256,
        token_ids_len: usize,
    ) -> Result<bool> {
        let contract_addresses = if contract_addresses.is_null() || contract_addresses_len == 0 {
            Vec::new()
        } else {
            let addresses =
                unsafe { std::slice::from_raw_parts(contract_addresses, contract_addresses_len) };
            addresses.iter().map(|f| f.clone().into()).collect::<Vec<Felt>>()
        };

        let account_addresses = if account_addresses.is_null() || account_addresses_len == 0 {
            Vec::new()
        } else {
            let addresses =
                unsafe { std::slice::from_raw_parts(account_addresses, account_addresses_len) };
            addresses.iter().map(|f| f.clone().into()).collect::<Vec<Felt>>()
        };

        let token_ids = if token_ids.is_null() || token_ids_len == 0 {
            Vec::new()
        } else {
            let ids = unsafe { std::slice::from_raw_parts(token_ids, token_ids_len) };
            ids.iter().map(|f| f.clone().into()).collect::<Vec<U256>>()
        };

        match RUNTIME.block_on((*client).inner.update_token_transfer_subscription(
            (*subscription).id,
            contract_addresses,
            account_addresses,
            token_ids,
        )) {
            Ok(_) => Result::Ok(true),
            Err(e) => Result::Err(e.into()),
        }
    }

    /// Performs a full-text search across indexed entities using FTS5
    ///
    /// # Parameters
    /// * `client` - Pointer to ToriiClient instance
    /// * `query` - Search query containing the search text and limit
    ///
    /// # Returns
    /// Result containing SearchResponse with results grouped by table or error
    #[no_mangle]
    pub unsafe extern "C" fn client_search(
        client: *mut ToriiClient,
        query: types::SearchQuery,
    ) -> Result<types::SearchResponse> {
        let query = query.into();
        let search_future = unsafe { (*client).inner.search(query) };

        match RUNTIME.block_on(search_future) {
            Ok(response) => Result::Ok(response.into()),
            Err(e) => Result::Err(e.into()),
        }
    }

    /// Serializes a string into a byte array
    ///
    /// # Parameters
    /// * `str` - String to serialize
    ///
    /// # Returns
    /// Result containing array of FieldElements or error
    #[no_mangle]
    pub unsafe extern "C" fn bytearray_serialize(
        str: *const c_char,
    ) -> Result<CArray<types::FieldElement>> {
        let str = unsafe { CStr::from_ptr(str).to_string_lossy().into_owned() };
        let bytearray = match ByteArray::from_string(str.as_str()) {
            Ok(bytearray) => bytearray,
            Err(e) => return Result::Err(e.into()),
        };

        let felts = cairo_serde::ByteArray::cairo_serialize(&bytearray);
        let felts = felts.into_iter().map(|f| f.into()).collect::<Vec<types::FieldElement>>();
        Result::Ok(felts.into())
    }

    /// Deserializes field elements into a string
    ///
    /// # Parameters
    /// * `felts` - Array of field elements
    /// * `felts_len` - Length of field elements array
    ///
    /// # Returns
    /// Result containing pointer to C string or error
    #[no_mangle]
    pub unsafe extern "C" fn bytearray_deserialize(
        felts: *const types::FieldElement,
        felts_len: usize,
    ) -> Result<*const c_char> {
        let felts = unsafe { std::slice::from_raw_parts(felts, felts_len) };
        let felts = felts.iter().map(|f| f.clone().into()).collect::<Vec<Felt>>();
        let bytearray = match cairo_serde::ByteArray::cairo_deserialize(&felts, 0) {
            Ok(bytearray) => bytearray,
            Err(e) => return Result::Err(e.into()),
        };

        let bytearray = match bytearray.to_string() {
            Ok(bytearray) => bytearray,
            Err(e) => return Result::Err(e.into()),
        };

        Result::Ok(CString::new(bytearray).unwrap().into_raw())
    }

    /// Computes Poseidon hash of field elements
    ///
    /// # Parameters
    /// * `felts` - Array of field elements
    /// * `felts_len` - Length of array
    ///
    /// # Returns
    /// FieldElement containing the hash result
    #[no_mangle]
    pub unsafe extern "C" fn poseidon_hash(
        felts: *const types::FieldElement,
        felts_len: usize,
    ) -> types::FieldElement {
        let felts = unsafe { std::slice::from_raw_parts(felts, felts_len) };
        let felts = felts.iter().map(|f| f.clone().into()).collect::<Vec<Felt>>();

        poseidon_hash_many(&felts).into()
    }

    /// Gets selector from name string
    ///
    /// # Parameters
    /// * `name` - Name to compute selector from
    ///
    /// # Returns
    /// Result containing FieldElement selector or error
    #[no_mangle]
    pub unsafe extern "C" fn get_selector_from_name(
        name: *const c_char,
    ) -> Result<types::FieldElement> {
        let name = unsafe { CStr::from_ptr(name).to_string_lossy().into_owned() };
        let selector = match starknet::core::utils::get_selector_from_name(name.as_str()) {
            Ok(selector) => selector,
            Err(e) => return Result::Err(e.into()),
        };

        Result::Ok(selector.into())
    }

    /// Gets selector from tag string
    ///
    /// # Parameters
    /// * `tag` - Tag to compute selector from
    ///
    /// # Returns
    /// FieldElement containing the computed selector
    #[no_mangle]
    pub unsafe extern "C" fn get_selector_from_tag(tag: *const c_char) -> types::FieldElement {
        let tag = unsafe { CStr::from_ptr(tag).to_string_lossy().into_owned() };
        let selector = compute_selector_from_tag(tag.as_str());

        selector.into()
    }

    /// Computes Starknet keccak hash of bytes
    ///
    /// # Parameters
    /// * `bytes` - Byte array to hash
    /// * `bytes_len` - Length of byte array
    ///
    /// # Returns
    /// FieldElement containing the hash result
    #[no_mangle]
    pub unsafe extern "C" fn starknet_keccak(
        bytes: *const u8,
        bytes_len: usize,
    ) -> types::FieldElement {
        let bytes = unsafe { std::slice::from_raw_parts(bytes, bytes_len) };
        let hash = starknet::core::utils::starknet_keccak(bytes);

        hash.into()
    }

    /// Converts a short string to field element
    ///
    /// # Parameters
    /// * `str` - String to convert
    ///
    /// # Returns
    /// Result containing FieldElement or error
    #[no_mangle]
    pub unsafe extern "C" fn cairo_short_string_to_felt(
        str: *const c_char,
    ) -> Result<types::FieldElement> {
        let str = unsafe { CStr::from_ptr(str).to_string_lossy().into_owned() };
        let felt = match starknet::core::utils::cairo_short_string_to_felt(str.as_str()) {
            Ok(felt) => felt,
            Err(e) => return Result::Err(e.into()),
        };

        Result::Ok(felt.into())
    }

    /// Parses a field element into a short string
    ///
    /// # Parameters
    /// * `felt` - FieldElement to parse
    ///
    /// # Returns
    /// Result containing pointer to C string or error
    #[no_mangle]
    pub unsafe extern "C" fn parse_cairo_short_string(
        felt: types::FieldElement,
    ) -> Result<*const c_char> {
        let felt = felt.into();
        let str = match starknet::core::utils::parse_cairo_short_string(&felt) {
            Ok(str) => str,
            Err(e) => return Result::Err(e.into()),
        };

        Result::Ok(CString::new(str).unwrap().into_raw())
    }

    /// Encodes typed data
    ///
    /// # Parameters
    /// * `typed_data` - JSON string of typed data
    /// * `address` - Address as FieldElement
    ///
    /// # Returns
    /// Result containing encoded FieldElement or error
    #[no_mangle]
    pub unsafe extern "C" fn typed_data_encode(
        typed_data: *const c_char,
        address: types::FieldElement,
    ) -> Result<types::FieldElement> {
        let typed_data = unsafe { CStr::from_ptr(typed_data).to_string_lossy().into_owned() };
        let typed_data = match serde_json::from_str::<TypedData>(typed_data.as_str()) {
            Ok(typed_data) => typed_data,
            Err(err) => {
                return Result::Err(Error {
                    message: CString::new(format!("Invalid typed data: {err}")).unwrap().into_raw(),
                });
            }
        };

        let address = address.into();
        let encoded = match typed_data.message_hash(address) {
            Ok(encoded) => encoded,
            Err(err) => return Result::Err(err.into()),
        };

        Result::Ok(encoded.into())
    }

    /// Generates a new signing key
    ///
    /// # Returns
    /// FieldElement containing the new private key
    #[no_mangle]
    pub unsafe extern "C" fn signing_key_new() -> types::FieldElement {
        let private_key = SigningKey::from_random();
        private_key.secret_scalar().into()
    }

    /// Signs a hash with a private key
    ///
    /// # Parameters
    /// * `private_key` - Private key as FieldElement
    /// * `hash` - Hash to sign as FieldElement
    ///
    /// # Returns
    /// Result containing Signature or error
    #[no_mangle]
    pub unsafe extern "C" fn signing_key_sign(
        private_key: types::FieldElement,
        hash: types::FieldElement,
    ) -> Result<Signature> {
        let private_key = SigningKey::from_secret_scalar(private_key.into());
        let sig = private_key.sign(&hash.into());

        match sig {
            Ok(sig) => Result::Ok(sig.into()),
            Err(e) => Result::Err(e.into()),
        }
    }

    /// Creates a verifying key from a signing key
    ///
    /// # Parameters
    /// * `signing_key` - Signing key as FieldElement
    ///
    /// # Returns
    /// FieldElement containing the verifying key
    #[no_mangle]
    pub unsafe extern "C" fn verifying_key_new(
        signing_key: types::FieldElement,
    ) -> types::FieldElement {
        let signing_key = signing_key.into();
        let verifying_key = starknet_crypto::get_public_key(&signing_key);

        verifying_key.into()
    }

    /// Verifies a signature
    ///
    /// # Parameters
    /// * `verifying_key` - Verifying key as FieldElement
    /// * `hash` - Hash that was signed
    /// * `signature` - Signature to verify
    ///
    /// # Returns
    /// Result containing verification success boolean or error
    #[no_mangle]
    pub unsafe extern "C" fn verifying_key_verify(
        verifying_key: types::FieldElement,
        hash: types::FieldElement,
        signature: types::Signature,
    ) -> Result<bool> {
        let verifying_key = VerifyingKey::from_scalar(verifying_key.into());
        let signature = &signature.into();
        let hash = &hash.into();

        match verifying_key.verify(hash, signature) {
            Ok(result) => Result::Ok(result),
            Err(e) => Result::Err(e.into()),
        }
    }

    /// Creates a new provider instance
    ///
    /// # Parameters
    /// * `rpc_url` - URL of the RPC endpoint
    ///
    /// # Returns
    /// Result containing pointer to Provider or error
    #[no_mangle]
    pub unsafe extern "C" fn provider_new(rpc_url: *const c_char) -> Result<*mut Provider> {
        let rpc_url = unsafe { CStr::from_ptr(rpc_url).to_string_lossy() };
        let rpc_url = match url::Url::parse(rpc_url.deref()) {
            Ok(url) => url,
            Err(e) => return Result::Err(e.into()),
        };

        let rpc = JsonRpcClient::new(HttpTransport::new(rpc_url));

        Result::Ok(Box::into_raw(Box::new(Provider(Arc::new(rpc)))))
    }

    /// Creates a new account instance
    ///
    /// # Parameters
    /// * `rpc` - Pointer to Provider
    /// * `private_key` - Private key as FieldElement
    /// * `address` - Account address as string
    ///
    /// # Returns
    /// Result containing pointer to Account or error
    #[no_mangle]
    pub unsafe extern "C" fn account_new(
        rpc: *mut Provider,
        private_key: types::FieldElement,
        address: *const c_char,
    ) -> Result<*mut Account> {
        let address = unsafe { CStr::from_ptr(address).to_string_lossy() };
        let address = match Felt::from_hex(address.deref()) {
            Ok(address) => address,
            Err(e) => return Result::Err(e.into()),
        };

        let chain_id = match RUNTIME.block_on((*rpc).0.chain_id()) {
            Ok(chain_id) => chain_id,
            Err(e) => return Result::Err(e.into()),
        };

        let signer =
            LocalWallet::from_signing_key(SigningKey::from_secret_scalar(private_key.into()));
        let account = SingleOwnerAccount::new(
            (*rpc).0.clone(),
            signer,
            address,
            chain_id,
            ExecutionEncoding::New,
        );

        Result::Ok(Box::into_raw(Box::new(Account(account))))
    }

    /// Makes a Starknet call
    ///
    /// # Parameters
    /// * `provider` - Pointer to Provider
    /// * `call` - Call parameters
    /// * `block_id` - Block identifier
    ///
    /// # Returns
    /// Result containing array of FieldElements or error
    #[no_mangle]
    pub unsafe extern "C" fn starknet_call(
        provider: *mut Provider,
        call: Call,
        block_id: BlockId,
    ) -> Result<CArray<types::FieldElement>> {
        let res = match RUNTIME.block_on(
            (*provider)
                .0
                .call::<FunctionCall, starknet::core::types::BlockId>(call.into(), block_id.into()),
        ) {
            Ok(res) => res,
            Err(e) => return Result::Err(e.into()),
        };

        Result::Ok(res.into())
    }

    /// Deploys a burner account
    ///
    /// # Parameters
    /// * `provider` - Pointer to Provider
    /// * `master_account` - Pointer to master Account
    /// * `signing_key` - Signing key for new account
    ///
    /// # Returns
    /// Result containing pointer to new Account or error
    #[no_mangle]
    pub unsafe extern "C" fn account_deploy_burner(
        provider: *mut Provider,
        master_account: *mut Account,
        signing_key: types::FieldElement,
    ) -> Result<*mut Account> {
        let signing_key = SigningKey::from_secret_scalar(signing_key.into());
        let verifying_key = signing_key.verifying_key();
        let address = get_contract_address(
            verifying_key.scalar(),
            constants::KATANA_ACCOUNT_CLASS_HASH,
            &[verifying_key.scalar()],
            Felt::ZERO,
        );
        let signer = LocalWallet::from_signing_key(signing_key);

        let chain_id = (*master_account).0.chain_id();

        let account = SingleOwnerAccount::new(
            (*provider).0.clone(),
            signer,
            address,
            chain_id,
            ExecutionEncoding::New,
        );

        // deploy the burner
        let exec = (*master_account).0.execute_v3(vec![starknet::core::types::Call {
            to: constants::UDC_ADDRESS,
            calldata: vec![
                constants::KATANA_ACCOUNT_CLASS_HASH, // class_hash
                verifying_key.scalar(),               // salt
                Felt::ZERO,                           // deployer_address
                Felt::ONE,                            // constructor calldata length (1)
                verifying_key.scalar(),               // constructor calldata
            ],
            selector: starknet::core::utils::get_selector_from_name("deployContract").unwrap(),
        }]);

        let result = match RUNTIME.block_on(exec.send()) {
            Ok(result) => result,
            Err(e) => return Result::Err(e.into()),
        };

        match RUNTIME.block_on(watch_tx(&(*provider).0, result.transaction_hash)) {
            Ok(_) => Result::Ok(Box::into_raw(Box::new(Account(account)))),
            Err(e) => {
                Result::Err(Error { message: CString::new(e.to_string()).unwrap().into_raw() })
            }
        }
    }

    /// Gets account address
    ///
    /// # Parameters
    /// * `account` - Pointer to Account
    ///
    /// # Returns
    /// FieldElement containing the account address
    #[no_mangle]
    pub unsafe extern "C" fn account_address(account: *mut Account) -> types::FieldElement {
        (*account).0.address().into()
    }

    /// Gets account chain ID
    ///
    /// # Parameters
    /// * `account` - Pointer to Account
    ///
    /// # Returns
    /// FieldElement containing the chain ID
    #[no_mangle]
    pub unsafe extern "C" fn account_chain_id(account: *mut Account) -> types::FieldElement {
        (*account).0.chain_id().into()
    }

    /// Sets block ID for account
    ///
    /// # Parameters
    /// * `account` - Pointer to Account
    /// * `block_id` - New block ID
    #[no_mangle]
    pub unsafe extern "C" fn account_set_block_id(account: *mut Account, block_id: BlockId) {
        let block_id = block_id.into();
        (*account).0.set_block_id(block_id);
    }

    /// Gets account nonce
    ///
    /// # Parameters
    /// * `account` - Pointer to Account
    ///
    /// # Returns
    /// Result containing FieldElement nonce or error
    #[no_mangle]
    pub unsafe extern "C" fn account_nonce(account: *mut Account) -> Result<types::FieldElement> {
        let nonce = match RUNTIME.block_on((*account).0.get_nonce()) {
            Ok(nonce) => nonce,
            Err(e) => return Result::Err(e.into()),
        };

        Result::Ok(nonce.into())
    }

    /// Executes raw transaction
    ///
    /// # Parameters
    /// * `account` - Pointer to Account
    /// * `calldata` - Array of Call structs
    /// * `calldata_len` - Length of calldata array
    ///
    /// # Returns
    /// Result containing transaction hash as FieldElement or error
    #[no_mangle]
    pub unsafe extern "C" fn account_execute_raw(
        account: *mut Account,
        calldata: *const Call,
        calldata_len: usize,
    ) -> Result<types::FieldElement> {
        let calldata = unsafe { std::slice::from_raw_parts(calldata, calldata_len).to_vec() };
        let calldata =
            calldata.into_iter().map(|c| c.into()).collect::<Vec<starknet::core::types::Call>>();
        let call = (*account).0.execute_v3(calldata);

        match RUNTIME.block_on(call.send()) {
            Ok(result) => Result::Ok(result.transaction_hash.into()),
            Err(e) => Result::Err(e.into()),
        }
    }

    /// Waits for transaction completion
    ///
    /// # Parameters
    /// * `rpc` - Pointer to Provider
    /// * `txn_hash` - Transaction hash as FieldElement
    ///
    /// # Returns
    /// Result containing success boolean or error
    #[no_mangle]
    pub unsafe extern "C" fn wait_for_transaction(
        rpc: *mut Provider,
        txn_hash: types::FieldElement,
    ) -> Result<bool> {
        let txn_hash = txn_hash.into();
        match RUNTIME.block_on(watch_tx(&(*rpc).0, txn_hash)) {
            Ok(_) => Result::Ok(true),
            Err(e) => {
                Result::Err(Error { message: CString::new(e.to_string()).unwrap().into_raw() })
            }
        }
    }

    /// Computes contract address
    ///
    /// # Parameters
    /// * `class_hash` - Class hash as FieldElement
    /// * `salt` - Salt as FieldElement
    /// * `constructor_calldata` - Array of constructor parameters
    /// * `constructor_calldata_len` - Length of constructor parameters
    /// * `deployer_address` - Deployer address as FieldElement
    ///
    /// # Returns
    /// FieldElement containing computed contract address
    #[no_mangle]
    pub unsafe extern "C" fn hash_get_contract_address(
        class_hash: types::FieldElement,
        salt: types::FieldElement,
        constructor_calldata: *const types::FieldElement,
        constructor_calldata_len: usize,
        deployer_address: types::FieldElement,
    ) -> types::FieldElement {
        let class_hash = class_hash.into();
        let salt = salt.into();
        let constructor_calldata = unsafe {
            std::slice::from_raw_parts(constructor_calldata, constructor_calldata_len).to_vec()
        };
        let constructor_calldata =
            constructor_calldata.iter().map(|f| f.clone().into()).collect::<Vec<Felt>>();
        let deployer_address = deployer_address.into();

        let address =
            get_contract_address(salt, class_hash, &constructor_calldata, deployer_address);

        address.into()
    }

    /// Cancels a subscription
    ///
    /// # Parameters
    /// * `subscription` - Pointer to Subscription to cancel
    #[no_mangle]
    pub unsafe extern "C" fn subscription_cancel(subscription: *mut Subscription) {
        if !subscription.is_null() {
            unsafe {
                let subscription = Box::from_raw(subscription);
                subscription.trigger.cancel();
            }
        }
    }

    /// Frees a ToriiClient instance
    ///
    /// # Parameters
    /// * `t` - Pointer to ToriiClient to free
    #[no_mangle]
    pub unsafe extern "C" fn client_free(t: *mut ToriiClient) {
        if !t.is_null() {
            unsafe {
                let _ = Box::from_raw(t);
            }
        }
    }

    /// Frees a Provider instance
    ///
    /// # Parameters
    /// * `rpc` - Pointer to Provider to free
    #[no_mangle]
    pub unsafe extern "C" fn provider_free(rpc: *mut Provider) {
        if !rpc.is_null() {
            unsafe {
                let _ = Box::from_raw(rpc);
            }
        }
    }

    /// Frees a Model instance
    ///
    /// # Parameters
    /// * `model` - Pointer to Model to free
    #[no_mangle]
    pub unsafe extern "C" fn model_free(model: *mut Struct) {
        if !model.is_null() {
            let _: dojo_types::schema::Struct = (*Box::<Struct>::from_raw(model)).into();
        }
    }

    /// Frees an Account instance
    ///
    /// # Parameters
    /// * `account` - Pointer to Account to free
    #[no_mangle]
    pub unsafe extern "C" fn account_free(account: *mut Account) {
        if !account.is_null() {
            unsafe {
                let _ = Box::from_raw(account);
            }
        }
    }

    /// Frees a Type instance
    ///
    /// # Parameters
    /// * `ty` - Pointer to Type to free
    #[no_mangle]
    pub unsafe extern "C" fn ty_free(ty: *mut Ty) {
        if !ty.is_null() {
            let _: dojo_types::schema::Ty = (*Box::<Ty>::from_raw(ty)).into();
        }
    }

    /// Frees an Entity instance
    ///
    /// # Parameters
    /// * `entity` - Pointer to Entity to free
    #[no_mangle]
    pub unsafe extern "C" fn entity_free(entity: *mut Entity) {
        if !entity.is_null() {
            let _: torii_proto::schema::Entity = (*Box::<Entity>::from_raw(entity)).into();
        }
    }

    /// Frees an Error instance
    ///
    /// # Parameters
    /// * `error` - Pointer to Error to free
    #[no_mangle]
    pub unsafe extern "C" fn error_free(error: *mut Error) {
        if !error.is_null() {
            let _: String = CString::from_raw((*error).message).into_string().unwrap();
        }
    }

    /// Frees a WorldMetadata instance
    ///
    /// # Parameters
    /// * `metadata` - Pointer to WorldMetadata to free
    #[no_mangle]
    pub unsafe extern "C" fn world_metadata_free(metadata: *mut World) {
        if !metadata.is_null() {
            let _: torii_proto::World = (*Box::<World>::from_raw(metadata)).into();
        }
    }

    /// Frees a CArray instance
    ///
    /// # Parameters
    /// * `data` - Pointer to array data
    /// * `data_len` - Length of array
    #[no_mangle]
    pub unsafe extern "C" fn carray_free(data: *mut c_void, data_len: usize) {
        if !data.is_null() {
            let _: Vec<c_void> = Vec::from_raw_parts(data, data_len, data_len);
        }
    }
    /// Frees a string
    ///
    /// # Parameters
    /// * `string` - Pointer to string to free
    #[no_mangle]
    pub unsafe extern "C" fn string_free(string: *mut c_char) {
        if !string.is_null() {
            let _: String = CString::from_raw(string).into_string().unwrap();
        }
    }
}
