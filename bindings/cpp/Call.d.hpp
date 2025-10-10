#ifndef Call_D_HPP
#define Call_D_HPP

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
namespace diplomat::capi { struct FieldElement; }
class FieldElement;


namespace diplomat {
namespace capi {
    struct Call;
} // namespace capi
} // namespace

/**
 * Represents a Starknet call
 */
class Call {
public:

  /**
   * Creates a new Call
   */
  inline static std::unique_ptr<Call> new_(const FieldElement& to, const FieldElement& selector);

  /**
   * Adds a field element to the calldata
   */
  inline void push_calldata(const FieldElement& felt);

  /**
   * Creates a call from selector name
   */
  inline static diplomat::result<std::unique_ptr<Call>, std::unique_ptr<DojoError>> new_from_selector_name(const FieldElement& to, std::string_view selector_name);

  inline const diplomat::capi::Call* AsFFI() const;
  inline diplomat::capi::Call* AsFFI();
  inline static const Call* FromFFI(const diplomat::capi::Call* ptr);
  inline static Call* FromFFI(diplomat::capi::Call* ptr);
  inline static void operator delete(void* ptr);
private:
  Call() = delete;
  Call(const Call&) = delete;
  Call(Call&&) noexcept = delete;
  Call operator=(const Call&) = delete;
  Call operator=(Call&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // Call_D_HPP
