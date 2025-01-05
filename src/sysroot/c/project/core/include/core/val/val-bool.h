#ifndef CORE_VAL_BOOL_H
#define CORE_VAL_BOOL_H

#include "val.h"
#include "core/operator.h"

struct val_bool {
    struct val base;
    bool data;
};

ELODIE_API struct val_bool *
val_bool_new(struct mem *mem, bool data);

ELODIE_API struct val_bool *
val_bool_copy(struct val_bool *self, struct mem *mem);

ELODIE_API bool
val_bool_equal(struct val_bool *lhs, struct val_bool *rhs);

ELODIE_API bool
val_bool_cmp(struct val_bool *lhs, enum CompareOperator op, struct val_bool *rhs);

ELODIE_API bool
val_bool_cmp_lit(struct val_bool *lhs, enum CompareOperator op, bool rhs);

ELODIE_API struct val_str *
val_bool_to_str(struct val_bool *self, struct mem *mem);

ELODIE_API void
val_bool_free(struct val_bool *self);

ELODIE_API void
val_bool_free_safe(struct val_bool **self);

#endif //CORE_VAL_BOOL_H
