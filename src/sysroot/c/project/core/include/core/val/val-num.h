#ifndef CORE_VAL_NUM_H
#define CORE_VAL_NUM_H

#include "val.h"
#include "core/operator.h"

struct val_num {
    struct val base;
    double data; // FIXME turn into big decimal
};

ELODIE_API struct val_num *
val_num_new(struct mem *mem, double val);

ELODIE_API struct val_num *
val_num_copy(struct val_num *self, struct mem *mem);

ELODIE_API struct val_num *
val_num_calc(struct mem *mem, struct val_num *lhs, enum CalculateOperator op, struct val_num *rhs);

ELODIE_API struct val_bool *
val_num_cmp(struct mem *mem, struct val_num *lhs, enum CompareOperator op, struct val_num *rhs);

ELODIE_API struct val_str *
val_num_to_str(struct val_num *self, struct mem *mem);

ELODIE_API void
val_num_free(struct val_num *self);

ELODIE_API void
val_num_free_safe(struct val_num **self);

#endif //CORE_VAL_NUM_H
