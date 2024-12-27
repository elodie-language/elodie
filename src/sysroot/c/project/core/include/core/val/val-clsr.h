#ifndef CORE_VAL_CLSR_H
#define CORE_VAL_CLSR_H

#include "val.h"

struct val_fn;
struct dep_val_str;
struct package_function;

struct val_clsr {
  struct dep_val base;
  struct val_fn *fn; // FIXME to be replaced by fn_proto
  struct package_function *package_fn;
};

HAMAL_API struct val_clsr *
val_clsr_new (struct mem *mem, struct val_fn *fn);

HAMAL_API struct dep_val_str *
val_clsr_to_str (struct val_clsr *self, struct mem *mem);

HAMAL_API void
val_clsr_clear (struct val_clsr *self);

HAMAL_API void
val_clsr_free (struct val_clsr *self);

HAMAL_API void
val_clsr_free_safe (struct val_clsr **self);

#endif //CORE_VAL_CLSR_H
