#ifndef AggregationEntry_D_HPP
#define AggregationEntry_D_HPP

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
    struct AggregationEntry;
} // namespace capi
} // namespace

/**
 * Represents an aggregation entry
 */
class AggregationEntry {
public:

  /**
   * Gets the aggregation ID
   */
  inline std::string id() const;
  template<typename W>
  inline void id_write(W& writeable_output) const;

  /**
   * Gets the aggregator ID
   */
  inline std::string aggregator_id() const;
  template<typename W>
  inline void aggregator_id_write(W& writeable_output) const;

  /**
   * Gets the entity ID
   */
  inline std::string entity_id() const;
  template<typename W>
  inline void entity_id_write(W& writeable_output) const;

  /**
   * Gets the model ID
   */
  inline std::string model_id() const;
  template<typename W>
  inline void model_id_write(W& writeable_output) const;

  /**
   * Gets the aggregation value
   */
  inline std::string value() const;
  template<typename W>
  inline void value_write(W& writeable_output) const;

  /**
   * Gets the display value
   */
  inline std::string display_value() const;
  template<typename W>
  inline void display_value_write(W& writeable_output) const;

  /**
   * Gets the position
   */
  inline uint64_t position() const;

  /**
   * Gets the created_at timestamp
   */
  inline uint64_t created_at() const;

  /**
   * Gets the updated_at timestamp
   */
  inline uint64_t updated_at() const;

  /**
   * Creates an aggregation entry from JSON
   */
  inline static diplomat::result<std::unique_ptr<AggregationEntry>, std::unique_ptr<DojoError>> from_json(std::string_view json);

  /**
   * Serializes the aggregation entry to JSON
   */
  inline diplomat::result<std::string, std::unique_ptr<DojoError>> to_json() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> to_json_write(W& writeable_output) const;

  inline const diplomat::capi::AggregationEntry* AsFFI() const;
  inline diplomat::capi::AggregationEntry* AsFFI();
  inline static const AggregationEntry* FromFFI(const diplomat::capi::AggregationEntry* ptr);
  inline static AggregationEntry* FromFFI(diplomat::capi::AggregationEntry* ptr);
  inline static void operator delete(void* ptr);
private:
  AggregationEntry() = delete;
  AggregationEntry(const AggregationEntry&) = delete;
  AggregationEntry(AggregationEntry&&) noexcept = delete;
  AggregationEntry operator=(const AggregationEntry&) = delete;
  AggregationEntry operator=(AggregationEntry&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // AggregationEntry_D_HPP
