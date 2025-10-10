#ifndef Member_D_HPP
#define Member_D_HPP

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
    struct Member;
} // namespace capi
} // namespace

/**
 * Represents a Dojo struct member (field)
 */
class Member {
public:

  /**
   * Gets the member name
   */
  inline std::string name() const;
  template<typename W>
  inline void name_write(W& writeable_output) const;

  /**
   * Returns true if this member is a key
   */
  inline bool is_key() const;

  inline const diplomat::capi::Member* AsFFI() const;
  inline diplomat::capi::Member* AsFFI();
  inline static const Member* FromFFI(const diplomat::capi::Member* ptr);
  inline static Member* FromFFI(diplomat::capi::Member* ptr);
  inline static void operator delete(void* ptr);
private:
  Member() = delete;
  Member(const Member&) = delete;
  Member(Member&&) noexcept = delete;
  Member operator=(const Member&) = delete;
  Member operator=(Member&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // Member_D_HPP
