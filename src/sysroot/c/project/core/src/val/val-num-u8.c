#include <string.h>
#include "core/check.h"
#include "core/val/val-num-u8.h"
#include "core/val/val-str.h"

struct val_u8 *
val_u8_new(struct mem *mem, u8 val) {
    CHECK_NOT_NULL(mem);
    struct val_u8 *result = mem_allocate(mem, sizeof(struct val_u8));
    val_init(&result->base, VAL_KIND_U8, mem);
    result->data = val;
    return result;
}

struct val_u8 *
val_u8_copy(struct val_u8 *self, struct mem *mem) {
    CHECK_NOT_NULL(self);
    CHECK_NOT_NULL(mem);
    return val_u8_new(mem, self->data);
}

bool
val_u8_equal(struct val_u8 *lhs, struct val_u8 *rhs) {
    CHECK_NOT_NULL(lhs);
    CHECK_NOT_NULL(rhs);
    if (lhs == rhs) return true;
    return lhs->data == rhs->data;
}

struct val_str *
val_u8_to_str(struct val_u8 *self, struct mem *mem) {
    CHECK_NOT_NULL(self);
    CHECK_NOT_NULL(mem);
    char output[50] = {0};
    snprintf(output, 50, "%lu", self->data);
    return val_str_new_from_bytes(mem, (struct bytes_view) {
            .data = (u8 *) output,
            .size = strlen(output)
    });
}

void
val_u8_free(struct val_u8 *self) {
    CHECK_NOT_NULL(self);
    mem_deallocate(self->base.mem, self);
}

void
val_u8_free_safe(struct val_u8 **self) {
    CHECK_NOT_NULL(self);
    val_u8_free(*self);
    *self = NULL;
}
