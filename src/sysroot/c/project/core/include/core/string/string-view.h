#ifndef CORE_STRING_VIEW_H
#define CORE_STRING_VIEW_H

#include "core/bytes/bytes-view.h"

struct string;

struct string_view {
  u4 length;
  char const *data;
};

ELODIE_API struct string_view
string_view_from_bytes (struct bytes_view bytes);

ELODIE_API struct string_view
string_view_from_str (struct string str);

ELODIE_API struct string_view
string_view_from_str_ptr (struct string *ptr);

ELODIE_API struct string_view
string_view_from_c_str (char const *str);

ELODIE_API struct bytes_view
string_view_as_byte_view (struct string_view self);

ELODIE_API u4
string_view_count (struct string_view self);

ELODIE_API bool
string_view_equal (struct string_view lhs, struct string_view rhs);

ELODIE_API bool
string_view_equal_c_str (struct string_view lhs, char const *rhs);

ELODIE_API bool
string_view_last_occurrence_of (struct string_view self, struct string_view pattern, u4 *position);

#ifdef IS_UNIT_TEST

typedef struct string string_t;
typedef struct string_view string_view_t;
typedef struct bytes_view bytes_view_t;

#define STRING_VIEW(T) _Generic((T),               \
	char *:             string_view_from_c_str,    \
	char const *:       string_view_from_c_str,    \
	string_t *:         string_view_from_str_ptr,      \
	bytes_view_t:        string_view_from_bytes     \
)(T)

#define STRING_VIEW_EQUAL(LHS, RHS) _Generic((RHS),         \
	string_view_t:         string_view_equal,               \
	char const *:            string_view_equal_c_str,        \
	char *:            string_view_equal_c_str        \
)(LHS, RHS)

#else

#define STRING_VIEW(T) _Generic((T),                \
    char *:                 string_view_from_c_str,    \
    char const *:           string_view_from_c_str,    \
    struct string *:        string_view_from_str_ptr,      \
    struct string:          string_view_from_str,      \
    struct bytes_view:      string_view_from_bytes     \
)(T)

#define STRING_VIEW_EQUAL(LHS, RHS) _Generic((RHS),         \
    struct string_view:     string_view_equal,          \
    char const*:            string_view_equal_c_str,          \
    char *:            string_view_equal_c_str          \
)(LHS, RHS)

#endif

#endif //CORE_STRING_VIEW_H
