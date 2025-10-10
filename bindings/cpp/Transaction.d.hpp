#ifndef Transaction_D_HPP
#define Transaction_D_HPP

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
    struct Transaction;
} // namespace capi
} // namespace

/**
 * Represents a transaction
 */
class Transaction {
public:

  /**
   * Gets the transaction hash (hex)
   */
  inline std::string transaction_hash() const;
  template<typename W>
  inline void transaction_hash_write(W& writeable_output) const;

  /**
   * Gets the sender address (hex)
   */
  inline std::string sender_address() const;
  template<typename W>
  inline void sender_address_write(W& writeable_output) const;

  /**
   * Gets the max fee as string
   */
  inline std::string max_fee() const;
  template<typename W>
  inline void max_fee_write(W& writeable_output) const;

  /**
   * Gets the signature as JSON array string
   */
  inline std::string signature() const;
  template<typename W>
  inline void signature_write(W& writeable_output) const;

  /**
   * Gets the nonce as string
   */
  inline std::string nonce() const;
  template<typename W>
  inline void nonce_write(W& writeable_output) const;

  /**
   * Gets the block timestamp
   */
  inline uint64_t block_timestamp() const;

  /**
   * Gets the block number
   */
  inline uint64_t block_number() const;

  /**
   * Creates a transaction from JSON
   */
  inline static diplomat::result<std::unique_ptr<Transaction>, std::unique_ptr<DojoError>> from_json(std::string_view json);

  /**
   * Serializes the transaction to JSON
   */
  inline diplomat::result<std::string, std::unique_ptr<DojoError>> to_json() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> to_json_write(W& writeable_output) const;

  inline const diplomat::capi::Transaction* AsFFI() const;
  inline diplomat::capi::Transaction* AsFFI();
  inline static const Transaction* FromFFI(const diplomat::capi::Transaction* ptr);
  inline static Transaction* FromFFI(diplomat::capi::Transaction* ptr);
  inline static void operator delete(void* ptr);
private:
  Transaction() = delete;
  Transaction(const Transaction&) = delete;
  Transaction(Transaction&&) noexcept = delete;
  Transaction operator=(const Transaction&) = delete;
  Transaction operator=(Transaction&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // Transaction_D_HPP
