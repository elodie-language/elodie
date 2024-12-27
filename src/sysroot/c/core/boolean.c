#include "boolean.h"

const char *core_bool_to_string(_Bool value) {
    return value ? "true" : "false";
}