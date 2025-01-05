#ifndef CORE_VAL_NUM_I8_H
#define CORE_VAL_NUM_I8_H

#include "core/core.h"
#include "val.h"

struct val_i8 {
    struct val base;
    i8 data;
};

ELODIE_API struct val_i8 *
val_i8_new(struct mem *mem, i8 val);

ELODIE_API struct val_i8 *
val_i8_copy(struct val_i8 *self, struct mem *mem);

ELODIE_API struct val_bool *
val_i8_cmp(struct mem *mem, struct val_i8 *lhs, enum CompareOperator op, struct val_i8 *rhs);

ELODIE_API struct val_str *
val_i8_to_str(struct val_i8 *self, struct mem *mem);

ELODIE_API void
val_i8_free(struct val_i8 *self);

ELODIE_API void
val_i8_free_safe(struct val_i8 **self);

#endif //CORE_VAL_NUM_I8_H
