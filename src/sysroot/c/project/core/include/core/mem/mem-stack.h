#ifndef CORE_MEM_STACK_H
#define CORE_MEM_STACK_H

#include "mem.h"

#define MEM_STACK_MAX_COUNT 256

struct mem_stack_config {
  size_t size;
  struct mem *root;
};

struct mem_stack {
  struct mem base;
  void *current;
  void *end;
  u1 stack_idx;
  size_t capacity;
  void *stack[MEM_STACK_MAX_COUNT];
};

ELODIE_API struct mem_stack *
mem_stack_new (struct mem_stack_config config);

ELODIE_API void
mem_stack_init (struct mem_stack *self, struct mem_stack_config config);

ELODIE_API void
mem_stack_push (struct mem_stack *self);

ELODIE_API void *
mem_stack_allocate (struct mem_stack *self, size_t size);

ELODIE_API void *
mem_stack_resolve (struct mem_stack *self, struct mem_ref);

ELODIE_API void
mem_stack_pop (struct mem_stack *self);

ELODIE_API void
mem_stack_reset (struct mem_stack *self);

ELODIE_API void
mem_stack_free (struct mem_stack *self);

ELODIE_API void
mem_stack_free_safe (struct mem_stack **self);

ELODIE_API size_t
mem_stack_size (struct mem_stack *self);

#endif //CORE_MEM_STACK_H
