cmake_minimum_required(VERSION 3.24)
project(rt C)

file(GLOB SRC_FILES
        src/io.c
)

add_library(rt STATIC EXCLUDE_FROM_ALL ${SRC_FILES})
target_include_directories(rt PUBLIC include)
target_link_libraries(rt PUBLIC core)


##unit tests
#file(GLOB SRC_UNIT_TEST_FILES
#        )
#
#add_executable(core_unit_test ${SRC_UNIT_TEST_FILES})
#target_include_directories(core_unit_test PUBLIC include)
#target_link_libraries(
#        core_unit_test
#        GTest::gtest_main
#        core
#)
#
#gtest_discover_tests(core_unit_test)
