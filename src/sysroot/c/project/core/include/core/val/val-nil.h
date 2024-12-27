#ifndef CORE_VAL_NIL_H
#define CORE_VAL_NIL_H

#include "val.h"

struct val_nil {
  struct dep_val base;
};

HAMAL_API struct val_nil *
val_nil_new (struct mem *mem);

HAMAL_API bool
val_nil_equal (struct val_nil *lhs, struct val_nil *rhs);

HAMAL_API struct dep_val_str *
val_nil_to_str (struct val_nil *self, struct mem *mem);

HAMAL_API void
val_nil_free (struct val_nil *self);

HAMAL_API void
val_nil_free_safe (struct val_nil **self);

#endif //CORE_VAL_NIL_H
