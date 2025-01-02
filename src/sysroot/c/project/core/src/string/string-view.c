#include <string.h>
#include "core/check.h"
#include "core/string/string.h"
#include "core/string/string-view.h"

typedef struct bytes_view bv;
typedef struct string s;
typedef struct string_view sv;

sv
string_view_from_bytes(bv bytes) {
    return (sv) {
            .data = (char const *) bytes.data,
            .length = bytes.size
    };
}

sv
string_view_from_str(struct string str) {
    return (sv) {
            .data = (char const *) str.data,
            .length = str.length
    };
}

sv
string_view_from_str_ptr(s *ptr) {
    CHECK_NOT_NULL(ptr);
    return (sv) {
            .data = (char const *) ptr->data,
            .length = ptr->length
    };
}

sv
string_view_from_c_str(char const *str) {
    CHECK_NOT_NULL(str);
    return (sv) {
            .data = str,
            .length = strlen(str)
    };
}

bv
string_view_as_byte_view(sv self) {
    return (bv) {.data = (u1 *) self.data, .size = self.length};
}

u4
string_view_count(sv self) {
    return self.length;
}

bool
string_view_equal(sv lhs, sv rhs) {
    if (lhs.length != rhs.length) return false;
    return strncmp(lhs.data, rhs.data, lhs.length) == 0;
}

bool
string_view_equal_c_str(struct string_view lhs, char const *rhs) {
    return string_view_equal(lhs, string_view_from_c_str(rhs));
}

bool
string_view_last_occurrence_of(struct string_view self, struct string_view pattern, u4 *position) {
    CHECK_GREATER_THAN(self.length, 0);
    CHECK_GREATER_THAN(pattern.length, 0);

    const char *self_to_test = self.data + self.length;
    char const *pattern_to_test = pattern.data + pattern.length;

    while (self_to_test - self.data >= pattern_to_test - pattern.data) {
        char const *cur_self = self_to_test;
        char const *cur_pattern = pattern_to_test;

        while (cur_pattern != pattern.data && *(cur_self - 1) == *(cur_pattern - 1)) {
            --cur_self;
            --cur_pattern;
        }

        if (cur_pattern == pattern.data) {
            self_to_test = cur_self;
            *position = self_to_test - self.data;
            return true;
        }
        --self_to_test;
    }

    return false;
}
