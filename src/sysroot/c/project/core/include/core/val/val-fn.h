#ifndef CORE_VAL_FN_H
#define CORE_VAL_FN_H

#include "val.h"

#include "val-str-view.h"

struct byte_list;
struct list;
struct val_str;

// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-[val fn block]+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-

struct val_fn_block {
  struct mem *mem;
  struct byte_list *data;
};

ELODIE_API struct val_fn_block *
val_fn_block_new (struct mem *mem);

ELODIE_API void
val_fn_block_append (struct val_fn_block *self, u4 instr);

ELODIE_API size_t
val_fn_block_count (struct val_fn_block *self);

ELODIE_API void
val_fn_block_free (struct val_fn_block *self);

ELODIE_API void
val_fn_block_free_safe (struct val_fn_block **self);

// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-[val fn]+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-

struct val_fn {
  struct val base;
  struct val_str *ident;
  struct list *blocks;
};

ELODIE_API struct val_fn *
val_fn_new (struct mem *mem, struct val_str_view ident);

ELODIE_API void
val_fn_append_block (struct val_fn *self, struct val_fn_block *block);

ELODIE_API struct val_fn_block *
val_fn_get_block_at (struct val_fn *self, size_t idx);

ELODIE_API size_t
val_fn_count (struct val_fn *self);

ELODIE_API bool
val_fn_equal (struct val_fn *lhs, struct val_fn *rhs);

ELODIE_API struct val_str *
val_fn_to_str (struct val_fn *self, struct mem *mem);

ELODIE_API void
val_fn_free (struct val_fn *self);

ELODIE_API void
val_fn_free_safe (struct val_fn **self);

#endif //CORE_VAL_FN_H
