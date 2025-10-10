#ifndef Model_D_HPP
#define Model_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"

namespace diplomat::capi { struct DojoError; }
class DojoError;


namespace diplomat {
namespace capi {
    struct Model;
} // namespace capi
} // namespace

/**
 * Represents a Dojo model
 */
class Model {
public:

  /**
   * Gets the model name
   */
  inline std::string name() const;
  template<typename W>
  inline void name_write(W& writeable_output) const;

  /**
   * Gets the model namespace
   */
  inline std::string namespace_() const;
  template<typename W>
  inline void namespace__write(W& writeable_output) const;

  /**
   * Gets the model selector (hex)
   */
  inline std::string selector() const;
  template<typename W>
  inline void selector_write(W& writeable_output) const;

  /**
   * Gets the model class hash (hex)
   */
  inline std::string class_hash() const;
  template<typename W>
  inline void class_hash_write(W& writeable_output) const;

  /**
   * Gets the model contract address (hex)
   */
  inline std::string contract_address() const;
  template<typename W>
  inline void contract_address_write(W& writeable_output) const;

  /**
   * Gets the packed size
   */
  inline uint32_t packed_size() const;

  /**
   * Gets the unpacked size
   */
  inline uint32_t unpacked_size() const;

  /**
   * Returns true if the model uses legacy store
   */
  inline bool use_legacy_store() const;

  /**
   * Creates a new model from JSON
   */
  inline static diplomat::result<std::unique_ptr<Model>, std::unique_ptr<DojoError>> from_json(std::string_view json);

  /**
   * Serializes the model to JSON
   */
  inline diplomat::result<std::string, std::unique_ptr<DojoError>> to_json() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> to_json_write(W& writeable_output) const;

  inline const diplomat::capi::Model* AsFFI() const;
  inline diplomat::capi::Model* AsFFI();
  inline static const Model* FromFFI(const diplomat::capi::Model* ptr);
  inline static Model* FromFFI(diplomat::capi::Model* ptr);
  inline static void operator delete(void* ptr);
private:
  Model() = delete;
  Model(const Model&) = delete;
  Model(Model&&) noexcept = delete;
  Model operator=(const Model&) = delete;
  Model operator=(Model&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // Model_D_HPP
