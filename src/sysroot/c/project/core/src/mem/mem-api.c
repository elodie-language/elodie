#include "core/check.h"
#include "core/mem/mem-api.h"

typedef struct mem m;
typedef struct bytes_view bv;
typedef struct mem_ref mr;

bool
mem_is_gc(m *self) {
    CHECK_NOT_NULL(self);
    return self->kind == MEM_KIND_GC;
}

bool
mem_is_stack(m *self) {
    CHECK_NOT_NULL(self);
    return self->kind == MEM_KIND_STACK;
}

bool
mem_is_vape(m *self) {
    CHECK_NOT_NULL(self);
    return self->kind == MEM_KIND_VAPE;
}

void *
mem_allocate(m *self, size_t size) {
    CHECK_NOT_NULL(self);
    return mem_allocator_allocate(self->allocator, size);
}

struct mem_ref
mem_next_ref(struct mem *self, u1 kind) {
    CHECK_NOT_NULL(self);
    return mem_ref_generator_next(self->ref_generator, kind);
}

void *
mem_resolve(struct mem *self, struct mem_ref ref) {
    CHECK_NOT_NULL(self);
    return mem_resolver_resolve(self->resolver, ref);
}

void
mem_deallocate(m *self, void *ptr) {
    CHECK_NOT_NULL(self);
    mem_deallocator_deallocate(self->deallocator, ptr);
}

