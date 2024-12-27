#include "unit-test.h"

#include "core/util.h"

TEST(is_digit, true)
{
	for (size_t i = 0; i < 9; i++)
		{
			ASSERT_TRUE(is_digit (static_cast<char>(i + 48)));
		}
}

TEST(is_digit, false)
{
	for (size_t i = 0; i < 48; i++)
		{
			ASSERT_FALSE(is_digit (static_cast<char>(i)));
		}

	for (size_t i = 58; i < UINT8_MAX; i++)
		{
			ASSERT_FALSE(is_digit (static_cast<char>(i)));
		}
}

TEST(is_alpha, true)
{
	for (size_t i = 65; i < 91; i++)
		{
			ASSERT_TRUE(is_alpha (static_cast<char>(i)));
		}

	for (size_t i = 97; i < 123; i++)
		{
			ASSERT_TRUE(is_alpha (static_cast<char>(i)));
		}
}

TEST(is_alpha, false)
{
	for (size_t i = 0; i < 64; i++)
		{
			ASSERT_FALSE(is_alpha (static_cast<char>(i)));
		}

	for (size_t i = 91; i < 97; i++)
		{
			ASSERT_FALSE(is_alpha (static_cast<char>(i)));
		}

	for (size_t i = 123; i < UINT8_MAX; i++)
		{
			ASSERT_FALSE(is_alpha (static_cast<char>(i)));
		}
}

TEST(is_underscore, true)
{
	ASSERT_TRUE(is_underscore ('_'));
}

TEST(is_underscore, false)
{
	for (size_t i = 0; i < 95; i++)
		{
			ASSERT_FALSE(is_underscore (static_cast<char>(i)));
		}

	for (size_t i = 96; i < UINT8_MAX; i++)
		{
			ASSERT_FALSE(is_underscore (static_cast<char>(i)));
		}
}

TEST(is_minus, true)
{
	ASSERT_TRUE(is_minus ('-'));
}

TEST(is_minus, false)
{
	for (size_t i = 0; i < 45; i++)
		{
			ASSERT_FALSE(is_minus (static_cast<char>(i)));
		}

	for (size_t i = 46; i < UINT8_MAX; i++)
		{
			ASSERT_FALSE(is_minus (static_cast<char>(i)));
		}
}

TEST(is_quote, true)
{
	ASSERT_TRUE(is_quote ('\"'));
	ASSERT_TRUE(is_quote ('\''));
}

TEST(is_quoute, false)
{
	for (size_t i = 0; i < UINT8_MAX; i++)
		{
			if (i == 34 || i == 39)
				{
					continue;
				}
			ASSERT_FALSE(is_quote (static_cast<char>(i)));
		}
}

TEST(is_whitespace, true)
{
	ASSERT_TRUE(is_whitespace (' '));
	ASSERT_TRUE(is_whitespace ('\t'));
	ASSERT_TRUE(is_whitespace ('\n'));
	ASSERT_TRUE(is_whitespace ('\r'));
}

TEST(is_whitespace, false)
{
	for (size_t i = 0; i < UINT8_MAX; i++)
		{
			if (i == 9 || i == 13 || i == 32 || i == 10)
				{
					continue;
				}
			ASSERT_FALSE(is_whitespace (static_cast<char>(i)));
		}
}

TEST(is_hex_char, true)
{
	for (size_t i = 48; i < 58; i++)
		{
			ASSERT_TRUE(is_hex_char (static_cast<char>(i)));
		}

	for (size_t i = 65; i < 71; i++)
		{
			ASSERT_TRUE(is_hex_char (static_cast<char>(i)));
		}

	for (size_t i = 97; i < 103; i++)
		{
			ASSERT_TRUE(is_hex_char (static_cast<char>(i)));
		}
}

TEST(is_hex_char, false)
{
	for (size_t i = 0; i < 48; i++)
		{
			ASSERT_FALSE(is_hex_char (static_cast<char>(i)));
		}

	for (size_t i = 58; i < 65; i++)
		{
			ASSERT_FALSE(is_hex_char (static_cast<char>(i)));
		}

	for (size_t i = 71; i < 97; i++)
		{
			ASSERT_FALSE(is_hex_char (static_cast<char>(i)));
		}

	for (size_t i = 103; i < UINT8_MAX; i++)
		{
			ASSERT_FALSE(is_hex_char (static_cast<char>(i)));
		}
}