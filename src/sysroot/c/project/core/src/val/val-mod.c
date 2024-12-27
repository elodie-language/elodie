#include "core/check.h"
#include "core/val/val-api.h"

struct val_mod *
val_mod_new (struct mem *mem, struct dep_val_str_view ident, struct val_clsr *init)
{
	CHECK_NOT_NULL(mem);

	struct val_mod *result = mem_allocate (mem, sizeof (struct val_mod));
	dep_val_init (&result->base, VAL_KIND_MOD, mem);
	result->ident = dep_val_str_allocate_from_view (mem, ident);
	result->initialized = false;
	result->init = init;

	val_mod_const_pool_init (&result->const_pool, mem);
	result->clsrs = val_lst_new (mem);

	return result;
}

bool
val_mod_register_clsr (struct val_mod *self, struct val_clsr *clsr)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(clsr);

	u2 result;
	if (val_mod_resolve_clsr_id (self, dep_val_str_view_from_str (clsr->fn->ident), &result))
		{
			return false;
		}

	val_lst_append_clsr (self->clsrs, clsr);
	return true;
}

bool
val_mod_resolve_clsr_id (struct val_mod *self, struct dep_val_str_view ident, u2 *out)
{
	for (size_t idx = 0; idx < val_lst_count (self->clsrs); idx++)
		{
			struct val_clsr *clsr = val_lst_at_clsr (self->clsrs, idx);
			if (dep_val_str_view_equal (
				dep_val_str_view_from_str (clsr->fn->ident),
				ident
			))
				{
					*out = idx;
					return true;
				}
		}

	return false;
}

void
val_mod_clear (struct val_mod *self)
{
	CHECK_NOT_NULL(self);
	if (self->init != NULL)
		{
			val_clsr_clear (self->init);
			val_clsr_free_safe (&self->init);
		}

	val_lst_clear (self->clsrs);
	val_lst_free_safe (&self->clsrs);

	val_mod_const_pool_reset (&self->const_pool);
}

void
val_mod_free (struct val_mod *self)
{
	CHECK_NOT_NULL(self);
	dep_val_str_deallocate_safe (&self->ident);
	mem_deallocate (self->base.mem, self);
}

void
val_mod_free_safe (struct val_mod **self)
{
	CHECK_NOT_NULL(self);
	val_mod_free (*self);
	*self = NULL;
}
