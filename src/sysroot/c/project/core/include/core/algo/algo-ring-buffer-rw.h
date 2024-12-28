#ifndef CORE_ALGO_RING_BUFFER_RW_H
#define CORE_ALGO_RING_BUFFER_RW_H

#include "algo-ring-buffer.h"

/**
 * A rw_ring_buffer (read write ring buffer) is a normal ring buffer, which keeps track of the
 * read position as well. Which comes handy when reading and writing happen independently of each other.
 */
struct rw_ring_buffer {
  struct ring_buffer underlying;
  size_t read_position;
};

ELODIE_API struct rw_ring_buffer *
prv_rw_ring_buffer_new (struct mem *mem, size_t capacity, size_t stride);

ELODIE_API void
prv_rw_ring_buffer_init (struct rw_ring_buffer *self, struct mem *mem, size_t capacity, size_t stride);

ELODIE_API void
rw_ring_buffer_append (struct rw_ring_buffer *self, void *val);

ELODIE_API void *
rw_ring_buffer_next (struct rw_ring_buffer *self);

ELODIE_API void *
rw_ring_buffer_current (struct rw_ring_buffer *self);

ELODIE_API void *
rw_ring_buffer_peek_next (struct rw_ring_buffer *self, size_t offset);

ELODIE_API bool
rw_ring_buffer_is_full (struct rw_ring_buffer *self);

ELODIE_API size_t
rw_ring_buffer_read_position (struct rw_ring_buffer *self);

ELODIE_API size_t
rw_ring_buffer_write_position (struct rw_ring_buffer *self);

ELODIE_API size_t
rw_ring_buffer_count (struct rw_ring_buffer *self);

ELODIE_API size_t
rw_ring_buffer_capacity (struct rw_ring_buffer *self);

ELODIE_API void
rw_ring_buffer_reset (struct rw_ring_buffer *self);

ELODIE_API void
rw_ring_buffer_free (struct rw_ring_buffer *self);

ELODIE_API void
rw_ring_buffer_free_safe (struct rw_ring_buffer **self);

#define rw_ring_buffer_new(mem, capacity, type)  prv_rw_ring_buffer_new(mem, capacity, sizeof(type))

#define rw_ring_buffer_init(rb, mem, capacity, type) prv_rw_ring_buffer_init(rb, mem, capacity, sizeof(type))

#define rw_ring_buffer_append_rval(lst, x)     do { \
        __auto_type temp = (x); \
        rw_ring_buffer_append((lst), &temp); \
    } while (0)

#endif //CORE_ALGO_RING_BUFFER_RW_H
