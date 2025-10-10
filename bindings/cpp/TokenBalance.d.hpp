#ifndef TokenBalance_D_HPP
#define TokenBalance_D_HPP

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
    struct TokenBalance;
} // namespace capi
} // namespace

/**
 * Represents a token balance for an account
 */
class TokenBalance {
public:

  /**
   * Gets the account address (hex)
   */
  inline std::string account_address() const;
  template<typename W>
  inline void account_address_write(W& writeable_output) const;

  /**
   * Gets the contract address (hex)
   */
  inline std::string contract_address() const;
  template<typename W>
  inline void contract_address_write(W& writeable_output) const;

  /**
   * Gets the balance as a string
   */
  inline std::string balance() const;
  template<typename W>
  inline void balance_write(W& writeable_output) const;

  /**
   * Creates a token balance from JSON
   */
  inline static diplomat::result<std::unique_ptr<TokenBalance>, std::unique_ptr<DojoError>> from_json(std::string_view json);

  /**
   * Serializes the token balance to JSON
   */
  inline diplomat::result<std::string, std::unique_ptr<DojoError>> to_json() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> to_json_write(W& writeable_output) const;

  inline const diplomat::capi::TokenBalance* AsFFI() const;
  inline diplomat::capi::TokenBalance* AsFFI();
  inline static const TokenBalance* FromFFI(const diplomat::capi::TokenBalance* ptr);
  inline static TokenBalance* FromFFI(diplomat::capi::TokenBalance* ptr);
  inline static void operator delete(void* ptr);
private:
  TokenBalance() = delete;
  TokenBalance(const TokenBalance&) = delete;
  TokenBalance(TokenBalance&&) noexcept = delete;
  TokenBalance operator=(const TokenBalance&) = delete;
  TokenBalance operator=(TokenBalance&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // TokenBalance_D_HPP
