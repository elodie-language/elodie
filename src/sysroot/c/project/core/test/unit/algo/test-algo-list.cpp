#include "../unit-test.h"

#include "core/mem/mem.h"
#include "core/algo/algo-list.h"

TEST(list_default_config, ok)
{
	auto tm = mem_test_new_default (512);

	struct list_config result = list_default_config (MEM(tm));
	ASSERT_EQ(2, result.resize_factor);
	ASSERT_EQ(8, result.initial_capacity);
	ASSERT_EQ(MEM (tm), result.mem);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(list_new, ok)
{
	auto tm = mem_test_new_default (512);

	auto config = list_default_config (MEM(tm));
	config.initial_capacity = 32;

	auto test_instance = list_new(config, size_t);
	ASSERT_EQ(0, list_count (test_instance));
	ASSERT_EQ(32, list_capacity (test_instance));

	list_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(list, ok)
{
	auto tm = mem_test_new_default (512);

	auto list_config = list_default_config (MEM(tm));
	list_config.mem = MEM(tm);

	struct magic {
	  u1 val;
	  u2 biggerVal;
	  u4 evenBiggerVal;
	  u8 bigboy;
	};

	auto test_instance = list_new(list_config, struct magic);
	for (size_t idx = 0; idx < 10; idx++)
		{
			struct magic val{
				.val = static_cast<u1>(idx),
				.biggerVal = static_cast<u2>(idx * idx),
				.evenBiggerVal = static_cast<u4>(idx * idx * idx),
				.bigboy = static_cast<u8>(idx * idx * idx * idx),
			};
			list_append (test_instance, &val);
		}

	for (size_t idx = 0; idx < 10; idx++)
		{
			struct magic d = *(struct magic *)list_at (test_instance, idx);

			ASSERT_EQ(idx, d.val);
			ASSERT_EQ(idx * idx, d.biggerVal);
			ASSERT_EQ(idx * idx * idx, d.evenBiggerVal);
			ASSERT_EQ(idx * idx * idx * idx, d.bigboy);
		}

	ASSERT_EQ(10, list_count (test_instance));
	ASSERT_EQ(16, list_capacity (test_instance));

	// 64 byte - list
	// 256 byte - list data for 16 elements
	ASSERT_EQ(296, mem_test_size (tm));

	list_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(list_ensure_capacity, ok)
{
	auto tm = mem_test_new_default (1024);
	auto list_config = list_default_config (MEM(tm));

	list_config.mem = MEM(tm);
	list_config.initial_capacity = 8;

	auto test_instance = list_new(list_config, int64_t);
	ASSERT_EQ(8, list_capacity (test_instance));

	list_ensure_capacity (test_instance, 50);
	ASSERT_EQ(64, list_capacity (test_instance));

	ASSERT_EQ(552, mem_test_size (tm));

	list_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(list_at, ok)
{
	auto tm = mem_test_new_default (2048);
	auto list_config = list_default_config (MEM(tm));

	auto test_instance = list_new(list_config, size_t);
	for (size_t idx = 0; idx < 100; idx++)
		{
			list_append_rval(test_instance, idx);
		}

	for (size_t idx = 0; idx < 100; idx++)
		{
			size_t val = *(size_t *)list_at (test_instance, idx);
			ASSERT_EQ(idx, val);
		}

	list_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(list_copy_into, ok)
{
	auto tm = mem_test_new_default (4096);
	auto list_config = list_default_config (MEM(tm));

	auto test_instance = list_new(list_config, size_t);
	auto target = list_new(list_config, size_t);
	for (size_t idx = 0; idx < 100; idx++)
		{
			list_append_rval(test_instance, idx);
		}
	ASSERT_NE(test_instance, target);
	ASSERT_EQ(1168, mem_test_size (tm));

	list_copy_into (test_instance, target);
	ASSERT_EQ(2128, mem_test_size (tm));

	for (size_t idx = 0; idx < 100; idx++)
		{
			size_t test_instance_val = *(size_t *)list_at (test_instance, idx);
			size_t target_val = *(size_t *)list_at (target, idx);
			ASSERT_EQ(test_instance_val, target_val);
		}

	ASSERT_EQ(list_count (test_instance), list_count (target));
	ASSERT_EQ(list_capacity (test_instance), list_capacity (target));

	list_free_safe (&test_instance);
	list_free_safe (&target);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(list_iterator, ok)
{
	auto tm = mem_test_new_default (2048);
	auto list_config = list_default_config (MEM(tm));

	auto test_instance = list_new(list_config, size_t);
	auto it = list_iterator (test_instance);

	for (size_t idx = 0; idx < 100; idx++)
		{
			list_append_rval(test_instance, idx);
		}

	for (size_t idx = 0; idx < 100; idx++)
		{
			ASSERT_TRUE(iterator_has_next (&it));
			ASSERT_EQ(idx, *(size_t *)iterator_next (&it));
		}
	ASSERT_FALSE(iterator_has_next (&it));

	list_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(list_free_safe, ok)
{

	auto tm = mem_test_new_default (1024);
	auto list_config = list_default_config (MEM(tm));

	auto test_instance = list_new(list_config, u1);

	for (size_t idx = 0; idx < 10; idx++)
		{
			list_append_rval(test_instance, idx);
		}

	list_free_safe (&test_instance);
	ASSERT_TRUE(test_instance == nullptr);

	mem_test_verify (tm);
	mem_test_free (tm);
}