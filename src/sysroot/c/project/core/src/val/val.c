#include "core/check.h"
#include "core/val/val.h"

void
val_init (struct val *self, enum val_kind kind, struct mem *mem)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(mem);
	self->kind = kind;
	self->mem = mem;
}
