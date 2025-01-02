#include "core/check.h"
#include "core/val/val-api.h"

typedef struct val_prop p;
typedef struct val_fld f;
typedef struct val_obj o;

p *
val_prop_new(struct mem *mem, u2 id, f *field, o *of) {
    CHECK_NOT_NULL(mem);
    CHECK_NOT_NULL(field);

    p *result = mem_allocate(mem, sizeof(p));
    val_init(&result->base, VAL_KIND_PROP, mem);
    result->id = id;
    result->field = field;
    result->of = of;
    return result;
}

void
val_prop_free(p *self) {
    CHECK_NOT_NULL(self);
    mem_deallocate(self->base.mem, self);
}

void
val_prop_free_safe(p **self) {
    CHECK_NOT_NULL(self);
    val_prop_free(*self);
    *self = NULL;
}
