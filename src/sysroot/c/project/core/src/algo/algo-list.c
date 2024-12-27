#include <string.h>
#include "core/core-api.h"
#include "core/algo/algo-list.h"

typedef struct list lst;
typedef struct list_config c;
typedef struct mem m;

static lst *
list_resize (void *self)
{
	CHECK_NOT_NULL(self);
	lst *impl = (lst *)self;

	size_t size = impl->capacity * 2 * impl->stride;
	LOG_TRACE("list %p resized from %lu to %lu", self, impl->capacity, impl->capacity * 2);
	void *new_data = mem_allocator_allocate (impl->mem->allocator, size);
	memcpy(new_data, impl->data, impl->count * impl->stride);
	mem_deallocator_deallocate (impl->mem->deallocator, impl->data);
	impl->data = new_data;
	impl->capacity *= 2;

	return impl;
}

c
list_default_config (m *mem)
{
	CHECK_NOT_NULL(mem);
	c result = {
		.initial_capacity = 8,
		.resize_factor = 2,
		.mem = mem
	};
	return result;
}

lst *
prv_list_new (c config, size_t stride)
{
	CHECK_NOT_NULL(config.mem);
	lst *result = mem_allocate (config.mem, sizeof (lst));
	prv_list_init (result, config, stride);
	return result;
}

void
prv_list_init (lst *self, c config, size_t stride)
{
	CHECK_NOT_NULL(self);
	self->count = 0;
	self->capacity = config.initial_capacity;
	self->stride = stride;
	self->mem = config.mem;
	self->data = mem_allocate (config.mem, self->capacity * self->stride);
}

void
list_copy_into (lst *self, lst *target)
{
	CHECK_NOT_NULL(self);
	for (size_t idx = 0; idx < list_count (self); idx++)
		{
			list_append (target, list_at (self, idx));
		}
}

lst *
list_append (lst *self, void *val)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(val);

	size_t count = self->count;
	size_t stride = self->stride;

	lst *result = list_ensure_capacity (self, (count + 1));
	memcpy((u1 *)result->data + count * stride, val, stride);
	result->count++;
	return result;
}

lst *
list_ensure_capacity (lst *self, size_t required_capacity)
{
	CHECK_NOT_NULL(self);
	CHECK_GREATER_THAN(required_capacity, 0);
	lst *result = (lst *)self;
	size_t capacity = list_capacity (result);

	// FIXME this is very inenativecient - it should resize only once
	while (required_capacity > capacity)
		{
			result = list_resize (self);
			capacity = list_capacity (result);
		}
	return result;
}

size_t
list_count (lst *self)
{
	CHECK_NOT_NULL(self);
	return self->count;
}

size_t
list_capacity (lst *self)
{
	CHECK_NOT_NULL(self);
	return self->capacity;
}

void *
list_at (lst *self, size_t idx)
{
	CHECK_NOT_NULL(self);
	CHECK_LESS_THAN(idx, self->capacity);
	return (void *)((u1 *)self->data + idx * self->stride);
}

static bool
list_iterator_cb_has_next (struct iterator *it)
{
	CHECK_NOT_NULL(it);
	return it->current.index < list_count (it->target);
}

static void *
list_iterator_cb_next (struct iterator *it)
{
	CHECK_NOT_NULL(it);
	return list_at (it->target, it->current.index++);
}

struct iterator
list_iterator (lst *self)
{
	CHECK_NOT_NULL(self);
	return iterator_index (self, list_iterator_cb_has_next, list_iterator_cb_next);
}

lst *
list_replace (lst *self, size_t idx, void *ptr)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(ptr);

	size_t stride = self->stride;
	memcpy((u1 *)self->data + idx * stride, ptr, stride);
	return self;
}

void
list_reset (lst *self)
{
	CHECK_NOT_NULL(self);
	if (self->data != NULL)
		{
			mem_deallocate (self->mem, self->data);
			self->data = NULL;
			self->count = 0;
			self->capacity = 0;
			self->stride = 0;
		}
}

void
list_free (lst *self)
{
	CHECK_NOT_NULL(self);
	if (self->data != NULL)
		{
			mem_deallocate (self->mem, self->data);
		}
	mem_deallocate (self->mem, self);
}

void
list_free_safe (lst **self)
{
	CHECK_NOT_NULL(self);
	list_free (*self);
	*self = NULL;
}
