#ifndef CORE_VAL_PROP_H
#define CORE_VAL_PROP_H

#include "val.h"

struct val_fld;
struct val_obj;

struct val_prop {
  struct dep_val base;
  u2 id;
  struct val_fld *field;
  struct val_obj *of;
};

HAMAL_API struct val_prop *
val_prop_new (struct mem *mem, u2 id, struct val_fld *field, struct val_obj *of);

HAMAL_API void
val_prop_free (struct val_prop *self);

HAMAL_API void
val_prop_free_safe (struct val_prop **self);

#endif //CORE_VAL_PROP_H
