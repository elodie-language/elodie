#ifndef CORE_VAL_H
#define CORE_VAL_H

#include "core/mem/mem-api.h"
#include "val-kind.h"

struct val {
  enum val_kind kind;
  struct mem *mem;
};

ELODIE_API void
val_init (struct val *self, enum val_kind kind, struct mem *mem);

#endif //CORE_VAL_H
