#ifndef EnumOption_D_HPP
#define EnumOption_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    struct EnumOption;
} // namespace capi
} // namespace

/**
 * Represents a Dojo enum option (variant)
 */
class EnumOption {
public:

  /**
   * Gets the option name
   */
  inline std::string name() const;
  template<typename W>
  inline void name_write(W& writeable_output) const;

  inline const diplomat::capi::EnumOption* AsFFI() const;
  inline diplomat::capi::EnumOption* AsFFI();
  inline static const EnumOption* FromFFI(const diplomat::capi::EnumOption* ptr);
  inline static EnumOption* FromFFI(diplomat::capi::EnumOption* ptr);
  inline static void operator delete(void* ptr);
private:
  EnumOption() = delete;
  EnumOption(const EnumOption&) = delete;
  EnumOption(EnumOption&&) noexcept = delete;
  EnumOption operator=(const EnumOption&) = delete;
  EnumOption operator=(EnumOption&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // EnumOption_D_HPP
