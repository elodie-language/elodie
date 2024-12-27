#include "../unit-test.h"

#include "core/algo/algo-iterator.h"

const size_t test_array_size = 10;

bool
array_iterator_cb_has_next (struct iterator *it)
{
	return it->current.index < test_array_size;
}

void *
array_iterator_cb_next (struct iterator *it)
{
	return (u4 *)it->target + it->current.index++;
}

TEST(iterator, index)
{
	u4 array[test_array_size]{};
	for (size_t idx = 0; idx < test_array_size; idx++)
		{
			array[idx] = idx;
		}

	auto test_instance = iterator_index (&array, array_iterator_cb_has_next, array_iterator_cb_next);
	for (size_t idx = 0; idx < test_array_size; idx++)
		{
			ASSERT_TRUE(iterator_has_next (&test_instance));
			u4 next_value = *(u4 *)iterator_next (&test_instance);
			ASSERT_EQ(idx, next_value);
		}
	ASSERT_FALSE(iterator_has_next (&test_instance));
}