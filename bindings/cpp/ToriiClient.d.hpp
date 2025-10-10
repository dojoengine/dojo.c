#ifndef ToriiClient_D_HPP
#define ToriiClient_D_HPP

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
namespace diplomat::capi { struct FieldElement; }
class FieldElement;


namespace diplomat {
namespace capi {
    struct ToriiClient;
} // namespace capi
} // namespace

/**
 * Opaque handle to a Torii client instance
 */
class ToriiClient {
public:

  /**
   * Creates a new Torii client instance
   */
  inline static diplomat::result<std::unique_ptr<ToriiClient>, std::unique_ptr<DojoError>> new_(std::string_view torii_url);

  /**
   * Gets information about the Torii server
   */
  inline diplomat::result<std::string, std::unique_ptr<DojoError>> info() const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> info_write(W& writeable_output) const;

  /**
   * Publishes a message to the network
   */
  inline diplomat::result<std::string, std::unique_ptr<DojoError>> publish_message(std::string_view message_json, const FieldElement& signature_r, const FieldElement& signature_s, const FieldElement& world_address) const;
  template<typename W>
  inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> publish_message_write(std::string_view message_json, const FieldElement& signature_r, const FieldElement& signature_s, const FieldElement& world_address, W& writeable_output) const;

  inline const diplomat::capi::ToriiClient* AsFFI() const;
  inline diplomat::capi::ToriiClient* AsFFI();
  inline static const ToriiClient* FromFFI(const diplomat::capi::ToriiClient* ptr);
  inline static ToriiClient* FromFFI(diplomat::capi::ToriiClient* ptr);
  inline static void operator delete(void* ptr);
private:
  ToriiClient() = delete;
  ToriiClient(const ToriiClient&) = delete;
  ToriiClient(ToriiClient&&) noexcept = delete;
  ToriiClient operator=(const ToriiClient&) = delete;
  ToriiClient operator=(ToriiClient&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ToriiClient_D_HPP
