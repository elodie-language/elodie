#include "core/check.h"
#include "core/val/val-clsr.h"
#include "core/val/val-fn.h"

struct val_clsr *
val_clsr_new (struct mem *mem, struct val_fn *fn)
{
	CHECK_NOT_NULL(mem);
	CHECK_NOT_NULL(fn);

	struct val_clsr *result = mem_allocate (mem, sizeof (struct val_clsr));
	dep_val_init (&result->base, VAL_KIND_CLSR, mem);
	result->fn = fn;
	return result;
}

struct dep_val_str *
val_clsr_to_str (struct val_clsr *self, struct mem *mem)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(mem);
	return val_fn_to_str (self->fn, mem);
}

void
val_clsr_clear (struct val_clsr *self)
{
	CHECK_NOT_NULL(self);
	val_fn_free_safe (&self->fn);
}

void
val_clsr_free (struct val_clsr *self)
{
	CHECK_NOT_NULL(self);
	mem_deallocate (self->base.mem, self);
}

void
val_clsr_free_safe (struct val_clsr **self)
{
	CHECK_NOT_NULL(self);
	val_clsr_free (*self);
	*self = NULL;
}
