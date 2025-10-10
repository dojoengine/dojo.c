#ifndef Achievement_D_HPP
#define Achievement_D_HPP

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
    struct Achievement;
} // namespace capi
} // namespace

/**
 * Represents an achievement
 */
class Achievement {
public:

  /**
   * Gets the achievement ID
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
   * Gets the achievement title
   */
  inline std::string title() const;
  template<typename W>
  inline void title_write(W& writeable_output) const;

  /**
   * Gets the achievement description
   */
  inline std::string description() const;
  template<typename W>
  inline void description_write(W& writeable_output) const;

  /**
   * Gets the hidden flag
   */
  inline bool hidden() const;

  /**
   * Gets the icon URI
   */
  inline std::string icon() const;
  template<typename W>
  inline void icon_write(W& writeable_output) const;

  /**
   * Gets the points for this achievement
   */
  inline uint32_t points() const;

  /**
   * Creates an achievement from JSON
   */
  inline static diplomat::result<std::unique_ptr<Achievement>, std::unique_ptr<DojoError>> from_json(std::string_view json);

  /**
   * Serializes the achievement to JSON
   */
  inline diplomat::result<std::string, std::unique_ptr<DojoError>> to_json() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> to_json_write(W& writeable_output) const;

  inline const diplomat::capi::Achievement* AsFFI() const;
  inline diplomat::capi::Achievement* AsFFI();
  inline static const Achievement* FromFFI(const diplomat::capi::Achievement* ptr);
  inline static Achievement* FromFFI(diplomat::capi::Achievement* ptr);
  inline static void operator delete(void* ptr);
private:
  Achievement() = delete;
  Achievement(const Achievement&) = delete;
  Achievement(Achievement&&) noexcept = delete;
  Achievement operator=(const Achievement&) = delete;
  Achievement operator=(Achievement&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // Achievement_D_HPP
