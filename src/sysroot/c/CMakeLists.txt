cmake_minimum_required(VERSION 3.28)
project(c C)

set(CMAKE_C_STANDARD 11)

add_executable(c main.c
        rt/intrinsics/io.c
        core/intrinsics/math.c
        core/boolean.c
)

target_link_libraries(c PRIVATE m)