
#include "core/operator.h"
#include "core/check.h"
#include "core/val/val-bool.h"
#include "core/val/val-str.h"

struct val_bool *
val_bool_new(struct mem *mem, bool data) {
    CHECK_NOT_NULL(mem);
    struct val_bool *result = mem_allocate(mem, sizeof(struct val_bool));
    val_init(&result->base, VAL_KIND_BOOL, mem);
    result->data = data;
    return result;
}

struct val_bool *
val_bool_copy(struct val_bool *self, struct mem *mem) {
    CHECK_NOT_NULL(self);
    CHECK_NOT_NULL(mem);
    return val_bool_new(mem, self->data);
}

bool
val_bool_equal(struct val_bool *lhs, struct val_bool *rhs) {
    CHECK_NOT_NULL(lhs);
    CHECK_NOT_NULL(rhs);
    if (lhs == rhs) return true;
    return lhs->data == rhs->data;
}

bool
val_bool_cmp(struct val_bool *lhs, enum CompareOperator op, struct val_bool *rhs) {
    CHECK_NOT_NULL(lhs);
    CHECK_NOT_NULL(rhs);
    switch (op) {
        case COMPARE_OPERATOR_EQUAL:
            return lhs->data == rhs->data;
        case COMPARE_OPERATOR_NOT_EQUAL:
            return lhs->data != rhs->data;
        default:
            // FIXME panic
            return false;
    }
}

bool
val_bool_cmp_lit(struct val_bool *lhs, enum CompareOperator op, bool rhs) {
    CHECK_NOT_NULL(lhs);
    switch (op) {
        case COMPARE_OPERATOR_EQUAL:
            return lhs->data == rhs;
        case COMPARE_OPERATOR_NOT_EQUAL:
            return lhs->data != rhs;
        default:
            // FIXME panic
            return false;
    }
}

struct val_str *
val_bool_to_str(struct val_bool *self, struct mem *mem) {
    CHECK_NOT_NULL(self);
    CHECK_NOT_NULL(mem);
    if (self->data) {
        return val_str_new_from_c_str(mem, "true");
    }
    return val_str_new_from_c_str(mem, "false");
}

void
val_bool_free(struct val_bool *self) {
    CHECK_NOT_NULL(self);
    mem_deallocate(self->base.mem, self);
}

void
val_bool_free_safe(struct val_bool **self) {
    CHECK_NOT_NULL(self);
    val_bool_free(*self);
    *self = NULL;
}
