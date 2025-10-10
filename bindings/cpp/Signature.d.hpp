#ifndef Signature_D_HPP
#define Signature_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"

namespace diplomat::capi { struct FieldElement; }
class FieldElement;


namespace diplomat {
namespace capi {
    struct Signature;
} // namespace capi
} // namespace

/**
 * Represents a signature (r, s pair)
 */
class Signature {
public:

  /**
   * Creates a new signature from r and s components
   */
  inline static std::unique_ptr<Signature> new_(const FieldElement& r, const FieldElement& s);

  /**
   * Gets the r component
   */
  inline std::string r() const;
  template<typename W>
  inline void r_write(W& writeable_output) const;

  /**
   * Gets the s component
   */
  inline std::string s() const;
  template<typename W>
  inline void s_write(W& writeable_output) const;

  inline const diplomat::capi::Signature* AsFFI() const;
  inline diplomat::capi::Signature* AsFFI();
  inline static const Signature* FromFFI(const diplomat::capi::Signature* ptr);
  inline static Signature* FromFFI(diplomat::capi::Signature* ptr);
  inline static void operator delete(void* ptr);
private:
  Signature() = delete;
  Signature(const Signature&) = delete;
  Signature(Signature&&) noexcept = delete;
  Signature operator=(const Signature&) = delete;
  Signature operator=(Signature&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // Signature_D_HPP
