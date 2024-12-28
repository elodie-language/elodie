#ifndef CORE_ALGO_BYTE_LIST_H
#define CORE_ALGO_BYTE_LIST_H

#include "core/macro.h"
#include "core/mem/mem-api.h"
#include "core/val/val-str.h"
#include "core/algo/algo-list.h"

#include "core/bytes/bytes-view.h"

struct val;
struct val_str;
struct val_str_view;

struct byte_list_config {
  size_t initial_capacity;
  struct mem *mem;
};

struct byte_list {
  struct list underlying_list;
};

ELODIE_API struct byte_list_config
byte_list_default_config (struct mem *mem);

ELODIE_API struct byte_list *
byte_list_new (struct byte_list_config config);

ELODIE_API void
byte_list_init (struct byte_list *self, struct byte_list_config config);

ELODIE_API void
byte_list_append_u1 (struct byte_list *self, u1 data);

ELODIE_API void
byte_list_append_u2 (struct byte_list *self, u2 data);

ELODIE_API void
byte_list_append_u4 (struct byte_list *self, u4 data);

ELODIE_API void
byte_list_replace_u4 (struct byte_list *self, size_t idx, u4 data);

ELODIE_API void
byte_list_append_u8 (struct byte_list *self, u8 data);

ELODIE_API void
byte_list_append_c_str (struct byte_list *self, char const *str);

ELODIE_API void
byte_list_append_str_view (struct byte_list *self, struct val_str_view str);

ELODIE_API void
byte_list_append_str (struct byte_list *self, struct val_str *str);

ELODIE_API void
byte_list_append_bytes (struct byte_list *self, struct bytes_view data);

ELODIE_API void
byte_list_replace_bytes (struct byte_list *self, size_t idx, struct bytes_view data);

ELODIE_API void
byte_list_append_front_bytes (struct byte_list *self, struct bytes_view data);

ELODIE_API void
byte_list_append_byte_list (struct byte_list *self, struct byte_list *lst);

ELODIE_API void
byte_list_append_front_byte_list (struct byte_list *self, struct byte_list *lst);

ELODIE_API bool
byte_list_at_u1 (struct byte_list *self, size_t idx, u1 *out);

ELODIE_API bool
byte_list_at_u2 (struct byte_list *self, size_t idx, u2 *out);

ELODIE_API bool
byte_list_at_u4 (struct byte_list *self, size_t idx, u4 *out);

ELODIE_API bool
byte_list_at_str_view (struct byte_list *self, size_t idx, size_t count, struct val_str_view *out);

ELODIE_API bool
byte_list_at (struct byte_list *self, size_t idx, u1 **out);

ELODIE_API struct bytes_view
byte_list_raw_bytes (struct byte_list *self);

ELODIE_API void *
byte_list_data_ptr (struct byte_list *self);

ELODIE_API size_t
byte_list_size (struct byte_list *self);

ELODIE_API size_t
byte_list_capacity (struct byte_list *self);

ELODIE_API void
byte_list_reset (struct byte_list *self);

ELODIE_API void
byte_list_free (struct byte_list *self);

ELODIE_API void
byte_list_free_safe (struct byte_list **self);

#endif // CORE_ALGO_BYTE_LIST_H
