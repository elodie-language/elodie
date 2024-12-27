#include "../unit-test.h"
#include "core/bytes/bytes-buffer.h"
#include "core/val/val-api.h"

TEST(bye_buffer, ok)
{
	auto tm = mem_test_new_default (128);

	auto test_instance = buffer_new (MEM(tm), 64);

	ASSERT_EQ(64, buffer_available (test_instance));
	ASSERT_EQ(64, buffer_capacity (test_instance));

	buffer_write_u1 (test_instance, 42);
	buffer_write_u1 (test_instance, 43);
	buffer_write_u1 (test_instance, 44);
	buffer_write_u1 (test_instance, 45);

	ASSERT_EQ(60, buffer_available (test_instance));
	ASSERT_EQ(64, buffer_capacity (test_instance));

	buffer_flip (test_instance);
	ASSERT_EQ(4, buffer_available (test_instance));
	ASSERT_EQ(64, buffer_capacity (test_instance));

	ASSERT_EQ(42, buffer_read_u1 (test_instance));
	ASSERT_EQ(43, buffer_read_u1 (test_instance));
	ASSERT_EQ(44, buffer_read_u1 (test_instance));
	ASSERT_EQ(45, buffer_read_u1 (test_instance));

	ASSERT_EQ(0, buffer_available (test_instance));
	ASSERT_EQ(64, buffer_capacity (test_instance));

	buffer_clear (test_instance);

	ASSERT_EQ(64, buffer_available (test_instance));
	ASSERT_EQ(64, buffer_capacity (test_instance));

	buffer_free_safe (&test_instance);
	ASSERT_TRUE(test_instance == nullptr);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(bye_buffer_compact, ok)
{
	auto tm = mem_test_new_default (128);

	auto test_instance = buffer_new (MEM(tm), 64);

	for (size_t idx = 0; idx < 64; idx++)
		{
			buffer_write_u1 (test_instance, static_cast<u1>(idx));
		}

	buffer_flip (test_instance);

	for (size_t idx = 0; idx < 16; idx++)
		{
			ASSERT_EQ(idx, buffer_read_u1 (test_instance));
		}

	ASSERT_EQ(16, buffer_position (test_instance));
	ASSERT_EQ(64, buffer_limit (test_instance));
	ASSERT_EQ(48, buffer_available (test_instance));
	ASSERT_EQ(64, buffer_capacity (test_instance));

	buffer_compact (test_instance);

	ASSERT_EQ(48, buffer_position (test_instance));
	ASSERT_EQ(64, buffer_limit (test_instance));
	ASSERT_EQ(16, buffer_available (test_instance));
	ASSERT_EQ(64, buffer_capacity (test_instance));

	ASSERT_EQ((u1)48, buffer_read_u1 (test_instance));
	ASSERT_EQ((u1)49, buffer_read_u1 (test_instance));

	buffer_free_safe (&test_instance);
	ASSERT_TRUE(test_instance == nullptr);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(buffer, ok)
{
	auto test_instance = buffer_new (MEM(mem_raw_new ()), 128);

	ASSERT_TRUE(test_instance != nullptr);
	ASSERT_EQ(0, test_instance->position);
	ASSERT_EQ(128, test_instance->limit);
	ASSERT_EQ(128, test_instance->capacity);
	ASSERT_TRUE(test_instance->data != nullptr);

	buffer_free_safe (&test_instance);
	ASSERT_TRUE(test_instance == nullptr);
}

TEST(buffer_capacity, ok)
{
	auto test_instance = buffer_new (MEM(mem_raw_new ()), 3);

	ASSERT_EQ(3, buffer_capacity (test_instance));
	buffer_write_u1 (test_instance, 1);
	ASSERT_EQ(3, buffer_capacity (test_instance));
	buffer_write_u1 (test_instance, 2);
	ASSERT_EQ(3, buffer_capacity (test_instance));
	buffer_write_u1 (test_instance, 3);
	ASSERT_EQ(3, buffer_capacity (test_instance));

	buffer_free_safe (&test_instance);
	ASSERT_TRUE(test_instance == nullptr);
}

TEST(buffer_available, ok)
{
	auto test_instance = buffer_new (MEM(mem_raw_new ()), 3);

	ASSERT_EQ(3, buffer_available (test_instance));
	buffer_write_u1 (test_instance, 1);
	ASSERT_EQ(2, buffer_available (test_instance));
	buffer_write_u1 (test_instance, 2);
	ASSERT_EQ(1, buffer_available (test_instance));
	buffer_write_u1 (test_instance, 3);
	ASSERT_EQ(0, buffer_available (test_instance));

	buffer_free_safe (&test_instance);
	ASSERT_TRUE(test_instance == nullptr);
}

TEST(buffer_u1, ok)
{
	auto test_instance = buffer_new (MEM(mem_raw_new ()), 3);

	ASSERT_EQ(3, buffer_available (test_instance));
	buffer_write_u1 (test_instance, 32);
	ASSERT_EQ(2, buffer_available (test_instance));
	buffer_write_u1 (test_instance, 64);
	ASSERT_EQ(1, buffer_available (test_instance));
	buffer_write_u1 (test_instance, 128);
	ASSERT_EQ(0, buffer_available (test_instance));

	buffer_flip (test_instance);
	ASSERT_EQ(3, buffer_available (test_instance));
	ASSERT_EQ(32, buffer_read_u1 (test_instance));
	ASSERT_EQ(2, buffer_available (test_instance));
	ASSERT_EQ(64, buffer_read_u1 (test_instance));
	ASSERT_EQ(1, buffer_available (test_instance));
	ASSERT_EQ(128, buffer_read_u1 (test_instance));
	ASSERT_EQ(0, buffer_available (test_instance));

	buffer_free_safe (&test_instance);
	ASSERT_TRUE(test_instance == nullptr);
}

TEST(buffer_bytes, ok)
{
	auto test_instance = buffer_new (MEM(mem_raw_new ()), 6);

	struct bytes_view input{};
	input.data = (u1 *)"HAMAL\0";
	input.size = 6;

	buffer_write_bytes (test_instance, input);
	ASSERT_EQ(0, buffer_available (test_instance));

	buffer_flip (test_instance);
	ASSERT_EQ(6, buffer_available (test_instance));

	struct bytes_view output = buffer_read_bytes (test_instance, 6);
	ASSERT_EQ(0, buffer_available (test_instance));

	ASSERT_EQ(6, output.size);
	struct dep_val_str_view input_view = dep_val_str_view_from_bytes (input);
	struct dep_val_str_view output_view = dep_val_str_view_from_bytes (output);

	ASSERT_TRUE(VAL_EQ (input_view, output_view));

	buffer_free_safe (&test_instance);
	ASSERT_TRUE(test_instance == nullptr);
}