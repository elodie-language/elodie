#include "core/check.h"
#include "core/algo/algo-map-ptr.h"

typedef struct ptr_map pm;
typedef struct ptr_map_key pmk;
typedef struct map_key mk;
typedef struct ptr_map_config c;

typedef struct ptr_map_entry {
    size_t *ptr;
} ev;

void
ptr_map_key_init(pmk *self, struct hash8 hash) {
    CHECK_NOT_NULL(self);
    CHECK_GREATER_THAN(hash.value, 0);
    map_key_init(&self->underlying_key, hash);
}

pmk
ptr_map_key_from_bytes(pm *self, struct bytes_view bytes) {
    CHECK_NOT_NULL(self);
    return (pmk) {.underlying_key=map_key_from_bytes(&self->underlying_map, bytes)};
}

pmk
ptr_map_key_from_string_view(pm *self, struct string_view view) {
    CHECK_NOT_NULL(self);
    return (pmk) {.underlying_key = map_key_from_string_view(&self->underlying_map, view)};
}

pmk
ptr_map_key_from_size_t(pm *self, size_t value) {
    CHECK_NOT_NULL(self);
    CHECK_LESS_THAN(value, U8_MAX - 1);
    return (pmk) {.underlying_key = map_key_from_size_t(&self->underlying_map, value)};
}

pmk
ptr_map_key_from_c_str(pm *self, char const *str) {
    CHECK_NOT_NULL(self);
    return (pmk) {.underlying_key = map_key_from_c_str(&self->underlying_map, str)};
}

pm *
ptr_map_new(c config) {
    pm *result = mem_allocate(config.mem, sizeof(pm));
    ptr_map_init(result, config);
    return result;
}

void
ptr_map_init(pm *self, c config) {
    CHECK_NOT_NULL(self);
    struct map_config underlying_config = {
            .mem = config.mem,
            .initial_capacity = config.initial_capacity,
            .key_hash_fn = config.key_hash_fn
    };
    map_init(&self->underlying_map, underlying_config);
}

bool
ptr_map_set(pm *self, pmk key, void *ptr) {
    CHECK_NOT_NULL(self);
    CHECK_NOT_NULL(ptr);
    ev entry = {.ptr = ptr};
    return map_set_bytes_view(&self->underlying_map, key.underlying_key, bytes_view_of_ptr(&entry, sizeof(entry)));
}

bool
ptr_map_get(pm *self, pmk key, void **out) {
    CHECK_NOT_NULL(self);
    struct bytes_view bytes_view;
    if (!map_get_as_bytes_view(&self->underlying_map, key.underlying_key, &bytes_view)) {
        return false;
    }
    ev *entry = bytes_view_as_ptr(bytes_view, sizeof(ev));
    *out = entry->ptr;
    return true;
}

bool
ptr_map_remove(pm *self, pmk key) {
    CHECK_NOT_NULL(self);
    return map_remove(&self->underlying_map, key.underlying_key);
}

bool
ptr_map_has_key(pm *self, pmk key) {
    CHECK_NOT_NULL(self);
    return map_has_key(&self->underlying_map, key.underlying_key);
}

static bool
ptr_map_iterator_cb_has_next(struct iterator *it) {
    CHECK_NOT_NULL(it);
    pm *instance = it->target;
    for (size_t idx = it->current.index; idx < ptr_map_capacity(instance); idx++) {
        if (instance->underlying_map.entries[idx].key.hash.value != 0) {
            return true;
        }
    }
    return false;
}

static void *
ptr_map_iterator_cb_next(struct iterator *it) {
    CHECK_NOT_NULL(it);
    pm *instance = it->target;
    while (it->current.index < ptr_map_capacity(instance)) {

        if (instance->underlying_map.entries[it->current.index].key.hash.value != 0) {
            void *result = &instance->underlying_map.entries[it->current.index].key;
            it->current.index++;
            return result;
        } else {
            it->current.index++;
        }
    }
    ILLEGAL_STATE();
}

struct iterator
ptr_map_keys_iterator(pm *self) {
    CHECK_NOT_NULL(self);
    return iterator_index(self, ptr_map_iterator_cb_has_next, ptr_map_iterator_cb_next);
}

u4
ptr_map_count(pm *self) {
    CHECK_NOT_NULL(self);
    return map_count(&self->underlying_map);
}

u4
ptr_map_capacity(pm *self) {
    CHECK_NOT_NULL(self);
    return map_capacity(&self->underlying_map);
}

void
ptr_map_reset(pm *self) {
    CHECK_NOT_NULL(self);
    map_reset(&self->underlying_map);
}

void
ptr_map_free(pm *self) {
    CHECK_NOT_NULL(self);
    map_free(&self->underlying_map);
}

void
ptr_map_free_safe(pm **self) {
    CHECK_NOT_NULL(self);
    ptr_map_free(*self);
    *self = NULL;
}

