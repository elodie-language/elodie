#include "core/check.h"
#include "core/val/val.h"
#include "core/val/val-api.h"

void
val_init(struct val *self, enum val_kind kind, struct mem *mem) {
    CHECK_NOT_NULL(self);
    CHECK_NOT_NULL(mem);
    self->kind = kind;
    self->mem = mem;
    val_rc_inc(self);
}

void
val_rc_inc(struct val *self) {
    CHECK_NOT_NULL(self);
    CHECK_LESS_THAN(self->rc + 1, U8_MAX);
    self->rc += 1;
    LOG_TRACE("RC %p: %d", self, self->rc);
}

void
val_rc_dec(struct val *self) {
    CHECK_NOT_NULL(self);
    CHECK_GREATER_THAN_EQUAL(self->rc, 1);
    self->rc -= 1;
    LOG_TRACE("RC %p: %d", self, self->rc);
    if (self->rc == 0) {
        if (self->kind == VAL_KIND_NUM) {
            val_num_free_safe((struct val_num **) &self);
        } else {
            val_str_free_safe((struct val_str **) &self);
        }
    }
}

void
val_rc_dec_safe(struct val **self) {
    NOT_IMPLEMENTED_YET()
}
