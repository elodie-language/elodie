#include "../unit-test.h"
#include "core/algo/algo-list.h"
#include "core/type/type-api.h"
#include "core/val/val-api.h"

TEST(val_lst_new, ok)
{
	auto tm = mem_test_new_default (1024);

	auto test_instance = val_lst_new (MEM(tm));
	ASSERT_EQ(VAL_KIND_LST, test_instance->base.kind);
	ASSERT_EQ(MEM (tm), test_instance->base.mem);

	ASSERT_EQ(0, ptr_list_count (test_instance->underlying_list));
	ASSERT_EQ(8, ptr_list_capacity (test_instance->underlying_list));

	val_lst_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_lst_append, bool)
{
	auto tm = mem_test_new_default (1024);

	auto val = val_bool_new(MEM(tm), true);
	auto test_instance = val_lst_new (MEM(tm));

	val_lst_append (test_instance, val);
	ASSERT_EQ(1, val_lst_count (test_instance));
	ASSERT_EQ(AS_VAL (val), val_lst_at_base (test_instance, 0));

	val_lst_clear (test_instance);
	val_lst_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_lst_append, field)
{
	auto tm = mem_test_new_default (1024);

	auto val = val_fld_new (MEM(tm), val_str_view_from_c_str ("some_field"), type_any);
	auto test_instance = val_lst_new (MEM(tm));

	val_lst_append (test_instance, val);
	ASSERT_EQ(1, val_lst_count (test_instance));
	ASSERT_EQ(AS_VAL (val), val_lst_at_base (test_instance, 0));

	val_lst_clear (test_instance);
	val_lst_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_lst_append, fn)
{
	auto tm = mem_test_new_default (1024);

	auto val = val_fn_new (MEM(tm), val_str_view_from_c_str ("fn"));
	auto test_instance = val_lst_new (MEM(tm));

	val_lst_append(test_instance, val);
	ASSERT_EQ(1, val_lst_count (test_instance));
	ASSERT_EQ(AS_VAL (val), val_lst_at_base (test_instance, 0));

	val_lst_clear (test_instance);
	val_lst_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_lst_append, list)
{
	auto tm = mem_test_new_default (1024);

	auto val = val_lst_new (MEM(tm));
	auto test_instance = val_lst_new (MEM(tm));

	val_lst_append (test_instance, val);
	ASSERT_EQ(1, val_lst_count (test_instance));
	ASSERT_EQ(AS_VAL (val), val_lst_at_base (test_instance, 0));

	val_lst_clear (test_instance);
	val_lst_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_lst_append, num)
{
	auto tm = mem_test_new_default (1024);

	auto val = val_num_new_from_double (MEM(tm), 1337);
	auto test_instance = val_lst_new (MEM(tm));

	val_lst_append (test_instance, val);
	ASSERT_EQ(1, val_lst_count (test_instance));
	ASSERT_EQ(AS_VAL (val), val_lst_at_base (test_instance, 0));

	val_lst_clear (test_instance);
	val_lst_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_lst_append, obj)
{
	auto tm = mem_test_new_default (1024);

	auto val = val_obj_new (MEM(tm), val_str_view_from_c_str ("some_obj"));
	auto test_instance = val_lst_new (MEM(tm));

	val_lst_append (test_instance, val);
	ASSERT_EQ(1, val_lst_count (test_instance));
	ASSERT_EQ(AS_VAL (val), val_lst_at_base (test_instance, 0));

	val_lst_clear (test_instance);
	val_lst_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_lst_append, str)
{
	auto tm = mem_test_new_default (1024);

	auto val = val_str_new_from_c_str (MEM(tm), "H4M41");
	auto test_instance = val_lst_new (MEM(tm));

	val_lst_append (test_instance, val);
	ASSERT_EQ(1, val_lst_count (test_instance));
	ASSERT_EQ(AS_VAL (val), val_lst_at_base (test_instance, 0));

	val_lst_clear (test_instance);
	val_lst_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_lst_replace_base, bool)
{
	auto tm = mem_test_new_default (1024);

	auto existing_val_one = val_str_new_from_c_str (MEM(tm), "existing val one");
	auto existing_val_two = val_str_new_from_c_str (MEM(tm), "existing val two");
	auto existing_val_three = val_str_new_from_c_str (MEM(tm), "existing val three");

	auto test_instance = val_lst_new (MEM(tm));
	val_lst_append_str (test_instance, existing_val_one);
	val_lst_append_str (test_instance, existing_val_two);
	val_lst_append_str (test_instance, existing_val_three);

	auto replacement = (struct val *) val_bool_new(MEM(tm), true);
	val_lst_replace_base (test_instance, 1, replacement);

	ASSERT_EQ(3, val_lst_count (test_instance));
	ASSERT_EQ((struct val *)existing_val_one, val_lst_at_base (test_instance, 0));
	ASSERT_EQ((struct val *)replacement, val_lst_at_base (test_instance, 1));
	ASSERT_EQ((struct val *)existing_val_three, val_lst_at_base (test_instance, 2));

    val_str_free_safe(&existing_val_two);
	val_lst_clear (test_instance);
	val_lst_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_lst_replace_base, fld)
{
	auto tm = mem_test_new_default (1024);

	auto existing_val_one = val_str_new_from_c_str (MEM(tm), "existing val one");
	auto existing_val_two = val_str_new_from_c_str (MEM(tm), "existing val two");
	auto existing_val_three = val_str_new_from_c_str (MEM(tm), "existing val three");

	auto test_instance = val_lst_new (MEM(tm));
	val_lst_append_str (test_instance, existing_val_one);
	val_lst_append_str (test_instance, existing_val_two);
	val_lst_append_str (test_instance, existing_val_three);

	auto replacement = (struct val *)val_fld_new (MEM(tm), val_str_view_from_c_str ("field"), type_any);
	val_lst_replace_base (test_instance, 1, replacement);

	ASSERT_EQ(3, val_lst_count (test_instance));
	ASSERT_EQ((struct val *)existing_val_one, val_lst_at_base (test_instance, 0));
	ASSERT_EQ((struct val *)replacement, val_lst_at_base (test_instance, 1));
	ASSERT_EQ((struct val *)existing_val_three, val_lst_at_base (test_instance, 2));

    val_str_free_safe(&existing_val_two);
	val_lst_clear (test_instance);
	val_lst_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_lst_replace_base, fn)
{
	auto tm = mem_test_new_default (1024);

	auto existing_val_one = val_str_new_from_c_str (MEM(tm), "existing val one");
	auto existing_val_two = val_str_new_from_c_str (MEM(tm), "existing val two");
	auto existing_val_three = val_str_new_from_c_str (MEM(tm), "existing val three");

	auto test_instance = val_lst_new (MEM(tm));
	val_lst_append_str (test_instance, existing_val_one);
	val_lst_append_str (test_instance, existing_val_two);
	val_lst_append_str (test_instance, existing_val_three);

	auto replacement = (struct val *)val_fn_new (MEM(tm), val_str_view_from_c_str ("fn"));
	val_lst_replace_base (test_instance, 1, replacement);

	ASSERT_EQ(3, val_lst_count (test_instance));
	ASSERT_EQ((struct val *)existing_val_one, val_lst_at_base (test_instance, 0));
	ASSERT_EQ((struct val *)replacement, val_lst_at_base (test_instance, 1));
	ASSERT_EQ((struct val *)existing_val_three, val_lst_at_base (test_instance, 2));

    val_str_free_safe(&existing_val_two);
	val_lst_clear (test_instance);
	val_lst_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_lst_replace_base, lst)
{
	auto tm = mem_test_new_default (1024);

	auto existing_val_one = val_str_new_from_c_str (MEM(tm), "existing val one");
	auto existing_val_two = val_str_new_from_c_str (MEM(tm), "existing val two");
	auto existing_val_three = val_str_new_from_c_str (MEM(tm), "existing val three");

	auto test_instance = val_lst_new (MEM(tm));
	val_lst_append_str (test_instance, existing_val_one);
	val_lst_append_str (test_instance, existing_val_two);
	val_lst_append_str (test_instance, existing_val_three);

	auto replacement = (struct val *)val_lst_new (MEM(tm));
	val_lst_replace_base (test_instance, 1, replacement);

	ASSERT_EQ(3, val_lst_count (test_instance));
	ASSERT_EQ((struct val *)existing_val_one, val_lst_at_base (test_instance, 0));
	ASSERT_EQ((struct val *)replacement, val_lst_at_base (test_instance, 1));
	ASSERT_EQ((struct val *)existing_val_three, val_lst_at_base (test_instance, 2));

    val_str_free_safe(&existing_val_two);
	val_lst_clear (test_instance);
	val_lst_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_lst_replace_base, num)
{
	auto tm = mem_test_new_default (1024);

	auto existing_val_one = val_str_new_from_c_str (MEM(tm), "existing val one");
	auto existing_val_two = val_str_new_from_c_str (MEM(tm), "existing val two");
	auto existing_val_three = val_str_new_from_c_str (MEM(tm), "existing val three");

	auto test_instance = val_lst_new (MEM(tm));
	val_lst_append_str (test_instance, existing_val_one);
	val_lst_append_str (test_instance, existing_val_two);
	val_lst_append_str (test_instance, existing_val_three);

	auto replacement = (struct val *)val_num_new_from_double (MEM(tm), 28);
	val_lst_replace_base (test_instance, 1, replacement);

	ASSERT_EQ(3, val_lst_count (test_instance));
	ASSERT_EQ((struct val *)existing_val_one, val_lst_at_base (test_instance, 0));
	ASSERT_EQ((struct val *)replacement, val_lst_at_base (test_instance, 1));
	ASSERT_EQ((struct val *)existing_val_three, val_lst_at_base (test_instance, 2));

    val_str_free_safe(&existing_val_two);
	val_lst_clear (test_instance);
	val_lst_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_lst_replace_base, obj)
{
	auto tm = mem_test_new_default (1024);

	auto existing_val_one = val_str_new_from_c_str (MEM(tm), "existing val one");
	auto existing_val_two = val_str_new_from_c_str (MEM(tm), "existing val two");
	auto existing_val_three = val_str_new_from_c_str (MEM(tm), "existing val three");

	auto test_instance = val_lst_new (MEM(tm));
	val_lst_append_str (test_instance, existing_val_one);
	val_lst_append_str (test_instance, existing_val_two);
	val_lst_append_str (test_instance, existing_val_three);

	auto replacement = (struct val *)val_obj_new (MEM(tm), val_str_view_from_c_str ("some_obj"));
	val_lst_replace_base (test_instance, 1, replacement);

	ASSERT_EQ(3, val_lst_count (test_instance));
	ASSERT_EQ((struct val *)existing_val_one, val_lst_at_base (test_instance, 0));
	ASSERT_EQ((struct val *)replacement, val_lst_at_base (test_instance, 1));
	ASSERT_EQ((struct val *)existing_val_three, val_lst_at_base (test_instance, 2));

    val_str_free_safe(&existing_val_two);
	val_lst_clear (test_instance);
	val_lst_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_lst_replace_base, str)
{
	auto tm = mem_test_new_default (1024);

	auto existing_val_one = val_str_new_from_c_str (MEM(tm), "existing val one");
	auto existing_val_two = val_str_new_from_c_str (MEM(tm), "existing val two");
	auto existing_val_three = val_str_new_from_c_str (MEM(tm), "existing val three");

	auto test_instance = val_lst_new (MEM(tm));
	val_lst_append_str (test_instance, existing_val_one);
	val_lst_append_str (test_instance, existing_val_two);
	val_lst_append_str (test_instance, existing_val_three);

	auto replacement = (struct val *)val_str_new_from_c_str (MEM(tm), "replacement str");
	val_lst_replace_base (test_instance, 1, replacement);

	ASSERT_EQ(3, val_lst_count (test_instance));
	ASSERT_EQ((struct val *)existing_val_one, val_lst_at_base (test_instance, 0));
	ASSERT_EQ((struct val *)replacement, val_lst_at_base (test_instance, 1));
	ASSERT_EQ((struct val *)existing_val_three, val_lst_at_base (test_instance, 2));

    val_str_free_safe(&existing_val_two);
	val_lst_clear (test_instance);
	val_lst_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_lst_at_base, ok)
{
	auto tm = mem_test_new_default (1024);

	auto val = val_str_new_from_c_str (MEM(tm), "H4M41 B453");
	auto test_instance = val_lst_new (MEM(tm));

	val_lst_append_str (test_instance, val);
	ASSERT_EQ(1, val_lst_count (test_instance));
	ASSERT_EQ(AS_VAL (val), val_lst_at_base (test_instance, 0));

	val_lst_clear (test_instance);
	val_lst_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_lst_at_bool, ok)
{
	auto tm = mem_test_new_default (1024);

	auto val = val_bool_new(MEM(tm), true);
	auto test_instance = val_lst_new (MEM(tm));

	val_lst_append_bool (test_instance, val);
	ASSERT_EQ(1, val_lst_count (test_instance));
	ASSERT_EQ(val, val_lst_at_bool (test_instance, 0));

	val_lst_clear (test_instance);
	val_lst_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_lst_at_field, ok)
{
	auto tm = mem_test_new_default (1024);

	auto val = val_fld_new (MEM(tm), val_str_view_from_c_str ("field"), type_any);
	auto test_instance = val_lst_new (MEM(tm));

	val_lst_append_field (test_instance, val);
	ASSERT_EQ(1, val_lst_count (test_instance));
	ASSERT_EQ(val, val_lst_at_field (test_instance, 0));

	val_lst_clear (test_instance);
	val_lst_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_lst_at_fn, ok)
{
	auto tm = mem_test_new_default (1024);

	auto val = val_fn_new (MEM(tm), val_str_view_from_c_str ("fn"));
	auto test_instance = val_lst_new (MEM(tm));

	val_lst_append_fn (test_instance, val);
	ASSERT_EQ(1, val_lst_count (test_instance));
	ASSERT_EQ(val, val_lst_at_fn (test_instance, 0));

	val_lst_clear (test_instance);
	val_lst_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_lst_at_list, ok)
{
	auto tm = mem_test_new_default (1024);

	auto val = val_lst_new (MEM(tm));
	auto test_instance = val_lst_new (MEM(tm));

	val_lst_append_list (test_instance, val);
	ASSERT_EQ(1, val_lst_count (test_instance));
	ASSERT_EQ(val, val_lst_at_list (test_instance, 0));

	val_lst_clear (test_instance);
	val_lst_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_lst_at_num, ok)
{
	auto tm = mem_test_new_default (1024);

	auto val = val_num_new_from_double (MEM(tm), 1337);
	auto test_instance = val_lst_new (MEM(tm));

	val_lst_append_num (test_instance, val);
	ASSERT_EQ(1, val_lst_count (test_instance));
	ASSERT_EQ(val, val_lst_at_num (test_instance, 0));

	val_lst_clear (test_instance);
	val_lst_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_lst_at_obj, ok)
{
	auto tm = mem_test_new_default (1024);

	auto val = val_obj_new (MEM(tm), val_str_view_from_c_str ("some_obj"));
	auto test_instance = val_lst_new (MEM(tm));

	val_lst_append_obj (test_instance, val);
	ASSERT_EQ(1, val_lst_count (test_instance));
	ASSERT_EQ(val, val_lst_at_obj (test_instance, 0));

	val_lst_clear (test_instance);
	val_lst_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_lst_at_str, ok)
{
	auto tm = mem_test_new_default (1024);

	auto val = val_str_new_from_c_str (MEM(tm), "H4M41");
	auto test_instance = val_lst_new (MEM(tm));

	val_lst_append_str (test_instance, val);
	ASSERT_EQ(1, val_lst_count (test_instance));
	ASSERT_EQ(val, val_lst_at_str (test_instance, 0));

	val_lst_clear (test_instance);
	val_lst_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_lst_count, ok)
{
	auto tm = mem_test_new_default (1024);

	auto test_instance = val_lst_new (MEM(tm));
	val_lst_append(test_instance, val_num_new_from_double (MEM (tm), 127));
	ASSERT_EQ(1, val_lst_count (test_instance));
	val_lst_append(test_instance, val_num_new_from_double (MEM (tm), 128));
	ASSERT_EQ(2, val_lst_count (test_instance));
	val_lst_append(test_instance, val_str_new_from_c_str (MEM (tm), "ELODIE"));
	ASSERT_EQ(3, val_lst_count (test_instance));

	val_lst_clear (test_instance);
	val_lst_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_lst_count, empty)
{
	auto tm = mem_test_new_default (1024);

	auto test_instance = val_lst_new (MEM(tm));

	ASSERT_EQ(0, val_lst_count (test_instance));

	val_lst_clear (test_instance);
	val_lst_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_lst_capacity, empty)
{
	auto tm = mem_test_new_default (1024);

	auto test_instance = val_lst_new (MEM(tm));

	ASSERT_EQ(8, val_lst_capacity (test_instance));

	val_lst_clear (test_instance);
	val_lst_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_lst_capacity, ok)
{
	auto tm = mem_test_new_default (1024);

	auto test_instance = val_lst_new (MEM(tm));

	for (size_t idx = 0; idx < 10; idx++)
		{
			val_lst_append(test_instance, val_num_new_from_double (MEM (tm), idx));
		}

	ASSERT_EQ(16, val_lst_capacity (test_instance));

	val_lst_clear (test_instance);
	val_lst_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_lst_free_safe, ok)
{
	auto tm = mem_test_new_default (1024);

	auto some_val = val_num_new_from_double (MEM(tm), 9);
	auto test_instance = val_lst_new (MEM(tm));
	val_lst_append (test_instance, some_val);

	val_lst_clear (test_instance);
	val_lst_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_lst_free_safe, empty)
{
	auto tm = mem_test_new_default (1024);

	auto test_instance = val_lst_new (MEM(tm));
	val_lst_free_safe (&test_instance);
	ASSERT_TRUE(test_instance == nullptr);

	mem_test_verify (tm);
	mem_test_free (tm);
}