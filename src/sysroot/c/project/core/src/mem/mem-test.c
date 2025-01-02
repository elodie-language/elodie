#include <stdlib.h>

#include "core/check.h"
#include "core/mem/mem-api.h"

static void *
allocate(void *ctx, size_t size) {
    CHECK_NOT_NULL(ctx);
    struct mem_test *self = (struct mem_test *) ctx;
    return mem_test_allocate(self, size);
}

static struct mem_ref
next_reference(void *ctx, u1 kind) {
    CHECK_NOT_NULL(ctx);
    struct mem_test *impl = (struct mem_test *) ctx;
    return (struct mem_ref) {
            .kind = kind,
            .realm = impl->base.realm,
            .value = impl->next_id
    };
}

static void
deallocate(void *ctx, void *ptr) {
    CHECK_NOT_NULL(ctx);
    struct mem_test *self = (struct mem_test *) ctx;
    mem_test_deallocate(self, ptr);
}

static void *
resolve(void *self, struct mem_ref ref) {
    CHECK_NOT_NULL(self);
    struct mem_test *impl = (struct mem_test *) self;
    return mem_test_resolve(impl, ref);
}

void
mem_test_reset(struct mem_test *self) {
    CHECK_NOT_NULL(self);
    self->start = NULL;
    self->end = NULL;
    for (struct mem_test_allocated_mem *current = self->tail; current != NULL; current = current->prev) {
        mem_deallocator_deallocate(self->base.root->deallocator, current);
    }
    LOG_DEBUG("mem_test %p reset", self);
}

struct mem_test *
mem_test_new(struct mem_test_config config) {
    struct mem_allocator allocator = config.root->allocator;
    struct mem_test *result = mem_allocator_allocate(allocator, sizeof(struct mem_test));
    mem_test_init(result, config);
    result->head = NULL;
    result->tail = NULL;
    return result;
}

struct mem_test *
mem_test_new_default(size_t size) {
    struct mem_test_config tm_config = {
            .size = size,
            .root = MEM (mem_raw_new())
    };
    return mem_test_new(tm_config);
}

void
mem_test_init(struct mem_test *self, struct mem_test_config config) {
    CHECK_NOT_NULL(self);
    CHECK_NOT_NULL(config.root->allocator.fn);
    CHECK_NOT_NULL(config.root->deallocator.fn);
    CHECK_GREATER_THAN(config.size, 0);

    self->base = (struct mem) {
            .kind = MEM_KIND_TEST,
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
            .root = config.root
    };

    self->start = mem_allocate(self->base.root, config.size);
    CHECK_NOT_NULL(self->start);

    self->end = (u1 *) self->start + config.size;
    self->next_id = 0;
}

void *
mem_test_allocate(struct mem_test *self, size_t size) {
    CHECK_NOT_NULL(self);
    CHECK_LESS_THAN_EQUAL((u1 *) self->start + size, (u1 *) self->end);

    struct mem_test_allocated_mem *alloc_mem = mem_allocator_allocate(self->base.root->allocator,
                                                                      sizeof(struct mem_test_allocated_mem));
    alloc_mem->start = self->start;
    alloc_mem->size = size;
    alloc_mem->prev = NULL;
    alloc_mem->next = NULL;
    alloc_mem->id = self->next_id++;

    if (self->tail == NULL) {
        self->head = alloc_mem;
        self->tail = alloc_mem;
    } else {
        alloc_mem->prev = self->tail;
        self->tail->next = alloc_mem;

        self->tail = alloc_mem;
    }

    u1 *result = self->start;
    LOG_TRACE("allocated %lu bytes - %p", size, result);

    self->start = result + size;
    return result;
}

void *
mem_test_resolve(struct mem_test *self, struct mem_ref ref) {
    for (struct mem_test_allocated_mem *current = self->head; current != NULL; current = current->next) {
        if (current->id == ref.value) {
            return current->start;
        }
    }
    ABORT("reference %d does not exist anymore", ref.value);
}

void
mem_test_deallocate(struct mem_test *self, void *ptr) {
    CHECK_NOT_NULL(self);
    CHECK_NOT_NULL(ptr);

    for (struct mem_test_allocated_mem *current = self->head; current != NULL; current = current->next) {
        if (current->start == ptr) {
            if (self->head == current) {
                self->head = current->next;
            }
            if (self->tail == current) {
                self->tail = current->prev;
            }

            if (current->next != NULL) {
                current->next->prev = current->prev;
            }

            if (current->prev != NULL) {
                current->prev->next = current->next;
            }

            LOG_TRACE("deallocated %p", current);
            mem_deallocator_deallocate(self->base.root->deallocator, current);
            return;
        }
    }

    ABORT("%p was not allocated by this mem or already freed", ptr);
}

void
mem_test_verify(struct mem_test *self) {
    CHECK_NOT_NULL(self);
    size_t failed_to_free_counter = 0;
    size_t leaked_bytes = 0;
    for (struct mem_test_allocated_mem *current = self->head; current != NULL; current = current->next) {
        LOG_FATAL("%p was not freed - %d bytes", current->start, current->size);
        failed_to_free_counter++;
        leaked_bytes += current->size;
    }

    if (failed_to_free_counter > 0) {
        ABORT("%d pointer were not freed again - leaked %d bytes", failed_to_free_counter, leaked_bytes);
    }
}

ELODIE_API size_t
mem_test_size(struct mem_test *self) {
    CHECK_NOT_NULL(self);

    size_t result = 0;
    for (struct mem_test_allocated_mem *current = self->head; current != NULL; current = current->next) {
        result += current->size;
    }
    return result;
}

void
mem_test_free(struct mem_test *self) {
    CHECK_NOT_NULL(self);
    struct mem_deallocator root_deallocator = self->base.root->deallocator;
    mem_test_reset(self);
    mem_deallocator_deallocate(root_deallocator, self);
}
