#include "../unit-test.h"

#include "core/algo/algo-list-byte.h"
#include "core/val/val-str.h"
#include "core/val/val-str-view.h"

TEST(val_str_new_from_c_str, ok)
{
	auto tm = mem_test_new_default (128);

	struct val_str *result = val_str_new_from_c_str (MEM(tm), "Elodie");
	ASSERT_EQ(6, val_str_count (result));
	ASSERT_TRUE(strncmp (result->data, "Elodie", result->count) == 0);

	ASSERT_EQ(VAL_KIND_STR, result->base.kind);
	ASSERT_EQ(MEM (tm), result->base.mem);

    val_str_free_safe(&result);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_str_new_from_bytes, ok)
{
	auto tm = mem_test_new_default (128);

	u1 input_array[] = {'H', 'A', 'M', 'A', 'L'};
	struct bytes_view bytes = {
		.data = input_array,
		.size = 2
	};

	struct val_str *result = val_str_new_from_bytes (MEM(tm), bytes);
	ASSERT_EQ(2, val_str_count (result));
	ASSERT_TRUE(strncmp (result->data, "HA", result->count) == 0);

	ASSERT_EQ(VAL_KIND_STR, result->base.kind);
	ASSERT_EQ(MEM (tm), result->base.mem);

    val_str_free_safe(&result);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_str_new_from_view, ok)
{
	auto tm = mem_test_new_default (128);

	struct val_str *result = val_str_new_from_view (MEM(tm), val_str_view_from_c_str ("ELODIE"));
	ASSERT_EQ(6, val_str_count (result));
	ASSERT_TRUE(strncmp (result->data, "ELODIE", result->count) == 0);

	ASSERT_EQ(VAL_KIND_STR, result->base.kind);
	ASSERT_EQ(MEM (tm), result->base.mem);

    val_str_free_safe(&result);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_str_new_from_byte_list, ok)
{
	auto tm = mem_test_new_default (128);

	auto byte_list_config = byte_list_default_config (MEM(tm));
	auto byte_list = byte_list_new (byte_list_config);
	byte_list_append_u1 (byte_list, 'h');
	byte_list_append_u1 (byte_list, '4');
	byte_list_append_u1 (byte_list, 'm');
	byte_list_append_u1 (byte_list, '4');
	byte_list_append_u1 (byte_list, 'l');

	struct val_str *result = val_str_new_from_byte_list (MEM(tm), byte_list);
	ASSERT_EQ(5, val_str_count (result));
	ASSERT_TRUE(strncmp (result->data, "h4m4l", result->count) == 0);

	ASSERT_EQ(VAL_KIND_STR, result->base.kind);
	ASSERT_EQ(MEM (tm), result->base.mem);

	byte_list_free_safe (&byte_list);
    val_str_free_safe(&result);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_str_copy, ok)
{
	auto tm = mem_test_new_default (128);

	struct val_str *given_str = val_str_new_from_c_str (MEM(tm), "Elodie");
	struct val_str *result = val_str_copy (given_str, MEM(tm));

	ASSERT_EQ(6, val_str_count (result));
	ASSERT_TRUE(strncmp (result->data, "Elodie", result->count) == 0);

	ASSERT_EQ(VAL_KIND_STR, result->base.kind);
	ASSERT_EQ(MEM (tm), result->base.mem);

    val_str_free_safe(&given_str);
    val_str_free_safe(&result);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_str_count, ok)
{
	auto test_instance = val_str_new_from_c_str (MEM(mem_raw_new ()), "!!Elodie Rocks!!\n");
	ASSERT_EQ(17, val_str_count (test_instance));
    val_str_free_safe(&test_instance);
}

TEST(val_str_count, empty)
{
	auto test_instance = val_str_new_from_c_str (MEM(mem_raw_new ()), "");
	ASSERT_EQ(0, val_str_count (test_instance));
    val_str_free_safe(&test_instance);
}

TEST(val_str_equal, same_pointer)
{
	auto tm = mem_test_new_default (128);

	struct val_str *test_instance = val_str_new_from_c_str (MEM(tm), "elodie");
	ASSERT_TRUE(val_str_equal (test_instance, test_instance));

    val_str_free_safe(&test_instance);
	ASSERT_TRUE(test_instance == nullptr);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_str_equal, same_val)
{
	auto tm = mem_test_new_default (128);

	struct val_str *str_one = val_str_new_from_c_str (MEM(tm), "elodie rockz");
	struct val_str *str_two = val_str_new_from_c_str (MEM(tm), "elodie rockz");
	ASSERT_TRUE(val_str_equal (str_one, str_two));

    val_str_free_safe(&str_one);
    val_str_free_safe(&str_two);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_str_equal, different_val)
{
	auto tm = mem_test_new_default (128);

	struct val_str *str_one = val_str_new_from_c_str (MEM(tm), "elodie");
	struct val_str *str_two = val_str_new_from_c_str (MEM(tm), "h4m41");
	ASSERT_FALSE(val_str_equal (str_one, str_two));

    val_str_free_safe(&str_one);
    val_str_free_safe(&str_two);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_str_equal, different_count)
{
	auto tm = mem_test_new_default (128);

	struct val_str *str_one = val_str_new_from_c_str (MEM(tm), "elodie");
	struct val_str *str_two = val_str_new_from_c_str (MEM(tm), "elodie rockz");
	ASSERT_FALSE(val_str_equal (str_one, str_two));

    val_str_free_safe(&str_one);
    val_str_free_safe(&str_two);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_str_concat, ok)
{
	auto tm = mem_test_new_default (256);

	struct val_str *str_one = val_str_new_from_c_str (MEM(tm), "Hello");
	struct val_str *str_two = val_str_new_from_c_str (MEM(tm), "World");

	struct val_str *expected = val_str_new_from_c_str (MEM(tm), "HelloWorld");
	struct val_str *result = val_str_concat (str_one, str_two, MEM(tm));
	ASSERT_TRUE(val_str_equal (result, expected));

    val_str_free_safe(&result);
    val_str_free_safe(&str_one);
    val_str_free_safe(&str_two);
    val_str_free_safe(&expected);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_str_free_safe, ok)
{
	auto tm = mem_test_new_default (128);

	struct val_str *test_instance = val_str_new_from_c_str (MEM(tm), "abc");
    val_str_free_safe(&test_instance);
	ASSERT_TRUE(test_instance == nullptr);

	mem_test_verify (tm);
	mem_test_free (tm);
}