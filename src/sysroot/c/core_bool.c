#include "core_bool.h"

const char *core_bool_to_string(_Bool value) {
    return value ? "true" : "false";
}