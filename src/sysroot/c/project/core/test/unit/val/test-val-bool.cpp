#include "../unit-test.h"

#include "core/val/val-bool.h"
#include "core/val/val-str.h"

TEST(val_bool_new_from_bool, ok)
{
	auto tm = mem_test_new_default (64);

	struct val_bool *test_instance = val_bool_new_from_bool (MEM(tm), true);
	ASSERT_EQ (true, test_instance->data);
	ASSERT_EQ (VAL_KIND_BOOL, test_instance->base.kind);
	ASSERT_EQ (MEM (tm), test_instance->base.mem);

	val_bool_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_bool_equal, same_pointer)
{
	auto tm = mem_test_new_default (64);

	struct val_bool *test_instance = val_bool_new_from_bool (MEM(tm), true);
	ASSERT_TRUE (val_bool_equal (test_instance, test_instance));

	val_bool_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_bool_equal, same_val)
{
	auto tm = mem_test_new_default (64);

	struct val_bool *bool_one = val_bool_new_from_bool (MEM(tm), true);
	struct val_bool *bool_two = val_bool_new_from_bool (MEM(tm), true);
	ASSERT_TRUE (val_bool_equal (bool_one, bool_two));

	val_bool_free_safe (&bool_one);
	val_bool_free_safe (&bool_two);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_bool_equal, different_val)
{
	auto tm = mem_test_new_default (64);

	struct val_bool *bool_one = val_bool_new_from_bool (MEM(tm), true);
	struct val_bool *bool_two = val_bool_new_from_bool (MEM(tm), false);
	ASSERT_FALSE (val_bool_equal (bool_one, bool_two));

	val_bool_free_safe (&bool_one);
	val_bool_free_safe (&bool_two);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_bool_to_str, true)
{
	auto tm = mem_test_new_default (128);

	struct val_bool *test_instance = val_bool_new_from_bool (MEM(tm), true);
	struct val_str *result = val_bool_to_str (test_instance, MEM(tm));
	struct val_str *expected = val_str_allocate_from_c_str (MEM(tm), "true");

	ASSERT_TRUE (val_str_equal (expected, result));

	val_bool_free_safe (&test_instance);
	val_str_deallocate_safe (&result);
	val_str_deallocate_safe (&expected);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_bool_to_str, false)
{
	auto tm = mem_test_new_default (128);

	struct val_bool *test_instance = val_bool_new_from_bool (MEM(tm), false);
	struct val_str *result = val_bool_to_str (test_instance, MEM(tm));
	struct val_str *expected = val_str_allocate_from_c_str (MEM(tm), "false");

	ASSERT_TRUE (val_str_equal (expected, result));

	val_bool_free_safe (&test_instance);
	val_str_deallocate_safe (&result);
	val_str_deallocate_safe (&expected);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_bool_free_safe, ok)
{
	auto tm = mem_test_new_default (64);

	struct val_bool *test_instance = val_bool_new_from_bool (MEM(tm), true);
	val_bool_free_safe (&test_instance);
	ASSERT_TRUE (test_instance == nullptr);

	mem_test_verify (tm);
	mem_test_free (tm);
}
