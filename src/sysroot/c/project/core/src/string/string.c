#include <string.h>
#include "core/algo/algo-list-byte.h"
#include "core/check.h"
#include "core/string/string.h"
#include "core/mem/mem-api.h"

typedef struct byte_list bl;
typedef struct bytes_view bv;
typedef struct mem m;
typedef struct string s;
typedef struct string_view sv;

s *
string_new_from_bytes(m *mem, bv bytes) {
    CHECK_NOT_NULL(mem);
    s *result = mem_allocate(mem, sizeof(s));
    string_init_from_bytes(result, mem, bytes);
    return result;
}

s *
string_new_from_c_str(m *mem, char const *src) {
    CHECK_NOT_NULL(mem);
    CHECK_NOT_NULL(src);
    return string_new_from_bytes(mem, (bv) {
            .data = (u1 *) src,
            .size = strlen(src)
    });
}

s *
string_new_from_byte_list(m *mem, bl *src) {
    CHECK_NOT_NULL(mem);
    CHECK_NOT_NULL(src);
    return string_new_from_bytes(mem, byte_list_raw_bytes(src));
}

s *
string_new_from_view(m *mem, sv view) {
    CHECK_NOT_NULL(mem);
    return string_new_from_bytes(mem, (bv) {
            .data = (u1 *) view.data,
            .size = view.length
    });
}

void
string_init_from_bytes(struct string *self, struct mem *mem, struct bytes_view bytes) {
    CHECK_NOT_NULL(self);
    CHECK_NOT_NULL(mem);
    self->length = bytes.size;
    self->data = mem_allocate(mem, bytes.size + 1);
    memcpy(self->data, bytes.data, bytes.size);
    self->data[bytes.size] = '\0';
}

void
string_init_from_c_str(struct string *self, struct mem *mem, char const *src) {
    CHECK_NOT_NULL(self);
    CHECK_NOT_NULL(mem);
    string_init_from_bytes(self, mem, (bv) {
            .data = (u1 *) src,
            .size = strlen(src)
    });
}

void
string_init_from_byte_list(struct string *self, struct mem *mem, struct byte_list *src) {
    CHECK_NOT_NULL(self);
    CHECK_NOT_NULL(mem);
    CHECK_NOT_NULL(src);
    string_init_from_bytes(self, mem, byte_list_raw_bytes(src));
}

void
string_init_from_view(struct string *self, struct mem *mem, struct string_view view) {
    CHECK_NOT_NULL(self);
    CHECK_NOT_NULL(mem);
    string_init_from_bytes(self, mem, (bv) {
            .data = (u1 *) view.data,
            .size = view.length
    });
}

u4
string_count(s self) {
    return self.length;
}

bool
string_equal(s lhs, s rhs) {
    if (lhs.length != rhs.length) return false;
    return strncmp(lhs.data, rhs.data, lhs.length) == 0;
}

bool
string_equal_c_str(struct string lhs, char const *rhs) {
    if (lhs.length != strlen(rhs)) return false;
    return strncmp(lhs.data, rhs, lhs.length) == 0;
}

s *
string_concat(s self, s other, m *mem) {
    CHECK_NOT_NULL(mem);
    size_t count = self.length + other.length;

    s *result = mem_allocate(mem, sizeof(s));
    result->length = count;
    result->data = mem_allocate(mem, count + 1);
    memcpy(result->data, self.data, self.length);
    memcpy(result->data + self.length, other.data, other.length);
    result->data[count] = '\0';
    return result;
}

void
string_reset(struct string *self, struct mem *mem) {
    CHECK_NOT_NULL(self);
    CHECK_NOT_NULL(mem);
    CHECK_NOT_NULL(self->data);
    mem_deallocate(mem, self->data);
    self->length = 0;
    self->data = NULL;
}

void
string_free(s *self, m *mem) {
    CHECK_NOT_NULL(self);
    CHECK_NOT_NULL(mem);
    mem_deallocate(mem, self->data);
    mem_deallocate(mem, self);
}

void
string_free_safe(struct string **self, struct mem *mem) {
    CHECK_NOT_NULL(self);
    CHECK_NOT_NULL(mem);
    string_free(*self, mem);
    *self = NULL;
}
