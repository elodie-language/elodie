#include "../unit-test.h"

#include "core/val/val-clsr.h"
#include "core/val/val-fld.h"
#include "core/val/val-lst.h"
#include "core/val/val-mod.h"
#include "core/val/val-obj.h"
#include "core/val/val-str.h"

TEST(val_mod_const_pool_init, ok)
{
	auto tm = mem_test_new_default (1024);

	struct val_mod_const_pool test_instance{};
	val_mod_const_pool_init (&test_instance, MEM(tm));

	ASSERT_EQ(MEM (tm), test_instance.mem);

	ASSERT_EQ(0, val_lst_count (test_instance.strs));
	ASSERT_EQ(8, val_lst_capacity (test_instance.strs));

	ASSERT_EQ(0, val_lst_count (test_instance.fields));
	ASSERT_EQ(8, val_lst_capacity (test_instance.fields));

	ASSERT_EQ(0, val_lst_count (test_instance.objs));
	ASSERT_EQ(8, val_lst_capacity (test_instance.objs));

	val_mod_const_pool_reset (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_mod_const_pool_append_str, ok)
{
	auto tm = mem_test_new_default (2048);

	struct val_mod_const_pool test_instance{};
	val_mod_const_pool_init (&test_instance, MEM(tm));

	for (size_t idx = 0; idx < 10; idx++)
		{
			val_mod_const_pool_append_str (&test_instance, dep_val_str_view_from_c_str ("HA_M_AL"));
			ASSERT_EQ(idx + 1, val_lst_count (test_instance.strs));
		}
	ASSERT_EQ(16, val_lst_capacity (test_instance.strs));

	val_mod_const_pool_reset (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_mod_const_pool_append_field, ok)
{
	auto tm = mem_test_new_default (4096);

	struct val_mod_const_pool test_instance{};
	val_mod_const_pool_init (&test_instance, MEM(tm));

	for (size_t idx = 0; idx < 10; idx++)
		{
			val_mod_const_pool_append_field (&test_instance, val_fld_new (MEM(tm), dep_val_str_view_from_c_str ("field"), type_any));
			ASSERT_EQ (idx + 1, val_lst_count (test_instance.fields));
		}
	ASSERT_EQ(16, val_lst_capacity (test_instance.fields));

	val_mod_const_pool_reset (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_mod_const_pool_append_obj, ok)
{
	auto tm = mem_test_new_default (4096);

	struct val_mod_const_pool test_instance{};
	val_mod_const_pool_init (&test_instance, MEM(tm));

	for (size_t idx = 0; idx < 10; idx++)
		{
			val_mod_const_pool_append_obj (&test_instance, val_obj_new (MEM(tm), dep_val_str_view_from_c_str ("some_obj")));
			ASSERT_EQ(idx + 1, val_lst_count (test_instance.objs));
		}
	ASSERT_EQ(16, val_lst_capacity (test_instance.objs));

	val_mod_const_pool_reset (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_mod_const_pool_reset, ok)
{
	auto tm = mem_test_new_default (2048);

	struct val_mod_const_pool test_instance{};
	val_mod_const_pool_init (&test_instance, MEM(tm));

	val_mod_const_pool_append_str (&test_instance, dep_val_str_view_from_c_str ("HA_M_AL"));
	val_mod_const_pool_append_field (&test_instance, val_fld_new (MEM(tm), dep_val_str_view_from_c_str ("field"), type_any));
	val_mod_const_pool_append_obj (&test_instance, val_obj_new (MEM(tm), dep_val_str_view_from_c_str ("some_obj")));

	val_mod_const_pool_reset (&test_instance);

	ASSERT_TRUE(test_instance.strs == nullptr);
	ASSERT_TRUE(test_instance.fields == nullptr);
	ASSERT_TRUE(test_instance.objs == nullptr);
	ASSERT_TRUE(test_instance.mem == nullptr);

	mem_test_verify (tm);
	mem_test_free (tm);
}