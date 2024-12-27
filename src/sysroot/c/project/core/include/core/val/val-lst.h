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
struct dep_val_str;
struct ptr_list;

struct val_lst {
  struct dep_val base;
  struct ptr_list *underlying_list;
};

HAMAL_API struct val_lst *
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
    struct dep_val_str*: val_lst_append_str,          \
    struct dep_val*: val_lst_append_base              \
)(self, T)

HAMAL_API void
val_lst_append_bool (struct val_lst *self, struct val_bool *val);

HAMAL_API void
val_lst_append_clsr (struct val_lst *self, struct val_clsr *val);

HAMAL_API void
val_lst_append_field (struct val_lst *self, struct val_fld *val);

HAMAL_API void
val_lst_append_fn (struct val_lst *self, struct val_fn *val);

HAMAL_API void
val_lst_append_list (struct val_lst *self, struct val_lst *val);

HAMAL_API void
val_lst_append_mod (struct val_lst *self, struct val_mod *val);

HAMAL_API void
val_lst_append_num (struct val_lst *self, struct val_num *val);

HAMAL_API void
val_lst_append_obj (struct val_lst *self, struct val_obj *val);

HAMAL_API void
val_lst_append_nil (struct val_lst *self, struct val_nil *val);

HAMAL_API void
val_lst_append_str (struct val_lst *self, struct dep_val_str *val);

HAMAL_API void
val_lst_append_base (struct val_lst *self, struct dep_val *val);

HAMAL_API void
val_lst_replace_base (struct val_lst *self, size_t idx, struct dep_val *val);

HAMAL_API struct dep_val *
val_lst_at_base (struct val_lst *self, size_t idx);

HAMAL_API struct val_bool *
val_lst_at_bool (struct val_lst *self, size_t idx);

HAMAL_API struct val_clsr *
val_lst_at_clsr (struct val_lst *self, size_t idx);

HAMAL_API struct val_fld *
val_lst_at_field (struct val_lst *self, size_t idx);

HAMAL_API struct val_fn *
val_lst_at_fn (struct val_lst *self, size_t idx);

HAMAL_API struct val_lst *
val_lst_at_list (struct val_lst *self, size_t idx);

HAMAL_API struct val_mod *
val_lst_at_mod (struct val_lst *self, size_t idx);

HAMAL_API struct val_num *
val_lst_at_num (struct val_lst *self, size_t idx);

HAMAL_API struct val_obj *
val_lst_at_obj (struct val_lst *self, size_t idx);

HAMAL_API struct dep_val_str *
val_lst_at_str (struct val_lst *self, size_t idx);

HAMAL_API size_t
val_lst_count (struct val_lst *self);

HAMAL_API size_t
val_lst_capacity (struct val_lst *self);

HAMAL_API void
val_lst_clear (struct val_lst *self);

HAMAL_API void
val_lst_free (struct val_lst *self);

HAMAL_API void
val_lst_free_safe (struct val_lst **self);

#endif //CORE_val_lst_H
