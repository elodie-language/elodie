#!/usr/bin/env bash

set -eu

DIFF_TOOL="${DIFF_TOOL:-diff --ignore-blank-lines --ignore-all-space}"
TEST_DIR="${1:-./test/smoke}"
BIN="${2:-../bootstrap/target/debug/bootstrap}"

EXIT_CODE=0
OK_COUNT=0
ERR_COUNT=0

shopt -s globstar

for FILE in "$TEST_DIR"/**/hello_world.ec; do

    if [[ ! -f "$FILE" ]]; then
        echo "No test files found in directory $TEST_DIR"
        exit 1
    fi

    if ! ${BIN} build "$FILE" 2> /dev/null; then
        printf "\e[31mFail\e[0m\t$FILE (Failed to build C code)\n"
        ERR_COUNT=$((ERR_COUNT + 1))
        EXIT_CODE=1
        continue
    fi

    BASE_NAME=$(basename "$FILE" .ec)

    if ! "/tmp/elodie/$BASE_NAME/$BASE_NAME" > "/tmp/elodie/$BASE_NAME/$BASE_NAME.out" 2> /dev/null; then
        printf "\e[31mFail\e[0m\t$FILE (Execution failed)\n"
        ERR_COUNT=$((ERR_COUNT + 1))
        EXIT_CODE=1
        continue
    fi

    if ! ${DIFF_TOOL} \
        <(awk -F '// out:' '/out/{print $2}' "$FILE") \
        "/tmp/elodie/$BASE_NAME/$BASE_NAME.out"; then
        printf "\e[31mFail\e[0m\t$FILE\n"
        ERR_COUNT=$((ERR_COUNT + 1))
        EXIT_CODE=1
    else
        printf "\e[32mPass\e[0m\t$FILE\n"
        OK_COUNT=$((OK_COUNT + 1))
    fi

done

echo "----------------------"
echo -e "bootstrap::build::smoke - Passed:\e[32m $OK_COUNT" "\e[0mFailed:\e[31m $ERR_COUNT\e[0m"
echo "----------------------"

exit $EXIT_CODE