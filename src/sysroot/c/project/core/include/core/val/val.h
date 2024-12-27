#ifndef CORE_VAL_H
#define CORE_VAL_H

#include "core/mem/mem-api.h"
#include "val-kind.h"

struct val {
  u1 kind;
  u1 mem_realm;
};

HAMAL_API void
val_init (struct val *self, enum val_kind kind, struct mem *mem);

struct dep_val {
  enum val_kind kind;
  struct mem *mem;
//  u1 kind;
//  u1 mem_realm;
};

HAMAL_API void
dep_val_init (struct dep_val *self, enum val_kind kind, struct mem *mem);

#endif //CORE_VAL_H
