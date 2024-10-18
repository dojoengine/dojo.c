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

  cdef enum LogicalOperator:
    And,
    Or,

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

  cdef enum MemberValue_Tag:
    Primitive,
    String,

  cdef struct MemberValue:
    MemberValue_Tag tag;
    Primitive primitive;
    const char *string;

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

  cdef struct Query:
    uint32_t limit;
    uint32_t offset;
    COptionClause clause;

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

  cdef struct CArrayEvent:
    Event *data;
    uintptr_t data_len;

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

  ResultToriiClient client_new(const char *torii_url,
                               const char *rpc_url,
                               const char *libp2p_relay_url,
                               FieldElement world);

  void client_set_logger(ToriiClient *client, void (*logger)(const char*));

  ResultCArrayu8 client_publish_message(ToriiClient *client,
                                        const char *message,
                                        const FieldElement *signature_felts,
                                        uintptr_t signature_felts_len);

  ResultCArrayEntity client_entities(ToriiClient *client, const Query *query);

  ResultCArrayEntity client_event_messages(ToriiClient *client, const Query *query);

  WorldMetadata client_metadata(ToriiClient *client);

  ResultSubscription client_on_entity_state_update(ToriiClient *client,
                                                   const EntityKeysClause *clauses,
                                                   uintptr_t clauses_len,
                                                   void (*callback)(FieldElement, CArrayStruct));

  Resultbool client_update_entity_subscription(ToriiClient *client,
                                               Subscription *subscription,
                                               const EntityKeysClause *clauses,
                                               uintptr_t clauses_len);

  ResultSubscription client_on_event_message_update(ToriiClient *client,
                                                    const EntityKeysClause *clauses,
                                                    uintptr_t clauses_len,
                                                    void (*callback)(FieldElement, CArrayStruct));

  Resultbool client_update_event_message_subscription(ToriiClient *client,
                                                      Subscription *subscription,
                                                      const EntityKeysClause *clauses,
                                                      uintptr_t clauses_len);

  ResultSubscription client_on_starknet_event(ToriiClient *client,
                                              const EntityKeysClause *clauses,
                                              uintptr_t clauses_len,
                                              void (*callback)(CArrayEvent));

  ResultSubscription on_indexer_update(ToriiClient *client,
                                       const FieldElement *contract_address,
                                       void (*callback)(IndexerUpdate));

  ResultCArrayFieldElement bytearray_serialize(const char *str);

  Resultc_char bytearray_deserialize(const FieldElement *felts, uintptr_t felts_len);

  FieldElement poseidon_hash(const FieldElement *felts, uintptr_t felts_len);

  ResultFieldElement get_selector_from_name(const char *name);

  FieldElement get_selector_from_tag(const char *tag);

  FieldElement starknet_keccak(const uint8_t *bytes, uintptr_t bytes_len);

  ResultFieldElement cairo_short_string_to_felt(const char *str);

  Resultc_char parse_cairo_short_string(FieldElement felt);

  ResultFieldElement typed_data_encode(const char *typed_data, FieldElement address);

  FieldElement signing_key_new();

  ResultSignature signing_key_sign(FieldElement private_key, FieldElement hash);

  FieldElement verifying_key_new(FieldElement signing_key);

  Resultbool verifying_key_verify(FieldElement verifying_key,
                                  FieldElement hash,
                                  Signature signature);

  ResultProvider provider_new(const char *rpc_url);

  ResultAccount account_new(Provider *rpc, FieldElement private_key, const char *address);

  ResultCArrayFieldElement starknet_call(Provider *provider, Call call, BlockId block_id);

  ResultAccount account_deploy_burner(Provider *provider,
                                      Account *master_account,
                                      FieldElement signing_key);

  FieldElement account_address(Account *account);

  FieldElement account_chain_id(Account *account);

  void account_set_block_id(Account *account, BlockId block_id);

  ResultFieldElement account_nonce(Account *account);

  ResultFieldElement account_execute_raw(Account *account,
                                         const Call *calldata,
                                         uintptr_t calldata_len);

  Resultbool wait_for_transaction(Provider *rpc, FieldElement txn_hash);

  FieldElement hash_get_contract_address(FieldElement class_hash,
                                         FieldElement salt,
                                         const FieldElement *constructor_calldata,
                                         uintptr_t constructor_calldata_len,
                                         FieldElement deployer_address);

  void subscription_cancel(Subscription *subscription);

  void client_free(ToriiClient *t);

  void provider_free(Provider *rpc);

  void model_free(Struct *model);

  void account_free(Account *account);

  void ty_free(Ty *ty);

  void entity_free(Entity *entity);

  void error_free(Error *error);

  void world_metadata_free(WorldMetadata *metadata);

  void carray_free(void *data, uintptr_t data_len);

  void string_free(char *string);
