cmake_minimum_required(VERSION 3.24)
project(project C)

set(CMAKE_C_FLAGS, "${CMAKE_C_FLAGS} -O3")

set(COMPILER_WARNINGS
        -Wall
        -Wextra
        -pedantic
        -Wcast-align
        -Wcast-qual
        -Wdisabled-optimization
        -Wformat=2
        -Winit-self
        -Wmissing-include-dirs
        -Wredundant-decls
        -Wno-gnu-statement-expression
        -Wno-gnu-auto-type
        -Wshadow
        -Wstrict-overflow=5
        -Wundef
        -Wno-cast-qual
        -fdiagnostics-show-option
        -Wno-implicit-fallthrough
        -Wno-c11-extensions
        -Wno-gnu-label-as-value
        )

if (NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE Debug)
endif ()


if (CMAKE_BUILD_TYPE MATCHES Debug)
    message(" ")
    message("Build project in Debug mode")
    message(" ")
    add_compile_options(${COMPILER_WARNINGS})

    if (NOT LOG_LEVEL)
        set(LOG_LEVEL 2)
    endif ()
    add_compile_definitions(LOG_LEVEL=${LOG_LEVEL} RUN_CHECKS=1)

elseif (CMAKE_BUILD_TYPE MATCHES Release)
    message(" ")
    message("Build project in Release mode")
    message(" ")
    add_compile_options(-Werror ${COMPILER_WARNINGS} -D RELEASE_BUILD=1)

    if (NOT LOG_LEVEL)
        set(LOG_LEVEL 3)
    endif ()

    add_compile_definitions(LOG_LEVEL=${LOG_LEVEL} RUN_CHECKS=0)
endif ()

add_custom_target(it)

add_subdirectory(core)
#add_subdirectory(playground)
add_subdirectory(rt)