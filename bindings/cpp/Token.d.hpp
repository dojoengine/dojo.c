#ifndef Token_D_HPP
#define Token_D_HPP

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
    struct Token;
} // namespace capi
} // namespace

/**
 * Represents a token (ERC20, ERC721, or ERC1155)
 */
class Token {
public:

  /**
   * Gets the contract address (hex)
   */
  inline std::string contract_address() const;
  template<typename W>
  inline void contract_address_write(W& writeable_output) const;

  /**
   * Gets the token name
   */
  inline std::string name() const;
  template<typename W>
  inline void name_write(W& writeable_output) const;

  /**
   * Gets the token symbol
   */
  inline std::string symbol() const;
  template<typename W>
  inline void symbol_write(W& writeable_output) const;

  /**
   * Gets the token decimals
   */
  inline uint8_t decimals() const;

  /**
   * Gets the metadata as JSON string
   */
  inline std::string metadata() const;
  template<typename W>
  inline void metadata_write(W& writeable_output) const;

  /**
   * Creates a token from JSON
   */
  inline static diplomat::result<std::unique_ptr<Token>, std::unique_ptr<DojoError>> from_json(std::string_view json);

  /**
   * Serializes the token to JSON
   */
  inline diplomat::result<std::string, std::unique_ptr<DojoError>> to_json() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> to_json_write(W& writeable_output) const;

  inline const diplomat::capi::Token* AsFFI() const;
  inline diplomat::capi::Token* AsFFI();
  inline static const Token* FromFFI(const diplomat::capi::Token* ptr);
  inline static Token* FromFFI(diplomat::capi::Token* ptr);
  inline static void operator delete(void* ptr);
private:
  Token() = delete;
  Token(const Token&) = delete;
  Token(Token&&) noexcept = delete;
  Token operator=(const Token&) = delete;
  Token operator=(Token&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // Token_D_HPP
