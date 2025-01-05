#ifndef CORE_VAL_NUM_U2_H
#define CORE_VAL_NUM_U2_H

#include "core/core.h"
#include "val.h"

struct val_u2 {
    struct val base;
    u2 data;
};

ELODIE_API struct val_u2 *
val_u2_new(struct mem *mem, u2 val);

ELODIE_API struct val_u2 *
val_u2_copy(struct val_u2 *self, struct mem *mem);

ELODIE_API struct val_bool *
val_u2_cmp(struct mem *mem, struct val_u2 *lhs, enum CompareOperator op, struct val_u2 *rhs);

ELODIE_API struct val_str *
val_u2_to_str(struct val_u2 *self, struct mem *mem);

ELODIE_API void
val_u2_free(struct val_u2 *self);

ELODIE_API void
val_u2_free_safe(struct val_u2 **self);

#endif //CORE_VAL_NUM_U2_H
