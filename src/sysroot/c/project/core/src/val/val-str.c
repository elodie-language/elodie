#include <string.h>

#include "core/check.h"
#include "core/algo/algo-list-byte.h"
#include "core/val/val-str.h"
#include "core/mem/mem-stack.h"

typedef struct mem m;
typedef struct byte_list bl;
typedef struct bytes_view bv;
typedef struct mem_ref mr;
typedef struct val_ref vr;
typedef struct val_str_view sv;
typedef struct mem m;
typedef struct byte_list bl;
typedef struct bytes_view bv;
typedef struct val_str s;
typedef struct val_str_view sv;

struct val_str *
val_str_new_from_c_str(struct mem *mem, char const *src) {
    CHECK_NOT_NULL(mem);
    CHECK_NOT_NULL(src);

    size_t count = strlen(src);
    struct val_str *result = mem_allocate(mem, sizeof(struct val_str));
    val_init(&result->base, VAL_KIND_STR, mem);

    result->count = count;
    result->data = mem_allocate(mem, count + 1);
    memcpy(result->data, src, count);
    result->data[count] = '\0';
    return result;
}

struct val_str *
val_str_new_from_bytes(struct mem *mem, struct bytes_view bytes) {
    CHECK_NOT_NULL(mem);

    size_t count = bytes.size;
    struct val_str *result = mem_allocate(mem, sizeof(struct val_str));
    val_init(&result->base, VAL_KIND_STR, mem);

    result->count = count;
    result->data = mem_allocate(mem, count + 1);
    memcpy(result->data, bytes.data, count);
    result->data[count] = '\0';
    return result;
}

struct val_str *
val_str_new_from_byte_list(struct mem *mem, struct byte_list *src) {
    CHECK_NOT_NULL(mem);
    CHECK_NOT_NULL(src);
    size_t count = byte_list_size(src);
    struct val_str *result = mem_allocate(mem, sizeof(struct val_str));
    val_init(&result->base, VAL_KIND_STR, mem);
    result->count = count;
    result->data = mem_allocate(mem, count + 1);
    memcpy(result->data, (u1 *) src->underlying_list.data, result->count);
    result->data[count] = '\0';
    return result;
}

struct val_str *
val_str_new_from_view(struct mem *mem, struct val_str_view view) {
    CHECK_NOT_NULL(mem);

    size_t count = view.count;
    struct val_str *result = mem_allocate(mem, sizeof(struct val_str));
    val_init(&result->base, VAL_KIND_STR, mem);
    result->count = count;
    result->data = mem_allocate(mem, count + 1);
    memcpy(result->data, view.data, count);
    result->data[count] = '\0';
    return result;
}

struct val_str *
val_str_copy(struct val_str *src, struct mem *mem) {
    CHECK_NOT_NULL(mem);
    CHECK_NOT_NULL(src);

    struct val_str *result = mem_allocate(mem, sizeof(struct val_str));
    val_init(&result->base, VAL_KIND_STR, mem);
    result->count = src->count;
    result->data = mem_allocate(mem, result->count);
    memcpy(result->data, src->data, result->count);
    return result;
}

size_t
val_str_count(struct val_str *self) {
    CHECK_NOT_NULL(self);
    return self->count;
}

bool
val_str_equal(struct val_str *lhs, struct val_str *rhs) {
    CHECK_NOT_NULL(lhs);
    CHECK_NOT_NULL(rhs);
    if (lhs == rhs) return true;
    if (lhs->count != rhs->count) return false;
    return strncmp(lhs->data, rhs->data, lhs->count) == 0;
}

struct val_str *
val_str_concat(struct val_str *self, struct val_str *other, struct mem *mem) {
    CHECK_NOT_NULL(mem);
    CHECK_NOT_NULL(self);
    CHECK_NOT_NULL(other);
    size_t count = self->count + other->count;

    struct val_str *result = mem_allocate(mem, sizeof(struct val_str));
    val_init(&result->base, VAL_KIND_STR, mem);

    result->count = count;
    result->data = mem_allocate(mem, count + 1);
    memcpy(result->data, self->data, self->count);
    memcpy(result->data + self->count, other->data, other->count);
    result->data[count] = '\0';

    return result;
}

void
val_str_free(struct val_str *self) {
    CHECK_NOT_NULL(self);
    mem_deallocate(self->base.mem, self->data);
    mem_deallocate(self->base.mem, self);
}

void
val_str_free_safe(struct val_str **self) {
    CHECK_NOT_NULL(self);
    val_str_free(*self);
    *self = NULL;
}
