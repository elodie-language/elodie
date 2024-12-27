#include "../unit-test.h"

#include "core/val/val-str.h"
#include "core/val/val-nil.h"

TEST(val_nil_new, ok)
{
	auto tm = mem_test_new_default (128);

	struct val_nil *result = val_nil_new (MEM(tm));
	ASSERT_EQ(VAL_KIND_NIL, result->base.kind);
	ASSERT_EQ(MEM (tm), result->base.mem);

	val_nil_free_safe (&result);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_nil_equal, same_pointer)
{
	auto tm = mem_test_new_default (64);

	struct val_nil *test_instance = val_nil_new (MEM(tm));
	ASSERT_TRUE(val_nil_equal (test_instance, test_instance));

	val_nil_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_nil_equal, different_pointer)
{
	auto tm = mem_test_new_default (64);

	struct val_nil *test_instance_one = val_nil_new (MEM(tm));
	struct val_nil *test_instance_two = val_nil_new (MEM(tm));
	ASSERT_TRUE(val_nil_equal (test_instance_one, test_instance_two));

	val_nil_free_safe (&test_instance_one);
	val_nil_free_safe (&test_instance_two);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_nil_to_str, ok)
{
	auto tm = mem_test_new_default (128);

	struct val_nil *test_instance = val_nil_new (MEM(tm));
	struct dep_val_str *result = val_nil_to_str (test_instance, MEM(tm));

	ASSERT_EQ(3, dep_val_str_count (result));
	ASSERT_TRUE(strncmp (result->data, "nil", result->count) == 0);

	val_nil_free_safe (&test_instance);
	dep_val_str_deallocate_safe (&result);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_nil_free_safe, ok)
{
	auto tm = mem_test_new_default (128);

	struct val_nil *result = val_nil_new (MEM(tm));
	val_nil_free_safe (&result);
	ASSERT_TRUE(result == nullptr);

	mem_test_verify (tm);
	mem_test_free (tm);
}