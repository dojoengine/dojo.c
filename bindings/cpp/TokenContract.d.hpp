#ifndef TokenContract_D_HPP
#define TokenContract_D_HPP

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
    struct TokenContract;
} // namespace capi
} // namespace

/**
 * Represents a token contract
 */
class TokenContract {
public:

  /**
   * Gets the contract address (hex)
   */
  inline std::string contract_address() const;
  template<typename W>
  inline void contract_address_write(W& writeable_output) const;

  /**
   * Gets the contract type as string
   */
  inline std::string contract_type() const;
  template<typename W>
  inline void contract_type_write(W& writeable_output) const;

  /**
   * Creates a token contract from JSON
   */
  inline static diplomat::result<std::unique_ptr<TokenContract>, std::unique_ptr<DojoError>> from_json(std::string_view json);

  /**
   * Serializes the token contract to JSON
   */
  inline diplomat::result<std::string, std::unique_ptr<DojoError>> to_json() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> to_json_write(W& writeable_output) const;

  inline const diplomat::capi::TokenContract* AsFFI() const;
  inline diplomat::capi::TokenContract* AsFFI();
  inline static const TokenContract* FromFFI(const diplomat::capi::TokenContract* ptr);
  inline static TokenContract* FromFFI(diplomat::capi::TokenContract* ptr);
  inline static void operator delete(void* ptr);
private:
  TokenContract() = delete;
  TokenContract(const TokenContract&) = delete;
  TokenContract(TokenContract&&) noexcept = delete;
  TokenContract operator=(const TokenContract&) = delete;
  TokenContract operator=(TokenContract&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // TokenContract_D_HPP
