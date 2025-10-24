#pragma once

#include <algorithm>
#include <bit>
#include <chrono>
#include <cstdint>
#include <exception>
#include <functional>
#include <iostream>
#include <map>
#include <memory>
#include <mutex>
#include <optional>
#include <stdexcept>
#include <streambuf>
#include <type_traits>
#include <variant>
#include <vector>

#include "dojo_scaffolding.hpp"

namespace dojo {
struct ToriiClient;
struct Achievement;
struct AchievementProgression;
struct AchievementQuery;
struct AchievementTask;
struct ActionCount;
struct Activity;
struct ActivityQuery;
struct AggregationEntry;
struct AggregationQuery;
struct AttributeFilter;
struct CompositeClause;
struct Contract;
struct ContractQuery;
struct Controller;
struct ControllerQuery;
struct Entity;
struct EnumOption;
struct EnumType;
struct Event;
struct EventQuery;
struct FixedSizeArray;
struct KeysClause;
struct Member;
struct MemberClause;
struct Message;
struct Model;
struct OrderBy;
struct PageAchievement;
struct PageActivity;
struct PageAggregationEntry;
struct PageController;
struct PageEntity;
struct PageEvent;
struct PagePlayerAchievement;
struct PageToken;
struct PageTokenBalance;
struct PageTokenContract;
struct PageTokenTransfer;
struct PageTransaction;
struct Pagination;
struct PlayerAchievementEntry;
struct PlayerAchievementProgress;
struct PlayerAchievementQuery;
struct PlayerAchievementStats;
struct Query;
struct Signature;
struct SqlField;
struct SqlRow;
struct Struct;
struct TaskProgress;
struct Token;
struct TokenBalance;
struct TokenBalanceQuery;
struct TokenContract;
struct TokenContractQuery;
struct TokenQuery;
struct TokenTransfer;
struct TokenTransferQuery;
struct Transaction;
struct TransactionCall;
struct TransactionFilter;
struct TransactionQuery;
struct World;
enum class CallType;
struct Clause;
enum class ComparisonOperator;
enum class ContractType;
struct DojoError;
enum class LogicalOperator;
struct MemberValue;
enum class OrderDirection;
enum class PaginationDirection;
enum class PatternMatching;
struct Primitive;
struct SqlValue;
struct Ty;
struct ValueType;
struct EntityUpdateCallback;
struct EventUpdateCallback;
struct TokenBalanceUpdateCallback;
struct TokenUpdateCallback;
struct TransactionUpdateCallback;
typedef std::string FieldElement;
typedef std::string U256;


struct PlayerAchievementStats {
    uint32_t total_points;
    uint32_t completed_achievements;
    uint32_t total_achievements;
    double completion_percentage;
    std::optional<uint64_t> last_achievement_at;
    uint64_t created_at;
    uint64_t updated_at;
};


enum class PaginationDirection: int32_t {
    kForward = 1,
    kBackward = 2
};

namespace uniffi {
struct FfiConverterSqlValue;
} // namespace uniffi

struct SqlValue {
    friend uniffi::FfiConverterSqlValue;
    struct kText {
        std::string value;
    };
    struct kInteger {
        int64_t value;
    };
    struct kReal {
        double value;
    };
    struct kBlob {
        std::vector<uint8_t> value;
    };
    struct kNull {
    };
    SqlValue(kText variant): variant(variant) {}
    SqlValue(kInteger variant): variant(variant) {}
    SqlValue(kReal variant): variant(variant) {}
    SqlValue(kBlob variant): variant(variant) {}
    SqlValue(kNull variant): variant(variant) {}

    SqlValue(const SqlValue &other): variant(other.variant) {}
    SqlValue(SqlValue &&other): variant(std::move(other.variant)) {}

    SqlValue &operator=(const SqlValue &other) {
        variant = other.variant;
        return *this;
    }

    SqlValue &operator=(SqlValue &&other) {
        variant = std::move(other.variant);
        return *this;
    }

    /**
     * Returns the variant of this enum
     */
    const std::variant<kText, kInteger, kReal, kBlob, kNull> &get_variant() const {
        return variant;
    }

private:
    std::variant<kText, kInteger, kReal, kBlob, kNull> variant;

    SqlValue();
};


struct AchievementTask {
    std::string task_id;
    std::string description;
    uint32_t total;
    uint32_t total_completions;
    double completion_rate;
    uint64_t created_at;
};


enum class LogicalOperator: int32_t {
    kAnd = 1,
    kOr = 2
};


enum class ContractType: int32_t {
    kWorld = 1,
    kErc20 = 2,
    kErc721 = 3,
    kErc1155 = 4,
    kUdc = 5,
    kOther = 6
};


struct AttributeFilter {
    std::string trait_name;
    std::string trait_value;
};


enum class CallType: int32_t {
    kExecute = 1,
    kExecuteFromOutside = 2
};


enum class PatternMatching: int32_t {
    kFixedLen = 1,
    kVariableLen = 2
};


enum class ComparisonOperator: int32_t {
    kEq = 1,
    kNeq = 2,
    kGt = 3,
    kGte = 4,
    kLt = 5,
    kLte = 6,
    kIn = 7,
    kNotIn = 8,
    kContains = 9,
    kContainsAll = 10,
    kContainsAny = 11,
    kArrayLengthEq = 12,
    kArrayLengthGt = 13,
    kArrayLengthLt = 14
};


struct TaskProgress {
    std::string task_id;
    uint32_t count;
    bool completed;
};


struct ActionCount {
    std::string action_name;
    uint32_t count;
};


enum class OrderDirection: int32_t {
    kAsc = 1,
    kDesc = 2
};


struct ContractQuery {
    std::vector<FieldElement> contract_addresses;
    std::vector<ContractType> contract_types;
};


struct Controller {
    FieldElement address;
    std::string username;
    uint64_t deployed_at_timestamp;
};


struct Achievement {
    std::string id;
    FieldElement world_address;
    std::string namespace_;
    std::string entity_id;
    bool hidden;
    uint32_t index;
    uint32_t points;
    std::string start;
    std::string end;
    std::string group;
    std::string icon;
    std::string title;
    std::string description;
    std::vector<AchievementTask> tasks;
    std::optional<std::string> data;
    uint32_t total_completions;
    double completion_rate;
    uint64_t created_at;
    uint64_t updated_at;
};

namespace uniffi {
struct FfiConverterPrimitive;
} // namespace uniffi

struct Primitive {
    friend uniffi::FfiConverterPrimitive;
    struct kI8 {
        int8_t value;
    };
    struct kI16 {
        int16_t value;
    };
    struct kI32 {
        int32_t value;
    };
    struct kI64 {
        int64_t value;
    };
    struct kI128 {
        std::vector<uint8_t> value;
    };
    struct kU8 {
        uint8_t value;
    };
    struct kU16 {
        uint16_t value;
    };
    struct kU32 {
        uint32_t value;
    };
    struct kU64 {
        uint64_t value;
    };
    struct kU128 {
        std::vector<uint8_t> value;
    };
    struct kU256 {
        U256 value;
    };
    struct kBool {
        bool value;
    };
    struct kFelt252 {
        FieldElement value;
    };
    struct kClassHash {
        FieldElement value;
    };
    struct kContractAddress {
        FieldElement value;
    };
    struct kEthAddress {
        FieldElement value;
    };
    Primitive(kI8 variant): variant(variant) {}
    Primitive(kI16 variant): variant(variant) {}
    Primitive(kI32 variant): variant(variant) {}
    Primitive(kI64 variant): variant(variant) {}
    Primitive(kI128 variant): variant(variant) {}
    Primitive(kU8 variant): variant(variant) {}
    Primitive(kU16 variant): variant(variant) {}
    Primitive(kU32 variant): variant(variant) {}
    Primitive(kU64 variant): variant(variant) {}
    Primitive(kU128 variant): variant(variant) {}
    Primitive(kU256 variant): variant(variant) {}
    Primitive(kBool variant): variant(variant) {}
    Primitive(kFelt252 variant): variant(variant) {}
    Primitive(kClassHash variant): variant(variant) {}
    Primitive(kContractAddress variant): variant(variant) {}
    Primitive(kEthAddress variant): variant(variant) {}

    Primitive(const Primitive &other): variant(other.variant) {}
    Primitive(Primitive &&other): variant(std::move(other.variant)) {}

    Primitive &operator=(const Primitive &other) {
        variant = other.variant;
        return *this;
    }

    Primitive &operator=(Primitive &&other) {
        variant = std::move(other.variant);
        return *this;
    }

    /**
     * Returns the variant of this enum
     */
    const std::variant<kI8, kI16, kI32, kI64, kI128, kU8, kU16, kU32, kU64, kU128, kU256, kBool, kFelt252, kClassHash, kContractAddress, kEthAddress> &get_variant() const {
        return variant;
    }

private:
    std::variant<kI8, kI16, kI32, kI64, kI128, kU8, kU16, kU32, kU64, kU128, kU256, kBool, kFelt252, kClassHash, kContractAddress, kEthAddress> variant;

    Primitive();
};


struct TokenContract {
    FieldElement contract_address;
    std::string name;
    std::string symbol;
    uint8_t decimals;
    std::string metadata;
    std::string token_metadata;
    std::optional<U256> total_supply;
};


struct TransactionCall {
    FieldElement contract_address;
    std::string entrypoint;
    std::vector<FieldElement> calldata;
    CallType call_type;
    FieldElement caller_address;
};


struct Event {
    std::vector<FieldElement> keys;
    std::vector<FieldElement> data;
    FieldElement transaction_hash;
};


struct TokenBalance {
    U256 balance;
    FieldElement account_address;
    FieldElement contract_address;
    std::optional<U256> token_id;
};


struct TokenTransfer {
    std::string id;
    FieldElement contract_address;
    FieldElement from_address;
    FieldElement to_address;
    U256 amount;
    std::optional<U256> token_id;
    uint64_t executed_at;
    std::optional<std::string> event_id;
};


struct AggregationEntry {
    std::string id;
    std::string aggregator_id;
    std::string entity_id;
    U256 value;
    std::string display_value;
    uint64_t position;
    std::string model_id;
    uint64_t created_at;
    uint64_t updated_at;
};


struct OrderBy {
    std::string field;
    OrderDirection direction;
};


struct Activity {
    std::string id;
    FieldElement world_address;
    std::string namespace_;
    FieldElement caller_address;
    uint64_t session_start;
    uint64_t session_end;
    uint32_t action_count;
    std::vector<ActionCount> actions;
    uint64_t updated_at;
};


struct Contract {
    FieldElement contract_address;
    ContractType contract_type;
    std::optional<uint64_t> head;
    std::optional<uint64_t> tps;
    std::optional<uint64_t> last_block_timestamp;
    std::optional<FieldElement> last_pending_block_tx;
    uint64_t updated_at;
    uint64_t created_at;
};


struct KeysClause {
    std::vector<std::optional<FieldElement>> keys;
    PatternMatching pattern_matching;
    std::vector<std::string> models;
};


struct Token {
    FieldElement contract_address;
    std::optional<U256> token_id;
    std::string name;
    std::string symbol;
    uint8_t decimals;
    std::string metadata;
    std::optional<U256> total_supply;
};


struct Message {
    std::string message;
    std::vector<FieldElement> signature;
    FieldElement world_address;
};


struct AchievementProgression {
    std::string id;
    std::string achievement_id;
    std::string task_id;
    FieldElement world_address;
    std::string namespace_;
    FieldElement player_id;
    uint32_t count;
    bool completed;
    std::optional<uint64_t> completed_at;
    uint64_t created_at;
    uint64_t updated_at;
};


struct Signature {
    FieldElement r;
    FieldElement s;
};


struct SqlField {
    std::string name;
    SqlValue value;
};


struct TransactionFilter {
    std::vector<FieldElement> transaction_hashes;
    std::vector<FieldElement> caller_addresses;
    std::vector<FieldElement> contract_addresses;
    std::vector<std::string> entrypoints;
    std::vector<FieldElement> model_selectors;
    std::optional<uint64_t> from_block;
    std::optional<uint64_t> to_block;
};


struct PageAchievement {
    std::vector<Achievement> items;
    std::optional<std::string> next_cursor;
};


struct PageEvent {
    std::vector<Event> items;
    std::optional<std::string> next_cursor;
};


struct PageTokenBalance {
    std::vector<TokenBalance> items;
    std::optional<std::string> next_cursor;
};


struct PageToken {
    std::vector<Token> items;
    std::optional<std::string> next_cursor;
};


struct SqlRow {
    std::vector<SqlField> fields;
};


struct PageActivity {
    std::vector<Activity> items;
    std::optional<std::string> next_cursor;
};


struct PageAggregationEntry {
    std::vector<AggregationEntry> items;
    std::optional<std::string> next_cursor;
};


struct Transaction {
    FieldElement transaction_hash;
    FieldElement sender_address;
    std::vector<FieldElement> calldata;
    FieldElement max_fee;
    std::vector<FieldElement> signature;
    FieldElement nonce;
    uint64_t block_number;
    std::string transaction_type;
    uint64_t block_timestamp;
    std::vector<TransactionCall> calls;
    std::vector<FieldElement> unique_models;
};


struct PageTokenContract {
    std::vector<TokenContract> items;
    std::optional<std::string> next_cursor;
};


struct PlayerAchievementProgress {
    Achievement achievement;
    std::vector<TaskProgress> task_progress;
    bool completed;
    double progress_percentage;
};


struct Pagination {
    std::optional<std::string> cursor;
    std::optional<uint32_t> limit;
    PaginationDirection direction;
    std::vector<OrderBy> order_by;
};


struct PageController {
    std::vector<Controller> items;
    std::optional<std::string> next_cursor;
};


struct PageTokenTransfer {
    std::vector<TokenTransfer> items;
    std::optional<std::string> next_cursor;
};


struct PlayerAchievementEntry {
    FieldElement player_address;
    PlayerAchievementStats stats;
    std::vector<PlayerAchievementProgress> achievements;
};


struct AggregationQuery {
    std::vector<std::string> aggregator_ids;
    std::vector<std::string> entity_ids;
    Pagination pagination;
};


struct PageTransaction {
    std::vector<Transaction> items;
    std::optional<std::string> next_cursor;
};


struct TokenQuery {
    std::vector<FieldElement> contract_addresses;
    std::vector<U256> token_ids;
    std::vector<AttributeFilter> attribute_filters;
    Pagination pagination;
};


struct TokenBalanceQuery {
    std::vector<FieldElement> contract_addresses;
    std::vector<FieldElement> account_addresses;
    std::vector<U256> token_ids;
    Pagination pagination;
};


struct TokenContractQuery {
    std::vector<FieldElement> contract_addresses;
    std::vector<ContractType> contract_types;
    Pagination pagination;
};


struct TransactionQuery {
    std::optional<std::shared_ptr<TransactionFilter>> filter;
    Pagination pagination;
};


struct ControllerQuery {
    Pagination pagination;
    std::vector<FieldElement> contract_addresses;
    std::vector<std::string> usernames;
};


struct AchievementQuery {
    std::vector<FieldElement> world_addresses;
    std::vector<std::string> namespaces;
    std::optional<bool> hidden;
    Pagination pagination;
};


struct ActivityQuery {
    std::vector<FieldElement> world_addresses;
    std::vector<std::string> namespaces;
    std::vector<FieldElement> caller_addresses;
    std::optional<uint64_t> from_time;
    std::optional<uint64_t> to_time;
    Pagination pagination;
};


struct EventQuery {
    std::optional<std::shared_ptr<KeysClause>> keys;
    Pagination pagination;
};


struct TokenTransferQuery {
    std::vector<FieldElement> contract_addresses;
    std::vector<FieldElement> account_addresses;
    std::vector<U256> token_ids;
    Pagination pagination;
};


struct PlayerAchievementQuery {
    std::vector<FieldElement> world_addresses;
    std::vector<std::string> namespaces;
    std::vector<FieldElement> player_addresses;
    Pagination pagination;
};


struct PagePlayerAchievement {
    std::vector<PlayerAchievementEntry> items;
    std::optional<std::string> next_cursor;
};


namespace uniffi {
    struct FfiConverterToriiClient;
} // namespace uniffi

struct ToriiClient



{
    friend uniffi::FfiConverterToriiClient;

    ToriiClient() = delete;

    ToriiClient(ToriiClient &&) = delete;

    ToriiClient &operator=(const ToriiClient &) = delete;
    ToriiClient &operator=(ToriiClient &&) = delete;

    ~ToriiClient();
    static std::shared_ptr<ToriiClient> init(const std::string &torii_url);
    static std::shared_ptr<ToriiClient> new_with_config(const std::string &torii_url, uint64_t max_message_size);
    PageAchievement achievements(const AchievementQuery &query);
    PageActivity activities(const ActivityQuery &query);
    PageAggregationEntry aggregations(const AggregationQuery &query);
    void cancel_subscription(uint64_t subscription_id);
    std::vector<Contract> contracts(const ContractQuery &query);
    PageController controllers(const ControllerQuery &query);
    PageEntity entities(const Query &query);
    PageEntity event_messages(const Query &query);
    PagePlayerAchievement player_achievements(const PlayerAchievementQuery &query);
    std::string publish_message(const Message &message);
    std::vector<std::string> publish_message_batch(const std::vector<Message> &messages);
    std::vector<SqlRow> sql(const std::string &query);
    PageEvent starknet_events(const EventQuery &query);
    uint64_t subscribe_entity_updates(std::optional<std::shared_ptr<Clause>> clause, const std::vector<FieldElement> &world_addresses, const std::shared_ptr<EntityUpdateCallback> &callback);
    uint64_t subscribe_event_updates(const std::vector<KeysClause> &keys, const std::shared_ptr<EventUpdateCallback> &callback);
    uint64_t subscribe_token_balance_updates(const std::vector<FieldElement> &contract_addresses, const std::vector<FieldElement> &account_addresses, const std::vector<U256> &token_ids, const std::shared_ptr<TokenBalanceUpdateCallback> &callback);
    uint64_t subscribe_token_updates(const std::vector<FieldElement> &contract_addresses, const std::vector<U256> &token_ids, const std::shared_ptr<TokenUpdateCallback> &callback);
    uint64_t subscribe_transaction_updates(std::optional<std::shared_ptr<TransactionFilter>> filter, const std::shared_ptr<TransactionUpdateCallback> &callback);
    PageTokenBalance token_balances(const TokenBalanceQuery &query);
    PageTokenContract token_contracts(const TokenContractQuery &query);
    PageTokenTransfer token_transfers(const TokenTransferQuery &query);
    PageToken tokens(const TokenQuery &query);
    PageTransaction transactions(const TransactionQuery &query);
    std::vector<std::shared_ptr<World>> worlds(const std::vector<FieldElement> &world_addresses);

    private:
    ToriiClient(const ToriiClient &);

    ToriiClient(void *);

    void *_uniffi_internal_clone_pointer() const;

    void *instance = nullptr;
};


struct CompositeClause {
    LogicalOperator operator_;
    std::vector<std::shared_ptr<Clause>> clauses;
};


struct Entity {
    FieldElement world_address;
    FieldElement hashed_keys;
    std::vector<std::shared_ptr<Struct>> models;
    uint64_t created_at;
    uint64_t updated_at;
    uint64_t executed_at;
};


struct EnumOption {
    std::string name;
    std::shared_ptr<Ty> ty;
};


struct EnumType {
    std::string name;
    uint8_t option;
    std::vector<std::shared_ptr<EnumOption>> options;
};


struct FixedSizeArray {
    std::vector<std::shared_ptr<Ty>> array;
    uint32_t size;
};


struct Member {
    std::string name;
    std::shared_ptr<Ty> ty;
    bool key;
};


struct MemberClause {
    std::string model;
    std::string member;
    ComparisonOperator operator_;
    std::shared_ptr<MemberValue> value;
};


struct Model {
    FieldElement world_address;
    std::shared_ptr<Ty> schema;
    std::string namespace_;
    std::string name;
    FieldElement selector;
    uint32_t packed_size;
    uint32_t unpacked_size;
    FieldElement class_hash;
    FieldElement contract_address;
    std::string layout;
    bool use_legacy_store;
};


struct PageEntity {
    std::vector<std::shared_ptr<Entity>> items;
    std::optional<std::string> next_cursor;
};


struct Query {
    std::vector<FieldElement> world_addresses;
    Pagination pagination;
    std::optional<std::shared_ptr<Clause>> clause;
    bool no_hashed_keys;
    std::vector<std::string> models;
    bool historical;
};


struct Struct {
    std::string name;
    std::vector<std::shared_ptr<Member>> children;
};


struct World {
    FieldElement world_address;
    std::vector<std::shared_ptr<Model>> models;
};

namespace uniffi {
struct FfiConverterClause;
} // namespace uniffi

struct Clause {
    friend uniffi::FfiConverterClause;
    struct kHashedKeys {
        std::vector<FieldElement> keys;
    };
    struct kKeys {
        KeysClause clause;
    };
    struct kMember {
        std::shared_ptr<MemberClause> clause;
    };
    struct kComposite {
        std::shared_ptr<CompositeClause> clause;
    };
    Clause(kHashedKeys variant): variant(variant) {}
    Clause(kKeys variant): variant(variant) {}
    Clause(kMember variant): variant(variant) {}
    Clause(kComposite variant): variant(variant) {}

    Clause(const Clause &other): variant(other.variant) {}
    Clause(Clause &&other): variant(std::move(other.variant)) {}

    Clause &operator=(const Clause &other) {
        variant = other.variant;
        return *this;
    }

    Clause &operator=(Clause &&other) {
        variant = std::move(other.variant);
        return *this;
    }

    /**
     * Returns the variant of this enum
     */
    const std::variant<kHashedKeys, kKeys, kMember, kComposite> &get_variant() const {
        return variant;
    }

private:
    std::variant<kHashedKeys, kKeys, kMember, kComposite> variant;

    Clause();
};

namespace uniffi {
struct FfiConverterDojoError;
} // namespace uniffi

struct DojoError: std::runtime_error {
    friend uniffi::FfiConverterDojoError;

    DojoError() : std::runtime_error("") {}
    DojoError(const std::string &what_arg) : std::runtime_error(what_arg) {}

    virtual ~DojoError() = default;

    virtual void throw_underlying() {
        throw *this;
    }

protected:
    virtual int32_t get_variant_idx() const {
        return 0;
    };
};
/**
 * Contains variants of DojoError
 */
namespace dojo_error {

struct ClientError: DojoError {

    ClientError() : DojoError("") {}
    ClientError(const std::string &what_arg) : DojoError(what_arg) {}

    void throw_underlying() override {
        throw *this;
    }

protected:
    int32_t get_variant_idx() const override {
        return 1;
    }
};

struct SerializationError: DojoError {

    SerializationError() : DojoError("") {}
    SerializationError(const std::string &what_arg) : DojoError(what_arg) {}

    void throw_underlying() override {
        throw *this;
    }

protected:
    int32_t get_variant_idx() const override {
        return 2;
    }
};

struct NetworkError: DojoError {

    NetworkError() : DojoError("") {}
    NetworkError(const std::string &what_arg) : DojoError(what_arg) {}

    void throw_underlying() override {
        throw *this;
    }

protected:
    int32_t get_variant_idx() const override {
        return 3;
    }
};

struct InvalidInput: DojoError {

    InvalidInput() : DojoError("") {}
    InvalidInput(const std::string &what_arg) : DojoError(what_arg) {}

    void throw_underlying() override {
        throw *this;
    }

protected:
    int32_t get_variant_idx() const override {
        return 4;
    }
};

struct ConnectionError: DojoError {

    ConnectionError() : DojoError("") {}
    ConnectionError(const std::string &what_arg) : DojoError(what_arg) {}

    void throw_underlying() override {
        throw *this;
    }

protected:
    int32_t get_variant_idx() const override {
        return 5;
    }
};

struct PublishError: DojoError {

    PublishError() : DojoError("") {}
    PublishError(const std::string &what_arg) : DojoError(what_arg) {}

    void throw_underlying() override {
        throw *this;
    }

protected:
    int32_t get_variant_idx() const override {
        return 6;
    }
};

struct QueryError: DojoError {

    QueryError() : DojoError("") {}
    QueryError(const std::string &what_arg) : DojoError(what_arg) {}

    void throw_underlying() override {
        throw *this;
    }

protected:
    int32_t get_variant_idx() const override {
        return 7;
    }
};

struct SubscriptionError: DojoError {

    SubscriptionError() : DojoError("") {}
    SubscriptionError(const std::string &what_arg) : DojoError(what_arg) {}

    void throw_underlying() override {
        throw *this;
    }

protected:
    int32_t get_variant_idx() const override {
        return 8;
    }
};
} // namespace dojo_error

namespace uniffi {
struct FfiConverterMemberValue;
} // namespace uniffi

struct MemberValue {
    friend uniffi::FfiConverterMemberValue;
    struct kPrimitive {
        Primitive value;
    };
    struct kString {
        std::string value;
    };
    struct kList {
        std::vector<std::shared_ptr<MemberValue>> values;
    };
    MemberValue(kPrimitive variant): variant(variant) {}
    MemberValue(kString variant): variant(variant) {}
    MemberValue(kList variant): variant(variant) {}

    MemberValue(const MemberValue &other): variant(other.variant) {}
    MemberValue(MemberValue &&other): variant(std::move(other.variant)) {}

    MemberValue &operator=(const MemberValue &other) {
        variant = other.variant;
        return *this;
    }

    MemberValue &operator=(MemberValue &&other) {
        variant = std::move(other.variant);
        return *this;
    }

    /**
     * Returns the variant of this enum
     */
    const std::variant<kPrimitive, kString, kList> &get_variant() const {
        return variant;
    }

private:
    std::variant<kPrimitive, kString, kList> variant;

    MemberValue();
};

namespace uniffi {
struct FfiConverterTy;
} // namespace uniffi

struct Ty {
    friend uniffi::FfiConverterTy;
    struct kPrimitive {
        Primitive value;
    };
    struct kStruct {
        std::shared_ptr<Struct> value;
    };
    struct kEnum {
        std::shared_ptr<EnumType> value;
    };
    struct kTuple {
        std::vector<std::shared_ptr<Ty>> values;
    };
    struct kArray {
        std::vector<std::shared_ptr<Ty>> values;
    };
    struct kFixedSizeArray {
        std::shared_ptr<FixedSizeArray> value;
    };
    struct kByteArray {
        std::string value;
    };
    Ty(kPrimitive variant): variant(variant) {}
    Ty(kStruct variant): variant(variant) {}
    Ty(kEnum variant): variant(variant) {}
    Ty(kTuple variant): variant(variant) {}
    Ty(kArray variant): variant(variant) {}
    Ty(kFixedSizeArray variant): variant(variant) {}
    Ty(kByteArray variant): variant(variant) {}

    Ty(const Ty &other): variant(other.variant) {}
    Ty(Ty &&other): variant(std::move(other.variant)) {}

    Ty &operator=(const Ty &other) {
        variant = other.variant;
        return *this;
    }

    Ty &operator=(Ty &&other) {
        variant = std::move(other.variant);
        return *this;
    }

    /**
     * Returns the variant of this enum
     */
    const std::variant<kPrimitive, kStruct, kEnum, kTuple, kArray, kFixedSizeArray, kByteArray> &get_variant() const {
        return variant;
    }

private:
    std::variant<kPrimitive, kStruct, kEnum, kTuple, kArray, kFixedSizeArray, kByteArray> variant;

    Ty();
};

namespace uniffi {
struct FfiConverterValueType;
} // namespace uniffi

struct ValueType {
    friend uniffi::FfiConverterValueType;
    struct kString {
        std::string value;
    };
    struct kInt {
        int64_t value;
    };
    struct kUInt {
        uint64_t value;
    };
    struct kBool {
        bool value;
    };
    struct kBytes {
        std::vector<uint8_t> value;
    };
    ValueType(kString variant): variant(variant) {}
    ValueType(kInt variant): variant(variant) {}
    ValueType(kUInt variant): variant(variant) {}
    ValueType(kBool variant): variant(variant) {}
    ValueType(kBytes variant): variant(variant) {}

    ValueType(const ValueType &other): variant(other.variant) {}
    ValueType(ValueType &&other): variant(std::move(other.variant)) {}

    ValueType &operator=(const ValueType &other) {
        variant = other.variant;
        return *this;
    }

    ValueType &operator=(ValueType &&other) {
        variant = std::move(other.variant);
        return *this;
    }

    /**
     * Returns the variant of this enum
     */
    const std::variant<kString, kInt, kUInt, kBool, kBytes> &get_variant() const {
        return variant;
    }

private:
    std::variant<kString, kInt, kUInt, kBool, kBytes> variant;

    ValueType();
};



struct EntityUpdateCallback {
    virtual ~EntityUpdateCallback() {}
    virtual
    void on_update(const Entity &entity) = 0;
    virtual
    void on_error(const std::string &error) = 0;
};

namespace uniffi {
    struct UniffiCallbackInterfaceEntityUpdateCallback {
        static void on_update(uint64_t uniffi_handle,RustBuffer entity,void * uniffi_out_return,RustCallStatus *out_status);
        static void on_error(uint64_t uniffi_handle,RustBuffer error,void * uniffi_out_return,RustCallStatus *out_status);

        static void uniffi_free(uint64_t uniffi_handle);
        static void init();
    private:
        static inline UniffiVTableCallbackInterfaceEntityUpdateCallback vtable = UniffiVTableCallbackInterfaceEntityUpdateCallback {
            .on_update = reinterpret_cast<void *>(&on_update),
            .on_error = reinterpret_cast<void *>(&on_error),
            .uniffi_free = reinterpret_cast<void *>(&uniffi_free)
        };
    };
}



struct EventUpdateCallback {
    virtual ~EventUpdateCallback() {}
    virtual
    void on_update(const Event &event) = 0;
    virtual
    void on_error(const std::string &error) = 0;
};

namespace uniffi {
    struct UniffiCallbackInterfaceEventUpdateCallback {
        static void on_update(uint64_t uniffi_handle,RustBuffer event,void * uniffi_out_return,RustCallStatus *out_status);
        static void on_error(uint64_t uniffi_handle,RustBuffer error,void * uniffi_out_return,RustCallStatus *out_status);

        static void uniffi_free(uint64_t uniffi_handle);
        static void init();
    private:
        static inline UniffiVTableCallbackInterfaceEventUpdateCallback vtable = UniffiVTableCallbackInterfaceEventUpdateCallback {
            .on_update = reinterpret_cast<void *>(&on_update),
            .on_error = reinterpret_cast<void *>(&on_error),
            .uniffi_free = reinterpret_cast<void *>(&uniffi_free)
        };
    };
}



struct TokenBalanceUpdateCallback {
    virtual ~TokenBalanceUpdateCallback() {}
    virtual
    void on_update(const TokenBalance &balance) = 0;
    virtual
    void on_error(const std::string &error) = 0;
};

namespace uniffi {
    struct UniffiCallbackInterfaceTokenBalanceUpdateCallback {
        static void on_update(uint64_t uniffi_handle,RustBuffer balance,void * uniffi_out_return,RustCallStatus *out_status);
        static void on_error(uint64_t uniffi_handle,RustBuffer error,void * uniffi_out_return,RustCallStatus *out_status);

        static void uniffi_free(uint64_t uniffi_handle);
        static void init();
    private:
        static inline UniffiVTableCallbackInterfaceTokenBalanceUpdateCallback vtable = UniffiVTableCallbackInterfaceTokenBalanceUpdateCallback {
            .on_update = reinterpret_cast<void *>(&on_update),
            .on_error = reinterpret_cast<void *>(&on_error),
            .uniffi_free = reinterpret_cast<void *>(&uniffi_free)
        };
    };
}



struct TokenUpdateCallback {
    virtual ~TokenUpdateCallback() {}
    virtual
    void on_update(const Token &token) = 0;
    virtual
    void on_error(const std::string &error) = 0;
};

namespace uniffi {
    struct UniffiCallbackInterfaceTokenUpdateCallback {
        static void on_update(uint64_t uniffi_handle,RustBuffer token,void * uniffi_out_return,RustCallStatus *out_status);
        static void on_error(uint64_t uniffi_handle,RustBuffer error,void * uniffi_out_return,RustCallStatus *out_status);

        static void uniffi_free(uint64_t uniffi_handle);
        static void init();
    private:
        static inline UniffiVTableCallbackInterfaceTokenUpdateCallback vtable = UniffiVTableCallbackInterfaceTokenUpdateCallback {
            .on_update = reinterpret_cast<void *>(&on_update),
            .on_error = reinterpret_cast<void *>(&on_error),
            .uniffi_free = reinterpret_cast<void *>(&uniffi_free)
        };
    };
}



struct TransactionUpdateCallback {
    virtual ~TransactionUpdateCallback() {}
    virtual
    void on_update(const Transaction &transaction) = 0;
    virtual
    void on_error(const std::string &error) = 0;
};

namespace uniffi {
    struct UniffiCallbackInterfaceTransactionUpdateCallback {
        static void on_update(uint64_t uniffi_handle,RustBuffer transaction,void * uniffi_out_return,RustCallStatus *out_status);
        static void on_error(uint64_t uniffi_handle,RustBuffer error,void * uniffi_out_return,RustCallStatus *out_status);

        static void uniffi_free(uint64_t uniffi_handle);
        static void init();
    private:
        static inline UniffiVTableCallbackInterfaceTransactionUpdateCallback vtable = UniffiVTableCallbackInterfaceTransactionUpdateCallback {
            .on_update = reinterpret_cast<void *>(&on_update),
            .on_error = reinterpret_cast<void *>(&on_error),
            .uniffi_free = reinterpret_cast<void *>(&uniffi_free)
        };
    };
}

namespace uniffi {struct RustStreamBuffer: std::basic_streambuf<char> {
    RustStreamBuffer(RustBuffer *buf) {
        char* data = reinterpret_cast<char*>(buf->data);
        this->setg(data, data, data + buf->len);
        this->setp(data, data + buf->capacity);
    }
    ~RustStreamBuffer() = default;

private:
    RustStreamBuffer() = delete;
    RustStreamBuffer(const RustStreamBuffer &) = delete;
    RustStreamBuffer(RustStreamBuffer &&) = delete;

    RustStreamBuffer &operator=(const RustStreamBuffer &) = delete;
    RustStreamBuffer &operator=(RustStreamBuffer &&) = delete;
};

struct RustStream: std::basic_iostream<char> {
    RustStream(RustBuffer *buf):
        std::basic_iostream<char>(&streambuf), streambuf(RustStreamBuffer(buf)) { }

    template <typename T, typename = std::enable_if_t<std::is_arithmetic_v<T>>>
    RustStream &operator>>(T &val) {
        read(reinterpret_cast<char *>(&val), sizeof(T));

        if (std::endian::native != std::endian::big) {
            auto bytes = reinterpret_cast<char *>(&val);

            std::reverse(bytes, bytes + sizeof(T));
        }

        return *this;
    }

    template <typename T, typename = std::enable_if_t<std::is_arithmetic_v<T>>>
    RustStream &operator<<(T val) {
        if (std::endian::native != std::endian::big) {
            auto bytes = reinterpret_cast<char *>(&val);

            std::reverse(bytes, bytes + sizeof(T));
        }

        write(reinterpret_cast<char *>(&val), sizeof(T));

        return *this;
    }
private:
    RustStreamBuffer streambuf;
};


RustBuffer rustbuffer_alloc(uint64_t);
RustBuffer rustbuffer_from_bytes(const ForeignBytes &);
void rustbuffer_free(RustBuffer);
template <typename T> struct HandleMap {
    HandleMap() = default;

    std::shared_ptr<T> at(uint64_t handle) {
        std::lock_guard<std::mutex> guard(this->mutex);

        return this->map.at(handle);
    }

    uint64_t insert(std::shared_ptr<T> impl) {
        std::lock_guard<std::mutex> guard(this->mutex);

        auto handle = this->cur_handle;

        this->map.insert({ handle, impl });
        this->cur_handle += 1;

        return handle;
    }

    void erase(uint64_t handle) {
        // We store the object here to avoid re-entrant locking
        std::shared_ptr<T> cleanup;
        {
            std::lock_guard<std::mutex> guard(this->mutex);
            auto it = this->map.find(handle);
            if (it != this->map.end()) {
                cleanup = it->second;
                this->map.erase(it);
            }
        }
    }
    private:
        HandleMap(const HandleMap<T> &) = delete;
        HandleMap(HandleMap<T> &&) = delete;

        HandleMap<T> &operator=(const HandleMap<T> &) = delete;
        HandleMap<T> &operator=(HandleMap<T> &&) = delete;

        std::mutex mutex;
        uint64_t cur_handle = 0;
        std::map<uint64_t, std::shared_ptr<T>> map;
};
struct FfiConverterUInt8 {
    static uint8_t lift(uint8_t);
    static uint8_t lower(uint8_t);
    static uint8_t read(RustStream &);
    static void write(RustStream &, uint8_t);
    static uint64_t allocation_size(uint8_t);
};
struct FfiConverterInt8 {
    static int8_t lift(int8_t);
    static int8_t lower(int8_t);
    static int8_t read(RustStream &);
    static void write(RustStream &, int8_t);
    static uint64_t allocation_size(int8_t);
};
struct FfiConverterUInt16 {
    static uint16_t lift(uint16_t);
    static uint16_t lower(uint16_t);
    static uint16_t read(RustStream &);
    static void write(RustStream &, uint16_t);
    static uint64_t allocation_size(uint16_t);
};
struct FfiConverterInt16 {
    static int16_t lift(int16_t);
    static int16_t lower(int16_t);
    static int16_t read(RustStream &);
    static void write(RustStream &, int16_t);
    static uint64_t allocation_size(int16_t);
};
struct FfiConverterUInt32 {
    static uint32_t lift(uint32_t);
    static uint32_t lower(uint32_t);
    static uint32_t read(RustStream &);
    static void write(RustStream &, uint32_t);
    static uint64_t allocation_size(uint32_t);
};
struct FfiConverterInt32 {
    static int32_t lift(int32_t);
    static int32_t lower(int32_t);
    static int32_t read(RustStream &);
    static void write(RustStream &, int32_t);
    static uint64_t allocation_size(int32_t);
};
struct FfiConverterUInt64 {
    static uint64_t lift(uint64_t);
    static uint64_t lower(uint64_t);
    static uint64_t read(RustStream &);
    static void write(RustStream &, uint64_t);
    static uint64_t allocation_size(uint64_t);
};
struct FfiConverterInt64 {
    static int64_t lift(int64_t);
    static int64_t lower(int64_t);
    static int64_t read(RustStream &);
    static void write(RustStream &, int64_t);
    static uint64_t allocation_size(int64_t);
};
struct FfiConverterDouble {
    static double lift(double);
    static double lower(double);
    static double read(RustStream &);
    static void write(RustStream &, double);
    static uint64_t allocation_size(double);
};
struct FfiConverterBool {
    static bool lift(uint8_t);
    static uint8_t lower(bool);
    static bool read(RustStream &);
    static void write(RustStream &, bool);
    static uint64_t allocation_size(bool);
};
struct FfiConverterString {
    static std::string lift(RustBuffer buf);
    static RustBuffer lower(const std::string &);
    static std::string read(RustStream &);
    static void write(RustStream &, const std::string &);
    static uint64_t allocation_size(const std::string &);
};


struct FfiConverterToriiClient {
    static std::shared_ptr<ToriiClient> lift(void *);
    static void *lower(const std::shared_ptr<ToriiClient> &);
    static std::shared_ptr<ToriiClient> read(RustStream &);
    static void write(RustStream &, const std::shared_ptr<ToriiClient> &);
    static uint64_t allocation_size(const std::shared_ptr<ToriiClient> &);
private:
};

struct FfiConverterTypeAchievement {
    static Achievement lift(RustBuffer);
    static RustBuffer lower(const Achievement &);
    static Achievement read(RustStream &);
    static void write(RustStream &, const Achievement &);
    static uint64_t allocation_size(const Achievement &);
};

struct FfiConverterTypeAchievementProgression {
    static AchievementProgression lift(RustBuffer);
    static RustBuffer lower(const AchievementProgression &);
    static AchievementProgression read(RustStream &);
    static void write(RustStream &, const AchievementProgression &);
    static uint64_t allocation_size(const AchievementProgression &);
};

struct FfiConverterTypeAchievementQuery {
    static AchievementQuery lift(RustBuffer);
    static RustBuffer lower(const AchievementQuery &);
    static AchievementQuery read(RustStream &);
    static void write(RustStream &, const AchievementQuery &);
    static uint64_t allocation_size(const AchievementQuery &);
};

struct FfiConverterTypeAchievementTask {
    static AchievementTask lift(RustBuffer);
    static RustBuffer lower(const AchievementTask &);
    static AchievementTask read(RustStream &);
    static void write(RustStream &, const AchievementTask &);
    static uint64_t allocation_size(const AchievementTask &);
};

struct FfiConverterTypeActionCount {
    static ActionCount lift(RustBuffer);
    static RustBuffer lower(const ActionCount &);
    static ActionCount read(RustStream &);
    static void write(RustStream &, const ActionCount &);
    static uint64_t allocation_size(const ActionCount &);
};

struct FfiConverterTypeActivity {
    static Activity lift(RustBuffer);
    static RustBuffer lower(const Activity &);
    static Activity read(RustStream &);
    static void write(RustStream &, const Activity &);
    static uint64_t allocation_size(const Activity &);
};

struct FfiConverterTypeActivityQuery {
    static ActivityQuery lift(RustBuffer);
    static RustBuffer lower(const ActivityQuery &);
    static ActivityQuery read(RustStream &);
    static void write(RustStream &, const ActivityQuery &);
    static uint64_t allocation_size(const ActivityQuery &);
};

struct FfiConverterTypeAggregationEntry {
    static AggregationEntry lift(RustBuffer);
    static RustBuffer lower(const AggregationEntry &);
    static AggregationEntry read(RustStream &);
    static void write(RustStream &, const AggregationEntry &);
    static uint64_t allocation_size(const AggregationEntry &);
};

struct FfiConverterTypeAggregationQuery {
    static AggregationQuery lift(RustBuffer);
    static RustBuffer lower(const AggregationQuery &);
    static AggregationQuery read(RustStream &);
    static void write(RustStream &, const AggregationQuery &);
    static uint64_t allocation_size(const AggregationQuery &);
};

struct FfiConverterTypeAttributeFilter {
    static AttributeFilter lift(RustBuffer);
    static RustBuffer lower(const AttributeFilter &);
    static AttributeFilter read(RustStream &);
    static void write(RustStream &, const AttributeFilter &);
    static uint64_t allocation_size(const AttributeFilter &);
};

struct FfiConverterTypeCompositeClause {
    static CompositeClause lift(RustBuffer);
    static RustBuffer lower(const CompositeClause &);
    static CompositeClause read(RustStream &);
    static void write(RustStream &, const CompositeClause &);
    static uint64_t allocation_size(const CompositeClause &);
};

struct FfiConverterTypeContract {
    static Contract lift(RustBuffer);
    static RustBuffer lower(const Contract &);
    static Contract read(RustStream &);
    static void write(RustStream &, const Contract &);
    static uint64_t allocation_size(const Contract &);
};

struct FfiConverterTypeContractQuery {
    static ContractQuery lift(RustBuffer);
    static RustBuffer lower(const ContractQuery &);
    static ContractQuery read(RustStream &);
    static void write(RustStream &, const ContractQuery &);
    static uint64_t allocation_size(const ContractQuery &);
};

struct FfiConverterTypeController {
    static Controller lift(RustBuffer);
    static RustBuffer lower(const Controller &);
    static Controller read(RustStream &);
    static void write(RustStream &, const Controller &);
    static uint64_t allocation_size(const Controller &);
};

struct FfiConverterTypeControllerQuery {
    static ControllerQuery lift(RustBuffer);
    static RustBuffer lower(const ControllerQuery &);
    static ControllerQuery read(RustStream &);
    static void write(RustStream &, const ControllerQuery &);
    static uint64_t allocation_size(const ControllerQuery &);
};

struct FfiConverterTypeEntity {
    static Entity lift(RustBuffer);
    static RustBuffer lower(const Entity &);
    static Entity read(RustStream &);
    static void write(RustStream &, const Entity &);
    static uint64_t allocation_size(const Entity &);
};

struct FfiConverterTypeEnumOption {
    static EnumOption lift(RustBuffer);
    static RustBuffer lower(const EnumOption &);
    static EnumOption read(RustStream &);
    static void write(RustStream &, const EnumOption &);
    static uint64_t allocation_size(const EnumOption &);
};

struct FfiConverterTypeEnumType {
    static EnumType lift(RustBuffer);
    static RustBuffer lower(const EnumType &);
    static EnumType read(RustStream &);
    static void write(RustStream &, const EnumType &);
    static uint64_t allocation_size(const EnumType &);
};

struct FfiConverterTypeEvent {
    static Event lift(RustBuffer);
    static RustBuffer lower(const Event &);
    static Event read(RustStream &);
    static void write(RustStream &, const Event &);
    static uint64_t allocation_size(const Event &);
};

struct FfiConverterTypeEventQuery {
    static EventQuery lift(RustBuffer);
    static RustBuffer lower(const EventQuery &);
    static EventQuery read(RustStream &);
    static void write(RustStream &, const EventQuery &);
    static uint64_t allocation_size(const EventQuery &);
};

struct FfiConverterTypeFixedSizeArray {
    static FixedSizeArray lift(RustBuffer);
    static RustBuffer lower(const FixedSizeArray &);
    static FixedSizeArray read(RustStream &);
    static void write(RustStream &, const FixedSizeArray &);
    static uint64_t allocation_size(const FixedSizeArray &);
};

struct FfiConverterTypeKeysClause {
    static KeysClause lift(RustBuffer);
    static RustBuffer lower(const KeysClause &);
    static KeysClause read(RustStream &);
    static void write(RustStream &, const KeysClause &);
    static uint64_t allocation_size(const KeysClause &);
};

struct FfiConverterTypeMember {
    static Member lift(RustBuffer);
    static RustBuffer lower(const Member &);
    static Member read(RustStream &);
    static void write(RustStream &, const Member &);
    static uint64_t allocation_size(const Member &);
};

struct FfiConverterTypeMemberClause {
    static MemberClause lift(RustBuffer);
    static RustBuffer lower(const MemberClause &);
    static MemberClause read(RustStream &);
    static void write(RustStream &, const MemberClause &);
    static uint64_t allocation_size(const MemberClause &);
};

struct FfiConverterTypeMessage {
    static Message lift(RustBuffer);
    static RustBuffer lower(const Message &);
    static Message read(RustStream &);
    static void write(RustStream &, const Message &);
    static uint64_t allocation_size(const Message &);
};

struct FfiConverterTypeModel {
    static Model lift(RustBuffer);
    static RustBuffer lower(const Model &);
    static Model read(RustStream &);
    static void write(RustStream &, const Model &);
    static uint64_t allocation_size(const Model &);
};

struct FfiConverterTypeOrderBy {
    static OrderBy lift(RustBuffer);
    static RustBuffer lower(const OrderBy &);
    static OrderBy read(RustStream &);
    static void write(RustStream &, const OrderBy &);
    static uint64_t allocation_size(const OrderBy &);
};

struct FfiConverterTypePageAchievement {
    static PageAchievement lift(RustBuffer);
    static RustBuffer lower(const PageAchievement &);
    static PageAchievement read(RustStream &);
    static void write(RustStream &, const PageAchievement &);
    static uint64_t allocation_size(const PageAchievement &);
};

struct FfiConverterTypePageActivity {
    static PageActivity lift(RustBuffer);
    static RustBuffer lower(const PageActivity &);
    static PageActivity read(RustStream &);
    static void write(RustStream &, const PageActivity &);
    static uint64_t allocation_size(const PageActivity &);
};

struct FfiConverterTypePageAggregationEntry {
    static PageAggregationEntry lift(RustBuffer);
    static RustBuffer lower(const PageAggregationEntry &);
    static PageAggregationEntry read(RustStream &);
    static void write(RustStream &, const PageAggregationEntry &);
    static uint64_t allocation_size(const PageAggregationEntry &);
};

struct FfiConverterTypePageController {
    static PageController lift(RustBuffer);
    static RustBuffer lower(const PageController &);
    static PageController read(RustStream &);
    static void write(RustStream &, const PageController &);
    static uint64_t allocation_size(const PageController &);
};

struct FfiConverterTypePageEntity {
    static PageEntity lift(RustBuffer);
    static RustBuffer lower(const PageEntity &);
    static PageEntity read(RustStream &);
    static void write(RustStream &, const PageEntity &);
    static uint64_t allocation_size(const PageEntity &);
};

struct FfiConverterTypePageEvent {
    static PageEvent lift(RustBuffer);
    static RustBuffer lower(const PageEvent &);
    static PageEvent read(RustStream &);
    static void write(RustStream &, const PageEvent &);
    static uint64_t allocation_size(const PageEvent &);
};

struct FfiConverterTypePagePlayerAchievement {
    static PagePlayerAchievement lift(RustBuffer);
    static RustBuffer lower(const PagePlayerAchievement &);
    static PagePlayerAchievement read(RustStream &);
    static void write(RustStream &, const PagePlayerAchievement &);
    static uint64_t allocation_size(const PagePlayerAchievement &);
};

struct FfiConverterTypePageToken {
    static PageToken lift(RustBuffer);
    static RustBuffer lower(const PageToken &);
    static PageToken read(RustStream &);
    static void write(RustStream &, const PageToken &);
    static uint64_t allocation_size(const PageToken &);
};

struct FfiConverterTypePageTokenBalance {
    static PageTokenBalance lift(RustBuffer);
    static RustBuffer lower(const PageTokenBalance &);
    static PageTokenBalance read(RustStream &);
    static void write(RustStream &, const PageTokenBalance &);
    static uint64_t allocation_size(const PageTokenBalance &);
};

struct FfiConverterTypePageTokenContract {
    static PageTokenContract lift(RustBuffer);
    static RustBuffer lower(const PageTokenContract &);
    static PageTokenContract read(RustStream &);
    static void write(RustStream &, const PageTokenContract &);
    static uint64_t allocation_size(const PageTokenContract &);
};

struct FfiConverterTypePageTokenTransfer {
    static PageTokenTransfer lift(RustBuffer);
    static RustBuffer lower(const PageTokenTransfer &);
    static PageTokenTransfer read(RustStream &);
    static void write(RustStream &, const PageTokenTransfer &);
    static uint64_t allocation_size(const PageTokenTransfer &);
};

struct FfiConverterTypePageTransaction {
    static PageTransaction lift(RustBuffer);
    static RustBuffer lower(const PageTransaction &);
    static PageTransaction read(RustStream &);
    static void write(RustStream &, const PageTransaction &);
    static uint64_t allocation_size(const PageTransaction &);
};

struct FfiConverterTypePagination {
    static Pagination lift(RustBuffer);
    static RustBuffer lower(const Pagination &);
    static Pagination read(RustStream &);
    static void write(RustStream &, const Pagination &);
    static uint64_t allocation_size(const Pagination &);
};

struct FfiConverterTypePlayerAchievementEntry {
    static PlayerAchievementEntry lift(RustBuffer);
    static RustBuffer lower(const PlayerAchievementEntry &);
    static PlayerAchievementEntry read(RustStream &);
    static void write(RustStream &, const PlayerAchievementEntry &);
    static uint64_t allocation_size(const PlayerAchievementEntry &);
};

struct FfiConverterTypePlayerAchievementProgress {
    static PlayerAchievementProgress lift(RustBuffer);
    static RustBuffer lower(const PlayerAchievementProgress &);
    static PlayerAchievementProgress read(RustStream &);
    static void write(RustStream &, const PlayerAchievementProgress &);
    static uint64_t allocation_size(const PlayerAchievementProgress &);
};

struct FfiConverterTypePlayerAchievementQuery {
    static PlayerAchievementQuery lift(RustBuffer);
    static RustBuffer lower(const PlayerAchievementQuery &);
    static PlayerAchievementQuery read(RustStream &);
    static void write(RustStream &, const PlayerAchievementQuery &);
    static uint64_t allocation_size(const PlayerAchievementQuery &);
};

struct FfiConverterTypePlayerAchievementStats {
    static PlayerAchievementStats lift(RustBuffer);
    static RustBuffer lower(const PlayerAchievementStats &);
    static PlayerAchievementStats read(RustStream &);
    static void write(RustStream &, const PlayerAchievementStats &);
    static uint64_t allocation_size(const PlayerAchievementStats &);
};

struct FfiConverterTypeQuery {
    static Query lift(RustBuffer);
    static RustBuffer lower(const Query &);
    static Query read(RustStream &);
    static void write(RustStream &, const Query &);
    static uint64_t allocation_size(const Query &);
};

struct FfiConverterTypeSignature {
    static Signature lift(RustBuffer);
    static RustBuffer lower(const Signature &);
    static Signature read(RustStream &);
    static void write(RustStream &, const Signature &);
    static uint64_t allocation_size(const Signature &);
};

struct FfiConverterTypeSqlField {
    static SqlField lift(RustBuffer);
    static RustBuffer lower(const SqlField &);
    static SqlField read(RustStream &);
    static void write(RustStream &, const SqlField &);
    static uint64_t allocation_size(const SqlField &);
};

struct FfiConverterTypeSqlRow {
    static SqlRow lift(RustBuffer);
    static RustBuffer lower(const SqlRow &);
    static SqlRow read(RustStream &);
    static void write(RustStream &, const SqlRow &);
    static uint64_t allocation_size(const SqlRow &);
};

struct FfiConverterTypeStruct {
    static Struct lift(RustBuffer);
    static RustBuffer lower(const Struct &);
    static Struct read(RustStream &);
    static void write(RustStream &, const Struct &);
    static uint64_t allocation_size(const Struct &);
};

struct FfiConverterTypeTaskProgress {
    static TaskProgress lift(RustBuffer);
    static RustBuffer lower(const TaskProgress &);
    static TaskProgress read(RustStream &);
    static void write(RustStream &, const TaskProgress &);
    static uint64_t allocation_size(const TaskProgress &);
};

struct FfiConverterTypeToken {
    static Token lift(RustBuffer);
    static RustBuffer lower(const Token &);
    static Token read(RustStream &);
    static void write(RustStream &, const Token &);
    static uint64_t allocation_size(const Token &);
};

struct FfiConverterTypeTokenBalance {
    static TokenBalance lift(RustBuffer);
    static RustBuffer lower(const TokenBalance &);
    static TokenBalance read(RustStream &);
    static void write(RustStream &, const TokenBalance &);
    static uint64_t allocation_size(const TokenBalance &);
};

struct FfiConverterTypeTokenBalanceQuery {
    static TokenBalanceQuery lift(RustBuffer);
    static RustBuffer lower(const TokenBalanceQuery &);
    static TokenBalanceQuery read(RustStream &);
    static void write(RustStream &, const TokenBalanceQuery &);
    static uint64_t allocation_size(const TokenBalanceQuery &);
};

struct FfiConverterTypeTokenContract {
    static TokenContract lift(RustBuffer);
    static RustBuffer lower(const TokenContract &);
    static TokenContract read(RustStream &);
    static void write(RustStream &, const TokenContract &);
    static uint64_t allocation_size(const TokenContract &);
};

struct FfiConverterTypeTokenContractQuery {
    static TokenContractQuery lift(RustBuffer);
    static RustBuffer lower(const TokenContractQuery &);
    static TokenContractQuery read(RustStream &);
    static void write(RustStream &, const TokenContractQuery &);
    static uint64_t allocation_size(const TokenContractQuery &);
};

struct FfiConverterTypeTokenQuery {
    static TokenQuery lift(RustBuffer);
    static RustBuffer lower(const TokenQuery &);
    static TokenQuery read(RustStream &);
    static void write(RustStream &, const TokenQuery &);
    static uint64_t allocation_size(const TokenQuery &);
};

struct FfiConverterTypeTokenTransfer {
    static TokenTransfer lift(RustBuffer);
    static RustBuffer lower(const TokenTransfer &);
    static TokenTransfer read(RustStream &);
    static void write(RustStream &, const TokenTransfer &);
    static uint64_t allocation_size(const TokenTransfer &);
};

struct FfiConverterTypeTokenTransferQuery {
    static TokenTransferQuery lift(RustBuffer);
    static RustBuffer lower(const TokenTransferQuery &);
    static TokenTransferQuery read(RustStream &);
    static void write(RustStream &, const TokenTransferQuery &);
    static uint64_t allocation_size(const TokenTransferQuery &);
};

struct FfiConverterTypeTransaction {
    static Transaction lift(RustBuffer);
    static RustBuffer lower(const Transaction &);
    static Transaction read(RustStream &);
    static void write(RustStream &, const Transaction &);
    static uint64_t allocation_size(const Transaction &);
};

struct FfiConverterTypeTransactionCall {
    static TransactionCall lift(RustBuffer);
    static RustBuffer lower(const TransactionCall &);
    static TransactionCall read(RustStream &);
    static void write(RustStream &, const TransactionCall &);
    static uint64_t allocation_size(const TransactionCall &);
};

struct FfiConverterTypeTransactionFilter {
    static TransactionFilter lift(RustBuffer);
    static RustBuffer lower(const TransactionFilter &);
    static TransactionFilter read(RustStream &);
    static void write(RustStream &, const TransactionFilter &);
    static uint64_t allocation_size(const TransactionFilter &);
};

struct FfiConverterTypeTransactionQuery {
    static TransactionQuery lift(RustBuffer);
    static RustBuffer lower(const TransactionQuery &);
    static TransactionQuery read(RustStream &);
    static void write(RustStream &, const TransactionQuery &);
    static uint64_t allocation_size(const TransactionQuery &);
};

struct FfiConverterTypeWorld {
    static World lift(RustBuffer);
    static RustBuffer lower(const World &);
    static World read(RustStream &);
    static void write(RustStream &, const World &);
    static uint64_t allocation_size(const World &);
};
struct FfiConverterCallType {
    static CallType lift(RustBuffer);
    static RustBuffer lower(const CallType &);
    static CallType read(RustStream &);
    static void write(RustStream &, const CallType &);
    static uint64_t allocation_size(const CallType &);
};
struct FfiConverterClause {
    static Clause lift(RustBuffer);
    static RustBuffer lower(const Clause &);
    static Clause read(RustStream &);
    static void write(RustStream &, const Clause &);
    static uint64_t allocation_size(const Clause &);
};
struct FfiConverterComparisonOperator {
    static ComparisonOperator lift(RustBuffer);
    static RustBuffer lower(const ComparisonOperator &);
    static ComparisonOperator read(RustStream &);
    static void write(RustStream &, const ComparisonOperator &);
    static uint64_t allocation_size(const ComparisonOperator &);
};
struct FfiConverterContractType {
    static ContractType lift(RustBuffer);
    static RustBuffer lower(const ContractType &);
    static ContractType read(RustStream &);
    static void write(RustStream &, const ContractType &);
    static uint64_t allocation_size(const ContractType &);
};

struct FfiConverterDojoError {
    static std::shared_ptr<DojoError> lift(RustBuffer buf);
    static RustBuffer lower(const DojoError &);
    static std::shared_ptr<DojoError> read(RustStream &stream);
    static void write(RustStream &stream, const DojoError &);
    static uint64_t allocation_size(const DojoError &);
};
struct FfiConverterLogicalOperator {
    static LogicalOperator lift(RustBuffer);
    static RustBuffer lower(const LogicalOperator &);
    static LogicalOperator read(RustStream &);
    static void write(RustStream &, const LogicalOperator &);
    static uint64_t allocation_size(const LogicalOperator &);
};
struct FfiConverterMemberValue {
    static MemberValue lift(RustBuffer);
    static RustBuffer lower(const MemberValue &);
    static MemberValue read(RustStream &);
    static void write(RustStream &, const MemberValue &);
    static uint64_t allocation_size(const MemberValue &);
};
struct FfiConverterOrderDirection {
    static OrderDirection lift(RustBuffer);
    static RustBuffer lower(const OrderDirection &);
    static OrderDirection read(RustStream &);
    static void write(RustStream &, const OrderDirection &);
    static uint64_t allocation_size(const OrderDirection &);
};
struct FfiConverterPaginationDirection {
    static PaginationDirection lift(RustBuffer);
    static RustBuffer lower(const PaginationDirection &);
    static PaginationDirection read(RustStream &);
    static void write(RustStream &, const PaginationDirection &);
    static uint64_t allocation_size(const PaginationDirection &);
};
struct FfiConverterPatternMatching {
    static PatternMatching lift(RustBuffer);
    static RustBuffer lower(const PatternMatching &);
    static PatternMatching read(RustStream &);
    static void write(RustStream &, const PatternMatching &);
    static uint64_t allocation_size(const PatternMatching &);
};
struct FfiConverterPrimitive {
    static Primitive lift(RustBuffer);
    static RustBuffer lower(const Primitive &);
    static Primitive read(RustStream &);
    static void write(RustStream &, const Primitive &);
    static uint64_t allocation_size(const Primitive &);
};
struct FfiConverterSqlValue {
    static SqlValue lift(RustBuffer);
    static RustBuffer lower(const SqlValue &);
    static SqlValue read(RustStream &);
    static void write(RustStream &, const SqlValue &);
    static uint64_t allocation_size(const SqlValue &);
};
struct FfiConverterTy {
    static Ty lift(RustBuffer);
    static RustBuffer lower(const Ty &);
    static Ty read(RustStream &);
    static void write(RustStream &, const Ty &);
    static uint64_t allocation_size(const Ty &);
};
struct FfiConverterValueType {
    static ValueType lift(RustBuffer);
    static RustBuffer lower(const ValueType &);
    static ValueType read(RustStream &);
    static void write(RustStream &, const ValueType &);
    static uint64_t allocation_size(const ValueType &);
};


struct FfiConverterEntityUpdateCallback {
    static std::shared_ptr<EntityUpdateCallback> lift(uint64_t);
    static uint64_t lower(const std::shared_ptr<EntityUpdateCallback> &);
    static std::shared_ptr<EntityUpdateCallback> read(RustStream &);
    static void write(RustStream &, const std::shared_ptr<EntityUpdateCallback> &);
    static uint64_t allocation_size(const std::shared_ptr<EntityUpdateCallback> &);

    inline static HandleMap<EntityUpdateCallback> handle_map = {};
};


struct FfiConverterEventUpdateCallback {
    static std::shared_ptr<EventUpdateCallback> lift(uint64_t);
    static uint64_t lower(const std::shared_ptr<EventUpdateCallback> &);
    static std::shared_ptr<EventUpdateCallback> read(RustStream &);
    static void write(RustStream &, const std::shared_ptr<EventUpdateCallback> &);
    static uint64_t allocation_size(const std::shared_ptr<EventUpdateCallback> &);

    inline static HandleMap<EventUpdateCallback> handle_map = {};
};


struct FfiConverterTokenBalanceUpdateCallback {
    static std::shared_ptr<TokenBalanceUpdateCallback> lift(uint64_t);
    static uint64_t lower(const std::shared_ptr<TokenBalanceUpdateCallback> &);
    static std::shared_ptr<TokenBalanceUpdateCallback> read(RustStream &);
    static void write(RustStream &, const std::shared_ptr<TokenBalanceUpdateCallback> &);
    static uint64_t allocation_size(const std::shared_ptr<TokenBalanceUpdateCallback> &);

    inline static HandleMap<TokenBalanceUpdateCallback> handle_map = {};
};


struct FfiConverterTokenUpdateCallback {
    static std::shared_ptr<TokenUpdateCallback> lift(uint64_t);
    static uint64_t lower(const std::shared_ptr<TokenUpdateCallback> &);
    static std::shared_ptr<TokenUpdateCallback> read(RustStream &);
    static void write(RustStream &, const std::shared_ptr<TokenUpdateCallback> &);
    static uint64_t allocation_size(const std::shared_ptr<TokenUpdateCallback> &);

    inline static HandleMap<TokenUpdateCallback> handle_map = {};
};


struct FfiConverterTransactionUpdateCallback {
    static std::shared_ptr<TransactionUpdateCallback> lift(uint64_t);
    static uint64_t lower(const std::shared_ptr<TransactionUpdateCallback> &);
    static std::shared_ptr<TransactionUpdateCallback> read(RustStream &);
    static void write(RustStream &, const std::shared_ptr<TransactionUpdateCallback> &);
    static uint64_t allocation_size(const std::shared_ptr<TransactionUpdateCallback> &);

    inline static HandleMap<TransactionUpdateCallback> handle_map = {};
};
struct FfiConverterOptionalUInt32 {
    static std::optional<uint32_t> lift(RustBuffer buf);
    static RustBuffer lower(const std::optional<uint32_t>& val);
    static std::optional<uint32_t> read(RustStream &stream);
    static void write(RustStream &stream, const std::optional<uint32_t>& value);
    static uint64_t allocation_size(const std::optional<uint32_t> &val);
};
struct FfiConverterOptionalUInt64 {
    static std::optional<uint64_t> lift(RustBuffer buf);
    static RustBuffer lower(const std::optional<uint64_t>& val);
    static std::optional<uint64_t> read(RustStream &stream);
    static void write(RustStream &stream, const std::optional<uint64_t>& value);
    static uint64_t allocation_size(const std::optional<uint64_t> &val);
};
struct FfiConverterOptionalBool {
    static std::optional<bool> lift(RustBuffer buf);
    static RustBuffer lower(const std::optional<bool>& val);
    static std::optional<bool> read(RustStream &stream);
    static void write(RustStream &stream, const std::optional<bool>& value);
    static uint64_t allocation_size(const std::optional<bool> &val);
};
struct FfiConverterOptionalString {
    static std::optional<std::string> lift(RustBuffer buf);
    static RustBuffer lower(const std::optional<std::string>& val);
    static std::optional<std::string> read(RustStream &stream);
    static void write(RustStream &stream, const std::optional<std::string>& value);
    static uint64_t allocation_size(const std::optional<std::string> &val);
};
struct FfiConverterOptionalTypeKeysClause {
    static std::optional<std::shared_ptr<KeysClause>> lift(RustBuffer buf);
    static RustBuffer lower(const std::optional<std::shared_ptr<KeysClause>>& val);
    static std::optional<std::shared_ptr<KeysClause>> read(RustStream &stream);
    static void write(RustStream &stream, const std::optional<std::shared_ptr<KeysClause>>& value);
    static uint64_t allocation_size(const std::optional<std::shared_ptr<KeysClause>> &val);
};
struct FfiConverterOptionalTypeTransactionFilter {
    static std::optional<std::shared_ptr<TransactionFilter>> lift(RustBuffer buf);
    static RustBuffer lower(const std::optional<std::shared_ptr<TransactionFilter>>& val);
    static std::optional<std::shared_ptr<TransactionFilter>> read(RustStream &stream);
    static void write(RustStream &stream, const std::optional<std::shared_ptr<TransactionFilter>>& value);
    static uint64_t allocation_size(const std::optional<std::shared_ptr<TransactionFilter>> &val);
};
struct FfiConverterOptionalClause {
    static std::optional<std::shared_ptr<Clause>> lift(RustBuffer buf);
    static RustBuffer lower(const std::optional<std::shared_ptr<Clause>>& val);
    static std::optional<std::shared_ptr<Clause>> read(RustStream &stream);
    static void write(RustStream &stream, const std::optional<std::shared_ptr<Clause>>& value);
    static uint64_t allocation_size(const std::optional<std::shared_ptr<Clause>> &val);
};
struct FfiConverterOptionalTypeFieldElement {
    static std::optional<FieldElement> lift(RustBuffer buf);
    static RustBuffer lower(const std::optional<FieldElement>& val);
    static std::optional<FieldElement> read(RustStream &stream);
    static void write(RustStream &stream, const std::optional<FieldElement>& value);
    static uint64_t allocation_size(const std::optional<FieldElement> &val);
};
struct FfiConverterOptionalTypeU256 {
    static std::optional<U256> lift(RustBuffer buf);
    static RustBuffer lower(const std::optional<U256>& val);
    static std::optional<U256> read(RustStream &stream);
    static void write(RustStream &stream, const std::optional<U256>& value);
    static uint64_t allocation_size(const std::optional<U256> &val);
};

struct FfiConverterSequenceUInt8 {
    static std::vector<uint8_t> lift(RustBuffer);
    static RustBuffer lower(const std::vector<uint8_t> &);
    static std::vector<uint8_t> read(RustStream &);
    static void write(RustStream &, const std::vector<uint8_t> &);
    static uint64_t allocation_size(const std::vector<uint8_t> &);
};

struct FfiConverterSequenceString {
    static std::vector<std::string> lift(RustBuffer);
    static RustBuffer lower(const std::vector<std::string> &);
    static std::vector<std::string> read(RustStream &);
    static void write(RustStream &, const std::vector<std::string> &);
    static uint64_t allocation_size(const std::vector<std::string> &);
};

struct FfiConverterSequenceTypeAchievement {
    static std::vector<Achievement> lift(RustBuffer);
    static RustBuffer lower(const std::vector<Achievement> &);
    static std::vector<Achievement> read(RustStream &);
    static void write(RustStream &, const std::vector<Achievement> &);
    static uint64_t allocation_size(const std::vector<Achievement> &);
};

struct FfiConverterSequenceTypeAchievementTask {
    static std::vector<AchievementTask> lift(RustBuffer);
    static RustBuffer lower(const std::vector<AchievementTask> &);
    static std::vector<AchievementTask> read(RustStream &);
    static void write(RustStream &, const std::vector<AchievementTask> &);
    static uint64_t allocation_size(const std::vector<AchievementTask> &);
};

struct FfiConverterSequenceTypeActionCount {
    static std::vector<ActionCount> lift(RustBuffer);
    static RustBuffer lower(const std::vector<ActionCount> &);
    static std::vector<ActionCount> read(RustStream &);
    static void write(RustStream &, const std::vector<ActionCount> &);
    static uint64_t allocation_size(const std::vector<ActionCount> &);
};

struct FfiConverterSequenceTypeActivity {
    static std::vector<Activity> lift(RustBuffer);
    static RustBuffer lower(const std::vector<Activity> &);
    static std::vector<Activity> read(RustStream &);
    static void write(RustStream &, const std::vector<Activity> &);
    static uint64_t allocation_size(const std::vector<Activity> &);
};

struct FfiConverterSequenceTypeAggregationEntry {
    static std::vector<AggregationEntry> lift(RustBuffer);
    static RustBuffer lower(const std::vector<AggregationEntry> &);
    static std::vector<AggregationEntry> read(RustStream &);
    static void write(RustStream &, const std::vector<AggregationEntry> &);
    static uint64_t allocation_size(const std::vector<AggregationEntry> &);
};

struct FfiConverterSequenceTypeAttributeFilter {
    static std::vector<AttributeFilter> lift(RustBuffer);
    static RustBuffer lower(const std::vector<AttributeFilter> &);
    static std::vector<AttributeFilter> read(RustStream &);
    static void write(RustStream &, const std::vector<AttributeFilter> &);
    static uint64_t allocation_size(const std::vector<AttributeFilter> &);
};

struct FfiConverterSequenceTypeContract {
    static std::vector<Contract> lift(RustBuffer);
    static RustBuffer lower(const std::vector<Contract> &);
    static std::vector<Contract> read(RustStream &);
    static void write(RustStream &, const std::vector<Contract> &);
    static uint64_t allocation_size(const std::vector<Contract> &);
};

struct FfiConverterSequenceTypeController {
    static std::vector<Controller> lift(RustBuffer);
    static RustBuffer lower(const std::vector<Controller> &);
    static std::vector<Controller> read(RustStream &);
    static void write(RustStream &, const std::vector<Controller> &);
    static uint64_t allocation_size(const std::vector<Controller> &);
};

struct FfiConverterSequenceTypeEntity {
    static std::vector<std::shared_ptr<Entity>> lift(RustBuffer);
    static RustBuffer lower(const std::vector<std::shared_ptr<Entity>> &);
    static std::vector<std::shared_ptr<Entity>> read(RustStream &);
    static void write(RustStream &, const std::vector<std::shared_ptr<Entity>> &);
    static uint64_t allocation_size(const std::vector<std::shared_ptr<Entity>> &);
};

struct FfiConverterSequenceTypeEnumOption {
    static std::vector<std::shared_ptr<EnumOption>> lift(RustBuffer);
    static RustBuffer lower(const std::vector<std::shared_ptr<EnumOption>> &);
    static std::vector<std::shared_ptr<EnumOption>> read(RustStream &);
    static void write(RustStream &, const std::vector<std::shared_ptr<EnumOption>> &);
    static uint64_t allocation_size(const std::vector<std::shared_ptr<EnumOption>> &);
};

struct FfiConverterSequenceTypeEvent {
    static std::vector<Event> lift(RustBuffer);
    static RustBuffer lower(const std::vector<Event> &);
    static std::vector<Event> read(RustStream &);
    static void write(RustStream &, const std::vector<Event> &);
    static uint64_t allocation_size(const std::vector<Event> &);
};

struct FfiConverterSequenceTypeKeysClause {
    static std::vector<KeysClause> lift(RustBuffer);
    static RustBuffer lower(const std::vector<KeysClause> &);
    static std::vector<KeysClause> read(RustStream &);
    static void write(RustStream &, const std::vector<KeysClause> &);
    static uint64_t allocation_size(const std::vector<KeysClause> &);
};

struct FfiConverterSequenceTypeMember {
    static std::vector<std::shared_ptr<Member>> lift(RustBuffer);
    static RustBuffer lower(const std::vector<std::shared_ptr<Member>> &);
    static std::vector<std::shared_ptr<Member>> read(RustStream &);
    static void write(RustStream &, const std::vector<std::shared_ptr<Member>> &);
    static uint64_t allocation_size(const std::vector<std::shared_ptr<Member>> &);
};

struct FfiConverterSequenceTypeMessage {
    static std::vector<Message> lift(RustBuffer);
    static RustBuffer lower(const std::vector<Message> &);
    static std::vector<Message> read(RustStream &);
    static void write(RustStream &, const std::vector<Message> &);
    static uint64_t allocation_size(const std::vector<Message> &);
};

struct FfiConverterSequenceTypeModel {
    static std::vector<std::shared_ptr<Model>> lift(RustBuffer);
    static RustBuffer lower(const std::vector<std::shared_ptr<Model>> &);
    static std::vector<std::shared_ptr<Model>> read(RustStream &);
    static void write(RustStream &, const std::vector<std::shared_ptr<Model>> &);
    static uint64_t allocation_size(const std::vector<std::shared_ptr<Model>> &);
};

struct FfiConverterSequenceTypeOrderBy {
    static std::vector<OrderBy> lift(RustBuffer);
    static RustBuffer lower(const std::vector<OrderBy> &);
    static std::vector<OrderBy> read(RustStream &);
    static void write(RustStream &, const std::vector<OrderBy> &);
    static uint64_t allocation_size(const std::vector<OrderBy> &);
};

struct FfiConverterSequenceTypePlayerAchievementEntry {
    static std::vector<PlayerAchievementEntry> lift(RustBuffer);
    static RustBuffer lower(const std::vector<PlayerAchievementEntry> &);
    static std::vector<PlayerAchievementEntry> read(RustStream &);
    static void write(RustStream &, const std::vector<PlayerAchievementEntry> &);
    static uint64_t allocation_size(const std::vector<PlayerAchievementEntry> &);
};

struct FfiConverterSequenceTypePlayerAchievementProgress {
    static std::vector<PlayerAchievementProgress> lift(RustBuffer);
    static RustBuffer lower(const std::vector<PlayerAchievementProgress> &);
    static std::vector<PlayerAchievementProgress> read(RustStream &);
    static void write(RustStream &, const std::vector<PlayerAchievementProgress> &);
    static uint64_t allocation_size(const std::vector<PlayerAchievementProgress> &);
};

struct FfiConverterSequenceTypeSqlField {
    static std::vector<SqlField> lift(RustBuffer);
    static RustBuffer lower(const std::vector<SqlField> &);
    static std::vector<SqlField> read(RustStream &);
    static void write(RustStream &, const std::vector<SqlField> &);
    static uint64_t allocation_size(const std::vector<SqlField> &);
};

struct FfiConverterSequenceTypeSqlRow {
    static std::vector<SqlRow> lift(RustBuffer);
    static RustBuffer lower(const std::vector<SqlRow> &);
    static std::vector<SqlRow> read(RustStream &);
    static void write(RustStream &, const std::vector<SqlRow> &);
    static uint64_t allocation_size(const std::vector<SqlRow> &);
};

struct FfiConverterSequenceTypeStruct {
    static std::vector<std::shared_ptr<Struct>> lift(RustBuffer);
    static RustBuffer lower(const std::vector<std::shared_ptr<Struct>> &);
    static std::vector<std::shared_ptr<Struct>> read(RustStream &);
    static void write(RustStream &, const std::vector<std::shared_ptr<Struct>> &);
    static uint64_t allocation_size(const std::vector<std::shared_ptr<Struct>> &);
};

struct FfiConverterSequenceTypeTaskProgress {
    static std::vector<TaskProgress> lift(RustBuffer);
    static RustBuffer lower(const std::vector<TaskProgress> &);
    static std::vector<TaskProgress> read(RustStream &);
    static void write(RustStream &, const std::vector<TaskProgress> &);
    static uint64_t allocation_size(const std::vector<TaskProgress> &);
};

struct FfiConverterSequenceTypeToken {
    static std::vector<Token> lift(RustBuffer);
    static RustBuffer lower(const std::vector<Token> &);
    static std::vector<Token> read(RustStream &);
    static void write(RustStream &, const std::vector<Token> &);
    static uint64_t allocation_size(const std::vector<Token> &);
};

struct FfiConverterSequenceTypeTokenBalance {
    static std::vector<TokenBalance> lift(RustBuffer);
    static RustBuffer lower(const std::vector<TokenBalance> &);
    static std::vector<TokenBalance> read(RustStream &);
    static void write(RustStream &, const std::vector<TokenBalance> &);
    static uint64_t allocation_size(const std::vector<TokenBalance> &);
};

struct FfiConverterSequenceTypeTokenContract {
    static std::vector<TokenContract> lift(RustBuffer);
    static RustBuffer lower(const std::vector<TokenContract> &);
    static std::vector<TokenContract> read(RustStream &);
    static void write(RustStream &, const std::vector<TokenContract> &);
    static uint64_t allocation_size(const std::vector<TokenContract> &);
};

struct FfiConverterSequenceTypeTokenTransfer {
    static std::vector<TokenTransfer> lift(RustBuffer);
    static RustBuffer lower(const std::vector<TokenTransfer> &);
    static std::vector<TokenTransfer> read(RustStream &);
    static void write(RustStream &, const std::vector<TokenTransfer> &);
    static uint64_t allocation_size(const std::vector<TokenTransfer> &);
};

struct FfiConverterSequenceTypeTransaction {
    static std::vector<Transaction> lift(RustBuffer);
    static RustBuffer lower(const std::vector<Transaction> &);
    static std::vector<Transaction> read(RustStream &);
    static void write(RustStream &, const std::vector<Transaction> &);
    static uint64_t allocation_size(const std::vector<Transaction> &);
};

struct FfiConverterSequenceTypeTransactionCall {
    static std::vector<TransactionCall> lift(RustBuffer);
    static RustBuffer lower(const std::vector<TransactionCall> &);
    static std::vector<TransactionCall> read(RustStream &);
    static void write(RustStream &, const std::vector<TransactionCall> &);
    static uint64_t allocation_size(const std::vector<TransactionCall> &);
};

struct FfiConverterSequenceTypeWorld {
    static std::vector<std::shared_ptr<World>> lift(RustBuffer);
    static RustBuffer lower(const std::vector<std::shared_ptr<World>> &);
    static std::vector<std::shared_ptr<World>> read(RustStream &);
    static void write(RustStream &, const std::vector<std::shared_ptr<World>> &);
    static uint64_t allocation_size(const std::vector<std::shared_ptr<World>> &);
};

struct FfiConverterSequenceClause {
    static std::vector<std::shared_ptr<Clause>> lift(RustBuffer);
    static RustBuffer lower(const std::vector<std::shared_ptr<Clause>> &);
    static std::vector<std::shared_ptr<Clause>> read(RustStream &);
    static void write(RustStream &, const std::vector<std::shared_ptr<Clause>> &);
    static uint64_t allocation_size(const std::vector<std::shared_ptr<Clause>> &);
};

struct FfiConverterSequenceContractType {
    static std::vector<ContractType> lift(RustBuffer);
    static RustBuffer lower(const std::vector<ContractType> &);
    static std::vector<ContractType> read(RustStream &);
    static void write(RustStream &, const std::vector<ContractType> &);
    static uint64_t allocation_size(const std::vector<ContractType> &);
};

struct FfiConverterSequenceMemberValue {
    static std::vector<std::shared_ptr<MemberValue>> lift(RustBuffer);
    static RustBuffer lower(const std::vector<std::shared_ptr<MemberValue>> &);
    static std::vector<std::shared_ptr<MemberValue>> read(RustStream &);
    static void write(RustStream &, const std::vector<std::shared_ptr<MemberValue>> &);
    static uint64_t allocation_size(const std::vector<std::shared_ptr<MemberValue>> &);
};

struct FfiConverterSequenceTy {
    static std::vector<std::shared_ptr<Ty>> lift(RustBuffer);
    static RustBuffer lower(const std::vector<std::shared_ptr<Ty>> &);
    static std::vector<std::shared_ptr<Ty>> read(RustStream &);
    static void write(RustStream &, const std::vector<std::shared_ptr<Ty>> &);
    static uint64_t allocation_size(const std::vector<std::shared_ptr<Ty>> &);
};

struct FfiConverterSequenceOptionalTypeFieldElement {
    static std::vector<std::optional<FieldElement>> lift(RustBuffer);
    static RustBuffer lower(const std::vector<std::optional<FieldElement>> &);
    static std::vector<std::optional<FieldElement>> read(RustStream &);
    static void write(RustStream &, const std::vector<std::optional<FieldElement>> &);
    static uint64_t allocation_size(const std::vector<std::optional<FieldElement>> &);
};

struct FfiConverterSequenceTypeFieldElement {
    static std::vector<FieldElement> lift(RustBuffer);
    static RustBuffer lower(const std::vector<FieldElement> &);
    static std::vector<FieldElement> read(RustStream &);
    static void write(RustStream &, const std::vector<FieldElement> &);
    static uint64_t allocation_size(const std::vector<FieldElement> &);
};

struct FfiConverterSequenceTypeU256 {
    static std::vector<U256> lift(RustBuffer);
    static RustBuffer lower(const std::vector<U256> &);
    static std::vector<U256> read(RustStream &);
    static void write(RustStream &, const std::vector<U256> &);
    static uint64_t allocation_size(const std::vector<U256> &);
};
typedef struct FfiConverterString FfiConverterTypeFieldElement;
typedef struct FfiConverterString FfiConverterTypeU256;
} // namespace uniffi

} // namespace dojo