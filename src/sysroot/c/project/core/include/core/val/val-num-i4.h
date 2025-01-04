#ifndef CORE_VAL_NUM_I4_H
#define CORE_VAL_NUM_I4_H

#include "core/core.h"
#include "val.h"

struct val_i4 {
    struct val base;
    i4 data;
};

ELODIE_API struct val_i4 *
val_i4_new(struct mem *mem, i4 val);

ELODIE_API struct val_i4 *
val_i4_copy(struct val_i4 *self, struct mem *mem);

ELODIE_API struct val_str *
val_i4_to_str(struct val_i4 *self, struct mem *mem);

ELODIE_API void
val_i4_free(struct val_i4 *self);

ELODIE_API void
val_i4_free_safe(struct val_i4 **self);

#endif //CORE_VAL_NUM_I4_H
