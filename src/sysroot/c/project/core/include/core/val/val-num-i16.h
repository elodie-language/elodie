#ifndef CORE_VAL_NUM_I16_H
#define CORE_VAL_NUM_I16_H

#include "core/core.h"
#include "val.h"

struct val_i16 {
    struct val base;
    i16 data;
};

ELODIE_API struct val_i16 *
val_i16_new(struct mem *mem, i16 val);

ELODIE_API struct val_i16 *
val_i16_copy(struct val_i16 *self, struct mem *mem);

ELODIE_API struct val_i16 *
val_i16_calc(struct mem *mem, struct val_i16 *lhs, enum CalculateOperator op, struct val_i16 *rhs);

ELODIE_API struct val_bool *
val_i16_cmp(struct mem *mem, struct val_i16 *lhs, enum CompareOperator op, struct val_i16 *rhs);

ELODIE_API struct val_str *
val_i16_to_str(struct val_i16 *self, struct mem *mem);

ELODIE_API void
val_i16_free(struct val_i16 *self);

ELODIE_API void
val_i16_free_safe(struct val_i16 **self);

#endif //CORE_VAL_NUM_I16_H
