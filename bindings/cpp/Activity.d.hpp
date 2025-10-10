#ifndef Activity_D_HPP
#define Activity_D_HPP

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
    struct Activity;
} // namespace capi
} // namespace

/**
 * Represents an activity
 */
class Activity {
public:

  /**
   * Gets the activity ID
   */
  inline std::string id() const;
  template<typename W>
  inline void id_write(W& writeable_output) const;

  /**
   * Gets the world address (hex)
   */
  inline std::string world_address() const;
  template<typename W>
  inline void world_address_write(W& writeable_output) const;

  /**
   * Gets the namespace
   */
  inline std::string namespace_() const;
  template<typename W>
  inline void namespace__write(W& writeable_output) const;

  /**
   * Gets the caller address (hex)
   */
  inline std::string caller_address() const;
  template<typename W>
  inline void caller_address_write(W& writeable_output) const;

  /**
   * Gets the session start timestamp
   */
  inline uint64_t session_start() const;

  /**
   * Gets the session end timestamp
   */
  inline uint64_t session_end() const;

  /**
   * Gets the action count
   */
  inline uint32_t action_count() const;

  /**
   * Gets the updated_at timestamp
   */
  inline uint64_t updated_at() const;

  /**
   * Creates an activity from JSON
   */
  inline static diplomat::result<std::unique_ptr<Activity>, std::unique_ptr<DojoError>> from_json(std::string_view json);

  /**
   * Serializes the activity to JSON
   */
  inline diplomat::result<std::string, std::unique_ptr<DojoError>> to_json() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> to_json_write(W& writeable_output) const;

  inline const diplomat::capi::Activity* AsFFI() const;
  inline diplomat::capi::Activity* AsFFI();
  inline static const Activity* FromFFI(const diplomat::capi::Activity* ptr);
  inline static Activity* FromFFI(diplomat::capi::Activity* ptr);
  inline static void operator delete(void* ptr);
private:
  Activity() = delete;
  Activity(const Activity&) = delete;
  Activity(Activity&&) noexcept = delete;
  Activity operator=(const Activity&) = delete;
  Activity operator=(Activity&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // Activity_D_HPP
