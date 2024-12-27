#ifndef CORE_MEM_MANAGED_H
#define CORE_MEM_MANAGED_H

#include "mem.h"

struct dep_val;
struct list;
struct val_lst;

enum mem_gc_color {
  MEM_GC_COLOR_WHITE = 1,
  MEM_GC_COLOR_BLACK = 2
};

struct mem_gc_config {
  size_t size;
  struct mem *root;
};

struct mem_gc {
  struct mem base;
  struct list *colors;
  struct val_lst *vals;
};

HAMAL_API struct mem_gc *
mem_gc_new (struct mem_gc_config config);

HAMAL_API void
mem_gc_init (struct mem_gc *self, struct mem_gc_config config);

HAMAL_API struct dep_val *
mem_gc_allocate (struct mem_gc *self, struct dep_val *val);

HAMAL_API void
mem_gc_mark_val (struct mem_gc *self, struct dep_val *val, enum mem_gc_color color);

HAMAL_API void
mem_gc_mark (struct mem_gc *self, struct dep_val *val);

HAMAL_API void
mem_gc_sweep (struct mem_gc *self);

HAMAL_API size_t
mem_gc_count (struct mem_gc *self);

HAMAL_API void
mem_gc_free (struct mem_gc *self);

HAMAL_API void
mem_gc_free_safe (struct mem_gc **self);

#endif //CORE_MEM_MANAGED_H
