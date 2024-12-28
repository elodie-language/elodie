#ifndef CORE_ALGO_LIST_PTR_H
#define CORE_ALGO_LIST_PTR_H

#include "core/core.h"
#include "core/mem/mem.h"
#include "core/algo/algo-list.h"

struct ptr_list_config {
  size_t initial_capacity;
  struct mem *mem;
};

struct ptr_list {
  struct list underlying_list;
};

ELODIE_API struct ptr_list_config
ptr_list_default_config (struct mem *mem);

ELODIE_API struct ptr_list *
ptr_list_new (struct ptr_list_config config);

ELODIE_API void
ptr_list_init (struct ptr_list *self, struct ptr_list_config config);

ELODIE_API void
ptr_list_append (struct ptr_list *self, void *ptr);

ELODIE_API void *
ptr_list_at (struct ptr_list *self, size_t idx);

ELODIE_API struct iterator
ptr_list_iterator (struct ptr_list *self);

ELODIE_API void
ptr_list_replace (struct ptr_list *self, size_t idx, void *ptr);

ELODIE_API size_t
ptr_list_count (struct ptr_list *self);

ELODIE_API size_t
ptr_list_capacity (struct ptr_list *self);

ELODIE_API void
ptr_list_reset (struct ptr_list *self);

ELODIE_API void
ptr_list_free (struct ptr_list *self);

ELODIE_API void
ptr_list_free_safe (struct ptr_list **self);

#endif //CORE_ALGO_LIST_PTR_H
