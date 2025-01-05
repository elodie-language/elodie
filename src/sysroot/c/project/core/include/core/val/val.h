#ifndef CORE_VAL_H
#define CORE_VAL_H

#include "core/mem/mem-api.h"
#include "val-kind.h"

struct val_str;

struct val {
    enum val_kind kind;
    struct mem *mem;
    u8 rc;
};

ELODIE_API void
val_init(struct val *self, enum val_kind kind, struct mem *mem);

ELODIE_API struct val_str *
val_to_str(struct val *self, struct mem *mem);

ELODIE_API void
val_rc_inc(struct val *self);

ELODIE_API void
val_rc_dec(struct val *self);

ELODIE_API void
val_rc_dec_safe (struct val **self);

#endif //CORE_VAL_H
