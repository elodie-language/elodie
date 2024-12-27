#include "core/check.h"
#include "core/native/native-registry.h"

typedef struct mem m;
typedef struct native_registry nr;

void
native_registry_init (nr *self, m *mem)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(mem);
}

bool
native_registry_register_fn (struct native_registry *self, struct native_fn native_fn)
{
	self->fbs[0] = native_fn;
	return true;
}

bool
native_registry_resolve_fn (struct native_registry *self, struct native_fn_signature signature, struct native_fn *out)
{
	*out = self->fbs[0];
	return true;
}

void
native_registry_reset (struct native_registry *self, struct mem *mem)
{
	for (size_t idx = 0; idx < 1; idx++)
		{
			native_fn_signature_reset (&self->fbs[0].sig, mem);
		}

}
