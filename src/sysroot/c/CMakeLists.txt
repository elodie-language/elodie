cmake_minimum_required(VERSION 3.28)
project(c C)

set(CMAKE_C_STANDARD 11)

add_executable(c main.c core_intrinsics_io.c core_intrinsics_math.c)

target_link_libraries(c PRIVATE m)