#ifndef Enum_D_HPP
#define Enum_D_HPP

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
    struct Enum;
} // namespace capi
} // namespace

/**
 * Represents a Dojo enum
 */
class Enum {
public:

  /**
   * Gets the enum name
   */
  inline std::string name() const;
  template<typename W>
  inline void name_write(W& writeable_output) const;

  /**
   * Gets the current option (selected variant) index
   */
  inline uint8_t option() const;

  /**
   * Gets the number of options (variants)
   */
  inline uint32_t options_count() const;

  /**
   * Creates a new enum from JSON schema
   */
  inline static diplomat::result<std::unique_ptr<Enum>, std::unique_ptr<DojoError>> from_json(std::string_view json);

  /**
   * Serializes the enum to JSON
   */
  inline diplomat::result<std::string, std::unique_ptr<DojoError>> to_json() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> to_json_write(W& writeable_output) const;

  inline const diplomat::capi::Enum* AsFFI() const;
  inline diplomat::capi::Enum* AsFFI();
  inline static const Enum* FromFFI(const diplomat::capi::Enum* ptr);
  inline static Enum* FromFFI(diplomat::capi::Enum* ptr);
  inline static void operator delete(void* ptr);
private:
  Enum() = delete;
  Enum(const Enum&) = delete;
  Enum(Enum&&) noexcept = delete;
  Enum operator=(const Enum&) = delete;
  Enum operator=(Enum&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // Enum_D_HPP
