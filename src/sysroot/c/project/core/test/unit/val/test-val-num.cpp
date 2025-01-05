#include "../unit-test.h"

#include "core/val/val-str.h"
#include "core/val/val-num.h"

TEST(val_num_new, ok)
{
	auto tm = mem_test_new_default (64);

	struct val_num *result = val_num_new(MEM(tm), 12.34);
	ASSERT_EQ(12.34, result->data);

	ASSERT_EQ(VAL_KIND_NUM, result->base.kind);
	ASSERT_EQ(MEM (tm), result->base.mem);

	val_num_free_safe (&result);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_num_copy, ok)
{
	auto tm = mem_test_new_default (64);

	struct val_num *test_instance = val_num_new(MEM(tm), 12.34);
	struct val_num *result = val_num_copy (test_instance, MEM(tm));
	ASSERT_EQ(12.34, result->data);

	ASSERT_EQ(VAL_KIND_NUM, result->base.kind);
	ASSERT_EQ(MEM (tm), result->base.mem);

	val_num_free_safe (&test_instance);
	val_num_free_safe (&result);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_num_to_str, floating_point)
{
	auto tm = mem_test_new_default (128);

	struct val_num *test_instance = val_num_new(MEM(tm), 13.37);
	struct val_str *result = val_num_to_str (test_instance, MEM(tm));

	ASSERT_EQ(5, val_str_count (result));
	ASSERT_TRUE(strncmp (result->data, "13.37", result->count) == 0);

	val_num_free_safe (&test_instance);
    val_str_free_safe(&result);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_num_free_safe, ok)
{
	auto tm = mem_test_new_default (64);

	struct val_num *test_instance = val_num_new(MEM(tm), 12.34);
	val_num_free_safe (&test_instance);
	ASSERT_TRUE(test_instance == nullptr);

	mem_test_verify (tm);
	mem_test_free (tm);
}
