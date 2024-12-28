#include "../unit-test.h"
#include "core/mem/mem.h"
#include "core/val/val-str.h"

TEST(mem_stack_init, ok)
{
	struct mem_stack_config config = {
		.size = 32,
		.root = MEM(mem_raw_new ()),
	};

	struct mem_stack test_instance{};

	mem_stack_init (&test_instance, config);
	ASSERT_EQ(test_instance.stack_idx, 0);
	ASSERT_TRUE(test_instance.end != nullptr);
	ASSERT_TRUE(test_instance.current != nullptr);
	ASSERT_EQ(static_cast<u1 *>(test_instance.current) + 32, test_instance.end);
}

TEST(mem_stack_push, ok)
{
	struct mem_stack_config config = {
		.size = 32,
		.root = MEM(mem_raw_new ()),
	};

	struct mem_stack test_instance{};
	mem_stack_init (&test_instance, config);
	void *end = test_instance.end;

	mem_stack_push (&test_instance);
	ASSERT_EQ(test_instance.stack_idx, 1);
	ASSERT_EQ(test_instance.stack[0], test_instance.current);
	ASSERT_EQ(test_instance.end, end);
	ASSERT_EQ(mem_stack_size (&test_instance), 0);

	mem_stack_allocate (&test_instance, 10);
	mem_stack_push (&test_instance);
	ASSERT_EQ(test_instance.stack_idx, 2);
	ASSERT_EQ(test_instance.end, end);
	ASSERT_EQ(test_instance.stack[0], static_cast<u1 *>(test_instance.current) - 10);
	ASSERT_EQ(test_instance.stack[1], static_cast<u1 *>(test_instance.current));
	ASSERT_EQ(mem_stack_size (&test_instance), 10);
}

TEST(mem_stack_pop, ok)
{
	struct mem_stack_config config = {
		.size = 32,
		.root = MEM(mem_raw_new ()),
	};

	struct mem_stack test_instance{};
	mem_stack_init (&test_instance, config);
	void *start = test_instance.current;
	void *end = test_instance.end;

	mem_stack_push (&test_instance);
	mem_stack_allocate (&test_instance, 10);
	mem_stack_push (&test_instance);
	mem_stack_pop (&test_instance);
	ASSERT_EQ(1, test_instance.stack_idx);
	ASSERT_EQ(end, test_instance.end);
	ASSERT_EQ(static_cast<u1 *>(start) + 10, test_instance.current);
	ASSERT_EQ(10, mem_stack_size (&test_instance));

	mem_stack_pop (&test_instance);
	ASSERT_EQ(0, test_instance.stack_idx);
	ASSERT_EQ(start, test_instance.current);
	ASSERT_EQ(end, test_instance.end);
	ASSERT_EQ(0, mem_stack_size (&test_instance));
}

TEST(mem_stack_allocate, ok)
{
	struct mem_stack_config config = {
		.size = 32,
		.root = MEM(mem_raw_new ()),
	};

	struct mem_stack test_instance{};
	mem_stack_init (&test_instance, config);
	void *start = test_instance.current;
	void *end = test_instance.end;

	mem_stack_allocate (&test_instance, 16);
	ASSERT_EQ(0, test_instance.stack_idx);
	ASSERT_EQ(static_cast<u1 *>(start) + 16, test_instance.current);
	ASSERT_EQ(end, test_instance.end);
}

TEST(mem_stack_new, ok)
{
	struct mem_stack_config config = {
		.size = 32,
		.root = MEM(mem_raw_new ())
	};

	auto result = mem_stack_new (config);
	ASSERT_TRUE(result != nullptr);
	mem_stack_free (result);
}

TEST(mem_stack_resolve, ok)
{
	struct mem_stack_config config = {
		.size = 64,
		.root = MEM(mem_raw_new ())
	};

	auto test_instance = mem_stack_new (config);
	auto next_ref = mem_ref_generator_next (test_instance->base.ref_generator, VAL_KIND_STR);

	auto some_string = (struct val_str *) mem_stack_allocate (test_instance, sizeof (struct val_str));
	some_string->base.kind = VAL_KIND_STR;
	some_string->count = 7;
	some_string->data = (char *)mem_stack_allocate (test_instance, 7);
	memcpy (some_string->data, "ELODIE\0", 7);

	auto result = (struct val_str *)mem_stack_resolve (test_instance, next_ref);
	ASSERT_TRUE(result != nullptr);
	ASSERT_EQ(VAL_KIND_STR, result->base.kind);
	ASSERT_EQ(7, result->count);
	ASSERT_TRUE(strncmp (some_string->data, "ELODIE", 7) == 0);

	auto result2 = (struct val_str *)mem_resolve (MEM(test_instance), next_ref);
	ASSERT_EQ(result, result2);

	mem_stack_free (test_instance);
}

TEST(mem_stack_next_reference, empty)
{
	struct mem_stack_config config = {
		.size = 32,
		.root = MEM(mem_raw_new ())
	};

	auto test_instance = mem_stack_new (config);

	auto result = mem_ref_generator_next (test_instance->base.ref_generator, VAL_KIND_FN);
	ASSERT_EQ(VAL_KIND_FN, result.kind);
	ASSERT_EQ(test_instance->base.realm, result.realm);
	ASSERT_EQ(0, result.value);

	mem_stack_free (test_instance);
}

TEST(mem_stack_next_reference, ok)
{
	struct mem_stack_config config = {
		.size = 32,
		.root = MEM(mem_raw_new ())
	};

	auto test_instance = mem_stack_new (config);
	mem_stack_allocate (test_instance, 4);

	auto next_ref = mem_ref_generator_next (test_instance->base.ref_generator, VAL_KIND_FN);
	ASSERT_EQ(VAL_KIND_FN, next_ref.kind);
	ASSERT_EQ(test_instance->base.realm, next_ref.realm);
	ASSERT_EQ(4, next_ref.value);

	mem_stack_allocate (test_instance, 7);

	next_ref = mem_ref_generator_next (test_instance->base.ref_generator, VAL_KIND_FN);
	ASSERT_EQ(VAL_KIND_FN, next_ref.kind);
	ASSERT_EQ(test_instance->base.realm, next_ref.realm);
	ASSERT_EQ(11, next_ref.value);

	mem_stack_free (test_instance);
}

TEST(mem_stack, full_flow_ok)
{
	struct mem_stack_config config = {
		.size = 32,
		.root = MEM(mem_raw_new ())
	};

	auto test_instance = mem_stack_new (config);
	for (size_t outer = 0; outer < 10; outer++)
		{
			mem_stack_push (test_instance);
			for (size_t idx = 0; idx < 8; idx++)
				{
					mem_stack_allocate (test_instance, 4);
					ASSERT_EQ((idx + 1) * 4, mem_stack_size (test_instance));
				}
			mem_stack_pop (test_instance);
		}
	mem_stack_free (test_instance);
}

TEST(mem_stack, runs_out_of_mem)
{
	struct mem_stack_config config = {
		.size = 32,
		.root = MEM(mem_raw_new ())
	};

	auto test_instance = mem_stack_new (config);
	testing::internal::CaptureStdout ();

	EXPECT_EXIT(
		mem_stack_allocate (test_instance, 48),
		testing::KilledBySignal(SIGABRT),
		""
	);

	ASSERT_EQ(
    "\x1B[31mFATAL\x1B[0m \x1B[90mmem-stack.c:102\x1B[0m \x1B[31mcheck '(u1 *)self->current + size <= (u1 *)"
		"self->end' failed\n",
    captured_output()
	);
}

TEST(mem_stack, too_many_begin_calls)
{
	struct mem_stack_config config = {
		.size = 32,
		.root = MEM(mem_raw_new ())
	};

	auto test_instance = mem_stack_new (config);
	testing::internal::CaptureStdout ();

	EXPECT_EXIT(
		{
			for (size_t idx = 0; idx < 257; idx++)
				{
					mem_stack_push (test_instance);
				}
		},
		testing::KilledBySignal(SIGABRT),
		""
	);

	ASSERT_EQ(
    "\x1B[31mFATAL\x1B[0m \x1B[90mmem-stack.c:92\x1B[0m \x1B[31mcheck 'self->stack_idx + 1 < 256' failed\n",
    captured_output()
	);
}