#include "core/check.h"
#include "core/algo/algo-list-byte.h"
#include "core/val/val-str.h"
#include "core/val/val-fn.h"

// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-[val fn block]+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-

struct val_fn_block *
val_fn_block_new (struct mem *mem)
{
	struct val_fn_block *result = mem_allocate (mem, sizeof (struct val_fn_block));
	result->mem = mem;
	result->data = byte_list_new (
		(struct byte_list_config){
			.initial_capacity = 64,
			.mem = mem
		}
	);
	return result;
}

void
val_fn_block_append (struct val_fn_block *self, u4 instr)
{
	CHECK_NOT_NULL(self);
	byte_list_append_u4 (self->data, instr);
}

size_t
val_fn_block_count (struct val_fn_block *self)
{
	CHECK_NOT_NULL(self);
	return byte_list_size (self->data) / 4;
}

void
val_fn_block_free (struct val_fn_block *self)
{
	CHECK_NOT_NULL(self);
	byte_list_free_safe (&self->data);
	mem_deallocate (self->mem, self);
}

void
val_fn_block_free_safe (struct val_fn_block **self)
{
	CHECK_NOT_NULL(self);
	val_fn_block_free (*self);
	*self = NULL;
}

// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-[val fn]+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-

struct val_fn_block_ptr {
  struct val_fn_block *ptr;
};

struct val_fn *
val_fn_new (struct mem *mem, struct dep_val_str_view ident)
{
	struct val_fn *result = mem_allocate (mem, sizeof (struct val_fn));
	dep_val_init (&result->base, VAL_KIND_FN, mem);
	result->ident = dep_val_str_allocate_from_view (mem, ident);

	struct list_config blocks_config = list_default_config (mem);
	result->blocks = list_new(blocks_config, struct val_fn_block_ptr);

	return result;
}

void
val_fn_append_block (struct val_fn *self, struct val_fn_block *block)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(block);
	struct val_fn_block_ptr entry = {.ptr = block};
	list_append (self->blocks, &entry);
}

struct val_fn_block *
val_fn_get_block_at (struct val_fn *self, size_t idx)
{
	CHECK_NOT_NULL(self);
	CHECK_LESS_THAN(idx, list_count (self->blocks));
	struct val_fn_block_ptr *entry = list_at (self->blocks, idx);
	return entry->ptr;
}

size_t
val_fn_count (struct val_fn *self)
{
	CHECK_NOT_NULL(self);
	return list_count (self->blocks);
}

bool
val_fn_equal (struct val_fn *lhs, struct val_fn *rhs)
{
	CHECK_NOT_NULL(lhs);
	CHECK_NOT_NULL(rhs);
	if (lhs == rhs) return true;
	return dep_val_str_equal (lhs->ident, rhs->ident);
}

struct dep_val_str *
val_fn_to_str (struct val_fn *self, struct mem *mem)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(mem);
	return dep_val_str_copy (self->ident, mem);
}

void
val_fn_free (struct val_fn *self)
{
	CHECK_NOT_NULL(self);
	dep_val_str_deallocate_safe (&self->ident);
	for (size_t idx = 0; idx < list_count (self->blocks); idx++)
		{
			struct val_fn_block_ptr *entry = list_at (self->blocks, idx);
			val_fn_block_free_safe (&entry->ptr);
		}
	list_free_safe (&self->blocks);
	mem_deallocate (self->base.mem, self);
}

void
val_fn_free_safe (struct val_fn **self)
{
	CHECK_NOT_NULL(self);
	val_fn_free (*self);
	*self = NULL;
}
