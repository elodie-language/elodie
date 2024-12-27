#include "core/check.h"
#include "core/val/val.h"

void
val_init (struct val *self, enum val_kind kind, struct mem *mem)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(mem);
	self->kind = kind;
	self->mem_realm = mem->realm;
}

void
dep_val_init (struct dep_val *self, enum val_kind kind, struct mem *mem)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(mem);
	self->kind = kind;
	self->mem = mem;
}
