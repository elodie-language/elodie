#ifndef CORE_VAL_FN_H
#define CORE_VAL_FN_H

#include "val.h"

#include "val-str-view.h"

struct byte_list;
struct list;
struct dep_val_str;

// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-[val fn block]+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-

struct val_fn_block {
  struct mem *mem;
  struct byte_list *data;
};

HAMAL_API struct val_fn_block *
val_fn_block_new (struct mem *mem);

HAMAL_API void
val_fn_block_append (struct val_fn_block *self, u4 instr);

HAMAL_API size_t
val_fn_block_count (struct val_fn_block *self);

HAMAL_API void
val_fn_block_free (struct val_fn_block *self);

HAMAL_API void
val_fn_block_free_safe (struct val_fn_block **self);

// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-[val fn]+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-

struct val_fn {
  struct dep_val base;
  struct dep_val_str *ident;
  struct list *blocks;
};

HAMAL_API struct val_fn *
val_fn_new (struct mem *mem, struct dep_val_str_view ident);

HAMAL_API void
val_fn_append_block (struct val_fn *self, struct val_fn_block *block);

HAMAL_API struct val_fn_block *
val_fn_get_block_at (struct val_fn *self, size_t idx);

HAMAL_API size_t
val_fn_count (struct val_fn *self);

HAMAL_API bool
val_fn_equal (struct val_fn *lhs, struct val_fn *rhs);

HAMAL_API struct dep_val_str *
val_fn_to_str (struct val_fn *self, struct mem *mem);

HAMAL_API void
val_fn_free (struct val_fn *self);

HAMAL_API void
val_fn_free_safe (struct val_fn **self);

#endif //CORE_VAL_FN_H
