#include <stdio.h>
#include "core_intrinsics_math.h"

int main(void) {
    double result = core_intrinsics_math_cos_f64(0);
    char str[20];
    snprintf(str, 20, "%.1f", result);
    printf("%s",str);
    return 0;
}
