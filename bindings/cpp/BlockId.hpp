#ifndef BlockId_HPP
#define BlockId_HPP

#include "BlockId.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "BlockTag.hpp"
#include "FieldElement.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {

    diplomat::capi::BlockId* BlockId_from_hash(const diplomat::capi::FieldElement* hash);

    diplomat::capi::BlockId* BlockId_from_number(uint64_t number);

    diplomat::capi::BlockId* BlockId_from_tag(diplomat::capi::BlockTag tag);

    void BlockId_destroy(BlockId* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<BlockId> BlockId::from_hash(const FieldElement& hash) {
  auto result = diplomat::capi::BlockId_from_hash(hash.AsFFI());
  return std::unique_ptr<BlockId>(BlockId::FromFFI(result));
}

inline std::unique_ptr<BlockId> BlockId::from_number(uint64_t number) {
  auto result = diplomat::capi::BlockId_from_number(number);
  return std::unique_ptr<BlockId>(BlockId::FromFFI(result));
}

inline std::unique_ptr<BlockId> BlockId::from_tag(BlockTag tag) {
  auto result = diplomat::capi::BlockId_from_tag(tag.AsFFI());
  return std::unique_ptr<BlockId>(BlockId::FromFFI(result));
}

inline const diplomat::capi::BlockId* BlockId::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::BlockId*>(this);
}

inline diplomat::capi::BlockId* BlockId::AsFFI() {
  return reinterpret_cast<diplomat::capi::BlockId*>(this);
}

inline const BlockId* BlockId::FromFFI(const diplomat::capi::BlockId* ptr) {
  return reinterpret_cast<const BlockId*>(ptr);
}

inline BlockId* BlockId::FromFFI(diplomat::capi::BlockId* ptr) {
  return reinterpret_cast<BlockId*>(ptr);
}

inline void BlockId::operator delete(void* ptr) {
  diplomat::capi::BlockId_destroy(reinterpret_cast<diplomat::capi::BlockId*>(ptr));
}


#endif // BlockId_HPP
