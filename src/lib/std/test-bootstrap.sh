#!/usr/bin/env bash

set -eu

TEST_DIR="${1:-.}"
BIN="${2:-../../../bootstrap/target/debug/bootstrap}"
EXIT_CODE=0

# Enable recursive globbing
shopt -s globstar

for FILE in "$TEST_DIR"/**/*.test.ec; do
    if [[ ! -f "$FILE" ]]; then
        echo "No test files found in directory $TEST_DIR"
        exit 1
    fi
    if ! ${BIN} test "$FILE" true true; then
        EXIT_CODE=-1
    fi
done

exit $EXIT_CODE