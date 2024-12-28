#include "../unit-test.h"

#include "core/algo/algo-list-byte.h"
#include "core/val/val-fn.h"
#include "core/val/val-str.h"

TEST(val_fn_block_new, ok)
{
	auto tm = mem_test_new_default (256);

	struct val_fn_block *test_instance = val_fn_block_new (MEM(tm));
	ASSERT_EQ (0, val_fn_block_count (test_instance));
	ASSERT_TRUE (test_instance->data != nullptr);

	val_fn_block_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_fn_block_append, ok)
{
	auto tm = mem_test_new_default (256);

	struct val_fn_block *test_instance = val_fn_block_new (MEM(tm));
	ASSERT_EQ (0, val_fn_block_count (test_instance));

	val_fn_block_append (test_instance, 1024);
	ASSERT_EQ (1, val_fn_block_count (test_instance));
	ASSERT_EQ (4, byte_list_size (test_instance->data));

	val_fn_block_append (test_instance, 2048);
	ASSERT_EQ (2, val_fn_block_count (test_instance));
	ASSERT_EQ (8, byte_list_size (test_instance->data));

	val_fn_block_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_fn_block_count, ok)
{
	auto tm = mem_test_new_default (1024);

	struct val_fn_block *test_instance = val_fn_block_new (MEM(tm));
	val_fn_block_append (test_instance, 1024);

	ASSERT_EQ (1, val_fn_block_count (test_instance));

	val_fn_block_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_fn_block_count, empty)
{
	auto tm = mem_test_new_default (256);

	struct val_fn_block *test_instance = val_fn_block_new (MEM(tm));
	ASSERT_EQ (0, val_fn_block_count (test_instance));

	val_fn_block_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_fn_block_free_safe, ok)
{
	auto tm = mem_test_new_default (1024);

	struct val_fn_block *test_instance = val_fn_block_new (MEM(tm));

	val_fn_block_free_safe (&test_instance);
	ASSERT_TRUE (test_instance == nullptr);

	mem_test_verify (tm);
	mem_test_free (tm);
}

// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-[val fn]+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-

TEST(val_fn_new, ok)
{
	auto tm = mem_test_new_default (4096);

	struct val_fn *test_instance = val_fn_new (
		MEM(tm), val_str_view_from_c_str ("some_fn")
	);

	ASSERT_EQ(VAL_KIND_FN, test_instance->base.kind);
	ASSERT_EQ(MEM (tm), test_instance->base.mem);
	ASSERT_TRUE(val_str_view_equal (val_str_view_from_c_str ("some_fn"), val_str_view_from_str (test_instance->ident)));

	val_fn_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_fn_count, ok)
{
	auto tm = mem_test_new_default (4096);

	struct val_fn *test_instance = val_fn_new (
		MEM(tm), val_str_view_from_c_str ("some_fn")
	);

	struct val_fn_block *first_block = val_fn_block_new (MEM(tm));
	val_fn_append_block (test_instance, first_block);
	ASSERT_EQ (1, val_fn_count (test_instance));
	struct val_fn_block *second_block = val_fn_block_new (MEM(tm));
	val_fn_append_block (test_instance, second_block);
	ASSERT_EQ (2, val_fn_count (test_instance));

	val_fn_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_fn_count, empty)
{

	auto tm = mem_test_new_default (256);

	struct val_fn *test_instance = val_fn_new (
		MEM(tm), val_str_view_from_c_str ("some_fn")
	);

	ASSERT_EQ (0, val_fn_count (test_instance));

	val_fn_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_fn_append_block, ok)
{
	auto tm = mem_test_new_default (4096);

	struct val_fn *test_instance = val_fn_new (
		MEM(tm), val_str_view_from_c_str ("some_fn")
	);

	val_fn_append_block (test_instance, val_fn_block_new (MEM(tm)));
	ASSERT_EQ (1, val_fn_count (test_instance));
	val_fn_append_block (test_instance, val_fn_block_new (MEM(tm)));
	ASSERT_EQ (2, val_fn_count (test_instance));
	val_fn_append_block (test_instance, val_fn_block_new (MEM(tm)));
	ASSERT_EQ (3, val_fn_count (test_instance));

	val_fn_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_fn_get_block_at, ok)
{
	auto tm = mem_test_new_default (4096);

	struct val_fn *test_instance = val_fn_new (
		MEM(tm), val_str_view_from_c_str ("some_fn")
	);

	val_fn_append_block (test_instance, val_fn_block_new (MEM(tm)));

	auto second_block = val_fn_block_new (MEM(tm));
	val_fn_append_block (test_instance, second_block);
	val_fn_append_block (test_instance, val_fn_block_new (MEM(tm)));

	auto result = val_fn_get_block_at (test_instance, 1);
	ASSERT_EQ (second_block, result);

	val_fn_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_fn_equal, same_pointer)
{
	auto tm = mem_test_new_default (4096);
	auto test_instance = val_fn_new (MEM(tm), val_str_view_from_c_str ("some_fn"));
	ASSERT_TRUE (val_fn_equal (test_instance, test_instance));

	val_fn_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);

}

TEST(val_fn_equal, same_val)
{
	auto tm = mem_test_new_default (4096);
	auto test_instance_one = val_fn_new (MEM(tm), val_str_view_from_c_str ("some_fn"));
	auto test_instance_two = val_fn_new (MEM(tm), val_str_view_from_c_str ("some_fn"));
	ASSERT_TRUE (val_fn_equal (test_instance_one, test_instance_two));

	val_fn_free_safe (&test_instance_one);
	val_fn_free_safe (&test_instance_two);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_fn_equal, different_val)
{
	auto tm = mem_test_new_default (4096);
	auto test_instance_one = val_fn_new (MEM(tm), val_str_view_from_c_str ("some_fn"));
	auto test_instance_two = val_fn_new (MEM(tm), val_str_view_from_c_str ("another_fn"));
	ASSERT_FALSE (val_fn_equal (test_instance_one, test_instance_two));

	val_fn_free_safe (&test_instance_one);
	val_fn_free_safe (&test_instance_two);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_fn_to_str, ok)
{
	auto tm = mem_test_new_default (4096);

	struct val_fn *test_instance = val_fn_new (
		MEM(tm), val_str_view_from_c_str ("some_fn")
	);

	struct val_str *result = val_fn_to_str (test_instance, MEM(tm));
	struct val_str *expected = val_str_allocate_from_c_str (MEM(tm), "some_fn");

	ASSERT_TRUE (val_str_equal (expected, result));

	val_fn_free_safe (&test_instance);
	val_str_deallocate_safe (&result);
	val_str_deallocate_safe (&expected);

	mem_test_verify (tm);
	mem_test_free (tm);

}

TEST(val_fn_free_safe, ok)
{
	auto tm = mem_test_new_default (4096);

	struct val_fn *test_instance = val_fn_new (
		MEM(tm), val_str_view_from_c_str ("some_fn")
	);

	val_fn_append_block (test_instance, val_fn_block_new (MEM(tm)));
	val_fn_append_block (test_instance, val_fn_block_new (MEM(tm)));

	val_fn_free_safe (&test_instance);
	ASSERT_TRUE (test_instance == nullptr);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_fn_free_safe, empty)
{
	auto tm = mem_test_new_default (256);

	struct val_fn *test_instance = val_fn_new (
		MEM(tm), val_str_view_from_c_str ("some_fn")
	);

	val_fn_free_safe (&test_instance);
	ASSERT_TRUE (test_instance == nullptr);

	mem_test_verify (tm);
	mem_test_free (tm);
}