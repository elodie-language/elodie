#include "../unit-test.h"

#include "core/algo/algo-list-byte.h"
#include "core/val/val-str.h"
#include "core/val/val-str-view.h"

TEST(internal_val_str_allocate_from_bytes, ok)
{
	auto tm = mem_test_new_default (128);

	u1 input_array[] = {'H', 'A', 'M', 'A', 'L'};
	struct bytes_view bytes = {
		.data = input_array,
		.size = 2
	};

	struct val_str *result = internal_val_str_allocate_from_bytes (MEM(tm), bytes);
	ASSERT_EQ(2, result->count);
	ASSERT_TRUE(strncmp (result->data, "HA", result->count) == 0);

	ASSERT_EQ(VAL_KIND_STR, result->base.kind);
	ASSERT_EQ(tm->base.realm, result->base.mem_realm);

	internal_val_str_deallocate (result, MEM(tm));
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(internal_val_str_deallocate, ok)
{
	auto tm = mem_test_new_default (128);

	u1 input_array[] = {'H', 'A', 'M', 'A', 'L'};
	struct bytes_view bytes = {
		.data = input_array,
		.size = 2
	};

	struct val_str *test_instance = internal_val_str_allocate_from_bytes (MEM(tm), bytes);

	internal_val_str_deallocate (test_instance, MEM(tm));
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(dep_val_str_allocate_from_c_str, ok)
{
	auto tm = mem_test_new_default (128);

	struct dep_val_str *result = dep_val_str_allocate_from_c_str (MEM(tm), "HamaL");
	ASSERT_EQ(5, dep_val_str_count (result));
	ASSERT_TRUE(strncmp (result->data, "HamaL", result->count) == 0);

	ASSERT_EQ(VAL_KIND_STR, result->base.kind);
	ASSERT_EQ(MEM (tm), result->base.mem);

	dep_val_str_deallocate_safe (&result);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(dep_val_str_allocate_from_bytes, ok)
{
	auto tm = mem_test_new_default (128);

	u1 input_array[] = {'H', 'A', 'M', 'A', 'L'};
	struct bytes_view bytes = {
		.data = input_array,
		.size = 2
	};

	struct dep_val_str *result = dep_val_str_allocate_from_bytes (MEM(tm), bytes);
	ASSERT_EQ(2, dep_val_str_count (result));
	ASSERT_TRUE(strncmp (result->data, "HA", result->count) == 0);

	ASSERT_EQ(VAL_KIND_STR, result->base.kind);
	ASSERT_EQ(MEM (tm), result->base.mem);

	dep_val_str_deallocate_safe (&result);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(dep_val_str_allocate_from_view, ok)
{
	auto tm = mem_test_new_default (128);

	struct dep_val_str *result = dep_val_str_allocate_from_view (MEM(tm), dep_val_str_view_from_c_str ("HAMAL"));
	ASSERT_EQ(5, dep_val_str_count (result));
	ASSERT_TRUE(strncmp (result->data, "HAMAL", result->count) == 0);

	ASSERT_EQ(VAL_KIND_STR, result->base.kind);
	ASSERT_EQ(MEM (tm), result->base.mem);

	dep_val_str_deallocate_safe (&result);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(dep_val_str_allocate_from_byte_list, ok)
{
	auto tm = mem_test_new_default (128);

	auto byte_list_config = byte_list_default_config (MEM(tm));
	auto byte_list = byte_list_new (byte_list_config);
	byte_list_append_u1 (byte_list, 'h');
	byte_list_append_u1 (byte_list, '4');
	byte_list_append_u1 (byte_list, 'm');
	byte_list_append_u1 (byte_list, '4');
	byte_list_append_u1 (byte_list, 'l');

	struct dep_val_str *result = dep_val_str_allocate_from_byte_list (MEM(tm), byte_list);
	ASSERT_EQ(5, dep_val_str_count (result));
	ASSERT_TRUE(strncmp (result->data, "h4m4l", result->count) == 0);

	ASSERT_EQ(VAL_KIND_STR, result->base.kind);
	ASSERT_EQ(MEM (tm), result->base.mem);

	byte_list_free_safe (&byte_list);
	dep_val_str_deallocate_safe (&result);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(dep_val_str_copy, ok)
{
	auto tm = mem_test_new_default (128);

	struct dep_val_str *given_str = dep_val_str_allocate_from_c_str (MEM(tm), "HamaL");
	struct dep_val_str *result = dep_val_str_copy (given_str, MEM(tm));

	ASSERT_EQ(5, dep_val_str_count (result));
	ASSERT_TRUE(strncmp (result->data, "HamaL", result->count) == 0);

	ASSERT_EQ(VAL_KIND_STR, result->base.kind);
	ASSERT_EQ(MEM (tm), result->base.mem);

	dep_val_str_deallocate_safe (&given_str);
	dep_val_str_deallocate_safe (&result);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(dep_val_str_count, ok)
{
	auto test_instance = dep_val_str_allocate_from_c_str (MEM(mem_raw_new ()), "!!Hamal Rocks!!\n");
	ASSERT_EQ(16, dep_val_str_count (test_instance));
	dep_val_str_deallocate_safe (&test_instance);
}

TEST(dep_val_str_count, empty)
{
	auto test_instance = dep_val_str_allocate_from_c_str (MEM(mem_raw_new ()), "");
	ASSERT_EQ(0, dep_val_str_count (test_instance));
	dep_val_str_deallocate_safe (&test_instance);
}

TEST(dep_val_str_equal, same_pointer)
{
	auto tm = mem_test_new_default (128);

	struct dep_val_str *test_instance = dep_val_str_allocate_from_c_str (MEM(tm), "hamal");
	ASSERT_TRUE(dep_val_str_equal (test_instance, test_instance));

	dep_val_str_deallocate_safe (&test_instance);
	ASSERT_TRUE(test_instance == nullptr);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(dep_val_str_equal, same_val)
{
	auto tm = mem_test_new_default (128);

	struct dep_val_str *str_one = dep_val_str_allocate_from_c_str (MEM(tm), "hamal rockz");
	struct dep_val_str *str_two = dep_val_str_allocate_from_c_str (MEM(tm), "hamal rockz");
	ASSERT_TRUE(dep_val_str_equal (str_one, str_two));

	dep_val_str_deallocate_safe (&str_one);
	dep_val_str_deallocate_safe (&str_two);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(dep_val_str_equal, different_val)
{
	auto tm = mem_test_new_default (128);

	struct dep_val_str *str_one = dep_val_str_allocate_from_c_str (MEM(tm), "hamal");
	struct dep_val_str *str_two = dep_val_str_allocate_from_c_str (MEM(tm), "h4m41");
	ASSERT_FALSE(dep_val_str_equal (str_one, str_two));

	dep_val_str_deallocate_safe (&str_one);
	dep_val_str_deallocate_safe (&str_two);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(dep_val_str_equal, different_count)
{
	auto tm = mem_test_new_default (128);

	struct dep_val_str *str_one = dep_val_str_allocate_from_c_str (MEM(tm), "hamal");
	struct dep_val_str *str_two = dep_val_str_allocate_from_c_str (MEM(tm), "hamal rockz");
	ASSERT_FALSE(dep_val_str_equal (str_one, str_two));

	dep_val_str_deallocate_safe (&str_one);
	dep_val_str_deallocate_safe (&str_two);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(dep_val_str_concat, ok)
{
	auto tm = mem_test_new_default (256);

	struct dep_val_str *str_one = dep_val_str_allocate_from_c_str (MEM(tm), "Hello");
	struct dep_val_str *str_two = dep_val_str_allocate_from_c_str (MEM(tm), "World");

	struct dep_val_str *expected = dep_val_str_allocate_from_c_str (MEM(tm), "HelloWorld");
	struct dep_val_str *result = dep_val_str_concat (str_one, str_two, MEM(tm));
	ASSERT_TRUE(dep_val_str_equal (result, expected));

	dep_val_str_deallocate_safe (&result);
	dep_val_str_deallocate_safe (&str_one);
	dep_val_str_deallocate_safe (&str_two);
	dep_val_str_deallocate_safe (&expected);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(dep_val_str_deallocate_safe, ok)
{
	auto tm = mem_test_new_default (128);

	struct dep_val_str *test_instance = dep_val_str_allocate_from_c_str (MEM(tm), "abc");
	dep_val_str_deallocate_safe (&test_instance);
	ASSERT_TRUE(test_instance == nullptr);

	mem_test_verify (tm);
	mem_test_free (tm);
}