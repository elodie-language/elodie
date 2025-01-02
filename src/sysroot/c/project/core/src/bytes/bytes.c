#include <string.h>
#include "core/check.h"
#include "core/bytes/bytes.h"
#include "core/mem/mem-api.h"

typedef struct bytes bs;
typedef struct mem m;

const bs NO_BYTES = {.size = 0, .data = NULL};

struct bytes *
bytes_new(m *mem, u4 size) {
    CHECK_NOT_NULL(mem);
    CHECK_GREATER_THAN(size, 0);
    bs *result = mem_allocate(mem, sizeof(bs));
    bytes_init(result, mem, size);
    return result;
}

void
bytes_init(bs *self, m *mem, u4 size) {
    CHECK_NOT_NULL(self);
    CHECK_NOT_NULL(mem);
    if (size > 0) {
        self->data = mem_allocate(mem, size);
        self->size = size;
        memset(self->data, 0, size);
    } else {
        *self = NO_BYTES;
    }
}

void
bytes_reset(bs *self, m *mem) {
    CHECK_NOT_NULL(self);
    CHECK_NOT_NULL(mem);
    if (self->data != NULL) {
        mem_deallocate(mem, self->data);
    }
    self->size = 0;
    self->data = NULL;
}

void
bytes_free(bs *self, m *mem) {
    CHECK_NOT_NULL(self);
    CHECK_NOT_NULL(mem);
    bytes_reset(self, mem);
    mem_deallocate(mem, self);
}

void
bytes_free_safe(bs **self, m *mem) {
    CHECK_NOT_NULL(self);
    CHECK_NOT_NULL(mem);
    bytes_free(*self, mem);
    *self = NULL;
}
