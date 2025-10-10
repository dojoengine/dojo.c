#ifndef Contract_D_HPP
#define Contract_D_HPP

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
    struct Contract;
} // namespace capi
} // namespace

/**
 * Represents a contract
 */
class Contract {
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
   * Gets the head block number (if any)
   */
  inline uint64_t head() const;

  /**
   * Gets the TPS (transactions per second) if available
   */
  inline uint64_t tps() const;

  /**
   * Gets the created_at timestamp
   */
  inline uint64_t created_at() const;

  /**
   * Gets the updated_at timestamp
   */
  inline uint64_t updated_at() const;

  /**
   * Creates a contract from JSON
   */
  inline static diplomat::result<std::unique_ptr<Contract>, std::unique_ptr<DojoError>> from_json(std::string_view json);

  /**
   * Serializes the contract to JSON
   */
  inline diplomat::result<std::string, std::unique_ptr<DojoError>> to_json() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> to_json_write(W& writeable_output) const;

  inline const diplomat::capi::Contract* AsFFI() const;
  inline diplomat::capi::Contract* AsFFI();
  inline static const Contract* FromFFI(const diplomat::capi::Contract* ptr);
  inline static Contract* FromFFI(diplomat::capi::Contract* ptr);
  inline static void operator delete(void* ptr);
private:
  Contract() = delete;
  Contract(const Contract&) = delete;
  Contract(Contract&&) noexcept = delete;
  Contract operator=(const Contract&) = delete;
  Contract operator=(Contract&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // Contract_D_HPP
