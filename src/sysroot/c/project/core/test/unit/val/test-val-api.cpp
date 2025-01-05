#include "../unit-test.h"

#include "core/mem/mem.h"
#include "core/val/val-api.h"

TEST(val_copy, num)
{
	auto tm = mem_test_new_default (64);

	auto *test_instance = (struct val *) val_num_new(MEM(tm), 12.34);
	auto *result = val_copy (test_instance, MEM(tm));

	ASSERT_TRUE(val_equal (test_instance, result));

	val_free_safe (&test_instance);
	val_free_safe (&result);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_str, ok)
{
	auto tm = mem_test_new_default (128);

	char const *str = "Elodie is about to kick... popo";
	auto result = val_str_new_from_c_str (MEM(tm), const_cast<char *>(str));

    val_str_free_safe(&result);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_equal, different_types)
{
	auto tm = mem_test_new_default (128);

	std::string str = "elodie rockz";
	auto val_one = (struct val *)val_str_new_from_c_str (MEM(tm), str.c_str ());
	auto val_two = (struct val *) val_num_new(MEM(tm), 42.12);
	ASSERT_FALSE(val_equal (val_one, val_two));

	val_free_safe (&val_one);
	val_free_safe (&val_two);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_equal, str_str_equal)
{
	auto tm = mem_test_new_default (128);

	std::string str = "elodie rockz";
	auto val_one = (struct val *)val_str_new_from_c_str (MEM(tm), str.c_str ());
	auto val_two = (struct val *)val_str_new_from_c_str (MEM(tm), str.c_str ());
	ASSERT_TRUE(val_equal (val_one, val_two));

	val_free_safe (&val_one);
	val_free_safe (&val_two);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_equal, str_str_not_equal)
{
	auto tm = mem_test_new_default (128);

	auto val_one = (struct val *)val_str_new_from_c_str (MEM(tm), "elodie rockz");
	auto val_two = (struct val *)val_str_new_from_c_str (MEM(tm), "other val");
	ASSERT_FALSE(val_equal (val_one, val_two));

	val_free_safe (&val_one);
	val_free_safe (&val_two);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_equal, num_num_equal)
{
	auto tm = mem_test_new_default (128);

	auto val_one = (struct val *) val_num_new(MEM(tm), 42);
	auto val_two = (struct val *) val_num_new(MEM(tm), 42);
	ASSERT_TRUE(val_equal (val_one, val_two));

	val_free_safe (&val_one);
	val_free_safe (&val_two);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_equal, num_num_not_equal)
{
	auto tm = mem_test_new_default (128);

	auto val_one = (struct val *) val_num_new(MEM(tm), 42);
	auto val_two = (struct val *) val_num_new(MEM(tm), 21);
	ASSERT_FALSE(val_equal (val_one, val_two));

	val_free_safe (&val_one);
	val_free_safe (&val_two);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_equal, bool_bool_equal)
{
	auto tm = mem_test_new_default (128);

	auto val_one = (struct val *) val_bool_new(MEM(tm), false);
	auto val_two = (struct val *) val_bool_new(MEM(tm), false);
	ASSERT_TRUE(val_equal (val_one, val_two));

	val_free_safe (&val_one);
	val_free_safe (&val_two);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(val_equal, bool_bool_not_equal)
{
	auto tm = mem_test_new_default (128);

	auto val_one = (struct val *) val_bool_new(MEM(tm), true);
	auto val_two = (struct val *) val_bool_new(MEM(tm), false);
	ASSERT_FALSE(val_equal (val_one, val_two));

	val_free_safe (&val_one);
	val_free_safe (&val_two);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(VAL_EQUAL, val_str_view__with__val_str_view)
{
	auto tm = mem_test_new_default (128);

	auto given_val = val_str_view_from_c_str ("val");
	auto same_val = val_str_view_from_c_str ("val");
	auto another_val = val_str_view_from_c_str ("another_val");

	ASSERT_TRUE(VAL_EQ (given_val, same_val));
	ASSERT_FALSE(VAL_EQ (given_val, another_val));

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(VAL_EQUAL, val_str_view__with__c_str)
{
	auto tm = mem_test_new_default (128);

	auto given_val = val_str_view_from_c_str ("val");
	auto same_val = "val";
	auto another_val = "another_val";

	ASSERT_TRUE(VAL_EQ (given_val, same_val));
	ASSERT_FALSE(VAL_EQ (given_val, another_val));

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(VAL_EQUAL, val_str_view__with__val_str)
{
	auto tm = mem_test_new_default (128);

	auto given_val = val_str_view_from_c_str ("val");
	auto same_val = val_str_new_from_c_str (MEM(tm), "val");
	auto another_val = val_str_new_from_c_str (MEM(tm), "another_val");

	ASSERT_TRUE(VAL_EQ (given_val, same_val));
	ASSERT_FALSE(VAL_EQ (given_val, another_val));

    val_str_free_safe(&same_val);
    val_str_free_safe(&another_val);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(VAL_EQUAL, val_str__with__c_str)
{
	auto tm = mem_test_new_default (128);

	auto given_val = val_str_new_from_c_str (MEM(tm), "val");
	auto same_val = "val";
	auto another_val = "another_val";

	ASSERT_TRUE(VAL_EQ (given_val, same_val));
	ASSERT_FALSE(VAL_EQ (given_val, another_val));

    val_str_free_safe(&given_val);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(VAL_EQUAL, val_str__with__val_str)
{
	auto tm = mem_test_new_default (128);

	auto given_val = val_str_new_from_c_str (MEM(tm), "val");
	auto same_val = val_str_new_from_c_str (MEM(tm), "val");
	auto another_val = val_str_new_from_c_str (MEM(tm), "another_val");

	ASSERT_TRUE(VAL_EQ (given_val, same_val));
	ASSERT_FALSE(VAL_EQ (given_val, another_val));

    val_str_free_safe(&given_val);
    val_str_free_safe(&same_val);
    val_str_free_safe(&another_val);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(VAL_EQUAL, val_str__with__val_str_view)
{
	auto tm = mem_test_new_default (128);

	auto given_val = val_str_new_from_c_str (MEM(tm), "val");
	auto same_val = val_str_view_from_c_str ("val");
	auto another_val = val_str_view_from_c_str ("another_val");

	ASSERT_TRUE(VAL_EQ (given_val, same_val));
	ASSERT_FALSE(VAL_EQ (given_val, another_val));

    val_str_free_safe(&given_val);

	mem_test_verify (tm);
	mem_test_free (tm);
}