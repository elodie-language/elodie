#include "../unit-test.h"

#include "core/mem/mem.h"
#include "core/algo/algo-ring-buffer.h"

TEST(ring_buffer_new, ok)
{
	auto tm = mem_test_new_default (512);

	auto test_instance = ring_buffer_new(MEM (tm), 32, size_t);
	ASSERT_EQ(0, ring_buffer_count (test_instance));
	ASSERT_EQ(32, ring_buffer_capacity (test_instance));

	ring_buffer_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(ring_buffer, ok)
{
	auto tm = mem_test_new_default (512);

	struct magic {
	  u1 val;
	  u2 biggerVal;
	  u4 evenBiggerVal;
	  u8 bigboy;
	};

	auto test_instance = ring_buffer_new(MEM (tm), 5, struct magic);
	for (size_t idx = 0; idx < 5; idx++)
		{
			struct magic val{
				.val = static_cast<u1>(idx),
				.biggerVal = static_cast<u2>(idx * idx),
				.evenBiggerVal = static_cast<u4>(idx * idx * idx),
				.bigboy = static_cast<u8>(idx * idx * idx * idx),
			};
			ring_buffer_append (test_instance, &val);
		}
	ASSERT_TRUE(ring_buffer_is_full (test_instance));

	for (size_t idx = 0; idx < 5; idx++)
		{
			struct magic d = *(struct magic *)ring_buffer_at (test_instance, idx);

			ASSERT_EQ(idx, d.val);
			ASSERT_EQ(idx * idx, d.biggerVal);
			ASSERT_EQ(idx * idx * idx, d.evenBiggerVal);
			ASSERT_EQ(idx * idx * idx * idx, d.bigboy);
		}

	ASSERT_EQ(5, ring_buffer_count (test_instance));
	ASSERT_EQ(5, ring_buffer_capacity (test_instance));
	ASSERT_TRUE(ring_buffer_is_full (test_instance));

	ASSERT_EQ(128, mem_test_size (tm));

	ring_buffer_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(ring_buffer_append, overflow)
{
	auto tm = mem_test_new_default (2048);

	auto test_instance = ring_buffer_new(MEM (tm), 10, size_t);
	for (size_t idx = 0; idx < 100; idx++)
		{
			ring_buffer_append_rval(test_instance, idx);
		}

	for (size_t idx = 0; idx < 10; idx++)
		{
			size_t val = *(size_t *)ring_buffer_at (test_instance, idx);
			ASSERT_EQ(90 + idx, val);
		}

	ring_buffer_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(ring_buffer_at, ok)
{
	auto tm = mem_test_new_default (2048);

	auto test_instance = ring_buffer_new(MEM (tm), 100, size_t);
	for (size_t idx = 0; idx < 100; idx++)
		{
			ring_buffer_append_rval(test_instance, idx);
		}

	for (size_t idx = 0; idx < 100; idx++)
		{
			size_t val = *(size_t *)ring_buffer_at (test_instance, idx);
			ASSERT_EQ(idx, val);
		}

	ring_buffer_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(ring_buffer_iterator, not_full)
{
	auto tm = mem_test_new_default (2048);

	auto test_instance = ring_buffer_new(MEM (tm), 128, size_t);
	for (size_t idx = 0; idx < 100; idx++)
		{
			ring_buffer_append_rval(test_instance, idx);
		}

	auto it = ring_buffer_iterator (test_instance);
	for (size_t idx = 0; idx < 100; idx++)
		{
			ASSERT_TRUE(iterator_has_next (&it));
			ASSERT_EQ(idx, *(size_t *)iterator_next (&it));
		}
	ASSERT_FALSE(iterator_has_next (&it));

	ring_buffer_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(ring_buffer_iterator, full)
{
	auto tm = mem_test_new_default (2048);

	auto test_instance = ring_buffer_new(MEM (tm), 128, size_t);
	auto it = ring_buffer_iterator (test_instance);

	for (size_t idx = 0; idx < 192; idx++)
		{
			ring_buffer_append_rval(test_instance, idx);
		}

	/**
	 * current ring buffer layout
	 * [0: 128, 1: 129....  63:64, 64: 65]
	 * as half of the ring buffer was filled twice
	 */

	// first half
	for (size_t idx = 0; idx < 64; idx++)
		{
			ASSERT_TRUE(iterator_has_next (&it));
			ASSERT_EQ(128 + idx, *(size_t *)iterator_next (&it));
		}

	// second half
	for (size_t idx = 0; idx < 64; idx++)
		{
			ASSERT_TRUE(iterator_has_next (&it));
			ASSERT_EQ(64 + idx, *(size_t *)iterator_next (&it));
		}
	ASSERT_FALSE(iterator_has_next (&it));

	ring_buffer_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(ring_buffer_free_safe, ok)
{
	auto tm = mem_test_new_default (1024);

	auto test_instance = ring_buffer_new(MEM (tm), 128, u1);

	for (size_t idx = 0; idx < 10; idx++)
		{
			ring_buffer_append_rval(test_instance, idx);
		}

	ring_buffer_free_safe (&test_instance);
	ASSERT_TRUE(test_instance == nullptr);

	mem_test_verify (tm);
	mem_test_free (tm);
}