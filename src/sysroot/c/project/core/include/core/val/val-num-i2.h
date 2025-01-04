#ifndef CORE_VAL_NUM_I2_H
#define CORE_VAL_NUM_I2_H

#include "core/core.h"
#include "val.h"

struct val_i2 {
    struct val base;
    i2 data;
};

ELODIE_API struct val_i2 *
val_i2_new(struct mem *mem, i2 val);

ELODIE_API struct val_i2 *
val_i2_copy(struct val_i2 *self, struct mem *mem);

ELODIE_API struct val_str *
val_i2_to_str(struct val_i2 *self, struct mem *mem);

ELODIE_API void
val_i2_free(struct val_i2 *self);

ELODIE_API void
val_i2_free_safe(struct val_i2 **self);

#endif //CORE_VAL_NUM_I2_H
