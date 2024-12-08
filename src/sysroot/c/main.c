#include <stdio.h>
#include "core_intrinsics_math.h"
#include <stdbool.h>

const char *core_boolean_to_string(_Bool value) {
    if (value == true) {
        return "true";
    } else {
        return "false";
    }
}

int main(void) {
//    double result = core_intrinsics_math_cos_f64(0);
//    char str[20];
//    snprintf(str, 20, "%.1f", result);
    printf("%s", core_boolean_to_string(true));
    return 0;
}
