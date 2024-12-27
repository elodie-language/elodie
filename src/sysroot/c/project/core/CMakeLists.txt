cmake_minimum_required(VERSION 3.24)
project(core)

file(GLOB SRC_FILES
        src/algo/algo-bit.c
        src/algo/algo-hash.c
        src/algo/algo-iterator.c
        src/algo/algo-list.c
        src/algo/algo-list-byte.c
        src/algo/algo-list-ptr.c
        src/algo/algo-map.c
        src/algo/algo-map-ptr.c
        src/algo/algo-ring-buffer.c
        src/algo/algo-ring-buffer-rw.c
        src/algo/algo-set.c

        src/bytes/bytes.c
        src/bytes/bytes-buffer.c
        src/bytes/bytes-view.c

        src/fsm/fsm.c
        src/fsm/fsm-event.c
        src/fsm/fsm-state.c
        src/fsm/fsm-transition.c

        src/mem/mem.c
        src/mem/mem-api.c
        src/mem/mem-gc.c
        src/mem/mem-null.c
        src/mem/mem-raw.c
        src/mem/mem-stack.c
        src/mem/mem-test.c
        src/mem/mem-gen.c
        src/mem/mem-vape.c

        src/val/val.c
        src/val/val-api.c
        src/val/val-bool.c
        src/val/val-clsr.c
        src/val/val-fld.c
        src/val/val-fn.c
        src/val/val-lst.c
        src/val/val-mod.c
        src/val/val-mod-const-pool.c
        src/val/val-nil.c
        src/val/val-num.c
        src/val/val-obj.c
        src/val/val-prop.c
        src/val/val-writer.c
        src/val/val-str.c
        src/val/val-str-view.c
        src/val/val-unit.c

        src/native/native-fn.c
        src/native/native-fn-sig.c
        src/native/native-registry.c

        src/string/string.c
        src/string/string-view.c

        src/type/type.c
        src/type/type-internal.c
        src/type/type-system.c

        src/core.c
        src/io.c
        src/json.c

        src/logger.c
        src/core.c
        src/util.c
        )

add_library(core STATIC EXCLUDE_FROM_ALL ${SRC_FILES})
target_include_directories(core PUBLIC include)

##unit tests
#file(GLOB SRC_UNIT_TEST_FILES
#        test/unit/main.cpp
#
#        test/unit/algo/test-algo-bit.cpp
#        test/unit/algo/test-algo-hash.cpp
#        test/unit/algo/test-algo-iterator.cpp
#        test/unit/algo/test-algo-list.cpp
#        test/unit/algo/test-algo-list-byte.cpp
#        test/unit/algo/test-algo-list-ptr.cpp
#        test/unit/algo/test-algo-map.cpp
#        test/unit/algo/test-algo-map-ptr.cpp
#        test/unit/algo/test-algo-ring-buffer.cpp
#        test/unit/algo/test-algo-ring-buffer-rw.cpp
#        test/unit/algo/test-algo-set.cpp
#
#        test/unit/bytes/test-bytes.cpp
#        test/unit/bytes/test-bytes-buffer.cpp
#        test/unit/bytes/test-bytes-view.cpp
#
#        test/unit/fsm/test-fsm.cpp
#        test/unit/fsm/test-fsm-event.cpp
#        test/unit/fsm/test-fsm-state.cpp
#
#        test/unit/mem/test-mem-gc.cpp
#        test/unit/mem/test-mem-stack.cpp
#        test/unit/mem/test-mem-test.cpp
#        test/unit/mem/test-mem-vape.cpp
#
#        test/unit/native/test-native-fn.cpp
#        test/unit/native/test-native-fn-sig.cpp
#
#        test/unit/string/test-string.cpp
#        test/unit/string/test-string-view.cpp
#
#        test/unit/type/test-type.cpp
#        test/unit/type/test-type-internal.cpp
#        test/unit/type/test-type-system.cpp
#
#        test/unit/val/test-val-api.cpp
#        test/unit/val/test-val-bool.cpp
#        test/unit/val/test-val-clsr.cpp
#        test/unit/val/test-val-fld.cpp
#        test/unit/val/test-val-fn.cpp
#        test/unit/val/test-val-lst.cpp
#        test/unit/val/test-val-mod.cpp
#        test/unit/val/test-val-mod-const-pool.cpp
#        test/unit/val/test-val-nil.cpp
#        test/unit/val/test-val-num.cpp
#        test/unit/val/test-val-obj.cpp
#        test/unit/val/test-val-prop.cpp
#        test/unit/val/test-val-str.cpp
#        test/unit/val/test-val-str-view.cpp
#        test/unit/val/test-val-type.cpp
#        test/unit/val/test-val-unit.cpp
#
#        test/unit/test-core.cpp
#        test/unit/test-json.cpp
#        test/unit/test-check.cpp
#        test/unit/test-util.cpp
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
