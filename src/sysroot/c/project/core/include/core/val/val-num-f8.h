#ifndef CORE_VAL_NUM_F8_H
#define CORE_VAL_NUM_F8_H

#include "core/core.h"
#include "val.h"

struct val_f8 {
    struct val base;
    f8 data;
};

ELODIE_API struct val_f8 *
val_f8_new(struct mem *mem, f8 val);

ELODIE_API struct val_f8 *
val_f8_copy(struct val_f8 *self, struct mem *mem);

ELODIE_API struct val_bool *
val_f8_cmp(struct mem *mem, struct val_f8 *lhs, enum CompareOperator op, struct val_f8 *rhs);

ELODIE_API struct val_str *
val_f8_to_str(struct val_f8 *self, struct mem *mem);

ELODIE_API void
val_f8_free(struct val_f8 *self);

ELODIE_API void
val_f8_free_safe(struct val_f8 **self);

#endif //CORE_VAL_NUM_F8_H
