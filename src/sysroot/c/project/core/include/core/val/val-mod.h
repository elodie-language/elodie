#ifndef CORE_VAL_MOD_H
#define CORE_VAL_MOD_H

#include "val.h"
#include "val-lst.h"
#include "val-str-view.h"

struct val_clsr;
struct val_lst;
struct val_obj;
struct dep_val_str;

// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-[val mod const pool]+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-

struct val_mod_const_pool {
  struct mem *mem;
  struct val_lst *strs;
  struct val_lst *fields;
  struct val_lst *objs;
//  struct val_list *nums;
//  struct val_list *lists;
//  struct val_list *types;
//  struct val_list *fns; ?
};

HAMAL_API void
val_mod_const_pool_init (struct val_mod_const_pool *self, struct mem *mem);

HAMAL_API void
val_mod_const_pool_append_str (struct val_mod_const_pool *self, struct dep_val_str_view str);

HAMAL_API void
val_mod_const_pool_append_field (struct val_mod_const_pool *self, struct val_fld *field);

HAMAL_API void
val_mod_const_pool_append_obj (struct val_mod_const_pool *self, struct val_obj *obj);

HAMAL_API struct dep_val_str_view
val_mod_const_pool_get_str_view (struct val_mod_const_pool *self, size_t idx);

HAMAL_API struct val_fld *
val_mod_const_pool_get_field (struct val_mod_const_pool *self, size_t idx);

HAMAL_API struct val_obj *
val_mod_const_pool_get_obj (struct val_mod_const_pool *self, size_t idx);

HAMAL_API void
val_mod_const_pool_reset (struct val_mod_const_pool *self);

// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-[val mod]+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-

struct val_mod {
  struct dep_val base;
  struct dep_val_str *ident;

  struct val_mod_const_pool const_pool;
  struct val_lst *clsrs;

  bool initialized;
  struct val_clsr *init;
};

HAMAL_API struct val_mod *
val_mod_new (struct mem *mem, struct dep_val_str_view ident, struct val_clsr *init);

HAMAL_API bool
val_mod_register_clsr (struct val_mod *self, struct val_clsr *clsr);

HAMAL_API bool
val_mod_resolve_clsr_id (struct val_mod *self, struct dep_val_str_view ident, u2 *out);

HAMAL_API void
val_mod_clear (struct val_mod *self);

HAMAL_API void
val_mod_free (struct val_mod *self);

HAMAL_API void
val_mod_free_safe (struct val_mod **self);

#endif //CORE_VAL_MOD_H
