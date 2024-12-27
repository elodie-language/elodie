#ifndef CORE_VAL_UNIT_H
#define CORE_VAL_UNIT_H

#include "val.h"

struct val_unit {
  struct dep_val base;
};

HAMAL_API struct val_unit *
val_unit_new (struct mem *mem);

HAMAL_API bool
val_unit_equal (struct val_unit *lhs, struct val_unit *rhs);

HAMAL_API struct dep_val_str *
val_unit_to_str (struct val_unit *self, struct mem *mem);

HAMAL_API void
val_unit_free (struct val_unit *self);

HAMAL_API void
val_unit_free_safe (struct val_unit **self);

#endif //CORE_VAL_UNIT_H
