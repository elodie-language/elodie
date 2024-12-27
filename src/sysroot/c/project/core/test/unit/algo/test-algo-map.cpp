#include "../unit-test.h"

#include "core/algo/algo-map.h"

static u1 some_data[] = {0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08};

TEST(map_key_from_bytes, ok)
{
	struct map_config config = {
		.mem = MEM(mem_raw_new ()),
		.key_hash_fn = hash_fn_murmur_3 (0),
	};

	struct map test_instance{};
	map_init (&test_instance, config);

	struct bytes_view bytes = {.data = some_data, .size = 5};
	auto result = map_key_from_bytes (&test_instance, bytes);
	ASSERT_EQ(5655619333011295838, result.hash.value);
}

TEST(map_key_from_size_t, ok)
{
	struct map_config config = {
		.mem = MEM(mem_raw_new ()),
		.key_hash_fn = hash_fn_identity_8 (),
	};

	struct map test_instance{};
	map_init (&test_instance, config);

	auto result = map_key_from_size_t (&test_instance, 42);
	ASSERT_EQ(43, result.hash.value);

	auto another_result = map_key_from_size_t (&test_instance, 1337);
	ASSERT_EQ(1338, another_result.hash.value);
}

TEST(map_new, ok)
{
	auto tm = mem_test_new_default (1024);

	struct map_config config = {
		.mem = MEM(tm),
		.initial_capacity = 16,
		.key_hash_fn = hash_fn_murmur_3 (0),
	};

	struct map *result = map_new (config);
	ASSERT_TRUE(result != nullptr);
	ASSERT_EQ(0, map_count (result));
	ASSERT_EQ(16, map_capacity (result));

	map_free_safe (&result);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(map_set, ok)
{
	auto tm = mem_test_new_default (1024);

	struct map_config config = {
		.mem = MEM(tm),
		.initial_capacity = 8,
		.key_hash_fn = hash_fn_identity_8 (),
	};

	struct map *test_instance = map_new (config);

	for (size_t idx = 0; idx < 6; idx++)
		{
			auto key = map_key_from_size_t (test_instance, idx + 1);

			struct bytes_view value = {
				.data = some_data,
				.size = 9
			};

			bool inserted = map_set_bytes_view (test_instance, key, value);
			ASSERT_TRUE(inserted);
			ASSERT_EQ(idx + 1, map_count (test_instance));
			ASSERT_EQ(8, map_capacity (test_instance));
		}

	map_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(map_keys_iterator, ok)
{
	auto tm = mem_test_new_default (1024);

	struct map_config config = {
		.mem = MEM(tm),
		.initial_capacity = 8,
		.key_hash_fn = hash_fn_identity_8 (),
	};

	struct map *test_instance = map_new (config);
	for (size_t idx = 0; idx < 6; idx++)
		{
			auto key = map_key_from_size_t (test_instance, idx + 1);
			struct bytes_view value = {
				.data = some_data,
				.size = 9
			};
			map_set_bytes_view (test_instance, key, value);
		}

	auto keys_iterator = map_keys_iterator (test_instance);
	for (size_t idx = 0; idx < 6; idx++)
		{
			ASSERT_TRUE(iterator_has_next (&keys_iterator));
			auto key = static_cast<map_key *>(iterator_next (&keys_iterator));
			ASSERT_EQ(idx + 2, key->hash.value);
		}
	ASSERT_FALSE(iterator_has_next (&keys_iterator));

	map_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(map_set, already_exists)
{
	auto tm = mem_test_new_default (1024);

	struct map_config config = {
		.mem = MEM(tm),
		.initial_capacity = 8,
		.key_hash_fn = hash_fn_murmur_3 (0),
	};

	struct map *test_instance = map_new (config);

	auto key = MAP_KEY(test_instance, "some_key");

	uint8_t different_data[] = {0x42};
	struct bytes_view different_value = {
		.data = different_data,
		.size = 1
	};

	bool inserted = map_set_bytes_view (test_instance, key, different_value);
	ASSERT_TRUE(inserted);
	ASSERT_EQ(1, map_count (test_instance));

	inserted = map_set_bytes_view (test_instance, key, different_value);
	ASSERT_FALSE(inserted);
	ASSERT_EQ(1, map_count (test_instance));

	struct map_entry_view result{};
	bool found = map_get_as_entry_view (test_instance, key, &result);
	ASSERT_TRUE(found);
	ASSERT_EQ(14868071397190093659LU, result.key.hash.value);
	ASSERT_EQ(1, result.value.size);
	ASSERT_EQ(0x42, result.value.data[0]);

	map_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(map_set, resize)
{
	auto tm = mem_test_new_default (2 << 13);

	struct map_config config = {
		.mem = MEM(tm),
		.initial_capacity = 8,
		.key_hash_fn = hash_fn_identity_8 (),
	};

	struct map *test_instance = map_new (config);
	for (size_t idx = 0; idx < 64; idx++)
		{
			struct bytes_view some_data_view = {
				.data = (u1 *)&idx,
				.size = 1
			};
			ASSERT_TRUE(map_set_bytes_view (test_instance, MAP_KEY (test_instance, idx + 1), some_data_view));
			ASSERT_EQ(idx + 1, map_count (test_instance));
		}
	ASSERT_EQ(128, map_capacity (test_instance));

	for (size_t idx = 0; idx < 64; idx++)
		{
			struct bytes_view result{};
			ASSERT_TRUE(map_get_as_bytes_view (test_instance, MAP_KEY (test_instance, idx + 1), &result));
			ASSERT_EQ(idx, *(u1 *)result.data);
		}

	map_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(map_get_as_entry_view, ok)
{
	auto tm = mem_test_new_default (512);

	struct map_config config = {
		.mem = MEM(tm),
		.initial_capacity = 8,
		.key_hash_fn = hash_fn_murmur_3 (0),
	};

	struct map *test_instance = map_new (config);

	auto key = MAP_KEY(test_instance, "key");
	char const *test_str = "Hamal Rocks";

	map_set_bytes_view (test_instance, key, bytes_view_of_c_str (test_str));

	struct map_entry_view result{};
	bool found = map_get_as_entry_view (test_instance, key, &result);
	ASSERT_TRUE(found);
	ASSERT_EQ(6938685063292419814L, result.key.hash.value);
	ASSERT_EQ(11, result.value.size);

	char data[11];
	memcpy (&data, result.value.data, result.value.size);
	ASSERT_TRUE(strncmp (test_str, data, 11) == 0);

	map_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(map_get_as_bytes_view, ok)
{
	auto tm = mem_test_new_default (512);

	struct map_config config = {
		.mem = MEM(tm),
		.initial_capacity = 8,
		.key_hash_fn = hash_fn_murmur_3 (0),
	};

	struct map *test_instance = map_new (config);

	auto key = MAP_KEY(test_instance, "key");
	char const *test_str = "Hamal Rocks";

	map_set_bytes_view (test_instance, key, bytes_view_of_c_str (test_str));

	struct bytes_view result{};
	bool found = map_get_as_bytes_view (test_instance, key, &result);
	ASSERT_TRUE(found);
	ASSERT_EQ(11, result.size);

	char data[11];
	memcpy (&data, result.data, result.size);
	ASSERT_TRUE(strncmp (test_str, data, 11) == 0);

	map_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(map_copy, ok)
{
	auto tm = mem_test_new_default (1024);

	struct map_config config = {
		.mem = MEM(tm),
		.initial_capacity = 8,
		.key_hash_fn = hash_fn_murmur_3 (0),
	};

	auto test_instance = map_new (config);
	map_set_bytes_view (test_instance, MAP_KEY(test_instance, "key"), bytes_view_of_c_str ("Hamal Rocks"));
	ASSERT_EQ(259, mem_test_size (tm));

	auto result = map_copy (test_instance, MEM(tm));
	ASSERT_NE(test_instance, result);

	struct map_entry_view test_instance_entry_view{};
	ASSERT_TRUE(map_get_as_entry_view (result, MAP_KEY (test_instance, "key"), &test_instance_entry_view));
	struct map_entry_view copy_entry_view{};
	ASSERT_TRUE(map_get_as_entry_view (result, MAP_KEY (test_instance, "key"), &copy_entry_view));

	ASSERT_EQ(test_instance_entry_view.key.hash.value, copy_entry_view.key.hash.value);
	ASSERT_EQ(test_instance_entry_view.value.size, copy_entry_view.value.size);

	ASSERT_EQ(518, mem_test_size (tm));

	map_free_safe (&test_instance);
	map_free_safe (&result);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(map_copy_into, ok)
{
	auto tm = mem_test_new_default (1024);

	struct map_config config = {
		.mem = MEM(tm),
		.initial_capacity = 8,
		.key_hash_fn = hash_fn_murmur_3 (0),
	};

	auto test_instance = map_new (config);
	map_set_bytes_view (test_instance, MAP_KEY(test_instance, "key"), bytes_view_of_c_str ("Hamal Rocks"));
	ASSERT_EQ(259, mem_test_size (tm));

	auto target = map_new (config);
	map_copy_into (test_instance, target);
	ASSERT_NE(test_instance, target);

	struct map_entry_view test_instance_entry_view{};
	ASSERT_TRUE(map_get_as_entry_view (target, MAP_KEY (test_instance, "key"), &test_instance_entry_view));
	struct map_entry_view copy_entry_view{};
	ASSERT_TRUE(map_get_as_entry_view (target, MAP_KEY (test_instance, "key"), &copy_entry_view));

	ASSERT_EQ(test_instance_entry_view.key.hash.value, copy_entry_view.key.hash.value);
	ASSERT_EQ(test_instance_entry_view.value.size, copy_entry_view.value.size);

	ASSERT_EQ(518, mem_test_size (tm));

	map_free_safe (&test_instance);
	map_free_safe (&target);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(map_get_as_entry_view, not_found)
{
	auto tm = mem_test_new_default (512);

	struct map_config config = {
		.mem = MEM(tm),
		.initial_capacity = 8,
		.key_hash_fn = hash_fn_identity_8 (),
	};

	struct map *test_instance = map_new (config);

	auto key = map_key_from_size_t (test_instance, 42);
	char const *test_str = "Hamal Rocks";
	map_set_bytes_view (test_instance, key, bytes_view_of_c_str (test_str));

	struct map_entry_view result{};
	auto another_key = map_key_from_size_t (test_instance, 1337);

	bool found = map_get_as_entry_view (test_instance, another_key, &result);
	ASSERT_FALSE(found);

	map_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(map_remove, ok)
{
	auto tm = mem_test_new_default (512);

	struct map_config config = {
		.mem = MEM(tm),
		.initial_capacity = 8,
		.key_hash_fn = hash_fn_identity_8 (),
	};

	struct map *test_instance = map_new (config);

	auto key = map_key_from_size_t (test_instance, 42);
	char const *test_str = "Hamal Rocks";
	map_set_bytes_view (test_instance, key, bytes_view_of_c_str (test_str));

	bool deleted = map_remove (test_instance, key);
	ASSERT_TRUE(deleted);
	ASSERT_EQ(0, map_count (test_instance));

	struct map_entry_view result{};
	bool found = map_get_as_entry_view (test_instance, key, &result);
	ASSERT_FALSE(found);

	map_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(map_remove, does_not_exists)
{
	auto tm = mem_test_new_default (512);

	struct map_config config = {
		.mem = MEM(tm),
		.initial_capacity = 8,
		.key_hash_fn = hash_fn_identity_8 (),
	};

	struct map *test_instance = map_new (config);
	auto key = map_key_from_size_t (test_instance, 42);

	bool deleted = map_remove (test_instance, key);
	ASSERT_FALSE(deleted);
	ASSERT_EQ(0, map_count (test_instance));

	map_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(map_remove, twice)
{
	auto tm = mem_test_new_default (512);

	struct map_config config = {
		.mem = MEM(tm),
		.initial_capacity = 8,
		.key_hash_fn = hash_fn_murmur_3 (0),
	};

	struct map *test_instance = map_new (config);

	auto key = map_key_from_size_t (test_instance, 42);
	map_set_bytes_view (test_instance, key, bytes_view_of_c_str ("Chuobaka"));

	bool deleted = map_remove (test_instance, key);
	ASSERT_TRUE(deleted);
	ASSERT_EQ(0, map_count (test_instance));

	deleted = map_remove (test_instance, key);
	ASSERT_FALSE(deleted);

	struct map_entry_view result{};
	bool found = map_get_as_entry_view (test_instance, key, &result);
	ASSERT_FALSE(found);

	map_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(map_remove, delete_and_reinsert)
{
	auto tm = mem_test_new_default (512);

	struct map_config config = {
		.mem = MEM(tm),
		.initial_capacity = 8,
		.key_hash_fn = hash_fn_identity_8 (),
	};

	struct map *test_instance = map_new (config);

	auto key = map_key_from_size_t (test_instance, 42);
	map_set_bytes_view (test_instance, key, bytes_view_of_c_str ("A"));

	bool deleted = map_remove (test_instance, key);
	ASSERT_TRUE(deleted);
	ASSERT_EQ(0, map_count (test_instance));

	bool inserted = map_set_bytes_view (test_instance, key, bytes_view_of_c_str ("B"));
	ASSERT_TRUE(inserted);
	ASSERT_EQ(1, map_count (test_instance));

	struct map_entry_view result{};
	bool found = map_get_as_entry_view (test_instance, key, &result);
	ASSERT_TRUE(found);

	map_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(map_free_safe, ok)
{
	auto tm = mem_test_new_default (1024);

	struct map_config config = {
		.mem = MEM(tm),
		.initial_capacity = 16,
		.key_hash_fn = hash_fn_murmur_3 (0),
	};

	struct map *test_instance = map_new (config);
	map_free_safe (&test_instance);
	ASSERT_TRUE(test_instance == nullptr);

	mem_test_verify (tm);
	mem_test_free (tm);
}
