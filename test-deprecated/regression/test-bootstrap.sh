#!/usr/bin/env bash

set -eu

DIFF_TOOL="${DIFF_TOOL:-diff --ignore-blank-lines --ignore-all-space}"
TEST_DIR="${1:-./test/e2e}"
BIN="${2:-../bootstrap/target/debug/bootstrap}"

EXIT_CODE=0
OK_COUNT=0
ERR_COUNT=0

# Enable recursive globbing
shopt -s globstar

# Loop through all test files in the TEST_DIR recursively
for FILE in $(find "$TEST_DIR" -type f -name "*.ec" ! -name "*.test.ec"); do
    if [[ ! -f "$FILE" ]]; then
        echo "No test files found in directory $TEST_DIR"
        exit 1
    fi

    # Run the test and compare output
    if ! ${DIFF_TOOL} \
        <(awk -F '// out:' '/out/{print $2}' "$FILE") \
        <(${BIN} "$FILE" 2> /dev/null); then
            printf "\e[31mFail\e[0m\t$FILE\n"
            ERR_COUNT=$((ERR_COUNT + 1))
            EXIT_CODE=1
    else
        printf "\e[32mPass\e[0m\t$FILE\n"
        OK_COUNT=$((OK_COUNT + 1))
    fi
done

# Summary
echo "----------------------"
echo -e "bootstrap::run::regression - Passed:\e[32m $OK_COUNT" "\e[0mFailed:\e[31m $ERR_COUNT\e[0m"
echo "----------------------"
echo -e "bootstrap::run::regression - self hosted tests"


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
