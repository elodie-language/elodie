#include <stdlib.h>

#include "core/check.h"
#include "core/mem/mem-api.h"

static void *
allocate(void *ctx, size_t size) {
    CHECK_NOT_NULL(ctx);
    struct mem_stack *impl = (struct mem_stack *) ctx;
    LOG_TRACE("%p allocates %lu bytes", impl, size);
    return mem_stack_allocate(impl, size);
}

static void
deallocate(void *ctx, void *) {
}

static struct mem_ref
next_reference(void *ctx, u1 kind) {
    CHECK_NOT_NULL(ctx);
    struct mem_stack *impl = (struct mem_stack *) ctx;
    u4 address = (u1 *) impl->current - (u1 *) impl->stack[0];
    return (struct mem_ref) {
            .kind = kind,
            .realm = impl->base.realm,
            .value = address
    };
}

struct mem_stack *
mem_stack_new(struct mem_stack_config config) {
    struct mem_allocator allocator = config.root->allocator;
    struct mem_stack *result = mem_allocator_allocate(allocator, sizeof(struct mem_stack));
    mem_stack_init(result, config);
    return result;
}

static void *
resolve(void *self, struct mem_ref ref) {
    CHECK_NOT_NULL(self);
    struct mem_stack *impl = (struct mem_stack *) self;
    return mem_stack_resolve(impl, ref);
}

void
mem_stack_init(struct mem_stack *self, struct mem_stack_config config) {
    CHECK_NOT_NULL(self);
    CHECK_NOT_NULL(config.root->allocator.fn);
    CHECK_NOT_NULL(config.root->deallocator.fn);
    CHECK_GREATER_THAN(config.size, 0);

    self->base = (struct mem) {
            .kind = MEM_KIND_STACK,
            .allocator = (struct mem_allocator) {
                    .self = self,
                    .fn = allocate
            },
            .deallocator = (struct mem_deallocator) {
                    .self = self,
                    .fn = deallocate
            },
            .ref_generator = (struct mem_ref_generator) {
                    .self = self,
                    .fn = next_reference
            },
            .resolver = (struct mem_resolver) {
                    .self = self,
                    .fn = resolve
            },
            .root = (struct mem *) config.root
    };
    self->stack_idx = 0;
    self->current = mem_allocator_allocate(self->base.root->allocator, config.size);
    CHECK_NOT_NULL(self->current);
    self->stack[0] = self->current;

    self->end = (u1 *) self->current + config.size;
    self->capacity = config.size;
}

void
mem_stack_push(struct mem_stack *self) {
    CHECK_NOT_NULL(self);
    CHECK_NOT_NULL(self);
    CHECK_LESS_THAN(self->stack_idx + 1, MEM_STACK_MAX_COUNT);
    self->stack[self->stack_idx] = self->current;
    self->stack_idx++;
    LOG_TRACE("pushed to %d", self->stack_idx);
}

void *
mem_stack_allocate(struct mem_stack *self, size_t size) {
    CHECK_NOT_NULL(self);
    CHECK_LESS_THAN_EQUAL((u1 *) self->current + size, (u1 *) self->end);

    void *result = (u1 *) self->current;
    LOG_TRACE("allocated %lu bytes - %p", size, self->current);

    self->current = (u1 *) result + size;
    return result;
}

void *
mem_stack_resolve(struct mem_stack *self, struct mem_ref ref) {
    CHECK_NOT_NULL(self);
    CHECK_LESS_THAN((u1 *) self->stack[0] + ref.value, (u1 *) self->end);
    return (void *) ((u1 *) self->stack[0] + ref.value);
}

void
mem_stack_pop(struct mem_stack *self) {
    CHECK_NOT_NULL(self);
    CHECK_GREATER_THAN(self->stack_idx, 0);
    self->stack_idx--;
    self->current = self->stack[self->stack_idx];
    LOG_TRACE("popped to %d", self->stack_idx);
}

void
mem_stack_reset(struct mem_stack *self) {
    struct mem_deallocator root_deallocator = self->base.root->deallocator;
    CHECK_NOT_NULL(root_deallocator.fn);

    mem_deallocator_deallocate(root_deallocator, self->stack[0]);

    self->current = self->stack[0];
    self->stack_idx = 0;
}

size_t
mem_stack_size(struct mem_stack *self) {
    CHECK_NOT_NULL(self);
    CHECK_NOT_NULL(self->current);
    CHECK_NOT_NULL(self->end);
    return self->capacity - ((u1 *) self->end - (u1 *) self->current);
}

void
mem_stack_free(struct mem_stack *self) {
    CHECK_NOT_NULL(self);
    struct mem_deallocator root_deallocator = self->base.root->deallocator;
    CHECK_NOT_NULL(root_deallocator.fn);

    if (self->stack[0] != NULL) {
        mem_deallocator_deallocate(root_deallocator, self->stack[0]);
    }
    mem_deallocator_deallocate(root_deallocator, self);
}

void
mem_stack_free_safe(struct mem_stack **self) {
    CHECK_NOT_NULL(self);
    mem_stack_free(*self);
    *self = NULL;
}
