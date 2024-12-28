#ifndef CORE_MEM_VAPE_H
#define CORE_MEM_VAPE_H

#include "mem.h"

struct mem_vape_config {
  size_t size;
  struct mem *root;
};

struct mem_vape {
  struct mem base;
  void *start;    // points to the start of the mem
  void *current;  // points to the current position in mem
  void *end;      // points to the end of the mem
  size_t capacity;
};

ELODIE_API struct mem_vape *
mem_vape_new (struct mem_vape_config config);

ELODIE_API void
mem_vape_init (struct mem_vape *self, struct mem_vape_config config);

ELODIE_API void *
mem_vape_allocate (struct mem_vape *self, size_t size);

ELODIE_API void *
mem_vape_resolve (struct mem_vape *self, struct mem_ref);

ELODIE_API size_t
mem_vape_size (struct mem_vape *self);

ELODIE_API void
mem_vape_reset (struct mem_vape *self);

ELODIE_API void
mem_vape_free (struct mem_vape *self);

ELODIE_API void
mem_vape_free_safe (struct mem_vape **self);

#endif // CORE_MEM_VAPE_H
