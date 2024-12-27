#!/usr/bin/env bash

set -eu

DIFF_TOOL="${DIFF_TOOL:-diff --ignore-blank-lines --ignore-all-space}"
TEST_DIR="${1:-./test/spec}"
BIN="${2:-../bootstrap/target/debug/bootstrap}"

EXIT_CODE=0

shopt -s globstar

echo "----------------------"
echo -e "bootstrap::spec::c"


for FILE in "$TEST_DIR"/**/*.test.ec; do
    if [[ ! -f "$FILE" ]]; then
        echo "No test files found in directory $TEST_DIR"
        exit 1
    fi
    echo "----------------------"
    echo -e "$FILE"
    if ! ${BIN} test "$FILE" true true; then
        EXIT_CODE=-1
    fi
done

exit $EXIT_CODE
