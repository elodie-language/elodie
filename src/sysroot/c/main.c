#include <stdio.h>
#include <math.h>
//#include "std_io.h"
int main(void) {
    double result = cos(0);
    char str[20];
    snprintf(str, 20, "%.1f", result);
    printf("%s",str);
    return 0;
}
