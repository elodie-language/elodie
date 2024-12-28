#include "../unit-test.h"
#include "core/val/val-str.h"

TEST(mem_vape_init, ok)
{
	struct mem_vape_config config = {
		.size = 32,
		.root = MEM(mem_raw_new ()),
	};

	struct mem_vape test_instance{};

	mem_vape_init (&test_instance, config);
	ASSERT_TRUE(test_instance.end != nullptr);
	ASSERT_TRUE(test_instance.start != nullptr);
	ASSERT_EQ(test_instance.start, test_instance.current);
	ASSERT_EQ(static_cast<u1 *>(test_instance.start) + 32, test_instance.end);
}

TEST(mem_vape_new, ok)
{
	struct mem_vape_config config = {
		.size = 32,
		.root = MEM(mem_raw_new ())
	};

	auto result = mem_vape_new (config);
	ASSERT_TRUE(result != nullptr);
	mem_vape_free (result);
}

TEST(mem_vape_allocate, ok)
{
	struct mem_vape_config config = {
		.size = 32,
		.root = MEM(mem_raw_new ()),
	};

	struct mem_vape test_instance{};
	mem_vape_init (&test_instance, config);
	void *start = test_instance.start;
	void *end = test_instance.end;

	mem_vape_allocate (&test_instance, 16);
	ASSERT_EQ(start, test_instance.start);
	ASSERT_EQ(static_cast<u1 *>(start) + 16, test_instance.current);
	ASSERT_EQ(end, test_instance.end);
}

TEST(mem_vape_resolve, ok)
{
	struct mem_vape_config config = {
		.size = 64,
		.root = MEM(mem_raw_new ())
	};

	auto test_instance = mem_vape_new (config);
	auto next_ref = mem_ref_generator_next (test_instance->base.ref_generator, VAL_KIND_STR);

	auto some_string = (struct val_str *)mem_vape_allocate (test_instance, sizeof (struct val_str));
	some_string->base.kind = VAL_KIND_STR;
	some_string->count = 6;
	some_string->data = (char *)mem_vape_allocate (test_instance, 6);
	memcpy (some_string->data, "ELODIE\0", 6);

	auto result = (struct val_str *)mem_vape_resolve (test_instance, next_ref);
	ASSERT_TRUE(result != nullptr);
	ASSERT_EQ(VAL_KIND_STR, result->base.kind);
	ASSERT_EQ(6, result->count);
	ASSERT_TRUE(strncmp (some_string->data, "ELODIE", 6) == 0);

	auto result2 = (struct val_str *)mem_resolve (MEM(test_instance), next_ref);
	ASSERT_EQ(result, result2);

	mem_vape_free (test_instance);
}

TEST(mem_vape_next_reference, empty)
{
	struct mem_vape_config config = {
		.size = 32,
		.root = MEM(mem_raw_new ())
	};

	auto test_instance = mem_vape_new (config);

	auto result = mem_ref_generator_next (test_instance->base.ref_generator, VAL_KIND_FN);
	ASSERT_EQ(VAL_KIND_FN, result.kind);
	ASSERT_EQ(test_instance->base.realm, result.realm);
	ASSERT_EQ(0, result.value);

	mem_vape_free (test_instance);
}

TEST(mem_vape_next_reference, ok)
{
	struct mem_vape_config config = {
		.size = 32,
		.root = MEM(mem_raw_new ())
	};

	auto test_instance = mem_vape_new (config);
	mem_vape_allocate (test_instance, 4);

	auto next_ref = mem_ref_generator_next (test_instance->base.ref_generator, VAL_KIND_FN);
	ASSERT_EQ(VAL_KIND_FN, next_ref.kind);
	ASSERT_EQ(test_instance->base.realm, next_ref.realm);
	ASSERT_EQ(0, next_ref.value);

	mem_vape_allocate (test_instance, 7);

	next_ref = mem_ref_generator_next (test_instance->base.ref_generator, VAL_KIND_FN);
	ASSERT_EQ(VAL_KIND_FN, next_ref.kind);
	ASSERT_EQ(test_instance->base.realm, next_ref.realm);
	ASSERT_EQ(0, next_ref.value);

	mem_vape_free (test_instance);
}

TEST(mem_vape_allocate, would_overflow_but_resetted)
{
	struct mem_vape_config config = {
		.size = 32,
		.root = MEM(mem_raw_new ()),
	};

	struct mem_vape test_instance{};
	mem_vape_init (&test_instance, config);
	void *start = test_instance.start;
	void *end = test_instance.end;

	mem_vape_allocate (&test_instance, 30);
	mem_vape_reset (&test_instance);

	mem_vape_allocate (&test_instance, 16);
	ASSERT_EQ(start, test_instance.start);
	ASSERT_EQ(static_cast<u1 *>(start) + 16, test_instance.current);
	ASSERT_EQ(end, test_instance.end);
}

TEST(mem_vape, full_flow_ok)
{
	struct mem_vape_config config = {
		.size = 32,
		.root = MEM(mem_raw_new ())
	};

	auto test_instance = mem_vape_new (config);
	for (size_t outer = 0; outer < 10; outer++)
		{
			for (size_t idx = 0; idx < 8; idx++)
				{
					mem_vape_allocate (test_instance, 4);
					ASSERT_EQ((idx + 1) * 4, mem_vape_size (test_instance));
				}
		}
	mem_vape_free (test_instance);
}
