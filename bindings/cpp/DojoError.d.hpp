#ifndef DojoError_D_HPP
#define DojoError_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"

class ErrorType;


namespace diplomat {
namespace capi {
    struct DojoError;
} // namespace capi
} // namespace

/**
 * Error types for Dojo operations
 */
class DojoError {
public:

  inline static diplomat::result<std::unique_ptr<DojoError>, diplomat::Utf8Error> new_(ErrorType error_type, std::string_view message);

  /**
   * Gets the error message
   */
  inline std::string message() const;
  template<typename W>
  inline void message_write(W& writeable_output) const;

  /**
   * Gets the error type
   */
  inline ErrorType error_type() const;

  inline const diplomat::capi::DojoError* AsFFI() const;
  inline diplomat::capi::DojoError* AsFFI();
  inline static const DojoError* FromFFI(const diplomat::capi::DojoError* ptr);
  inline static DojoError* FromFFI(diplomat::capi::DojoError* ptr);
  inline static void operator delete(void* ptr);
private:
  DojoError() = delete;
  DojoError(const DojoError&) = delete;
  DojoError(DojoError&&) noexcept = delete;
  DojoError operator=(const DojoError&) = delete;
  DojoError operator=(DojoError&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // DojoError_D_HPP
