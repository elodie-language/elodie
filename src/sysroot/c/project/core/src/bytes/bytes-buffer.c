#include <memory.h>
#include "core/core-api.h"

typedef struct buffer bv;

bv *
buffer_new(struct mem *mem, size_t capacity) {
    bv *result = mem_allocate(mem, sizeof(bv));
    buffer_init(result, mem, capacity);
    return result;
}

void
buffer_init(bv *self, struct mem *mem, size_t capacity) {
    CHECK_NOT_NULL(self);
    self->capacity = capacity;
    self->position = 0;
    self->limit = capacity;
    self->mem = mem;
    self->data = mem_allocate(mem, capacity);
}

ELODIE_API void
buffer_flip(bv *self) {
    CHECK_NOT_NULL(self);
    self->limit = self->position;
    self->position = 0;
}

ELODIE_API void
buffer_compact(bv *self) {
    CHECK_NOT_NULL(self);
    size_t num_of_bytes = self->limit - self->position;

    memmove(self->data, &(self->data[self->position]), num_of_bytes);
    self->limit = self->capacity;
    self->position = num_of_bytes;
}

void
buffer_clear(bv *self) {
    CHECK_NOT_NULL(self);
    self->position = 0;
    self->limit = self->capacity;
}

u4
buffer_position(bv *self) {
    CHECK_NOT_NULL(self);
    return self->position;
}

u4
buffer_limit(bv *self) {
    CHECK_NOT_NULL(self);
    return self->limit;
}

u4
buffer_capacity(bv *self) {
    CHECK_NOT_NULL(self);
    return self->capacity;
}

u4
buffer_available(bv *self) {
    CHECK_NOT_NULL(self);
    return self->limit - self->position;
}

void
buffer_write_u1(bv *self, u1 data) {
    CHECK_NOT_NULL(self);
    CHECK_LESS_THAN_EQUAL(self->position + 1, self->limit);
    self->data[self->position++] = data;
}

void
buffer_write_bytes(bv *self, struct bytes_view bytes) {
    CHECK_NOT_NULL(self);
    CHECK_LESS_THAN_EQUAL(self->position + bytes.size, self->limit);
    memcpy(self->data + self->position, bytes.data, bytes.size);
    self->position += bytes.size;
}

u1
buffer_read_u1(bv *self) {
    CHECK_NOT_NULL(self);
    CHECK_LESS_THAN_EQUAL(self->position + 1, self->limit);
    return self->data[self->position++];
}

struct bytes_view
buffer_read_bytes(bv *self, size_t size) {
    CHECK_NOT_NULL(self);
    CHECK_LESS_THAN_EQUAL(self->position + size, self->limit);
    struct bytes_view result = (struct bytes_view) {
            .data = self->data + self->position,
            .size = size
    };
    self->position += size;
    return result;
}

struct bytes_view
buffer_read_all_bytes(bv *self) {
    CHECK_NOT_NULL(self);
    struct bytes_view result = (struct bytes_view) {
            .data = self->data,
            .size = self->limit
    };
    self->position = self->limit;
    return result;
}

void
buffer_reset(bv *self) {
    CHECK_NOT_NULL(self);
    struct mem_deallocator deallocator = self->mem->deallocator;
    mem_deallocator_deallocate(deallocator, self->data);

    self->capacity = 0;
    self->position = 0;
    self->limit = 0;
    self->mem = MEM(mem_null_new());
    self->data = NULL;
}

void
buffer_free(bv *self) {
    CHECK_NOT_NULL(self);
    struct mem_deallocator deallocator = self->mem->deallocator;
    mem_deallocator_deallocate(deallocator, self->data);
    mem_deallocator_deallocate(deallocator, self);
}

void
buffer_free_safe(bv **self) {
    CHECK_NOT_NULL(self);
    buffer_free(*self);
    *self = NULL;
}
