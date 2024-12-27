#include "../unit-test.h"
#include "core/type/type-system.h"
#include "core/val/val-api.h"

TEST(type_system_new, ok)
{
	auto tm = mem_test_new_default (1024);

	auto result = type_system_new (MEM(tm));
	ASSERT_EQ(MEM (tm), result->mem);
	// ensure number of base types
	ASSERT_EQ(6, ptr_list_count (&result->nodes));
	// each base type has one edge (any-any or any-type)
	ASSERT_EQ(6, ptr_list_count (&result->edges));

	type_system_free_safe (&result);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(type_system_base_types, any)
{
	auto tm = mem_test_new_default (1024);

	auto test_instance = type_system_new (MEM(tm));
	ASSERT_EQ(MEM (tm), test_instance->mem);

	auto any_type_info = type_system_get_info (test_instance, type_any);
	ASSERT_EQ(0, any_type_info.id);
	ASSERT_EQ(0, any_type_info.base_id);
	ASSERT_TRUE(VAL_EQ (any_type_info.ident, "any"));

	ASSERT_TRUE(type_system_is_base_type (test_instance, type_any));

	type_system_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(type_system_base_types, nil)
{
	auto tm = mem_test_new_default (1024);

	auto test_instance = type_system_new (MEM(tm));
	ASSERT_EQ(MEM (tm), test_instance->mem);

	auto nil_type_info = type_system_get_info (test_instance, type_nil);
	ASSERT_EQ(1, nil_type_info.id);
	ASSERT_EQ(0, nil_type_info.base_id);
	ASSERT_TRUE(VAL_EQ (nil_type_info.ident, "nil"));

	ASSERT_TRUE(type_system_is_base_type (test_instance, type_nil));

	type_system_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(type_system_base_types, obj)
{
	auto tm = mem_test_new_default (1024);

	auto test_instance = type_system_new (MEM(tm));
	ASSERT_EQ(MEM (tm), test_instance->mem);

	auto obj_type_info = type_system_get_info (test_instance, type_object);
	ASSERT_EQ(2, obj_type_info.id);
	ASSERT_EQ(0, obj_type_info.base_id);
	ASSERT_TRUE(VAL_EQ (obj_type_info.ident, "object"));

	ASSERT_TRUE(type_system_is_base_type (test_instance, type_object));

	type_system_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(type_system_base_types, number)
{
	auto tm = mem_test_new_default (1024);

	auto test_instance = type_system_new (MEM(tm));
	ASSERT_EQ(MEM (tm), test_instance->mem);

	auto number_type_info = type_system_get_info (test_instance, type_number);
	ASSERT_EQ(3, number_type_info.id);
	ASSERT_EQ(0, number_type_info.base_id);
	ASSERT_TRUE(VAL_EQ (number_type_info.ident, "number"));

	ASSERT_TRUE(type_system_is_base_type (test_instance, type_number));

	type_system_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(type_system_base_types, string)
{
	auto tm = mem_test_new_default (1024);

	auto test_instance = type_system_new (MEM(tm));
	ASSERT_EQ(MEM (tm), test_instance->mem);

	auto string_type_info = type_system_get_info (test_instance, type_string);
	ASSERT_EQ(4, string_type_info.id);
	ASSERT_EQ(0, string_type_info.base_id);
	ASSERT_TRUE(VAL_EQ (string_type_info.ident, "string"));

	ASSERT_TRUE(type_system_is_base_type (test_instance, type_string));

	type_system_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(type_system_base_types, unit)
{
	auto tm = mem_test_new_default (1024);

	auto test_instance = type_system_new (MEM(tm));
	ASSERT_EQ(MEM (tm), test_instance->mem);

	auto unit_type_info = type_system_get_info (test_instance, type_unit);
	ASSERT_EQ(5, unit_type_info.id);
	ASSERT_EQ(0, unit_type_info.base_id);
	ASSERT_TRUE(VAL_EQ (unit_type_info.ident, "unit"));

	ASSERT_TRUE(type_system_is_base_type (test_instance, type_string));

	type_system_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(type_system_compose, number)
{
	auto tm = mem_test_new_default (1024);

	auto test_instance = type_system_new (MEM(tm));
	ASSERT_EQ(MEM (tm), test_instance->mem);

	auto new_type = type_system_compose (test_instance, type_object, type_number);
	ASSERT_EQ(6, new_type.id);

	auto new_type_info = type_system_get_info (test_instance, new_type);
	ASSERT_EQ(6, new_type_info.id);
	ASSERT_EQ(type_object.id, new_type_info.base_id);
	ASSERT_TRUE(VAL_EQ (new_type_info.ident, "object"));
	ASSERT_FALSE(type_system_is_base_type (test_instance, new_type));

	type_system_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(type_system_compose, nested_object)
{
	auto tm = mem_test_new_default (1024);

	auto test_instance = type_system_new (MEM(tm));
	ASSERT_EQ(MEM (tm), test_instance->mem);

	/**
	 * outer: object ={
	 * 	num: number = .. ,
	 * 	inner: inner
	 * }
	 * inner: object = {
	 * 	num: number = ..
	 * }
	 */

	auto inner_type = type_system_compose (test_instance, type_object, type_number);
	ASSERT_EQ(6, inner_type.id);

	auto inner_type_info = type_system_get_info (test_instance, inner_type);
	ASSERT_EQ(6, inner_type_info.id);
	ASSERT_EQ(type_object.id, inner_type_info.base_id);
	ASSERT_TRUE(VAL_EQ (inner_type_info.ident, "object"));
	ASSERT_FALSE(type_system_is_base_type (test_instance, inner_type));

	// at this point in type both types are equal as both are { number }
	auto outer_type = type_system_compose (test_instance, type_object, type_number);
	ASSERT_EQ(6, outer_type.id);

	ASSERT_TRUE(type_equal (inner_type, outer_type));

	// inner_type is now part of outer_type
	outer_type = type_system_compose (test_instance, outer_type, inner_type);
	ASSERT_EQ(7, outer_type.id);

	ASSERT_FALSE(type_equal (inner_type, outer_type));

	type_system_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(type_system_free_safe, ok)
{
	auto tm = mem_test_new_default (1024);
	auto test_instance = type_system_new (MEM(tm));

	type_system_free_safe (&test_instance);
	ASSERT_TRUE(test_instance == nullptr);
	mem_test_verify (tm);
	mem_test_free (tm);
}