#ifndef CORE_VAL_API_H
#define CORE_VAL_API_H

#include "val.h"
#include "val-bool.h"
#include "val-clsr.h"
#include "val-fld.h"
#include "val-fn.h"
#include "val-lst.h"
#include "val-mod.h"
#include "val-nil.h"
#include "val-num.h"
#include "val-obj.h"
#include "val-prop.h"
#include "val-ref.h"
#include "val-str.h"
#include "val-str-view.h"
#include "val-unit.h"
#include "val-writer.h"

HAMAL_API struct dep_val *
val_copy (struct dep_val *self, struct mem *mem);

HAMAL_API struct dep_val_str *
val_to_str (struct dep_val *self, struct mem *mem);

HAMAL_API bool
val_equal (struct dep_val *lhs, struct dep_val *rhs);

HAMAL_API void
val_clear (struct dep_val *self);

HAMAL_API void
val_free (struct dep_val *self);

HAMAL_API void
val_free_safe (struct dep_val **self);

HAMAL_API bool
_val_equal_str_str_view (struct dep_val_str *lhs, struct dep_val_str_view rhs);

HAMAL_API bool
_val_equal_str_view_str (struct dep_val_str_view lhs, struct dep_val_str *rhs);

HAMAL_API bool
_val_equal_str_c_str (struct dep_val_str *lhs, char const *rhs);

HAMAL_API bool
_val_equal_str_view_c_str (struct dep_val_str_view lhs, char const *rhs);

#ifdef IS_UNIT_TEST

typedef struct dep_val_str val_str_t;
typedef struct dep_val_str_view val_str_view_t;

#define INTERNAL_VAL_EQ_STR_VIEW(RHS) _Generic((RHS), \
    val_str_t*: _val_equal_str_view_str,          \
    val_str_view_t: dep_val_str_view_equal,          \
    char const*: _val_equal_str_view_c_str           \
)

#define INTERNAL_VAL_EQ_STR(RHS) _Generic((RHS),     \
    char const*: _val_equal_str_c_str,               \
    val_str_t*:  dep_val_str_equal,                 \
    dep_val_str_view:  _val_equal_str_str_view  \
)

#define VAL_EQ(LHS, RHS) _Generic((LHS), \
    val_str_t*: INTERNAL_VAL_EQ_STR(RHS), \
    val_str_view_t: INTERNAL_VAL_EQ_STR_VIEW(RHS) \
)(LHS, RHS)

#else

#define INTERNAL_VAL_EQ_STR_VIEW(RHS) _Generic((RHS), \
	struct dep_val_str*: _val_equal_str_view_str,          \
	struct dep_val_str_view: dep_val_str_view_equal,          \
	char const*: _val_equal_str_view_c_str,           \
	char *: _val_equal_str_view_c_str           \
)

#define INTERNAL_VAL_EQ_STR(RHS) _Generic((RHS),     \
	char const*: _val_equal_str_c_str,               \
	char *: _val_equal_str_c_str,               \
	struct dep_val_str*:  dep_val_str_equal,                 \
	struct dep_val_str_view:  _val_equal_str_str_view  \
)

#define VAL_EQ(LHS, RHS) _Generic((LHS), \
	struct dep_val_str*:  INTERNAL_VAL_EQ_STR(RHS), \
	struct dep_val_str_view: INTERNAL_VAL_EQ_STR_VIEW(RHS) \
)(LHS, RHS)

#endif

#define AS_BOOL(T) val_as_bool(T)
#define AS_CLSR(T) val_as_clsr(T)
#define AS_LST(T) val_as_lst(T)
#define AS_MOD(T) val_as_mod(T)
#define AS_NUM(T) val_as_num(T)
#define AS_OBJ(T) val_as_obj(T)
#define AS_PROP(T) val_as_prop(T)
#define AS_STR(T) val_as_str(T)
#define AS_VAL(T) (struct dep_val*)T

HAMAL_API  struct val_bool *
val_as_bool (struct dep_val *val);

HAMAL_API  struct val_clsr *
val_as_clsr (struct dep_val *val);

HAMAL_API  struct val_lst *
val_as_lst (struct dep_val *val);

HAMAL_API  struct val_mod *
val_as_mod (struct dep_val *val);

HAMAL_API  struct val_num *
val_as_num (struct dep_val *val);

HAMAL_API  struct val_obj *
val_as_obj (struct dep_val *val);

HAMAL_API  struct val_prop *
val_as_prop (struct dep_val *val);

HAMAL_API  struct dep_val_str *
val_as_str (struct dep_val *val);

#endif //CORE_VAL_API_H
