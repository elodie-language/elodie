#include "core/check.h"
#include "core/algo/algo-list-ptr.h"
#include "core/val/val-api.h"
#include "core/val/val-lst.h"

struct val_lst *
val_lst_new (struct mem *mem)
{
	CHECK_NOT_NULL(mem);

	struct val_lst *result = mem_allocate (mem, sizeof (struct val_lst));
	dep_val_init (&result->base, VAL_KIND_LST, mem);

	struct ptr_list_config data_config = {
		.initial_capacity = 8,
		.mem = mem
	};

	result->underlying_list = ptr_list_new (data_config);

	return result;
}

void
val_lst_append_base (struct val_lst *self, struct dep_val *val)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(val);
	ptr_list_append (self->underlying_list, val);
}

void
val_lst_append_bool (struct val_lst *self, struct val_bool *val)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(val);
	ptr_list_append (self->underlying_list, val);
}

void
val_lst_append_clsr (struct val_lst *self, struct val_clsr *val)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(val);
	ptr_list_append (self->underlying_list, val);
}

void
val_lst_append_field (struct val_lst *self, struct val_fld *val)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(val);
	ptr_list_append (self->underlying_list, val);
}

void
val_lst_append_fn (struct val_lst *self, struct val_fn *val)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(val);
	ptr_list_append (self->underlying_list, val);
}

void
val_lst_append_list (struct val_lst *self, struct val_lst *val)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(val);
	ptr_list_append (self->underlying_list, val);
}

void
val_lst_append_mod (struct val_lst *self, struct val_mod *val)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(val);
	ptr_list_append (self->underlying_list, val);
}

void
val_lst_append_nil (struct val_lst *self, struct val_nil *val)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(val);
	ptr_list_append (self->underlying_list, val);
}

void
val_lst_append_num (struct val_lst *self, struct val_num *val)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(val);
	ptr_list_append (self->underlying_list, val);
}

void
val_lst_append_obj (struct val_lst *self, struct val_obj *val)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(val);
	ptr_list_append (self->underlying_list, val);
}

void
val_lst_append_str (struct val_lst *self, struct dep_val_str *val)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(val);
	ptr_list_append (self->underlying_list, val);
}

static inline void *
direct_underlying_data (struct val_lst *self, size_t idx)
{
	return ptr_list_at (self->underlying_list, idx);
}

void
val_lst_replace_base (struct val_lst *self, size_t idx, struct dep_val *val)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(val);
	ptr_list_replace (self->underlying_list, idx, val);
}

struct dep_val *
val_lst_at_base (struct val_lst *self, size_t idx)
{
	CHECK_NOT_NULL(self);
	CHECK_LESS_THAN_EQUAL(idx, val_lst_count (self));
	return ptr_list_at (self->underlying_list, idx);
}

struct val_bool *
val_lst_at_bool (struct val_lst *self, size_t idx)
{
	CHECK_NOT_NULL(self);
	CHECK_LESS_THAN_EQUAL(idx, val_lst_count (self));
	struct dep_val *entry = ptr_list_at (self->underlying_list, idx);
	CHECK_EQUAL(VAL_KIND_BOOL, entry->kind)
	return (struct val_bool *)entry;
}

struct val_clsr *
val_lst_at_clsr (struct val_lst *self, size_t idx)
{
	CHECK_NOT_NULL(self);
	CHECK_LESS_THAN_EQUAL(idx, val_lst_count (self));
	struct dep_val *entry = ptr_list_at (self->underlying_list, idx);
	CHECK_EQUAL(VAL_KIND_CLSR, entry->kind)
	return (struct val_clsr *)entry;
}

struct val_fld *
val_lst_at_field (struct val_lst *self, size_t idx)
{
	CHECK_LESS_THAN_EQUAL(idx, val_lst_count (self));
	struct dep_val *entry = ptr_list_at (self->underlying_list, idx);
	CHECK_EQUAL(VAL_KIND_FLD, entry->kind)
	return (struct val_fld *)entry;
}

struct val_fn *
val_lst_at_fn (struct val_lst *self, size_t idx)
{
	CHECK_NOT_NULL(self);
	CHECK_LESS_THAN_EQUAL(idx, val_lst_count (self));
	struct dep_val *entry = ptr_list_at (self->underlying_list, idx);
	CHECK_EQUAL(VAL_KIND_FN, entry->kind)
	return (struct val_fn *)entry;
}

struct val_lst *
val_lst_at_list (struct val_lst *self, size_t idx)
{
	CHECK_NOT_NULL(self);
	CHECK_LESS_THAN_EQUAL(idx, val_lst_count (self));
	struct dep_val *entry = ptr_list_at (self->underlying_list, idx);
	CHECK_EQUAL(VAL_KIND_LST, entry->kind)
	return (struct val_lst *)entry;
}

struct val_mod *
val_lst_at_mod (struct val_lst *self, size_t idx)
{
	CHECK_NOT_NULL(self);
	CHECK_LESS_THAN_EQUAL(idx, val_lst_count (self));
	struct dep_val *entry = ptr_list_at (self->underlying_list, idx);
	CHECK_EQUAL(VAL_KIND_MOD, entry->kind)
	return (struct val_mod *)entry;
}

struct val_num *
val_lst_at_num (struct val_lst *self, size_t idx)
{
	CHECK_NOT_NULL(self);
	CHECK_LESS_THAN_EQUAL(idx, val_lst_count (self));
	struct dep_val *entry = ptr_list_at (self->underlying_list, idx);
	CHECK_EQUAL(VAL_KIND_NUM, entry->kind)
	return (struct val_num *)entry;
}

struct val_obj *
val_lst_at_obj (struct val_lst *self, size_t idx)
{
	CHECK_NOT_NULL(self);
	CHECK_LESS_THAN_EQUAL(idx, val_lst_count (self));
	struct dep_val *entry = ptr_list_at (self->underlying_list, idx);
	CHECK_EQUAL(VAL_KIND_OBJ, entry->kind)
	return (struct val_obj *)entry;
}

struct dep_val_str *
val_lst_at_str (struct val_lst *self, size_t idx)
{
	CHECK_NOT_NULL(self);
	CHECK_LESS_THAN_EQUAL(idx, val_lst_count (self));
	struct dep_val *entry = ptr_list_at (self->underlying_list, idx);
	CHECK_EQUAL(VAL_KIND_STR, entry->kind)
	return (struct dep_val_str *)entry;
}

size_t
val_lst_count (struct val_lst *self)
{
	CHECK_NOT_NULL(self);
	return ptr_list_count (self->underlying_list);
}

size_t
val_lst_capacity (struct val_lst *self)
{
	CHECK_NOT_NULL(self);
	return ptr_list_capacity (self->underlying_list);
}

void
val_lst_clear (struct val_lst *self)
{
	CHECK_NOT_NULL(self);
	for (size_t idx = 0; idx < ptr_list_count (self->underlying_list); idx++)
		{
			struct dep_val *val = val_lst_at_base (self, idx);
			val_clear (val);
			val_free_safe (&val);
		}
	ptr_list_reset (self->underlying_list);
}

void
val_lst_free (struct val_lst *self)
{
	CHECK_NOT_NULL(self);
	ptr_list_free_safe (&self->underlying_list);
	mem_deallocate (self->base.mem, self);
}

void
val_lst_free_safe (struct val_lst **self)
{
	CHECK_NOT_NULL(self);
	val_lst_free (*self);
	*self = NULL;
}
