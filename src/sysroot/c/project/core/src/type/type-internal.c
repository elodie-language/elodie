#include "core/check.h"
#include "core/type/type.h"
#include "core/val/val-str.h"

#include "type-impl.h"

typedef struct type_node n;
typedef struct type_edge entry;

n *
type_node_new (struct mem *mem, u2 id, u2 base_id, struct val_str_view ident)
{
	CHECK_NOT_NULL(mem);
	n *result = mem_allocate (mem, sizeof (n));
	result->id = id;
	result->base_id = base_id;
	result->ident = val_str_allocate_from_view (mem, ident);
	return result;
}

void
type_node_free (n *self, struct mem *mem)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(mem);
	val_str_deallocate (self->ident);
	mem_deallocate (mem, self);
}

entry *
type_edge_new (struct mem *mem, u2 type_id, u2 base_id, entry *prev)
{
	CHECK_NOT_NULL(mem);
	entry *result = mem_allocate (mem, sizeof (entry));
	result->base_id = base_id;
	result->type_id = type_id;
	result->prev = prev;
	return result;
}

void
type_edge_free (entry *self, struct mem *mem)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(mem);
	mem_deallocate (mem, self);
}
