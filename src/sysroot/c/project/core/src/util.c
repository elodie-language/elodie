#include "stdio.h"
#include "core/core-api.h"

bool
is_digit(char c) {
    return c >= '0' && c <= '9';
}

bool
is_alpha(char c) {
    return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z');
}

bool
is_underscore(char c) {
    return c == '_';
}

bool
is_minus(char c) {
    return c == '-';
}

bool
is_colon(char c) {
    return c == ':';
}

bool
is_comma(char c) {
    return c == ',';
}

bool
is_quote(char c) {
    return c == '\"' || c == '\'';
}

bool
is_whitespace(char c) {
    return c == ' ' || c == '\t' || c == '\n' || c == '\r';
}

bool
is_hex_char(char c) {
    return is_digit(c) || (c >= 'a' && c <= 'f') || (c >= 'A' && c <= 'F');
}
