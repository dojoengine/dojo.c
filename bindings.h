#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct FieldElement {
  uint8_t data[32];
};

struct EntityModel {
  const char *model;
  const FieldElement *keys;
  uintptr_t keys_len;
};

struct Error {
  const char *message;
};

extern "C" {

Client *client_new(const char *torii_url,
                   const char *rpc_url,
                   const FieldElement *world,
                   const EntityModel *entities,
                   uintptr_t entities_len,
                   Error *error);

} // extern "C"
