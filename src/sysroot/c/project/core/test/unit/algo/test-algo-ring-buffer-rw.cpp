#include "../unit-test.h"

#include "core/mem/mem.h"
#include "core/algo/algo-ring-buffer-rw.h"

TEST(rw_ring_buffer_new, ok)
{
	auto tm = mem_test_new_default (512);

	auto test_instance = rw_ring_buffer_new(MEM (tm), 32, size_t);
	ASSERT_EQ(0, rw_ring_buffer_count (test_instance));
	ASSERT_EQ(32, rw_ring_buffer_capacity (test_instance));
	ASSERT_EQ(0, rw_ring_buffer_read_position (test_instance));
	ASSERT_EQ(0, rw_ring_buffer_write_position (test_instance));
	ASSERT_FALSE(rw_ring_buffer_is_full (test_instance));

	rw_ring_buffer_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(rw_ring_buffer, ok)
{
	auto tm = mem_test_new_default (512);

	struct magic {
	  u1 val;
	  u2 biggerVal;
	  u4 evenBiggerVal;
	  u8 bigboy;
	};

	auto test_instance = rw_ring_buffer_new(MEM (tm), 5, struct magic);
	for (size_t idx = 0; idx < 5; idx++)
		{
			struct magic val{
				.val = static_cast<u1>(idx),
				.biggerVal = static_cast<u2>(idx * idx),
				.evenBiggerVal = static_cast<u4>(idx * idx * idx),
				.bigboy = static_cast<u8>(idx * idx * idx * idx),
			};

			rw_ring_buffer_append (test_instance, &val);
		}
	ASSERT_TRUE(rw_ring_buffer_is_full (test_instance));

	for (size_t multiple_iterations = 0; multiple_iterations < 10; multiple_iterations++)
		{
			for (size_t idx = 0; idx < 5; idx++)
				{
					ASSERT_EQ(0, rw_ring_buffer_write_position (test_instance));
					ASSERT_EQ(idx, rw_ring_buffer_read_position (test_instance));

					struct magic d = *(struct magic *)rw_ring_buffer_current (test_instance);

					ASSERT_EQ(idx, d.val);
					ASSERT_EQ(idx * idx, d.biggerVal);
					ASSERT_EQ(idx * idx * idx, d.evenBiggerVal);
					ASSERT_EQ(idx * idx * idx * idx, d.bigboy);

					rw_ring_buffer_next (test_instance);
				}
		}

	ASSERT_EQ(5, rw_ring_buffer_count (test_instance));
	ASSERT_EQ(5, rw_ring_buffer_capacity (test_instance));
	ASSERT_TRUE(rw_ring_buffer_is_full (test_instance));

	ASSERT_EQ(136, mem_test_size (tm));

	rw_ring_buffer_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(rw_ring_buffer, read_write_simulation)
{
	auto tm = mem_test_new_default (512);

	struct magic {
	  u1 val;
	  u2 biggerVal;
	  u4 evenBiggerVal;
	  u8 bigboy;
	};

	auto test_instance = rw_ring_buffer_new(MEM (tm), 10, u4);
	for (size_t idx = 0; idx < 10; idx++)
		{
			rw_ring_buffer_append_rval(test_instance, (u4)idx);
		}
	ASSERT_TRUE(rw_ring_buffer_is_full (test_instance));

	ASSERT_EQ(0, *(u4 *)rw_ring_buffer_current (test_instance));

	for (u4 idx = 0; idx < 10; idx++)
		{
			rw_ring_buffer_append_rval(test_instance, idx + 10);
			ASSERT_EQ(idx + 1, *(u4 *)rw_ring_buffer_next (test_instance));
		}

	for (u4 idx = 0; idx < 10; idx++)
		{
			ASSERT_EQ(idx + 11, *(u4 *)rw_ring_buffer_next (test_instance));
			rw_ring_buffer_append_rval(test_instance, idx + 20);
		}

	rw_ring_buffer_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(rw_ring_buffer_append, overflow)
{
	auto tm = mem_test_new_default (2048);

	auto test_instance = rw_ring_buffer_new(MEM (tm), 10, size_t);
	for (size_t idx = 0; idx < 100; idx++)
		{

			if (idx < 10)
				{
					ASSERT_FALSE(rw_ring_buffer_is_full (test_instance));
				}
			else
				{
					ASSERT_TRUE(rw_ring_buffer_is_full (test_instance));
				}

			ASSERT_EQ(idx % 10, rw_ring_buffer_write_position (test_instance));
			ASSERT_EQ(0, rw_ring_buffer_read_position (test_instance));
			rw_ring_buffer_append_rval(test_instance, idx);
		}

	ASSERT_EQ(0, rw_ring_buffer_write_position (test_instance));
	ASSERT_EQ(0, rw_ring_buffer_read_position (test_instance));
	ASSERT_TRUE(rw_ring_buffer_is_full (test_instance));

	for (size_t idx = 0; idx < 10; idx++)
		{
			size_t val = *(size_t *)rw_ring_buffer_current (test_instance);
			ASSERT_EQ(90 + idx, val);
			rw_ring_buffer_next (test_instance);
		}

	rw_ring_buffer_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(rw_ring_buffer_next, ok)
{
	auto tm = mem_test_new_default (2048);

	auto test_instance = rw_ring_buffer_new(MEM (tm), 10, size_t);
	for (size_t idx = 0; idx < 10; idx++)
		{
			rw_ring_buffer_append_rval(test_instance, idx);
		}

	for (size_t outer = 0; outer < 5; outer++)
		{
			for (size_t idx = 0; idx < 10; idx++)
				{
					size_t val = *(size_t *)rw_ring_buffer_next (test_instance);
					ASSERT_EQ((idx + 1) % 10, val);
				}
		}
	rw_ring_buffer_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(rw_ring_buffer_current, ok)
{
	auto tm = mem_test_new_default (2048);

	auto test_instance = rw_ring_buffer_new(MEM (tm), 4, size_t);
	rw_ring_buffer_append_rval(test_instance, (size_t)2);
	rw_ring_buffer_append_rval(test_instance, (size_t)4);
	rw_ring_buffer_append_rval(test_instance, (size_t)6);
	rw_ring_buffer_append_rval(test_instance, (size_t)8);

	ASSERT_EQ(2, *(size_t *)rw_ring_buffer_current (test_instance));

	ASSERT_EQ(4, *(size_t *)rw_ring_buffer_next (test_instance));
	ASSERT_EQ(4, *(size_t *)rw_ring_buffer_current (test_instance));

	ASSERT_EQ(6, *(size_t *)rw_ring_buffer_next (test_instance));
	ASSERT_EQ(6, *(size_t *)rw_ring_buffer_current (test_instance));

	ASSERT_EQ(8, *(size_t *)rw_ring_buffer_next (test_instance));
	ASSERT_EQ(8, *(size_t *)rw_ring_buffer_current (test_instance));

	rw_ring_buffer_append_rval(test_instance, (size_t)10);
	ASSERT_EQ(8, *(size_t *)rw_ring_buffer_current (test_instance));
	ASSERT_EQ(10, *(size_t *)rw_ring_buffer_next (test_instance));

	ASSERT_EQ(10, *(size_t *)rw_ring_buffer_current (test_instance));
	ASSERT_EQ(4, *(size_t *)rw_ring_buffer_next (test_instance));

	rw_ring_buffer_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(rw_ring_buffer_peek_next, ok)
{
	auto tm = mem_test_new_default (2048);

	auto test_instance = rw_ring_buffer_new(MEM (tm), 4, size_t);
	rw_ring_buffer_append_rval(test_instance, (size_t)2);
	rw_ring_buffer_append_rval(test_instance, (size_t)4);
	rw_ring_buffer_append_rval(test_instance, (size_t)6);
	rw_ring_buffer_append_rval(test_instance, (size_t)8);

	ASSERT_EQ(4, *(size_t *)rw_ring_buffer_next (test_instance));
	ASSERT_EQ(6, *(size_t *)rw_ring_buffer_next (test_instance));

	ASSERT_EQ(6, *(size_t *)rw_ring_buffer_peek_next (test_instance, 0));
	ASSERT_EQ(8, *(size_t *)rw_ring_buffer_peek_next (test_instance, 1));
	//roll over
	ASSERT_EQ(2, *(size_t *)rw_ring_buffer_peek_next (test_instance, 2));
	ASSERT_EQ(4, *(size_t *)rw_ring_buffer_peek_next (test_instance, 3));

	rw_ring_buffer_append_rval(test_instance, (size_t)10);
	ASSERT_EQ(10, *(size_t *)rw_ring_buffer_peek_next (test_instance, 2));

	rw_ring_buffer_append_rval(test_instance, (size_t)12);
	ASSERT_EQ(12, *(size_t *)rw_ring_buffer_peek_next (test_instance, 3));

	rw_ring_buffer_append_rval(test_instance, (size_t)14);
	ASSERT_EQ(14, *(size_t *)rw_ring_buffer_peek_next (test_instance, 0));

	rw_ring_buffer_append_rval(test_instance, (size_t)16);
	ASSERT_EQ(16, *(size_t *)rw_ring_buffer_peek_next (test_instance, 1));

	rw_ring_buffer_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(rw_ring_buffer_peek_next, single_element)
{
	auto tm = mem_test_new_default (2048);

	auto test_instance = rw_ring_buffer_new(MEM (tm), 1, size_t);
	rw_ring_buffer_append_rval(test_instance, (size_t)2);

	ASSERT_EQ(2, *(size_t *)rw_ring_buffer_peek_next (test_instance, 0));

	rw_ring_buffer_next (test_instance);
	ASSERT_EQ(2, *(size_t *)rw_ring_buffer_peek_next (test_instance, 0));

	rw_ring_buffer_free_safe (&test_instance);
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(rw_ring_buffer_free_safe, ok)
{
	auto tm = mem_test_new_default (1024);

	auto test_instance = rw_ring_buffer_new(MEM (tm), 128, u1);

	for (size_t idx = 0; idx < 10; idx++)
		{
			rw_ring_buffer_append_rval(test_instance, idx);
		}

	rw_ring_buffer_free_safe (&test_instance);
	ASSERT_TRUE(test_instance == nullptr);

	mem_test_verify (tm);
	mem_test_free (tm);
}