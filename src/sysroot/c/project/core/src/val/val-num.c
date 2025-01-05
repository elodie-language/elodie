#include <string.h>
#include "core/check.h"
#include "core/val/val-bool.h"
#include "core/val/val-num.h"
#include "core/val/val-str.h"

struct val_num *
val_num_new(struct mem *mem, double val) {
    CHECK_NOT_NULL(mem);
    struct val_num *result = mem_allocate(mem, sizeof(struct val_num));
    val_init(&result->base, VAL_KIND_NUM, mem);
    result->data = val;
    return result;
}

struct val_num *
val_num_copy(struct val_num *self, struct mem *mem) {
    CHECK_NOT_NULL(self);
    CHECK_NOT_NULL(mem);
    return val_num_new(mem, self->data);
}

struct val_num *
val_num_calc(struct mem *mem, struct val_num *lhs, enum CalculateOperator op, struct val_num *rhs) {
    CHECK_NOT_NULL(mem);
    CHECK_NOT_NULL(lhs);
    CHECK_NOT_NULL(rhs);
    switch (op) {
        case CALCULATE_OPERATOR_ADD_WRAP_AROUND:
            return val_num_new(mem, lhs->data + rhs->data);
        case CALCULATE_OPERATOR_SUB_WRAP_AROUND:
            return val_num_new(mem, lhs->data - rhs->data);
        case CALCULATE_OPERATOR_MULTIPLY_WRAP_AROUND:
            return val_num_new(mem, lhs->data * rhs->data);
        case CALCULATE_OPERATOR_DIV_WRAP_AROUND:
            return val_num_new(mem, lhs->data / rhs->data);
        default:
            NOT_IMPLEMENTED_YET();
    }
}

struct val_bool *
val_num_cmp(struct mem *mem, struct val_num *lhs, enum CompareOperator op, struct val_num *rhs) {
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
val_num_to_str(struct val_num *self, struct mem *mem) {
    CHECK_NOT_NULL(self);
    CHECK_NOT_NULL(mem);
    char output[50] = {0};
    snprintf(output, 50, "%g", self->data);
    return val_str_new_from_bytes(mem, (struct bytes_view) {
            .data = (u1 *) output,
            .size = strlen(output)
    });
}

void
val_num_free(struct val_num *self) {
    CHECK_NOT_NULL(self);
    mem_deallocate(self->base.mem, self);
}

void
val_num_free_safe(struct val_num **self) {
    CHECK_NOT_NULL(self);
    val_num_free(*self);
    *self = NULL;
}
