#include "core/check.h"
#include "core/algo/algo-list.h"
#include "core/algo/algo-list-ptr.h"

typedef struct ptr_list lst;
typedef struct ptr_list_config c;
typedef struct ptr_list_entry {
    size_t *ptr;
} list_entry;

c
ptr_list_default_config(struct mem *mem) {
    CHECK_NOT_NULL(mem);
    return (c) {
            .initial_capacity = 8,
            .mem = mem,
    };
}

lst *
ptr_list_new(c config) {
    lst *result = mem_allocate(config.mem, sizeof(lst));
    ptr_list_init(result, config);
    return result;
}

void
ptr_list_init(lst *self, c config) {
    CHECK_NOT_NULL(self);
    struct list_config underlying_config = list_default_config(config.mem);
    underlying_config.initial_capacity = config.initial_capacity;
    list_init(&self->underlying_list, underlying_config, list_entry);
}

void
ptr_list_append(lst *self, void *ptr) {
    CHECK_NOT_NULL(self);
    CHECK_NOT_NULL(ptr);
    list_append_rval(&self->underlying_list, (list_entry) {.ptr = ptr});
}

void
ptr_list_replace(struct ptr_list *self, size_t idx, void *ptr) {
    CHECK_NOT_NULL(self);
    CHECK_NOT_NULL(ptr);
    CHECK_LESS_THAN(idx, ptr_list_capacity(self));
    list_replace_rval(&self->underlying_list, idx, (list_entry) {.ptr = ptr});
}

void *
ptr_list_at(lst *self, size_t idx) {
    CHECK_NOT_NULL(self);
    list_entry *entry = (list_entry *) list_at(&self->underlying_list, idx);
    CHECK_NOT_NULL(entry);
    return entry->ptr;
}

bool
ptr_list_iterator_cb_has_next(struct iterator *it) {
    CHECK_NOT_NULL(it);
    return it->current.index < ptr_list_count(it->target);
}

void *
ptr_list_iterator_cb_next(struct iterator *it) {
    CHECK_NOT_NULL(it);
    return ptr_list_at(it->target, it->current.index++);
}

struct iterator
ptr_list_iterator(lst *self) {
    CHECK_NOT_NULL(self);
    return iterator_index(self, ptr_list_iterator_cb_has_next, ptr_list_iterator_cb_next);
}

size_t
ptr_list_count(lst *self) {
    CHECK_NOT_NULL(self);
    return list_count(&self->underlying_list);
}

size_t
ptr_list_capacity(lst *self) {
    CHECK_NOT_NULL(self);
    return list_capacity(&self->underlying_list);
}

void
ptr_list_reset(lst *self) {
    CHECK_NOT_NULL(self);
    list_reset(&self->underlying_list);
}

void
ptr_list_free(lst *self) {
    CHECK_NOT_NULL(self);
    struct mem *mem = self->underlying_list.mem;
    list_reset(&self->underlying_list);
    mem_deallocate(mem, self);
}

void
ptr_list_free_safe(lst **self) {
    CHECK_NOT_NULL(self);
    ptr_list_free(*self);
    *self = NULL;
}
