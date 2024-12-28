#ifndef CORE_BYTES_VIEW_H
#define CORE_BYTES_VIEW_H

#include "core/core.h"
#include "core/bytes/bytes.h"

struct string_view;

struct bytes_view {
    u1 *data;
    u4 size;
};

ELODIE_API struct bytes_view
bytes_view_of_c_str(char const *c_str);

ELODIE_API struct bytes_view
bytes_view_of_u2(u2 const *data);

ELODIE_API u2
bytes_view_as_u2(struct bytes_view self);

ELODIE_API struct bytes_view
bytes_view_of_u4(u4 const *data);

ELODIE_API u4
bytes_view_as_u4(struct bytes_view self);

ELODIE_API struct bytes_view
bytes_view_of_u8(u8 const *data);

ELODIE_API u8
bytes_view_as_u8(struct bytes_view self);

ELODIE_API struct bytes_view
bytes_view_of_bytes(struct bytes bytes);

ELODIE_API struct bytes_view
bytes_view_of_ptr(void *ptr, u4 size);

ELODIE_API void *
bytes_view_as_ptr(struct bytes_view self, u4 size);

#endif //CORE_BYTES_VIEW_H
