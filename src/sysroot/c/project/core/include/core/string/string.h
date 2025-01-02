#ifndef CORE_STRING_H
#define CORE_STRING_H

#include "core/core.h"
#include "core/bytes/bytes-view.h"
#include "core/string/string-view.h"

struct byte_list;

struct string {
  u4 length;
  char *data;
};

ELODIE_API struct string *
string_allocate_from_bytes (struct mem *mem, struct bytes_view bytes);

ELODIE_API struct string *
string_allocate_from_c_str (struct mem *mem, char const *src);

ELODIE_API struct string *
string_allocate_from_byte_list (struct mem *mem, struct byte_list *src);

ELODIE_API struct string *
string_allocate_from_view (struct mem *mem, struct string_view view);

ELODIE_API void
string_init_from_bytes (struct string *self, struct mem *mem, struct bytes_view bytes);

ELODIE_API void
string_init_from_c_str (struct string *self, struct mem *mem, char const *src);

ELODIE_API void
string_init_from_byte_list (struct string *self, struct mem *mem, struct byte_list *src);

ELODIE_API void
string_init_from_view (struct string *self, struct mem *mem, struct string_view view);

ELODIE_API u4
string_count (struct string self);

ELODIE_API bool
string_equal (struct string lhs, struct string rhs);

ELODIE_API bool
string_equal_c_str (struct string lhs, char const *rhs);

ELODIE_API struct string *
string_concat (struct string self, struct string other, struct mem *mem);

ELODIE_API void
string_reset (struct string *self, struct mem *mem);

ELODIE_API void
string_deallocate (struct string *self, struct mem *mem);

ELODIE_API void
string_deallocate_safe (struct string **self, struct mem *mem);

#endif //CORE_STRING_H
