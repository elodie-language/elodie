#include "unit-test.h"

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmacro-redefined"

#include "core/check.h"

void invoke_abort ()
{
	ABORT("abort invoked");
}

TEST(ABORT_, ok)
{
	testing::internal::CaptureStdout ();

	EXPECT_EXIT(
		invoke_abort (),
		testing::KilledBySignal(SIGABRT),
		""
	);

	ASSERT_EQ(
		given_output (),
		"\x1B[31mFATAL\x1B[0m \x1B[90mtest-check.cpp:10\x1B[0m \x1B[31mabort invoked\n"
	);
}

TEST(CHECK, ok)
{
	int x = 10;
	CHECK(x == 10);
}

void test_CHECK_fail ()
{
	int x = 10;
	CHECK(x != 10);
}

TEST(CHECK, violated)
{
	testing::internal::CaptureStdout ();

	EXPECT_EXIT(
		test_CHECK_fail (),
		testing::KilledBySignal(SIGABRT),
		""
	);

	ASSERT_EQ(
		"\x1B[31mFATAL\x1B[0m \x1B[90mtest-check.cpp:38\x1B[0m \x1B[31mcheck 'x != 10' failed\n",
		given_output ()
	);
}

TEST(CHECK_NOT_NULL, ok)
{
	int x = 10;
	int *intptr = &x;
	CHECK_NOT_NULL(intptr);
}

void test_CHECK_NOT_NULL_fail ()
{
	int *intptr = nullptr;
	CHECK_NOT_NULL(intptr);
}

TEST(CHECK_NOT_NULL, violated)
{
	testing::internal::CaptureStdout ();

	EXPECT_EXIT(
		test_CHECK_NOT_NULL_fail (),
		testing::KilledBySignal(SIGABRT),
		""
	);

	ASSERT_EQ(
		"\x1B[31mFATAL\x1B[0m \x1B[90mtest-check.cpp:67\x1B[0m \x1B[31mcheck '(intptr) != NULL' failed\n",
		given_output ()
	);
}

TEST(CHECK_NULL, ok)
{
	int *intptr = nullptr;
	CHECK_NULL(intptr);
}

void test_CHECK_NULL_fail ()
{
	int x = 10;
	int *intptr = &x;
	CHECK_NULL(intptr);
}

TEST(CHECK_NULL, violated)
{
	testing::internal::CaptureStdout ();

	EXPECT_EXIT(
		test_CHECK_NULL_fail (),
		testing::KilledBySignal(SIGABRT),
		""
	);

	ASSERT_EQ(
		"\x1B[31mFATAL\x1B[0m \x1B[90mtest-check.cpp:96\x1B[0m \x1B[31mcheck '(intptr) == NULL' failed\n",
		given_output ()
	);
}

void test_CHECK_EQUAL_fail ()
{
	CHECK_EQUAL(21, 42);
}

TEST(CHECK_EQUAL, ok)
{
	CHECK_EQUAL(10, 10);
	const char *char_ptr = "A";
	CHECK_EQUAL(char_ptr, char_ptr);
	CHECK_EQUAL(true, true);
	CHECK_EQUAL(false, false);
	CHECK_EQUAL(NULL, NULL);
}

TEST(CHECK_EQUAL, violated)
{
	testing::internal::CaptureStdout ();

	EXPECT_EXIT(
		test_CHECK_EQUAL_fail (),
		testing::KilledBySignal(SIGABRT),
		""
	);

	ASSERT_EQ(
		"\x1B[31mFATAL\x1B[0m \x1B[90mtest-check.cpp:117\x1B[0m \x1B[31mcheck '21 == 42' failed\n",
		given_output ()
	);
}

TEST(CHECK_NOT_EQUAL, ok)
{
	CHECK_NOT_EQUAL(21, 42);
	char const *char_ptr = "A";
	char const *char_ptr_2 = "B";
	CHECK_NOT_EQUAL(char_ptr, char_ptr_2);
	CHECK_NOT_EQUAL(true, false);
}

void test_CHECK_NOT_EQUAL_fail ()
{
	CHECK_NOT_EQUAL(42, 42);
}

TEST(CHECK_NOT_EQUAL, violated)
{
	testing::internal::CaptureStdout ();

	EXPECT_EXIT(
		test_CHECK_NOT_EQUAL_fail (),
		testing::KilledBySignal(SIGABRT),
		""
	);

	ASSERT_EQ(
		"\x1B[31mFATAL\x1B[0m \x1B[90mtest-check.cpp:157\x1B[0m \x1B[31mcheck '42 != 42' failed\n",
		given_output ()
	);
}

TEST(CHECK_GREATER_THAN, ok)
{
	CHECK_GREATER_THAN(42, 21);
}

void test_CHECK_GREATER_THAN_fail ()
{
	CHECK_GREATER_THAN(21, 42);
}

TEST(CHECK_GREATER_THAN, violated)
{
	testing::internal::CaptureStdout ();

	EXPECT_EXIT(
		test_CHECK_GREATER_THAN_fail (),
		testing::KilledBySignal(SIGABRT),
		""
	);

	ASSERT_EQ(
		"\x1B[31mFATAL\x1B[0m \x1B[90mtest-check.cpp:183\x1B[0m \x1B[31mcheck '21 > 42' failed\n",
		given_output ()
	);
}

TEST(CHECK_LESS_THAN, ok)
{
	CHECK_LESS_THAN(21, 42);
}

void test_CHECK_LESS_THAN_fail ()
{
	CHECK_LESS_THAN(42, 21);
}

TEST(CHECK_LESS_THAN, violated)
{
	testing::internal::CaptureStdout ();

	EXPECT_EXIT(
		test_CHECK_LESS_THAN_fail (),
		testing::KilledBySignal(SIGABRT),
		""
	);

	ASSERT_EQ(
		"\x1B[31mFATAL\x1B[0m \x1B[90mtest-check.cpp:209\x1B[0m \x1B[31mcheck '42 < 21' failed\n",
		given_output ()
	);
}

TEST(CHECK_GREATER_THAN_EQUAL, ok)
{
	CHECK_GREATER_THAN_EQUAL(42, 21);
	CHECK_GREATER_THAN_EQUAL(42, 42);
}

void test_CHECK_GREATER_THAN_EQUAL_fail ()
{
	CHECK_GREATER_THAN_EQUAL(21, 42);
}

TEST(CHECK_GREATER_THAN_EQUAL, violated)
{
	testing::internal::CaptureStdout ();

	EXPECT_EXIT(
		test_CHECK_GREATER_THAN_EQUAL_fail (),
		testing::KilledBySignal(SIGABRT),
		""
	);

	ASSERT_EQ(
		"\x1B[31mFATAL\x1B[0m \x1B[90mtest-check.cpp:236\x1B[0m \x1B[31mcheck '21 >= 42' failed\n",
		given_output ()
	);
}

TEST(CHECK_LESS_THAN_EQUAL, ok)
{
	CHECK_LESS_THAN_EQUAL(21, 42);
	CHECK_LESS_THAN_EQUAL(42, 42);
}

void test_CHECK_LESS_THAN_EQUAL_fail ()
{
	CHECK_LESS_THAN_EQUAL(42, 21);
}

TEST(CHECK_LESS_THAN_EQUAL, violated)
{
	testing::internal::CaptureStdout ();

	EXPECT_EXIT(
		test_CHECK_LESS_THAN_EQUAL_fail (),
		testing::KilledBySignal(SIGABRT),
		""
	);

	ASSERT_EQ(
		"\x1B[31mFATAL\x1B[0m \x1B[90mtest-check.cpp:263\x1B[0m \x1B[31mcheck '42 <= 21' failed\n",
		given_output ()
	);
}

TEST(CHECK_TRUE, ok)
{
	CHECK_TRUE(10 == 10);
	CHECK_TRUE(true);
}

TEST(CHECK_TRUE, violated)
{
	testing::internal::CaptureStdout ();

	EXPECT_EXIT(
		{
			CHECK_TRUE (false);
		},
		testing::KilledBySignal(SIGABRT),
		""
	);

	ASSERT_EQ(
		"\x1B[31mFATAL\x1B[0m \x1B[90mtest-check.cpp:294\x1B[0m \x1B[31mcheck '(false) == true' failed\n",
		given_output ()
	);
}

TEST(CHECK_FALSE, ok)
{
	CHECK_FALSE(21 == 42);
	CHECK_FALSE(false);
}

TEST(CHECK_FALSE, violated)
{
	testing::internal::CaptureStdout ();

	EXPECT_EXIT(
		{
			CHECK_FALSE (true);
		},
		testing::KilledBySignal(SIGABRT),
		""
	);

	ASSERT_EQ(
		"\x1B[31mFATAL\x1B[0m \x1B[90mtest-check.cpp:318\x1B[0m \x1B[31mcheck '(true) == false' failed\n",
		given_output ()
	);
}

#pragma GCC diagnostic pop