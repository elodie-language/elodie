#ifndef CORE_BYTES_BUFFER_H
#define CORE_BYTES_BUFFER_H

#include "core/core.h"
#include "core/macro.h"
#include "bytes-view.h"

struct buffer {
  u4 capacity;
  u4 position;
  u4 limit;
  struct mem *mem;
  u1 *data;
};

HAMAL_API struct buffer *
buffer_new (struct mem *mem, size_t capacity);

HAMAL_API void
buffer_init (struct buffer *self, struct mem *mem, size_t capacity);

HAMAL_API void
buffer_flip (struct buffer *self);

HAMAL_API void
buffer_compact (struct buffer *self);

HAMAL_API void
buffer_clear (struct buffer *self);

HAMAL_API u4
buffer_position (struct buffer *self);

HAMAL_API u4
buffer_limit (struct buffer *self);

HAMAL_API u4
buffer_capacity (struct buffer *self);

HAMAL_API u4
buffer_available (struct buffer *self);

HAMAL_API void
buffer_write_u1 (struct buffer *self, u1 data);

HAMAL_API void
buffer_write_bytes (struct buffer *self, struct bytes_view bytes);

HAMAL_API u1
buffer_read_u1 (struct buffer *self);

HAMAL_API struct bytes_view
buffer_read_bytes (struct buffer *self, size_t size);

HAMAL_API struct bytes_view
buffer_read_all_bytes (struct buffer *self);

HAMAL_API void
buffer_reset (struct buffer *self);

HAMAL_API void
buffer_free (struct buffer *self);

HAMAL_API void
buffer_free_safe (struct buffer **self);

#endif //CORE_BYTES_BUFFER_H
