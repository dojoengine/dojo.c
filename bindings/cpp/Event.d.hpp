#ifndef Event_D_HPP
#define Event_D_HPP

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
    struct Event;
} // namespace capi
} // namespace

/**
 * Represents a Dojo event message
 */
class Event {
public:

  /**
   * Gets the keys as JSON array string
   */
  inline std::string keys() const;
  template<typename W>
  inline void keys_write(W& writeable_output) const;

  /**
   * Gets the data as JSON array string
   */
  inline std::string data() const;
  template<typename W>
  inline void data_write(W& writeable_output) const;

  /**
   * Gets the transaction hash (hex)
   */
  inline std::string transaction_hash() const;
  template<typename W>
  inline void transaction_hash_write(W& writeable_output) const;

  /**
   * Creates an event from JSON
   */
  inline static diplomat::result<std::unique_ptr<Event>, std::unique_ptr<DojoError>> from_json(std::string_view json);

  /**
   * Serializes the event to JSON
   */
  inline diplomat::result<std::string, std::unique_ptr<DojoError>> to_json() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> to_json_write(W& writeable_output) const;

  inline const diplomat::capi::Event* AsFFI() const;
  inline diplomat::capi::Event* AsFFI();
  inline static const Event* FromFFI(const diplomat::capi::Event* ptr);
  inline static Event* FromFFI(diplomat::capi::Event* ptr);
  inline static void operator delete(void* ptr);
private:
  Event() = delete;
  Event(const Event&) = delete;
  Event(Event&&) noexcept = delete;
  Event operator=(const Event&) = delete;
  Event operator=(Event&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // Event_D_HPP
