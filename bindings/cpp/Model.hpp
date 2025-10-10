#ifndef Model_HPP
#define Model_HPP

#include "Model.d.hpp"

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

    void Model_name(const diplomat::capi::Model* self, diplomat::capi::DiplomatWrite* write);

    void Model_namespace(const diplomat::capi::Model* self, diplomat::capi::DiplomatWrite* write);

    void Model_selector(const diplomat::capi::Model* self, diplomat::capi::DiplomatWrite* write);

    void Model_class_hash(const diplomat::capi::Model* self, diplomat::capi::DiplomatWrite* write);

    void Model_contract_address(const diplomat::capi::Model* self, diplomat::capi::DiplomatWrite* write);

    uint32_t Model_packed_size(const diplomat::capi::Model* self);

    uint32_t Model_unpacked_size(const diplomat::capi::Model* self);

    bool Model_use_legacy_store(const diplomat::capi::Model* self);

    typedef struct Model_from_json_result {union {diplomat::capi::Model* ok; diplomat::capi::DojoError* err;}; bool is_ok;} Model_from_json_result;
    Model_from_json_result Model_from_json(diplomat::capi::DiplomatStringView json);

    typedef struct Model_to_json_result {union { diplomat::capi::DojoError* err;}; bool is_ok;} Model_to_json_result;
    Model_to_json_result Model_to_json(const diplomat::capi::Model* self, diplomat::capi::DiplomatWrite* write);

    void Model_destroy(Model* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::string Model::name() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::Model_name(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void Model::name_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::Model_name(this->AsFFI(),
    &write);
}

inline std::string Model::namespace_() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::Model_namespace(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void Model::namespace__write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::Model_namespace(this->AsFFI(),
    &write);
}

inline std::string Model::selector() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::Model_selector(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void Model::selector_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::Model_selector(this->AsFFI(),
    &write);
}

inline std::string Model::class_hash() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::Model_class_hash(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void Model::class_hash_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::Model_class_hash(this->AsFFI(),
    &write);
}

inline std::string Model::contract_address() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::Model_contract_address(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void Model::contract_address_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::Model_contract_address(this->AsFFI(),
    &write);
}

inline uint32_t Model::packed_size() const {
  auto result = diplomat::capi::Model_packed_size(this->AsFFI());
  return result;
}

inline uint32_t Model::unpacked_size() const {
  auto result = diplomat::capi::Model_unpacked_size(this->AsFFI());
  return result;
}

inline bool Model::use_legacy_store() const {
  auto result = diplomat::capi::Model_use_legacy_store(this->AsFFI());
  return result;
}

inline diplomat::result<std::unique_ptr<Model>, std::unique_ptr<DojoError>> Model::from_json(std::string_view json) {
  auto result = diplomat::capi::Model_from_json({json.data(), json.size()});
  return result.is_ok ? diplomat::result<std::unique_ptr<Model>, std::unique_ptr<DojoError>>(diplomat::Ok<std::unique_ptr<Model>>(std::unique_ptr<Model>(Model::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<Model>, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline diplomat::result<std::string, std::unique_ptr<DojoError>> Model::to_json() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::Model_to_json(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}
template<typename W>
inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> Model::to_json_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  auto result = diplomat::capi::Model_to_json(this->AsFFI(),
    &write);
  return result.is_ok ? diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, std::unique_ptr<DojoError>>(diplomat::Err<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result.err))));
}

inline const diplomat::capi::Model* Model::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::Model*>(this);
}

inline diplomat::capi::Model* Model::AsFFI() {
  return reinterpret_cast<diplomat::capi::Model*>(this);
}

inline const Model* Model::FromFFI(const diplomat::capi::Model* ptr) {
  return reinterpret_cast<const Model*>(ptr);
}

inline Model* Model::FromFFI(diplomat::capi::Model* ptr) {
  return reinterpret_cast<Model*>(ptr);
}

inline void Model::operator delete(void* ptr) {
  diplomat::capi::Model_destroy(reinterpret_cast<diplomat::capi::Model*>(ptr));
}


#endif // Model_HPP
