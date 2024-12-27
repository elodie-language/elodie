#include "../unit-test.h"

#include "core/val/val-clsr.h"
#include "core/val/val-fn.h"
#include "core/val/val-lst.h"
#include "core/val/val-mod.h"
#include "core/val/val-str.h"

TEST(val_mod_new, ok)
{
	auto tm = mem_test_new_default (4096);

	auto init_clsr = val_clsr_new (
		MEM(tm),
		val_fn_new (
			MEM(tm),
			dep_val_str_view_from_c_str ("init")
		)
	);

	auto test_instance = val_mod_new (
		MEM(tm),
		dep_val_str_view_from_c_str ("some_mod"),
		init_clsr
	);

	ASSERT_TRUE (
		dep_val_str_view_equal (
			dep_val_str_view_from_c_str ("some_mod"),
			dep_val_str_view_from_str (test_instance->ident)
		)
	);

	ASSERT_EQ (false, test_instance->initialized);
	ASSERT_EQ (init_clsr, test_instance->init);


	// const pool initialized
	ASSERT_EQ (MEM (tm), test_instance->const_pool.mem);
	ASSERT_EQ (8, val_lst_capacity (test_instance->const_pool.strs));

	val_mod_clear (test_instance);
	val_mod_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_mod_register_clsr, ok)
{
	auto tm = mem_test_new_default (4096);

	auto test_instance = val_mod_new (MEM(tm), dep_val_str_view_from_c_str ("some_mod"), nullptr);

	auto some_clsr = val_clsr_new (
		MEM(tm),
		val_fn_new (
			MEM(tm),
			dep_val_str_view_from_c_str ("some_fn")
		)
	);

	ASSERT_TRUE (val_mod_register_clsr (test_instance, some_clsr));
	ASSERT_EQ (1, val_lst_count (test_instance->clsrs));

	val_mod_clear (test_instance);
	val_mod_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_mod_register_clsr, twice)
{
	auto tm = mem_test_new_default (2048);

	auto test_instance = val_mod_new (MEM(tm), dep_val_str_view_from_c_str ("some_mod"), nullptr);

	auto some_clsr = val_clsr_new (
		MEM(tm),
		val_fn_new (
			MEM(tm),
			dep_val_str_view_from_c_str ("some_fn")
		)
	);

	ASSERT_TRUE (val_mod_register_clsr (test_instance, some_clsr));
	ASSERT_EQ (1, val_lst_count (test_instance->clsrs));

	auto another_clsr_with_same_fn_ident = val_clsr_new (
		MEM(tm),
		val_fn_new (
			MEM(tm),
			dep_val_str_view_from_c_str ("some_fn")
		)
	);

	ASSERT_FALSE (val_mod_register_clsr (test_instance, another_clsr_with_same_fn_ident));
	ASSERT_EQ (1, val_lst_count (test_instance->clsrs));

	val_clsr_clear (another_clsr_with_same_fn_ident);
	val_clsr_free (another_clsr_with_same_fn_ident);

	val_mod_clear (test_instance);
	val_mod_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_mod_resolve_clsr_id, ok)
{
	auto tm = mem_test_new_default (4096);

	auto test_instance = val_mod_new (MEM(tm), dep_val_str_view_from_c_str ("some_mod"), nullptr);

	auto some_clsr = val_clsr_new (
		MEM(tm),
		val_fn_new (
			MEM(tm),
			dep_val_str_view_from_c_str ("some_fn")
		)
	);

	val_mod_register_clsr (test_instance, some_clsr);

	u2 result = 23;
	ASSERT_TRUE (val_mod_resolve_clsr_id (test_instance, dep_val_str_view_from_c_str ("some_fn"), &result));
	ASSERT_EQ (0, result);

	val_mod_clear (test_instance);
	val_mod_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_mod_resolve_clsr_id, not_found)
{
	auto tm = mem_test_new_default (4096);

	auto test_instance = val_mod_new (MEM(tm), dep_val_str_view_from_c_str ("some_mod"), nullptr);

	auto some_clsr = val_clsr_new (
		MEM(tm),
		val_fn_new (
			MEM(tm),
			dep_val_str_view_from_c_str ("some_fn")
		)
	);

	val_mod_register_clsr (test_instance, some_clsr);

	u2 result = 0;
	ASSERT_FALSE (val_mod_resolve_clsr_id (test_instance, dep_val_str_view_from_c_str ("another_fn"), &result));

	val_mod_clear (test_instance);
	val_mod_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_mod_free_safe, ok)
{
	auto tm = mem_test_new_default (4096);

	auto test_instance = val_mod_new (
		MEM(tm),
		dep_val_str_view_from_c_str ("some_mod"),
		val_clsr_new (
			MEM(tm),
			val_fn_new (
				MEM(tm),
				dep_val_str_view_from_c_str ("init")
			)
		)
	);

	val_mod_clear (test_instance);
	val_mod_free_safe (&test_instance);
	ASSERT_TRUE (test_instance == nullptr);

	mem_test_verify (tm);
	mem_test_free (tm);
}