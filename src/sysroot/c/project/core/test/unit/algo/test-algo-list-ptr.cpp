#include "../unit-test.h"

#include "core/algo/algo-list-ptr.h"
#include "core/val/val-num.h"

TEST(ptr_list_default_config, ok)
{
	auto tm = mem_test_new_default (512);

	auto result = ptr_list_default_config (MEM(tm));
	ASSERT_EQ(8, result.initial_capacity);
	ASSERT_EQ(MEM (tm), result.mem);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(ptr_list_new, ok)
{
	auto tm = mem_test_new_default (512);

	auto config = ptr_list_default_config (MEM(tm));
	config.initial_capacity = 32;

	auto test_instance = ptr_list_new (config);
	ASSERT_EQ(0, ptr_list_count (test_instance));
	ASSERT_EQ(32, ptr_list_capacity (test_instance));
	ASSERT_EQ(MEM (tm), test_instance->underlying_list.mem);

	ptr_list_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(ptr_list, ok)
{
	auto tm = mem_test_new_default (1024);

	auto ptr_list_config = ptr_list_default_config (MEM(tm));
	auto test_instance = ptr_list_new (ptr_list_config);

	for (size_t idx = 0; idx < 10; idx++)
		{
			auto val = val_num_new(MEM(tm), (double) idx);
			ptr_list_append (test_instance, val);
			ASSERT_EQ(idx + 1, ptr_list_count (test_instance));
		}

	for (size_t idx = 0; idx < 10; idx++)
		{
			auto *num = (struct val_num *)ptr_list_at (test_instance, idx);
			ASSERT_EQ(idx, num->data);
			val_num_free_safe (&num);
		}

	ASSERT_EQ(10, ptr_list_count (test_instance));
	ASSERT_EQ(16, ptr_list_capacity (test_instance));

	ptr_list_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(ptr_list_replace, ok)
{
	auto tm = mem_test_new_default (512);

	auto ptr_list_config = ptr_list_default_config (MEM(tm));
	ptr_list_config.initial_capacity = 10;
	auto test_instance = ptr_list_new (ptr_list_config);

	for (size_t idx = 9; idx > 0; idx--)
		{
			auto val = val_num_new(MEM(tm), (10 - (double) idx));
			ptr_list_replace (test_instance, idx, val);
			// technically no item gets added
			ASSERT_EQ(0, ptr_list_count (test_instance));
		}

	for (size_t idx = 1; idx < 10; idx++)
		{
			auto *num = (struct val_num *)ptr_list_at (test_instance, idx);
			ASSERT_EQ(10 - idx, num->data);
			val_num_free_safe (&num);
		}

	// technically no item was added
	ASSERT_EQ(0, ptr_list_count (test_instance));
	ASSERT_EQ(10, ptr_list_capacity (test_instance));

	ptr_list_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(ptr_list_iterator, ok)
{
	auto tm = mem_test_new_default (512);

	auto ptr_list_config = ptr_list_default_config (MEM(tm));
	auto test_instance = ptr_list_new (ptr_list_config);

	for (size_t idx = 0; idx < 10; idx++)
		{
			auto val = val_num_new(MEM(tm), (double) idx);
			ptr_list_append (test_instance, val);
			ASSERT_EQ(idx + 1, ptr_list_count (test_instance));
		}

	auto it = ptr_list_iterator (test_instance);

	for (size_t idx = 0; idx < 10; idx++)
		{
			ASSERT_TRUE(iterator_has_next (&it));
			auto *num = (struct val_num *)iterator_next (&it);
			ASSERT_EQ(idx, num->data);
			val_num_free_safe (&num);
		}
	ASSERT_FALSE(iterator_has_next (&it));

	ASSERT_EQ(10, ptr_list_count (test_instance));
	ASSERT_EQ(16, ptr_list_capacity (test_instance));

	ptr_list_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(ptr_list_free_safe, ok)
{
	auto tm = mem_test_new_default (512);

	auto config = ptr_list_default_config (MEM(tm));

	auto test_instance = ptr_list_new (config);

	ptr_list_free_safe (&test_instance);
	ASSERT_TRUE(test_instance == nullptr);

	mem_test_verify (tm);
	mem_test_free (tm);
}
