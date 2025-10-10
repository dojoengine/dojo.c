#ifndef Struct_D_HPP
#define Struct_D_HPP

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
    struct Struct;
} // namespace capi
} // namespace

/**
 * Represents a Dojo struct
 */
class Struct {
public:

  /**
   * Gets the struct name
   */
  inline std::string name() const;
  template<typename W>
  inline void name_write(W& writeable_output) const;

  /**
   * Gets the number of children (members)
   */
  inline uint32_t children_count() const;

  /**
   * Creates a new struct from JSON schema
   */
  inline static diplomat::result<std::unique_ptr<Struct>, std::unique_ptr<DojoError>> from_json(std::string_view json);

  /**
   * Serializes the struct to JSON
   */
  inline diplomat::result<std::string, std::unique_ptr<DojoError>> to_json() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> to_json_write(W& writeable_output) const;

  inline const diplomat::capi::Struct* AsFFI() const;
  inline diplomat::capi::Struct* AsFFI();
  inline static const Struct* FromFFI(const diplomat::capi::Struct* ptr);
  inline static Struct* FromFFI(diplomat::capi::Struct* ptr);
  inline static void operator delete(void* ptr);
private:
  Struct() = delete;
  Struct(const Struct&) = delete;
  Struct(Struct&&) noexcept = delete;
  Struct operator=(const Struct&) = delete;
  Struct operator=(Struct&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // Struct_D_HPP
