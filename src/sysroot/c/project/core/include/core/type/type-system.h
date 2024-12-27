#ifndef CORE_TYPE_SYSTEM_H
#define CORE_TYPE_SYSTEM_H

#include "core/macro.h"
#include "core/algo/algo-list-ptr.h"
#include "core/type/type.h"

struct dep_val_str;
struct ptr_list;

struct type_system {
  struct mem *mem;
  struct ptr_list nodes;
  struct ptr_list edges;
};

HAMAL_API struct type_system *
type_system_new (struct mem *mem);

HAMAL_API struct type
type_system_find_by_ident (struct type_system *self, struct dep_val_str_view ident);

HAMAL_API struct type
type_system_compose (struct type_system *self, struct type base_type, struct type);

HAMAL_API struct type_info
type_system_get_info (struct type_system *self, struct type type);

HAMAL_API bool
type_system_is_base_type (struct type_system *self, struct type type);

HAMAL_API void
type_system_free (struct type_system *self);

HAMAL_API void
type_system_free_safe (struct type_system **self);

#endif //CORE_TYPE_SYSTEM_H
