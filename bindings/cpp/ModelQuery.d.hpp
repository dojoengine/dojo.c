#ifndef ModelQuery_D_HPP
#define ModelQuery_D_HPP

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
    struct ModelQuery;
} // namespace capi
} // namespace

/**
 * Model query for retrieving model definitions
 */
class ModelQuery {
public:

  /**
   * Creates a new model query
   */
  inline static std::unique_ptr<ModelQuery> new_();

  /**
   * Adds a model name to query
   */
  inline diplomat::result<std::monostate, std::unique_ptr<DojoError>> add_model(std::string_view model_name);

  inline const diplomat::capi::ModelQuery* AsFFI() const;
  inline diplomat::capi::ModelQuery* AsFFI();
  inline static const ModelQuery* FromFFI(const diplomat::capi::ModelQuery* ptr);
  inline static ModelQuery* FromFFI(diplomat::capi::ModelQuery* ptr);
  inline static void operator delete(void* ptr);
private:
  ModelQuery() = delete;
  ModelQuery(const ModelQuery&) = delete;
  ModelQuery(ModelQuery&&) noexcept = delete;
  ModelQuery operator=(const ModelQuery&) = delete;
  ModelQuery operator=(ModelQuery&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ModelQuery_D_HPP
