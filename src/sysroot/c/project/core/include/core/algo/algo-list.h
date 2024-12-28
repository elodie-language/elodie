#ifndef CORE_ALGO_LIST_H
#define CORE_ALGO_LIST_H

#include "core/mem/mem-api.h"
#include "core/algo/algo-iterator.h"

struct list_config {
  size_t initial_capacity;
  size_t resize_factor;
  struct mem *mem;
};

struct list {
  size_t count;
  size_t capacity;
  size_t stride;
  struct mem *mem;
  void *data;
};

ELODIE_API struct list_config
list_default_config (struct mem *mem);

ELODIE_API struct list *
prv_list_new (struct list_config config, size_t stride);

ELODIE_API void
prv_list_init (struct list *self, struct list_config config, size_t stride);

ELODIE_API void
list_copy_into (struct list *self, struct list *target);

ELODIE_API struct list *
list_append (struct list *self, void *val);

ELODIE_API struct list *
list_ensure_capacity (struct list *self, size_t required_capacity);

ELODIE_API void *
list_at (struct list *self, size_t idx);

ELODIE_API struct iterator
list_iterator (struct list *self);

ELODIE_API struct list *
list_replace (struct list *self, size_t idx, void *ptr);

ELODIE_API size_t
list_count (struct list *self);

ELODIE_API size_t
list_capacity (struct list *self);

ELODIE_API void
list_reset (struct list *self);

ELODIE_API void
list_free (struct list *self);

ELODIE_API void
list_free_safe (struct list **self);

#define list_new(config, type)  prv_list_new(config, sizeof(type))

#define list_init(lst, config, type) prv_list_init(lst, config, sizeof(type))

#define list_append_rval(lst, x)     do { \
        __auto_type temp = (x); \
        list_append((lst), &temp); \
    } while (0)

#define list_replace_rval(lst, idx, x)     do { \
        __auto_type temp = (x); \
        list_replace((lst),idx, &temp); \
    } while (0)

#endif //CORE_ALGO_LIST_H
