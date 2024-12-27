#include "core/check.h"
#include "core/val/val-api.h"

struct dep_val *
val_copy (struct dep_val *self, struct mem *mem)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(mem);
	switch (self->kind)
		{
			case VAL_KIND_BOOL: return AS_VAL(val_bool_copy (AS_BOOL (self), mem));
			case VAL_KIND_NIL: return AS_VAL(val_nil_new (mem));
			case VAL_KIND_NUM: return AS_VAL(val_num_copy (AS_NUM (self), mem));
			case VAL_KIND_STR: return AS_VAL(dep_val_str_copy (AS_STR (self), mem));
			default: NOT_IMPLEMENTED_YET();
		}
}

struct dep_val_str *
val_to_str (struct dep_val *self, struct mem *mem)
{
	CHECK_NOT_NULL(self);
	switch (self->kind)
		{
			case VAL_KIND_BOOL: return val_bool_to_str ((struct val_bool *)self, mem);
			case VAL_KIND_NIL: return val_nil_to_str ((struct val_nil *)self, mem);
			case VAL_KIND_NUM: return val_num_to_str ((struct val_num *)self, mem);
			case VAL_KIND_STR: return (struct dep_val_str *)self;
			case VAL_KIND_UNIT: return val_unit_to_str ((struct val_unit *)self, mem);
			default: NOT_IMPLEMENTED_YET();
		}
}

bool
val_equal (struct dep_val *lhs, struct dep_val *rhs)
{
	CHECK_NOT_NULL(lhs);
	CHECK_NOT_NULL(rhs);
	if (lhs->kind != rhs->kind) return false;
	switch (lhs->kind)
		{
			case VAL_KIND_STR: return dep_val_str_equal ((struct dep_val_str *)lhs, (struct dep_val_str *)rhs);
			case VAL_KIND_NUM: return val_numb_equal ((struct val_num *)lhs, (struct val_num *)rhs);
			case VAL_KIND_BOOL: return val_bool_equal ((struct val_bool *)lhs, (struct val_bool *)rhs);
			case VAL_KIND_FN: return val_fn_equal ((struct val_fn *)lhs, (struct val_fn *)rhs);
			default: NOT_IMPLEMENTED_YET();
		}
}

void
val_clear (struct dep_val *self)
{
	CHECK_NOT_NULL(self);
	switch (self->kind)
		{

			case VAL_KIND_BOOL:break;
			case VAL_KIND_CLSR:
				{
					val_clsr_clear (AS_CLSR(self));
					break;
				};
			case VAL_KIND_FLD:break;
			case VAL_KIND_FN:break;
			case VAL_KIND_LST:
				{
					val_lst_clear (AS_LST(self));
					break;
				};
			case VAL_KIND_MOD:
				{
					val_mod_clear (AS_MOD (self));
					break;
				}
			case VAL_KIND_NIL:break;
			case VAL_KIND_NUM:break;
			case VAL_KIND_OBJ:break;
			case VAL_KIND_PROP:break;
			case VAL_KIND_STR:break;
			case VAL_KIND_STR_VIEW:break;
			case VAL_KIND_UNIT:break;
			default: NOT_IMPLEMENTED_YET();
		}
}

void
val_free (struct dep_val *self)
{
	CHECK_NOT_NULL(self);
	switch (self->kind)
		{
			case VAL_KIND_BOOL:
				{
					val_bool_free ((struct val_bool *)self);
					break;
				}
			case VAL_KIND_CLSR:
				{
					val_clsr_free ((struct val_clsr *)self);
					break;
				}
			case VAL_KIND_FLD:
				{
					val_fld_free ((struct val_fld *)self);
					break;
				}
			case VAL_KIND_FN:
				{
					val_fn_free ((struct val_fn *)self);
					break;
				}
			case VAL_KIND_LST:
				{
					val_lst_free ((struct val_lst *)self);
					break;
				}
			case VAL_KIND_MOD:
				{
					val_mod_free ((struct val_mod *)self);
					break;
				}
			case VAL_KIND_NIL:
				{
					val_nil_free ((struct val_nil *)self);
					break;
				}
			case VAL_KIND_NUM:
				{
					val_num_free ((struct val_num *)self);
					break;
				}
			case VAL_KIND_OBJ:
				{
					val_obj_free ((struct val_obj *)self);
					break;
				}
			case VAL_KIND_STR:
				{
					dep_val_str_deallocate ((struct dep_val_str *)self);
					break;
				}
			case VAL_KIND_UNIT:
				{
					val_unit_free ((struct val_unit *)self);
					break;
				}
			default:
				{
					NOT_IMPLEMENTED_YET();
				}
		}
}

void
val_free_safe (struct dep_val **self)
{
	CHECK_NOT_NULL(self);
	val_free (*self);
	*self = NULL;
}

bool
_val_equal_str_view_c_str (struct dep_val_str_view lhs, char const *rhs)
{
	return dep_val_str_view_equal (lhs, dep_val_str_view_from_c_str (rhs));
}

bool
_val_equal_str_c_str (struct dep_val_str *lhs, char const *rhs)
{
	return dep_val_str_view_equal (
		dep_val_str_view_from_str (lhs),
		dep_val_str_view_from_c_str (rhs)
	);
}

bool
_val_equal_str_str_view (struct dep_val_str *lhs, struct dep_val_str_view rhs)
{
	return dep_val_str_view_equal (
		dep_val_str_view_from_str (lhs),
		rhs
	);
}

bool
_val_equal_str_view_str (struct dep_val_str_view lhs, struct dep_val_str *rhs)
{
	return _val_equal_str_str_view (rhs, lhs);
}

struct val_bool *
val_as_bool (struct dep_val *val)
{
	CHECK_NOT_NULL(val);
	CHECK_EQUAL(VAL_KIND_BOOL, val->kind);
	return (struct val_bool *)val;
}

struct val_clsr *
val_as_clsr (struct dep_val *val)
{
	CHECK_NOT_NULL(val);
	CHECK_EQUAL(VAL_KIND_CLSR, val->kind);
	return (struct val_clsr *)val;
}

struct val_lst *
val_as_lst (struct dep_val *val)
{
	CHECK_NOT_NULL(val);
	CHECK_EQUAL(VAL_KIND_LST, val->kind);
	return (struct val_lst *)val;
}

struct val_mod *
val_as_mod (struct dep_val *val)
{
	CHECK_NOT_NULL(val);
	CHECK_EQUAL(VAL_KIND_MOD, val->kind);
	return (struct val_mod *)val;
}

struct val_num *
val_as_num (struct dep_val *val)
{
	CHECK_NOT_NULL(val);
	CHECK_EQUAL(VAL_KIND_NUM, val->kind);
	return (struct val_num *)val;
}

struct val_obj *
val_as_obj (struct dep_val *val)
{
	CHECK_NOT_NULL(val);
	CHECK_EQUAL(VAL_KIND_OBJ, val->kind);
	return (struct val_obj *)val;
}

struct val_prop *
val_as_prop (struct dep_val *val)
{
	CHECK_NOT_NULL(val);
	CHECK_EQUAL(VAL_KIND_PROP, val->kind);
	return (struct val_prop *)val;
}

struct dep_val_str *
val_as_str (struct dep_val *val)
{
	CHECK_NOT_NULL(val);
	CHECK_EQUAL(VAL_KIND_STR, val->kind);
	return (struct dep_val_str *)val;
}
