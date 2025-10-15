// Client wrapper for UniFFI - exposes torii_client functionality

use super::types::*;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU64, Ordering};
use std::collections::HashMap;
use tokio::task::JoinHandle;

// Callback traits for subscriptions
pub trait EntityUpdateCallback: Send + Sync {
    fn on_update(&self, entity: Entity);
    fn on_error(&self, error: String);
}

pub trait TokenBalanceUpdateCallback: Send + Sync {
    fn on_update(&self, balance: TokenBalance);
    fn on_error(&self, error: String);
}

pub trait TokenUpdateCallback: Send + Sync {
    fn on_update(&self, token: Token);
    fn on_error(&self, error: String);
}

pub trait TransactionUpdateCallback: Send + Sync {
    fn on_update(&self, transaction: Transaction);
    fn on_error(&self, error: String);
}

pub trait EventUpdateCallback: Send + Sync {
    fn on_update(&self, event: Event);
    fn on_error(&self, error: String);
}

/// Main Dojo client for interacting with the Torii indexer
pub struct ToriiClient {
    inner: torii_client::Client,
    subscriptions: Arc<Mutex<HashMap<u64, JoinHandle<()>>>>,
    next_sub_id: Arc<AtomicU64>,
}

impl ToriiClient {
    /// Create a new Torii client with default configuration (4MB max message size)
    pub async fn new(torii_url: String) -> Result<Self, DojoError> {
        let client = torii_client::Client::new(torii_url)
            .await
            .map_err(|_e| DojoError::ConnectionError)?;
        Ok(Self {
            inner: client,
            subscriptions: Arc::new(Mutex::new(HashMap::new())),
            next_sub_id: Arc::new(AtomicU64::new(0)),
        })
    }

    /// Create a new Torii client with custom max message size
    pub async fn new_with_config(
        torii_url: String,
        max_message_size: u64,
    ) -> Result<Self, DojoError> {
        let client = torii_client::Client::new_with_config(torii_url, max_message_size as usize)
            .await
            .map_err(|_e| DojoError::ConnectionError)?;
        Ok(Self {
            inner: client,
            subscriptions: Arc::new(Mutex::new(HashMap::new())),
            next_sub_id: Arc::new(AtomicU64::new(0)),
        })
    }

    /// Publish an offchain message to the world
    /// Returns the entity ID of the published message
    pub async fn publish_message(&self, message: Message) -> Result<String, DojoError> {
        let msg: torii_proto::Message = message.into();
        self.inner
            .publish_message(msg)
            .await
            .map_err(|_| DojoError::PublishError)
    }

    /// Publish multiple offchain messages to the world
    /// Returns the entity IDs of the published messages
    pub async fn publish_message_batch(&self, messages: Vec<Message>) -> Result<Vec<String>, DojoError> {
        let msgs: Vec<torii_proto::Message> = messages
            .into_iter()
            .map(|m| m.into())
            .collect();
        self.inner
            .publish_message_batch(msgs)
            .await
            .map_err(|_| DojoError::PublishError)
    }

    /// Get world metadata for specified world addresses
    pub async fn worlds(&self, world_addresses: Vec<FieldElement>) -> Result<Vec<World>, DojoError> {
        let addrs: Result<Vec<starknet::core::types::Felt>, DojoError> = world_addresses
            .iter()
            .map(field_element_to_felt)
            .collect();
        
        let worlds = self.inner
            .worlds(addrs?)
            .await
            .map_err(|_| DojoError::QueryError)?;
        
        Ok(worlds.into_iter().map(|w| w.into()).collect())
    }

    /// Retrieve controllers matching the query
    pub async fn controllers(&self, query: ControllerQuery) -> Result<PageController, DojoError> {
        let q: torii_proto::ControllerQuery = query.into();
        let page = self.inner
            .controllers(q)
            .await
            .map_err(|_| DojoError::QueryError)?;
        
        Ok(PageController {
            items: page.items.into_iter().map(|c| c.into()).collect(),
            next_cursor: page.next_cursor,
        })
    }

    /// Retrieve contracts matching the query
    pub async fn contracts(&self, query: ContractQuery) -> Result<Vec<Contract>, DojoError> {
        let q: torii_proto::ContractQuery = query.into();
        let contracts = self.inner
            .contracts(q)
            .await
            .map_err(|_| DojoError::QueryError)?;
        
        Ok(contracts.into_iter().map(|c| c.into()).collect())
    }

    /// Retrieve tokens matching the query
    pub async fn tokens(&self, query: TokenQuery) -> Result<PageToken, DojoError> {
        let q: torii_proto::TokenQuery = query.into();
        let page = self.inner
            .tokens(q)
            .await
            .map_err(|_| DojoError::QueryError)?;
        
        Ok(PageToken {
            items: page.items.into_iter().map(|t| t.into()).collect(),
            next_cursor: page.next_cursor,
        })
    }

    /// Retrieve token balances
    pub async fn token_balances(&self, query: TokenBalanceQuery) -> Result<PageTokenBalance, DojoError> {
        let q: torii_proto::TokenBalanceQuery = query.into();
        let page = self.inner
            .token_balances(q)
            .await
            .map_err(|_| DojoError::QueryError)?;
        
        Ok(PageTokenBalance {
            items: page.items.into_iter().map(|b| b.into()).collect(),
            next_cursor: page.next_cursor,
        })
    }

    /// Retrieve token contracts
    pub async fn token_contracts(&self, query: TokenContractQuery) -> Result<PageTokenContract, DojoError> {
        let q: torii_proto::TokenContractQuery = query.into();
        let page = self.inner
            .token_contracts(q)
            .await
            .map_err(|_| DojoError::QueryError)?;
        
        Ok(PageTokenContract {
            items: page.items.into_iter().map(|tc| tc.into()).collect(),
            next_cursor: page.next_cursor,
        })
    }

    /// Retrieve token transfers
    pub async fn token_transfers(&self, query: TokenTransferQuery) -> Result<PageTokenTransfer, DojoError> {
        let q: torii_proto::TokenTransferQuery = query.into();
        let page = self.inner
            .token_transfers(q)
            .await
            .map_err(|_| DojoError::QueryError)?;
        
        Ok(PageTokenTransfer {
            items: page.items.into_iter().map(|t| t.into()).collect(),
            next_cursor: page.next_cursor,
        })
    }

    /// Retrieve transactions
    pub async fn transactions(&self, query: TransactionQuery) -> Result<PageTransaction, DojoError> {
        let q: torii_proto::TransactionQuery = query.into();
        let page = self.inner
            .transactions(q)
            .await
            .map_err(|_| DojoError::QueryError)?;
        
        Ok(PageTransaction {
            items: page.items.into_iter().map(|t| t.into()).collect(),
            next_cursor: page.next_cursor,
        })
    }

    /// Retrieve aggregations (leaderboards, stats, rankings)
    pub async fn aggregations(&self, query: AggregationQuery) -> Result<PageAggregationEntry, DojoError> {
        let q: torii_proto::AggregationQuery = query.into();
        let page = self.inner
            .aggregations(q)
            .await
            .map_err(|_| DojoError::QueryError)?;
        
        Ok(PageAggregationEntry {
            items: page.items.into_iter().map(|a| a.into()).collect(),
            next_cursor: page.next_cursor,
        })
    }

    /// Retrieve activities (user session tracking)
    pub async fn activities(&self, query: ActivityQuery) -> Result<PageActivity, DojoError> {
        let q: torii_proto::ActivityQuery = query.into();
        let page = self.inner
            .activities(q)
            .await
            .map_err(|_| DojoError::QueryError)?;
        
        Ok(PageActivity {
            items: page.items.into_iter().map(|a| a.into()).collect(),
            next_cursor: page.next_cursor,
        })
    }

    /// Retrieve achievements
    pub async fn achievements(&self, query: AchievementQuery) -> Result<PageAchievement, DojoError> {
        let q: torii_proto::AchievementQuery = query.into();
        let page = self.inner
            .achievements(q)
            .await
            .map_err(|_| DojoError::QueryError)?;
        
        Ok(PageAchievement {
            items: page.items.into_iter().map(|a| a.into()).collect(),
            next_cursor: page.next_cursor,
        })
    }

    /// Retrieve player achievements
    pub async fn player_achievements(&self, query: PlayerAchievementQuery) -> Result<PagePlayerAchievement, DojoError> {
        let q: torii_proto::PlayerAchievementQuery = query.into();
        let page = self.inner
            .player_achievements(q)
            .await
            .map_err(|_| DojoError::QueryError)?;
        
        Ok(PagePlayerAchievement {
            items: page.items.into_iter().map(|p| p.into()).collect(),
            next_cursor: page.next_cursor,
        })
    }

    /// Retrieve entities matching the query
    pub async fn entities(&self, query: Query) -> Result<PageEntity, DojoError> {
        let q: torii_proto::Query = query.into();
        let page = self.inner
            .entities(q)
            .await
            .map_err(|_| DojoError::QueryError)?;
        
        Ok(PageEntity {
            items: page.items.into_iter().map(|e| e.into()).collect(),
            next_cursor: page.next_cursor,
        })
    }

    /// Retrieve event messages matching the query
    pub async fn event_messages(&self, query: Query) -> Result<PageEntity, DojoError> {
        let q: torii_proto::Query = query.into();
        let page = self.inner
            .event_messages(q)
            .await
            .map_err(|_| DojoError::QueryError)?;
        
        Ok(PageEntity {
            items: page.items.into_iter().map(|e| e.into()).collect(),
            next_cursor: page.next_cursor,
        })
    }

    /// Retrieve raw Starknet events
    pub async fn starknet_events(&self, query: EventQuery) -> Result<PageEvent, DojoError> {
        let q: torii_proto::EventQuery = query.try_into()?;
        let page = self.inner
            .starknet_events(q)
            .await
            .map_err(|_| DojoError::QueryError)?;
        
        Ok(PageEvent {
            items: page.items.into_iter().map(|e| e.into()).collect(),
            next_cursor: page.next_cursor,
        })
    }

    /// Execute a SQL query against the Torii database
    pub async fn sql(&self, query: String) -> Result<Vec<SqlRow>, DojoError> {
        let rows = self.inner
            .sql(query)
            .await
            .map_err(|_| DojoError::QueryError)?;
        
        rows.into_iter().map(|r| r.try_into()).collect()
    }

    /// Subscribe to entity updates
    pub async fn subscribe_entity_updates(
        &self,
        clause: Option<Clause>,
        world_addresses: Vec<FieldElement>,
        callback: Box<dyn EntityUpdateCallback>,
    ) -> Result<u64, DojoError> {
        let sub_id = self.next_sub_id.fetch_add(1, Ordering::SeqCst);
        
        let addrs: Result<Vec<starknet::core::types::Felt>, DojoError> = world_addresses
            .iter()
            .map(field_element_to_felt)
            .collect();
        let addrs = addrs?;
        
        let clause_proto = clause.map(|c| c.into());
        
        let mut stream = self.inner
            .on_entity_updated(clause_proto, addrs)
            .await
            .map_err(|_| DojoError::SubscriptionError)?;
        
        let handle = tokio::spawn(async move {
            use futures_util::StreamExt;
            // Skip the first message which contains the subscription ID
            let _ = stream.next().await;
            
            while let Some(result) = stream.next().await {
                match result {
                    Ok((_id, entity)) => {
                        callback.on_update(entity.into());
                    }
                    Err(e) => {
                        callback.on_error(e.to_string());
                        break;
                    }
                }
            }
        });
        
        self.subscriptions.lock().unwrap().insert(sub_id, handle);
        Ok(sub_id)
    }

    /// Subscribe to token balance updates
    pub async fn subscribe_token_balance_updates(
        &self,
        contract_addresses: Vec<FieldElement>,
        account_addresses: Vec<FieldElement>,
        token_ids: Vec<U256>,
        callback: Box<dyn TokenBalanceUpdateCallback>,
    ) -> Result<u64, DojoError> {
        let sub_id = self.next_sub_id.fetch_add(1, Ordering::SeqCst);
        
        let contracts: Result<Vec<starknet::core::types::Felt>, DojoError> = contract_addresses
            .iter()
            .map(field_element_to_felt)
            .collect();
        let accounts: Result<Vec<starknet::core::types::Felt>, DojoError> = account_addresses
            .iter()
            .map(field_element_to_felt)
            .collect();
        let ids: Result<Vec<crypto_bigint::U256>, DojoError> = token_ids
            .iter()
            .map(uniffi_to_u256)
            .collect();
        
        let mut stream = self.inner
            .on_token_balance_updated(contracts?, accounts?, ids?)
            .await
            .map_err(|_| DojoError::SubscriptionError)?;
        
        let handle = tokio::spawn(async move {
            use futures_util::StreamExt;
            // Skip the first message which contains the subscription ID
            let _ = stream.next().await;
            
            while let Some(result) = stream.next().await {
                match result {
                    Ok((_id, balance)) => {
                        callback.on_update(balance.into());
                    }
                    Err(e) => {
                        callback.on_error(e.to_string());
                        break;
                    }
                }
            }
        });
        
        self.subscriptions.lock().unwrap().insert(sub_id, handle);
        Ok(sub_id)
    }

    /// Subscribe to token updates
    pub async fn subscribe_token_updates(
        &self,
        contract_addresses: Vec<FieldElement>,
        token_ids: Vec<U256>,
        callback: Box<dyn TokenUpdateCallback>,
    ) -> Result<u64, DojoError> {
        let sub_id = self.next_sub_id.fetch_add(1, Ordering::SeqCst);
        
        let contracts: Result<Vec<starknet::core::types::Felt>, DojoError> = contract_addresses
            .iter()
            .map(field_element_to_felt)
            .collect();
        let ids: Result<Vec<crypto_bigint::U256>, DojoError> = token_ids
            .iter()
            .map(uniffi_to_u256)
            .collect();
        
        let mut stream = self.inner
            .on_token_updated(contracts?, ids?)
            .await
            .map_err(|_| DojoError::SubscriptionError)?;
        
        let handle = tokio::spawn(async move {
            use futures_util::StreamExt;
            // Skip the first message which contains the subscription ID
            let _ = stream.next().await;
            
            while let Some(result) = stream.next().await {
                match result {
                    Ok((_id, token)) => {
                        callback.on_update(token.into());
                    }
                    Err(e) => {
                        callback.on_error(e.to_string());
                        break;
                    }
                }
            }
        });
        
        self.subscriptions.lock().unwrap().insert(sub_id, handle);
        Ok(sub_id)
    }

    /// Subscribe to transaction updates
    pub async fn subscribe_transaction_updates(
        &self,
        filter: Option<TransactionFilter>,
        callback: Box<dyn TransactionUpdateCallback>,
    ) -> Result<u64, DojoError> {
        let sub_id = self.next_sub_id.fetch_add(1, Ordering::SeqCst);
        
        let filter_proto = filter.map(|f| f.into());
        
        let mut stream = self.inner
            .on_transaction(filter_proto)
            .await
            .map_err(|_| DojoError::SubscriptionError)?;
        
        let handle = tokio::spawn(async move {
            use futures_util::StreamExt;
            // Skip the first message which contains the subscription ID
            let _ = stream.next().await;
            
            while let Some(result) = stream.next().await {
                match result {
                    Ok(transaction) => {
                        callback.on_update(transaction.into());
                    }
                    Err(e) => {
                        callback.on_error(e.to_string());
                        break;
                    }
                }
            }
        });
        
        self.subscriptions.lock().unwrap().insert(sub_id, handle);
        Ok(sub_id)
    }

    /// Subscribe to Starknet event updates
    pub async fn subscribe_event_updates(
        &self,
        keys: Vec<KeysClause>,
        callback: Box<dyn EventUpdateCallback>,
    ) -> Result<u64, DojoError> {
        let sub_id = self.next_sub_id.fetch_add(1, Ordering::SeqCst);
        
        let keys_proto: Vec<torii_proto::KeysClause> = keys.into_iter().map(|k| k.into()).collect();
        
        let mut stream = self.inner
            .on_starknet_event(keys_proto)
            .await
            .map_err(|_| DojoError::SubscriptionError)?;
        
        let handle = tokio::spawn(async move {
            use futures_util::StreamExt;
            // Skip the first message which contains the subscription ID
            let _ = stream.next().await;
            
            while let Some(result) = stream.next().await {
                match result {
                    Ok(event) => {
                        callback.on_update(event.into());
                    }
                    Err(e) => {
                        callback.on_error(e.to_string());
                        break;
                    }
                }
            }
        });
        
        self.subscriptions.lock().unwrap().insert(sub_id, handle);
        Ok(sub_id)
    }

    /// Cancel a subscription
    pub fn cancel_subscription(&self, subscription_id: u64) -> Result<(), DojoError> {
        let mut subs = self.subscriptions.lock().unwrap();
        if let Some(handle) = subs.remove(&subscription_id) {
            handle.abort();
            Ok(())
        } else {
            Err(DojoError::SubscriptionError)
        }
    }
}
