#!/bin/bash

if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <test_files_location> <binary>"
    exit 1
fi

TEST_FILES_LOCATION="$1"
BINARY="$2"

"$TEST_FILES_LOCATION/test-bootstrap-build.sh" "$TEST_FILES_LOCATION" "$BINARY"
EXIT_CODE_BUILD=$?

"$TEST_FILES_LOCATION/test-bootstrap-run.sh" "$TEST_FILES_LOCATION" "$BINARY"
EXIT_CODE_RUN=$?

if [ $EXIT_CODE_BUILD -ne 0 ] || [ $EXIT_CODE_RUN -ne 0 ]; then
    echo "One or more tests failed."
    exit 1
fi

echo "All tests passed."
exit 0