cmake_minimum_required(VERSION 3.24)
project(playground C)

file(GLOB SRC_FILES
        src/main.c
)

add_executable(playground ${SRC_FILES})
target_link_libraries(playground PUBLIC core rt)