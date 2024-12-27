#include "../unit-test.h"
#include "core/algo/algo-map-ptr.h"
#include "core/val/val-num.h"

TEST(ptr_map_key_from_bytes, ok)
{
	static u1 some_data[] = {0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08};

	struct ptr_map_config config = {
		.mem = MEM(mem_raw_new ()),
		.key_hash_fn = hash_fn_murmur_3 (0),
	};

	struct ptr_map test_instance{};
	ptr_map_init (&test_instance, config);

	struct bytes_view bytes = {.data = some_data, .size = 5};
	auto result = ptr_map_key_from_bytes (&test_instance, bytes);
	ASSERT_EQ(5655619333011295838, result.underlying_key.hash.value);
}

TEST(ptr_map_key_from_size_t, ok)
{
	struct ptr_map_config config = {
		.mem = MEM(mem_raw_new ()),
		.key_hash_fn = hash_fn_identity_8 (),
	};

	struct ptr_map test_instance{};
	ptr_map_init (&test_instance, config);

	auto result = ptr_map_key_from_size_t (&test_instance, 42);
	ASSERT_EQ(43, result.underlying_key.hash.value);

	auto another_result = ptr_map_key_from_size_t (&test_instance, 1337);
	ASSERT_EQ(1338, another_result.underlying_key.hash.value);
}

TEST(ptr_map_new, ok)
{
	auto tm = mem_test_new_default (1024);

	struct ptr_map_config config = {
		.mem = MEM(tm),
		.initial_capacity = 8,
		.key_hash_fn = hash_fn_identity_8 (),
	};
	config.initial_capacity = 32;

	auto test_instance = ptr_map_new (config);
	ASSERT_EQ(0, ptr_map_count (test_instance));
	ASSERT_EQ(32, ptr_map_capacity (test_instance));
	ASSERT_EQ(MEM (tm), test_instance->underlying_map.mem);

	ptr_map_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(ptr_map, ok)
{
	auto tm = mem_test_new_default (1024);

	struct ptr_map_config config = {
		.mem = MEM(tm),
		.initial_capacity = 8,
		.key_hash_fn = hash_fn_identity_8 (),
	};
	auto test_instance = ptr_map_new (config);

	for (size_t idx = 0; idx < 2; idx++)
		{
			auto key = ptr_map_key_from_size_t (test_instance, idx);
			auto some_number = val_num_new_from_double (MEM(tm), (double)idx);
			ASSERT_TRUE(ptr_map_set (test_instance, key, some_number));
			ASSERT_EQ(idx + 1, ptr_map_count (test_instance));
		}

	for (size_t idx = 0; idx < 2; idx++)
		{
			auto key = ptr_map_key_from_size_t (test_instance, idx);

			struct val_num *result = nullptr;
			ASSERT_TRUE(ptr_map_get (test_instance, key, reinterpret_cast<void **>(&result)));
			ASSERT_EQ(idx, result->data);
			val_num_free_safe (&result);
		}

	ASSERT_EQ(2, ptr_map_count (test_instance));
	ASSERT_EQ(8, ptr_map_capacity (test_instance));

	ptr_map_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(ptr_map, resize)
{
	auto tm = mem_test_new_default (1024);

	struct ptr_map_config config = {
		.mem = MEM(tm),
		.initial_capacity = 8,
		.key_hash_fn = hash_fn_identity_8 (),
	};
	auto test_instance = ptr_map_new (config);

	for (size_t idx = 0; idx < 10; idx++)
		{
			auto key = ptr_map_key_from_size_t (test_instance, idx);
			auto some_number = val_num_new_from_double (MEM(tm), (double)idx);
			ASSERT_TRUE(ptr_map_set (test_instance, key, some_number));
			ASSERT_EQ(idx + 1, ptr_map_count (test_instance));
		}

	for (size_t idx = 0; idx < 10; idx++)
		{
			auto key = ptr_map_key_from_size_t (test_instance, idx);

			struct val_num *result = nullptr;
			ASSERT_TRUE(ptr_map_get (test_instance, key, reinterpret_cast<void **>(&result)));
			ASSERT_EQ(idx, result->data);
			val_num_free_safe (&result);
		}

	ASSERT_EQ(10, ptr_map_count (test_instance));
	ASSERT_EQ(16, ptr_map_capacity (test_instance));

	ptr_map_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(ptr_map_remove, ok)
{
	auto tm = mem_test_new_default (1024);

	struct ptr_map_config config = {
		.mem = MEM(tm),
		.initial_capacity = 8,
		.key_hash_fn = hash_fn_identity_8 (),
	};
	auto test_instance = ptr_map_new (config);

	for (size_t idx = 0; idx < 10; idx++)
		{
			auto key = ptr_map_key_from_size_t (test_instance, idx);
			ptr_map_set (test_instance, key, val_num_new_from_double (MEM(tm), (double)idx));
		}

	for (size_t idx = 0; idx < 10; idx++)
		{
			auto key = ptr_map_key_from_size_t (test_instance, idx);

			struct val_num *result = nullptr;
			ASSERT_TRUE(ptr_map_get (test_instance, key, reinterpret_cast<void **>(&result)));
			ASSERT_TRUE(ptr_map_remove (test_instance, key));

			struct val_num *null_result = nullptr;
			ASSERT_FALSE(ptr_map_get (test_instance, key, reinterpret_cast<void **>(&null_result)));
			ASSERT_TRUE(null_result == nullptr);
			val_num_free_safe (&result);
		}

	ASSERT_EQ(0, ptr_map_count (test_instance));
	ASSERT_EQ(16, ptr_map_capacity (test_instance));

	ptr_map_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(ptr_map_free_safe, ok)
{
	auto tm = mem_test_new_default (1024);

	struct ptr_map_config config = {
		.mem = MEM(tm),
		.initial_capacity = 8,
		.key_hash_fn = hash_fn_identity_8 (),
	};

	auto test_instance = ptr_map_new (config);
	auto key = ptr_map_key_from_size_t (test_instance, 2810);
	auto some_number = val_num_new_from_double (MEM(tm), (double)2810);
	ptr_map_set (test_instance, key, some_number);

	ptr_map_free_safe (&test_instance);
	val_num_free_safe (&some_number);
	ASSERT_TRUE(test_instance == nullptr);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(ptr_map_free_safe, empty)
{
	auto tm = mem_test_new_default (1024);

	struct ptr_map_config config = {
		.mem = MEM(tm),
		.initial_capacity = 8,
		.key_hash_fn = hash_fn_identity_8 (),
	};

	auto test_instance = ptr_map_new (config);

	ptr_map_free_safe (&test_instance);
	ASSERT_TRUE(test_instance == nullptr);

	mem_test_verify (tm);
	mem_test_free (tm);
}



