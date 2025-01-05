#include "../unit-test.h"
#include "core/mem/mem-gc.h"

#include "core/algo/algo-list.h"
#include "core/val/val-api.h"

TEST(mem_gc_init, ok)
{
	auto tm = mem_test_new_default (1024);

	struct mem_gc_config config{};
	config.root = MEM(tm);
	config.size = 16;

	auto result = mem_gc_new (config);

	ASSERT_TRUE(result->colors != nullptr);
	ASSERT_TRUE(result->vals != nullptr);

	ASSERT_EQ(0, list_count (result->colors));
	ASSERT_EQ(0, val_lst_count (result->vals));

	mem_gc_free_safe (&result);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(mem_gc_allocate, ok)
{
	auto tm = mem_test_new_default (1024);

	struct mem_gc_config config{};
	config.root = MEM(tm);
	config.size = 16;

	auto some_val = (struct val *) val_num_new(MEM(tm), 28);
	auto another_val = (struct val *) val_bool_new(MEM(tm), true);

	auto test_instance = mem_gc_new (config);

	mem_gc_allocate (test_instance, some_val);
	ASSERT_EQ(1, mem_gc_count (test_instance));

	u1 color = *(u1 *)list_at (test_instance->colors, 0);
	ASSERT_EQ(MEM_GC_COLOR_WHITE, color);

	auto num_val = val_lst_at_num (test_instance->vals, 0);
	ASSERT_EQ(28, num_val->data);
	ASSERT_TRUE(some_val != (struct val *)num_val);

	mem_gc_allocate (test_instance, another_val);
	ASSERT_EQ(2, mem_gc_count (test_instance));

	color = *(u1 *)list_at (test_instance->colors, 0);
	ASSERT_EQ(MEM_GC_COLOR_WHITE, color);

	auto bool_val = val_lst_at_bool (test_instance->vals, 1);
	ASSERT_EQ(true, bool_val->data);
	ASSERT_TRUE(another_val != (struct val *)bool_val);

	mem_gc_free_safe (&test_instance);
	val_free_safe (&some_val);
	val_free_safe (&another_val);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(mem_gc_free_safe, ok)
{
	auto tm = mem_test_new_default (1024);

	struct mem_gc_config config{};
	config.root = MEM(tm);
	config.size = 16;

	auto some_val = (struct val *) val_num_new(MEM(tm), 28);

	auto test_instance = mem_gc_new (config);
	mem_gc_allocate (test_instance, some_val);

	mem_gc_free_safe (&test_instance);
	ASSERT_TRUE(test_instance == nullptr);

	val_free_safe (&some_val);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(mem_gc_free_safe, empty)
{
	auto tm = mem_test_new_default (1024);

	struct mem_gc_config config{};
	config.root = MEM(tm);
	config.size = 16;

	auto test_instance = mem_gc_new (config);

	mem_gc_free_safe (&test_instance);
	ASSERT_TRUE(test_instance == nullptr);

	mem_test_verify (tm);
	mem_test_free (tm);
}