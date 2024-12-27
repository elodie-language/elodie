#ifndef CORE_VAL_BOOL_H
#define CORE_VAL_BOOL_H

#include "val.h"

struct val_bool {
  struct dep_val base;
  bool data;
};

HAMAL_API struct val_bool *
val_bool_new_from_bool (struct mem *mem, bool data);

HAMAL_API struct val_bool *
val_bool_copy (struct val_bool *self, struct mem *mem);

HAMAL_API bool
val_bool_equal (struct val_bool *lhs, struct val_bool *rhs);

HAMAL_API struct dep_val_str *
val_bool_to_str (struct val_bool *self, struct mem *mem);

HAMAL_API void
val_bool_free (struct val_bool *self);

HAMAL_API void
val_bool_free_safe (struct val_bool **self);

#endif //CORE_VAL_BOOL_H
