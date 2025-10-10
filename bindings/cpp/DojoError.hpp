#ifndef DojoError_HPP
#define DojoError_HPP

#include "DojoError.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "ErrorType.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {

    diplomat::capi::DojoError* DojoError_new(diplomat::capi::ErrorType error_type, diplomat::capi::DiplomatStringView message);

    void DojoError_message(const diplomat::capi::DojoError* self, diplomat::capi::DiplomatWrite* write);

    diplomat::capi::ErrorType DojoError_error_type(const diplomat::capi::DojoError* self);

    void DojoError_destroy(DojoError* self);

    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<DojoError>, diplomat::Utf8Error> DojoError::new_(ErrorType error_type, std::string_view message) {
  if (!diplomat::capi::diplomat_is_str(message.data(), message.size())) {
    return diplomat::Err<diplomat::Utf8Error>();
  }
  auto result = diplomat::capi::DojoError_new(error_type.AsFFI(),
    {message.data(), message.size()});
  return diplomat::Ok<std::unique_ptr<DojoError>>(std::unique_ptr<DojoError>(DojoError::FromFFI(result)));
}

inline std::string DojoError::message() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::DojoError_message(this->AsFFI(),
    &write);
  return output;
}
template<typename W>
inline void DojoError::message_write(W& writeable) const {
  diplomat::capi::DiplomatWrite write = diplomat::WriteTrait<W>::Construct(writeable);
  diplomat::capi::DojoError_message(this->AsFFI(),
    &write);
}

inline ErrorType DojoError::error_type() const {
  auto result = diplomat::capi::DojoError_error_type(this->AsFFI());
  return ErrorType::FromFFI(result);
}

inline const diplomat::capi::DojoError* DojoError::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::DojoError*>(this);
}

inline diplomat::capi::DojoError* DojoError::AsFFI() {
  return reinterpret_cast<diplomat::capi::DojoError*>(this);
}

inline const DojoError* DojoError::FromFFI(const diplomat::capi::DojoError* ptr) {
  return reinterpret_cast<const DojoError*>(ptr);
}

inline DojoError* DojoError::FromFFI(diplomat::capi::DojoError* ptr) {
  return reinterpret_cast<DojoError*>(ptr);
}

inline void DojoError::operator delete(void* ptr) {
  diplomat::capi::DojoError_destroy(reinterpret_cast<diplomat::capi::DojoError*>(ptr));
}


#endif // DojoError_HPP
