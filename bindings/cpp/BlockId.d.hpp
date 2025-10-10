#ifndef BlockId_D_HPP
#define BlockId_D_HPP

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
class BlockTag;


namespace diplomat {
namespace capi {
    struct BlockId;
} // namespace capi
} // namespace

/**
 * Block identifier (hash, number, or tag)
 */
class BlockId {
public:

  /**
   * Creates a BlockId from a block hash
   */
  inline static std::unique_ptr<BlockId> from_hash(const FieldElement& hash);

  /**
   * Creates a BlockId from a block number
   */
  inline static std::unique_ptr<BlockId> from_number(uint64_t number);

  /**
   * Creates a BlockId from a block tag
   */
  inline static std::unique_ptr<BlockId> from_tag(BlockTag tag);

  inline const diplomat::capi::BlockId* AsFFI() const;
  inline diplomat::capi::BlockId* AsFFI();
  inline static const BlockId* FromFFI(const diplomat::capi::BlockId* ptr);
  inline static BlockId* FromFFI(diplomat::capi::BlockId* ptr);
  inline static void operator delete(void* ptr);
private:
  BlockId() = delete;
  BlockId(const BlockId&) = delete;
  BlockId(BlockId&&) noexcept = delete;
  BlockId operator=(const BlockId&) = delete;
  BlockId operator=(BlockId&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // BlockId_D_HPP
