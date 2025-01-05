#include "core/string/string-api.h"
#include <stdio.h>
#include <stdbool.h>
#include "rt/io.h"
#include "core/core-api.h"
int main(void){

    auto tm = mem_test_new_default (1024 * 1024 );

    struct val_i1 * n1_1 = val_i1_new(MEM(tm), 1);
    struct val_i1 * n2_2 = val_i1_new(MEM(tm), 1);
    struct val_i1 * n3_3 = val_i1_new(MEM(tm), 2);
    struct val_str * temp_1 = val_str_new_from_c_str(MEM(tm), "n1 == n1 => ");
    struct val_bool * temp_3 = val_i1_cmp(MEM(tm), n1_1, COMPARE_OPERATOR_EQUAL, n1_1);
    struct val_str * temp_2 = val_bool_to_str(temp_3, MEM(tm));
    char temp_4[100];
    snprintf(temp_4, 100, "%s%s", temp_1->data, temp_2->data);
    const struct val_str * arg_1 = val_str_new_from_c_str(MEM(tm), temp_4);
    rt_io_println(arg_1);
    struct val_str * temp_5 = val_str_new_from_c_str(MEM(tm), "n1 == n2 => ");
    struct val_bool * temp_7 = val_i1_cmp(MEM(tm), n1_1, COMPARE_OPERATOR_EQUAL, n2_2);
    struct val_str * temp_6 = val_bool_to_str(temp_7, MEM(tm));
    char temp_8[100];
    snprintf(temp_8, 100, "%s%s", temp_5->data, temp_6->data);
    const struct val_str * arg_2 = val_str_new_from_c_str(MEM(tm), temp_8);
    rt_io_println(arg_2);
    struct val_str * temp_9 = val_str_new_from_c_str(MEM(tm), "n2 == n1 => ");
    struct val_bool * temp_11 = val_i1_cmp(MEM(tm), n2_2, COMPARE_OPERATOR_EQUAL, n1_1);
    struct val_str * temp_10 = val_bool_to_str(temp_11, MEM(tm));
    char temp_12[100];
    snprintf(temp_12, 100, "%s%s", temp_9->data, temp_10->data);
    const struct val_str * arg_3 = val_str_new_from_c_str(MEM(tm), temp_12);
    rt_io_println(arg_3);
    struct val_str * temp_13 = val_str_new_from_c_str(MEM(tm), "n1 == n3 => ");
    struct val_bool * temp_15 = val_i1_cmp(MEM(tm), n1_1, COMPARE_OPERATOR_EQUAL, n3_3);
    struct val_str * temp_14 = val_bool_to_str(temp_15, MEM(tm));
    char temp_16[100];
    snprintf(temp_16, 100, "%s%s", temp_13->data, temp_14->data);
    const struct val_str * arg_4 = val_str_new_from_c_str(MEM(tm), temp_16);
    rt_io_println(arg_4);
    struct val_str * temp_17 = val_str_new_from_c_str(MEM(tm), "n3 == n1 => ");
    struct val_bool * temp_19 = val_i1_cmp(MEM(tm), n3_3, COMPARE_OPERATOR_EQUAL, n1_1);
    struct val_str * temp_18 = val_bool_to_str(temp_19, MEM(tm));
    char temp_20[100];
    snprintf(temp_20, 100, "%s%s", temp_17->data, temp_18->data);
    const struct val_str * arg_5 = val_str_new_from_c_str(MEM(tm), temp_20);
    rt_io_println(arg_5);
    val_rc_dec(((struct val *)arg_1));
    val_rc_dec(((struct val *)arg_2));
    val_rc_dec(((struct val *)arg_3));
    val_rc_dec(((struct val *)arg_4));
    val_rc_dec(((struct val *)arg_5));
    val_rc_dec(((struct val *)n3_3));
    val_rc_dec(((struct val *)n2_2));
    val_rc_dec(((struct val *)n1_1));
    val_rc_dec(((struct val *)temp_1));
    val_rc_dec(((struct val *)temp_1));
    val_rc_dec(((struct val *)temp_3));
    val_rc_dec(((struct val *)temp_5));
    val_rc_dec(((struct val *)temp_7));
    val_rc_dec(((struct val *)temp_9));
    val_rc_dec(((struct val *)temp_11));
    val_rc_dec(((struct val *)temp_13));
    val_rc_dec(((struct val *)temp_15));
    val_rc_dec(((struct val *)temp_17));
    val_rc_dec(((struct val *)temp_19));

    mem_test_verify (tm);
    mem_test_free (tm);

}
