#ifndef CallList_D_HPP
#define CallList_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"

namespace diplomat::capi { struct Call; }
class Call;


namespace diplomat {
namespace capi {
    struct CallList;
} // namespace capi
} // namespace

/**
 * List of calls for batch transactions
 */
class CallList {
public:

  /**
   * Creates a new empty call list
   */
  inline static std::unique_ptr<CallList> new_();

  /**
   * Adds a call to the list
   */
  inline void add_call(const Call& call);

  /**
   * Returns the number of calls in the list
   */
  inline size_t len() const;

  inline const diplomat::capi::CallList* AsFFI() const;
  inline diplomat::capi::CallList* AsFFI();
  inline static const CallList* FromFFI(const diplomat::capi::CallList* ptr);
  inline static CallList* FromFFI(diplomat::capi::CallList* ptr);
  inline static void operator delete(void* ptr);
private:
  CallList() = delete;
  CallList(const CallList&) = delete;
  CallList(CallList&&) noexcept = delete;
  CallList operator=(const CallList&) = delete;
  CallList operator=(CallList&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // CallList_D_HPP
