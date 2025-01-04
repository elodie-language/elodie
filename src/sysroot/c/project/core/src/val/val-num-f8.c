#include <string.h>
#include "core/check.h"
#include "core/val/val-num-f8.h"
#include "core/val/val-str.h"

struct val_f8 *
val_f8_new(struct mem *mem, f8 val) {
    CHECK_NOT_NULL(mem);
    struct val_f8 *result = mem_allocate(mem, sizeof(struct val_f8));
    val_init(&result->base, VAL_KIND_F8, mem);
    result->data = val;
    return result;
}

struct val_f8 *
val_f8_copy(struct val_f8 *self, struct mem *mem) {
    CHECK_NOT_NULL(self);
    CHECK_NOT_NULL(mem);
    return val_f8_new(mem, self->data);
}

bool
val_f8_equal(struct val_f8 *lhs, struct val_f8 *rhs) {
    CHECK_NOT_NULL(lhs);
    CHECK_NOT_NULL(rhs);
    if (lhs == rhs) return true;
    return lhs->data == rhs->data;
}

struct val_str *
val_f8_to_str(struct val_f8 *self, struct mem *mem) {
    CHECK_NOT_NULL(self);
    CHECK_NOT_NULL(mem);
    char output[50] = {0};
    snprintf(output, 50, "%f", self->data);
    return val_str_new_from_bytes(mem, (struct bytes_view) {
            .data = (u1 *) output,
            .size = strlen(output)
    });
}

void
val_f8_free(struct val_f8 *self) {
    CHECK_NOT_NULL(self);
    mem_deallocate(self->base.mem, self);
}

void
val_f8_free_safe(struct val_f8 **self) {
    CHECK_NOT_NULL(self);
    val_f8_free(*self);
    *self = NULL;
}
