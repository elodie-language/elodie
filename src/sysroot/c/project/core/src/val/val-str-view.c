#include <string.h>

#include "core/check.h"
#include "core/val/val-str.h"
#include "core/val/val-str-view.h"

struct val_str_view
val_str_view_from_bytes(struct bytes_view bytes) {
    struct val_str_view result = (struct val_str_view) {
            .data = (char const *) bytes.data,
            .count = bytes.size
    };
    val_init(&result.base, VAL_KIND_STR_VIEW, MEM(mem_null_new()));
    return result;
}

struct val_str_view
val_str_view_from_str(struct val_str *ptr) {
    CHECK_NOT_NULL(ptr);
    struct val_str_view result = (struct val_str_view) {
            .data = (char const *) ptr->data,
            .count = ptr->count
    };
    val_init(&result.base, VAL_KIND_STR_VIEW, MEM(mem_null_new()));
    return result;
}

struct val_str_view
val_str_view_from_c_str(char const *str) {
    CHECK_NOT_NULL(str);
    struct val_str_view result = (struct val_str_view) {
            .data = str,
            .count = strlen(str)
    };
    val_init(&result.base, VAL_KIND_STR_VIEW, MEM(mem_null_new()));
    return result;
}

size_t
val_str_view_count(struct val_str_view *self) {
    CHECK_NOT_NULL(self);
    return self->count;
}

bool
val_str_view_equal(struct val_str_view lhs, struct val_str_view rhs) {
    if (lhs.count != rhs.count) return false;
    return strncmp(lhs.data, rhs.data, lhs.count) == 0;
}
