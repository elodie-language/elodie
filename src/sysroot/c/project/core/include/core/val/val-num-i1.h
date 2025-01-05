#ifndef CORE_VAL_NUM_I1_H
#define CORE_VAL_NUM_I1_H

#include "core/core.h"
#include "core/operator.h"
#include "val.h"

struct val_i1 {
    struct val base;
    i1 data;
};

ELODIE_API struct val_i1 *
val_i1_new(struct mem *mem, i1 val);

ELODIE_API struct val_i1 *
val_i1_copy(struct val_i1 *self, struct mem *mem);

ELODIE_API struct val_bool *
val_i1_cmp(struct mem *mem, struct val_i1 *lhs, enum CompareOperator op, struct val_i1 *rhs);

ELODIE_API struct val_str *
val_i1_to_str(struct val_i1 *self, struct mem *mem);

ELODIE_API void
val_i1_free(struct val_i1 *self);

ELODIE_API void
val_i1_free_safe(struct val_i1 **self);

#endif //CORE_VAL_NUM_I1_H
