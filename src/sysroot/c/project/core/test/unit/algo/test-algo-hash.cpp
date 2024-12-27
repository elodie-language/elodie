#include "../unit-test.h"

#include "core/mem/mem.h"
#include "core/algo/algo-hash.h"

static uint8_t data[] = {0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08};
static size_t seed_1 = 42;
static size_t seed_2 = 21;
static struct bytes_view bytes = {
	.data = data,
	.size = 9,
};

TEST(hash4_equal, same_value)
{
	struct hash4 test_instance_one{};
	test_instance_one.value = 24;

	struct hash4 test_instance_two{};
	test_instance_two.value = 24;

	ASSERT_TRUE(hash4_equal (test_instance_one, test_instance_two));
}

TEST(hash4_equal, different_value)
{
	struct hash4 test_instance_one{};
	test_instance_one.value = 24;

	struct hash4 test_instance_two{};
	test_instance_two.value = 42;

	ASSERT_FALSE(hash4_equal (test_instance_one, test_instance_two));
}

TEST(hash8_equal, same_value)
{
	struct hash8 test_instance_one{};
	test_instance_one.value = 24;

	struct hash8 test_instance_two{};
	test_instance_two.value = 24;

	ASSERT_TRUE(hash8_equal (test_instance_one, test_instance_two));
}

TEST(hash8_equal, different_value)
{
	struct hash8 test_instance_one{};
	test_instance_one.value = 24;

	struct hash8 test_instance_two{};
	test_instance_two.value = 42;

	ASSERT_FALSE(hash8_equal (test_instance_one, test_instance_two));
}

TEST(hash_fn_sip_8, ok)
{
	auto test_instance = hash_fn_sip_8 (seed_1, seed_2);

	struct hash8 result = hash8_of (test_instance, bytes);
	ASSERT_EQ(5589350328979318521, result.value);

}

TEST(hash_fn_murmur_3, ok)
{
	auto test_instance = hash_fn_murmur_3 (seed_1);

	struct hash8 result = hash8_of (test_instance, bytes);
	ASSERT_EQ(1902106830578520072, result.value);
}

TEST(hash_fn_crc4, ok)
{
	auto test_instance = hash_fn_crc4 ();

	struct hash4 result = hash4_of (test_instance, bytes);
	ASSERT_EQ(3168879362, result.value);
}

TEST(hash4_of_hashes, single)
{
	auto test_instance = hash_fn_crc4 ();

	struct hash4 hashes[1];
	hashes[0].value = 2810;

	auto result = hash4_of_hashes (test_instance, hashes, 1);
	ASSERT_EQ(3305679881, result.value);
}

TEST(hash4_of_hashes, multiple)
{
	auto test_instance = hash_fn_crc4 ();

	struct hash4 hashes[3];
	hashes[0].value = 2810;
	hashes[1].value = 1212;
	hashes[2].value = 1506;

	auto result = hash4_of_hashes (test_instance, hashes, 3);
	ASSERT_EQ(946549821, result.value);
}

TEST(hash8_of_hashes, single)
{
	auto test_instance = hash_fn_murmur_3 (128);

	struct hash8 hashes[1];
	hashes[0].value = 2810;

	auto result = hash8_of_hashes (test_instance, hashes, 1);
	ASSERT_EQ(15845007680257927770LLU, result.value);
}

TEST(hash8_of_hashes, multiple)
{
	auto test_instance = hash_fn_murmur_3 (128);

	struct hash8 hashes[3];
	hashes[0].value = 2810;
	hashes[1].value = 1212;
	hashes[2].value = 1506;

	auto result = hash8_of_hashes (test_instance, hashes, 3);
	ASSERT_EQ(12014013340162161763LLU, result.value);
}