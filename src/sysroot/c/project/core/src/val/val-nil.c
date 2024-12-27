#include "core/check.h"
#include "core/val/val-str.h"
#include "core/val/val-nil.h"

struct val_nil *
val_nil_new (struct mem *mem)
{
	CHECK_NOT_NULL(mem);

	struct val_nil *result = mem_allocate (mem, sizeof (struct val_nil));
	dep_val_init (&result->base, VAL_KIND_NIL, mem);
	return result;
}

bool
val_nil_equal (struct val_nil *lhs, struct val_nil *rhs)
{
	CHECK_NOT_NULL(lhs);
	CHECK_NOT_NULL(rhs);
	return true;
}

struct dep_val_str *
val_nil_to_str (struct val_nil *self, struct mem *mem)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(mem);
	return dep_val_str_allocate_from_c_str (mem, "nil");
}

void
val_nil_free (struct val_nil *self)
{
	CHECK_NOT_NULL(self);
	mem_deallocate (self->base.mem, self);
}

void
val_nil_free_safe (struct val_nil **self)
{
	CHECK_NOT_NULL(self);
	val_nil_free (*self);
	*self = NULL;
}
