#ifndef CORE_VAL_NUM_F4_H
#define CORE_VAL_NUM_F4_H

#include "core/core.h"
#include "val.h"

struct val_f4 {
    struct val base;
    f4 data;
};

ELODIE_API struct val_f4 *
val_f4_new(struct mem *mem, f4 val);

ELODIE_API struct val_f4 *
val_f4_copy(struct val_f4 *self, struct mem *mem);

ELODIE_API struct val_f4 *
val_f4_calc(struct mem *mem, struct val_f4 *lhs, enum CalculateOperator op, struct val_f4 *rhs);

ELODIE_API struct val_bool *
val_f4_cmp(struct mem *mem, struct val_f4 *lhs, enum CompareOperator op, struct val_f4 *rhs);

ELODIE_API struct val_str *
val_f4_to_str(struct val_f4 *self, struct mem *mem);

ELODIE_API void
val_f4_free(struct val_f4 *self);

ELODIE_API void
val_f4_free_safe(struct val_f4 **self);

#endif //CORE_VAL_NUM_F4_H
