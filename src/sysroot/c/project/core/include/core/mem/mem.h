#ifndef CORE_MEM_H
#define CORE_MEM_H

#include "core/core.h"
#include "core/macro.h"

struct mem_ref {
  u1 kind;
  u1 realm;
  u4 value;
};

enum mem_kind {
  MEM_KIND_GC,
  MEM_KIND_NULL,
  MEM_KIND_RAW,
  MEM_KIND_STACK,
  MEM_KIND_TEST,
  MEM_KIND_VAL,
  MEM_KIND_VAPE
};

ELODIE_API void
mem_new (size_t size);

ELODIE_API void
mem_free (void);

struct mem_allocator {
  void *self;
  void *(*fn) (void *, size_t);
};

struct mem_ref_generator {
  void *self;
  struct mem_ref (*fn) (void *, u1);
};

struct mem_resolver {
  void *self;
  void *(*fn) (void *, struct mem_ref);
};

struct mem_deallocator {
  void *self;
  void (*fn) (void *, void *);
};

struct mem {
  enum mem_kind kind;
  struct mem_allocator allocator;
  struct mem_deallocator deallocator;
  struct mem_ref_generator ref_generator;
  struct mem_resolver resolver;
  struct mem *root;
  u1 realm;
};

ELODIE_API void *
mem_allocator_allocate (struct mem_allocator, size_t);

ELODIE_API struct mem_ref
mem_ref_generator_next (struct mem_ref_generator referencer, u1 kind);

ELODIE_API void *
mem_resolver_resolve (struct mem_resolver, struct mem_ref);

ELODIE_API void
mem_deallocator_deallocate (struct mem_deallocator, void *);

#endif //CORE_MEM_H
