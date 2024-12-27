#ifndef CORE_ALGO_RING_BUFFER_H
#define CORE_ALGO_RING_BUFFER_H

#include "core/macro.h"
#include "core/algo/algo-iterator.h"

struct mem;

struct ring_buffer {
  size_t capacity;
  size_t position;
  size_t stride;
  bool full;
  struct mem *mem;
  u1 *data;
};

HAMAL_API struct ring_buffer *
prv_ring_buffer_new (struct mem *mem, size_t capacity, size_t stride);

HAMAL_API void
prv_ring_buffer_init (struct ring_buffer *self, struct mem *mem, size_t capacity, size_t stride);

HAMAL_API void
ring_buffer_append (struct ring_buffer *self, void *val);

HAMAL_API void *
ring_buffer_at (struct ring_buffer *self, size_t idx);

HAMAL_API struct iterator
ring_buffer_iterator (struct ring_buffer *self);

HAMAL_API bool
ring_buffer_is_full (struct ring_buffer *self);

HAMAL_API size_t
ring_buffer_count (struct ring_buffer *self);

HAMAL_API size_t
ring_buffer_capacity (struct ring_buffer *self);

HAMAL_API void
ring_buffer_reset (struct ring_buffer *self);

HAMAL_API void
ring_buffer_free (struct ring_buffer *self);

HAMAL_API void
ring_buffer_free_safe (struct ring_buffer **self);

#define ring_buffer_new(mem, capacity, type)  prv_ring_buffer_new(mem, capacity, sizeof(type))

#define ring_buffer_init(rb, mem, capacity, type) prv_ring_buffer_init(rb, mem, capacity, sizeof(type))

#define ring_buffer_append_rval(lst, x)     do { \
        __auto_type temp = (x); \
        ring_buffer_append((lst), &temp); \
    } while (0)

#endif //CORE_ALGO_RING_BUFFER_H
