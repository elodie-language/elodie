#include <string.h>
#include "core/check.h"
#include "core/val/val-num.h"
#include "core/val/val-str.h"

struct val_num *
val_num_new_from_double(struct mem *mem, double val) {
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
    return val_num_new_from_double(mem, self->data);
}

bool
val_num_equal(struct val_num *lhs, struct val_num *rhs) {
    CHECK_NOT_NULL(lhs);
    CHECK_NOT_NULL(rhs);
    if (lhs == rhs) return true;
    return lhs->data == rhs->data;
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
