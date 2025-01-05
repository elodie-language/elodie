#ifndef CORE_VAL_NUM_U16_H
#define CORE_VAL_NUM_U16_H

#include "core/core.h"
#include "val.h"

struct val_u16 {
    struct val base;
    u16 data;
};

ELODIE_API struct val_u16 *
val_u16_new(struct mem *mem, u16 val);

ELODIE_API struct val_u16 *
val_u16_copy(struct val_u16 *self, struct mem *mem);

ELODIE_API struct val_u16 *
val_u16_calc(struct mem *mem, struct val_u16 *lhs, enum CalculateOperator op, struct val_u16 *rhs);

ELODIE_API struct val_bool *
val_u16_cmp(struct mem *mem, struct val_u16 *lhs, enum CompareOperator op, struct val_u16 *rhs);

ELODIE_API struct val_str *
val_u16_to_str(struct val_u16 *self, struct mem *mem);

ELODIE_API void
val_u16_free(struct val_u16 *self);

ELODIE_API void
val_u16_free_safe(struct val_u16 **self);

#endif //CORE_VAL_NUM_U16_H
