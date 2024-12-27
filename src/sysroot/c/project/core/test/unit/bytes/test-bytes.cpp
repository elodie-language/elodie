#include "../unit-test.h"
#include "core/bytes/byte-api.h"

TEST(bytes_allocate, ok)
{
	auto tm = mem_test_new_default (128);

	auto result = bytes_allocate (MEM(tm), 64);
	ASSERT_EQ(64, result->size);
	/**
	 * 64 byte + 16 byte (8 byte data, 4 byte size + padding)
	 */
	ASSERT_EQ(80, mem_test_size (tm));

	bytes_deallocate_safe (&result, MEM(tm));

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(bytes_init, ok)
{
	auto tm = mem_test_new_default (128);

	struct bytes test_instance{};
	bytes_init (&test_instance, MEM(tm), 32);

	ASSERT_EQ(32, test_instance.size);
	ASSERT_EQ(32, mem_test_size (tm));

	bytes_reset (&test_instance, MEM(tm));

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(bytes_reset, ok)
{
	auto tm = mem_test_new_default (128);

	struct bytes test_instance{};
	bytes_init (&test_instance, MEM(tm), 32);

	bytes_reset (&test_instance, MEM(tm));
	ASSERT_EQ(0, test_instance.size);
	ASSERT_TRUE(test_instance.data == nullptr);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(bytes_deallocate_safe, ok)
{
	auto tm = mem_test_new_default (128);

	auto test_instance = bytes_allocate (MEM(tm), 64);

	bytes_deallocate_safe (&test_instance, MEM(tm));
	ASSERT_TRUE(test_instance == nullptr);

	mem_test_verify (tm);
	mem_test_free (tm);
}