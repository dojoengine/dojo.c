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

  cdef enum PaginationDirection:
    Forward,
    Backward,

  cdef enum PatternMatching:
    FixedLen # = 0,
    VariableLen # = 1,

  cdef struct Account:
    pass

  cdef struct ControllerAccount:
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

  cdef enum ResultControllerAccount_Tag:
    OkControllerAccount,
    ErrControllerAccount,

  cdef struct ResultControllerAccount:
    ResultControllerAccount_Tag tag;
    ControllerAccount *ok;
    Error err;

  cdef enum Resultbool_Tag:
    Okbool,
    Errbool,

  cdef struct Resultbool:
    Resultbool_Tag tag;
    bool ok;
    Error err;

  cdef enum ResultFieldElement_Tag:
    OkFieldElement,
    ErrFieldElement,

  cdef struct ResultFieldElement:
    ResultFieldElement_Tag tag;
    FieldElement ok;
    Error err;

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

  cdef struct CArrayController:
    Controller *data;
    uintptr_t data_len;

  cdef enum ResultCArrayController_Tag:
    OkCArrayController,
    ErrCArrayController,

  cdef struct ResultCArrayController:
    ResultCArrayController_Tag tag;
    CArrayController ok;
    Error err;

  cdef struct CArrayEntity:
    Entity *data;
    uintptr_t data_len;

  cdef enum COptionc_char_Tag:
    Somec_char,
    Nonec_char,

  cdef struct COptionc_char:
    COptionc_char_Tag tag;
    const char *some;

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

  cdef struct CArrayOrderBy:
    OrderBy *data;
    uintptr_t data_len;

  cdef struct Pagination:
    COptionc_char cursor;
    uint32_t limit;
    PaginationDirection direction;
    CArrayOrderBy order_by;

  cdef struct CArrayFieldElement:
    FieldElement *data;
    uintptr_t data_len;

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
    Pagination pagination;
    COptionClause clause;
    bool no_hashed_keys;
    CArrayc_char models;
    bool historical;

  cdef struct CArrayCHashItemFieldElementModelMetadata:
    CHashItemFieldElementModelMetadata *data;
    uintptr_t data_len;

  cdef struct WorldMetadata:
    FieldElement world_address;
    CArrayCHashItemFieldElementModelMetadata models;

  cdef enum ResultWorldMetadata_Tag:
    OkWorldMetadata,
    ErrWorldMetadata,

  cdef struct ResultWorldMetadata:
    ResultWorldMetadata_Tag tag;
    WorldMetadata ok;
    Error err;

  cdef enum ResultSubscription_Tag:
    OkSubscription,
    ErrSubscription,

  cdef struct ResultSubscription:
    ResultSubscription_Tag tag;
    Subscription *ok;
    Error err;

  cdef struct CArrayStruct:
    Struct *data;
    uintptr_t data_len;

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

  cdef struct Token:
    FieldElement contract_address;
    U256 token_id;
    const char *name;
    const char *symbol;
    uint8_t decimals;
    const char *metadata;

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

  cdef struct IndexerUpdate:
    int64_t head;
    int64_t tps;
    int64_t last_block_timestamp;
    FieldElement contract_address;

  cdef struct TokenBalance:
    U256 balance;
    FieldElement account_address;
    FieldElement contract_address;
    U256 token_id;

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

  cdef struct Policy:
    FieldElement target;
    const char *method;
    const char *description;

  cdef struct Controller:
    FieldElement address;
    const char *username;
    uint64_t deployed_at_timestamp;

  cdef struct Entity:
    FieldElement hashed_keys;
    CArrayStruct models;

  cdef struct OrderBy:
    const char *model;
    const char *member;
    OrderDirection direction;

  cdef enum COptionFieldElement_Tag:
    SomeFieldElement,
    NoneFieldElement,

  cdef struct COptionFieldElement:
    COptionFieldElement_Tag tag;
    FieldElement some;

  cdef struct CArrayMember:
    Member *data;
    uintptr_t data_len;

  cdef struct Struct:
    const char *name;
    CArrayMember children;

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

  cdef struct Member:
    const char *name;
    Ty *ty;
    bool key;

  cdef struct EnumOption:
    const char *name;
    Ty *ty;

  # Creates a new Torii client instance
  #
  # # Parameters
  # * `torii_url` - URL of the Torii server
  # * `libp2p_relay_url` - URL of the libp2p relay server
  # * `world` - World address as a FieldElement
  #
  # # Returns
  # Result containing pointer to new ToriiClient instance or error
  ResultToriiClient client_new(const char *torii_url,
                               const char *libp2p_relay_url,
                               FieldElement world);

  # Initiates a connection to establish a new session account
  #
  # This function:
  # 1. Generates a new signing key pair
  # 2. Starts a local HTTP server to receive the callback
  # 3. Opens the keychain session URL in browser
  # 4. Waits for callback with session details
  # 5. Creates and stores the session
  # 6. Calls the provided callback with the new session account
  #
  # # Safety
  # This function is marked as unsafe because it:
  # - Handles raw C pointers
  # - Performs FFI operations
  # - Creates system-level resources (HTTP server, keyring entries)
  #
  # # Parameters
  # * `rpc_url` - Pointer to null-terminated string containing the RPC endpoint URL
  # * `policies` - Pointer to array of Policy structs defining session permissions
  # * `policies_len` - Length of the policies array
  # * `account_callback` - Function pointer called with the new session account when ready
  #
  # # Example
  # ```c
  # void on_account(SessionAccount* account) {
  #     // Handle new session account
  # }
  #
  # controller_connect(
  #     "https://rpc.example.com",
  #     policies,
  #     policies_length,
  #     on_account
  # );
  # ```
  void controller_connect(const char *rpc_url,
                          const Policy *policies,
                          uintptr_t policies_len,
                          void (*account_callback)(ControllerAccount*));

  # Retrieves a stored session account if one exists and is valid
  #
  # # Parameters
  # * `policies` - Array of policies to match the session
  # * `policies_len` - Length of policies array
  # * `chain_id` - Chain ID to verify against
  #
  # # Returns
  # Result containing pointer to SessionAccount or error if no valid account exists
  ResultControllerAccount controller_account(const Policy *policies,
                                             uintptr_t policies_len,
                                             FieldElement chain_id);

  # Clears sessions matching the specified policies and chain ID
  #
  # # Parameters
  # * `policies` - Array of policies to match
  # * `policies_len` - Length of policies array
  # * `chain_id` - Chain ID to match
  #
  # # Returns
  # Result containing success boolean or error
  Resultbool controller_clear(const Policy *policies,
                              uintptr_t policies_len,
                              FieldElement chain_id);

  # Gets the username of controller
  #
  # # Parameters
  # * `account` - Pointer to Account
  #
  # # Returns
  # CString containing the username
  const char *controller_username(ControllerAccount *controller);

  # Gets account address
  #
  # # Parameters
  # * `account` - Pointer to Account
  #
  # # Returns
  # FieldElement containing the account address
  FieldElement controller_address(ControllerAccount *controller);

  # Gets account chain ID
  #
  # # Parameters
  # * `account` - Pointer to Account
  #
  # # Returns
  # FieldElement containing the chain ID
  FieldElement controller_chain_id(ControllerAccount *controller);

  # Gets account nonce
  #
  # # Parameters
  # * `account` - Pointer to Account
  #
  # # Returns
  # Result containing FieldElement nonce or error
  ResultFieldElement controller_nonce(ControllerAccount *controller);

  # Executes raw transaction
  #
  # # Parameters
  # * `account` - Pointer to Account
  # * `calldata` - Array of Call structs
  # * `calldata_len` - Length of calldata array
  #
  # # Returns
  # Result containing transaction hash as FieldElement or error
  ResultFieldElement controller_execute_raw(ControllerAccount *controller,
                                            const Call *calldata,
                                            uintptr_t calldata_len);

  # Executes a transaction from outside (paymaster)
  #
  # # Parameters
  # * `account` - Pointer to Account
  # * `calldata` - Array of Call structs
  # * `calldata_len` - Length of calldata array
  #
  # # Returns
  # Result containing transaction hash as FieldElement or error
  ResultFieldElement controller_execute_from_outside(ControllerAccount *controller,
                                                     const Call *calldata,
                                                     uintptr_t calldata_len);

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

  # Retrieves controllers for the given contract addresses
  #
  # # Parameters
  # * `client` - Pointer to ToriiClient instance
  # * `contract_addresses` - Array of contract addresses. If empty, all controllers will be
  #   returned.
  #
  # # Returns
  # Result containing controllers or error
  ResultCArrayController client_controllers(ToriiClient *client,
                                            const FieldElement *contract_addresses,
                                            uintptr_t contract_addresses_len);

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
  # WorldMetadata structure containing world information
  ResultWorldMetadata client_metadata(ToriiClient *client);

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
                                               COptionClause clause);

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
                                                    void (*callback)(FieldElement, CArrayStruct));

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
                                                      COptionClause clause);

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
  ResultPageToken client_tokens(ToriiClient *client,
                                const FieldElement *contract_addresses,
                                uintptr_t contract_addresses_len,
                                const U256 *token_ids,
                                uintptr_t token_ids_len,
                                uint32_t limit,
                                COptionc_char cursor);

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
  ResultPageTokenBalance client_token_balances(ToriiClient *client,
                                               const FieldElement *contract_addresses,
                                               uintptr_t contract_addresses_len,
                                               const FieldElement *account_addresses,
                                               uintptr_t account_addresses_len,
                                               const U256 *token_ids,
                                               uintptr_t token_ids_len,
                                               uint32_t limit,
                                               COptionc_char cursor);

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
