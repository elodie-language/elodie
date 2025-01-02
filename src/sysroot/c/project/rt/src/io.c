#include "rt/io.h"

#include "core/val/val-str.h"

#include <stdio.h>

void rt_io_print(const struct val_str *message) {
    printf("%s", message->data);
}

void rt_io_println(const struct val_str *message) {
    printf("%s\n", message->data);
}
