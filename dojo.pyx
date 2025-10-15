from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  cdef enum BlockTag:
    Latest,
    PreConfirmed,

  cdef enum CallType:
    Execute,
    ExecuteFromOutside,

  cdef enum ComparisonOperator:
    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
    In,
    NotIn,
    Contains,
    ContainsAll,
    ContainsAny,
    ArrayLengthEq,
    ArrayLengthGt,
    ArrayLengthLt,

  cdef enum ContractType:
    WORLD,
    ERC20,
    ERC721,
    ERC1155,
    UDC,
    OTHER,

  cdef enum LogicalOperator:
    And,
    Or,

  cdef enum OrderDirection:
    Asc,
    Desc,

  cdef enum PaginationDirection:
    Forward,
    Backward,

  cdef enum PatternMatching:
    FixedLen # = 0,
    VariableLen # = 1,

  cdef struct Account:
    pass

  cdef struct Provider:
    pass

  cdef struct Subscription:
    pass

  cdef struct ToriiClient:
    pass

  cdef struct Error:
    char *message;

  cdef enum ResultToriiClient_Tag:
    OkToriiClient,
    ErrToriiClient,

  cdef struct ResultToriiClient:
    ResultToriiClient_Tag tag;
    ToriiClient *ok;
    Error err;

  cdef enum Resultc_char_Tag:
    Okc_char,
    Errc_char,

  cdef struct Resultc_char:
    Resultc_char_Tag tag;
    const char *ok;
    Error err;

  cdef struct CArrayFieldElement:
    FieldElement *data;
    uintptr_t data_len;

  cdef struct FieldElement:
    uint8_t data[32];

  cdef struct Message:
    const char *message;
    CArrayFieldElement signature;
    FieldElement world_address;

  cdef struct CArrayc_char:
    const char **data;
    uintptr_t data_len;

  cdef enum ResultCArrayc_char_Tag:
    OkCArrayc_char,
    ErrCArrayc_char,

  cdef struct ResultCArrayc_char:
    ResultCArrayc_char_Tag tag;
    CArrayc_char ok;
    Error err;

  cdef struct CArrayController:
    Controller *data;
    uintptr_t data_len;

  cdef enum COptionc_char_Tag:
    Somec_char,
    Nonec_char,

  cdef struct COptionc_char:
    COptionc_char_Tag tag;
    const char *some;

  cdef struct PageController:
    CArrayController items;
    COptionc_char next_cursor;

  cdef enum ResultPageController_Tag:
    OkPageController,
    ErrPageController,

  cdef struct ResultPageController:
    ResultPageController_Tag tag;
    PageController ok;
    Error err;

  cdef enum COptionu32_Tag:
    Someu32,
    Noneu32,

  cdef struct COptionu32:
    COptionu32_Tag tag;
    uint32_t some;

  cdef struct CArrayOrderBy:
    OrderBy *data;
    uintptr_t data_len;

  cdef struct Pagination:
    COptionc_char cursor;
    COptionu32 limit;
    PaginationDirection direction;
    CArrayOrderBy order_by;

  cdef struct ControllerQuery:
    Pagination pagination;
    CArrayFieldElement contract_addresses;
    CArrayc_char usernames;

  cdef struct CArrayEntity:
    Entity *data;
    uintptr_t data_len;

  cdef struct PageEntity:
    CArrayEntity items;
    COptionc_char next_cursor;

  cdef enum ResultPageEntity_Tag:
    OkPageEntity,
    ErrPageEntity,

  cdef struct ResultPageEntity:
    ResultPageEntity_Tag tag;
    PageEntity ok;
    Error err;

  cdef struct CArrayCOptionFieldElement:
    COptionFieldElement *data;
    uintptr_t data_len;

  cdef struct KeysClause:
    CArrayCOptionFieldElement keys;
    PatternMatching pattern_matching;
    CArrayc_char models;

  cdef struct U256:
    uint8_t data[32];

  cdef enum Primitive_Tag:
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
    U256_,
    Bool,
    Felt252,
    ClassHash,
    ContractAddress,
    EthAddress,

  cdef struct Primitive:
    Primitive_Tag tag;
    int8_t i8;
    int16_t i16;
    int32_t i32;
    int64_t i64;
    uint8_t i128[16];
    uint8_t u8;
    uint16_t u16;
    uint32_t u32;
    uint64_t u64;
    uint8_t u128[16];
    U256 u256;
    bool bool_;
    FieldElement felt252;
    FieldElement class_hash;
    FieldElement contract_address;
    FieldElement eth_address;

  cdef struct CArrayMemberValue:
    MemberValue *data;
    uintptr_t data_len;

  cdef enum MemberValue_Tag:
    PrimitiveValue,
    String,
    List,

  cdef struct MemberValue:
    MemberValue_Tag tag;
    Primitive primitive_value;
    const char *string;
    CArrayMemberValue list;

  cdef struct MemberClause:
    const char *model;
    const char *member;
    ComparisonOperator operator_;
    MemberValue value;

  cdef struct CArrayClause:
    Clause *data;
    uintptr_t data_len;

  cdef struct CompositeClause:
    LogicalOperator operator_;
    CArrayClause clauses;

  cdef enum Clause_Tag:
    HashedKeys,
    Keys,
    CMember,
    Composite,

  cdef struct Clause:
    Clause_Tag tag;
    CArrayFieldElement hashed_keys;
    KeysClause keys;
    MemberClause c_member;
    CompositeClause composite;

  cdef enum COptionClause_Tag:
    SomeClause,
    NoneClause,

  cdef struct COptionClause:
    COptionClause_Tag tag;
    Clause some;

  cdef struct Query:
    CArrayFieldElement world_addresses;
    Pagination pagination;
    COptionClause clause;
    bool no_hashed_keys;
    CArrayc_char models;
    bool historical;

  cdef struct CArrayWorld:
    World *data;
    uintptr_t data_len;

  cdef enum ResultCArrayWorld_Tag:
    OkCArrayWorld,
    ErrCArrayWorld,

  cdef struct ResultCArrayWorld:
    ResultCArrayWorld_Tag tag;
    CArrayWorld ok;
    Error err;

  cdef struct CArrayTransaction:
    Transaction *data;
    uintptr_t data_len;

  cdef struct PageTransaction:
    CArrayTransaction items;
    COptionc_char next_cursor;

  cdef enum ResultPageTransaction_Tag:
    OkPageTransaction,
    ErrPageTransaction,

  cdef struct ResultPageTransaction:
    ResultPageTransaction_Tag tag;
    PageTransaction ok;
    Error err;

  cdef enum COptionu64_Tag:
    Someu64,
    Noneu64,

  cdef struct COptionu64:
    COptionu64_Tag tag;
    uint64_t some;

  cdef struct TransactionFilter:
    CArrayFieldElement transaction_hashes;
    CArrayFieldElement caller_addresses;
    CArrayFieldElement contract_addresses;
    CArrayc_char entrypoints;
    CArrayFieldElement model_selectors;
    COptionu64 from_block;
    COptionu64 to_block;

  cdef enum COptionTransactionFilter_Tag:
    SomeTransactionFilter,
    NoneTransactionFilter,

  cdef struct COptionTransactionFilter:
    COptionTransactionFilter_Tag tag;
    TransactionFilter some;

  cdef struct TransactionQuery:
    COptionTransactionFilter filter;
    Pagination pagination;

  cdef enum ResultSubscription_Tag:
    OkSubscription,
    ErrSubscription,

  cdef struct ResultSubscription:
    ResultSubscription_Tag tag;
    Subscription *ok;
    Error err;

  cdef struct CArrayTransactionCall:
    TransactionCall *data;
    uintptr_t data_len;

  cdef struct Transaction:
    FieldElement transaction_hash;
    FieldElement sender_address;
    CArrayFieldElement calldata;
    FieldElement max_fee;
    CArrayFieldElement signature;
    FieldElement nonce;
    uint64_t block_number;
    const char *transaction_type;
    uint64_t block_timestamp;
    CArrayTransactionCall calls;
    CArrayFieldElement unique_models;

  cdef struct CArrayStruct:
    Struct *data;
    uintptr_t data_len;

  cdef struct Entity:
    FieldElement world_address;
    FieldElement hashed_keys;
    CArrayStruct models;
    uint64_t created_at;
    uint64_t updated_at;
    uint64_t executed_at;

  cdef enum Resultbool_Tag:
    Okbool,
    Errbool,

  cdef struct Resultbool:
    Resultbool_Tag tag;
    bool ok;
    Error err;

  cdef struct CArrayAggregationEntry:
    AggregationEntry *data;
    uintptr_t data_len;

  cdef struct PageAggregationEntry:
    CArrayAggregationEntry items;
    COptionc_char next_cursor;

  cdef enum ResultPageAggregationEntry_Tag:
    OkPageAggregationEntry,
    ErrPageAggregationEntry,

  cdef struct ResultPageAggregationEntry:
    ResultPageAggregationEntry_Tag tag;
    PageAggregationEntry ok;
    Error err;

  cdef struct AggregationQuery:
    CArrayc_char aggregator_ids;
    CArrayc_char entity_ids;
    Pagination pagination;

  cdef struct AggregationEntry:
    const char *id;
    const char *aggregator_id;
    const char *entity_id;
    U256 value;
    const char *display_value;
    uint64_t position;
    const char *model_id;
    uint64_t created_at;
    uint64_t updated_at;

  cdef struct CArrayAchievement:
    Achievement *data;
    uintptr_t data_len;

  cdef struct PageAchievement:
    CArrayAchievement items;
    COptionc_char next_cursor;

  cdef enum ResultPageAchievement_Tag:
    OkPageAchievement,
    ErrPageAchievement,

  cdef struct ResultPageAchievement:
    ResultPageAchievement_Tag tag;
    PageAchievement ok;
    Error err;

  cdef enum COptionbool_Tag:
    Somebool,
    Nonebool,

  cdef struct COptionbool:
    COptionbool_Tag tag;
    bool some;

  cdef struct AchievementQuery:
    CArrayFieldElement world_addresses;
    CArrayc_char namespaces;
    COptionbool hidden;
    Pagination pagination;

  cdef struct CArrayPlayerAchievementEntry:
    PlayerAchievementEntry *data;
    uintptr_t data_len;

  cdef struct PagePlayerAchievementEntry:
    CArrayPlayerAchievementEntry items;
    COptionc_char next_cursor;

  cdef enum ResultPagePlayerAchievementEntry_Tag:
    OkPagePlayerAchievementEntry,
    ErrPagePlayerAchievementEntry,

  cdef struct ResultPagePlayerAchievementEntry:
    ResultPagePlayerAchievementEntry_Tag tag;
    PagePlayerAchievementEntry ok;
    Error err;

  cdef struct PlayerAchievementQuery:
    CArrayFieldElement world_addresses;
    CArrayc_char namespaces;
    CArrayFieldElement player_addresses;
    Pagination pagination;

  cdef struct AchievementProgression:
    const char *id;
    const char *achievement_id;
    const char *task_id;
    FieldElement world_address;
    const char *namespace_;
    FieldElement player_id;
    uint32_t count;
    bool completed;
    COptionu64 completed_at;
    uint64_t created_at;
    uint64_t updated_at;

  cdef struct CArrayActivity:
    Activity *data;
    uintptr_t data_len;

  cdef struct PageActivity:
    CArrayActivity items;
    COptionc_char next_cursor;

  cdef enum ResultPageActivity_Tag:
    OkPageActivity,
    ErrPageActivity,

  cdef struct ResultPageActivity:
    ResultPageActivity_Tag tag;
    PageActivity ok;
    Error err;

  cdef struct ActivityQuery:
    CArrayFieldElement world_addresses;
    CArrayc_char namespaces;
    CArrayFieldElement caller_addresses;
    COptionu64 from_time;
    COptionu64 to_time;
    Pagination pagination;

  cdef struct CArrayActionCount:
    ActionCount *data;
    uintptr_t data_len;

  cdef struct Activity:
    const char *id;
    FieldElement world_address;
    const char *namespace_;
    FieldElement caller_address;
    uint64_t session_start;
    uint64_t session_end;
    uint32_t action_count;
    CArrayActionCount actions;
    uint64_t updated_at;

  cdef struct Event:
    CArrayFieldElement keys;
    CArrayFieldElement data;
    FieldElement transaction_hash;

  cdef struct CArrayToken:
    Token *data;
    uintptr_t data_len;

  cdef struct PageToken:
    CArrayToken items;
    COptionc_char next_cursor;

  cdef enum ResultPageToken_Tag:
    OkPageToken,
    ErrPageToken,

  cdef struct ResultPageToken:
    ResultPageToken_Tag tag;
    PageToken ok;
    Error err;

  cdef struct CArrayU256:
    U256 *data;
    uintptr_t data_len;

  cdef struct CArrayAttributeFilter:
    AttributeFilter *data;
    uintptr_t data_len;

  cdef struct TokenQuery:
    CArrayFieldElement contract_addresses;
    CArrayU256 token_ids;
    CArrayAttributeFilter attribute_filters;
    Pagination pagination;

  cdef enum COptionU256_Tag:
    SomeU256,
    NoneU256,

  cdef struct COptionU256:
    COptionU256_Tag tag;
    U256 some;

  cdef struct Token:
    FieldElement contract_address;
    COptionU256 token_id;
    const char *name;
    const char *symbol;
    uint8_t decimals;
    const char *metadata;
    COptionU256 total_supply;

  cdef struct CArrayTokenBalance:
    TokenBalance *data;
    uintptr_t data_len;

  cdef struct PageTokenBalance:
    CArrayTokenBalance items;
    COptionc_char next_cursor;

  cdef enum ResultPageTokenBalance_Tag:
    OkPageTokenBalance,
    ErrPageTokenBalance,

  cdef struct ResultPageTokenBalance:
    ResultPageTokenBalance_Tag tag;
    PageTokenBalance ok;
    Error err;

  cdef struct TokenBalanceQuery:
    CArrayFieldElement contract_addresses;
    CArrayFieldElement account_addresses;
    CArrayU256 token_ids;
    Pagination pagination;

  cdef struct CArrayTokenContract:
    TokenContract *data;
    uintptr_t data_len;

  cdef struct PageTokenContract:
    CArrayTokenContract items;
    COptionc_char next_cursor;

  cdef enum ResultPageTokenContract_Tag:
    OkPageTokenContract,
    ErrPageTokenContract,

  cdef struct ResultPageTokenContract:
    ResultPageTokenContract_Tag tag;
    PageTokenContract ok;
    Error err;

  cdef struct CArrayContractType:
    ContractType *data;
    uintptr_t data_len;

  cdef struct TokenContractQuery:
    CArrayFieldElement contract_addresses;
    CArrayContractType contract_types;
    Pagination pagination;

  cdef struct CArrayContract:
    Contract *data;
    uintptr_t data_len;

  cdef enum ResultCArrayContract_Tag:
    OkCArrayContract,
    ErrCArrayContract,

  cdef struct ResultCArrayContract:
    ResultCArrayContract_Tag tag;
    CArrayContract ok;
    Error err;

  cdef struct ContractQuery:
    CArrayFieldElement contract_addresses;
    CArrayContractType contract_types;

  cdef struct CArrayTokenTransfer:
    TokenTransfer *data;
    uintptr_t data_len;

  cdef struct PageTokenTransfer:
    CArrayTokenTransfer items;
    COptionc_char next_cursor;

  cdef enum ResultPageTokenTransfer_Tag:
    OkPageTokenTransfer,
    ErrPageTokenTransfer,

  cdef struct ResultPageTokenTransfer:
    ResultPageTokenTransfer_Tag tag;
    PageTokenTransfer ok;
    Error err;

  cdef struct TokenTransferQuery:
    CArrayFieldElement contract_addresses;
    CArrayFieldElement account_addresses;
    CArrayU256 token_ids;
    Pagination pagination;

  cdef enum COptionFieldElement_Tag:
    SomeFieldElement,
    NoneFieldElement,

  cdef struct COptionFieldElement:
    COptionFieldElement_Tag tag;
    FieldElement some;

  cdef struct Contract:
    FieldElement contract_address;
    ContractType contract_type;
    COptionu64 head;
    COptionu64 tps;
    COptionu64 last_block_timestamp;
    COptionFieldElement last_pending_block_tx;
    uint64_t updated_at;
    uint64_t created_at;

  cdef struct TokenBalance:
    U256 balance;
    FieldElement account_address;
    FieldElement contract_address;
    COptionU256 token_id;

  cdef struct TokenTransfer:
    const char *id;
    FieldElement contract_address;
    FieldElement from_address;
    FieldElement to_address;
    U256 amount;
    COptionU256 token_id;
    uint64_t executed_at;
    COptionc_char event_id;

  cdef enum ResultCArrayFieldElement_Tag:
    OkCArrayFieldElement,
    ErrCArrayFieldElement,

  cdef struct ResultCArrayFieldElement:
    ResultCArrayFieldElement_Tag tag;
    CArrayFieldElement ok;
    Error err;

  cdef enum ResultFieldElement_Tag:
    OkFieldElement,
    ErrFieldElement,

  cdef struct ResultFieldElement:
    ResultFieldElement_Tag tag;
    FieldElement ok;
    Error err;

  cdef struct Signature:
    # The `r` value of a signature
    FieldElement r;
    # The `s` value of a signature
    FieldElement s;

  cdef enum ResultSignature_Tag:
    OkSignature,
    ErrSignature,

  cdef struct ResultSignature:
    ResultSignature_Tag tag;
    Signature ok;
    Error err;

  cdef enum ResultProvider_Tag:
    OkProvider,
    ErrProvider,

  cdef struct ResultProvider:
    ResultProvider_Tag tag;
    Provider *ok;
    Error err;

  cdef enum ResultAccount_Tag:
    OkAccount,
    ErrAccount,

  cdef struct ResultAccount:
    ResultAccount_Tag tag;
    Account *ok;
    Error err;

  cdef struct Call:
    FieldElement to;
    const char *selector;
    CArrayFieldElement calldata;

  # Block hash, number or tag
  cdef enum BlockId_Tag:
    Hash,
    Number,
    BlockTag_,

  cdef struct BlockId:
    BlockId_Tag tag;
    FieldElement hash;
    uint64_t number;
    BlockTag block_tag;

  cdef struct Controller:
    FieldElement address;
    const char *username;
    uint64_t deployed_at_timestamp;

  cdef struct OrderBy:
    const char *field;
    OrderDirection direction;

  cdef struct CArrayModel:
    Model *data;
    uintptr_t data_len;

  cdef struct World:
    FieldElement world_address;
    CArrayModel models;

  cdef struct TransactionCall:
    FieldElement contract_address;
    const char *entrypoint;
    CArrayFieldElement calldata;
    CallType call_type;
    FieldElement caller_address;

  cdef struct CArrayMember:
    Member *data;
    uintptr_t data_len;

  cdef struct Struct:
    const char *name;
    CArrayMember children;

  cdef struct CArrayAchievementTask:
    AchievementTask *data;
    uintptr_t data_len;

  cdef struct Achievement:
    const char *id;
    FieldElement world_address;
    const char *namespace_;
    const char *entity_id;
    bool hidden;
    uint32_t index;
    uint32_t points;
    const char *start;
    const char *end;
    const char *group;
    const char *icon;
    const char *title;
    const char *description;
    CArrayAchievementTask tasks;
    const char *data;
    uint32_t total_completions;
    double completion_rate;
    uint64_t created_at;
    uint64_t updated_at;

  cdef struct PlayerAchievementStats:
    uint32_t total_points;
    uint32_t completed_achievements;
    uint32_t total_achievements;
    double completion_percentage;
    COptionu64 last_achievement_at;
    uint64_t created_at;
    uint64_t updated_at;

  cdef struct CArrayPlayerAchievementProgress:
    PlayerAchievementProgress *data;
    uintptr_t data_len;

  cdef struct PlayerAchievementEntry:
    FieldElement player_address;
    PlayerAchievementStats stats;
    CArrayPlayerAchievementProgress achievements;

  cdef struct ActionCount:
    const char *action_name;
    uint32_t count;

  cdef struct AttributeFilter:
    const char *trait_name;
    const char *trait_value;

  cdef struct TokenContract:
    FieldElement contract_address;
    const char *name;
    const char *symbol;
    uint8_t decimals;
    const char *metadata;
    const char *token_metadata;
    COptionU256 total_supply;

  cdef struct CArrayEnumOption:
    EnumOption *data;
    uintptr_t data_len;

  cdef struct Enum:
    const char *name;
    uint8_t option;
    CArrayEnumOption options;

  cdef struct CArrayTy:
    Ty *data;
    uintptr_t data_len;

  cdef struct FixedSizeArray:
    CArrayTy array;
    uint32_t size;

  cdef enum Ty_Tag:
    Primitive_,
    Struct_,
    Enum_,
    Tuple_,
    Array_,
    FixedSizeArray_,
    ByteArray,

  cdef struct Ty:
    Ty_Tag tag;
    Primitive primitive;
    Struct struct_;
    Enum enum_;
    CArrayTy tuple;
    CArrayTy array;
    FixedSizeArray fixed_size_array;
    const char *byte_array;

  cdef struct Model:
    FieldElement world_address;
    Ty schema;
    const char *namespace_;
    const char *name;
    FieldElement selector;
    uint32_t packed_size;
    uint32_t unpacked_size;
    FieldElement class_hash;
    FieldElement contract_address;
    const char *layout;
    bool use_legacy_store;

  cdef struct Member:
    const char *name;
    Ty *ty;
    bool key;

  cdef struct AchievementTask:
    const char *task_id;
    const char *description;
    uint32_t total;
    uint32_t total_completions;
    double completion_rate;
    uint64_t created_at;

  cdef struct CArrayTaskProgress:
    TaskProgress *data;
    uintptr_t data_len;

  cdef struct PlayerAchievementProgress:
    Achievement achievement;
    CArrayTaskProgress task_progress;
    bool completed;
    double progress_percentage;

  cdef struct EnumOption:
    const char *name;
    Ty *ty;

  cdef struct TaskProgress:
    const char *task_id;
    uint32_t count;
    bool completed;

  # Creates a new Torii client instance
  #
  # # Parameters
  # * `torii_url` - URL of the Torii server
  # * `libp2p_relay_url` - URL of the libp2p relay server
  #
  # # Returns
  # Result containing pointer to new ToriiClient instance or error
  ResultToriiClient client_new(const char *torii_url);

  # Sets a logger callback function for the client
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `logger` - Callback function that takes a C string parameter
  void client_set_logger(ToriiClient *client, void (*logger)(const char*));

  # Publishes a message to the network
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `message` - JSON string containing typed data message
  # * `signature_felts` - Array of field elements containing signature
  # * `signature_felts_len` - Length of signature array
  #
  # # Returns
  # Result containing byte array or error
  Resultc_char client_publish_message(ToriiClient *client, Message message);

  # Publishes multiple messages to the network
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `messages` - Array of Message structs
  # * `messages_len` - Length of messages array
  #
  # # Returns
  # Result containing array of message IDs or error
  ResultCArrayc_char client_publish_message_batch(ToriiClient *client,
                                                  const Message *messages,
                                                  uintptr_t messages_len);

  # Retrieves controllers for the given contract addresses
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `contract_addresses` - Array of contract addresses. If empty, all controllers will be
  #   returned.
  #
  # # Returns
  # Result containing controllers or error
  ResultPageController client_controllers(ToriiClient *client, ControllerQuery query);

  # Queries entities matching given criteria
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `query` - Query parameters
  #
  # # Returns
  # Result containing array of matching entities or error
  ResultPageEntity client_entities(ToriiClient *client, Query query);

  # Retrieves event messages matching the given query
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `query` - Query parameters
  # * `historical` - Whether to include historical messages
  #
  # # Returns
  # Result containing array of matching event message entities or error
  ResultPageEntity client_event_messages(ToriiClient *client, Query query);

  # Gets the world metadata for the client
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  #
  # # Returns
  # World structure containing world information
  ResultCArrayWorld client_worlds(ToriiClient *client,
                                  const FieldElement *world_addresses,
                                  uintptr_t world_addresses_len);

  # Retrieves transactions matching the given query
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `query` - Query parameters
  #
  # # Returns
  # Result containing array of matching transactions or error
  ResultPageTransaction client_transactions(ToriiClient *client, TransactionQuery query);

  # Subscribes to transaction updates
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `filter` - Filter parameters
  # * `callback` - Function called when updates occur
  #
  # # Returns
  # Result containing pointer to Subscription or error
  ResultSubscription client_on_transaction(ToriiClient *client,
                                           COptionTransactionFilter filter,
                                           void (*callback)(Transaction));

  # Subscribes to entity state updates
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `clauses` - Array of entity key clauses to filter updates
  # * `clauses_len` - Length of clauses array
  # * `callback` - Function called when updates occur
  #
  # # Returns
  # Result containing pointer to Subscription or error
  ResultSubscription client_on_entity_state_update(ToriiClient *client,
                                                   COptionClause clause,
                                                   const FieldElement *world_addresses,
                                                   uintptr_t world_addresses_len,
                                                   void (*callback)(Entity));

  # Updates an existing entity subscription with new clauses
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `subscription` - Pointer to existing Subscription
  # * `clauses` - New array of entity key clauses
  # * `clauses_len` - Length of new clauses array
  #
  # # Returns
  # Result containing success boolean or error
  Resultbool client_update_entity_subscription(ToriiClient *client,
                                               Subscription *subscription,
                                               COptionClause clause,
                                               const FieldElement *world_addresses,
                                               uintptr_t world_addresses_len);

  # Retrieves aggregations (leaderboards, stats, rankings) matching query parameter
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `query` - AggregationQuery containing aggregator_ids, entity_ids, and pagination
  #
  # # Returns
  # Result containing Page of AggregationEntry or error
  ResultPageAggregationEntry client_aggregations(ToriiClient *client, AggregationQuery query);

  # Subscribes to aggregation updates (leaderboards, stats, rankings)
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `aggregator_ids` - Array of aggregator IDs to subscribe to
  # * `aggregator_ids_len` - Length of aggregator_ids array
  # * `entity_ids` - Array of entity IDs to subscribe to
  # * `entity_ids_len` - Length of entity_ids array
  # * `callback` - Function called when updates occur
  #
  # # Returns
  # Result containing pointer to Subscription or error
  ResultSubscription client_on_aggregation_update(ToriiClient *client,
                                                  const char *const *aggregator_ids,
                                                  uintptr_t aggregator_ids_len,
                                                  const char *const *entity_ids,
                                                  uintptr_t entity_ids_len,
                                                  void (*callback)(AggregationEntry));

  # Updates an existing aggregation subscription with new parameters
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `subscription` - Pointer to existing Subscription
  # * `aggregator_ids` - Array of aggregator IDs to subscribe to
  # * `aggregator_ids_len` - Length of aggregator_ids array
  # * `entity_ids` - Array of entity IDs to subscribe to
  # * `entity_ids_len` - Length of entity_ids array
  #
  # # Returns
  # Result containing success boolean or error
  Resultbool client_update_aggregation_subscription(ToriiClient *client,
                                                    Subscription *subscription,
                                                    const char *const *aggregator_ids,
                                                    uintptr_t aggregator_ids_len,
                                                    const char *const *entity_ids,
                                                    uintptr_t entity_ids_len);

  # Retrieves achievements matching query parameter
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `query` - AchievementQuery containing world_addresses, namespaces, hidden filter, and
  #   pagination
  #
  # # Returns
  # Result containing Page of Achievement or error
  ResultPageAchievement client_achievements(ToriiClient *client, AchievementQuery query);

  # Retrieves player achievement data matching query parameter
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `query` - PlayerAchievementQuery containing world_addresses, namespaces, player_addresses, and
  #   pagination
  #
  # # Returns
  # Result containing Page of PlayerAchievementEntry or error
  ResultPagePlayerAchievementEntry client_player_achievements(ToriiClient *client,
                                                              PlayerAchievementQuery query);

  # Subscribes to achievement progression updates
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `world_addresses` - Array of world addresses to subscribe to
  # * `world_addresses_len` - Length of world_addresses array
  # * `namespaces` - Array of namespaces to subscribe to
  # * `namespaces_len` - Length of namespaces array
  # * `player_addresses` - Array of player addresses to subscribe to
  # * `player_addresses_len` - Length of player_addresses array
  # * `achievement_ids` - Array of achievement IDs to subscribe to
  # * `achievement_ids_len` - Length of achievement_ids array
  # * `callback` - Function called when updates occur
  #
  # # Returns
  # Result containing pointer to Subscription or error
  ResultSubscription client_on_achievement_progression_update(ToriiClient *client,
                                                              const FieldElement *world_addresses,
                                                              uintptr_t world_addresses_len,
                                                              const char *const *namespaces,
                                                              uintptr_t namespaces_len,
                                                              const FieldElement *player_addresses,
                                                              uintptr_t player_addresses_len,
                                                              const char *const *achievement_ids,
                                                              uintptr_t achievement_ids_len,
                                                              void (*callback)(AchievementProgression));

  # Updates an existing achievement progression subscription with new parameters
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `subscription` - Pointer to existing Subscription
  # * `world_addresses` - Array of world addresses to subscribe to
  # * `world_addresses_len` - Length of world_addresses array
  # * `namespaces` - Array of namespaces to subscribe to
  # * `namespaces_len` - Length of namespaces array
  # * `player_addresses` - Array of player addresses to subscribe to
  # * `player_addresses_len` - Length of player_addresses array
  # * `achievement_ids` - Array of achievement IDs to subscribe to
  # * `achievement_ids_len` - Length of achievement_ids array
  #
  # # Returns
  # Result containing success boolean or error
  Resultbool client_update_achievement_progression_subscription(ToriiClient *client,
                                                                Subscription *subscription,
                                                                const FieldElement *world_addresses,
                                                                uintptr_t world_addresses_len,
                                                                const char *const *namespaces,
                                                                uintptr_t namespaces_len,
                                                                const FieldElement *player_addresses,
                                                                uintptr_t player_addresses_len,
                                                                const char *const *achievement_ids,
                                                                uintptr_t achievement_ids_len);

  # Retrieves activities (user session tracking) matching query parameter
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `query` - ActivityQuery containing world_addresses, namespaces, caller_addresses, and
  #   pagination
  #
  # # Returns
  # Result containing Page of Activity or error
  ResultPageActivity client_activities(ToriiClient *client, ActivityQuery query);

  # Subscribes to activity updates (user session tracking)
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `world_addresses` - Array of world addresses to subscribe to
  # * `world_addresses_len` - Length of world_addresses array
  # * `namespaces` - Array of namespaces to subscribe to
  # * `namespaces_len` - Length of namespaces array
  # * `caller_addresses` - Array of caller addresses to subscribe to
  # * `caller_addresses_len` - Length of caller_addresses array
  # * `callback` - Function called when updates occur
  #
  # # Returns
  # Result containing pointer to Subscription or error
  ResultSubscription client_on_activity_update(ToriiClient *client,
                                               const FieldElement *world_addresses,
                                               uintptr_t world_addresses_len,
                                               const char *const *namespaces,
                                               uintptr_t namespaces_len,
                                               const FieldElement *caller_addresses,
                                               uintptr_t caller_addresses_len,
                                               void (*callback)(Activity));

  # Updates an existing activity subscription with new parameters
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `subscription` - Pointer to existing Subscription
  # * `world_addresses` - Array of world addresses to subscribe to
  # * `world_addresses_len` - Length of world_addresses array
  # * `namespaces` - Array of namespaces to subscribe to
  # * `namespaces_len` - Length of namespaces array
  # * `caller_addresses` - Array of caller addresses to subscribe to
  # * `caller_addresses_len` - Length of caller_addresses array
  #
  # # Returns
  # Result containing success boolean or error
  Resultbool client_update_activity_subscription(ToriiClient *client,
                                                 Subscription *subscription,
                                                 const FieldElement *world_addresses,
                                                 uintptr_t world_addresses_len,
                                                 const char *const *namespaces,
                                                 uintptr_t namespaces_len,
                                                 const FieldElement *caller_addresses,
                                                 uintptr_t caller_addresses_len);

  # Subscribes to event message updates
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `clauses` - Array of entity key clauses to filter updates
  # * `clauses_len` - Length of clauses array
  # * `callback` - Function called when updates occur
  #
  # # Returns
  # Result containing pointer to Subscription or error
  ResultSubscription client_on_event_message_update(ToriiClient *client,
                                                    COptionClause clause,
                                                    const FieldElement *world_addresses,
                                                    uintptr_t world_addresses_len,
                                                    void (*callback)(Entity));

  # Updates an existing event message subscription
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `subscription` - Pointer to existing Subscription
  # * `clauses` - New array of entity key clauses
  # * `clauses_len` - Length of new clauses array
  #
  # # Returns
  # Result containing success boolean or error
  Resultbool client_update_event_message_subscription(ToriiClient *client,
                                                      Subscription *subscription,
                                                      COptionClause clause,
                                                      const FieldElement *world_addresses,
                                                      uintptr_t world_addresses_len);

  # Subscribes to Starknet events
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `clauses` - Array of entity key clauses to filter events
  # * `clauses_len` - Length of clauses array
  # * `callback` - Function called when events occur
  #
  # # Returns
  # Result containing pointer to Subscription or error
  ResultSubscription client_on_starknet_event(ToriiClient *client,
                                              const KeysClause *clauses,
                                              uintptr_t clauses_len,
                                              void (*callback)(Event));

  # Retrieves token information for given contract addresses
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `contract_addresses` - Array of contract addresses
  # * `contract_addresses_len` - Length of addresses array
  # * `token_ids` - Array of token ids
  # * `token_ids_len` - Length of token ids array
  # * `limit` - Maximum number of tokens to return
  # * `cursor` - Cursor to start from
  #
  # # Returns
  # Result containing array of Token information or error
  ResultPageToken client_tokens(ToriiClient *client, TokenQuery query);

  # Subscribes to token updates
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `contract_addresses` - Array of contract addresses
  # * `callback` - Function called when updates occur
  #
  # # Returns
  # Result containing pointer to Subscription or error
  ResultSubscription client_on_token_update(ToriiClient *client,
                                            const FieldElement *contract_addresses,
                                            uintptr_t contract_addresses_len,
                                            const U256 *token_ids,
                                            uintptr_t token_ids_len,
                                            void (*callback)(Token));

  # Gets token balances for given accounts and contracts
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `contract_addresses` - Array of contract addresses
  # * `contract_addresses_len` - Length of contract addresses array
  # * `account_addresses` - Array of account addresses
  # * `account_addresses_len` - Length of account addresses array
  # * `token_ids` - Array of token ids
  # * `token_ids_len` - Length of token ids array
  # * `limit` - Maximum number of token balances to return
  # * `cursor` - Cursor to start from
  #
  # # Returns
  # Result containing array of TokenBalance information or error
  ResultPageTokenBalance client_token_balances(ToriiClient *client, TokenBalanceQuery query);

  # Gets token collections for given accounts and contracts
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `contract_addresses` - Array of contract addresses
  # * `contract_addresses_len` - Length of contract addresses array
  # * `account_addresses` - Array of account addresses
  # * `account_addresses_len` - Length of account addresses array
  # * `token_ids` - Array of token ids
  # * `token_ids_len` - Length of token ids array
  # * `limit` - Maximum number of token balances to return
  # * `cursor` - Cursor to start from
  #
  # # Returns
  # Result containing array of TokenBalance information or error
  ResultPageTokenContract client_token_contracts(ToriiClient *client, TokenContractQuery query);

  # Gets contracts matching the given query
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `query` - ContractQuery parameters
  #
  # # Returns
  # Result containing array of Contract information or error
  ResultCArrayContract client_contracts(ToriiClient *client, ContractQuery query);

  # Retrieves token transfers matching the given query
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `query` - TokenTransferQuery parameters
  #
  # # Returns
  # Result containing array of TokenTransfer information or error
  ResultPageTokenTransfer client_token_transfers(ToriiClient *client, TokenTransferQuery query);

  # Subscribes to contract updates
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `contract_address` - Optional contract address to filter updates
  # * `callback` - Function called when updates occur
  #
  # # Returns
  # Result containing pointer to Subscription or error
  ResultSubscription on_contract_update(ToriiClient *client,
                                        const FieldElement *contract_address,
                                        void (*callback)(Contract));

  # Subscribes to token balance updates
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `contract_addresses` - Array of contract addresses to filter (empty for all)
  # * `contract_addresses_len` - Length of contract addresses array
  # * `account_addresses` - Array of account addresses to filter (empty for all)
  # * `account_addresses_len` - Length of account addresses array
  # * `callback` - Function called when updates occur
  #
  # # Returns
  # Result containing pointer to Subscription or error
  ResultSubscription client_on_token_balance_update(ToriiClient *client,
                                                    const FieldElement *contract_addresses,
                                                    uintptr_t contract_addresses_len,
                                                    const FieldElement *account_addresses,
                                                    uintptr_t account_addresses_len,
                                                    const U256 *token_ids,
                                                    uintptr_t token_ids_len,
                                                    void (*callback)(TokenBalance));

  # Updates an existing token balance subscription
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `subscription` - Pointer to existing Subscription
  # * `contract_addresses` - Array of contract addresses to filter (empty for all)
  # * `contract_addresses_len` - Length of contract addresses array
  # * `account_addresses` - Array of account addresses to filter (empty for all)
  # * `account_addresses_len` - Length of account addresses array
  #
  # # Returns
  # Result containing success boolean or error
  Resultbool client_update_token_balance_subscription(ToriiClient *client,
                                                      Subscription *subscription,
                                                      const FieldElement *contract_addresses,
                                                      uintptr_t contract_addresses_len,
                                                      const FieldElement *account_addresses,
                                                      uintptr_t account_addresses_len,
                                                      const U256 *token_ids,
                                                      uintptr_t token_ids_len);

  # Subscribes to token transfer updates
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `contract_addresses` - Array of contract addresses to filter (empty for all)
  # * `contract_addresses_len` - Length of contract addresses array
  # * `account_addresses` - Array of account addresses to filter (empty for all)
  # * `account_addresses_len` - Length of account addresses array
  # * `token_ids` - Array of token IDs to filter (empty for all)
  # * `token_ids_len` - Length of token IDs array
  # * `callback` - Function called when updates occur
  #
  # # Returns
  # Result containing pointer to Subscription or error
  ResultSubscription client_on_token_transfer_update(ToriiClient *client,
                                                     const FieldElement *contract_addresses,
                                                     uintptr_t contract_addresses_len,
                                                     const FieldElement *account_addresses,
                                                     uintptr_t account_addresses_len,
                                                     const U256 *token_ids,
                                                     uintptr_t token_ids_len,
                                                     void (*callback)(TokenTransfer));

  # Updates an existing token transfer subscription
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `subscription` - Pointer to existing Subscription
  # * `contract_addresses` - Array of contract addresses to filter (empty for all)
  # * `contract_addresses_len` - Length of contract addresses array
  # * `account_addresses` - Array of account addresses to filter (empty for all)
  # * `account_addresses_len` - Length of account addresses array
  # * `token_ids` - Array of token IDs to filter (empty for all)
  # * `token_ids_len` - Length of token IDs array
  #
  # # Returns
  # Result containing success boolean or error
  Resultbool client_update_token_transfer_subscription(ToriiClient *client,
                                                       Subscription *subscription,
                                                       const FieldElement *contract_addresses,
                                                       uintptr_t contract_addresses_len,
                                                       const FieldElement *account_addresses,
                                                       uintptr_t account_addresses_len,
                                                       const U256 *token_ids,
                                                       uintptr_t token_ids_len);

  # Serializes a string into a byte array
  #
  # # Parameters
  # * `str` - String to serialize
  #
  # # Returns
  # Result containing array of FieldElements or error
  ResultCArrayFieldElement bytearray_serialize(const char *str);

  # Deserializes field elements into a string
  #
  # # Parameters
  # * `felts` - Array of field elements
  # * `felts_len` - Length of field elements array
  #
  # # Returns
  # Result containing pointer to C string or error
  Resultc_char bytearray_deserialize(const FieldElement *felts, uintptr_t felts_len);

  # Computes Poseidon hash of field elements
  #
  # # Parameters
  # * `felts` - Array of field elements
  # * `felts_len` - Length of array
  #
  # # Returns
  # FieldElement containing the hash result
  FieldElement poseidon_hash(const FieldElement *felts, uintptr_t felts_len);

  # Gets selector from name string
  #
  # # Parameters
  # * `name` - Name to compute selector from
  #
  # # Returns
  # Result containing FieldElement selector or error
  ResultFieldElement get_selector_from_name(const char *name);

  # Gets selector from tag string
  #
  # # Parameters
  # * `tag` - Tag to compute selector from
  #
  # # Returns
  # FieldElement containing the computed selector
  FieldElement get_selector_from_tag(const char *tag);

  # Computes Starknet keccak hash of bytes
  #
  # # Parameters
  # * `bytes` - Byte array to hash
  # * `bytes_len` - Length of byte array
  #
  # # Returns
  # FieldElement containing the hash result
  FieldElement starknet_keccak(const uint8_t *bytes, uintptr_t bytes_len);

  # Converts a short string to field element
  #
  # # Parameters
  # * `str` - String to convert
  #
  # # Returns
  # Result containing FieldElement or error
  ResultFieldElement cairo_short_string_to_felt(const char *str);

  # Parses a field element into a short string
  #
  # # Parameters
  # * `felt` - FieldElement to parse
  #
  # # Returns
  # Result containing pointer to C string or error
  Resultc_char parse_cairo_short_string(FieldElement felt);

  # Encodes typed data
  #
  # # Parameters
  # * `typed_data` - JSON string of typed data
  # * `address` - Address as FieldElement
  #
  # # Returns
  # Result containing encoded FieldElement or error
  ResultFieldElement typed_data_encode(const char *typed_data, FieldElement address);

  # Generates a new signing key
  #
  # # Returns
  # FieldElement containing the new private key
  FieldElement signing_key_new();

  # Signs a hash with a private key
  #
  # # Parameters
  # * `private_key` - Private key as FieldElement
  # * `hash` - Hash to sign as FieldElement
  #
  # # Returns
  # Result containing Signature or error
  ResultSignature signing_key_sign(FieldElement private_key, FieldElement hash);

  # Creates a verifying key from a signing key
  #
  # # Parameters
  # * `signing_key` - Signing key as FieldElement
  #
  # # Returns
  # FieldElement containing the verifying key
  FieldElement verifying_key_new(FieldElement signing_key);

  # Verifies a signature
  #
  # # Parameters
  # * `verifying_key` - Verifying key as FieldElement
  # * `hash` - Hash that was signed
  # * `signature` - Signature to verify
  #
  # # Returns
  # Result containing verification success boolean or error
  Resultbool verifying_key_verify(FieldElement verifying_key,
                                  FieldElement hash,
                                  Signature signature);

  # Creates a new provider instance
  #
  # # Parameters
  # * `rpc_url` - URL of the RPC endpoint
  #
  # # Returns
  # Result containing pointer to Provider or error
  ResultProvider provider_new(const char *rpc_url);

  # Creates a new account instance
  #
  # # Parameters
  # * `rpc` - Pointer to Provider
  # * `private_key` - Private key as FieldElement
  # * `address` - Account address as string
  #
  # # Returns
  # Result containing pointer to Account or error
  ResultAccount account_new(Provider *rpc, FieldElement private_key, const char *address);

  # Makes a Starknet call
  #
  # # Parameters
  # * `provider` - Pointer to Provider
  # * `call` - Call parameters
  # * `block_id` - Block identifier
  #
  # # Returns
  # Result containing array of FieldElements or error
  ResultCArrayFieldElement starknet_call(Provider *provider, Call call, BlockId block_id);

  # Deploys a burner account
  #
  # # Parameters
  # * `provider` - Pointer to Provider
  # * `master_account` - Pointer to master Account
  # * `signing_key` - Signing key for new account
  #
  # # Returns
  # Result containing pointer to new Account or error
  ResultAccount account_deploy_burner(Provider *provider,
                                      Account *master_account,
                                      FieldElement signing_key);

  # Gets account address
  #
  # # Parameters
  # * `account` - Pointer to Account
  #
  # # Returns
  # FieldElement containing the account address
  FieldElement account_address(Account *account);

  # Gets account chain ID
  #
  # # Parameters
  # * `account` - Pointer to Account
  #
  # # Returns
  # FieldElement containing the chain ID
  FieldElement account_chain_id(Account *account);

  # Sets block ID for account
  #
  # # Parameters
  # * `account` - Pointer to Account
  # * `block_id` - New block ID
  void account_set_block_id(Account *account, BlockId block_id);

  # Gets account nonce
  #
  # # Parameters
  # * `account` - Pointer to Account
  #
  # # Returns
  # Result containing FieldElement nonce or error
  ResultFieldElement account_nonce(Account *account);

  # Executes raw transaction
  #
  # # Parameters
  # * `account` - Pointer to Account
  # * `calldata` - Array of Call structs
  # * `calldata_len` - Length of calldata array
  #
  # # Returns
  # Result containing transaction hash as FieldElement or error
  ResultFieldElement account_execute_raw(Account *account,
                                         const Call *calldata,
                                         uintptr_t calldata_len);

  # Waits for transaction completion
  #
  # # Parameters
  # * `rpc` - Pointer to Provider
  # * `txn_hash` - Transaction hash as FieldElement
  #
  # # Returns
  # Result containing success boolean or error
  Resultbool wait_for_transaction(Provider *rpc, FieldElement txn_hash);

  # Computes contract address
  #
  # # Parameters
  # * `class_hash` - Class hash as FieldElement
  # * `salt` - Salt as FieldElement
  # * `constructor_calldata` - Array of constructor parameters
  # * `constructor_calldata_len` - Length of constructor parameters
  # * `deployer_address` - Deployer address as FieldElement
  #
  # # Returns
  # FieldElement containing computed contract address
  FieldElement hash_get_contract_address(FieldElement class_hash,
                                         FieldElement salt,
                                         const FieldElement *constructor_calldata,
                                         uintptr_t constructor_calldata_len,
                                         FieldElement deployer_address);

  # Cancels a subscription
  #
  # # Parameters
  # * `subscription` - Pointer to Subscription to cancel
  void subscription_cancel(Subscription *subscription);

  # Frees a ToriiClient instance
  #
  # # Parameters
  # * `t` - Pointer to ToriiClient to free
  void client_free(ToriiClient *t);

  # Frees a Provider instance
  #
  # # Parameters
  # * `rpc` - Pointer to Provider to free
  void provider_free(Provider *rpc);

  # Frees a Model instance
  #
  # # Parameters
  # * `model` - Pointer to Model to free
  void model_free(Struct *model);

  # Frees an Account instance
  #
  # # Parameters
  # * `account` - Pointer to Account to free
  void account_free(Account *account);

  # Frees a Type instance
  #
  # # Parameters
  # * `ty` - Pointer to Type to free
  void ty_free(Ty *ty);

  # Frees an Entity instance
  #
  # # Parameters
  # * `entity` - Pointer to Entity to free
  void entity_free(Entity *entity);

  # Frees an Error instance
  #
  # # Parameters
  # * `error` - Pointer to Error to free
  void error_free(Error *error);

  # Frees a WorldMetadata instance
  #
  # # Parameters
  # * `metadata` - Pointer to WorldMetadata to free
  void world_metadata_free(World *metadata);

  # Frees a CArray instance
  #
  # # Parameters
  # * `data` - Pointer to array data
  # * `data_len` - Length of array
  void carray_free(void *data, uintptr_t data_len);

  # Frees a string
  #
  # # Parameters
  # * `string` - Pointer to string to free
  void string_free(char *string);
