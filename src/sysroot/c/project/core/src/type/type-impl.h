#ifndef CORE_TYPE_IMPL_H
#define CORE_TYPE_IMPL_H

#include "core/macro.h"
#include "core/core.h"

struct type_node {
  u2 id;
  u2 base_id;
  struct val_str *ident;
};

ELODIE_API struct type_node *
type_node_new (struct mem *mem, u2 id, u2 base_id, struct val_str_view ident);

ELODIE_API void
type_node_free (struct type_node *self, struct mem *mem);

struct type_edge {
  u2 base_id;
  u2 type_id;
  struct type_edge *prev;
};

ELODIE_API struct type_edge *
type_edge_new (struct mem *mem, u2 type_id, u2 base_id, struct type_edge *prev);

ELODIE_API void
type_edge_free (struct type_edge *self, struct mem *mem);

#endif //CORE_TYPE_IMPL_H
