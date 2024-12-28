#include "../unit-test.h"

#include "core/mem/mem.h"

TEST(mem_test_new, ok)
{
	struct mem_test_config config = {
		.size = 32,
		.root = MEM(mem_raw_new ())
	};

	struct mem_test *result = mem_test_new (config);
	ASSERT_TRUE(result != nullptr);

	mem_test_free (result);
}

TEST(mem_test, full_flow_ok)
{
	struct mem_test_config config = {
		.size = 32,
		.root =MEM(mem_raw_new ())
	};

	struct mem_test *test_instance = mem_test_new (config);
	for (size_t idx = 0; idx < 8; idx++)
		{
			mem_test_allocate (test_instance, 4);
		}
	mem_test_free (test_instance);
}

TEST(mem_test, runs_out_of_mem)
{
	struct mem_test_config config = {
		.size = 32,
		.root = MEM(mem_raw_new ())
	};

	struct mem_test *test_instance = mem_test_new (config);
	testing::internal::CaptureStdout ();

	ASSERT_DEATH(
		{
			mem_test_allocate (test_instance, 48);
		},
        ""
	);

	ASSERT_EQ(
    "\x1B[31mFATAL\x1B[0m \x1B[90mmem-test.c:115\x1B[0m \x1B[31mcheck '(u1 *)self->start + size <= (u1 *)self->end' failed\n",
    captured_output()
	);
}

TEST(mem_test_init, ok)
{
	struct mem_test_config config = {
		.size = 32,
		.root = MEM(mem_raw_new ()),
	};

	struct mem_test test_instance{};
	mem_test_init (&test_instance, config);
	ASSERT_TRUE(test_instance.end != nullptr);
	ASSERT_TRUE(test_instance.start != nullptr);
	ASSERT_EQ(static_cast<u1 *>(test_instance.start) + 32, test_instance.end);
}

TEST(mem_test_allocate, ok)
{
	struct mem_test_config config = {
		.size = 32,
		.root =MEM(mem_raw_new ()),
	};

	struct mem_test test_instance{};
	mem_test_init (&test_instance, config);
	void *start = test_instance.start;
	void *end = test_instance.end;

	mem_test_allocate (&test_instance, 16);

	ASSERT_EQ(static_cast<u1 *>(start) + 16, test_instance.start);
	ASSERT_EQ(end, test_instance.end);
}

TEST(mem_test_deallocate, ok)
{
	struct mem_test_config config = {
		.size = 32,
		.root = MEM(mem_raw_new ()),
	};

	struct mem_test test_instance{};
	mem_test_init (&test_instance, config);
	void *start = test_instance.start;
	void *end = test_instance.end;

	void *ptr = mem_test_allocate (&test_instance, 16);
	mem_test_deallocate (&test_instance, ptr);

	ASSERT_EQ(static_cast<u1 *>(start) + 16, test_instance.start);
	ASSERT_EQ(end, test_instance.end);
}

TEST(mem_test_deallocate, deallocated_twice)
{
	struct mem_test_config config = {
		.size = 32,
		.root = MEM(mem_raw_new ()),
	};

	struct mem_test test_instance{};
	mem_test_init (&test_instance, config);

	void *ptr = mem_test_allocate (&test_instance, 16);
	mem_test_deallocate (&test_instance, ptr);

	EXPECT_EXIT(
		{
			mem_test_deallocate (&test_instance, ptr);
		},
		testing::KilledBySignal(SIGABRT),
		""
	);
}

TEST(mem_test_deallocate, not_allocated)
{
	struct mem_test_config config = {
		.size = 32,
		.root = MEM(mem_raw_new ()),
	};

	struct mem_test test_instance{};
	mem_test_init (&test_instance, config);

	EXPECT_EXIT(
		{
			mem_test_deallocate (&test_instance, nullptr);
		},
		testing::KilledBySignal(SIGABRT),
		""
	);
}

TEST(mem_test_verify, ok)
{
	struct mem_test_config config = {
		.size = 128,
		.root =MEM(mem_raw_new ()),
	};

	struct mem_test test_instance{};
	mem_test_init (&test_instance, config);

	void *ptr_one = mem_test_allocate (&test_instance, 16);
	void *ptr_two = mem_test_allocate (&test_instance, 16);
	void *ptr_three = mem_test_allocate (&test_instance, 16);
	mem_test_deallocate (&test_instance, ptr_three);
	mem_test_deallocate (&test_instance, ptr_one);
	mem_test_deallocate (&test_instance, ptr_two);

	mem_test_verify (&test_instance);
}

TEST(mem_test_verify, not_deallocated)
{
	struct mem_test_config config = {
		.size = 32,
		.root = MEM(mem_raw_new ()),
	};

	struct mem_test test_instance{};
	mem_test_init (&test_instance, config);
	mem_test_allocate (&test_instance, 16);

	EXPECT_EXIT(
		{
			mem_test_verify (&test_instance);
		},
		testing::KilledBySignal(SIGABRT),
		""
	);
}