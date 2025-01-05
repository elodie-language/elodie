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
        switch (self->kind) {
            case VAL_KIND_BOOL:
                return val_bool_free_safe((struct val_bool **) &self);
            case VAL_KIND_F4:
                return val_f4_free_safe((struct val_f4 **) &self);
            case VAL_KIND_F8:
                return val_f8_free_safe((struct val_f8 **) &self);
            case VAL_KIND_I1:
                return val_i1_free_safe((struct val_i1 **) &self);
            case VAL_KIND_I2:
                return val_i2_free_safe((struct val_i2 **) &self);
            case VAL_KIND_I4:
                return val_i4_free_safe((struct val_i4 **) &self);
            case VAL_KIND_I8:
                return val_i8_free_safe((struct val_i8 **) &self);
            case VAL_KIND_I16:
                return val_i16_free_safe((struct val_i16 **) &self);
            case VAL_KIND_U1:
                return val_u1_free_safe((struct val_u1 **) &self);
            case VAL_KIND_U2:
                return val_u2_free_safe((struct val_u2 **) &self);
            case VAL_KIND_U4:
                return val_u4_free_safe((struct val_u4 **) &self);
            case VAL_KIND_U8:
                return val_u8_free_safe((struct val_u8 **) &self);
            case VAL_KIND_U16:
                return val_u16_free_safe((struct val_u16 **) &self);
            case VAL_KIND_NUM:
                return val_num_free_safe((struct val_num **) &self);
            case VAL_KIND_STR:
                return val_str_free_safe((struct val_str **) &self);
            default:
                NOT_IMPLEMENTED_YET();
        }
    }
}

void
val_rc_dec_safe(struct val **self) {
    NOT_IMPLEMENTED_YET()
}
