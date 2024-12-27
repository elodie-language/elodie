#include "core/check.h"
#include "core/type/type.h"
#include "core/val/val-api.h"

#include "core/type/type-system.h"
#include "type-impl.h"

typedef struct type tok;
typedef struct type_info ti;
typedef struct type_system ts;
typedef struct type_edge entry;
typedef struct type_node n;

static u2
next_type_id (ts *self)
{
	CHECK_NOT_NULL(self);
	return ptr_list_count (&self->edges);
}

static tok
add_any_type (ts *self)
{
	ptr_list_append (&self->nodes, type_node_new (self->mem, type_any.id, type_any.id, DEP_VAL_STR_VIEW("any")));
	ptr_list_append (&self->edges, type_edge_new (self->mem, type_any.id, type_any.id, NULL));
	return type_any;
}

static tok
add_base_type (ts *self, u2 base_id, struct dep_val_str_view ident)
{
	CHECK_NOT_NULL(self);
	u2 new_type_id = next_type_id (self);

	n *new_node = type_node_new (self->mem, new_type_id, base_id, ident);
	ptr_list_append (&self->nodes, new_node);

	entry *edges_to_append = ptr_list_at (&self->edges, base_id);

	entry *new_edge = type_edge_new (self->mem, new_node->id, base_id, edges_to_append);
	ptr_list_append (&self->edges, new_edge);

	return (tok){.id = new_type_id};
}

static void
add_base_types (ts *self)
{
	CHECK_NOT_NULL(self);

	const tok any_type = add_any_type (self);
	CHECK_EQUAL(type_nil.id, add_base_type (self, any_type.id, DEP_VAL_STR_VIEW ("nil")).id);
	CHECK_EQUAL(type_object.id, add_base_type (self, any_type.id, DEP_VAL_STR_VIEW ("object")).id);
	CHECK_EQUAL(type_number.id, add_base_type (self, any_type.id, DEP_VAL_STR_VIEW ("number")).id);
	CHECK_EQUAL(type_string.id, add_base_type (self, any_type.id, DEP_VAL_STR_VIEW ("string")).id);
	CHECK_EQUAL(type_unit.id, add_base_type (self, any_type.id, DEP_VAL_STR_VIEW ("unit")).id);
}

static void
free_nodes (ts *self)
{
	CHECK_NOT_NULL(self);
	for (size_t idx = 0; idx < ptr_list_count (&self->nodes); idx++)
		{
			n *to_free = ptr_list_at (&self->nodes, idx);
			type_node_free (to_free, self->mem);
		}
}

static void
free_edges (ts *self)
{
	CHECK_NOT_NULL(self);
	for (size_t idx = 0; idx < ptr_list_count (&self->edges); idx++)
		{
			entry *to_free = ptr_list_at (&self->edges, idx);
			type_edge_free (to_free, self->mem);
		}
}

static n *
resolve_base_node (ts *self, tok type)
{
	CHECK_NOT_NULL(self);
	entry *type_edge = ptr_list_at (&self->edges, type.id);
	for (entry *cur = type_edge; cur != NULL; cur = cur->prev)
		{
			if (cur->base_id == 0)
				{
					n *result = ptr_list_at (&self->nodes, cur->type_id);
					return result;
				}
		}
	return NULL;
}

ts *
type_system_new (struct mem *mem)
{
	CHECK_NOT_NULL(mem);
	ts *result = mem_allocate (mem, sizeof (ts));
	result->mem = mem;
	ptr_list_init (&result->nodes, ptr_list_default_config (mem));
	ptr_list_init (&result->edges, ptr_list_default_config (mem));

	add_base_types (result);

	return result;
}

tok
type_system_compose (ts *self, tok base_type, tok type)
{
	CHECK_NOT_NULL(self);

	// check if this type already exists - if so return otherwise create a new one
	// FIXME It probably makes sense to have a list of all children of a base_node, to greatly decrease the search effort
	for (size_t edge_idx = base_type.id; edge_idx < ptr_list_count (&self->edges); edge_idx++)
		{
			entry *edge = ptr_list_at (&self->edges, edge_idx);
			if (edge->base_id && base_type.id && edge->type_id == type.id)
				{
					return (tok){.id = edge_idx};
				}
		}

	u2 new_type_id = next_type_id (self);
	//FIXME ensure that its actually possible to add to the root type

	entry *edges_to_append = ptr_list_at (&self->edges, base_type.id);

	entry *new_edge = type_edge_new (self->mem, type.id, base_type.id, edges_to_append);
	ptr_list_append (&self->edges, new_edge);

	return (tok){.id = new_type_id};
}

tok
type_system_find_by_ident (ts *self, struct dep_val_str_view ident)
{
	CHECK_NOT_NULL(self);
	for (size_t idx = 0; idx < ptr_list_count (&self->nodes); idx++)
		{
			n *node = ptr_list_at (&self->nodes, idx);

			if (VAL_EQ(node->ident, ident))
				{
					return (tok){.id = node->id};
				}

		}
	ABORT("unable to find type %.*s", (int)ident.count, ident.data);
}

ti
type_system_get_info (ts *self, struct type type)
{
	CHECK_NOT_NULL(self);
	entry *edge = ptr_list_at (&self->edges, type.id);

	n *base_node = resolve_base_node (self, type);
	return (ti){
		.id = type.id,
		.base_id = edge->base_id,
		.ident = DEP_VAL_STR_VIEW(base_node->ident)
	};
}

bool
type_system_is_base_type (ts *self, struct type type)
{
	CHECK_NOT_NULL(self);
	entry *edge = ptr_list_at (&self->edges, type.id);
	return edge->base_id == 0;
}

void
type_system_free (ts *self)
{
	CHECK_NOT_NULL(self);
	free_nodes (self);
	free_edges (self);

	ptr_list_reset (&self->nodes);
	ptr_list_reset (&self->edges);
	mem_deallocate (self->mem, self);
}

void
type_system_free_safe (ts **self)
{
	CHECK_NOT_NULL(self);
	type_system_free (*self);
	*self = NULL;
}
