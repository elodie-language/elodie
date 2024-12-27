#include "../unit-test.h"

#include "core/val/val-clsr.h"
#include "core/val/val-fn.h"
#include "core/val/val-str.h"

TEST(val_clsr_new, ok)
{
	auto tm = mem_test_new_default (1024);

	struct val_fn *some_fn = val_fn_new (MEM(tm), dep_val_str_view_from_c_str ("some_fn"));
	struct val_clsr *test_instance = val_clsr_new (MEM(tm), some_fn);

	ASSERT_EQ(VAL_KIND_CLSR, test_instance->base.kind);
	ASSERT_EQ(MEM (tm), test_instance->base.mem);
	ASSERT_EQ(some_fn, test_instance->fn);

	val_clsr_clear (test_instance);
	val_clsr_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_clsr_to_str, ok)
{
	auto tm = mem_test_new_default (1024);

	struct val_fn *some_fn = val_fn_new (MEM(tm), dep_val_str_view_from_c_str ("some_fn"));
	struct val_clsr *test_instance = val_clsr_new (MEM(tm), some_fn);

	struct dep_val_str *result = val_clsr_to_str (test_instance, MEM(tm));
	struct dep_val_str *expected = dep_val_str_allocate_from_c_str (MEM(tm), "some_fn");
	ASSERT_TRUE(dep_val_str_equal (expected, result));

	dep_val_str_deallocate_safe (&expected);
	dep_val_str_deallocate_safe (&result);

	val_clsr_clear (test_instance);
	val_clsr_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_clsr_free_safe, ok)
{
	auto tm = mem_test_new_default (1024);

	struct val_fn *some_fn = val_fn_new (MEM(tm), dep_val_str_view_from_c_str ("fn"));
	struct val_clsr *test_instance = val_clsr_new (MEM(tm), some_fn);

	val_clsr_clear (test_instance);
	val_clsr_free_safe (&test_instance);
	ASSERT_TRUE(test_instance == nullptr);

	mem_test_verify (tm);
	mem_test_free (tm);
}