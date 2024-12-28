#ifndef CORE_val_lst_H
#define CORE_val_lst_H

#include "val.h"

struct val_bool;
struct val_clsr;
struct val_fld;
struct val_fn;
struct val_mod;
struct val_nil;
struct val_num;
struct val_obj;
struct val_str;
struct ptr_list;

struct val_lst {
  struct val base;
  struct ptr_list *underlying_list;
};

ELODIE_API struct val_lst *
val_lst_new (struct mem *mem);

#define val_lst_append(self, T) _Generic((T),       \
    struct val_bool*: val_lst_append_bool,        \
    struct val_clsr*: val_lst_append_clsr,        \
    struct val_fld*: val_lst_append_field,      \
    struct val_fn*: val_lst_append_fn,            \
    struct val_lst*: val_lst_append_list,        \
    struct val_mod*: val_lst_append_mod,        \
    struct val_nil*: val_lst_append_nil,          \
    struct val_num*: val_lst_append_num,          \
    struct val_obj*: val_lst_append_obj,          \
    struct val_str*: val_lst_append_str,          \
    struct val*: val_lst_append_base              \
)(self, T)

ELODIE_API void
val_lst_append_bool (struct val_lst *self, struct val_bool *val);

ELODIE_API void
val_lst_append_clsr (struct val_lst *self, struct val_clsr *val);

ELODIE_API void
val_lst_append_field (struct val_lst *self, struct val_fld *val);

ELODIE_API void
val_lst_append_fn (struct val_lst *self, struct val_fn *val);

ELODIE_API void
val_lst_append_list (struct val_lst *self, struct val_lst *val);

ELODIE_API void
val_lst_append_mod (struct val_lst *self, struct val_mod *val);

ELODIE_API void
val_lst_append_num (struct val_lst *self, struct val_num *val);

ELODIE_API void
val_lst_append_obj (struct val_lst *self, struct val_obj *val);

ELODIE_API void
val_lst_append_nil (struct val_lst *self, struct val_nil *val);

ELODIE_API void
val_lst_append_str (struct val_lst *self, struct val_str *val);

ELODIE_API void
val_lst_append_base (struct val_lst *self, struct val *val);

ELODIE_API void
val_lst_replace_base (struct val_lst *self, size_t idx, struct val *val);

ELODIE_API struct val *
val_lst_at_base (struct val_lst *self, size_t idx);

ELODIE_API struct val_bool *
val_lst_at_bool (struct val_lst *self, size_t idx);

ELODIE_API struct val_clsr *
val_lst_at_clsr (struct val_lst *self, size_t idx);

ELODIE_API struct val_fld *
val_lst_at_field (struct val_lst *self, size_t idx);

ELODIE_API struct val_fn *
val_lst_at_fn (struct val_lst *self, size_t idx);

ELODIE_API struct val_lst *
val_lst_at_list (struct val_lst *self, size_t idx);

ELODIE_API struct val_mod *
val_lst_at_mod (struct val_lst *self, size_t idx);

ELODIE_API struct val_num *
val_lst_at_num (struct val_lst *self, size_t idx);

ELODIE_API struct val_obj *
val_lst_at_obj (struct val_lst *self, size_t idx);

ELODIE_API struct val_str *
val_lst_at_str (struct val_lst *self, size_t idx);

ELODIE_API size_t
val_lst_count (struct val_lst *self);

ELODIE_API size_t
val_lst_capacity (struct val_lst *self);

ELODIE_API void
val_lst_clear (struct val_lst *self);

ELODIE_API void
val_lst_free (struct val_lst *self);

ELODIE_API void
val_lst_free_safe (struct val_lst **self);

#endif //CORE_val_lst_H
