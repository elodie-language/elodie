#ifndef CORE_UTIL_H
#define CORE_UTIL_H

#include <stdbool.h>

#include "macro.h"

HAMAL_API bool
is_digit (char c);

HAMAL_API bool
is_alpha (char c);

HAMAL_API bool
is_underscore (char c);

HAMAL_API bool
is_minus (char c);

HAMAL_API bool
is_colon (char c);

HAMAL_API bool
is_comma (char c);

HAMAL_API bool
is_quote (char c);

HAMAL_API bool
is_whitespace (char c);

HAMAL_API bool
is_hex_char (char c);

#endif //CORE_UTIL_H
