#include "../unit-test.h"

#include "core/val/val-api.h"

TEST(val_obj_new, ok)
{
	auto tm = mem_test_new_default (1024);

	struct val_obj *result = val_obj_new (MEM(tm), dep_val_str_view_from_c_str ("some_obj"));
	ASSERT_EQ(VAL_KIND_OBJ, result->base.kind);
	ASSERT_EQ(MEM (tm), result->base.mem);
	ASSERT_TRUE(VAL_EQ (result->ident, "some_obj"));

	ASSERT_EQ(0, ptr_list_count (&result->props));
	ASSERT_EQ(0, ptr_list_count (&result->values));

	val_obj_clear (result);
	val_obj_free_safe (&result);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_obj, single_number)
{
	auto tm = mem_test_new_default (1024);

	auto test_instance = val_obj_new (MEM(tm), DEP_VAL_STR_VIEW ("some_obj"));

	auto some_number_value = val_num_new_from_double (MEM(tm), 28.10);
	auto some_number_field = val_fld_new (MEM(tm), DEP_VAL_STR_VIEW ("some_number"), type_number);

	val_obj_append (test_instance, some_number_field, AS_VAL(some_number_value));
	ASSERT_EQ(1, ptr_list_count (&test_instance->props));
	ASSERT_EQ(1, ptr_list_count (&test_instance->values));

	auto value_from_obj = val_obj_val_at (test_instance, 0);
	ASSERT_EQ(VAL_KIND_NUM, value_from_obj->kind);
	ASSERT_EQ(AS_VAL (some_number_value), value_from_obj);

	val_obj_clear (test_instance);
	val_obj_free_safe (&test_instance);
	val_fld_free_safe (&some_number_field);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_obj, simple_nested)
{
	/**
	 * memory layout
	 * [ inner obj ]
	 * [ some_number ]
	 */
	auto tm = mem_test_new_default (1024);

	auto some_number_value = val_num_new_from_double (MEM(tm), 28.10);
	auto some_number_field = val_fld_new (MEM(tm), DEP_VAL_STR_VIEW ("some_number"), type_number);

	auto inner_obj = val_obj_new (MEM(tm), DEP_VAL_STR_VIEW ("inner_obj"));
	val_obj_append (inner_obj, some_number_field, AS_VAL(some_number_value));

	auto inner_obj_field = val_fld_new (MEM(tm), DEP_VAL_STR_VIEW ("inner_obj"), type_object);
	auto test_instance = val_obj_new (MEM(tm), DEP_VAL_STR_VIEW ("some_obj"));

	val_obj_append (test_instance, inner_obj_field, AS_VAL(inner_obj));
	ASSERT_EQ(2, ptr_list_count (&test_instance->props));
	ASSERT_EQ(2, ptr_list_count (&test_instance->values));

	auto inner_obj_from_obj = val_obj_val_at (test_instance, 0);
	ASSERT_EQ(VAL_KIND_OBJ, inner_obj_from_obj->kind);
	ASSERT_EQ(AS_VAL (inner_obj), inner_obj_from_obj);

	auto inner_obj_prop = val_obj_prop_at (test_instance, 0);
	ASSERT_EQ(0, inner_obj_prop->id);

	auto some_number_from_obj = val_obj_val_at (test_instance, 1);
	ASSERT_EQ(VAL_KIND_NUM, some_number_from_obj->kind);
	ASSERT_EQ(AS_VAL (some_number_value), some_number_from_obj);

	auto some_number_prop = val_obj_prop_at (test_instance, 1);
	ASSERT_EQ(1, some_number_prop->id);

	val_obj_clear (test_instance);
	val_obj_free_safe (&test_instance);

	val_fld_free_safe (&some_number_field);
	val_fld_free_safe (&inner_obj_field);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_obj_free_safe, ok)
{
	auto tm = mem_test_new_default (1024);

	struct val_obj *test_instance = val_obj_new (MEM(tm), dep_val_str_view_from_c_str ("some_obj"));
	val_obj_free_safe (&test_instance);
	ASSERT_TRUE(test_instance == nullptr);

	mem_test_verify (tm);
	mem_test_free (tm);
}