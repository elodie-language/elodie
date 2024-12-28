#include "unit-test.h"

#include "core/mem/mem.h"

int
main (int argc, char **argv)
{
	::testing::InitGoogleTest (&argc, argv);
	mem_new (1 << 20);
	int result = RUN_ALL_TESTS ();
	mem_free ();
	return result;
}

std::string
captured_output ()
{
	std::string output = testing::internal::GetCapturedStdout ();
	if (output.empty ())
		{
			return "";
		}
	return output.substr (18, output.size ());
}