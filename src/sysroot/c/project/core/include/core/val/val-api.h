#ifndef CORE_VAL_API_H
#define CORE_VAL_API_H

#include "val.h"
#include "val-bool.h"
#include "val-fld.h"
#include "val-fn.h"
#include "val-lst.h"
#include "val-num.h"
#include "val-num-f4.h"
#include "val-num-f8.h"
#include "val-num-i1.h"
#include "val-num-i2.h"
#include "val-num-i4.h"
#include "val-num-i8.h"
#include "val-num-i16.h"

#include "val-num-u1.h"
#include "val-num-u2.h"
#include "val-num-u4.h"
#include "val-num-u8.h"
#include "val-num-u16.h"

#include "val-obj.h"
#include "val-prop.h"
#include "val-ref.h"
#include "val-str.h"
#include "val-str-view.h"
#include "val-unit.h"
#include "val-writer.h"

ELODIE_API struct val *
val_copy(struct val *self, struct mem *mem);

ELODIE_API struct val_str *
val_to_str(struct val *self, struct mem *mem);

ELODIE_API bool
val_equal(struct val *lhs, struct val *rhs);

ELODIE_API void
val_clear(struct val *self);

ELODIE_API void
val_free(struct val *self);

ELODIE_API void
val_free_safe(struct val **self);

ELODIE_API bool
_val_equal_str_str_view(struct val_str *lhs, struct val_str_view rhs);

ELODIE_API bool
_val_equal_str_view_str(struct val_str_view lhs, struct val_str *rhs);

ELODIE_API bool
_val_equal_str_c_str(struct val_str *lhs, char const *rhs);

ELODIE_API bool
_val_equal_str_view_c_str(struct val_str_view lhs, char const *rhs);

#ifdef IS_UNIT_TEST

typedef struct val_str val_str_t;
typedef struct val_str_view val_str_view_t;

#define INTERNAL_VAL_EQ_STR_VIEW(RHS) _Generic((RHS), \
    val_str_t*: _val_equal_str_view_str,          \
    val_str_view_t: val_str_view_equal,          \
    char const*: _val_equal_str_view_c_str           \
)

#define INTERNAL_VAL_EQ_STR(RHS) _Generic((RHS),     \
    char const*: _val_equal_str_c_str,               \
    val_str_t*:  val_str_equal,                 \
    val_str_view:  _val_equal_str_str_view  \
)

#define VAL_EQ(LHS, RHS) _Generic((LHS), \
    val_str_t*: INTERNAL_VAL_EQ_STR(RHS), \
    val_str_view_t: INTERNAL_VAL_EQ_STR_VIEW(RHS) \
)(LHS, RHS)

#else

#define INTERNAL_VAL_EQ_STR_VIEW(RHS) _Generic((RHS), \
    struct val_str*: _val_equal_str_view_str,          \
    struct val_str_view: val_str_view_equal,          \
    char const*: _val_equal_str_view_c_str,           \
    char *: _val_equal_str_view_c_str           \
)

#define INTERNAL_VAL_EQ_STR(RHS) _Generic((RHS),     \
    char const*: _val_equal_str_c_str,               \
    char *: _val_equal_str_c_str,               \
    struct val_str*:  val_str_equal,                 \
    struct val_str_view:  _val_equal_str_str_view  \
)

#define VAL_EQ(LHS, RHS) _Generic((LHS), \
    struct val_str*:  INTERNAL_VAL_EQ_STR(RHS), \
    struct val_str_view: INTERNAL_VAL_EQ_STR_VIEW(RHS) \
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
#define AS_VAL(T) (struct val*)T

ELODIE_API struct val_bool *
val_as_bool(struct val *val);

ELODIE_API struct val_clsr *
val_as_clsr(struct val *val);

ELODIE_API struct val_lst *
val_as_lst(struct val *val);

ELODIE_API struct val_mod *
val_as_mod(struct val *val);

ELODIE_API struct val_num *
val_as_num(struct val *val);

ELODIE_API struct val_obj *
val_as_obj(struct val *val);

ELODIE_API struct val_prop *
val_as_prop(struct val *val);

ELODIE_API struct val_str *
val_as_str(struct val *val);

#endif //CORE_VAL_API_H
