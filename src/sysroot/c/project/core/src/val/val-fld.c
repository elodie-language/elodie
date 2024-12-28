#include "core/check.h"
#include "core/val/val-fld.h"
#include "core/type/type-api.h"

typedef struct val_fld f;

f *
val_fld_new (struct mem *mem, struct val_str_view ident, struct type type)
{
	CHECK_NOT_NULL(mem);

	f *result = mem_allocate (mem, sizeof (f));
	val_init (&result->base, VAL_KIND_FLD, mem);
	result->ident = val_str_allocate_from_view (mem, ident);
	result->type = type;
	return result;
}

void
val_fld_free (f *self)
{
	CHECK_NOT_NULL(self);
	val_str_deallocate_safe (&self->ident);
	mem_deallocate (self->base.mem, self);
}

void
val_fld_free_safe (f **self)
{
	CHECK_NOT_NULL(self);
	val_fld_free (*self);
	*self = NULL;
}
