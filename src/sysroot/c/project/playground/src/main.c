#include <stdbool.h>
#include "core/core-api.h"
#include "core/string/string-api.h"
#include "rt/io.h"
#include <stdio.h>


int main(void) {

    auto tm = mem_test_new_default(1024 * 1024);

    struct val_i1 *number_1 = val_i1_new(MEM(tm), 1);
    struct val_str *temp_1 = val_i1_to_str(number_1, MEM(tm));
    char temp_2[100];
    snprintf(temp_2, 100, "%s", temp_1->data);
    const struct val_str *arg_1 = val_str_new_from_c_str(MEM(tm), temp_2);
    rt_io_println(arg_1);
}