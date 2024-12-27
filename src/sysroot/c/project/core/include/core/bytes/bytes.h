#ifndef CORE_BYTES_H
#define CORE_BYTES_H

#include "core/macro.h"
#include "core/core.h"

struct mem;

struct bytes {
  u1 *data;
  u4 size;
};

extern const struct bytes NO_BYTES;

HAMAL_API struct bytes *
bytes_allocate (struct mem *mem, u4 size);

HAMAL_API void
bytes_init (struct bytes *self, struct mem *mem, u4 size);

HAMAL_API void
bytes_reset (struct bytes *self, struct mem *mem);

HAMAL_API void
bytes_deallocate (struct bytes *self, struct mem *mem);

HAMAL_API void
bytes_deallocate_safe (struct bytes **self, struct mem *mem);

#endif //CORE_BYTES_H
