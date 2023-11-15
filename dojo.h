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

typedef enum Primitive_Tag {
  U8,
  U16,
  U32,
  U64,
  U128,
  U256,
  USize,
  Bool,
  Felt252,
  ClassHash,
  ContractAddress,
} Primitive_Tag;

typedef struct Primitive {
  Primitive_Tag tag;
  union {
    struct {
      const uint8_t *u8;
    };
    struct {
      const uint16_t *u16;
    };
    struct {
      const uint32_t *u32;
    };
    struct {
      const uint64_t *u64;
    };
    struct {
      uint8_t u128[16];
    };
    struct {
      uint64_t u256[4];
    };
    struct {
      const uint32_t *u_size;
    };
    struct {
      const bool *bool_;
    };
    struct {
      const struct FieldElement *felt252;
    };
    struct {
      const struct FieldElement *class_hash;
    };
    struct {
      const struct FieldElement *contract_address;
    };
  };
} Primitive;

typedef struct Member {
  const char *name;
  struct Ty ty;
  bool key;
} Member;

typedef struct CArray_Member {
  const struct Member *data;
  uintptr_t data_len;
} CArray_Member;

typedef struct Struct {
  const char *name;
  struct CArray_Member children;
} Struct;

typedef struct EnumOption {
  const char *name;
  struct Ty ty;
} EnumOption;

typedef struct CArray_EnumOption {
  const struct EnumOption *data;
  uintptr_t data_len;
} CArray_EnumOption;

typedef struct Enum {
  const char *name;
  uint8_t option;
  struct CArray_EnumOption options;
} Enum;

typedef struct CArray_Ty {
  const struct Ty *data;
  uintptr_t data_len;
} CArray_Ty;

typedef enum Ty_Tag {
  Primitive,
  Struct,
  Enum,
  Tuple,
} Ty_Tag;

typedef struct Ty {
  Ty_Tag tag;
  union {
    struct {
      struct Primitive primitive;
    };
    struct {
      struct Struct struct_;
    };
    struct {
      struct Enum enum_;
    };
    struct {
      struct CArray_Ty tuple;
    };
  };
} Ty;

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

typedef struct ModelMetadata {
  struct Ty schema;
  const char *name;
  uint32_t packed_size;
  uint32_t unpacked_size;
  struct FieldElement class_hash;
  struct CArray_FieldElement layout;
} ModelMetadata;

typedef struct CHashMap______c_char__ModelMetadata {
  const char *const *keys;
  const struct ModelMetadata *values;
  uintptr_t len;
} CHashMap______c_char__ModelMetadata;

typedef struct WorldMetadata {
  struct FieldElement world_address;
  struct FieldElement world_class_hash;
  struct FieldElement executor_address;
  struct FieldElement executor_class_hash;
  struct CHashMap______c_char__ModelMetadata models;
} WorldMetadata;

typedef struct Client {
  struct ToriiClient *client;
  struct Ty *(*entity)(struct ToriiClient *client,
                       const struct EntityQuery *entity,
                       struct Error *error);
  struct WorldMetadata (*metadata)(struct ToriiClient *client);
  void (*add_entities_to_sync)(struct ToriiClient *client,
                               const struct EntityQuery *entities,
                               uintptr_t entities_len,
                               struct Error *error);
  void (*remove_entities_to_sync)(struct ToriiClient *client,
                                  const struct EntityQuery *entities,
                                  uintptr_t entities_len,
                                  struct Error *error);
  void (*free)(struct ToriiClient *client);
} Client;

struct Client *client_new(const char *torii_url,
                          const char *rpc_url,
                          const struct FieldElement *world,
                          const struct EntityQuery *entities,
                          uintptr_t entities_len,
                          struct Error *error);
