#include "../unit-test.h"

#include "core/val/val-str.h"
#include "core/val/val-str-view.h"

void assert_base_val (struct val_str_view instance)
{
	ASSERT_EQ(VAL_KIND_STR_VIEW, instance.base.kind);
	ASSERT_EQ((struct mem *)mem_null_new (), instance.base.mem);
}

TEST(val_str_view_from_bytes, ok)
{
	u1 input_array[] = {'H', 'A', 'M', 'A', 'L'};
	struct bytes_view bytes = {
		.data = input_array,
		.size = 2
	};

	auto result = val_str_view_from_bytes (bytes);
	ASSERT_EQ(2, val_str_view_count (&result));
	ASSERT_TRUE(strncmp (result.data, "HA", result.count) == 0);

	assert_base_val (result);
}

TEST(val_str_view_from_str, ok)
{
	auto tm = mem_test_new_default (128);

	struct val_str *given_str = val_str_allocate_from_c_str (MEM(tm), "Elodie");

	auto result = val_str_view_from_str (given_str);
	ASSERT_EQ(6, val_str_view_count (&result));
	ASSERT_TRUE(strncmp (result.data, "Elodie", 6) == 0);

	val_str_deallocate_safe (&given_str);
	mem_test_verify (tm);
	mem_test_free (tm);

	assert_base_val (result);
}

TEST(val_str_view_from_c_str, ok)
{
	auto result = val_str_view_from_c_str ("ELODIE");
	ASSERT_EQ(6, val_str_view_count (&result));
	ASSERT_TRUE(strncmp (result.data, "ELODIE", 6) == 0);

	assert_base_val (result);
}

TEST(val_str_view_count, ok)
{
	auto test_instance = val_str_view_from_c_str ("!!Elodie Rocks!!\n");
	ASSERT_EQ(17, val_str_view_count (&test_instance));
}

TEST(val_str_view_count, empty)
{
	auto test_instance = val_str_view_from_c_str ("");
	ASSERT_EQ(0, val_str_view_count (&test_instance));
}

TEST(val_str_view_equal, same_val)
{
	struct val_str_view val_str_view_one = val_str_view_from_c_str ("elodie rockz");
	struct val_str_view val_str_view_two = val_str_view_from_c_str ("elodie rockz");
	ASSERT_TRUE(val_str_view_equal (val_str_view_one, val_str_view_two));
}

TEST(val_str_view_equal, different_val)
{
	struct val_str_view val_str_view_one = val_str_view_from_c_str ("elodie");
	struct val_str_view val_str_view_two = val_str_view_from_c_str ("h4m41");
	ASSERT_FALSE(val_str_view_equal (val_str_view_one, val_str_view_two));
}

TEST(val_str_view_equal, different_count)
{
	struct val_str_view val_str_view_one = val_str_view_from_c_str ("elodie");
	struct val_str_view val_str_view_two = val_str_view_from_c_str ("elodie rockz");
	ASSERT_FALSE(val_str_view_equal (val_str_view_one, val_str_view_two));
}

