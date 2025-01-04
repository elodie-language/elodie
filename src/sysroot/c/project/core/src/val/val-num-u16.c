#include <string.h>
#include "core/check.h"
#include "core/val/val-num-u16.h"
#include "core/val/val-str.h"

struct val_u16 *
val_u16_new(struct mem *mem, u16 val) {
    CHECK_NOT_NULL(mem);
    struct val_u16 *result = mem_allocate(mem, sizeof(struct val_u16));
    val_init(&result->base, VAL_KIND_U16, mem);
    result->data = val;
    return result;
}

struct val_u16 *
val_u16_copy(struct val_u16 *self, struct mem *mem) {
    CHECK_NOT_NULL(self);
    CHECK_NOT_NULL(mem);
    return val_u16_new(mem, self->data);
}

bool
val_u16_equal(struct val_u16 *lhs, struct val_u16 *rhs) {
    CHECK_NOT_NULL(lhs);
    CHECK_NOT_NULL(rhs);
    if (lhs == rhs) return true;
    return lhs->data == rhs->data;
}

struct val_str *
val_u16_to_str(struct val_u16 *self, struct mem *mem) {
    CHECK_NOT_NULL(self);
    CHECK_NOT_NULL(mem);
    char output[50] = {0};
    snprintf(output, 50, "%llu", self->data);
    return val_str_new_from_bytes(mem, (struct bytes_view) {
            .data = (u16 *) output,
            .size = strlen(output)
    });
}

void
val_u16_free(struct val_u16 *self) {
    CHECK_NOT_NULL(self);
    mem_deallocate(self->base.mem, self);
}

void
val_u16_free_safe(struct val_u16 **self) {
    CHECK_NOT_NULL(self);
    val_u16_free(*self);
    *self = NULL;
}
