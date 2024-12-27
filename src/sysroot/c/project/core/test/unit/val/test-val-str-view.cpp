#include "../unit-test.h"

#include "core/val/val-str.h"
#include "core/val/val-str-view.h"

void assert_base_val (struct dep_val_str_view instance)
{
	ASSERT_EQ(VAL_KIND_STR_VIEW, instance.base.kind);
	ASSERT_EQ((struct mem *)mem_null_new (), instance.base.mem);
}

TEST(dep_val_str_view_from_bytes, ok)
{
	u1 input_array[] = {'H', 'A', 'M', 'A', 'L'};
	struct bytes_view bytes = {
		.data = input_array,
		.size = 2
	};

	auto result = dep_val_str_view_from_bytes (bytes);
	ASSERT_EQ(2, dep_val_str_view_count (&result));
	ASSERT_TRUE(strncmp (result.data, "HA", result.count) == 0);

	assert_base_val (result);
}

TEST(dep_val_str_view_from_str, ok)
{
	auto tm = mem_test_new_default (128);

	struct dep_val_str *given_str = dep_val_str_allocate_from_c_str (MEM(tm), "HamaL");

	auto result = dep_val_str_view_from_str (given_str);
	ASSERT_EQ(5, dep_val_str_view_count (&result));
	ASSERT_TRUE(strncmp (result.data, "HamaL", 5) == 0);

	dep_val_str_deallocate_safe (&given_str);
	mem_test_verify (tm);
	mem_test_free (tm);

	assert_base_val (result);
}

TEST(dep_val_str_view_from_c_str, ok)
{
	auto result = dep_val_str_view_from_c_str ("HAMAL");
	ASSERT_EQ(5, dep_val_str_view_count (&result));
	ASSERT_TRUE(strncmp (result.data, "HAMAL", 5) == 0);

	assert_base_val (result);
}

TEST(dep_val_str_view_count, ok)
{
	auto test_instance = dep_val_str_view_from_c_str ("!!Hamal Rocks!!\n");
	ASSERT_EQ(16, dep_val_str_view_count (&test_instance));
}

TEST(dep_val_str_view_count, empty)
{
	auto test_instance = dep_val_str_view_from_c_str ("");
	ASSERT_EQ(0, dep_val_str_view_count (&test_instance));
}

TEST(dep_val_str_view_equal, same_val)
{
	struct dep_val_str_view val_str_view_one = dep_val_str_view_from_c_str ("hamal rockz");
	struct dep_val_str_view val_str_view_two = dep_val_str_view_from_c_str ("hamal rockz");
	ASSERT_TRUE(dep_val_str_view_equal (val_str_view_one, val_str_view_two));
}

TEST(dep_val_str_view_equal, different_val)
{
	struct dep_val_str_view val_str_view_one = dep_val_str_view_from_c_str ("hamal");
	struct dep_val_str_view val_str_view_two = dep_val_str_view_from_c_str ("h4m41");
	ASSERT_FALSE(dep_val_str_view_equal (val_str_view_one, val_str_view_two));
}

TEST(dep_val_str_view_equal, different_count)
{
	struct dep_val_str_view val_str_view_one = dep_val_str_view_from_c_str ("hamal");
	struct dep_val_str_view val_str_view_two = dep_val_str_view_from_c_str ("hamal rockz");
	ASSERT_FALSE(dep_val_str_view_equal (val_str_view_one, val_str_view_two));
}

