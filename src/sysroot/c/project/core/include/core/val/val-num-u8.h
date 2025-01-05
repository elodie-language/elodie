#ifndef CORE_VAL_NUM_U8_H
#define CORE_VAL_NUM_U8_H

#include "core/core.h"
#include "val.h"

struct val_u8 {
    struct val base;
    u8 data;
};

ELODIE_API struct val_u8 *
val_u8_new(struct mem *mem, u8 val);

ELODIE_API struct val_u8 *
val_u8_copy(struct val_u8 *self, struct mem *mem);

ELODIE_API struct val_u8 *
val_u8_calc(struct mem *mem, struct val_u8 *lhs, enum CalculateOperator op, struct val_u8 *rhs);

ELODIE_API struct val_bool *
val_u8_cmp(struct mem *mem, struct val_u8 *lhs, enum CompareOperator op, struct val_u8 *rhs);

ELODIE_API struct val_str *
val_u8_to_str(struct val_u8 *self, struct mem *mem);

ELODIE_API void
val_u8_free(struct val_u8 *self);

ELODIE_API void
val_u8_free_safe(struct val_u8 **self);

#endif //CORE_VAL_NUM_U8_H
