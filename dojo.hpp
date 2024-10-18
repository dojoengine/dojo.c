#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

namespace dojo_bindings {

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
};

enum class LogicalOperator {
  And,
  Or,
};

enum class PatternMatching {
  FixedLen = 0,
  VariableLen = 1,
};

struct Account;

struct Provider;

struct Subscription;

struct ToriiClient;

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
#if !defined(TARGET_POINTER_WIDTH_32)
    U256,
#endif
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

#if !defined(TARGET_POINTER_WIDTH_32)
  struct U256_Body {
    uint64_t _0[4];
  };
#endif

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
#if !defined(TARGET_POINTER_WIDTH_32)
    U256_Body u256;
#endif
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

#if !defined(TARGET_POINTER_WIDTH_32)
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
#endif

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
    Primitive,
    String,
  };

  struct Primitive_Body {
    Primitive _0;
  };

  struct String_Body {
    const char *_0;
  };

  Tag tag;
  union {
    Primitive_Body primitive;
    String_Body string;
  };

  static MemberValue Primitive(const Primitive &_0) {
    MemberValue result;
    ::new (&result.primitive._0) (Primitive)(_0);
    result.tag = Tag::Primitive;
    return result;
  }

  bool IsPrimitive() const {
    return tag == Tag::Primitive;
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

struct Query {
  uint32_t limit;
  uint32_t offset;
  COption<Clause> clause;
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

struct Event {
  CArray<FieldElement> keys;
  CArray<FieldElement> data;
  FieldElement transaction_hash;
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

extern "C" {

Result<ToriiClient*> client_new(const char *torii_url,
                                const char *rpc_url,
                                const char *libp2p_relay_url,
                                FieldElement world);

void client_set_logger(ToriiClient *client, void (*logger)(const char*));

Result<CArray<uint8_t>> client_publish_message(ToriiClient *client,
                                               const char *message,
                                               const FieldElement *signature_felts,
                                               uintptr_t signature_felts_len);

Result<CArray<Entity>> client_entities(ToriiClient *client, const Query *query);

Result<CArray<Entity>> client_event_messages(ToriiClient *client, const Query *query);

WorldMetadata client_metadata(ToriiClient *client);

Result<Subscription*> client_on_entity_state_update(ToriiClient *client,
                                                    const EntityKeysClause *clauses,
                                                    uintptr_t clauses_len,
                                                    void (*callback)(FieldElement, CArray<Struct>));

Result<bool> client_update_entity_subscription(ToriiClient *client,
                                               Subscription *subscription,
                                               const EntityKeysClause *clauses,
                                               uintptr_t clauses_len);

Result<Subscription*> client_on_event_message_update(ToriiClient *client,
                                                     const EntityKeysClause *clauses,
                                                     uintptr_t clauses_len,
                                                     void (*callback)(FieldElement, CArray<Struct>));

Result<bool> client_update_event_message_subscription(ToriiClient *client,
                                                      Subscription *subscription,
                                                      const EntityKeysClause *clauses,
                                                      uintptr_t clauses_len);

Result<Subscription*> client_on_starknet_event(ToriiClient *client,
                                               const EntityKeysClause *clauses,
                                               uintptr_t clauses_len,
                                               void (*callback)(CArray<Event>));

Result<Subscription*> on_indexer_update(ToriiClient *client,
                                        const FieldElement *contract_address,
                                        void (*callback)(IndexerUpdate));

Result<CArray<FieldElement>> bytearray_serialize(const char *str);

Result<const char*> bytearray_deserialize(const FieldElement *felts, uintptr_t felts_len);

FieldElement poseidon_hash(const FieldElement *felts, uintptr_t felts_len);

Result<FieldElement> get_selector_from_name(const char *name);

FieldElement get_selector_from_tag(const char *tag);

FieldElement starknet_keccak(const uint8_t *bytes, uintptr_t bytes_len);

Result<FieldElement> cairo_short_string_to_felt(const char *str);

Result<const char*> parse_cairo_short_string(FieldElement felt);

Result<FieldElement> typed_data_encode(const char *typed_data, FieldElement address);

FieldElement signing_key_new();

Result<Signature> signing_key_sign(FieldElement private_key, FieldElement hash);

FieldElement verifying_key_new(FieldElement signing_key);

Result<bool> verifying_key_verify(FieldElement verifying_key,
                                  FieldElement hash,
                                  Signature signature);

Result<Provider*> provider_new(const char *rpc_url);

Result<Account*> account_new(Provider *rpc, FieldElement private_key, const char *address);

Result<CArray<FieldElement>> starknet_call(Provider *provider, Call call, BlockId block_id);

Result<Account*> account_deploy_burner(Provider *provider,
                                       Account *master_account,
                                       FieldElement signing_key);

FieldElement account_address(Account *account);

FieldElement account_chain_id(Account *account);

void account_set_block_id(Account *account, BlockId block_id);

Result<FieldElement> account_nonce(Account *account);

Result<FieldElement> account_execute_raw(Account *account,
                                         const Call *calldata,
                                         uintptr_t calldata_len);

Result<bool> wait_for_transaction(Provider *rpc, FieldElement txn_hash);

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

} // extern "C"

} // namespace dojo_bindings
