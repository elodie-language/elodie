#ifndef CORE_IO_H
#define CORE_IO_H

#include "core/algo/algo-list-byte.h"
#include "core/mem/mem.h"
#include "core/val/val-str.h"

struct io {
  struct mem *mem;
};

HAMAL_API struct io *
io_new (struct mem *mem);

HAMAL_API void
io_init (struct io *self, struct mem *mem);

HAMAL_API struct buffer *
io_read_file (struct io *self, struct dep_val_str_view path);

HAMAL_API void
io_reset (struct io *self);

HAMAL_API void
io_free (struct io *self);

#endif //CORE_IO_H
