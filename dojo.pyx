from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  cdef enum BlockTag:
    Latest,
    Pending,

  cdef enum ComparisonOperator:
    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
    In,
    NotIn,

  cdef enum LogicalOperator:
    And,
    Or,

  cdef enum OrderDirection:
    Asc,
    Desc,

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

  cdef struct FieldElement:
    uint8_t data[32];

  cdef struct CArrayu8:
    uint8_t *data;
    uintptr_t data_len;

  cdef enum ResultCArrayu8_Tag:
    OkCArrayu8,
    ErrCArrayu8,

  cdef struct ResultCArrayu8:
    ResultCArrayu8_Tag tag;
    CArrayu8 ok;
    Error err;

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
    U256,
    U256,
    USize,
    Bool,
    Felt252,
    ClassHash,
    ContractAddress,

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
    uint64_t u256[4];
    uint32_t u256[8];
    uint32_t u_size;
    bool bool_;
    FieldElement felt252;
    FieldElement class_hash;
    FieldElement contract_address;

  cdef struct EnumOption:
    const char *name;
    Ty *ty;

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

  cdef enum Ty_Tag:
    Primitive_,
    Struct_,
    Enum_,
    Tuple_,
    Array_,
    ByteArray,

  cdef struct Ty:
    Ty_Tag tag;
    Primitive primitive;
    Struct struct_;
    Enum enum_;
    CArrayTy tuple;
    CArrayTy array;
    const char *byte_array;

  cdef struct Member:
    const char *name;
    Ty *ty;
    bool key;

  cdef struct CArrayMember:
    Member *data;
    uintptr_t data_len;

  cdef struct Struct:
    const char *name;
    CArrayMember children;

  cdef struct CArrayStruct:
    Struct *data;
    uintptr_t data_len;

  cdef struct Entity:
    FieldElement hashed_keys;
    CArrayStruct models;

  cdef struct CArrayEntity:
    Entity *data;
    uintptr_t data_len;

  cdef enum ResultCArrayEntity_Tag:
    OkCArrayEntity,
    ErrCArrayEntity,

  cdef struct ResultCArrayEntity:
    ResultCArrayEntity_Tag tag;
    CArrayEntity ok;
    Error err;

  cdef enum COptionFieldElement_Tag:
    SomeFieldElement,
    NoneFieldElement,

  cdef struct COptionFieldElement:
    COptionFieldElement_Tag tag;
    FieldElement some;

  cdef struct CArrayCOptionFieldElement:
    COptionFieldElement *data;
    uintptr_t data_len;

  cdef struct CArrayc_char:
    const char **data;
    uintptr_t data_len;

  cdef struct KeysClause:
    CArrayCOptionFieldElement keys;
    PatternMatching pattern_matching;
    CArrayc_char models;

  cdef struct CArrayMemberValue:
    MemberValue *data;
    uintptr_t data_len;

  cdef enum MemberValue_Tag:
    Primitive,
    String,
    List,

  cdef struct MemberValue:
    MemberValue_Tag tag;
    Primitive primitive;
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
    Keys,
    CMember,
    Composite,

  cdef struct Clause:
    Clause_Tag tag;
    KeysClause keys;
    MemberClause c_member;
    CompositeClause composite;

  cdef enum COptionClause_Tag:
    SomeClause,
    NoneClause,

  cdef struct COptionClause:
    COptionClause_Tag tag;
    Clause some;

  cdef struct OrderBy:
    const char *model;
    const char *member;
    OrderDirection direction;

  cdef struct CArrayOrderBy:
    OrderBy *data;
    uintptr_t data_len;

  cdef struct Query:
    uint32_t limit;
    uint32_t offset;
    COptionClause clause;
    bool dont_include_hashed_keys;
    CArrayOrderBy order_by;
    CArrayc_char entity_models;
    uint64_t entity_updated_after;

  cdef struct CArrayFieldElement:
    FieldElement *data;
    uintptr_t data_len;

  cdef struct ModelMetadata:
    Ty schema;
    const char *namespace_;
    const char *name;
    uint32_t packed_size;
    uint32_t unpacked_size;
    FieldElement class_hash;
    FieldElement contract_address;
    CArrayFieldElement layout;

  cdef struct CHashItemFieldElementModelMetadata:
    FieldElement key;
    ModelMetadata value;

  cdef struct CArrayCHashItemFieldElementModelMetadata:
    CHashItemFieldElementModelMetadata *data;
    uintptr_t data_len;

  cdef struct WorldMetadata:
    FieldElement world_address;
    CArrayCHashItemFieldElementModelMetadata models;

  cdef enum ResultSubscription_Tag:
    OkSubscription,
    ErrSubscription,

  cdef struct ResultSubscription:
    ResultSubscription_Tag tag;
    Subscription *ok;
    Error err;

  cdef enum EntityKeysClause_Tag:
    HashedKeys,
    EntityKeys,

  cdef struct EntityKeysClause:
    EntityKeysClause_Tag tag;
    CArrayFieldElement hashed_keys;
    KeysClause entity_keys;

  cdef enum Resultbool_Tag:
    Okbool,
    Errbool,

  cdef struct Resultbool:
    Resultbool_Tag tag;
    bool ok;
    Error err;

  cdef struct Event:
    CArrayFieldElement keys;
    CArrayFieldElement data;
    FieldElement transaction_hash;

  cdef struct Token:
    FieldElement contract_address;
    const char *name;
    const char *symbol;
    uint8_t decimals;
    const char *metadata;

  cdef struct CArrayToken:
    Token *data;
    uintptr_t data_len;

  cdef enum ResultCArrayToken_Tag:
    OkCArrayToken,
    ErrCArrayToken,

  cdef struct ResultCArrayToken:
    ResultCArrayToken_Tag tag;
    CArrayToken ok;
    Error err;

  cdef struct TokenBalance:
    uint64_t balance[4];
    uint32_t balance[8];
    FieldElement account_address;
    FieldElement contract_address;
    const char *token_id;

  cdef struct CArrayTokenBalance:
    TokenBalance *data;
    uintptr_t data_len;

  cdef enum ResultCArrayTokenBalance_Tag:
    OkCArrayTokenBalance,
    ErrCArrayTokenBalance,

  cdef struct ResultCArrayTokenBalance:
    ResultCArrayTokenBalance_Tag tag;
    CArrayTokenBalance ok;
    Error err;

  cdef struct IndexerUpdate:
    int64_t head;
    int64_t tps;
    int64_t last_block_timestamp;
    FieldElement contract_address;

  cdef enum ResultCArrayFieldElement_Tag:
    OkCArrayFieldElement,
    ErrCArrayFieldElement,

  cdef struct ResultCArrayFieldElement:
    ResultCArrayFieldElement_Tag tag;
    CArrayFieldElement ok;
    Error err;

  cdef enum Resultc_char_Tag:
    Okc_char,
    Errc_char,

  cdef struct Resultc_char:
    Resultc_char_Tag tag;
    const char *ok;
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

  # Creates a new Torii client instance
  #
  # # Parameters
  # * `torii_url` - URL of the Torii server
  # * `rpc_url` - URL of the Starknet RPC endpoint
  # * `libp2p_relay_url` - URL of the libp2p relay server
  # * `world` - World address as a FieldElement
  #
  # # Returns
  # Result containing pointer to new ToriiClient instance or error
  ResultToriiClient client_new(const char *torii_url,
                               const char *rpc_url,
                               const char *libp2p_relay_url,
                               FieldElement world);

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
  ResultCArrayu8 client_publish_message(ToriiClient *client,
                                        const char *message,
                                        const FieldElement *signature_felts,
                                        uintptr_t signature_felts_len);

  # Queries entities matching given criteria
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `query` - Query parameters
  #
  # # Returns
  # Result containing array of matching entities or error
  ResultCArrayEntity client_entities(ToriiClient *client, const Query *query);

  # Retrieves event messages matching the given query
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `query` - Query parameters
  # * `historical` - Whether to include historical messages
  #
  # # Returns
  # Result containing array of matching event message entities or error
  ResultCArrayEntity client_event_messages(ToriiClient *client,
                                           const Query *query,
                                           bool historical);

  # Gets the world metadata for the client
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  #
  # # Returns
  # WorldMetadata structure containing world information
  WorldMetadata client_metadata(ToriiClient *client);

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
                                                   const EntityKeysClause *clauses,
                                                   uintptr_t clauses_len,
                                                   void (*callback)(FieldElement, CArrayStruct));

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
                                               const EntityKeysClause *clauses,
                                               uintptr_t clauses_len);

  # Subscribes to event message updates
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `clauses` - Array of entity key clauses to filter updates
  # * `clauses_len` - Length of clauses array
  # * `historical` - Whether to include historical messages
  # * `callback` - Function called when updates occur
  #
  # # Returns
  # Result containing pointer to Subscription or error
  ResultSubscription client_on_event_message_update(ToriiClient *client,
                                                    const EntityKeysClause *clauses,
                                                    uintptr_t clauses_len,
                                                    bool historical,
                                                    void (*callback)(FieldElement, CArrayStruct));

  # Updates an existing event message subscription
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `subscription` - Pointer to existing Subscription
  # * `clauses` - New array of entity key clauses
  # * `clauses_len` - Length of new clauses array
  # * `historical` - Whether to include historical messages
  #
  # # Returns
  # Result containing success boolean or error
  Resultbool client_update_event_message_subscription(ToriiClient *client,
                                                      Subscription *subscription,
                                                      const EntityKeysClause *clauses,
                                                      uintptr_t clauses_len,
                                                      bool historical);

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
                                              const EntityKeysClause *clauses,
                                              uintptr_t clauses_len,
                                              void (*callback)(Event));

  # Retrieves token information for given contract addresses
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `contract_addresses` - Array of contract addresses
  # * `contract_addresses_len` - Length of addresses array
  #
  # # Returns
  # Result containing array of Token information or error
  ResultCArrayToken client_tokens(ToriiClient *client,
                                  const FieldElement *contract_addresses,
                                  uintptr_t contract_addresses_len);

  # Gets token balances for given accounts and contracts
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `account_addresses` - Array of account addresses
  # * `account_addresses_len` - Length of account addresses array
  # * `contract_addresses` - Array of contract addresses
  # * `contract_addresses_len` - Length of contract addresses array
  #
  # # Returns
  # Result containing array of TokenBalance information or error
  ResultCArrayTokenBalance client_token_balances(ToriiClient *client,
                                                 const FieldElement *account_addresses,
                                                 uintptr_t account_addresses_len,
                                                 const FieldElement *contract_addresses,
                                                 uintptr_t contract_addresses_len);

  # Subscribes to indexer updates
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `contract_address` - Optional contract address to filter updates
  # * `callback` - Function called when updates occur
  #
  # # Returns
  # Result containing pointer to Subscription or error
  ResultSubscription on_indexer_update(ToriiClient *client,
                                       const FieldElement *contract_address,
                                       void (*callback)(IndexerUpdate));

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
  void world_metadata_free(WorldMetadata *metadata);

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
