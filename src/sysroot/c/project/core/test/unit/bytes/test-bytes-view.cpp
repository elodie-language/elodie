#include "../unit-test.h"

#include "core/bytes/bytes-view.h"

TEST(bytes, u2)
{
	u2 data = 128;
	auto bytes = bytes_view_of_u2 (&data);
	ASSERT_EQ(2, bytes.size);

	u2 result = bytes_view_as_u2 (bytes);
	ASSERT_EQ(128, result);
}

TEST(bytes, u4)
{
	u4 data = 128821;
	auto bytes = bytes_view_of_u4 (&data);
	ASSERT_EQ(4, bytes.size);

	u4 result = bytes_view_as_u4 (bytes);
	ASSERT_EQ(128821, result);
}

TEST(bytes, u8)
{
	u8 data = 18446744073709551615LU;
	auto bytes = bytes_view_of_u8 (&data);
	ASSERT_EQ(8, bytes.size);

	u8 result = bytes_view_as_u8 (bytes);
	ASSERT_EQ(18446744073709551615LU, result);
}

TEST(bytes, bytes)
{
	auto tm = mem_test_new_default (128);

	struct bytes input{};
	bytes_init (&input, MEM(tm), 32);

	auto result = bytes_view_of_bytes (input);
	ASSERT_EQ(32, result.size);
	ASSERT_EQ(input.data, result.data);

	bytes_reset (&input, MEM(tm));

	mem_test_verify (tm);
	mem_test_free (tm);
}