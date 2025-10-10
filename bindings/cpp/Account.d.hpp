#ifndef Account_D_HPP
#define Account_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"

namespace diplomat::capi { struct CallList; }
class CallList;
namespace diplomat::capi { struct DojoError; }
class DojoError;
namespace diplomat::capi { struct FieldElement; }
class FieldElement;
namespace diplomat::capi { struct Provider; }
class Provider;
namespace diplomat::capi { struct SigningKey; }
class SigningKey;


namespace diplomat {
namespace capi {
    struct Account;
} // namespace capi
} // namespace

/**
 * Starknet account for signing and executing transactions
 */
class Account {
public:

  /**
   * Creates a new account
   */
  inline static std::unique_ptr<Account> new_(const Provider& provider, const SigningKey& signer, const FieldElement& address, const FieldElement& chain_id);

  /**
   * Gets the account address
   */
  inline std::string address() const;
  template<typename W>
  inline void address_write(W& writeable_output) const;

  /**
   * Gets the chain ID
   */
  inline std::string chain_id() const;
  template<typename W>
  inline void chain_id_write(W& writeable_output) const;

  /**
   * Executes a transaction with the given calls
   */
  inline diplomat::result<std::string, std::unique_ptr<DojoError>> execute(const CallList& calls) const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> execute_write(const CallList& calls, W& writeable_output) const;

  /**
   * Gets the nonce for the account
   */
  inline diplomat::result<uint64_t, std::unique_ptr<DojoError>> nonce() const;

  inline const diplomat::capi::Account* AsFFI() const;
  inline diplomat::capi::Account* AsFFI();
  inline static const Account* FromFFI(const diplomat::capi::Account* ptr);
  inline static Account* FromFFI(diplomat::capi::Account* ptr);
  inline static void operator delete(void* ptr);
private:
  Account() = delete;
  Account(const Account&) = delete;
  Account(Account&&) noexcept = delete;
  Account operator=(const Account&) = delete;
  Account operator=(Account&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // Account_D_HPP
