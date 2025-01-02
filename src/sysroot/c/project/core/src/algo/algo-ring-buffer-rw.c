#include "core/check.h"
#include "core/mem/mem-api.h"
#include "core/algo/algo-ring-buffer-rw.h"

typedef struct rw_ring_buffer rw;

rw *
prv_rw_ring_buffer_new(struct mem *mem, size_t capacity, size_t stride) {
    CHECK_NOT_NULL(mem);
    rw *result = mem_allocate(mem, sizeof(rw));
    prv_rw_ring_buffer_init(result, mem, capacity, stride);
    return result;
}

void
prv_rw_ring_buffer_init(rw *self, struct mem *mem, size_t capacity, size_t stride) {
    CHECK_NOT_NULL(self);
    CHECK_NOT_NULL(mem);
    prv_ring_buffer_init(&self->underlying, mem, capacity, stride);
    self->read_position = 0;
}

void
rw_ring_buffer_append(rw *self, void *val) {
    CHECK_NOT_NULL(self);
    CHECK_NOT_NULL(val);
    ring_buffer_append(&self->underlying, val);
}

void *
rw_ring_buffer_next(rw *self) {
    CHECK_NOT_NULL(self);
    if (ring_buffer_count(&self->underlying) == 0) {
        ABORT("rw_ring_buffer is empty");
    }
    void *data = self->underlying.data;
    size_t stride = self->underlying.stride;

    if (++self->read_position >= rw_ring_buffer_count(self)) {
        self->read_position = 0;
    }

    return (void *) ((u1 *) data + self->read_position * stride);
}

void *
rw_ring_buffer_current(rw *self) {
    CHECK_NOT_NULL(self);
    if (ring_buffer_count(&self->underlying) == 0) {
        return NULL;
    }
    void *data = self->underlying.data;
    size_t stride = self->underlying.stride;
    return (void *) ((u1 *) data + self->read_position * stride);
}

void *
rw_ring_buffer_peek_next(rw *self, size_t offset) {
    CHECK_NOT_NULL(self);
    CHECK_TRUE(rw_ring_buffer_is_full(self));
    CHECK_LESS_THAN(offset, rw_ring_buffer_count(self));
    if (ring_buffer_count(&self->underlying) == 0) {
        return NULL;
    }
    void *data = self->underlying.data;
    size_t stride = self->underlying.stride;
    size_t position = self->read_position;
    size_t count = rw_ring_buffer_count(self);

    if (offset + position >= count) {
        return (void *) ((u1 *) data + (offset + position - count) * stride);
    }

    return (void *) ((u1 *) data + (offset + position) * stride);
}

size_t
rw_ring_buffer_read_position(struct rw_ring_buffer *self) {
    CHECK_NOT_NULL(self);
    return self->read_position;
}

size_t
rw_ring_buffer_write_position(struct rw_ring_buffer *self) {
    CHECK_NOT_NULL(self);
    return self->underlying.position;
}

bool
rw_ring_buffer_is_full(rw *self) {
    CHECK_NOT_NULL(self);
    return ring_buffer_is_full(&self->underlying);
}

size_t
rw_ring_buffer_count(rw *self) {
    CHECK_NOT_NULL(self);
    return ring_buffer_count(&self->underlying);
}

size_t
rw_ring_buffer_capacity(rw *self) {
    CHECK_NOT_NULL(self);
    return ring_buffer_capacity(&self->underlying);
}

void
rw_ring_buffer_reset(rw *self) {
    CHECK_NOT_NULL(self);
    ring_buffer_reset(&self->underlying);
    self->read_position = 0;
}

void
rw_ring_buffer_free(rw *self) {
    CHECK_NOT_NULL(self);
    struct mem *mem = self->underlying.mem;
    ring_buffer_reset(&self->underlying);
    mem_deallocate(mem, self);
}

void
rw_ring_buffer_free_safe(rw **self) {
    CHECK_NOT_NULL(self);
    rw_ring_buffer_free(*self);
    *self = NULL;
}
