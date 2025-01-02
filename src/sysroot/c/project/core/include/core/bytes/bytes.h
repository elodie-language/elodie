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

ELODIE_API struct bytes *
bytes_new (struct mem *mem, u4 size);

ELODIE_API void
bytes_init (struct bytes *self, struct mem *mem, u4 size);

ELODIE_API void
bytes_reset (struct bytes *self, struct mem *mem);

ELODIE_API void
bytes_free (struct bytes *self, struct mem *mem);

ELODIE_API void
bytes_free_safe (struct bytes **self, struct mem *mem);

#endif //CORE_BYTES_H
