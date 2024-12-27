#ifndef CORE_MEM_API_H
#define CORE_MEM_API_H

#include "mem.h"
#include "mem-gc.h"
#include "mem-null.h"
#include "mem-raw.h"
#include "mem-stack.h"
#include "mem-test.h"
#include "mem-vape.h"

#include "core/bytes/bytes-view.h"
#include "core/val/val-ref.h"

#define MEM(T) (struct mem*) (T)

#define IS_GC(M) mem_is_gc(M)
#define IS_STACK(M) mem_is_stack(M)
#define IS_VAPE(M) mem_is_vape(M)

HAMAL_API  bool
mem_is_gc (struct mem *self);

HAMAL_API  bool
mem_is_stack (struct mem *self);

HAMAL_API  bool
mem_is_vape (struct mem *self);

HAMAL_API void *
mem_allocate (struct mem *self, size_t);

HAMAL_API struct mem_ref
mem_next_ref (struct mem *self, u1);

HAMAL_API void *
mem_resolve (struct mem *self, struct mem_ref ref);

HAMAL_API void
mem_deallocate (struct mem *self, void *ptr);

#endif //CORE_MEM_API_H
