#ifndef CORE_VAL_str_H
#define CORE_VAL_str_H

#include "val.h"
#include "val-ref.h"
#include "val-str-view.h"
#include "core/bytes/bytes-view.h"

struct byte_list;

struct val_str {
  struct val base;
  size_t count;
  char *data;
};


//expects '\0' terminated c chars
ELODIE_API struct val_str *
val_str_new_from_c_str (struct mem *mem, char const *src);

ELODIE_API struct val_str *
val_str_new_from_bytes (struct mem *mem, struct bytes_view bytes);

ELODIE_API struct val_str *
val_str_new_from_byte_list (struct mem *mem, struct byte_list *src);

ELODIE_API struct val_str *
val_str_new_from_view (struct mem *mem, struct val_str_view view);

ELODIE_API struct val_str *
val_str_copy (struct val_str *src, struct mem *mem);

ELODIE_API size_t
val_str_count (struct val_str *self);

ELODIE_API bool
val_str_equal (struct val_str *lhs, struct val_str *rhs);

ELODIE_API struct val_str *
val_str_concat (struct val_str *self, struct val_str *other, struct mem *mem);

ELODIE_API void
val_str_free (struct val_str *self);

ELODIE_API void
val_str_free_safe (struct val_str **self);

#endif //CORE_VAL_str_H
