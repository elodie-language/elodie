#include <string.h>
#include "core/check.h"
#include "core/operator.h"
#include "core/val/val-bool.h"
#include "core/val/val-num-u2.h"
#include "core/val/val-str.h"

struct val_u2 *
val_u2_new(struct mem *mem, u2 val) {
    CHECK_NOT_NULL(mem);
    struct val_u2 *result = mem_allocate(mem, sizeof(struct val_u2));
    val_init(&result->base, VAL_KIND_U2, mem);
    result->data = val;
    return result;
}

struct val_u2 *
val_u2_copy(struct val_u2 *self, struct mem *mem) {
    CHECK_NOT_NULL(self);
    CHECK_NOT_NULL(mem);
    return val_u2_new(mem, self->data);
}

struct val_u2 *
val_u2_calc(struct mem *mem, struct val_u2 *lhs, enum CalculateOperator op, struct val_u2 *rhs) {
    CHECK_NOT_NULL(mem);
    CHECK_NOT_NULL(lhs);
    CHECK_NOT_NULL(rhs);
    switch (op) {
        case CALCULATE_OPERATOR_ADD_WRAP_AROUND:
            return val_u2_new(mem, lhs->data + rhs->data);
        case CALCULATE_OPERATOR_SUB_WRAP_AROUND:
            return val_u2_new(mem, lhs->data - rhs->data);
        case CALCULATE_OPERATOR_MUL_WRAP_AROUND:
            return val_u2_new(mem, lhs->data * rhs->data);
        case CALCULATE_OPERATOR_DIV_WRAP_AROUND:
            return val_u2_new(mem, lhs->data / rhs->data);
        case CALCULATE_OPERATOR_MOD_WRAP_AROUND:
            return val_u2_new(mem, lhs->data % rhs->data);
        default:
            NOT_IMPLEMENTED_YET();
    }
}

struct val_bool *
val_u2_cmp(struct mem *mem, struct val_u2 *lhs, enum CompareOperator op, struct val_u2 *rhs){
    CHECK_NOT_NULL(mem);
    CHECK_NOT_NULL(lhs);
    CHECK_NOT_NULL(rhs);
    switch (op) {
        case COMPARE_OPERATOR_EQUAL:
            return val_bool_new(mem, lhs->data == rhs->data);
        case COMPARE_OPERATOR_NOT_EQUAL:
            return val_bool_new(mem, lhs->data != rhs->data);
        case COMPARE_OPERATOR_GREATER_THAN:
            return val_bool_new(mem, lhs->data > rhs->data);
        case COMPARE_OPERATOR_GREATER_THAN_EQUAL:
            return val_bool_new(mem, lhs->data >= rhs->data);
        case COMPARE_OPERATOR_LESS_THAN:
            return val_bool_new(mem, lhs->data < rhs->data);
        case COMPARE_OPERATOR_LESS_THAN_EQUAL:
            return val_bool_new(mem, lhs->data <= rhs->data);
        default:
            NOT_IMPLEMENTED_YET();
    }
}

struct val_str *
val_u2_to_str(struct val_u2 *self, struct mem *mem) {
    CHECK_NOT_NULL(self);
    CHECK_NOT_NULL(mem);
    char output[50] = {0};
    snprintf(output, 50, "%hu", self->data);
    return val_str_new_from_bytes(mem, (struct bytes_view) {
            .data = (u2 *) output,
            .size = strlen(output)
    });
}

void
val_u2_free(struct val_u2 *self) {
    CHECK_NOT_NULL(self);
    mem_deallocate(self->base.mem, self);
}

void
val_u2_free_safe(struct val_u2 **self) {
    CHECK_NOT_NULL(self);
    val_u2_free(*self);
    *self = NULL;
}
