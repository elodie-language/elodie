#include "core/check.h"
#include "core/algo/algo-list.h"
#include "core/val/val-api.h"
#include "core/mem/mem-gc.h"

static void *
allocate (void *ctx, size_t size)
{
	NOT_IMPLEMENTED_YET();
}

static void
deallocate (void *ctx, void *)
{
	NOT_IMPLEMENTED_YET();
}

struct mem_gc *
mem_gc_new (struct mem_gc_config config)
{
	struct mem_allocator allocator = config.root->allocator;
	struct mem_gc *result = mem_allocator_allocate (allocator, sizeof (struct mem_gc));
	mem_gc_init (result, config);
	return result;
}

void
mem_gc_init (struct mem_gc *self, struct mem_gc_config config)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(config.root);
	CHECK_GREATER_THAN(config.size, 0);
	self->base = (struct mem){
		.kind = MEM_KIND_GC,
		.allocator = (struct mem_allocator){
			.self = self,
			.fn = allocate
		},
		.deallocator = (struct mem_deallocator){
			.self = self,
			.fn = deallocate
		},
		.root = config.root
	};

	struct list_config list_config = list_default_config (config.root);
	list_config.initial_capacity = config.size;

	self->colors = list_new(list_config, enum mem_gc_color);
	self->vals = val_lst_new (config.root);

}

struct val *
mem_gc_allocate (struct mem_gc *self, struct val *val)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(val);

	struct val *result = val_copy (val, self->base.root);

	switch (val->kind)
		{
			//
			case VAL_KIND_BOOL:
			case VAL_KIND_NUM:
				{
					list_append_rval(self->colors, MEM_GC_COLOR_WHITE);
					val_lst_append (self->vals, result);
					break;
				}
			default: NOT_IMPLEMENTED_YET();
		}

	return result;
}

void
mem_gc_mark_val (struct mem_gc *self, struct val *val, enum mem_gc_color color)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(val);

	for (size_t idx = 0; idx < val_lst_count (self->vals); idx++)
		{
			struct val *current = NULL;
			if (current == val)
				{
					enum mem_gc_color *ptr = list_at (self->colors, idx);
					*ptr = color;
				}
		}

}

void
mem_gc_mark (struct mem_gc *self, struct val *val)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(val);

//  for (size_t val_idx = 0; val_idx < val_lst_count (vals); val_idx++)
//    {
//      struct val *val = val_lst_get_base (vals, val_idx);
	mem_gc_mark_val (self, val, MEM_GC_COLOR_BLACK);
//    }

}

void
mem_gc_sweep (struct mem_gc *self)
{
	CHECK_NOT_NULL(self);

	struct list *new_colors = list_new(list_default_config (self->base.root), enum mem_gc_color);
	struct val_lst *new_val_list = val_lst_new (self->base.root);

	size_t keep_counter = 0;
	size_t sweep_counter = 0;
	for (size_t idx = 0; idx < list_count (self->colors); idx++)
		{
			enum mem_gc_color color = *(enum mem_gc_color *)list_at (self->colors, idx);

			if (color == MEM_GC_COLOR_WHITE)
				{
					struct val *val_to_sweep = val_lst_at_base (self->vals, idx);
					val_free_safe (&val_to_sweep);
					sweep_counter++;
				}
			else
				{
					struct val *val_to_keep = val_lst_at_base (self->vals, idx);
					list_append_rval(new_colors, MEM_GC_COLOR_WHITE);
					val_lst_append_base (new_val_list, val_to_keep);
					keep_counter++;
				}

		}
	LOG_DEBUG("keep %d and swept %d", keep_counter, sweep_counter);

	list_free_safe (&self->colors);
	val_lst_free_safe (&self->vals);

	self->colors = new_colors;
	self->vals = new_val_list;
}

size_t
mem_gc_count (struct mem_gc *self)
{
	CHECK_NOT_NULL(self);
	return list_count (self->colors);
}

void
mem_gc_free (struct mem_gc *self)
{
	CHECK_NOT_NULL(self);

	list_free_safe (&self->colors);
	val_lst_clear (self->vals);
	val_lst_free_safe (&self->vals);
	mem_deallocator_deallocate (self->base.root->deallocator, self);
}

void
mem_gc_free_safe (struct mem_gc **self)
{
	CHECK_NOT_NULL(self);
	mem_gc_free (*self);
	*self = NULL;
}
