#include "../unit-test.h"

#include "core/algo/algo-list-byte.h"
#include "core/algo/algo-list.h"

TEST(byte_list_default_config, ok)
{
	auto tm = mem_test_new_default (512);
	auto result = byte_list_default_config (MEM(tm));
	ASSERT_EQ(8, result.initial_capacity);
	ASSERT_EQ(MEM (tm), result.mem);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(byte_list_new, ok)
{
	auto tm = mem_test_new_default (512);
	auto config = byte_list_default_config (MEM(tm));
	config.initial_capacity = 32;

	auto test_instance = byte_list_new (config);
	ASSERT_EQ(0, byte_list_size (test_instance));
	ASSERT_EQ(32, byte_list_capacity (test_instance));

	byte_list_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(byte_list_size, ok)
{
	auto tm = mem_test_new_default (512);
	auto test_instance = byte_list_new (byte_list_default_config (MEM(tm)));
	ASSERT_EQ(0, byte_list_size (test_instance));

	byte_list_append_u1 (test_instance, 42);
	ASSERT_EQ(1, byte_list_size (test_instance));

	byte_list_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(byte_list_capacity, ok)
{
	auto tm = mem_test_new_default (512);
	auto test_instance = byte_list_new (byte_list_default_config (MEM(tm)));
	ASSERT_EQ(8, byte_list_capacity (test_instance));

	for (size_t idx = 0; idx < 9; idx++)
		{
			ASSERT_EQ(8, byte_list_capacity (test_instance));
			byte_list_append_u1 (test_instance, 42);
		}
	ASSERT_EQ(16, byte_list_capacity (test_instance));

	byte_list_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(byte_list_append_bytes, ok)
{
	auto tm = mem_test_new_default (512);
	auto test_instance = byte_list_new (byte_list_default_config (MEM(tm)));

	ASSERT_EQ(8, byte_list_capacity (test_instance));

	byte_list_append_u1 (test_instance, 1);
	byte_list_append_u1 (test_instance, 2);
	byte_list_append_u1 (test_instance, 3);

	u1 byte_list_to_insert[] = {2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16};
	struct bytes_view bytes = {
		.data = byte_list_to_insert + 2,
		.size = 10  // not the complete content
	};
	byte_list_append_bytes (test_instance, bytes);

	ASSERT_EQ(13, byte_list_size (test_instance));
	ASSERT_EQ(16, byte_list_capacity (test_instance));

	for (size_t idx = 0; idx < 13; idx++)
		{
			u1 val;
			byte_list_at_u1 (test_instance, idx, &val);
			ASSERT_EQ(idx + 1, val);
		}

	byte_list_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(byte_list_append_byte_list, ok)
{
	auto tm = mem_test_new_default (512);

	auto source = byte_list_new (byte_list_default_config (MEM(tm)));
	byte_list_append_u1 (source, 42);
	byte_list_append_u4 (source, 1337);
	byte_list_append_u1 (source, 24);

	auto test_instance = byte_list_new (byte_list_default_config (MEM(tm)));
	byte_list_append_byte_list (test_instance, source);

	ASSERT_EQ(6, byte_list_size (test_instance));

	u1 first_val;
	ASSERT_TRUE(byte_list_at_u1 (test_instance, 0, &first_val));
	ASSERT_EQ(42, first_val);

	u4 second_val;
	ASSERT_TRUE(byte_list_at_u4 (test_instance, 1, &second_val));
	ASSERT_EQ(1337, second_val);

	u1 third_val;
	ASSERT_TRUE(byte_list_at_u1 (test_instance, 5, &third_val));
	ASSERT_EQ(24, third_val);

	byte_list_free_safe (&source);
	byte_list_free_safe (&test_instance);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(byte_list_append_str, ok)
{
	auto tm = mem_test_new_default (512);
	auto test_instance = byte_list_new (byte_list_default_config (MEM(tm)));

	auto c_str = "Elodie rockz";
	auto str = val_str_new_from_c_str (MEM(tm), c_str);
	byte_list_append_str (test_instance, str);

	ASSERT_EQ(12, byte_list_size (test_instance));
	ASSERT_EQ(16, byte_list_capacity (test_instance));

	struct val_str_view str_view{};
	byte_list_at_str_view (test_instance, 0, 12, &str_view);
	ASSERT_EQ(0, strncmp (c_str, str_view.data, 12));

	byte_list_free_safe (&test_instance);
    val_str_free_safe(&str);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(byte_list_u1, ok)
{
	auto tm = mem_test_new_default (512);
	auto test_instance = byte_list_new (byte_list_default_config (MEM(tm)));

	byte_list_append_u1 (test_instance, static_cast<u1>(0));
	byte_list_append_u1 (test_instance, static_cast<u1>(127));
	byte_list_append_u1 (test_instance, static_cast<u1>(U1_MAX));

	u1 val;
	ASSERT_TRUE(byte_list_at_u1 (test_instance, 0, &val));
	ASSERT_EQ(0, val);
	ASSERT_TRUE(byte_list_at_u1 (test_instance, 1, &val));
	ASSERT_EQ(127, val);
	ASSERT_TRUE(byte_list_at_u1 (test_instance, 2, &val));
	ASSERT_EQ(255, val);

	byte_list_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(byte_list_u2, ok)
{
	auto tm = mem_test_new_default (512);
	auto test_instance = byte_list_new (byte_list_default_config (MEM(tm)));

	byte_list_append_u2 (test_instance, static_cast<u2>(0));
	byte_list_append_u2 (test_instance, static_cast<u2>(1000));
	byte_list_append_u2 (test_instance, static_cast<u2>(U2_MAX));

	u2 val;
	ASSERT_TRUE(byte_list_at_u2 (test_instance, 0, &val));
	ASSERT_EQ(0, val);
	ASSERT_TRUE(byte_list_at_u2 (test_instance, 2, &val));
	ASSERT_EQ(1000, val);
	ASSERT_TRUE(byte_list_at_u2 (test_instance, 4, &val));
	ASSERT_EQ(U2_MAX, val);

	byte_list_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(byte_list_u4, ok)
{
	auto tm = mem_test_new_default (512);
	auto test_instance = byte_list_new (byte_list_default_config (MEM(tm)));

	byte_list_append_u4 (test_instance, static_cast<u4>(0));
	byte_list_append_u4 (test_instance, static_cast<u4>(1000));
	byte_list_append_u4 (test_instance, static_cast<u4>(U4_MAX));

	u4 val;
	ASSERT_TRUE(byte_list_at_u4 (test_instance, 0, &val));
	ASSERT_EQ(0, val);
	ASSERT_TRUE(byte_list_at_u4 (test_instance, 4, &val));
	ASSERT_EQ(1000, val);
	ASSERT_TRUE(byte_list_at_u4 (test_instance, 8, &val));
	ASSERT_EQ(U4_MAX, val);

	byte_list_replace_u4 (test_instance, 4, static_cast<u4>(2810));
	ASSERT_TRUE(byte_list_at_u4 (test_instance, 0, &val));
	ASSERT_EQ(0, val);
	ASSERT_TRUE(byte_list_at_u4 (test_instance, 4, &val));
	ASSERT_EQ(2810, val);
	ASSERT_TRUE(byte_list_at_u4 (test_instance, 8, &val));
	ASSERT_EQ(U4_MAX, val);

	byte_list_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(byte_list_at, ok)
{
	auto tm = mem_test_new_default (512);
	auto test_instance = byte_list_new (byte_list_default_config (MEM(tm)));

	byte_list_append_u2 (test_instance, static_cast<u2>(21));
	byte_list_append_u4 (test_instance, static_cast<u4>(42));

	u1 *ptr = nullptr;
	ASSERT_TRUE(byte_list_at (test_instance, 0, &ptr));
	ASSERT_EQ(ptr, test_instance->underlying_list.data);
	ASSERT_EQ(21, *(u2 *)(void *)ptr);
	ASSERT_TRUE(byte_list_at (test_instance, 2, &ptr));
	ASSERT_EQ(ptr, (u1 *)test_instance->underlying_list.data + 2);
	ASSERT_EQ(42, *(u4 *)(void *)ptr);

	byte_list_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}