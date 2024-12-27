#include "../unit-test.h"

#include "core/val/val-str.h"
#include "core/val/val-unit.h"

TEST(val_unit_new, ok)
{
	auto tm = mem_test_new_default (128);

	struct val_unit *result = val_unit_new (MEM(tm));
	ASSERT_EQ(VAL_KIND_UNIT, result->base.kind);
	ASSERT_EQ(MEM (tm), result->base.mem);

	val_unit_free_safe (&result);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_unit_equal, same_pointer)
{
	auto tm = mem_test_new_default (64);

	struct val_unit *test_instance = val_unit_new (MEM(tm));
	ASSERT_TRUE(val_unit_equal (test_instance, test_instance));

	val_unit_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_unit_equal, different_pointer)
{
	auto tm = mem_test_new_default (64);

	struct val_unit *test_instance_one = val_unit_new (MEM(tm));
	struct val_unit *test_instance_two = val_unit_new (MEM(tm));
	ASSERT_TRUE(val_unit_equal (test_instance_one, test_instance_two));

	val_unit_free_safe (&test_instance_one);
	val_unit_free_safe (&test_instance_two);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_unit_to_str, ok)
{
	auto tm = mem_test_new_default (128);

	struct val_unit *test_instance = val_unit_new (MEM(tm));
	struct dep_val_str *result = val_unit_to_str (test_instance, MEM(tm));

	ASSERT_EQ(4, dep_val_str_count (result));
	ASSERT_TRUE(strncmp (result->data, "unit", result->count) == 0);

	val_unit_free_safe (&test_instance);
	dep_val_str_deallocate_safe (&result);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_unit_free_safe, ok)
{
	auto tm = mem_test_new_default (128);

	struct val_unit *result = val_unit_new (MEM(tm));
	val_unit_free_safe (&result);
	ASSERT_TRUE(result == nullptr);

	mem_test_verify (tm);
	mem_test_free (tm);
}