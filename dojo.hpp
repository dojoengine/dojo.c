#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

namespace dojo_bindings {

struct ToriiClient;
struct Policy;
struct ControllerAccount;
struct Call;
struct Ty;
struct Subscription;
struct Provider;
struct Account;

enum class BlockTag {
  Latest,
  PreConfirmed,
};

enum class CallType {
  Execute,
  ExecuteFromOutside,
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
  Contains,
  ContainsAll,
  ContainsAny,
  ArrayLengthEq,
  ArrayLengthGt,
  ArrayLengthLt,
};

enum class ContractType {
  WORLD,
  ERC20,
  ERC721,
  ERC1155,
  UDC,
  OTHER,
};

enum class LogicalOperator {
  And,
  Or,
};

enum class OrderDirection {
  Asc,
  Desc,
};

enum class PaginationDirection {
  Forward,
  Backward,
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

struct Message {
  const char *message;
  CArray<FieldElement> signature;
  FieldElement world_address;
};

struct Controller {
  FieldElement address;
  const char *username;
  uint64_t deployed_at_timestamp;
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

template<typename T>
struct Page {
  CArray<T> items;
  COption<const char*> next_cursor;
};

struct OrderBy {
  const char *field;
  OrderDirection direction;
};

struct Pagination {
  COption<const char*> cursor;
  COption<uint32_t> limit;
  PaginationDirection direction;
  CArray<OrderBy> order_by;
};

struct ControllerQuery {
  Pagination pagination;
  CArray<FieldElement> contract_addresses;
  CArray<const char*> usernames;
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
  FieldElement world_address;
  FieldElement hashed_keys;
  CArray<Struct> models;
  uint64_t created_at;
  uint64_t updated_at;
  uint64_t executed_at;
};

struct KeysClause {
  CArray<COption<FieldElement>> keys;
  PatternMatching pattern_matching;
  CArray<const char*> models;
};

struct U256 {
  uint8_t data[32];
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
    U256_,
    Bool,
    Felt252,
    ClassHash,
    ContractAddress,
    EthAddress,
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

  struct U256__Body {
    U256 _0;
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

  struct EthAddress_Body {
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
    U256__Body u256;
    Bool_Body bool_;
    Felt252_Body felt252;
    ClassHash_Body class_hash;
    ContractAddress_Body contract_address;
    EthAddress_Body eth_address;
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

  static Primitive U256_(const U256 &_0) {
    Primitive result;
    ::new (&result.u256._0) (U256)(_0);
    result.tag = Tag::U256_;
    return result;
  }

  bool IsU256_() const {
    return tag == Tag::U256_;
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

  static Primitive EthAddress(const FieldElement &_0) {
    Primitive result;
    ::new (&result.eth_address._0) (FieldElement)(_0);
    result.tag = Tag::EthAddress;
    return result;
  }

  bool IsEthAddress() const {
    return tag == Tag::EthAddress;
  }
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
    HashedKeys,
    Keys,
    CMember,
    Composite,
  };

  struct HashedKeys_Body {
    CArray<FieldElement> _0;
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
    HashedKeys_Body hashed_keys;
    Keys_Body keys;
    CMember_Body c_member;
    Composite_Body composite;
  };

  static Clause HashedKeys(const CArray<FieldElement> &_0) {
    Clause result;
    ::new (&result.hashed_keys._0) (CArray<FieldElement>)(_0);
    result.tag = Tag::HashedKeys;
    return result;
  }

  bool IsHashedKeys() const {
    return tag == Tag::HashedKeys;
  }

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
  CArray<FieldElement> world_addresses;
  Pagination pagination;
  COption<Clause> clause;
  bool no_hashed_keys;
  CArray<const char*> models;
  bool historical;
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

struct FixedSizeArray {
  CArray<Ty> array;
  uint32_t size;
};

struct Ty {
  enum class Tag {
    Primitive_,
    Struct_,
    Enum_,
    Tuple_,
    Array_,
    FixedSizeArray_,
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

  struct FixedSizeArray__Body {
    FixedSizeArray _0;
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
    FixedSizeArray__Body fixed_size_array;
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

  static Ty FixedSizeArray_(const FixedSizeArray &_0) {
    Ty result;
    ::new (&result.fixed_size_array._0) (FixedSizeArray)(_0);
    result.tag = Tag::FixedSizeArray_;
    return result;
  }

  bool IsFixedSizeArray_() const {
    return tag == Tag::FixedSizeArray_;
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

struct Model {
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
};

struct World {
  FieldElement world_address;
  CArray<Model> models;
};

struct TransactionCall {
  FieldElement contract_address;
  const char *entrypoint;
  CArray<FieldElement> calldata;
  CallType call_type;
  FieldElement caller_address;
};

struct Transaction {
  FieldElement transaction_hash;
  FieldElement sender_address;
  CArray<FieldElement> calldata;
  FieldElement max_fee;
  CArray<FieldElement> signature;
  FieldElement nonce;
  uint64_t block_number;
  const char *transaction_type;
  uint64_t block_timestamp;
  CArray<TransactionCall> calls;
  CArray<FieldElement> unique_models;
};

struct TransactionFilter {
  CArray<FieldElement> transaction_hashes;
  CArray<FieldElement> caller_addresses;
  CArray<FieldElement> contract_addresses;
  CArray<const char*> entrypoints;
  CArray<FieldElement> model_selectors;
  COption<uint64_t> from_block;
  COption<uint64_t> to_block;
};

struct TransactionQuery {
  COption<TransactionFilter> filter;
  Pagination pagination;
};

struct AggregationEntry {
  const char *id;
  const char *aggregator_id;
  const char *entity_id;
  U256 value;
  const char *display_value;
  uint64_t position;
  const char *model_id;
  uint64_t created_at;
  uint64_t updated_at;
};

struct AggregationQuery {
  CArray<const char*> aggregator_ids;
  CArray<const char*> entity_ids;
  Pagination pagination;
};

struct AchievementTask {
  const char *task_id;
  const char *description;
  uint32_t total;
  uint32_t total_completions;
  double completion_rate;
  uint64_t created_at;
};

struct Achievement {
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
  CArray<AchievementTask> tasks;
  const char *data;
  uint32_t total_completions;
  double completion_rate;
  uint64_t created_at;
  uint64_t updated_at;
};

struct AchievementQuery {
  CArray<FieldElement> world_addresses;
  CArray<const char*> namespaces;
  COption<bool> hidden;
  Pagination pagination;
};

struct PlayerAchievementStats {
  uint32_t total_points;
  uint32_t completed_achievements;
  uint32_t total_achievements;
  double completion_percentage;
  COption<uint64_t> last_achievement_at;
  uint64_t created_at;
  uint64_t updated_at;
};

struct TaskProgress {
  const char *task_id;
  uint32_t count;
  bool completed;
};

struct PlayerAchievementProgress {
  Achievement achievement;
  CArray<TaskProgress> task_progress;
  bool completed;
  double progress_percentage;
};

struct PlayerAchievementEntry {
  FieldElement player_address;
  PlayerAchievementStats stats;
  CArray<PlayerAchievementProgress> achievements;
};

struct PlayerAchievementQuery {
  CArray<FieldElement> world_addresses;
  CArray<const char*> namespaces;
  CArray<FieldElement> player_addresses;
  Pagination pagination;
};

struct AchievementProgression {
  const char *id;
  const char *achievement_id;
  const char *task_id;
  FieldElement world_address;
  const char *namespace_;
  FieldElement player_id;
  uint32_t count;
  bool completed;
  COption<uint64_t> completed_at;
  uint64_t created_at;
  uint64_t updated_at;
};

struct ActionCount {
  const char *action_name;
  uint32_t count;
};

struct Activity {
  const char *id;
  FieldElement world_address;
  const char *namespace_;
  FieldElement caller_address;
  uint64_t session_start;
  uint64_t session_end;
  uint32_t action_count;
  CArray<ActionCount> actions;
  uint64_t updated_at;
};

struct ActivityQuery {
  CArray<FieldElement> world_addresses;
  CArray<const char*> namespaces;
  CArray<FieldElement> caller_addresses;
  COption<uint64_t> from_time;
  COption<uint64_t> to_time;
  Pagination pagination;
};

struct Event {
  CArray<FieldElement> keys;
  CArray<FieldElement> data;
  FieldElement transaction_hash;
};

struct Token {
  FieldElement contract_address;
  COption<U256> token_id;
  const char *name;
  const char *symbol;
  uint8_t decimals;
  const char *metadata;
  COption<U256> total_supply;
};

struct AttributeFilter {
  const char *trait_name;
  const char *trait_value;
};

struct TokenQuery {
  CArray<FieldElement> contract_addresses;
  CArray<U256> token_ids;
  CArray<AttributeFilter> attribute_filters;
  Pagination pagination;
};

struct TokenBalance {
  U256 balance;
  FieldElement account_address;
  FieldElement contract_address;
  COption<U256> token_id;
};

struct TokenBalanceQuery {
  CArray<FieldElement> contract_addresses;
  CArray<FieldElement> account_addresses;
  CArray<U256> token_ids;
  Pagination pagination;
};

struct TokenContract {
  FieldElement contract_address;
  const char *name;
  const char *symbol;
  uint8_t decimals;
  const char *metadata;
  const char *token_metadata;
  COption<U256> total_supply;
};

struct TokenContractQuery {
  CArray<FieldElement> contract_addresses;
  CArray<ContractType> contract_types;
  Pagination pagination;
};

struct Contract {
  FieldElement contract_address;
  ContractType contract_type;
  COption<uint64_t> head;
  COption<uint64_t> tps;
  COption<uint64_t> last_block_timestamp;
  COption<FieldElement> last_pending_block_tx;
  uint64_t updated_at;
  uint64_t created_at;
};

struct ContractQuery {
  CArray<FieldElement> contract_addresses;
  CArray<ContractType> contract_types;
};

struct TokenTransfer {
  const char *id;
  FieldElement contract_address;
  FieldElement from_address;
  FieldElement to_address;
  U256 amount;
  COption<U256> token_id;
  uint64_t executed_at;
  COption<const char*> event_id;
};

struct TokenTransferQuery {
  CArray<FieldElement> contract_addresses;
  CArray<FieldElement> account_addresses;
  CArray<U256> token_ids;
  Pagination pagination;
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

struct Policy {
  FieldElement target;
  const char *method;
  const char *description;
};

extern "C" {

/// Creates a new Torii client instance
///
/// # Parameters
/// * `torii_url` - URL of the Torii server
/// * `libp2p_relay_url` - URL of the libp2p relay server
///
/// # Returns
/// Result containing pointer to new ToriiClient instance or error
Result<ToriiClient*> client_new(const char *torii_url);

/// Initiates a connection to establish a new session account
///
/// This function:
/// 1. Generates a new signing key pair
/// 2. Starts a local HTTP server to receive the callback
/// 3. Opens the keychain session URL in browser
/// 4. Waits for callback with session details
/// 5. Creates and stores the session
/// 6. Calls the provided callback with the new session account
///
/// # Safety
/// This function is marked as unsafe because it:
/// - Handles raw C pointers
/// - Performs FFI operations
/// - Creates system-level resources (HTTP server, keyring entries)
///
/// # Parameters
/// * `rpc_url` - Pointer to null-terminated string containing the RPC endpoint URL
/// * `policies` - Pointer to array of Policy structs defining session permissions
/// * `policies_len` - Length of the policies array
/// * `account_callback` - Function pointer called with the new session account when ready
///
/// # Example
/// ```c
/// void on_account(SessionAccount* account) {
///     // Handle new session account
/// }
///
/// controller_connect(
///     "https://rpc.example.com",
///     policies,
///     policies_length,
///     on_account
/// );
/// ```
void controller_connect(const char *rpc_url,
                        const Policy *policies,
                        uintptr_t policies_len,
                        void (*account_callback)(ControllerAccount*));

/// Retrieves a stored session account if one exists and is valid
///
/// # Parameters
/// * `policies` - Array of policies to match the session
/// * `policies_len` - Length of policies array
/// * `chain_id` - Chain ID to verify against
///
/// # Returns
/// Result containing pointer to SessionAccount or error if no valid account exists
Result<ControllerAccount*> controller_account(const Policy *policies,
                                              uintptr_t policies_len,
                                              FieldElement chain_id);

/// Clears sessions matching the specified policies and chain ID
///
/// # Parameters
/// * `policies` - Array of policies to match
/// * `policies_len` - Length of policies array
/// * `chain_id` - Chain ID to match
///
/// # Returns
/// Result containing success boolean or error
Result<bool> controller_clear(const Policy *policies,
                              uintptr_t policies_len,
                              FieldElement chain_id);

/// Gets the username of controller
///
/// # Parameters
/// * `account` - Pointer to Account
///
/// # Returns
/// CString containing the username
const char *controller_username(ControllerAccount *controller);

/// Gets account address
///
/// # Parameters
/// * `account` - Pointer to Account
///
/// # Returns
/// FieldElement containing the account address
FieldElement controller_address(ControllerAccount *controller);

/// Gets account chain ID
///
/// # Parameters
/// * `account` - Pointer to Account
///
/// # Returns
/// FieldElement containing the chain ID
FieldElement controller_chain_id(ControllerAccount *controller);

/// Gets account nonce
///
/// # Parameters
/// * `account` - Pointer to Account
///
/// # Returns
/// Result containing FieldElement nonce or error
Result<FieldElement> controller_nonce(ControllerAccount *controller);

/// Executes raw transaction
///
/// # Parameters
/// * `account` - Pointer to Account
/// * `calldata` - Array of Call structs
/// * `calldata_len` - Length of calldata array
///
/// # Returns
/// Result containing transaction hash as FieldElement or error
Result<FieldElement> controller_execute_raw(ControllerAccount *controller,
                                            const Call *calldata,
                                            uintptr_t calldata_len);

/// Executes a transaction from outside (paymaster)
///
/// # Parameters
/// * `account` - Pointer to Account
/// * `calldata` - Array of Call structs
/// * `calldata_len` - Length of calldata array
///
/// # Returns
/// Result containing transaction hash as FieldElement or error
Result<FieldElement> controller_execute_from_outside(ControllerAccount *controller,
                                                     const Call *calldata,
                                                     uintptr_t calldata_len);

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
Result<const char*> client_publish_message(ToriiClient *client, Message message);

/// Publishes multiple messages to the network
///
/// # Parameters
/// * `client` - Pointer to ToriiClient instance
/// * `messages` - Array of Message structs
/// * `messages_len` - Length of messages array
///
/// # Returns
/// Result containing array of message IDs or error
Result<CArray<const char*>> client_publish_message_batch(ToriiClient *client,
                                                         const Message *messages,
                                                         uintptr_t messages_len);

/// Retrieves controllers for the given contract addresses
///
/// # Parameters
/// * `client` - Pointer to ToriiClient instance
/// * `contract_addresses` - Array of contract addresses. If empty, all controllers will be
///   returned.
///
/// # Returns
/// Result containing controllers or error
Result<Page<Controller>> client_controllers(ToriiClient *client, ControllerQuery query);

/// Queries entities matching given criteria
///
/// # Parameters
/// * `client` - Pointer to ToriiClient instance
/// * `query` - Query parameters
///
/// # Returns
/// Result containing array of matching entities or error
Result<Page<Entity>> client_entities(ToriiClient *client, Query query);

/// Retrieves event messages matching the given query
///
/// # Parameters
/// * `client` - Pointer to ToriiClient instance
/// * `query` - Query parameters
/// * `historical` - Whether to include historical messages
///
/// # Returns
/// Result containing array of matching event message entities or error
Result<Page<Entity>> client_event_messages(ToriiClient *client, Query query);

/// Gets the world metadata for the client
///
/// # Parameters
/// * `client` - Pointer to ToriiClient instance
///
/// # Returns
/// World structure containing world information
Result<CArray<World>> client_worlds(ToriiClient *client,
                                    const FieldElement *world_addresses,
                                    uintptr_t world_addresses_len);

/// Retrieves transactions matching the given query
///
/// # Parameters
/// * `client` - Pointer to ToriiClient instance
/// * `query` - Query parameters
///
/// # Returns
/// Result containing array of matching transactions or error
Result<Page<Transaction>> client_transactions(ToriiClient *client, TransactionQuery query);

/// Subscribes to transaction updates
///
/// # Parameters
/// * `client` - Pointer to ToriiClient instance
/// * `filter` - Filter parameters
/// * `callback` - Function called when updates occur
///
/// # Returns
/// Result containing pointer to Subscription or error
Result<Subscription*> client_on_transaction(ToriiClient *client,
                                            COption<TransactionFilter> filter,
                                            void (*callback)(Transaction));

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
                                                    COption<Clause> clause,
                                                    const FieldElement *world_addresses,
                                                    uintptr_t world_addresses_len,
                                                    void (*callback)(Entity));

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
                                               COption<Clause> clause,
                                               const FieldElement *world_addresses,
                                               uintptr_t world_addresses_len);

/// Retrieves aggregations (leaderboards, stats, rankings) matching query parameter
///
/// # Parameters
/// * `client` - Pointer to ToriiClient instance
/// * `query` - AggregationQuery containing aggregator_ids, entity_ids, and pagination
///
/// # Returns
/// Result containing Page of AggregationEntry or error
Result<Page<AggregationEntry>> client_aggregations(ToriiClient *client, AggregationQuery query);

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
Result<Subscription*> client_on_aggregation_update(ToriiClient *client,
                                                   const char *const *aggregator_ids,
                                                   uintptr_t aggregator_ids_len,
                                                   const char *const *entity_ids,
                                                   uintptr_t entity_ids_len,
                                                   void (*callback)(AggregationEntry));

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
Result<bool> client_update_aggregation_subscription(ToriiClient *client,
                                                    Subscription *subscription,
                                                    const char *const *aggregator_ids,
                                                    uintptr_t aggregator_ids_len,
                                                    const char *const *entity_ids,
                                                    uintptr_t entity_ids_len);

/// Retrieves achievements matching query parameter
///
/// # Parameters
/// * `client` - Pointer to ToriiClient instance
/// * `query` - AchievementQuery containing world_addresses, namespaces, hidden filter, and
///   pagination
///
/// # Returns
/// Result containing Page of Achievement or error
Result<Page<Achievement>> client_achievements(ToriiClient *client, AchievementQuery query);

/// Retrieves player achievement data matching query parameter
///
/// # Parameters
/// * `client` - Pointer to ToriiClient instance
/// * `query` - PlayerAchievementQuery containing world_addresses, namespaces, player_addresses, and
///   pagination
///
/// # Returns
/// Result containing Page of PlayerAchievementEntry or error
Result<Page<PlayerAchievementEntry>> client_player_achievements(ToriiClient *client,
                                                                PlayerAchievementQuery query);

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
Result<Subscription*> client_on_achievement_progression_update(ToriiClient *client,
                                                               const FieldElement *world_addresses,
                                                               uintptr_t world_addresses_len,
                                                               const char *const *namespaces,
                                                               uintptr_t namespaces_len,
                                                               const FieldElement *player_addresses,
                                                               uintptr_t player_addresses_len,
                                                               const char *const *achievement_ids,
                                                               uintptr_t achievement_ids_len,
                                                               void (*callback)(AchievementProgression));

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
Result<bool> client_update_achievement_progression_subscription(ToriiClient *client,
                                                                Subscription *subscription,
                                                                const FieldElement *world_addresses,
                                                                uintptr_t world_addresses_len,
                                                                const char *const *namespaces,
                                                                uintptr_t namespaces_len,
                                                                const FieldElement *player_addresses,
                                                                uintptr_t player_addresses_len,
                                                                const char *const *achievement_ids,
                                                                uintptr_t achievement_ids_len);

/// Retrieves activities (user session tracking) matching query parameter
///
/// # Parameters
/// * `client` - Pointer to ToriiClient instance
/// * `query` - ActivityQuery containing world_addresses, namespaces, caller_addresses, and
///   pagination
///
/// # Returns
/// Result containing Page of Activity or error
Result<Page<Activity>> client_activities(ToriiClient *client, ActivityQuery query);

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
Result<Subscription*> client_on_activity_update(ToriiClient *client,
                                                const FieldElement *world_addresses,
                                                uintptr_t world_addresses_len,
                                                const char *const *namespaces,
                                                uintptr_t namespaces_len,
                                                const FieldElement *caller_addresses,
                                                uintptr_t caller_addresses_len,
                                                void (*callback)(Activity));

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
Result<bool> client_update_activity_subscription(ToriiClient *client,
                                                 Subscription *subscription,
                                                 const FieldElement *world_addresses,
                                                 uintptr_t world_addresses_len,
                                                 const char *const *namespaces,
                                                 uintptr_t namespaces_len,
                                                 const FieldElement *caller_addresses,
                                                 uintptr_t caller_addresses_len);

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
Result<Subscription*> client_on_event_message_update(ToriiClient *client,
                                                     COption<Clause> clause,
                                                     const FieldElement *world_addresses,
                                                     uintptr_t world_addresses_len,
                                                     void (*callback)(Entity));

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
Result<bool> client_update_event_message_subscription(ToriiClient *client,
                                                      Subscription *subscription,
                                                      COption<Clause> clause,
                                                      const FieldElement *world_addresses,
                                                      uintptr_t world_addresses_len);

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
                                               const KeysClause *clauses,
                                               uintptr_t clauses_len,
                                               void (*callback)(Event));

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
Result<Page<Token>> client_tokens(ToriiClient *client, TokenQuery query);

/// Subscribes to token updates
///
/// # Parameters
/// * `client` - Pointer to ToriiClient instance
/// * `contract_addresses` - Array of contract addresses
/// * `callback` - Function called when updates occur
///
/// # Returns
/// Result containing pointer to Subscription or error
Result<Subscription*> client_on_token_update(ToriiClient *client,
                                             const FieldElement *contract_addresses,
                                             uintptr_t contract_addresses_len,
                                             const U256 *token_ids,
                                             uintptr_t token_ids_len,
                                             void (*callback)(Token));

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
Result<Page<TokenBalance>> client_token_balances(ToriiClient *client, TokenBalanceQuery query);

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
Result<Page<TokenContract>> client_token_contracts(ToriiClient *client, TokenContractQuery query);

/// Gets contracts matching the given query
///
/// # Parameters
/// * `client` - Pointer to ToriiClient instance
/// * `query` - ContractQuery parameters
///
/// # Returns
/// Result containing array of Contract information or error
Result<CArray<Contract>> client_contracts(ToriiClient *client, ContractQuery query);

/// Retrieves token transfers matching the given query
///
/// # Parameters
/// * `client` - Pointer to ToriiClient instance
/// * `query` - TokenTransferQuery parameters
///
/// # Returns
/// Result containing array of TokenTransfer information or error
Result<Page<TokenTransfer>> client_token_transfers(ToriiClient *client, TokenTransferQuery query);

/// Subscribes to contract updates
///
/// # Parameters
/// * `client` - Pointer to ToriiClient instance
/// * `contract_address` - Optional contract address to filter updates
/// * `callback` - Function called when updates occur
///
/// # Returns
/// Result containing pointer to Subscription or error
Result<Subscription*> on_contract_update(ToriiClient *client,
                                         const FieldElement *contract_address,
                                         void (*callback)(Contract));

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
                                                     const U256 *token_ids,
                                                     uintptr_t token_ids_len,
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
                                                      uintptr_t account_addresses_len,
                                                      const U256 *token_ids,
                                                      uintptr_t token_ids_len);

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
Result<Subscription*> client_on_token_transfer_update(ToriiClient *client,
                                                      const FieldElement *contract_addresses,
                                                      uintptr_t contract_addresses_len,
                                                      const FieldElement *account_addresses,
                                                      uintptr_t account_addresses_len,
                                                      const U256 *token_ids,
                                                      uintptr_t token_ids_len,
                                                      void (*callback)(TokenTransfer));

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
Result<bool> client_update_token_transfer_subscription(ToriiClient *client,
                                                       Subscription *subscription,
                                                       const FieldElement *contract_addresses,
                                                       uintptr_t contract_addresses_len,
                                                       const FieldElement *account_addresses,
                                                       uintptr_t account_addresses_len,
                                                       const U256 *token_ids,
                                                       uintptr_t token_ids_len);

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
void world_metadata_free(World *metadata);

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
