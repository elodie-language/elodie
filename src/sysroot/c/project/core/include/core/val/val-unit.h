#ifndef CORE_VAL_UNIT_H
#define CORE_VAL_UNIT_H

#include "val.h"

struct val_unit {
  struct val base;
};

ELODIE_API struct val_unit *
val_unit_new (struct mem *mem);

ELODIE_API bool
val_unit_equal (struct val_unit *lhs, struct val_unit *rhs);

ELODIE_API struct val_str *
val_unit_to_str (struct val_unit *self, struct mem *mem);

ELODIE_API void
val_unit_free (struct val_unit *self);

ELODIE_API void
val_unit_free_safe (struct val_unit **self);

#endif //CORE_VAL_UNIT_H
