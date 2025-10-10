#ifndef Provider_D_HPP
#define Provider_D_HPP

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
    struct Provider;
} // namespace capi
} // namespace

/**
 * JSON-RPC provider for Starknet
 */
class Provider {
public:

  /**
   * Creates a new provider from an RPC URL
   */
  inline static diplomat::result<std::unique_ptr<Provider>, std::unique_ptr<DojoError>> new_(std::string_view rpc_url);

  /**
   * Gets the chain ID
   */
  inline diplomat::result<std::string, std::unique_ptr<DojoError>> chain_id() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> chain_id_write(W& writeable_output) const;

  /**
   * Gets the latest block number
   */
  inline diplomat::result<uint64_t, std::unique_ptr<DojoError>> block_number() const;

  inline const diplomat::capi::Provider* AsFFI() const;
  inline diplomat::capi::Provider* AsFFI();
  inline static const Provider* FromFFI(const diplomat::capi::Provider* ptr);
  inline static Provider* FromFFI(diplomat::capi::Provider* ptr);
  inline static void operator delete(void* ptr);
private:
  Provider() = delete;
  Provider(const Provider&) = delete;
  Provider(Provider&&) noexcept = delete;
  Provider operator=(const Provider&) = delete;
  Provider operator=(Provider&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // Provider_D_HPP
