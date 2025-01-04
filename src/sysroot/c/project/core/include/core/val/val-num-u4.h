#ifndef CORE_VAL_NUM_U4_H
#define CORE_VAL_NUM_U4_H

#include "core/core.h"
#include "val.h"

struct val_u4 {
    struct val base;
    u4 data;
};

ELODIE_API struct val_u4 *
val_u4_new(struct mem *mem, u4 val);

ELODIE_API struct val_u4 *
val_u4_copy(struct val_u4 *self, struct mem *mem);

ELODIE_API struct val_str *
val_u4_to_str(struct val_u4 *self, struct mem *mem);

ELODIE_API void
val_u4_free(struct val_u4 *self);

ELODIE_API void
val_u4_free_safe(struct val_u4 **self);

#endif //CORE_VAL_NUM_U4_H
