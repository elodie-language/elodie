#ifndef CORE_ALGO_SET_H
#define CORE_ALGO_SET_H

#include "core/algo/algo-hash.h"
#include "core/algo/algo-map.h"

struct set_config {
  struct mem *mem;
  size_t initial_capacity;
  struct hash8_fn hash_fn;
};

struct set {
  struct mem *mem;
  struct map store;
};

ELODIE_API struct set *
set_new (struct set_config config);

ELODIE_API void
set_init (struct set *self, struct set_config config);

ELODIE_API size_t
set_count (struct set *self);

ELODIE_API size_t
set_capacity (struct set *self);

ELODIE_API bool
set_set (struct set *self, struct bytes_view bytes);

ELODIE_API bool
set_has (struct set *self, struct bytes_view bytes);

ELODIE_API void
set_reset (struct set *self);

ELODIE_API void
set_free (struct set *self);

ELODIE_API void
set_free_safe (struct set **self);

#endif //CORE_ALGO_SET_H
