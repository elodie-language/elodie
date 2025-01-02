#include <string.h>
#include "core/check.h"
#include "core/mem/mem-api.h"
#include "core/algo/algo-iterator.h"
#include "core/algo/algo-ring-buffer.h"

typedef struct ring_buffer rb;

rb *
prv_ring_buffer_new(struct mem *mem, size_t capacity, size_t stride) {
    CHECK_NOT_NULL(mem);
    rb *result = mem_allocate(mem, sizeof(rb));
    prv_ring_buffer_init(result, mem, capacity, stride);
    return result;
}

void
prv_ring_buffer_init(rb *self, struct mem *mem, size_t capacity, size_t stride) {
    CHECK_NOT_NULL(self);
    CHECK_NOT_NULL(mem);
    self->position = 0;
    self->capacity = capacity;
    self->stride = stride;
    self->mem = mem;
    self->full = false;
    self->data = mem_allocate(mem, self->capacity * self->stride);
}

void
ring_buffer_append(rb *self, void *val) {
    CHECK_NOT_NULL(self);
    CHECK_NOT_NULL(val);
    memcpy((u1 *) self->data + self->position * self->stride, val, self->stride);
    if (++self->position >= self->capacity) {
        self->position = 0;
        self->full = true;
    }
}

void *
ring_buffer_at(struct ring_buffer *self, size_t idx) {
    CHECK_NOT_NULL(self);
    CHECK_LESS_THAN(idx, self->capacity);
    return (void *) ((u1 *) self->data + idx * self->stride);
}

bool
ring_buffer_iterator_cb_has_next(struct iterator *it) {
    CHECK_NOT_NULL(it);
    if (ring_buffer_is_full(it->target)) {
        return it->current.index < ring_buffer_capacity(it->target);
    } else {
        return it->current.index < ring_buffer_count(it->target);
    }
}

void *
ring_buffer_iterator_cb_next(struct iterator *it) {
    CHECK_NOT_NULL(it);
    return ring_buffer_at(it->target, it->current.index++);
}

struct iterator
ring_buffer_iterator(rb *self) {
    CHECK_NOT_NULL(self);
    return iterator_index(self, ring_buffer_iterator_cb_has_next, ring_buffer_iterator_cb_next);
}

bool
ring_buffer_is_full(rb *self) {
    CHECK_NOT_NULL(self);
    return self->full;
}

size_t
ring_buffer_count(rb *self) {
    CHECK_NOT_NULL(self);
    if (self->full) {
        return self->capacity;
    } else {
        return self->position;
    }
}

size_t
ring_buffer_capacity(rb *self) {
    CHECK_NOT_NULL(self);
    return self->capacity;
}

void
ring_buffer_reset(rb *self) {
    CHECK_NOT_NULL(self);
    if (self->data != NULL) {
        mem_deallocate(self->mem, self->data);
        self->data = NULL;
        self->position = 0;
        self->full = false;
        self->capacity = 0;
        self->stride = 0;
    }
}

void
ring_buffer_free(rb *self) {
    CHECK_NOT_NULL(self);
    if (self->data != NULL) {
        mem_deallocate(self->mem, self->data);
    }
    mem_deallocate(self->mem, self);
}

void
ring_buffer_free_safe(rb **self) {
    CHECK_NOT_NULL(self);
    ring_buffer_free(*self);
    *self = NULL;
}
