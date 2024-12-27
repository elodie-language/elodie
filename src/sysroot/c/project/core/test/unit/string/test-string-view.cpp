#include "../unit-test.h"

#include "core/string/string.h"
#include "core/string/string-view.h"

TEST(string_view_from_bytes, ok)
{
	u1 input_array[] = {'H', 'A', 'M', 'A', 'L'};
	struct bytes_view bytes = {
		.data = input_array,
		.size = 2
	};

	auto result = string_view_from_bytes (bytes);
	ASSERT_EQ(2, string_view_count (result));
	ASSERT_TRUE(strncmp (result.data, "HA", result.count) == 0);
}

TEST(string_view_from_str_ptr, ok)
{
	auto tm = mem_test_new_default (128);

	auto given_str = string_allocate_from_c_str (MEM(tm), "HamaL");

	auto result = string_view_from_str_ptr (given_str);
	ASSERT_EQ(5, string_view_count (result));
	ASSERT_TRUE(strncmp (result.data, "HamaL", 5) == 0);

	string_deallocate_safe (&given_str, MEM(tm));
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(string_view_from_c_str, ok)
{
	auto result = string_view_from_c_str ("HAMAL");
	ASSERT_EQ(5, string_view_count (result));
	ASSERT_TRUE(strncmp (result.data, "HAMAL", 5) == 0);
}

TEST(string_view_count, ok)
{
	auto test_instance = string_view_from_c_str ("!!Hamal Rocks!!\n");
	ASSERT_EQ(16, string_view_count (test_instance));
}

TEST(string_view_count, empty)
{
	auto test_instance = string_view_from_c_str ("");
	ASSERT_EQ(0, string_view_count (test_instance));
}

TEST(string_view_equal, same_val)
{
	struct string_view str_view_one = string_view_from_c_str ("hamal rockz");
	struct string_view str_view_two = string_view_from_c_str ("hamal rockz");
	ASSERT_TRUE(string_view_equal (str_view_one, str_view_two));
}

TEST(string_view_equal, different_val)
{
	struct string_view str_view_one = string_view_from_c_str ("hamal");
	struct string_view str_view_two = string_view_from_c_str ("h4m41");
	ASSERT_FALSE(string_view_equal (str_view_one, str_view_two));
}

TEST(string_view_equal, different_count)
{
	struct string_view str_view_one = string_view_from_c_str ("hamal");
	struct string_view str_view_two = string_view_from_c_str ("hamal rockz");
	ASSERT_FALSE(string_view_equal (str_view_one, str_view_two));
}

TEST(string_view_last_occurrence_of, no_occurrence)
{
	u4 position = 0;
	ASSERT_FALSE(string_view_last_occurrence_of (STRING_VIEW ("some::hamal"), STRING_VIEW ("@@"), &position));
	ASSERT_EQ(0, position);
}

TEST(string_view_last_occurrence_of, pattern_longer_than_string)
{
	u4 position = 0;
	ASSERT_FALSE(string_view_last_occurrence_of (STRING_VIEW ("::"), STRING_VIEW (":::"), &position));
	ASSERT_EQ(0, position);
}

TEST(string_view_last_occurrence_of, no_occurrence_but_close)
{
	u4 position = 0;
	ASSERT_FALSE(string_view_last_occurrence_of (STRING_VIEW ("some::hamal"), STRING_VIEW (":::"), &position));
	ASSERT_EQ(0, position);
}

TEST(string_view_last_occurrence_of, one_occurrence)
{
	u4 position = 0;
	ASSERT_TRUE(string_view_last_occurrence_of (STRING_VIEW ("some::hamal"), STRING_VIEW ("::"), &position));
	ASSERT_EQ(4, position);
}

TEST(string_view_last_occurrence_of, multiple_occurrences)
{
	u4 position = 0;
	ASSERT_TRUE(string_view_last_occurrence_of (STRING_VIEW ("some::package::fn::hamal"), STRING_VIEW ("::"), &position));
	ASSERT_EQ(17, position);
}

TEST(string_view_last_occurrence_of, same_content)
{
	u4 position = 0;
	ASSERT_TRUE(string_view_last_occurrence_of (STRING_VIEW ("hamal"), STRING_VIEW ("hamal"), &position));
	ASSERT_EQ(0, position);
}