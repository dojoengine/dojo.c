#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum ComparisonOperator {
  Eq,
  Neq,
  Gt,
  Gte,
  Lt,
  Lte,
} ComparisonOperator;

typedef enum LogicalOperator {
  And,
  Or,
} LogicalOperator;

typedef struct ToriiClient ToriiClient;

typedef struct FieldElement {
  uint8_t data[32];
} FieldElement;

typedef struct CArray_FieldElement {
  const struct FieldElement *data;
  uintptr_t data_len;
} CArray_FieldElement;

typedef struct CArray_FieldElement KeysClause;

typedef struct CArray_u8 {
  const uint8_t *data;
  uintptr_t data_len;
} CArray_u8;

typedef enum Value_Tag {
  String,
  Int,
  UInt,
  Bool,
  Bytes,
} Value_Tag;

typedef struct Value {
  Value_Tag tag;
  union {
    struct {
      const char *string;
    };
    struct {
      int64_t int_;
    };
    struct {
      uint64_t u_int;
    };
    struct {
      bool bool_;
    };
    struct {
      struct CArray_u8 bytes;
    };
  };
} Value;

typedef struct AttributeClause {
  const char *attribute;
  enum ComparisonOperator operator_;
  struct Value value;
} AttributeClause;

typedef struct CompositeClause {
  enum LogicalOperator operator_;
  const struct Clause *clauses;
  uintptr_t clauses_len;
} CompositeClause;

typedef enum Clause_Tag {
  Keys,
  Attribute,
  Composite,
} Clause_Tag;

typedef struct Clause {
  Clause_Tag tag;
  union {
    struct {
      KeysClause keys;
    };
    struct {
      struct AttributeClause attribute;
    };
    struct {
      struct CompositeClause composite;
    };
  };
} Clause;

typedef struct EntityQuery {
  const char *model;
  struct Clause clause;
} EntityQuery;

typedef struct Error {
  const char *message;
} Error;

struct ToriiClient *client_new(const char *torii_url,
                               const char *rpc_url,
                               const struct FieldElement *world,
                               const struct EntityQuery *entities,
                               uintptr_t entities_len,
                               struct Error *error);

Ty *client_entity(struct ToriiClient *client,
                  const struct EntityQuery *entity,
                  struct Error *error);

void client_add_entities_to_sync(struct ToriiClient *client,
                                 const struct EntityQuery *entities,
                                 uintptr_t entities_len,
                                 struct Error *error);

void client_remove_entities_to_sync(struct ToriiClient *client,
                                    const struct EntityQuery *entities,
                                    uintptr_t entities_len,
                                    struct Error *error);

void client_free(struct ToriiClient *client);
