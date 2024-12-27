#include "../unit-test.h"

#include "core/algo/algo-bit.h"

TEST(bit8_init, ok)
{
	struct bit8 test_instance{};
	bit8_init (&test_instance);
	ASSERT_EQ(0, test_instance.data);
}

TEST(bit8_set, ok)
{
	struct bit8 test_instance{};
	bit8_init (&test_instance);

	bit8_set (&test_instance, 0);
	ASSERT_EQ(1, test_instance.data);

	bit8_set (&test_instance, 2);
	ASSERT_EQ(5, test_instance.data);

	bit8_set (&test_instance, 7);
	ASSERT_EQ(133, test_instance.data);
}

TEST(bit8_set_mask, ok)
{
	struct bit8 test_instance{};
	bit8_init (&test_instance);

	bit8_set_mask (&test_instance, 8);
	ASSERT_EQ(8, test_instance.data);
	ASSERT_TRUE(bit8_at (&test_instance, 3));

	bit8_set_mask (&test_instance, 96);
	ASSERT_EQ(104, test_instance.data);
	ASSERT_TRUE(bit8_at (&test_instance, 3));
	ASSERT_TRUE(bit8_at (&test_instance, 5));
	ASSERT_TRUE(bit8_at (&test_instance, 6));

	ASSERT_FALSE(bit8_at (&test_instance, 0));
	ASSERT_FALSE(bit8_at (&test_instance, 1));
	ASSERT_FALSE(bit8_at (&test_instance, 2));
	ASSERT_FALSE(bit8_at (&test_instance, 4));
	ASSERT_FALSE(bit8_at (&test_instance, 7));

}

TEST(bit8_unset, ok)
{
	struct bit8 test_instance{.data = 255};
	bit8_unset (&test_instance, 7);
	ASSERT_EQ(127, bit8_get_mask (&test_instance));

	bit8_unset (&test_instance, 3);
	ASSERT_EQ(119, bit8_get_mask (&test_instance));
}

TEST(bit8_unset_mask, ok)
{
	struct bit8 test_instance{.data = 255};

	bit8_unset_mask (&test_instance, 8);
	ASSERT_EQ(247, test_instance.data);
	ASSERT_FALSE(bit8_at (&test_instance, 3));

	bit8_unset_mask (&test_instance, 96);
	ASSERT_EQ(151, test_instance.data);
	ASSERT_FALSE(bit8_at (&test_instance, 3));
	ASSERT_FALSE(bit8_at (&test_instance, 5));
	ASSERT_FALSE(bit8_at (&test_instance, 6));

	ASSERT_TRUE(bit8_at (&test_instance, 1));
	ASSERT_TRUE(bit8_at (&test_instance, 7));
	ASSERT_TRUE(bit8_at (&test_instance, 2));
	ASSERT_TRUE(bit8_at (&test_instance, 4));
	ASSERT_TRUE(bit8_at (&test_instance, 0));
}

TEST(bit8_toggle, ok)
{
	struct bit8 test_instance{};
	bit8_init (&test_instance);

	bit8_toggle (&test_instance, 0);
	ASSERT_EQ(1, test_instance.data);

	bit8_toggle (&test_instance, 2);
	ASSERT_EQ(5, test_instance.data);

	bit8_toggle (&test_instance, 7);
	ASSERT_EQ(133, test_instance.data);

	bit8_toggle (&test_instance, 7);
	ASSERT_EQ(5, test_instance.data);

	bit8_toggle (&test_instance, 2);
	ASSERT_EQ(1, test_instance.data);

	bit8_toggle (&test_instance, 0);
	ASSERT_EQ(0, test_instance.data);
}

TEST(bit8_toggle_mask, ok)
{
	struct bit8 test_instance{};
	bit8_init (&test_instance);

	bit8_toggle_mask (&test_instance, 8);
	ASSERT_EQ(8, test_instance.data);

	bit8_toggle_mask (&test_instance, 96);
	ASSERT_EQ(104, test_instance.data);

	bit8_toggle_mask (&test_instance, 96);
	ASSERT_EQ(8, test_instance.data);

	bit8_toggle_mask (&test_instance, 8);
	ASSERT_EQ(0, test_instance.data);
}

TEST(bit8_at, ok)
{
	struct bit8 test_instance = {128};
	ASSERT_TRUE(bit8_at (&test_instance, 7));
	for (size_t idx = 0; idx < 7; idx++)
		{
			ASSERT_FALSE(bit8_at (&test_instance, idx));
		}

	test_instance = {127};
	ASSERT_FALSE(bit8_at (&test_instance, 7));
	for (size_t idx = 0; idx < 7; idx++)
		{
			ASSERT_TRUE(bit8_at (&test_instance, idx));
		}
}

TEST(bit8_get_mask, ok)
{
	struct bit8 test_instance = {0};
	ASSERT_EQ(0, bit8_get_mask (&test_instance));
	bit8_set (&test_instance, 7);
	ASSERT_EQ(128, bit8_get_mask (&test_instance));

	test_instance.data = 255;
	ASSERT_EQ(255, bit8_get_mask (&test_instance));
}

TEST(bit8_reset, ok)
{
	struct bit8 test_instance = {0};
	bit8_set_mask (&test_instance, 255);
	bit8_reset (&test_instance);
	ASSERT_EQ(0, bit8_get_mask (&test_instance));
}

TEST(bit64_init, ok)
{
	struct bit64 test_instance{};
	bit64_init (&test_instance);
	ASSERT_EQ(0, test_instance.numeric);
}

TEST(bit64_set, ok)
{
	struct bit64 test_instance{};
	bit64_init (&test_instance);

	bit64_set (&test_instance, 0);
	ASSERT_EQ(1, test_instance.numeric);

	bit64_set (&test_instance, 2);
	ASSERT_EQ(5, test_instance.numeric);

	bit64_set (&test_instance, 7);
	ASSERT_EQ(133, test_instance.numeric);
}

TEST(bit64_set, each_bucket)
{
	struct bit64 test_instance{};
	bit64_init (&test_instance);

	bit64_set (&test_instance, 0);
	ASSERT_EQ(1, test_instance.numeric);

	bit64_set (&test_instance, 9);
	ASSERT_EQ(513, test_instance.numeric);

	bit64_set (&test_instance, 17);
	ASSERT_EQ(131585, test_instance.numeric);

	bit64_set (&test_instance, 25);
	ASSERT_EQ(33686017, test_instance.numeric);

	bit64_set (&test_instance, 33);
	ASSERT_EQ(8623620609, test_instance.numeric);

	bit64_set (&test_instance, 41);
	ASSERT_EQ(2207646876161, test_instance.numeric);

	bit64_set (&test_instance, 49);
	ASSERT_EQ(565157600297473, test_instance.numeric);

	bit64_set (&test_instance, 57);
	ASSERT_EQ(144680345676153345, test_instance.numeric);
}

TEST(bit64_set_mask, ok)
{
	struct bit64 test_instance{};
	bit64_init (&test_instance);

	bit64_set_mask (&test_instance, 8);
	ASSERT_EQ(8, test_instance.numeric);
	ASSERT_TRUE(bit64_at (&test_instance, 3));

	bit64_set_mask (&test_instance, 96);
	ASSERT_EQ(104, test_instance.numeric);
	ASSERT_TRUE(bit64_at (&test_instance, 3));
	ASSERT_TRUE(bit64_at (&test_instance, 5));
	ASSERT_TRUE(bit64_at (&test_instance, 6));

	ASSERT_FALSE(bit64_at (&test_instance, 0));
	ASSERT_FALSE(bit64_at (&test_instance, 1));
	ASSERT_FALSE(bit64_at (&test_instance, 2));
	ASSERT_FALSE(bit64_at (&test_instance, 4));
	ASSERT_FALSE(bit64_at (&test_instance, 7));
}

TEST(bit64_set_mask, each_bit)
{
	struct bit64 test_instance{};
	bit64_init (&test_instance);

	bit64_set_mask (&test_instance, U8_MAX);

	for (size_t idx = 0; idx < 64; idx++)
		{
			ASSERT_TRUE(bit64_at (&test_instance, idx));
		}
}

TEST(bit64_unset, ok)
{
	struct bit64 test_instance{.numeric = 255};
	bit64_unset (&test_instance, 7);
	ASSERT_EQ(127, bit64_get_mask (&test_instance));

	bit64_unset (&test_instance, 3);
	ASSERT_EQ(119, bit64_get_mask (&test_instance));
}

TEST(bit64_unset, each_bucket)
{
	struct bit64 test_instance{.numeric=144680345676153345};

	bit64_unset (&test_instance, 57);
	ASSERT_EQ(565157600297473, test_instance.numeric);

	bit64_unset (&test_instance, 49);
	ASSERT_EQ(2207646876161, test_instance.numeric);

	bit64_unset (&test_instance, 41);
	ASSERT_EQ(8623620609, test_instance.numeric);

	bit64_unset (&test_instance, 33);
	ASSERT_EQ(33686017, test_instance.numeric);

	bit64_unset (&test_instance, 25);
	ASSERT_EQ(131585, test_instance.numeric);

	bit64_unset (&test_instance, 17);
	ASSERT_EQ(513, test_instance.numeric);

	bit64_unset (&test_instance, 9);
	ASSERT_EQ(1, test_instance.numeric);

	bit64_unset (&test_instance, 0);
	ASSERT_EQ(0, test_instance.numeric);
}

TEST(bit64_unset_mask, ok)
{
	struct bit64 test_instance{.numeric = 255};

	bit64_unset_mask (&test_instance, 8);
	ASSERT_EQ(247, test_instance.numeric);
	ASSERT_FALSE(bit64_at (&test_instance, 3));

	bit64_unset_mask (&test_instance, 96);
	ASSERT_EQ(151, test_instance.numeric);
	ASSERT_FALSE(bit64_at (&test_instance, 3));
	ASSERT_FALSE(bit64_at (&test_instance, 5));
	ASSERT_FALSE(bit64_at (&test_instance, 6));

	ASSERT_TRUE(bit64_at (&test_instance, 1));
	ASSERT_TRUE(bit64_at (&test_instance, 7));
	ASSERT_TRUE(bit64_at (&test_instance, 2));
	ASSERT_TRUE(bit64_at (&test_instance, 4));
	ASSERT_TRUE(bit64_at (&test_instance, 0));
}

TEST(bit64_unset_mask, each_bit)
{
	struct bit64 test_instance{.numeric = U8_MAX};
	bit64_unset_mask (&test_instance, U8_MAX);

	for (size_t idx = 0; idx < 64; idx++)
		{
			ASSERT_FALSE(bit64_at (&test_instance, idx));
		}
}

TEST(bit64_toggle, ok)
{
	struct bit64 test_instance{};
	bit64_init (&test_instance);

	bit64_toggle (&test_instance, 0);
	ASSERT_EQ(1, test_instance.numeric);

	bit64_toggle (&test_instance, 2);
	ASSERT_EQ(5, test_instance.numeric);

	bit64_toggle (&test_instance, 7);
	ASSERT_EQ(133, test_instance.numeric);

	bit64_toggle (&test_instance, 7);
	ASSERT_EQ(5, test_instance.numeric);

	bit64_toggle (&test_instance, 2);
	ASSERT_EQ(1, test_instance.numeric);

	bit64_toggle (&test_instance, 0);
	ASSERT_EQ(0, test_instance.numeric);
}

TEST(bit64_toggle, each_bucket)
{
	struct bit64 test_instance{};
	bit64_init (&test_instance);

	bit64_toggle (&test_instance, 0);
	ASSERT_EQ(1, test_instance.numeric);

	bit64_toggle (&test_instance, 9);
	ASSERT_EQ(513, test_instance.numeric);

	bit64_toggle (&test_instance, 17);
	ASSERT_EQ(131585, test_instance.numeric);

	bit64_toggle (&test_instance, 25);
	ASSERT_EQ(33686017, test_instance.numeric);

	bit64_toggle (&test_instance, 33);
	ASSERT_EQ(8623620609, test_instance.numeric);

	bit64_toggle (&test_instance, 41);
	ASSERT_EQ(2207646876161, test_instance.numeric);

	bit64_toggle (&test_instance, 49);
	ASSERT_EQ(565157600297473, test_instance.numeric);

	bit64_toggle (&test_instance, 57);
	ASSERT_EQ(144680345676153345, test_instance.numeric);

	bit64_toggle (&test_instance, 57);
	ASSERT_EQ(565157600297473, test_instance.numeric);

	bit64_toggle (&test_instance, 49);
	ASSERT_EQ(2207646876161, test_instance.numeric);

	bit64_toggle (&test_instance, 41);
	ASSERT_EQ(8623620609, test_instance.numeric);

	bit64_toggle (&test_instance, 33);
	ASSERT_EQ(33686017, test_instance.numeric);

	bit64_toggle (&test_instance, 25);
	ASSERT_EQ(131585, test_instance.numeric);

	bit64_toggle (&test_instance, 17);
	ASSERT_EQ(513, test_instance.numeric);

	bit64_toggle (&test_instance, 9);
	ASSERT_EQ(1, test_instance.numeric);

	bit64_toggle (&test_instance, 0);
	ASSERT_EQ(0, test_instance.numeric);
}

TEST(bit64_toggle_mask, ok)
{
	struct bit64 test_instance{};
	bit64_init (&test_instance);

	bit64_toggle_mask (&test_instance, 8);
	ASSERT_EQ(8, test_instance.numeric);

	bit64_toggle_mask (&test_instance, 96);
	ASSERT_EQ(104, test_instance.numeric);

	bit64_toggle_mask (&test_instance, 96);
	ASSERT_EQ(8, test_instance.numeric);

	bit64_toggle_mask (&test_instance, 8);
	ASSERT_EQ(0, test_instance.numeric);
}

TEST(bit64_toggle_mask, each_bit)
{
	struct bit64 test_instance{};
	bit64_init (&test_instance);

	bit64_toggle_mask (&test_instance, U8_MAX);

	for (size_t idx = 0; idx < 64; idx++)
		{
			ASSERT_TRUE(bit64_at (&test_instance, idx));
		}

	bit64_toggle_mask (&test_instance, U8_MAX);
	for (size_t idx = 0; idx < 64; idx++)
		{
			ASSERT_FALSE(bit64_at (&test_instance, idx));
		}

}

TEST(bit64_at, ok)
{
	struct bit64 test_instance = {.numeric=128};
	ASSERT_TRUE(bit64_at (&test_instance, 7));
	for (size_t idx = 0; idx < 7; idx++)
		{
			ASSERT_FALSE(bit64_at (&test_instance, idx));
		}

	test_instance = {.numeric=127};
	ASSERT_FALSE(bit64_at (&test_instance, 7));
	for (size_t idx = 0; idx < 7; idx++)
		{
			ASSERT_TRUE(bit64_at (&test_instance, idx));
		}
}

TEST(bit64_get_mask, ok)
{
	struct bit64 test_instance = {.numeric=0};
	ASSERT_EQ(0, bit64_get_mask (&test_instance));
	bit64_set (&test_instance, 7);
	ASSERT_EQ(128, bit64_get_mask (&test_instance));

	test_instance.numeric = 255;
	ASSERT_EQ(255, bit64_get_mask (&test_instance));
}

TEST(bit64_reset, ok)
{
	struct bit64 test_instance = {.numeric=0};
	bit64_set_mask (&test_instance, 255);
	bit64_reset (&test_instance);
	ASSERT_EQ(0, bit64_get_mask (&test_instance));
}