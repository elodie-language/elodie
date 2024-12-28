#ifndef CORE_VAL_str_VIEW_H
#define CORE_VAL_str_VIEW_H

#include "val.h"
#include "core/bytes/bytes-view.h"

struct val_str;

struct val_str_view {
  struct val base;
  size_t count;
  char const *data;
};

ELODIE_API struct val_str_view
val_str_view_from_bytes (struct bytes_view bytes);

ELODIE_API struct val_str_view
val_str_view_from_str (struct val_str *ptr);

ELODIE_API struct val_str_view
val_str_view_from_c_str (char const *str);

ELODIE_API size_t
val_str_view_count (struct val_str_view *self);

ELODIE_API bool
val_str_view_equal (struct val_str_view lhs, struct val_str_view rhs);

#ifdef IS_UNIT_TEST

typedef struct val_str val_str_t;
typedef struct bytes_view bytes_view_t;

#define VAL_STR_VIEW(T) _Generic((T),               \
    char *:             val_str_view_from_c_str,    \
    char const *:       val_str_view_from_c_str,    \
    val_str_t *:        val_str_view_from_str,      \
    bytes_view_t:        val_str_view_from_bytes     \
)(T)

#else

#define VAL_STR_VIEW(T) _Generic((T),                            \
	char *:                val_str_view_from_c_str,        \
	char const *:                val_str_view_from_c_str,        \
	struct val_str *:            val_str_view_from_str,    \
	struct bytes_view:            val_str_view_from_bytes \
)(T)

#endif

#endif //CORE_VAL_str_VIEW_H
