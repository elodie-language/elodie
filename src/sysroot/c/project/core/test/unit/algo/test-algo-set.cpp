#include "../unit-test.h"

#include "core/algo/algo-set.h"

TEST(set_new, ok)
{
	auto tm = mem_test_new_default (1024);

	struct set_config config = {
		.mem = MEM(tm),
		.initial_capacity = 8,
		.hash_fn = hash_fn_murmur_3 (0)
	};

	struct set *result = set_new (config);
	ASSERT_TRUE(result != nullptr);
	ASSERT_EQ(0, set_count (result));
	ASSERT_EQ(8, set_capacity (result));

	set_free_safe (&result);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(set_set, ok)
{
	auto tm = mem_test_new_default (1024);

	struct set_config config = {
		.mem = MEM(tm),
		.initial_capacity = 8,
		.hash_fn = hash_fn_murmur_3 (0)
	};

	auto test_instance = set_new (config);

	u2 value = 28;
	ASSERT_TRUE(set_set (test_instance, bytes_view_of_u2 (&value)));
	ASSERT_EQ(1, set_count (test_instance));
	ASSERT_EQ(8, set_capacity (test_instance));

	set_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(set_set, twice)
{
	auto tm = mem_test_new_default (1024);

	struct set_config config = {
		.mem = MEM(tm),
		.initial_capacity = 8,
		.hash_fn = hash_fn_murmur_3 (0)
	};

	u2 value = 9;
	auto test_instance = set_new (config);
	set_set (test_instance, bytes_view_of_u2 (&value));

	ASSERT_FALSE(set_set (test_instance, bytes_view_of_u2 (&value)));
	ASSERT_EQ(1, set_count (test_instance));
	ASSERT_EQ(8, set_capacity (test_instance));

	set_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(set_set, resize)
{
	auto tm = mem_test_new_default (1024);

	struct set_config config = {
		.mem = MEM(tm),
		.initial_capacity = 8,
		.hash_fn = hash_fn_murmur_3 (0)
	};

	auto test_instance = set_new (config);

	for (size_t idx = 0; idx < 10; idx++)
		{
			u2 value = idx;
			ASSERT_TRUE(set_set (test_instance, bytes_view_of_u2 (&value)));
		}

	ASSERT_EQ(16, set_capacity (test_instance));

	set_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(set_has, ok)
{

	auto tm = mem_test_new_default (1024);

	struct set_config config = {
		.mem = MEM(tm),
		.initial_capacity = 8,
		.hash_fn = hash_fn_murmur_3 (0)
	};

	auto test_instance = set_new (config);

	u2 value = 23;
	u2 other_value = 45;
	set_set (test_instance, bytes_view_of_u2 (&value));

	ASSERT_TRUE(set_has (test_instance, bytes_view_of_u2 (&value)));
	ASSERT_FALSE(set_has (test_instance, bytes_view_of_u2 (&other_value)));

	set_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);

}

TEST(set_free_safe, ok)
{
	auto tm = mem_test_new_default (1024);

	struct set_config config = {
		.mem = MEM(tm),
		.initial_capacity = 8,
		.hash_fn = hash_fn_murmur_3 (0)
	};

	struct set *test_instance = set_new (config);
	set_free_safe (&test_instance);
	ASSERT_TRUE(test_instance == nullptr);
	mem_test_verify (tm);
	mem_test_free (tm);
}