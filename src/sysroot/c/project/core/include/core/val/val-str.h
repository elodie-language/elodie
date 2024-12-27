#ifndef CORE_VAL_str_H
#define CORE_VAL_str_H

#include "val.h"
#include "val-ref.h"
#include "val-str-view.h"
#include "core/bytes/bytes-view.h"

struct byte_list;

struct val_str {
  struct val base;
  u4 count;
  char *data;
};

HAMAL_API struct val_ref
val_str_new_from_c_str (struct mem *mem, char const *src);

HAMAL_API struct val_ref
val_str_new_from_view (struct mem *mem, struct dep_val_str_view view);

HAMAL_API struct val_ref
val_str_new_from_bytes (struct mem *mem, struct bytes_view bytes);

HAMAL_API struct val_ref
val_str_new_from_byte_list (struct mem *mem, struct byte_list *lst);

HAMAL_API struct val_str *
internal_val_str_allocate_from_bytes (struct mem *mem, struct bytes_view bytes);

HAMAL_API void
internal_val_str_deallocate (struct val_str *self, struct mem *mem);

struct dep_val_str {
  struct dep_val base;
  size_t count;
  char *data;
};

// allocate / deallocate should be internal

//expects '\0' terminated c chars
HAMAL_API struct dep_val_str *
dep_val_str_allocate_from_c_str (struct mem *mem, char const *src);

HAMAL_API struct dep_val_str *
dep_val_str_allocate_from_bytes (struct mem *mem, struct bytes_view bytes);

HAMAL_API struct dep_val_str *
dep_val_str_allocate_from_byte_list (struct mem *mem, struct byte_list *src);

HAMAL_API struct dep_val_str *
dep_val_str_allocate_from_view (struct mem *mem, struct dep_val_str_view view);

HAMAL_API struct dep_val_str *
dep_val_str_copy (struct dep_val_str *src, struct mem *mem);

HAMAL_API size_t
dep_val_str_count (struct dep_val_str *self);

HAMAL_API bool
dep_val_str_equal (struct dep_val_str *lhs, struct dep_val_str *rhs);

HAMAL_API struct dep_val_str *
dep_val_str_concat (struct dep_val_str *self, struct dep_val_str *other, struct mem *mem);

HAMAL_API void
dep_val_str_deallocate (struct dep_val_str *self);

HAMAL_API void
dep_val_str_deallocate_safe (struct dep_val_str **self);

#endif //CORE_VAL_str_H
