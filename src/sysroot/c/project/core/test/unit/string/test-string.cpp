#include "../unit-test.h"

#include "core/string/string-api.h"
#include "core/algo/algo-list-byte.h"

TEST(string_new_from_bytes, ok)
{
	auto tm = mem_test_new_default (128);

	u1 input_array[] = {'H', 'A', 'M', 'A', 'L'};
	struct bytes_view bytes = {
		.data = input_array,
		.size = 2
	};

	struct string *result = string_new_from_bytes (MEM(tm), bytes);
	ASSERT_EQ (2, result->length);
	ASSERT_TRUE (strncmp (result->data, "HA", result->length) == 0);

	string_free (result, MEM(tm));
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(string_new_from_c_str, ok)
{
	auto tm = mem_test_new_default (128);

	auto result = string_new_from_c_str (MEM(tm), "Elodie");
	ASSERT_EQ (6, string_count (*result));
	ASSERT_TRUE (strncmp (result->data, "Elodie", result->length) == 0);

	string_free_safe (&result, MEM(tm));
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(string_new_from_view, ok)
{
	auto tm = mem_test_new_default (128);

	auto result = string_new_from_view (MEM(tm), string_view_from_c_str ("ELODIE"));
	ASSERT_EQ (6, string_count (*result));
	ASSERT_TRUE (strncmp (result->data, "ELODIE", result->length) == 0);

	string_free_safe (&result, MEM(tm));
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(string_new_from_byte_list, ok)
{
	auto tm = mem_test_new_default (128);

	auto byte_list_config = byte_list_default_config (MEM(tm));
	auto byte_list = byte_list_new (byte_list_config);
	byte_list_append_u1 (byte_list, 'h');
	byte_list_append_u1 (byte_list, '4');
	byte_list_append_u1 (byte_list, 'm');
	byte_list_append_u1 (byte_list, '4');
	byte_list_append_u1 (byte_list, 'l');

	auto result = string_new_from_byte_list (MEM(tm), byte_list);
	ASSERT_EQ (5, string_count (*result));
	ASSERT_TRUE (strncmp (result->data, "h4m4l", result->length) == 0);

	byte_list_free_safe (&byte_list);
	string_free_safe (&result, MEM(tm));
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(string_init_from_bytes, ok)
{
	auto tm = mem_test_new_default (128);

	u1 input_array[] = {'H', 'A', 'M', 'A', 'L'};
	struct bytes_view bytes = {
		.data = input_array,
		.size = 2
	};

	struct string test_instance{};
	string_init_from_bytes (&test_instance, MEM(tm), bytes);
	ASSERT_EQ (2, test_instance.length);
	ASSERT_TRUE (strncmp (test_instance.data, "HA", test_instance.length) == 0);

	string_reset (&test_instance, MEM(tm));
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(string_init_from_c_str, ok)
{
	auto tm = mem_test_new_default (128);

	struct string test_instance{};
	string_init_from_c_str (&test_instance, MEM(tm), "Elodie");
	ASSERT_EQ (6, string_count (test_instance));
	ASSERT_TRUE (strncmp (test_instance.data, "Elodie", test_instance.length) == 0);

	string_reset (&test_instance, MEM(tm));
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(string_init_from_view, ok)
{
	auto tm = mem_test_new_default (128);

	struct string test_instance{};
	string_init_from_view (&test_instance, MEM(tm), string_view_from_c_str ("ELODIE"));
	ASSERT_EQ (6, string_count (test_instance));
	ASSERT_TRUE (strncmp (test_instance.data, "ELODIE", test_instance.length) == 0);

	string_reset (&test_instance, MEM(tm));
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(string_init_from_byte_list, ok)
{
	auto tm = mem_test_new_default (128);

	auto byte_list_config = byte_list_default_config (MEM(tm));
	auto byte_list = byte_list_new (byte_list_config);
	byte_list_append_u1 (byte_list, 'h');
	byte_list_append_u1 (byte_list, '4');
	byte_list_append_u1 (byte_list, 'm');
	byte_list_append_u1 (byte_list, '4');
	byte_list_append_u1 (byte_list, 'l');

	struct string test_instance{};
	string_init_from_byte_list (&test_instance, MEM(tm), byte_list);
	ASSERT_EQ (5, string_count (test_instance));
	ASSERT_TRUE (strncmp (test_instance.data, "h4m4l", test_instance.length) == 0);

	byte_list_free_safe (&byte_list);
	string_reset (&test_instance, MEM(tm));
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(string_count, ok)
{
	auto tm = mem_test_new_default (128);
	auto test_instance = string_new_from_c_str (MEM(tm), "!!Elodie Rocks!!\n");
	ASSERT_EQ (17, string_count (*test_instance));
	string_free_safe (&test_instance, MEM(tm));
}

TEST(string_count, empty)
{
	auto tm = mem_test_new_default (128);
	auto test_instance = string_new_from_c_str (MEM(tm), "");
	ASSERT_EQ (0, string_count (*test_instance));
	string_free_safe (&test_instance, MEM(tm));
}

TEST(string_equal, same_pointer)
{
	auto tm = mem_test_new_default (128);

	auto test_instance = string_new_from_c_str (MEM(tm), "elodie");
	ASSERT_TRUE (string_equal (*test_instance, *test_instance));

	string_free_safe (&test_instance, MEM(tm));
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(string_equal, same_value)
{
	auto tm = mem_test_new_default (128);

	auto str_one = string_new_from_c_str (MEM(tm), "elodie rockz");
	auto str_two = string_new_from_c_str (MEM(tm), "elodie rockz");
	ASSERT_TRUE (string_equal (*str_one, *str_two));

	string_free_safe (&str_one, MEM(tm));
	string_free_safe (&str_two, MEM(tm));

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(string_equal, string_euals_c_str)
{
	auto tm = mem_test_new_default (128);

	struct string str_one{};
	string_init_from_c_str (&str_one, MEM(tm), "elodie rockz");
	auto str_two = "elodie rockz";
	ASSERT_TRUE (string_equal_c_str (str_one, str_two));

	string_reset (&str_one, MEM(tm));

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(string_equal, different_value)
{
	auto tm = mem_test_new_default (128);

	auto str_one = string_new_from_c_str (MEM(tm), "elodie");
	auto str_two = string_new_from_c_str (MEM(tm), "h4m41");
	ASSERT_FALSE (string_equal (*str_one, *str_two));

	string_free_safe (&str_one, MEM(tm));
	string_free_safe (&str_two, MEM(tm));

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(string_equal, different_count)
{
	auto tm = mem_test_new_default (128);

	auto str_one = string_new_from_c_str (MEM(tm), "elodie");
	auto str_two = string_new_from_c_str (MEM(tm), "elodie rockz");
	ASSERT_FALSE (string_equal (*str_one, *str_two));

	string_free_safe (&str_one, MEM(tm));
	string_free_safe (&str_two, MEM(tm));

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(string_concat, ok)
{
	auto tm = mem_test_new_default (256);

	auto str_one = string_new_from_c_str (MEM(tm), "Hello");
	auto str_two = string_new_from_c_str (MEM(tm), "World");

	auto expected = string_new_from_c_str (MEM(tm), "HelloWorld");
	auto result = string_concat (*str_one, *str_two, MEM(tm));
	ASSERT_TRUE (string_equal (*result, *expected));

	string_free_safe (&result, MEM(tm));
	string_free_safe (&str_one, MEM(tm));
	string_free_safe (&str_two, MEM(tm));
	string_free_safe (&expected, MEM(tm));

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(string_free_safe, ok)
{
	auto tm = mem_test_new_default (128);

	auto test_instance = string_new_from_c_str (MEM(tm), "abc");

	string_free_safe (&test_instance, MEM(tm));
	ASSERT_TRUE (test_instance == nullptr);

	mem_test_verify (tm);
	mem_test_free (tm);
}