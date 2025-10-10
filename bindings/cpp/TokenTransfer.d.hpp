#ifndef TokenTransfer_D_HPP
#define TokenTransfer_D_HPP

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
    struct TokenTransfer;
} // namespace capi
} // namespace

/**
 * Represents a token transfer event
 */
class TokenTransfer {
public:

  /**
   * Gets the from address (hex)
   */
  inline std::string from_address() const;
  template<typename W>
  inline void from_address_write(W& writeable_output) const;

  /**
   * Gets the to address (hex)
   */
  inline std::string to_address() const;
  template<typename W>
  inline void to_address_write(W& writeable_output) const;

  /**
   * Gets the contract address (hex)
   */
  inline std::string contract_address() const;
  template<typename W>
  inline void contract_address_write(W& writeable_output) const;

  /**
   * Gets the amount as a string
   */
  inline std::string amount() const;
  template<typename W>
  inline void amount_write(W& writeable_output) const;

  /**
   * Gets the executed_at timestamp
   */
  inline uint64_t executed_at() const;

  /**
   * Creates a token transfer from JSON
   */
  inline static diplomat::result<std::unique_ptr<TokenTransfer>, std::unique_ptr<DojoError>> from_json(std::string_view json);

  /**
   * Serializes the token transfer to JSON
   */
  inline diplomat::result<std::string, std::unique_ptr<DojoError>> to_json() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> to_json_write(W& writeable_output) const;

  inline const diplomat::capi::TokenTransfer* AsFFI() const;
  inline diplomat::capi::TokenTransfer* AsFFI();
  inline static const TokenTransfer* FromFFI(const diplomat::capi::TokenTransfer* ptr);
  inline static TokenTransfer* FromFFI(diplomat::capi::TokenTransfer* ptr);
  inline static void operator delete(void* ptr);
private:
  TokenTransfer() = delete;
  TokenTransfer(const TokenTransfer&) = delete;
  TokenTransfer(TokenTransfer&&) noexcept = delete;
  TokenTransfer operator=(const TokenTransfer&) = delete;
  TokenTransfer operator=(TokenTransfer&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // TokenTransfer_D_HPP
