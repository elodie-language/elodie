#ifndef CORE_MEM_TEST_H
#define CORE_MEM_TEST_H

#include "mem.h"

/**
 * mem intended to be used or tests only - it does not free any mem.
 * The sole purpose of this mem is to make sure that all mem gets freed
 */

struct mem_test_config {
  size_t size;
  struct mem *root;
};

struct mem_test_allocated_mem {
  u4 id;
  void *start;
  size_t size;
  struct mem_test_allocated_mem *next;
  struct mem_test_allocated_mem *prev;
};

struct mem_test {
  struct mem base;
  void *start;
  void *end;
  struct mem_test_allocated_mem *head;
  struct mem_test_allocated_mem *tail;
  u4 next_id;
};

HAMAL_API struct mem_test *
mem_test_new (struct mem_test_config config);

HAMAL_API void
mem_test_init (struct mem_test *self, struct mem_test_config config);

HAMAL_API struct mem_test *
mem_test_new_default (size_t size);

HAMAL_API void *
mem_test_allocate (struct mem_test *self, size_t size);

HAMAL_API void *
mem_test_resolve (struct mem_test *self, struct mem_ref);

HAMAL_API void
mem_test_deallocate (struct mem_test *self, void *ptr);

HAMAL_API void
mem_test_verify (struct mem_test *self);

HAMAL_API void
mem_test_reset (struct mem_test *self);

HAMAL_API void
mem_test_free (struct mem_test *self);

HAMAL_API size_t
mem_test_size (struct mem_test *self);

#endif //CORE_MEM_TEST_H
