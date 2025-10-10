#ifndef AggregationEntry_HPP
#define AggregationEntry_HPP

#include "AggregationEntry.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "DojoError.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {

    void AggregationEntry_id(const diplomat::capi::AggregationEntry* self, diplomat::capi::DiplomatWrite* write);

    void AggregationEntry_aggregator_id(const diplomat::capi::AggregationEntry* self, diplomat::capi::DiplomatWrite* write);

    void AggregationEntry_entity_id(const diplomat::capi::AggregationEntry* self, diplomat::capi::DiplomatWrite* write);

    void AggregationEntry_model_id(const diplomat::capi::AggregationEntry* self, diplomat::capi::DiplomatWrite* write);

    void AggregationEntry_value(const diplomat::capi::AggregationEntry* self, diplomat::capi::DiplomatWrite* write);

    void AggregationEntry_display_value(const diplomat::capi::AggregationEntry* self, diplomat::capi::DiplomatWrite* write);

    uint64_t AggregationEntry_position(const diplomat::capi::AggregationEntry* self);

    uint64_t AggregationEntry_created_at(const diplomat::capi::AggregationEntry* self);

    uint64_t AggregationEntry_updated_at(const diplomat::capi::AggregationEntry* self);

    typedef struct AggregationEntry_from_json_result {union {diplomat::capi::AggregationEntry* ok; diplomat::capi::DojoError* err;}; bool is_ok;} AggregationEntry_from_json_result;
    AggregationEntry_from_json_result AggregationEntry_from_json(diplomat::capi::DiplomatStringView json);

    typedef struct AggregationEntry_to_json_result {union { diplomat::capi::DojoError* err;}; bool is_ok;} AggregationEntry_to_json_result;
    AggregationEntry_to_json_result AggregationEntry_to_json(const diplomat::capi::AggregationEntry* self, diplomat::capi::DiplomatWrite* write);

    void AggregationEntry_destroy(AggregationEntry* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::string AggregationEntry::id() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::AggregationEntry_id(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void AggregationEntry::id_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::AggregationEntry_id(this->AsFFI(),
    &write);
}

inline std::string AggregationEntry::aggregator_id() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::AggregationEntry_aggregator_id(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void AggregationEntry::aggregator_id_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::AggregationEntry_aggregator_id(this->AsFFI(),
    &write);
}

inline std::string AggregationEntry::entity_id() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::AggregationEntry_entity_id(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void AggregationEntry::entity_id_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::AggregationEntry_entity_id(this->AsFFI(),
    &write);
}

inline std::string AggregationEntry::model_id() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::AggregationEntry_model_id(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void AggregationEntry::model_id_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::AggregationEntry_model_id(this->AsFFI(),
    &write);
}

inline std::string AggregationEntry::value() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::AggregationEntry_value(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void AggregationEntry::value_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::AggregationEntry_value(this->AsFFI(),
    &write);
}

inline std::string AggregationEntry::display_value() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::AggregationEntry_display_value(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void AggregationEntry::display_value_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::AggregationEntry_display_value(this->AsFFI(),
    &write);
}

inline uint64_t AggregationEntry::position() const {
  auto result = diplomat::capi::AggregationEntry_position(this->AsFFI());
  return result;
}

inline uint64_t AggregationEntry::created_at() const {
  auto result = diplomat::capi::AggregationEntry_created_at(this->AsFFI());
  return result;
}

inline uint64_t AggregationEntry::updated_at() const {
  auto result = diplomat::capi::AggregationEntry_updated_at(this->AsFFI());
  return result;
}

inline diplomat::result<std::unique_ptr<AggregationEntry>, std::unique_ptr<DojoError>> AggregationEntry::from_json(std::string_view json) {
  auto result = diplomat::capi::AggregationEntry_from_json({json.data(), json.size()});
  return result.is_ok ? diplomat::result<std::unique_ptr<AggregationEntry>, std::unique_ptr<DojoError>>(diplomat::Ok<std::unique_ptr<AggregationEntry>>(std::unique_ptr<AggregationEntry>(AggregationEntry::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<AggregationEntry>, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<DojoError>> AggregationEntry::to_json() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::AggregationEntry_to_json(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> AggregationEntry::to_json_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  auto result = diplomat::capi::AggregationEntry_to_json(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline const diplomat::capi::AggregationEntry* AggregationEntry::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::AggregationEntry*>(this);
}

inline diplomat::capi::AggregationEntry* AggregationEntry::AsFFI() {
  return reinterpret_cast<diplomat::capi::AggregationEntry*>(this);
}

inline const AggregationEntry* AggregationEntry::FromFFI(const diplomat::capi::AggregationEntry* ptr) {
  return reinterpret_cast<const AggregationEntry*>(ptr);
}

inline AggregationEntry* AggregationEntry::FromFFI(diplomat::capi::AggregationEntry* ptr) {
  return reinterpret_cast<AggregationEntry*>(ptr);
}

inline void AggregationEntry::operator delete(void* ptr) {
  diplomat::capi::AggregationEntry_destroy(reinterpret_cast<diplomat::capi::AggregationEntry*>(ptr));
}


#endif // AggregationEntry_HPP
