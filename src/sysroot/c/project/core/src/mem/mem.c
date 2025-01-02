#include <stdlib.h>

#include "core/mem/mem.h"
#include "core/core-api.h"

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wunused-parameter"

void
mem_new(size_t size) {
    LOG_WARN("replace dummy mem management implementation and mem_allocator_allocate %lu "
             "bytes", size);
}

#pragma GCC diagnostic pop

void mem_free(void) {
    LOG_WARN("replace dummy mem management");
}

void *
mem_allocator_allocate(struct mem_allocator allocator, size_t size) {
    CHECK_NOT_NULL(allocator.fn);
    return allocator.fn(allocator.self, size);
}

struct mem_ref
mem_ref_generator_next(struct mem_ref_generator referencer, u1 kind) {
    CHECK_NOT_NULL(referencer.fn);
    return referencer.fn(referencer.self, kind);
}

void *
mem_resolver_resolve(struct mem_resolver resolver, struct mem_ref ref) {
    CHECK_NOT_NULL(resolver.fn);
    return resolver.fn(resolver.self, ref);
}

void
mem_deallocator_deallocate(struct mem_deallocator deallocator, void *ptr) {
    CHECK_NOT_NULL(deallocator.fn);
    deallocator.fn(deallocator.self, ptr);
}
