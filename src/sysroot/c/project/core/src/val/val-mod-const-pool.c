#include "core/check.h"
#include "core/val/val-api.h"

void
val_mod_const_pool_init (struct val_mod_const_pool *self, struct mem *mem)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(mem);

	self->mem = mem;
	self->strs = val_lst_new (mem);
	self->fields = val_lst_new (mem);
	self->objs = val_lst_new (mem);
}

void
val_mod_const_pool_append_str (struct val_mod_const_pool *self, struct dep_val_str_view str)
{
	CHECK_NOT_NULL(self);
	val_lst_append_str (self->strs, dep_val_str_allocate_from_view (self->mem, str));
}

void
val_mod_const_pool_append_field (struct val_mod_const_pool *self, struct val_fld *field)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(field);
	val_lst_append_field (self->fields, field);
}

void
val_mod_const_pool_append_obj (struct val_mod_const_pool *self, struct val_obj *obj)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(obj);
	val_lst_append_obj (self->objs, obj);
}

struct dep_val_str_view
val_mod_const_pool_get_str_view (struct val_mod_const_pool *self, size_t idx)
{
	CHECK_LESS_THAN(idx, val_lst_count (self->strs));
	return dep_val_str_view_from_str (val_lst_at_str (self->strs, idx));
}

struct val_fld *
val_mod_const_pool_get_field (struct val_mod_const_pool *self, size_t idx)
{
	CHECK_LESS_THAN(idx, val_lst_count (self->fields));
	return val_lst_at_field (self->fields, idx);
}

struct val_obj *
val_mod_const_pool_get_obj (struct val_mod_const_pool *self, size_t idx)
{
	CHECK_LESS_THAN(idx, val_lst_count (self->objs));
	return val_lst_at_obj (self->objs, idx);
}

void
val_mod_const_pool_reset (struct val_mod_const_pool *self)
{
	val_lst_clear (self->strs);
	val_lst_free_safe (&self->strs);

	val_lst_clear (self->fields);
	val_lst_free_safe (&self->fields);

	val_lst_clear (self->objs);
	val_lst_free_safe (&self->objs);
	self->mem = NULL;
}
