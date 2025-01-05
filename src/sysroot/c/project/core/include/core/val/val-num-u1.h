#ifndef CORE_VAL_NUM_U1_H
#define CORE_VAL_NUM_U1_H

#include "core/core.h"
#include "val.h"

struct val_u1 {
    struct val base;
    u1 data;
};

ELODIE_API struct val_u1 *
val_u1_new(struct mem *mem, u1 val);

ELODIE_API struct val_u1 *
val_u1_copy(struct val_u1 *self, struct mem *mem);

ELODIE_API struct val_bool *
val_u1_cmp(struct mem *mem, struct val_u1 *lhs, enum CompareOperator op, struct val_u1 *rhs);

ELODIE_API struct val_str *
val_u1_to_str(struct val_u1 *self, struct mem *mem);

ELODIE_API void
val_u1_free(struct val_u1 *self);

ELODIE_API void
val_u1_free_safe(struct val_u1 **self);

#endif //CORE_VAL_NUM_U1_H
