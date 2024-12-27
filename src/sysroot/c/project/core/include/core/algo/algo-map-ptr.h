#ifndef CORE_ALGO_MAP_PTR_H
#define CORE_ALGO_MAP_PTR_H

#include "algo-map.h"

struct ptr_map;
struct ptr_map_key { struct map_key underlying_key; };

HAMAL_API void
ptr_map_key_init (struct ptr_map_key *self, struct hash8 hash);

HAMAL_API struct ptr_map_key
ptr_map_key_from_bytes (struct ptr_map *self, struct bytes_view bytes);

HAMAL_API struct ptr_map_key
ptr_map_key_from_string_view (struct ptr_map *self, struct string_view view);

HAMAL_API struct ptr_map_key
ptr_map_key_from_size_t (struct ptr_map *self, size_t value);

HAMAL_API struct ptr_map_key
ptr_map_key_from_c_str (struct ptr_map *self, char const *str);

struct ptr_map_config {
  struct mem *mem;
  size_t initial_capacity;
  struct hash8_fn key_hash_fn;
};

struct ptr_map {
  struct map underlying_map;
};

HAMAL_API struct ptr_map *
ptr_map_new (struct ptr_map_config config);

HAMAL_API void
ptr_map_init (struct ptr_map *self, struct ptr_map_config config);

HAMAL_API bool
ptr_map_set (struct ptr_map *self, struct ptr_map_key key, void *ptr);

HAMAL_API bool
ptr_map_get (struct ptr_map *self, struct ptr_map_key key, void **out);

HAMAL_API bool
ptr_map_remove (struct ptr_map *self, struct ptr_map_key key);

HAMAL_API bool
ptr_map_has_key (struct ptr_map *self, struct ptr_map_key key);

HAMAL_API struct iterator
ptr_map_keys_iterator (struct ptr_map *self);

HAMAL_API u4
ptr_map_count (struct ptr_map *self);

HAMAL_API u4
ptr_map_capacity (struct ptr_map *self);

HAMAL_API void
ptr_map_reset (struct ptr_map *self);

HAMAL_API void
ptr_map_free (struct ptr_map *self);

HAMAL_API void
ptr_map_free_safe (struct ptr_map **self);

#endif //CORE_ALGO_MAP_PTR_H
