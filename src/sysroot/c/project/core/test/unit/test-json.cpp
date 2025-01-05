#include "unit-test.h"

#include "core/json.h"
#include "core/val/val-api.h"

TEST(json_writer, write_empty_obj)
{
	auto tm = mem_test_new_default (1024);

	struct json_writer test_instance{};
	json_writer_init (&test_instance, MEM(tm));

	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_start (&test_instance));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_end (&test_instance));

	struct val_str_view result{};
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_to_str_view (&test_instance, &result));
	struct val_str_view expected = val_str_view_from_c_str ("{}");
	ASSERT_TRUE(VAL_EQ (expected, result));

	json_writer_reset (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(json_writer, sub_objs)
{
	auto tm = mem_test_new_default (1024);

	struct json_writer test_instance{};
	json_writer_init (&test_instance, MEM(tm));

	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_start (&test_instance));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_c_str (&test_instance, JSON_KEY ("hello"), "elodie"));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_start_obj (&test_instance, JSON_KEY ("sub")));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_c_str (&test_instance, JSON_KEY ("hello"), "elodie"));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_c_str (&test_instance, JSON_KEY ("bye"), "elodie"));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_end (&test_instance));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_end (&test_instance));

	struct val_str_view result{};
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_to_str_view (&test_instance, &result));
	struct val_str_view expected = val_str_view_from_c_str (R"({"hello":"elodie","sub":{"hello":"elodie","bye":"elodie"}})");
	ASSERT_TRUE(VAL_EQ (expected, result));

	json_writer_reset (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(json_writer, json_writer_obj_c_str)
{
	auto tm = mem_test_new_default (1024);

	struct json_writer test_instance{};
	json_writer_init (&test_instance, MEM(tm));

	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_start (&test_instance));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_c_str (&test_instance, JSON_KEY ("hello"), "elodie"));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_c_str (&test_instance, JSON_KEY ("bye"), "elodie"));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_end (&test_instance));

	struct val_str_view result{};
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_to_str_view (&test_instance, &result));
	struct val_str_view expected = val_str_view_from_c_str (R"({"hello":"elodie","bye":"elodie"})");
	ASSERT_TRUE(VAL_EQ (expected, result));

	json_writer_reset (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(json_writer, json_writer_obj_null)
{
	auto tm = mem_test_new_default (1024);

	struct json_writer test_instance{};
	json_writer_init (&test_instance, MEM(tm));

	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_start (&test_instance));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_null (&test_instance, JSON_KEY ("rootNode")));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_null (&test_instance, JSON_KEY ("subNode")));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_end (&test_instance));

	struct val_str_view result{};
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_to_str_view (&test_instance, &result));
	struct val_str_view expected = val_str_view_from_c_str (R"({"rootNode":null,"subNode":null})");
	ASSERT_TRUE(VAL_EQ (expected, result));

	json_writer_reset (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(json_writer, json_writer_obj_num)
{
	auto tm = mem_test_new_default (1024);

	struct json_writer test_instance{};
	json_writer_init (&test_instance, MEM(tm));

	auto num_one = val_num_new_from_double (MEM(tm), 23);
	auto num_two = val_num_new_from_double (MEM(tm), 42);

	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_start (&test_instance));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_num (&test_instance, JSON_KEY ("lhs"), num_one));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_num (&test_instance, JSON_KEY ("rhs"), num_two));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_end (&test_instance));

	struct val_str_view result{};
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_to_str_view (&test_instance, &result));
	struct val_str_view expected = val_str_view_from_c_str (R"({"lhs":23,"rhs":42})");
	ASSERT_TRUE(VAL_EQ (expected, result));

	json_writer_reset (&test_instance);

	val_num_free_safe (&num_one);
	val_num_free_safe (&num_two);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(json_writer, json_writer_obj_bool)
{
	auto tm = mem_test_new_default (1024);

	struct json_writer test_instance{};
	json_writer_init (&test_instance, MEM(tm));

	auto bool_one = val_bool_new(MEM(tm), true);
	auto bool_two = val_bool_new(MEM(tm), false);

	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_start (&test_instance));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_bool (&test_instance, JSON_KEY ("lhs"), bool_one));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_bool (&test_instance, JSON_KEY ("rhs"), bool_two));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_end (&test_instance));

	struct val_str_view result{};
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_to_str_view (&test_instance, &result));
	struct val_str_view expected = val_str_view_from_c_str (R"({"lhs":true,"rhs":false})");
	ASSERT_TRUE(VAL_EQ (expected, result));

	json_writer_reset (&test_instance);
	val_bool_free_safe (&bool_one);
	val_bool_free_safe (&bool_two);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(json_writer, json_writer_obj_str_view)
{
	auto tm = mem_test_new_default (1024);

	struct json_writer test_instance{};
	json_writer_init (&test_instance, MEM(tm));

	auto str_one = val_str_view_from_c_str ("hello");
	auto str_two = val_str_view_from_c_str ("world");

	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_start (&test_instance));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_str_view (&test_instance, JSON_KEY ("lhs"), str_one));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_str_view (&test_instance, JSON_KEY ("rhs"), str_two));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_end (&test_instance));

	struct val_str_view result{};
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_to_str_view (&test_instance, &result));
	struct val_str_view expected = val_str_view_from_c_str (R"({"lhs":"hello","rhs":"world"})");
	ASSERT_TRUE(VAL_EQ (expected, result));

	json_writer_reset (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(json_writer, json_writer_obj_str)
{
	auto tm = mem_test_new_default (1024);

	struct json_writer test_instance{};
	json_writer_init (&test_instance, MEM(tm));

	auto str_one = val_str_new_from_c_str (MEM(tm), "hello");
	auto str_two = val_str_new_from_c_str (MEM(tm), "world");

	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_start (&test_instance));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_str (&test_instance, JSON_KEY ("lhs"), str_one));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_str (&test_instance, JSON_KEY ("rhs"), str_two));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_end (&test_instance));

	struct val_str_view result{};
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_to_str_view (&test_instance, &result));
	struct val_str_view expected = val_str_view_from_c_str (R"({"lhs":"hello","rhs":"world"})");
	ASSERT_TRUE(VAL_EQ (expected, result));

	json_writer_reset (&test_instance);

    val_str_free_safe(&str_one);
    val_str_free_safe(&str_two);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(json_writer, json_writer_obj_val)
{
	auto tm = mem_test_new_default (2048);

	struct json_writer test_instance{};
	json_writer_init (&test_instance, MEM(tm));

	auto str_val = val_str_new_from_c_str (MEM(tm), "elodie");
	auto num_val = val_num_new_from_double (MEM(tm), 1337);
	auto bool_val = val_bool_new(MEM(tm), true);

	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_start (&test_instance));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_val (&test_instance, JSON_KEY ("str"), (struct val *)str_val));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_val (&test_instance, JSON_KEY ("num"), (struct val *)num_val));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_val (&test_instance, JSON_KEY ("bool"), (struct val *)bool_val));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_end (&test_instance));

	struct val_str_view result{};
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_to_str_view (&test_instance, &result));
	struct val_str_view expected = val_str_view_from_c_str (R"({"str":"elodie","num":1337,"bool":true})");
	ASSERT_TRUE(VAL_EQ (expected, result));

	json_writer_reset (&test_instance);

    val_str_free_safe(&str_val);
	val_num_free_safe (&num_val);
	val_bool_free_safe (&bool_val);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(json_writer, json_writer_obj_start_array)
{
	auto tm = mem_test_new_default (1024);

	struct json_writer test_instance{};
	json_writer_init (&test_instance, MEM(tm));

	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_start (&test_instance));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_start_array (&test_instance, JSON_KEY ("lst")));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_end (&test_instance));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_end (&test_instance));

	struct val_str_view result{};
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_to_str_view (&test_instance, &result));
	struct val_str_view expected = val_str_view_from_c_str (R"({"lst":[]})");
	ASSERT_TRUE(VAL_EQ (expected, result));

	json_writer_reset (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(json_writer, write_empty_array)
{
	auto tm = mem_test_new_default (1024);

	struct json_writer test_instance{};
	json_writer_init (&test_instance, MEM(tm));

	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_start (&test_instance));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_end (&test_instance));

	struct val_str_view result{};
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_to_str_view (&test_instance, &result));
	struct val_str_view expected = val_str_view_from_c_str ("[]");
	ASSERT_TRUE(VAL_EQ (expected, result));

	json_writer_reset (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(json_writer, sub_arrays)
{
	auto tm = mem_test_new_default (1024);

	struct json_writer test_instance{};
	json_writer_init (&test_instance, MEM(tm));

	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_start (&test_instance));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_c_str (&test_instance, "elodie"));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_start (&test_instance));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_c_str (&test_instance, "elodie"));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_end (&test_instance));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_end (&test_instance));

	struct val_str_view result{};
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_to_str_view (&test_instance, &result));
	struct val_str_view expected = val_str_view_from_c_str (R"(["elodie",["elodie"]])");
	ASSERT_TRUE(VAL_EQ (expected, result));

	json_writer_reset (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(json_writer, json_writer_array_c_str)
{
	auto tm = mem_test_new_default (1024);

	struct json_writer test_instance{};
	json_writer_init (&test_instance, MEM(tm));

	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_start (&test_instance));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_c_str (&test_instance, "elodie_1"));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_c_str (&test_instance, "elodie_2"));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_c_str (&test_instance, "elodie_3"));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_end (&test_instance));

	struct val_str_view result{};
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_to_str_view (&test_instance, &result));
	struct val_str_view expected = val_str_view_from_c_str (R"(["elodie_1","elodie_2","elodie_3"])");
	ASSERT_TRUE(VAL_EQ (expected, result));

	json_writer_reset (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(json_writer, json_writer_array_str_view)
{
	auto tm = mem_test_new_default (1024);

	struct json_writer test_instance{};
	json_writer_init (&test_instance, MEM(tm));

	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_start (&test_instance));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_str_view (&test_instance, val_str_view_from_c_str ("elodie_1")));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_str_view (&test_instance, val_str_view_from_c_str ("elodie_2")));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_str_view (&test_instance, val_str_view_from_c_str ("elodie_3")));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_end (&test_instance));

	struct val_str_view result{};
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_to_str_view (&test_instance, &result));
	struct val_str_view expected = val_str_view_from_c_str (R"(["elodie_1","elodie_2","elodie_3"])");
	ASSERT_TRUE(VAL_EQ (expected, result));

	json_writer_reset (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(json_writer, json_writer_array_null)
{
	auto tm = mem_test_new_default (1024);

	struct json_writer test_instance{};
	json_writer_init (&test_instance, MEM(tm));

	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_start (&test_instance));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_null (&test_instance));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_null (&test_instance));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_end (&test_instance));

	struct val_str_view result{};
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_to_str_view (&test_instance, &result));
	struct val_str_view expected = val_str_view_from_c_str (R"([null,null])");
	ASSERT_TRUE(VAL_EQ (expected, result));

	json_writer_reset (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(json_writer, json_writer_array_num)
{
	auto tm = mem_test_new_default (1024);

	struct json_writer test_instance{};
	json_writer_init (&test_instance, MEM(tm));

	auto num_one = val_num_new_from_double (MEM(tm), 23);
	auto num_two = val_num_new_from_double (MEM(tm), 42);

	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_start (&test_instance));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_num (&test_instance, num_one));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_num (&test_instance, num_two));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_end (&test_instance));

	struct val_str_view result{};
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_to_str_view (&test_instance, &result));
	struct val_str_view expected = val_str_view_from_c_str (R"([23,42])");
	ASSERT_TRUE(VAL_EQ (expected, result));

	json_writer_reset (&test_instance);

	val_num_free_safe (&num_one);
	val_num_free_safe (&num_two);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(json_writer, json_writer_array_bool)
{
	auto tm = mem_test_new_default (1024);

	struct json_writer test_instance{};
	json_writer_init (&test_instance, MEM(tm));

	auto bool_one = val_bool_new(MEM(tm), true);
	auto bool_two = val_bool_new(MEM(tm), false);

	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_start (&test_instance));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_bool (&test_instance, bool_one));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_bool (&test_instance, bool_two));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_end (&test_instance));

	struct val_str_view result{};
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_to_str_view (&test_instance, &result));
	struct val_str_view expected = val_str_view_from_c_str (R"([true,false])");
	ASSERT_TRUE(VAL_EQ (expected, result));

	json_writer_reset (&test_instance);

	val_bool_free_safe (&bool_one);
	val_bool_free_safe (&bool_two);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(json_writer, json_writer_array_str)
{
	auto tm = mem_test_new_default (1024);

	struct json_writer test_instance{};
	json_writer_init (&test_instance, MEM(tm));

	auto str_one = val_str_new_from_c_str (MEM(tm), "hello");
	auto str_two = val_str_new_from_c_str (MEM(tm), "world");

	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_start (&test_instance));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_str (&test_instance, str_one));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_str (&test_instance, str_two));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_end (&test_instance));

	struct val_str_view result{};
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_to_str_view (&test_instance, &result));
	struct val_str_view expected = val_str_view_from_c_str (R"(["hello","world"])");
	ASSERT_TRUE(VAL_EQ (expected, result));

	json_writer_reset (&test_instance);

    val_str_free_safe(&str_one);
    val_str_free_safe(&str_two);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(json_writer, json_writer_array_val)
{
	auto tm = mem_test_new_default (1024);

	struct json_writer test_instance{};
	json_writer_init (&test_instance, MEM(tm));

	auto str_val = val_str_new_from_c_str (MEM(tm), "elodie");
	auto num_val = val_num_new_from_double (MEM(tm), 1337);
	auto bool_val = val_bool_new(MEM(tm), true);

	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_start (&test_instance));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_val (&test_instance, (struct val *)str_val));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_val (&test_instance, (struct val *)num_val));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_val (&test_instance, (struct val *)bool_val));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_end (&test_instance));

	struct val_str_view result{};
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_to_str_view (&test_instance, &result));
	struct val_str_view expected = val_str_view_from_c_str (R"(["elodie",1337,true])");
	ASSERT_TRUE(VAL_EQ (expected, result));

	json_writer_reset (&test_instance);

    val_str_free_safe(&str_val);
	val_num_free_safe (&num_val);
	val_bool_free_safe (&bool_val);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(json_writer, write_multiple_empty_objs_to_array)
{
	auto tm = mem_test_new_default (1024);

	struct json_writer test_instance{};
	json_writer_init (&test_instance, MEM(tm));

	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_start (&test_instance));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_start (&test_instance));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_end (&test_instance));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_start (&test_instance));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_end (&test_instance));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_end (&test_instance));

	struct val_str_view result{};
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_to_str_view (&test_instance, &result));
	struct val_str_view expected = val_str_view_from_c_str (R"([{},{}])");
	ASSERT_TRUE(VAL_EQ (expected, result));

	json_writer_reset (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(json_writer, start_obj_but_close_array)
{
	auto tm = mem_test_new_default (1024);

	struct json_writer test_instance{};
	json_writer_init (&test_instance, MEM(tm));

	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_start (&test_instance));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_c_str (&test_instance, JSON_KEY ("hello"), "elodie"));
	ASSERT_EQ(JSON_WRITER_STATUS_NESTING_ERROR, json_writer_array_end (&test_instance));

	struct val_str_view result{};
	ASSERT_EQ(JSON_WRITER_STATUS_NESTING_ERROR, json_writer_to_str_view (&test_instance, &result));
	struct val_str_view expected = val_str_view_from_c_str ("");
	ASSERT_TRUE(VAL_EQ (expected, result));

	json_writer_reset (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(json_writer, start_array_but_close_obj)
{
	auto tm = mem_test_new_default (1024);

	struct json_writer test_instance{};
	json_writer_init (&test_instance, MEM(tm));

	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_start (&test_instance));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_c_str (&test_instance, "elodie"));
	ASSERT_EQ(JSON_WRITER_STATUS_NESTING_ERROR, json_writer_obj_end (&test_instance));

	struct val_str_view result{};
	ASSERT_EQ(JSON_WRITER_STATUS_NESTING_ERROR, json_writer_to_str_view (&test_instance, &result));
	struct val_str_view expected = val_str_view_from_c_str ("");
	ASSERT_TRUE(VAL_EQ (expected, result));

	json_writer_reset (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(json_writer, not_closed_obj)
{
	auto tm = mem_test_new_default (1024);

	struct json_writer test_instance{};
	json_writer_init (&test_instance, MEM(tm));

	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_start (&test_instance));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_obj_c_str (&test_instance, JSON_KEY ("hello"), "elodie"));

	struct val_str_view result{};
	ASSERT_EQ(JSON_WRITER_STATUS_NESTING_ERROR, json_writer_to_str_view (&test_instance, &result));
	struct val_str_view expected = val_str_view_from_c_str ("");
	ASSERT_TRUE(VAL_EQ (expected, result));

	json_writer_reset (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(json_writer, not_closed_array)
{
	auto tm = mem_test_new_default (1024);

	struct json_writer test_instance{};
	json_writer_init (&test_instance, MEM(tm));

	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_start (&test_instance));
	ASSERT_EQ(JSON_WRITER_STATUS_OK, json_writer_array_c_str (&test_instance, "elodie"));

	struct val_str_view result{};
	ASSERT_EQ(JSON_WRITER_STATUS_NESTING_ERROR, json_writer_to_str_view (&test_instance, &result));
	struct val_str_view expected = val_str_view_from_c_str ("");
	ASSERT_TRUE(VAL_EQ (expected, result));

	json_writer_reset (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

