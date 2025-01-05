#include <stdbool.h>
#include <stdio.h>
#include "core/string/string-api.h"
#include "core/core-api.h"
#include "rt/io.h"
int main(void){

    auto tm = mem_test_new_default (1024 * 1024 );

    struct val_f4 * n1_1 = val_f4_new(MEM(tm), 2);
    struct val_f4 * n2_2 = val_f4_new(MEM(tm), 4);
    struct val_str * temp_1 = val_str_new_from_c_str(MEM(tm), "n1 + n2 => ");
    struct val_num * temp_3 = val_f4_calc(MEM(tm), n1_1, CALCULATE_OPERATOR_ADD_WRAP_AROUND, n2_2);
    struct val_str * temp_2 = val_to_str(((struct val *)temp_3), MEM(tm));
    char temp_4[100];
    snprintf(temp_4, 100, "%s%s", temp_1->data, temp_2->data);
    const struct val_str * arg_1 = val_str_new_from_c_str(MEM(tm), temp_4);
    rt_io_println(arg_1);
}
