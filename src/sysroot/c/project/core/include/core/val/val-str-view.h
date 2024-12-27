#ifndef CORE_VAL_str_VIEW_H
#define CORE_VAL_str_VIEW_H

#include "val.h"
#include "core/bytes/bytes-view.h"

struct dep_val_str;

struct dep_val_str_view {
  struct dep_val base;
  size_t count;
  char const *data;
};

HAMAL_API struct dep_val_str_view
dep_val_str_view_from_bytes (struct bytes_view bytes);

HAMAL_API struct dep_val_str_view
dep_val_str_view_from_str (struct dep_val_str *ptr);

HAMAL_API struct dep_val_str_view
dep_val_str_view_from_c_str (char const *str);

HAMAL_API size_t
dep_val_str_view_count (struct dep_val_str_view *self);

HAMAL_API bool
dep_val_str_view_equal (struct dep_val_str_view lhs, struct dep_val_str_view rhs);

#ifdef IS_UNIT_TEST

typedef struct dep_val_str val_str_t;
typedef struct bytes_view bytes_view_t;

#define DEP_VAL_STR_VIEW(T) _Generic((T),               \
    char *:             dep_val_str_view_from_c_str,    \
    char const *:       dep_val_str_view_from_c_str,    \
    val_str_t *:        dep_val_str_view_from_str,      \
    bytes_view_t:        dep_val_str_view_from_bytes     \
)(T)

#else

#define DEP_VAL_STR_VIEW(T) _Generic((T),                            \
	char *:                dep_val_str_view_from_c_str,        \
	char const *:                dep_val_str_view_from_c_str,        \
	struct dep_val_str *:            dep_val_str_view_from_str,    \
	struct bytes_view:            dep_val_str_view_from_bytes \
)(T)

#endif

#endif //CORE_VAL_str_VIEW_H
