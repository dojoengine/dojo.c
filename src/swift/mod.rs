use std::sync::Arc;
use starknet::core::types::Felt;
use crypto_bigint::Encoding;

#[swift_bridge::bridge]
mod ffi {
    // ============================================================================
    // Core Types
    // ============================================================================
    
    extern "Rust" {
        type ToriiClient;
        type ToriiClientError;
        type Subscription;
        
        async fn new_torii_client(torii_url: String) -> Result<ToriiClient, ToriiClientError>;
        
        async fn new_torii_client_with_config(
            torii_url: String,
            max_message_size: usize
        ) -> Result<ToriiClient, ToriiClientError>;
        
        fn message(self: &ToriiClientError) -> String;
        
        fn id(self: &Subscription) -> u64;
        fn cancel(self: Subscription);
    }
    
    // ============================================================================
    // Basic Transparent Types
    // ============================================================================
    
    #[swift_bridge(swift_repr = "struct")]
    struct FeltBridge {
        hex: String,
    }
    
    #[swift_bridge(swift_repr = "struct")]
    struct PageBridge {
        data: String,
    }
    
    #[swift_bridge(swift_repr = "struct")]
    struct WorldBridge {
        world_address: FeltBridge,
        models_json: String,
    }
    
    #[swift_bridge(swift_repr = "struct")]
    struct QueryBridge {
        limit: u32,
        cursor: String,
        world_addresses: Vec<FeltBridge>,
        dont_include_hashed_keys: bool,
    }
    
    #[swift_bridge(swift_repr = "struct")]
    struct U256Bridge {
        hex: String,
    }
    
    // ============================================================================
    // Entity & Model Types
    // ============================================================================
    
    #[swift_bridge(swift_repr = "struct")]
    struct EntityBridge {
        world_address: FeltBridge,
        hashed_keys: FeltBridge,
        models_json: String,
        created_at: u64,
        updated_at: u64,
        executed_at: u64,
    }
    
    #[swift_bridge(swift_repr = "struct")]
    struct ModelBridge {
        world_address: FeltBridge,
        namespace: String,
        name: String,
        selector: FeltBridge,
        packed_size: u32,
        unpacked_size: u32,
        class_hash: FeltBridge,
        contract_address: FeltBridge,
        layout_json: String,
        schema_json: String,
        use_legacy_store: bool,
    }
    
    // ============================================================================
    // Token Types
    // ============================================================================
    
    #[swift_bridge(swift_repr = "struct")]
    struct TokenBridge {
        contract_address: FeltBridge,
        token_id: Optional<String>,
        name: String,
        symbol: String,
        decimals: u8,
        metadata: String,
        total_supply: String,  // Optional U256 as JSON
    }
    
    #[swift_bridge(swift_repr = "struct")]
    struct TokenBalanceBridge {
        balance: U256Bridge,
        account_address: FeltBridge,
        contract_address: FeltBridge,
        token_id: String,  // Optional U256 as JSON
    }
    
    #[swift_bridge(swift_repr = "struct")]
    struct TokenContractBridge {
        contract_address: FeltBridge,
        name: String,
        symbol: String,
        decimals: u8,
        metadata: String,
        token_metadata: String,
        total_supply: String,  // Optional U256 as JSON
    }
    
    #[swift_bridge(swift_repr = "struct")]
    struct TokenTransferBridge {
        id: String,
        contract_address: FeltBridge,
        from_address: FeltBridge,
        to_address: FeltBridge,
        amount: U256Bridge,
        token_id: String,  // Optional U256 as JSON
        executed_at: u64,
        event_id: String,  // Optional string
    }
    
    // ============================================================================
    // Contract & Controller Types
    // ============================================================================
    
    #[swift_bridge(swift_repr = "struct")]
    struct ControllerBridge {
        address: FeltBridge,
        username: String,
        deployed_at_timestamp: u64,
    }
    
    #[swift_bridge(swift_repr = "struct")]
    struct ContractBridge {
        contract_address: FeltBridge,
        contract_type: String,
        head: String,  // Optional u64 as JSON
        tps: String,  // Optional u64 as JSON
        last_block_timestamp: String,  // Optional u64 as JSON
        last_pending_block_tx: String,  // Optional Felt as JSON
        updated_at: u64,
        created_at: u64,
    }
    
    // ============================================================================
    // Transaction Types
    // ============================================================================
    
    #[swift_bridge(swift_repr = "struct")]
    struct TransactionBridge {
        transaction_hash: FeltBridge,
        sender_address: FeltBridge,
        calldata: Vec<FeltBridge>,
        max_fee: FeltBridge,
        signature: Vec<FeltBridge>,
        nonce: FeltBridge,
        block_number: u64,
        transaction_type: String,
        block_timestamp: u64,
        calls_json: String,
        unique_models: Vec<FeltBridge>,
    }
    
    #[swift_bridge(swift_repr = "struct")]
    struct TransactionCallBridge {
        contract_address: FeltBridge,
        entrypoint: String,
        calldata: Vec<FeltBridge>,
        call_type: String,
        caller_address: FeltBridge,
    }
    
    // ============================================================================
    // Event Types
    // ============================================================================
    
    #[swift_bridge(swift_repr = "struct")]
    struct EventBridge {
        keys: Vec<FeltBridge>,
        data: Vec<FeltBridge>,
        transaction_hash: FeltBridge,
    }
    
    // ============================================================================
    // Aggregation & Activity Types
    // ============================================================================
    
    #[swift_bridge(swift_repr = "struct")]
    struct AggregationEntryBridge {
        id: String,
        aggregator_id: String,
        entity_id: String,
        value: U256Bridge,
        display_value: String,
        position: u64,
        model_id: String,
        created_at: u64,
        updated_at: u64,
    }
    
    #[swift_bridge(swift_repr = "struct")]
    struct ActivityBridge {
        id: String,
        world_address: FeltBridge,
        namespace: String,
        caller_address: FeltBridge,
        session_start: u64,
        session_end: u64,
        action_count: u32,
        actions_json: String,
        updated_at: u64,
    }
    
    #[swift_bridge(swift_repr = "struct")]
    struct ActionCountBridge {
        action_name: String,
        count: u32,
    }
    
    // ============================================================================
    // Achievement Types
    // ============================================================================
    
    #[swift_bridge(swift_repr = "struct")]
    struct AchievementBridge {
        id: String,
        world_address: FeltBridge,
        namespace: String,
        entity_id: String,
        hidden: bool,
        index: u32,
        points: u32,
        start: String,
        end: String,
        group: String,
        icon: String,
        title: String,
        description: String,
        tasks_json: String,
        data: String,
        total_completions: u32,
        completion_rate: f64,
        created_at: u64,
        updated_at: u64,
    }
    
    #[swift_bridge(swift_repr = "struct")]
    struct AchievementTaskBridge {
        task_id: String,
        description: String,
        total: u32,
        total_completions: u32,
        completion_rate: f64,
        created_at: u64,
    }
    
    #[swift_bridge(swift_repr = "struct")]
    struct PlayerAchievementEntryBridge {
        player_address: FeltBridge,
        stats: PlayerAchievementStatsBridge,
        achievements_json: String,
    }
    
    #[swift_bridge(swift_repr = "struct")]
    struct PlayerAchievementStatsBridge {
        total_points: u32,
        completed_achievements: u32,
        total_achievements: u32,
        completion_percentage: f64,
        last_achievement_at: String,  // Optional u64 as JSON
        created_at: u64,
        updated_at: u64,
    }
    
    #[swift_bridge(swift_repr = "struct")]
    struct PlayerAchievementProgressBridge {
        achievement: AchievementBridge,
        task_progress_json: String,
        completed: bool,
        progress_percentage: f64,
    }
    
    #[swift_bridge(swift_repr = "struct")]
    struct TaskProgressBridge {
        task_id: String,
        count: u32,
        completed: bool,
    }
    
    // ============================================================================
    // World Operations
    // ============================================================================
    
    extern "Rust" {
        async fn worlds(
            self: &ToriiClient,
            world_addresses: Vec<FeltBridge>
        ) -> Result<Vec<WorldBridge>, ToriiClientError>;
    }
    
    // ============================================================================
    // Message Operations
    // ============================================================================
    
    extern "Rust" {
        async fn publish_message(
            self: &ToriiClient,
            message_json: String
        ) -> Result<String, ToriiClientError>;
        
        async fn publish_message_batch(
            self: &ToriiClient,
            messages_json: String
        ) -> Result<String, ToriiClientError>;
    }
    
    // ============================================================================
    // Entity Operations
    // ============================================================================
    
    extern "Rust" {
        async fn entities(
            self: &ToriiClient,
            query: QueryBridge
        ) -> Result<PageBridge, ToriiClientError>;
        
        async fn event_messages(
            self: &ToriiClient,
            query: QueryBridge
        ) -> Result<PageBridge, ToriiClientError>;
        
        async fn starknet_events(
            self: &ToriiClient,
            keys_json: String,
            limit: u32,
            cursor: String
        ) -> Result<PageBridge, ToriiClientError>;
    }
    
    // ============================================================================
    // Token Operations  
    // ============================================================================
    
    extern "Rust" {
        async fn tokens(
            self: &ToriiClient,
            contract_addresses: Vec<FeltBridge>,
            limit: u32,
            cursor: String
        ) -> Result<PageBridge, ToriiClientError>;
        
        async fn token_balances(
            self: &ToriiClient,
            account_addresses: Vec<FeltBridge>,
            contract_addresses: Vec<FeltBridge>,
            limit: u32,
            cursor: String
        ) -> Result<PageBridge, ToriiClientError>;
        
        async fn token_contracts(
            self: &ToriiClient,
            contract_addresses: Vec<FeltBridge>,
            limit: u32,
            cursor: String
        ) -> Result<PageBridge, ToriiClientError>;
        
        async fn token_transfers(
            self: &ToriiClient,
            account_addresses: Vec<FeltBridge>,
            contract_addresses: Vec<FeltBridge>,
            limit: u32,
            cursor: String
        ) -> Result<PageBridge, ToriiClientError>;
    }
    
    // ============================================================================
    // Transaction Operations
    // ============================================================================
    
    extern "Rust" {
        async fn transactions(
            self: &ToriiClient,
            filter_json: String,
            limit: u32,
            cursor: String
        ) -> Result<PageBridge, ToriiClientError>;
    }
    
    // ============================================================================
    // Contract & Controller Operations
    // ============================================================================
    
    extern "Rust" {
        async fn contracts(
            self: &ToriiClient,
            contract_addresses: Vec<FeltBridge>
        ) -> Result<String, ToriiClientError>;
        
        async fn controllers(
            self: &ToriiClient,
            contract_addresses: Vec<FeltBridge>,
            limit: u32,
            cursor: String
        ) -> Result<PageBridge, ToriiClientError>;
    }
    
    // ============================================================================
    // Aggregation Operations
    // ============================================================================
    
    extern "Rust" {
        async fn aggregations(
            self: &ToriiClient,
            aggregator_ids: Vec<String>,
            entity_ids: Vec<String>,
            limit: u32,
            cursor: String
        ) -> Result<PageBridge, ToriiClientError>;
    }
    
    // ============================================================================
    // Activity Operations
    // ============================================================================
    
    extern "Rust" {
        async fn activities(
            self: &ToriiClient,
            world_addresses: Vec<FeltBridge>,
            namespaces: Vec<String>,
            caller_addresses: Vec<FeltBridge>,
            limit: u32,
            cursor: String
        ) -> Result<PageBridge, ToriiClientError>;
    }
    
    // ============================================================================
    // Achievement Operations
    // ============================================================================
    
    extern "Rust" {
        async fn achievements(
            self: &ToriiClient,
            world_addresses: Vec<FeltBridge>,
            namespaces: Vec<String>,
            limit: u32,
            cursor: String
        ) -> Result<PageBridge, ToriiClientError>;
        
        async fn player_achievements(
            self: &ToriiClient,
            world_addresses: Vec<FeltBridge>,
            namespaces: Vec<String>,
            player_addresses: Vec<FeltBridge>,
            limit: u32,
            cursor: String
        ) -> Result<PageBridge, ToriiClientError>;
    }
    
    // ============================================================================
    // SQL Operations
    // ============================================================================
    
    extern "Rust" {
        async fn sql(self: &ToriiClient, query: String) -> Result<String, ToriiClientError>;
    }
    
    // ============================================================================
    // Subscription Operations
    // ============================================================================
    
    extern "Rust" {
        async fn on_entity_updated(
            self: &ToriiClient,
            clause_json: String,
            world_addresses: Vec<FeltBridge>
        ) -> Result<Subscription, ToriiClientError>;
        
        async fn on_starknet_event(
            self: &ToriiClient,
            keys_json: String
        ) -> Result<Subscription, ToriiClientError>;
    }
}

// ============================================================================
// Implementation
// ============================================================================

pub struct ToriiClient {
    inner: Arc<torii_client::Client>,
}

pub struct ToriiClientError {
    message: String,
}

pub struct Subscription {
    id: u64,
    trigger: stream_cancel::Trigger,
}

impl ToriiClientError {
    pub fn message(&self) -> String {
        self.message.clone()
    }
}

impl Subscription {
    pub fn id(&self) -> u64 {
        self.id
    }
    
    pub fn cancel(self) {
        self.trigger.cancel();
    }
}

pub async fn new_torii_client(torii_url: String) -> Result<ToriiClient, ToriiClientError> {
    let client = torii_client::Client::new(torii_url)
        .await
        .map_err(|e| ToriiClientError { message: e.to_string() })?;
    
    Ok(ToriiClient {
        inner: Arc::new(client),
    })
}

pub async fn new_torii_client_with_config(
    torii_url: String,
    max_message_size: usize,
) -> Result<ToriiClient, ToriiClientError> {
    let client = torii_client::Client::new_with_config(torii_url, max_message_size)
        .await
        .map_err(|e| ToriiClientError { message: e.to_string() })?;
    
    Ok(ToriiClient {
        inner: Arc::new(client),
    })
}

// Helper functions for Felt conversion
fn felt_bridge_to_felt(bridge: &ffi::FeltBridge) -> Result<Felt, ToriiClientError> {
    Felt::from_hex(&bridge.hex)
        .map_err(|e| ToriiClientError { message: format!("Invalid Felt hex: {}", e) })
}

fn felt_to_bridge(felt: Felt) -> ffi::FeltBridge {
    ffi::FeltBridge {
        hex: format!("{:#x}", felt),
    }
}

fn create_pagination(limit: u32, cursor: &str) -> torii_proto::Pagination {
    torii_proto::Pagination {
        cursor: if cursor.is_empty() { None } else { Some(cursor.to_string()) },
        limit: if limit == 0 { None } else { Some(limit) },
        direction: torii_proto::PaginationDirection::Forward,
        order_by: vec![],
    }
}

impl ToriiClient {
    pub async fn worlds(&self, world_addresses: Vec<ffi::FeltBridge>) -> Result<Vec<ffi::WorldBridge>, ToriiClientError> {
        let addresses: Result<Vec<Felt>, _> = world_addresses.iter()
            .map(felt_bridge_to_felt)
            .collect();
        let addresses = addresses?;
        
        let worlds = self.inner.worlds(addresses)
            .await
            .map_err(|e| ToriiClientError { message: e.to_string() })?;
        
        Ok(worlds.into_iter().map(|w| ffi::WorldBridge {
            world_address: felt_to_bridge(w.world_address),
            models_json: serde_json::to_string(&w.models).unwrap_or_default(),
        }).collect())
    }
    
    pub async fn publish_message(&self, message_json: String) -> Result<String, ToriiClientError> {
        let message: torii_proto::Message = serde_json::from_str(&message_json)
            .map_err(|e| ToriiClientError { message: format!("Failed to parse message: {}", e) })?;
        
        self.inner.publish_message(message)
            .await
            .map_err(|e| ToriiClientError { message: e.to_string() })
    }
    
    pub async fn publish_message_batch(&self, messages_json: String) -> Result<String, ToriiClientError> {
        let messages: Vec<torii_proto::Message> = serde_json::from_str(&messages_json)
            .map_err(|e| ToriiClientError { message: format!("Failed to parse messages: {}", e) })?;
        
        let ids = self.inner.publish_message_batch(messages)
            .await
            .map_err(|e| ToriiClientError { message: e.to_string() })?;
        
        serde_json::to_string(&ids)
            .map_err(|e| ToriiClientError { message: format!("Serialization error: {}", e) })
    }
    
    pub async fn entities(&self, query: ffi::QueryBridge) -> Result<ffi::PageBridge, ToriiClientError> {
        let world_addresses: Result<Vec<Felt>, _> = query.world_addresses.iter()
            .map(felt_bridge_to_felt)
            .collect();
        let world_addresses = world_addresses?;
        
        let q = torii_proto::Query {
            clause: None,
            pagination: create_pagination(query.limit, &query.cursor),
            no_hashed_keys: query.dont_include_hashed_keys,
            models: vec![],
            historical: false,
            world_addresses,
        };
        
        let page = self.inner.entities(q)
            .await
            .map_err(|e| ToriiClientError { message: e.to_string() })?;
        
        let json = serde_json::to_string(&page)
            .map_err(|e| ToriiClientError { message: format!("Serialization error: {}", e) })?;
        
        Ok(ffi::PageBridge { data: json })
    }
    
    pub async fn event_messages(&self, query: ffi::QueryBridge) -> Result<ffi::PageBridge, ToriiClientError> {
        let world_addresses: Result<Vec<Felt>, _> = query.world_addresses.iter()
            .map(felt_bridge_to_felt)
            .collect();
        let world_addresses = world_addresses?;
        
        let q = torii_proto::Query {
            clause: None,
            pagination: create_pagination(query.limit, &query.cursor),
            no_hashed_keys: query.dont_include_hashed_keys,
            models: vec![],
            historical: false,
            world_addresses,
        };
        
        let page = self.inner.event_messages(q)
            .await
            .map_err(|e| ToriiClientError { message: e.to_string() })?;
        
        let json = serde_json::to_string(&page)
            .map_err(|e| ToriiClientError { message: format!("Serialization error: {}", e) })?;
        
        Ok(ffi::PageBridge { data: json })
    }
    
    pub async fn starknet_events(
        &self,
        keys_json: String,
        limit: u32,
        cursor: String
    ) -> Result<ffi::PageBridge, ToriiClientError> {
        let keys = if keys_json.is_empty() {
            None
        } else {
            Some(serde_json::from_str(&keys_json)
                .map_err(|e| ToriiClientError { message: format!("Failed to parse keys: {}", e) })?)
        };
        
        let q = torii_proto::EventQuery {
            keys,
            pagination: create_pagination(limit, &cursor),
        };
        
        let page = self.inner.starknet_events(q)
            .await
            .map_err(|e| ToriiClientError { message: e.to_string() })?;
        
        let json = serde_json::to_string(&page)
            .map_err(|e| ToriiClientError { message: format!("Serialization error: {}", e) })?;
        
        Ok(ffi::PageBridge { data: json })
    }
    
    pub async fn tokens(
        &self,
        contract_addresses: Vec<ffi::FeltBridge>,
        limit: u32,
        cursor: String
    ) -> Result<ffi::PageBridge, ToriiClientError> {
        let addresses: Result<Vec<Felt>, _> = contract_addresses.iter()
            .map(felt_bridge_to_felt)
            .collect();
        let addresses = addresses?;
        
        let q = torii_proto::TokenQuery {
            contract_addresses: addresses,
            token_ids: vec![],
            attribute_filters: vec![],
            pagination: create_pagination(limit, &cursor),
        };
        
        let page = self.inner.tokens(q)
            .await
            .map_err(|e| ToriiClientError { message: e.to_string() })?;
        
        let json = serde_json::to_string(&page)
            .map_err(|e| ToriiClientError { message: format!("Serialization error: {}", e) })?;
        
        Ok(ffi::PageBridge { data: json })
    }
    
    pub async fn token_balances(
        &self,
        account_addresses: Vec<ffi::FeltBridge>,
        contract_addresses: Vec<ffi::FeltBridge>,
        limit: u32,
        cursor: String
    ) -> Result<ffi::PageBridge, ToriiClientError> {
        let accounts: Result<Vec<Felt>, _> = account_addresses.iter()
            .map(felt_bridge_to_felt)
            .collect();
        let accounts = accounts?;
        
        let contracts: Result<Vec<Felt>, _> = contract_addresses.iter()
            .map(felt_bridge_to_felt)
            .collect();
        let contracts = contracts?;
        
        let q = torii_proto::TokenBalanceQuery {
            account_addresses: accounts,
            contract_addresses: contracts,
            token_ids: vec![],
            pagination: create_pagination(limit, &cursor),
        };
        
        let page = self.inner.token_balances(q)
            .await
            .map_err(|e| ToriiClientError { message: e.to_string() })?;
        
        let json = serde_json::to_string(&page)
            .map_err(|e| ToriiClientError { message: format!("Serialization error: {}", e) })?;
        
        Ok(ffi::PageBridge { data: json })
    }
    
    pub async fn token_contracts(
        &self,
        contract_addresses: Vec<ffi::FeltBridge>,
        limit: u32,
        cursor: String
    ) -> Result<ffi::PageBridge, ToriiClientError> {
        let addresses: Result<Vec<Felt>, _> = contract_addresses.iter()
            .map(felt_bridge_to_felt)
            .collect();
        let addresses = addresses?;
        
        let q = torii_proto::TokenContractQuery {
            contract_addresses: addresses,
            contract_types: vec![],
            pagination: create_pagination(limit, &cursor),
        };
        
        let page = self.inner.token_contracts(q)
            .await
            .map_err(|e| ToriiClientError { message: e.to_string() })?;
        
        let json = serde_json::to_string(&page)
            .map_err(|e| ToriiClientError { message: format!("Serialization error: {}", e) })?;
        
        Ok(ffi::PageBridge { data: json })
    }
    
    pub async fn token_transfers(
        &self,
        account_addresses: Vec<ffi::FeltBridge>,
        contract_addresses: Vec<ffi::FeltBridge>,
        limit: u32,
        cursor: String
    ) -> Result<ffi::PageBridge, ToriiClientError> {
        let accounts: Result<Vec<Felt>, _> = account_addresses.iter()
            .map(felt_bridge_to_felt)
            .collect();
        let accounts = accounts?;
        
        let contracts: Result<Vec<Felt>, _> = contract_addresses.iter()
            .map(felt_bridge_to_felt)
            .collect();
        let contracts = contracts?;
        
        let q = torii_proto::TokenTransferQuery {
            account_addresses: accounts,
            contract_addresses: contracts,
            token_ids: vec![],
            pagination: create_pagination(limit, &cursor),
        };
        
        let page = self.inner.token_transfers(q)
            .await
            .map_err(|e| ToriiClientError { message: e.to_string() })?;
        
        let json = serde_json::to_string(&page)
            .map_err(|e| ToriiClientError { message: format!("Serialization error: {}", e) })?;
        
        Ok(ffi::PageBridge { data: json })
    }
    
    pub async fn transactions(
        &self,
        filter_json: String,
        limit: u32,
        cursor: String
    ) -> Result<ffi::PageBridge, ToriiClientError> {
        let filter = if filter_json.is_empty() {
            None
        } else {
            Some(serde_json::from_str(&filter_json)
                .map_err(|e| ToriiClientError { message: format!("Failed to parse filter: {}", e) })?)
        };
        
        let q = torii_proto::TransactionQuery {
            filter,
            pagination: create_pagination(limit, &cursor),
        };
        
        let page = self.inner.transactions(q)
            .await
            .map_err(|e| ToriiClientError { message: e.to_string() })?;
        
        let json = serde_json::to_string(&page)
            .map_err(|e| ToriiClientError { message: format!("Serialization error: {}", e) })?;
        
        Ok(ffi::PageBridge { data: json })
    }
    
    pub async fn contracts(
        &self,
        contract_addresses: Vec<ffi::FeltBridge>
    ) -> Result<String, ToriiClientError> {
        let addresses: Result<Vec<Felt>, _> = contract_addresses.iter()
            .map(felt_bridge_to_felt)
            .collect();
        let addresses = addresses?;
        
        let q = torii_proto::ContractQuery {
            contract_addresses: addresses,
            contract_types: vec![],
        };
        
        let contracts = self.inner.contracts(q)
            .await
            .map_err(|e| ToriiClientError { message: e.to_string() })?;
        
        serde_json::to_string(&contracts)
            .map_err(|e| ToriiClientError { message: format!("Serialization error: {}", e) })
    }
    
    pub async fn controllers(
        &self,
        contract_addresses: Vec<ffi::FeltBridge>,
        limit: u32,
        cursor: String
    ) -> Result<ffi::PageBridge, ToriiClientError> {
        let addresses: Result<Vec<Felt>, _> = contract_addresses.iter()
            .map(felt_bridge_to_felt)
            .collect();
        let addresses = addresses?;
        
        let q = torii_proto::ControllerQuery {
            contract_addresses: addresses,
            usernames: vec![],
            pagination: create_pagination(limit, &cursor),
        };
        
        let page = self.inner.controllers(q)
            .await
            .map_err(|e| ToriiClientError { message: e.to_string() })?;
        
        let json = serde_json::to_string(&page)
            .map_err(|e| ToriiClientError { message: format!("Serialization error: {}", e) })?;
        
        Ok(ffi::PageBridge { data: json })
    }
    
    pub async fn aggregations(
        &self,
        aggregator_ids: Vec<String>,
        entity_ids: Vec<String>,
        limit: u32,
        cursor: String
    ) -> Result<ffi::PageBridge, ToriiClientError> {
        let q = torii_proto::AggregationQuery {
            aggregator_ids,
            entity_ids,
            pagination: create_pagination(limit, &cursor),
        };
        
        let page = self.inner.aggregations(q)
            .await
            .map_err(|e| ToriiClientError { message: e.to_string() })?;
        
        let json = serde_json::to_string(&page)
            .map_err(|e| ToriiClientError { message: format!("Serialization error: {}", e) })?;
        
        Ok(ffi::PageBridge { data: json })
    }
    
    pub async fn activities(
        &self,
        world_addresses: Vec<ffi::FeltBridge>,
        namespaces: Vec<String>,
        caller_addresses: Vec<ffi::FeltBridge>,
        limit: u32,
        cursor: String
    ) -> Result<ffi::PageBridge, ToriiClientError> {
        let worlds: Result<Vec<Felt>, _> = world_addresses.iter()
            .map(felt_bridge_to_felt)
            .collect();
        let worlds = worlds?;
        
        let callers: Result<Vec<Felt>, _> = caller_addresses.iter()
            .map(felt_bridge_to_felt)
            .collect();
        let callers = callers?;
        
        let q = torii_proto::ActivityQuery {
            world_addresses: worlds,
            namespaces,
            caller_addresses: callers,
            from_time: None,
            to_time: None,
            pagination: create_pagination(limit, &cursor),
        };
        
        let page = self.inner.activities(q)
            .await
            .map_err(|e| ToriiClientError { message: e.to_string() })?;
        
        let json = serde_json::to_string(&page)
            .map_err(|e| ToriiClientError { message: format!("Serialization error: {}", e) })?;
        
        Ok(ffi::PageBridge { data: json })
    }
    
    pub async fn achievements(
        &self,
        world_addresses: Vec<ffi::FeltBridge>,
        namespaces: Vec<String>,
        limit: u32,
        cursor: String
    ) -> Result<ffi::PageBridge, ToriiClientError> {
        let worlds: Result<Vec<Felt>, _> = world_addresses.iter()
            .map(felt_bridge_to_felt)
            .collect();
        let worlds = worlds?;
        
        let q = torii_proto::AchievementQuery {
            world_addresses: worlds,
            namespaces,
            hidden: None,
            pagination: create_pagination(limit, &cursor),
        };
        
        let page = self.inner.achievements(q)
            .await
            .map_err(|e| ToriiClientError { message: e.to_string() })?;
        
        let json = serde_json::to_string(&page)
            .map_err(|e| ToriiClientError { message: format!("Serialization error: {}", e) })?;
        
        Ok(ffi::PageBridge { data: json })
    }
    
    pub async fn player_achievements(
        &self,
        world_addresses: Vec<ffi::FeltBridge>,
        namespaces: Vec<String>,
        player_addresses: Vec<ffi::FeltBridge>,
        limit: u32,
        cursor: String
    ) -> Result<ffi::PageBridge, ToriiClientError> {
        let worlds: Result<Vec<Felt>, _> = world_addresses.iter()
            .map(felt_bridge_to_felt)
            .collect();
        let worlds = worlds?;
        
        let players: Result<Vec<Felt>, _> = player_addresses.iter()
            .map(felt_bridge_to_felt)
            .collect();
        let players = players?;
        
        let q = torii_proto::PlayerAchievementQuery {
            world_addresses: worlds,
            namespaces,
            player_addresses: players,
            pagination: create_pagination(limit, &cursor),
        };
        
        let page = self.inner.player_achievements(q)
            .await
            .map_err(|e| ToriiClientError { message: e.to_string() })?;
        
        let json = serde_json::to_string(&page)
            .map_err(|e| ToriiClientError { message: format!("Serialization error: {}", e) })?;
        
        Ok(ffi::PageBridge { data: json })
    }
    
    pub async fn sql(&self, query: String) -> Result<String, ToriiClientError> {
        let rows = self.inner.sql(query)
            .await
            .map_err(|e| ToriiClientError { message: e.to_string() })?;
        
        serde_json::to_string(&rows)
            .map_err(|e| ToriiClientError { message: format!("Serialization error: {}", e) })
    }
    
    pub async fn on_entity_updated(
        &self,
        clause_json: String,
        world_addresses: Vec<ffi::FeltBridge>
    ) -> Result<Subscription, ToriiClientError> {
        let clause = if clause_json.is_empty() {
            None
        } else {
            Some(serde_json::from_str(&clause_json)
                .map_err(|e| ToriiClientError { message: format!("Failed to parse clause: {}", e) })?)
        };
        
        let addresses: Result<Vec<Felt>, _> = world_addresses.iter()
            .map(felt_bridge_to_felt)
            .collect();
        let addresses = addresses?;
        
        let _stream = self.inner.on_entity_updated(clause, addresses)
            .await
            .map_err(|e| ToriiClientError { message: e.to_string() })?;
        
        let (trigger, _tripwire) = stream_cancel::Tripwire::new();
        Ok(Subscription {
            id: 0,
            trigger,
        })
    }
    
    pub async fn on_starknet_event(
        &self,
        keys_json: String
    ) -> Result<Subscription, ToriiClientError> {
        let keys: Vec<torii_proto::KeysClause> = if keys_json.is_empty() {
            vec![]
        } else {
            serde_json::from_str(&keys_json)
                .map_err(|e| ToriiClientError { message: format!("Failed to parse keys: {}", e) })?
        };
        
        let _stream = self.inner.on_starknet_event(keys)
            .await
            .map_err(|e| ToriiClientError { message: e.to_string() })?;
        
        let (trigger, _tripwire) = stream_cancel::Tripwire::new();
        Ok(Subscription {
            id: 0,
            trigger,
        })
    }
}

// ============================================================================
// Conversion Functions - Bridge Types to Torii Proto Types
// ============================================================================

// U256 conversions
fn u256_to_bridge(value: crypto_bigint::U256) -> ffi::U256Bridge {
    ffi::U256Bridge {
        hex: format!("0x{}", hex::encode(value.to_be_bytes())),
    }
}

fn u256_bridge_to_u256(bridge: &ffi::U256Bridge) -> Result<crypto_bigint::U256, ToriiClientError> {
    let hex_str = bridge.hex.trim_start_matches("0x");
    let bytes = hex::decode(hex_str)
        .map_err(|e| ToriiClientError { message: format!("Invalid U256 hex: {}", e) })?;
    
    Ok(crypto_bigint::U256::from_be_slice(&bytes))
}

// Entity conversions
impl From<torii_proto::schema::Entity> for ffi::EntityBridge {
    fn from(entity: torii_proto::schema::Entity) -> Self {
        let models_json = serde_json::to_string(&entity.models).unwrap_or_default();
        
        ffi::EntityBridge {
            world_address: felt_to_bridge(entity.world_address),
            hashed_keys: felt_to_bridge(entity.hashed_keys),
            models_json,
            created_at: entity.created_at.timestamp() as u64,
            updated_at: entity.updated_at.timestamp() as u64,
            executed_at: entity.executed_at.timestamp() as u64,
        }
    }
}

// Model conversions
impl From<torii_proto::Model> for ffi::ModelBridge {
    fn from(model: torii_proto::Model) -> Self {
        let layout_json = serde_json::to_string(&model.layout).unwrap_or_default();
        let schema_json = serde_json::to_string(&model.schema).unwrap_or_default();
        
        ffi::ModelBridge {
            world_address: felt_to_bridge(model.world_address),
            namespace: model.namespace,
            name: model.name,
            selector: felt_to_bridge(model.selector),
            packed_size: model.packed_size,
            unpacked_size: model.unpacked_size,
            class_hash: felt_to_bridge(model.class_hash),
            contract_address: felt_to_bridge(model.contract_address),
            layout_json,
            schema_json,
            use_legacy_store: model.use_legacy_store,
        }
    }
}

// Token conversions
impl From<torii_proto::Token> for ffi::TokenBridge {
    fn from(token: torii_proto::Token) -> Self {
        let token_id = token.token_id.map(|id| serde_json::to_string(&id).unwrap_or_default()).unwrap_or_default();
        let total_supply = token.total_supply.map(|ts| serde_json::to_string(&ts).unwrap_or_default()).unwrap_or_default();
        
        ffi::TokenBridge {
            contract_address: felt_to_bridge(token.contract_address),
            token_id,
            name: token.name,
            symbol: token.symbol,
            decimals: token.decimals,
            metadata: token.metadata,
            total_supply,
        }
    }
}

impl From<torii_proto::TokenBalance> for ffi::TokenBalanceBridge {
    fn from(balance: torii_proto::TokenBalance) -> Self {
        let token_id = balance.token_id.map(|id| serde_json::to_string(&id).unwrap_or_default()).unwrap_or_default();
        
        ffi::TokenBalanceBridge {
            balance: u256_to_bridge(balance.balance),
            account_address: felt_to_bridge(balance.account_address),
            contract_address: felt_to_bridge(balance.contract_address),
            token_id,
        }
    }
}

impl From<torii_proto::TokenContract> for ffi::TokenContractBridge {
    fn from(contract: torii_proto::TokenContract) -> Self {
        let total_supply = contract.total_supply.map(|ts| serde_json::to_string(&ts).unwrap_or_default()).unwrap_or_default();
        
        ffi::TokenContractBridge {
            contract_address: felt_to_bridge(contract.contract_address),
            name: contract.name,
            symbol: contract.symbol,
            decimals: contract.decimals,
            metadata: contract.metadata,
            token_metadata: contract.token_metadata,
            total_supply,
        }
    }
}

impl From<torii_proto::TokenTransfer> for ffi::TokenTransferBridge {
    fn from(transfer: torii_proto::TokenTransfer) -> Self {
        let token_id = transfer.token_id.map(|id| serde_json::to_string(&id).unwrap_or_default()).unwrap_or_default();
        let event_id = transfer.event_id.unwrap_or_default();
        
        ffi::TokenTransferBridge {
            id: transfer.id,
            contract_address: felt_to_bridge(transfer.contract_address),
            from_address: felt_to_bridge(transfer.from_address),
            to_address: felt_to_bridge(transfer.to_address),
            amount: u256_to_bridge(transfer.amount),
            token_id,
            executed_at: transfer.executed_at.timestamp() as u64,
            event_id,
        }
    }
}

// Controller conversions
impl From<torii_proto::Controller> for ffi::ControllerBridge {
    fn from(controller: torii_proto::Controller) -> Self {
        ffi::ControllerBridge {
            address: felt_to_bridge(controller.address),
            username: controller.username,
            deployed_at_timestamp: controller.deployed_at.timestamp() as u64,
        }
    }
}

// Contract conversions
impl From<torii_proto::Contract> for ffi::ContractBridge {
    fn from(contract: torii_proto::Contract) -> Self {
        let head = contract.head.map(|h| h.to_string()).unwrap_or_default();
        let tps = contract.tps.map(|t| t.to_string()).unwrap_or_default();
        let last_block_timestamp = contract.last_block_timestamp.map(|ts| ts.to_string()).unwrap_or_default();
        let last_pending_block_tx = contract.last_pending_block_tx.map(|tx| format!("{:#x}", tx)).unwrap_or_default();
        
        let contract_type = match contract.contract_type {
            torii_proto::ContractType::WORLD => "WORLD",
            torii_proto::ContractType::ERC20 => "ERC20",
            torii_proto::ContractType::ERC721 => "ERC721",
            torii_proto::ContractType::ERC1155 => "ERC1155",
            torii_proto::ContractType::UDC => "UDC",
            torii_proto::ContractType::OTHER => "OTHER",
        }.to_string();
        
        ffi::ContractBridge {
            contract_address: felt_to_bridge(contract.contract_address),
            contract_type,
            head,
            tps,
            last_block_timestamp,
            last_pending_block_tx,
            updated_at: contract.updated_at.timestamp() as u64,
            created_at: contract.created_at.timestamp() as u64,
        }
    }
}

// Transaction conversions
impl From<torii_proto::Transaction> for ffi::TransactionBridge {
    fn from(tx: torii_proto::Transaction) -> Self {
        let calls_json = serde_json::to_string(&tx.calls).unwrap_or_default();
        
        ffi::TransactionBridge {
            transaction_hash: felt_to_bridge(tx.transaction_hash),
            sender_address: felt_to_bridge(tx.sender_address),
            calldata: tx.calldata.into_iter().map(felt_to_bridge).collect(),
            max_fee: felt_to_bridge(tx.max_fee),
            signature: tx.signature.into_iter().map(felt_to_bridge).collect(),
            nonce: felt_to_bridge(tx.nonce),
            block_number: tx.block_number,
            transaction_type: tx.transaction_type,
            block_timestamp: tx.block_timestamp.timestamp() as u64,
            calls_json,
            unique_models: tx.unique_models.into_iter().map(felt_to_bridge).collect(),
        }
    }
}

impl From<torii_proto::TransactionCall> for ffi::TransactionCallBridge {
    fn from(call: torii_proto::TransactionCall) -> Self {
        let call_type = match call.call_type {
            torii_proto::CallType::Execute => "Execute",
            torii_proto::CallType::ExecuteFromOutside => "ExecuteFromOutside",
        }.to_string();
        
        ffi::TransactionCallBridge {
            contract_address: felt_to_bridge(call.contract_address),
            entrypoint: call.entrypoint,
            calldata: call.calldata.into_iter().map(felt_to_bridge).collect(),
            call_type,
            caller_address: felt_to_bridge(call.caller_address),
        }
    }
}

// Event conversions
impl From<torii_proto::Event> for ffi::EventBridge {
    fn from(event: torii_proto::Event) -> Self {
        ffi::EventBridge {
            keys: event.keys.into_iter().map(felt_to_bridge).collect(),
            data: event.data.into_iter().map(felt_to_bridge).collect(),
            transaction_hash: felt_to_bridge(event.transaction_hash),
        }
    }
}

// Aggregation conversions
impl From<torii_proto::AggregationEntry> for ffi::AggregationEntryBridge {
    fn from(entry: torii_proto::AggregationEntry) -> Self {
        ffi::AggregationEntryBridge {
            id: entry.id,
            aggregator_id: entry.aggregator_id,
            entity_id: entry.entity_id,
            value: u256_to_bridge(entry.value),
            display_value: entry.display_value,
            position: entry.position,
            model_id: entry.model_id,
            created_at: entry.created_at.timestamp() as u64,
            updated_at: entry.updated_at.timestamp() as u64,
        }
    }
}

// Activity conversions
impl From<torii_proto::Activity> for ffi::ActivityBridge {
    fn from(activity: torii_proto::Activity) -> Self {
        let actions_json = serde_json::to_string(&activity.actions).unwrap_or_default();
        
        ffi::ActivityBridge {
            id: activity.id,
            world_address: felt_to_bridge(activity.world_address),
            namespace: activity.namespace,
            caller_address: felt_to_bridge(activity.caller_address),
            session_start: activity.session_start.timestamp() as u64,
            session_end: activity.session_end.timestamp() as u64,
            action_count: activity.action_count,
            actions_json,
            updated_at: activity.updated_at.timestamp() as u64,
        }
    }
}

// Achievement conversions
impl From<torii_proto::Achievement> for ffi::AchievementBridge {
    fn from(achievement: torii_proto::Achievement) -> Self {
        let tasks_json = serde_json::to_string(&achievement.tasks).unwrap_or_default();
        
        ffi::AchievementBridge {
            id: achievement.id,
            world_address: felt_to_bridge(achievement.world_address),
            namespace: achievement.namespace,
            entity_id: achievement.entity_id,
            hidden: achievement.hidden,
            index: achievement.index,
            points: achievement.points,
            start: achievement.start,
            end: achievement.end,
            group: achievement.group,
            icon: achievement.icon,
            title: achievement.title,
            description: achievement.description,
            tasks_json,
            data: achievement.data.unwrap_or_default(),
            total_completions: achievement.total_completions,
            completion_rate: achievement.completion_rate,
            created_at: achievement.created_at.timestamp() as u64,
            updated_at: achievement.updated_at.timestamp() as u64,
        }
    }
}

impl From<torii_proto::AchievementTask> for ffi::AchievementTaskBridge {
    fn from(task: torii_proto::AchievementTask) -> Self {
        ffi::AchievementTaskBridge {
            task_id: task.task_id,
            description: task.description,
            total: task.total,
            total_completions: task.total_completions,
            completion_rate: task.completion_rate,
            created_at: task.created_at.timestamp() as u64,
        }
    }
}

impl From<torii_proto::PlayerAchievementEntry> for ffi::PlayerAchievementEntryBridge {
    fn from(entry: torii_proto::PlayerAchievementEntry) -> Self {
        let achievements_json = serde_json::to_string(&entry.achievements).unwrap_or_default();
        
        ffi::PlayerAchievementEntryBridge {
            player_address: felt_to_bridge(entry.player_address),
            stats: entry.stats.into(),
            achievements_json,
        }
    }
}

impl From<torii_proto::PlayerAchievementStats> for ffi::PlayerAchievementStatsBridge {
    fn from(stats: torii_proto::PlayerAchievementStats) -> Self {
        let last_achievement_at = stats.last_achievement_at
            .map(|t| t.timestamp().to_string())
            .unwrap_or_default();
        
        ffi::PlayerAchievementStatsBridge {
            total_points: stats.total_points,
            completed_achievements: stats.completed_achievements,
            total_achievements: stats.total_achievements,
            completion_percentage: stats.completion_percentage,
            last_achievement_at,
            created_at: stats.created_at.timestamp() as u64,
            updated_at: stats.updated_at.timestamp() as u64,
        }
    }
}

impl From<torii_proto::PlayerAchievementProgress> for ffi::PlayerAchievementProgressBridge {
    fn from(progress: torii_proto::PlayerAchievementProgress) -> Self {
        let task_progress_json = serde_json::to_string(&progress.task_progress).unwrap_or_default();
        
        ffi::PlayerAchievementProgressBridge {
            achievement: progress.achievement.into(),
            task_progress_json,
            completed: progress.completed,
            progress_percentage: progress.progress_percentage,
        }
    }
}

impl From<torii_proto::TaskProgress> for ffi::TaskProgressBridge {
    fn from(progress: torii_proto::TaskProgress) -> Self {
        ffi::TaskProgressBridge {
            task_id: progress.task_id,
            count: progress.count,
            completed: progress.completed,
        }
    }
}
