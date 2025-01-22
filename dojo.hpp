#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

namespace dojo_bindings {

struct ToriiClient;
struct Ty;
struct Query;
struct Subscription;
struct EntityKeysClause;
struct Provider;
struct Account;

enum class BlockTag {
  Latest,
  Pending,
};

enum class ComparisonOperator {
  Eq,
  Neq,
  Gt,
  Gte,
  Lt,
  Lte,
  In,
  NotIn,
};

enum class LogicalOperator {
  And,
  Or,
};

enum class OrderDirection {
  Asc,
  Desc,
};

enum class PatternMatching {
  FixedLen = 0,
  VariableLen = 1,
};

struct Error {
  char *message;
};

template<typename T>
struct Result {
  enum class Tag {
    Ok,
    Err,
  };

  struct Ok_Body {
    T _0;
  };

  struct Err_Body {
    Error _0;
  };

  Tag tag;
  union {
    Ok_Body ok;
    Err_Body err;
  };

  static Result Ok(const T &_0) {
    Result result;
    ::new (&result.ok._0) (T)(_0);
    result.tag = Tag::Ok;
    return result;
  }

  bool IsOk() const {
    return tag == Tag::Ok;
  }

  static Result Err(const Error &_0) {
    Result result;
    ::new (&result.err._0) (Error)(_0);
    result.tag = Tag::Err;
    return result;
  }

  bool IsErr() const {
    return tag == Tag::Err;
  }
};

struct FieldElement {
  uint8_t data[32];
};

template<typename T>
struct CArray {
  T *data;
  uintptr_t data_len;
};

struct Member {
  const char *name;
  Ty *ty;
  bool key;
};

struct Struct {
  const char *name;
  CArray<Member> children;
};

struct Entity {
  FieldElement hashed_keys;
  CArray<Struct> models;
};

struct Primitive {
  enum class Tag {
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
#if defined(TARGET_POINTER_WIDTH_32)
    U256,
#endif
    USize,
    Bool,
    Felt252,
    ClassHash,
    ContractAddress,
  };

  struct I8_Body {
    int8_t _0;
  };

  struct I16_Body {
    int16_t _0;
  };

  struct I32_Body {
    int32_t _0;
  };

  struct I64_Body {
    int64_t _0;
  };

  struct I128_Body {
    uint8_t _0[16];
  };

  struct U8_Body {
    uint8_t _0;
  };

  struct U16_Body {
    uint16_t _0;
  };

  struct U32_Body {
    uint32_t _0;
  };

  struct U64_Body {
    uint64_t _0;
  };

  struct U128_Body {
    uint8_t _0[16];
  };

  struct U256_Body {
    uint64_t _0[4];
  };

#if defined(TARGET_POINTER_WIDTH_32)
  struct U256_Body {
    uint32_t _0[8];
  };
#endif

  struct USize_Body {
    uint32_t _0;
  };

  struct Bool_Body {
    bool _0;
  };

  struct Felt252_Body {
    FieldElement _0;
  };

  struct ClassHash_Body {
    FieldElement _0;
  };

  struct ContractAddress_Body {
    FieldElement _0;
  };

  Tag tag;
  union {
    I8_Body i8;
    I16_Body i16;
    I32_Body i32;
    I64_Body i64;
    I128_Body i128;
    U8_Body u8;
    U16_Body u16;
    U32_Body u32;
    U64_Body u64;
    U128_Body u128;
    U256_Body u256;
#if defined(TARGET_POINTER_WIDTH_32)
    U256_Body u256;
#endif
    USize_Body u_size;
    Bool_Body bool_;
    Felt252_Body felt252;
    ClassHash_Body class_hash;
    ContractAddress_Body contract_address;
  };

  static Primitive I8(const int8_t &_0) {
    Primitive result;
    ::new (&result.i8._0) (int8_t)(_0);
    result.tag = Tag::I8;
    return result;
  }

  bool IsI8() const {
    return tag == Tag::I8;
  }

  static Primitive I16(const int16_t &_0) {
    Primitive result;
    ::new (&result.i16._0) (int16_t)(_0);
    result.tag = Tag::I16;
    return result;
  }

  bool IsI16() const {
    return tag == Tag::I16;
  }

  static Primitive I32(const int32_t &_0) {
    Primitive result;
    ::new (&result.i32._0) (int32_t)(_0);
    result.tag = Tag::I32;
    return result;
  }

  bool IsI32() const {
    return tag == Tag::I32;
  }

  static Primitive I64(const int64_t &_0) {
    Primitive result;
    ::new (&result.i64._0) (int64_t)(_0);
    result.tag = Tag::I64;
    return result;
  }

  bool IsI64() const {
    return tag == Tag::I64;
  }

  static Primitive I128(const uint8_t (&_0)[16]) {
    Primitive result;
    for (int i = 0; i < 16; i++) {
      ::new (&result.i128._0[i]) (uint8_t)(_0[i]);
    }
    result.tag = Tag::I128;
    return result;
  }

  bool IsI128() const {
    return tag == Tag::I128;
  }

  static Primitive U8(const uint8_t &_0) {
    Primitive result;
    ::new (&result.u8._0) (uint8_t)(_0);
    result.tag = Tag::U8;
    return result;
  }

  bool IsU8() const {
    return tag == Tag::U8;
  }

  static Primitive U16(const uint16_t &_0) {
    Primitive result;
    ::new (&result.u16._0) (uint16_t)(_0);
    result.tag = Tag::U16;
    return result;
  }

  bool IsU16() const {
    return tag == Tag::U16;
  }

  static Primitive U32(const uint32_t &_0) {
    Primitive result;
    ::new (&result.u32._0) (uint32_t)(_0);
    result.tag = Tag::U32;
    return result;
  }

  bool IsU32() const {
    return tag == Tag::U32;
  }

  static Primitive U64(const uint64_t &_0) {
    Primitive result;
    ::new (&result.u64._0) (uint64_t)(_0);
    result.tag = Tag::U64;
    return result;
  }

  bool IsU64() const {
    return tag == Tag::U64;
  }

  static Primitive U128(const uint8_t (&_0)[16]) {
    Primitive result;
    for (int i = 0; i < 16; i++) {
      ::new (&result.u128._0[i]) (uint8_t)(_0[i]);
    }
    result.tag = Tag::U128;
    return result;
  }

  bool IsU128() const {
    return tag == Tag::U128;
  }

  static Primitive U256(const uint64_t (&_0)[4]) {
    Primitive result;
    for (int i = 0; i < 4; i++) {
      ::new (&result.u256._0[i]) (uint64_t)(_0[i]);
    }
    result.tag = Tag::U256;
    return result;
  }

  bool IsU256() const {
    return tag == Tag::U256;
  }

#if defined(TARGET_POINTER_WIDTH_32)
  static Primitive U256(const uint32_t (&_0)[8]) {
    Primitive result;
    for (int i = 0; i < 8; i++) {
      ::new (&result.u256._0[i]) (uint32_t)(_0[i]);
    }
    result.tag = Tag::U256;
    return result;
  }

  bool IsU256() const {
    return tag == Tag::U256;
  }
#endif

  static Primitive USize(const uint32_t &_0) {
    Primitive result;
    ::new (&result.u_size._0) (uint32_t)(_0);
    result.tag = Tag::USize;
    return result;
  }

  bool IsUSize() const {
    return tag == Tag::USize;
  }

  static Primitive Bool(const bool &_0) {
    Primitive result;
    ::new (&result.bool_._0) (bool)(_0);
    result.tag = Tag::Bool;
    return result;
  }

  bool IsBool() const {
    return tag == Tag::Bool;
  }

  static Primitive Felt252(const FieldElement &_0) {
    Primitive result;
    ::new (&result.felt252._0) (FieldElement)(_0);
    result.tag = Tag::Felt252;
    return result;
  }

  bool IsFelt252() const {
    return tag == Tag::Felt252;
  }

  static Primitive ClassHash(const FieldElement &_0) {
    Primitive result;
    ::new (&result.class_hash._0) (FieldElement)(_0);
    result.tag = Tag::ClassHash;
    return result;
  }

  bool IsClassHash() const {
    return tag == Tag::ClassHash;
  }

  static Primitive ContractAddress(const FieldElement &_0) {
    Primitive result;
    ::new (&result.contract_address._0) (FieldElement)(_0);
    result.tag = Tag::ContractAddress;
    return result;
  }

  bool IsContractAddress() const {
    return tag == Tag::ContractAddress;
  }
};

struct EnumOption {
  const char *name;
  Ty *ty;
};

struct Enum {
  const char *name;
  uint8_t option;
  CArray<EnumOption> options;
};

struct Ty {
  enum class Tag {
    Primitive_,
    Struct_,
    Enum_,
    Tuple_,
    Array_,
    ByteArray,
  };

  struct Primitive__Body {
    Primitive _0;
  };

  struct Struct__Body {
    Struct _0;
  };

  struct Enum__Body {
    Enum _0;
  };

  struct Tuple__Body {
    CArray<Ty> _0;
  };

  struct Array__Body {
    CArray<Ty> _0;
  };

  struct ByteArray_Body {
    const char *_0;
  };

  Tag tag;
  union {
    Primitive__Body primitive;
    Struct__Body struct_;
    Enum__Body enum_;
    Tuple__Body tuple;
    Array__Body array;
    ByteArray_Body byte_array;
  };

  static Ty Primitive_(const Primitive &_0) {
    Ty result;
    ::new (&result.primitive._0) (Primitive)(_0);
    result.tag = Tag::Primitive_;
    return result;
  }

  bool IsPrimitive_() const {
    return tag == Tag::Primitive_;
  }

  static Ty Struct_(const Struct &_0) {
    Ty result;
    ::new (&result.struct_._0) (Struct)(_0);
    result.tag = Tag::Struct_;
    return result;
  }

  bool IsStruct_() const {
    return tag == Tag::Struct_;
  }

  static Ty Enum_(const Enum &_0) {
    Ty result;
    ::new (&result.enum_._0) (Enum)(_0);
    result.tag = Tag::Enum_;
    return result;
  }

  bool IsEnum_() const {
    return tag == Tag::Enum_;
  }

  static Ty Tuple_(const CArray<Ty> &_0) {
    Ty result;
    ::new (&result.tuple._0) (CArray<Ty>)(_0);
    result.tag = Tag::Tuple_;
    return result;
  }

  bool IsTuple_() const {
    return tag == Tag::Tuple_;
  }

  static Ty Array_(const CArray<Ty> &_0) {
    Ty result;
    ::new (&result.array._0) (CArray<Ty>)(_0);
    result.tag = Tag::Array_;
    return result;
  }

  bool IsArray_() const {
    return tag == Tag::Array_;
  }

  static Ty ByteArray(const char *const &_0) {
    Ty result;
    ::new (&result.byte_array._0) (const char*)(_0);
    result.tag = Tag::ByteArray;
    return result;
  }

  bool IsByteArray() const {
    return tag == Tag::ByteArray;
  }
};

struct ModelMetadata {
  Ty schema;
  const char *namespace_;
  const char *name;
  uint32_t packed_size;
  uint32_t unpacked_size;
  FieldElement class_hash;
  FieldElement contract_address;
  CArray<FieldElement> layout;
};

template<typename K, typename V>
struct CHashItem {
  K key;
  V value;
};

struct WorldMetadata {
  FieldElement world_address;
  CArray<CHashItem<FieldElement, ModelMetadata>> models;
};

struct Event {
  CArray<FieldElement> keys;
  CArray<FieldElement> data;
  FieldElement transaction_hash;
};

struct Token {
  FieldElement contract_address;
  const char *name;
  const char *symbol;
  uint8_t decimals;
  const char *metadata;
};

struct TokenBalance {
  uint64_t balance[4];
#if defined(TARGET_POINTER_WIDTH_32)
  uint32_t balance[8]
#endif
  ;
  FieldElement account_address;
  FieldElement contract_address;
  const char *token_id;
};

struct IndexerUpdate {
  int64_t head;
  int64_t tps;
  int64_t last_block_timestamp;
  FieldElement contract_address;
};

struct Signature {
  /// The `r` value of a signature
  FieldElement r;
  /// The `s` value of a signature
  FieldElement s;
};

struct Call {
  FieldElement to;
  const char *selector;
  CArray<FieldElement> calldata;
};

/// Block hash, number or tag
struct BlockId {
  enum class Tag {
    Hash,
    Number,
    BlockTag_,
  };

  struct Hash_Body {
    FieldElement _0;
  };

  struct Number_Body {
    uint64_t _0;
  };

  struct BlockTag__Body {
    BlockTag _0;
  };

  Tag tag;
  union {
    Hash_Body hash;
    Number_Body number;
    BlockTag__Body block_tag;
  };

  static BlockId Hash(const FieldElement &_0) {
    BlockId result;
    ::new (&result.hash._0) (FieldElement)(_0);
    result.tag = Tag::Hash;
    return result;
  }

  bool IsHash() const {
    return tag == Tag::Hash;
  }

  static BlockId Number(const uint64_t &_0) {
    BlockId result;
    ::new (&result.number._0) (uint64_t)(_0);
    result.tag = Tag::Number;
    return result;
  }

  bool IsNumber() const {
    return tag == Tag::Number;
  }

  static BlockId BlockTag_(const BlockTag &_0) {
    BlockId result;
    ::new (&result.block_tag._0) (BlockTag)(_0);
    result.tag = Tag::BlockTag_;
    return result;
  }

  bool IsBlockTag_() const {
    return tag == Tag::BlockTag_;
  }
};

template<typename T>
struct COption {
  enum class Tag {
    Some,
    None,
  };

  struct Some_Body {
    T _0;
  };

  Tag tag;
  union {
    Some_Body some;
  };

  static COption Some(const T &_0) {
    COption result;
    ::new (&result.some._0) (T)(_0);
    result.tag = Tag::Some;
    return result;
  }

  bool IsSome() const {
    return tag == Tag::Some;
  }

  static COption None() {
    COption result;
    result.tag = Tag::None;
    return result;
  }

  bool IsNone() const {
    return tag == Tag::None;
  }
};

struct KeysClause {
  CArray<COption<FieldElement>> keys;
  PatternMatching pattern_matching;
  CArray<const char*> models;
};

struct MemberValue {
  enum class Tag {
    PrimitiveValue,
    String,
    List,
  };

  struct PrimitiveValue_Body {
    Primitive _0;
  };

  struct String_Body {
    const char *_0;
  };

  struct List_Body {
    CArray<MemberValue> _0;
  };

  Tag tag;
  union {
    PrimitiveValue_Body primitive_value;
    String_Body string;
    List_Body list;
  };

  static MemberValue PrimitiveValue(const Primitive &_0) {
    MemberValue result;
    ::new (&result.primitive_value._0) (Primitive)(_0);
    result.tag = Tag::PrimitiveValue;
    return result;
  }

  bool IsPrimitiveValue() const {
    return tag == Tag::PrimitiveValue;
  }

  static MemberValue String(const char *const &_0) {
    MemberValue result;
    ::new (&result.string._0) (const char*)(_0);
    result.tag = Tag::String;
    return result;
  }

  bool IsString() const {
    return tag == Tag::String;
  }

  static MemberValue List(const CArray<MemberValue> &_0) {
    MemberValue result;
    ::new (&result.list._0) (CArray<MemberValue>)(_0);
    result.tag = Tag::List;
    return result;
  }

  bool IsList() const {
    return tag == Tag::List;
  }
};

struct MemberClause {
  const char *model;
  const char *member;
  ComparisonOperator operator_;
  MemberValue value;
};

struct CompositeClause {
  LogicalOperator operator_;
  CArray<Clause> clauses;
};

struct Clause {
  enum class Tag {
    Keys,
    CMember,
    Composite,
  };

  struct Keys_Body {
    KeysClause _0;
  };

  struct CMember_Body {
    MemberClause _0;
  };

  struct Composite_Body {
    CompositeClause _0;
  };

  Tag tag;
  union {
    Keys_Body keys;
    CMember_Body c_member;
    Composite_Body composite;
  };

  static Clause Keys(const KeysClause &_0) {
    Clause result;
    ::new (&result.keys._0) (KeysClause)(_0);
    result.tag = Tag::Keys;
    return result;
  }

  bool IsKeys() const {
    return tag == Tag::Keys;
  }

  static Clause CMember(const MemberClause &_0) {
    Clause result;
    ::new (&result.c_member._0) (MemberClause)(_0);
    result.tag = Tag::CMember;
    return result;
  }

  bool IsCMember() const {
    return tag == Tag::CMember;
  }

  static Clause Composite(const CompositeClause &_0) {
    Clause result;
    ::new (&result.composite._0) (CompositeClause)(_0);
    result.tag = Tag::Composite;
    return result;
  }

  bool IsComposite() const {
    return tag == Tag::Composite;
  }
};

struct OrderBy {
  const char *model;
  const char *member;
  OrderDirection direction;
};

struct Query {
  uint32_t limit;
  uint32_t offset;
  COption<Clause> clause;
  bool dont_include_hashed_keys;
  CArray<OrderBy> order_by;
  CArray<const char*> entity_models;
  uint64_t entity_updated_after;
};

struct EntityKeysClause {
  enum class Tag {
    HashedKeys,
    EntityKeys,
  };

  struct HashedKeys_Body {
    CArray<FieldElement> _0;
  };

  struct EntityKeys_Body {
    KeysClause _0;
  };

  Tag tag;
  union {
    HashedKeys_Body hashed_keys;
    EntityKeys_Body entity_keys;
  };

  static EntityKeysClause HashedKeys(const CArray<FieldElement> &_0) {
    EntityKeysClause result;
    ::new (&result.hashed_keys._0) (CArray<FieldElement>)(_0);
    result.tag = Tag::HashedKeys;
    return result;
  }

  bool IsHashedKeys() const {
    return tag == Tag::HashedKeys;
  }

  static EntityKeysClause EntityKeys(const KeysClause &_0) {
    EntityKeysClause result;
    ::new (&result.entity_keys._0) (KeysClause)(_0);
    result.tag = Tag::EntityKeys;
    return result;
  }

  bool IsEntityKeys() const {
    return tag == Tag::EntityKeys;
  }
};

extern "C" {

/// Creates a new Torii client instance
///
/// # Parameters
/// * `torii_url` - URL of the Torii server
/// * `rpc_url` - URL of the Starknet RPC endpoint
/// * `libp2p_relay_url` - URL of the libp2p relay server
/// * `world` - World address as a FieldElement
///
/// # Returns
/// Result containing pointer to new ToriiClient instance or error
Result<ToriiClient*> client_new(const char *torii_url,
                                const char *rpc_url,
                                const char *libp2p_relay_url,
                                FieldElement world);

/// Sets a logger callback function for the client
///
/// # Parameters
/// * `client` - Pointer to ToriiClient instance
/// * `logger` - Callback function that takes a C string parameter
void client_set_logger(ToriiClient *client, void (*logger)(const char*));

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
Result<CArray<uint8_t>> client_publish_message(ToriiClient *client,
                                               const char *message,
                                               const FieldElement *signature_felts,
                                               uintptr_t signature_felts_len);

/// Queries entities matching given criteria
///
/// # Parameters
/// * `client` - Pointer to ToriiClient instance
/// * `query` - Query parameters
///
/// # Returns
/// Result containing array of matching entities or error
Result<CArray<Entity>> client_entities(ToriiClient *client, const Query *query);

/// Retrieves event messages matching the given query
///
/// # Parameters
/// * `client` - Pointer to ToriiClient instance
/// * `query` - Query parameters
/// * `historical` - Whether to include historical messages
///
/// # Returns
/// Result containing array of matching event message entities or error
Result<CArray<Entity>> client_event_messages(ToriiClient *client,
                                             const Query *query,
                                             bool historical);

/// Gets the world metadata for the client
///
/// # Parameters
/// * `client` - Pointer to ToriiClient instance
///
/// # Returns
/// WorldMetadata structure containing world information
WorldMetadata client_metadata(ToriiClient *client);

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
Result<Subscription*> client_on_entity_state_update(ToriiClient *client,
                                                    const EntityKeysClause *clauses,
                                                    uintptr_t clauses_len,
                                                    void (*callback)(FieldElement, CArray<Struct>));

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
Result<bool> client_update_entity_subscription(ToriiClient *client,
                                               Subscription *subscription,
                                               const EntityKeysClause *clauses,
                                               uintptr_t clauses_len);

/// Subscribes to event message updates
///
/// # Parameters
/// * `client` - Pointer to ToriiClient instance
/// * `clauses` - Array of entity key clauses to filter updates
/// * `clauses_len` - Length of clauses array
/// * `historical` - Whether to include historical messages
/// * `callback` - Function called when updates occur
///
/// # Returns
/// Result containing pointer to Subscription or error
Result<Subscription*> client_on_event_message_update(ToriiClient *client,
                                                     const EntityKeysClause *clauses,
                                                     uintptr_t clauses_len,
                                                     bool historical,
                                                     void (*callback)(FieldElement, CArray<Struct>));

/// Updates an existing event message subscription
///
/// # Parameters
/// * `client` - Pointer to ToriiClient instance
/// * `subscription` - Pointer to existing Subscription
/// * `clauses` - New array of entity key clauses
/// * `clauses_len` - Length of new clauses array
/// * `historical` - Whether to include historical messages
///
/// # Returns
/// Result containing success boolean or error
Result<bool> client_update_event_message_subscription(ToriiClient *client,
                                                      Subscription *subscription,
                                                      const EntityKeysClause *clauses,
                                                      uintptr_t clauses_len,
                                                      bool historical);

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
Result<Subscription*> client_on_starknet_event(ToriiClient *client,
                                               const EntityKeysClause *clauses,
                                               uintptr_t clauses_len,
                                               void (*callback)(Event));

/// Retrieves token information for given contract addresses
///
/// # Parameters
/// * `client` - Pointer to ToriiClient instance
/// * `contract_addresses` - Array of contract addresses
/// * `contract_addresses_len` - Length of addresses array
///
/// # Returns
/// Result containing array of Token information or error
Result<CArray<Token>> client_tokens(ToriiClient *client,
                                    const FieldElement *contract_addresses,
                                    uintptr_t contract_addresses_len);

/// Gets token balances for given accounts and contracts
///
/// # Parameters
/// * `client` - Pointer to ToriiClient instance
/// * `contract_addresses` - Array of contract addresses
/// * `contract_addresses_len` - Length of contract addresses array
/// * `account_addresses` - Array of account addresses
/// * `account_addresses_len` - Length of account addresses array
///
/// # Returns
/// Result containing array of TokenBalance information or error
Result<CArray<TokenBalance>> client_token_balances(ToriiClient *client,
                                                   const FieldElement *contract_addresses,
                                                   uintptr_t contract_addresses_len,
                                                   const FieldElement *account_addresses,
                                                   uintptr_t account_addresses_len);

/// Subscribes to indexer updates
///
/// # Parameters
/// * `client` - Pointer to ToriiClient instance
/// * `contract_address` - Optional contract address to filter updates
/// * `callback` - Function called when updates occur
///
/// # Returns
/// Result containing pointer to Subscription or error
Result<Subscription*> on_indexer_update(ToriiClient *client,
                                        const FieldElement *contract_address,
                                        void (*callback)(IndexerUpdate));

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
Result<Subscription*> client_on_token_balance_update(ToriiClient *client,
                                                     const FieldElement *contract_addresses,
                                                     uintptr_t contract_addresses_len,
                                                     const FieldElement *account_addresses,
                                                     uintptr_t account_addresses_len,
                                                     void (*callback)(TokenBalance));

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
Result<bool> client_update_token_balance_subscription(ToriiClient *client,
                                                      Subscription *subscription,
                                                      const FieldElement *contract_addresses,
                                                      uintptr_t contract_addresses_len,
                                                      const FieldElement *account_addresses,
                                                      uintptr_t account_addresses_len);

/// Serializes a string into a byte array
///
/// # Parameters
/// * `str` - String to serialize
///
/// # Returns
/// Result containing array of FieldElements or error
Result<CArray<FieldElement>> bytearray_serialize(const char *str);

/// Deserializes field elements into a string
///
/// # Parameters
/// * `felts` - Array of field elements
/// * `felts_len` - Length of field elements array
///
/// # Returns
/// Result containing pointer to C string or error
Result<const char*> bytearray_deserialize(const FieldElement *felts, uintptr_t felts_len);

/// Computes Poseidon hash of field elements
///
/// # Parameters
/// * `felts` - Array of field elements
/// * `felts_len` - Length of array
///
/// # Returns
/// FieldElement containing the hash result
FieldElement poseidon_hash(const FieldElement *felts, uintptr_t felts_len);

/// Gets selector from name string
///
/// # Parameters
/// * `name` - Name to compute selector from
///
/// # Returns
/// Result containing FieldElement selector or error
Result<FieldElement> get_selector_from_name(const char *name);

/// Gets selector from tag string
///
/// # Parameters
/// * `tag` - Tag to compute selector from
///
/// # Returns
/// FieldElement containing the computed selector
FieldElement get_selector_from_tag(const char *tag);

/// Computes Starknet keccak hash of bytes
///
/// # Parameters
/// * `bytes` - Byte array to hash
/// * `bytes_len` - Length of byte array
///
/// # Returns
/// FieldElement containing the hash result
FieldElement starknet_keccak(const uint8_t *bytes, uintptr_t bytes_len);

/// Converts a short string to field element
///
/// # Parameters
/// * `str` - String to convert
///
/// # Returns
/// Result containing FieldElement or error
Result<FieldElement> cairo_short_string_to_felt(const char *str);

/// Parses a field element into a short string
///
/// # Parameters
/// * `felt` - FieldElement to parse
///
/// # Returns
/// Result containing pointer to C string or error
Result<const char*> parse_cairo_short_string(FieldElement felt);

/// Encodes typed data
///
/// # Parameters
/// * `typed_data` - JSON string of typed data
/// * `address` - Address as FieldElement
///
/// # Returns
/// Result containing encoded FieldElement or error
Result<FieldElement> typed_data_encode(const char *typed_data, FieldElement address);

/// Generates a new signing key
///
/// # Returns
/// FieldElement containing the new private key
FieldElement signing_key_new();

/// Signs a hash with a private key
///
/// # Parameters
/// * `private_key` - Private key as FieldElement
/// * `hash` - Hash to sign as FieldElement
///
/// # Returns
/// Result containing Signature or error
Result<Signature> signing_key_sign(FieldElement private_key, FieldElement hash);

/// Creates a verifying key from a signing key
///
/// # Parameters
/// * `signing_key` - Signing key as FieldElement
///
/// # Returns
/// FieldElement containing the verifying key
FieldElement verifying_key_new(FieldElement signing_key);

/// Verifies a signature
///
/// # Parameters
/// * `verifying_key` - Verifying key as FieldElement
/// * `hash` - Hash that was signed
/// * `signature` - Signature to verify
///
/// # Returns
/// Result containing verification success boolean or error
Result<bool> verifying_key_verify(FieldElement verifying_key,
                                  FieldElement hash,
                                  Signature signature);

/// Creates a new provider instance
///
/// # Parameters
/// * `rpc_url` - URL of the RPC endpoint
///
/// # Returns
/// Result containing pointer to Provider or error
Result<Provider*> provider_new(const char *rpc_url);

/// Creates a new account instance
///
/// # Parameters
/// * `rpc` - Pointer to Provider
/// * `private_key` - Private key as FieldElement
/// * `address` - Account address as string
///
/// # Returns
/// Result containing pointer to Account or error
Result<Account*> account_new(Provider *rpc, FieldElement private_key, const char *address);

/// Makes a Starknet call
///
/// # Parameters
/// * `provider` - Pointer to Provider
/// * `call` - Call parameters
/// * `block_id` - Block identifier
///
/// # Returns
/// Result containing array of FieldElements or error
Result<CArray<FieldElement>> starknet_call(Provider *provider, Call call, BlockId block_id);

/// Deploys a burner account
///
/// # Parameters
/// * `provider` - Pointer to Provider
/// * `master_account` - Pointer to master Account
/// * `signing_key` - Signing key for new account
///
/// # Returns
/// Result containing pointer to new Account or error
Result<Account*> account_deploy_burner(Provider *provider,
                                       Account *master_account,
                                       FieldElement signing_key);

/// Gets account address
///
/// # Parameters
/// * `account` - Pointer to Account
///
/// # Returns
/// FieldElement containing the account address
FieldElement account_address(Account *account);

/// Gets account chain ID
///
/// # Parameters
/// * `account` - Pointer to Account
///
/// # Returns
/// FieldElement containing the chain ID
FieldElement account_chain_id(Account *account);

/// Sets block ID for account
///
/// # Parameters
/// * `account` - Pointer to Account
/// * `block_id` - New block ID
void account_set_block_id(Account *account, BlockId block_id);

/// Gets account nonce
///
/// # Parameters
/// * `account` - Pointer to Account
///
/// # Returns
/// Result containing FieldElement nonce or error
Result<FieldElement> account_nonce(Account *account);

/// Executes raw transaction
///
/// # Parameters
/// * `account` - Pointer to Account
/// * `calldata` - Array of Call structs
/// * `calldata_len` - Length of calldata array
///
/// # Returns
/// Result containing transaction hash as FieldElement or error
Result<FieldElement> account_execute_raw(Account *account,
                                         const Call *calldata,
                                         uintptr_t calldata_len);

/// Waits for transaction completion
///
/// # Parameters
/// * `rpc` - Pointer to Provider
/// * `txn_hash` - Transaction hash as FieldElement
///
/// # Returns
/// Result containing success boolean or error
Result<bool> wait_for_transaction(Provider *rpc, FieldElement txn_hash);

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
FieldElement hash_get_contract_address(FieldElement class_hash,
                                       FieldElement salt,
                                       const FieldElement *constructor_calldata,
                                       uintptr_t constructor_calldata_len,
                                       FieldElement deployer_address);

/// Cancels a subscription
///
/// # Parameters
/// * `subscription` - Pointer to Subscription to cancel
void subscription_cancel(Subscription *subscription);

/// Frees a ToriiClient instance
///
/// # Parameters
/// * `t` - Pointer to ToriiClient to free
void client_free(ToriiClient *t);

/// Frees a Provider instance
///
/// # Parameters
/// * `rpc` - Pointer to Provider to free
void provider_free(Provider *rpc);

/// Frees a Model instance
///
/// # Parameters
/// * `model` - Pointer to Model to free
void model_free(Struct *model);

/// Frees an Account instance
///
/// # Parameters
/// * `account` - Pointer to Account to free
void account_free(Account *account);

/// Frees a Type instance
///
/// # Parameters
/// * `ty` - Pointer to Type to free
void ty_free(Ty *ty);

/// Frees an Entity instance
///
/// # Parameters
/// * `entity` - Pointer to Entity to free
void entity_free(Entity *entity);

/// Frees an Error instance
///
/// # Parameters
/// * `error` - Pointer to Error to free
void error_free(Error *error);

/// Frees a WorldMetadata instance
///
/// # Parameters
/// * `metadata` - Pointer to WorldMetadata to free
void world_metadata_free(WorldMetadata *metadata);

/// Frees a CArray instance
///
/// # Parameters
/// * `data` - Pointer to array data
/// * `data_len` - Length of array
void carray_free(void *data, uintptr_t data_len);

/// Frees a string
///
/// # Parameters
/// * `string` - Pointer to string to free
void string_free(char *string);

}  // extern "C"

}  // namespace dojo_bindings
