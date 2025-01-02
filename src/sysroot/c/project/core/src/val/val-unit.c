#include "core/check.h"
#include "core/val/val-str.h"
#include "core/val/val-unit.h"

struct val_unit *
val_unit_new(struct mem *mem) {
    CHECK_NOT_NULL(mem);

    struct val_unit *result = mem_allocate(mem, sizeof(struct val_unit));
    val_init(&result->base, VAL_KIND_UNIT, mem);
    return result;
}

bool
val_unit_equal(struct val_unit *lhs, struct val_unit *rhs) {
    CHECK_NOT_NULL(lhs);
    CHECK_NOT_NULL(rhs);
    return true;
}

struct val_str *
val_unit_to_str(struct val_unit *self, struct mem *mem) {
    CHECK_NOT_NULL(self);
    CHECK_NOT_NULL(mem);
    return val_str_new_from_c_str(mem, "unit");
}

void
val_unit_free(struct val_unit *self) {
    CHECK_NOT_NULL(self);
    mem_deallocate(self->base.mem, self);
}

void
val_unit_free_safe(struct val_unit **self) {
    CHECK_NOT_NULL(self);
    val_unit_free(*self);
    *self = NULL;
}
