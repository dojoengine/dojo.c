#ifndef KeysClause_D_HPP
#define KeysClause_D_HPP

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
    struct KeysClause;
} // namespace capi
} // namespace

/**
 * Keys clause for filtering by entity keys
 */
class KeysClause {
public:

  /**
   * Creates a new keys clause
   */
  inline static std::unique_ptr<KeysClause> new_();

  /**
   * Adds a key to filter by
   */
  inline void add_key(const FieldElement& key);

  /**
   * Adds a model to filter by
   */
  inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> add_model(std::string_view model);

  inline const diplomat::capi::KeysClause* AsFFI() const;
  inline diplomat::capi::KeysClause* AsFFI();
  inline static const KeysClause* FromFFI(const diplomat::capi::KeysClause* ptr);
  inline static KeysClause* FromFFI(diplomat::capi::KeysClause* ptr);
  inline static void operator delete(void* ptr);
private:
  KeysClause() = delete;
  KeysClause(const KeysClause&) = delete;
  KeysClause(KeysClause&&) noexcept = delete;
  KeysClause operator=(const KeysClause&) = delete;
  KeysClause operator=(KeysClause&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // KeysClause_D_HPP
