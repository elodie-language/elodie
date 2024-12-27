#ifndef CORE_ALGO_MAP_H
#define CORE_ALGO_MAP_H

#include "core/algo/algo-hash.h"
#include "core/algo/algo-iterator.h"
#include "core/val/val-str-view.h"
#include "core/string/string-api.h"
#include "core/bytes/byte-api.h"

struct map;
struct map_key;
struct dep_val_str;

struct map_key { struct hash8 hash; };

HAMAL_API void
map_key_init (struct map_key *self, struct hash8 hash);

HAMAL_API struct map_key
map_key_from_bytes (struct map *self, struct bytes_view bytes);

HAMAL_API struct map_key
map_key_from_str (struct map *self, struct dep_val_str *str);

HAMAL_API struct map_key
dep_map_key_from_str_view (struct map *self, struct dep_val_str_view view);

HAMAL_API struct map_key
map_key_from_string_view (struct map *self, struct string_view view);

HAMAL_API struct map_key
map_key_from_size_t (struct map *self, size_t value);

HAMAL_API struct map_key
map_key_from_c_str (struct map *self, char const *str);

#ifdef IS_UNIT_TEST
#include "core/val/val-str-view.h"
typedef struct dep_val_str_view dep_val_str_view_t;
typedef struct string_view string_view_t;
#define MAP_KEY(map, T) _Generic((T),                    \
    int:                 map_key_from_size_t,             \
    size_t:              map_key_from_size_t,             \
    char const*:         map_key_from_c_str,              \
    struct val_str*:     map_key_from_str,                \
    dep_val_str_view_t: dep_map_key_from_str_view,            \
    string_view_t:         map_key_from_string_view            \
)(map, T)
#else
#define MAP_KEY(map, T) _Generic((T),                    \
	int:                 map_key_from_size_t,             \
	size_t:              map_key_from_size_t,             \
	char const*:         map_key_from_c_str,              \
	struct dep_val_str*:  map_key_from_str,                \
	struct dep_val_str_view: dep_map_key_from_str_view,            \
	struct string_view: map_key_from_string_view            \
)(map, T)
#endif

struct map_entry {
  struct map_key key;
  struct bytes value;
};

struct map_entry_view {
  struct map_key key;
  struct bytes_view value;
};

struct map_config {
  struct mem *mem;
  size_t initial_capacity;
  struct hash8_fn key_hash_fn;
};

struct map {
  struct mem *mem;
  struct hash8_fn key_hash_fn;
  size_t count;
  size_t capacity;
  struct map_entry *entries;
};
HAMAL_API struct map *
map_new (struct map_config config);

HAMAL_API void
map_init (struct map *self, struct map_config config);

HAMAL_API struct map *
map_copy (struct map *self, struct mem *mem);

HAMAL_API void
map_copy_into (struct map *self, struct map *target);

HAMAL_API size_t
map_count (struct map *self);

HAMAL_API size_t
map_capacity (struct map *self);

HAMAL_API bool
map_set_bytes_view (struct map *self, struct map_key key, struct bytes_view bytes);

HAMAL_API bool
map_get_as_entry_view (struct map *self, struct map_key key, struct map_entry_view *out);

HAMAL_API bool
map_get_as_bytes_view (struct map *self, struct map_key key, struct bytes_view *out);

HAMAL_API bool
map_has_key (struct map *self, struct map_key key);

HAMAL_API struct iterator
map_keys_iterator (struct map *self);

HAMAL_API bool
map_remove (struct map *self, struct map_key key);

HAMAL_API void
map_reset (struct map *self);

HAMAL_API void
map_free (struct map *self);

HAMAL_API void
map_free_safe (struct map **self);

#endif //CORE_ALGO_MAP_H
