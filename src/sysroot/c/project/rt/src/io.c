#include "rt/io.h"

#include <stdio.h>

void rt_io_print(const char *text) {
    printf("%s", text);
}

void rt_io_println(char const * message) {
    rt_io_print(message);
    rt_io_print("\n");
}
