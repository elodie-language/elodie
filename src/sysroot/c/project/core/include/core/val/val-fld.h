#ifndef CORE_val_fld_H
#define CORE_val_fld_H

#include "val.h"
#include "val-str.h"

#include "core/type/type.h"

struct val_fld {
  struct dep_val base;
  struct dep_val_str *ident;
  struct type type;
};

HAMAL_API struct val_fld *
val_fld_new (struct mem *mem, struct dep_val_str_view ident, struct type type);

HAMAL_API void
val_fld_free (struct val_fld *self);

HAMAL_API void
val_fld_free_safe (struct val_fld **self);

#endif //CORE_val_fld_H
