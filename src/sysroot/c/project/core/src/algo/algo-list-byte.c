#include "core/core-api.h"

#include <string.h>

typedef struct byte_list bv;
typedef struct byte_list_config c;

static inline void *
direct_underlying_data (bv *self, size_t idx)
{
	return list_at (&self->underlying_list, idx);
}

c
byte_list_default_config (struct mem *mem)
{
	CHECK_NOT_NULL(mem);
	return (c){
		.initial_capacity = 8,
		.mem = mem,
	};
}

bv *
byte_list_new (c config)
{
	struct byte_list *result = mem_allocate (config.mem, sizeof (bv));
	byte_list_init (result, config);
	return result;
}

void
byte_list_init (bv *self, c config)
{
	struct list_config underlying_config = list_default_config (config.mem);
	underlying_config.initial_capacity = config.initial_capacity;

	list_init(&self->underlying_list, underlying_config, u1);
}

void
byte_list_append_u1 (struct byte_list *self, u1 data)
{
	CHECK_NOT_NULL(self);
	struct list *underlying_list = &self->underlying_list;
	list_append (underlying_list, &data);
}

void
byte_list_append_u2 (struct byte_list *self, u2 data)
{
	CHECK_NOT_NULL(self);
	byte_list_append_bytes (self, bytes_view_of_u2 (&data));
}

void
byte_list_append_u4 (struct byte_list *self, u4 data)
{
	CHECK_NOT_NULL(self);
	byte_list_append_bytes (self, bytes_view_of_u4 (&data));
}

void
byte_list_replace_u4 (struct byte_list *self, size_t idx, u4 data)
{
	CHECK_NOT_NULL(self);
	byte_list_replace_bytes (self, idx, bytes_view_of_u4 (&data));
}

void
byte_list_append_u8 (struct byte_list *self, u8 data)
{
	CHECK_NOT_NULL(self);
	byte_list_append_bytes (self, bytes_view_of_u8 (&data));
}

void
byte_list_append_c_str (struct byte_list *self, char const *str)
{
	CHECK_NOT_NULL(self);
	byte_list_append_bytes (self, bytes_view_of_c_str (str));
}

void
byte_list_append_bytes (bv *self, struct bytes_view data)
{
	CHECK_NOT_NULL(self);
	struct list *lst = &self->underlying_list;
	list_ensure_capacity (lst, byte_list_size (self) + data.size);
	memcpy((u1 *)lst->data + lst->count, data.data, data.size);
	lst->count += data.size;
}

void
byte_list_replace_bytes (struct byte_list *self, size_t idx, struct bytes_view data)
{
	CHECK_NOT_NULL(self);
	struct list *lst = &self->underlying_list;
	memcpy((u1 *)lst->data + idx, data.data, data.size);
}

void
byte_list_append_front_bytes (struct byte_list *self, struct bytes_view data)
{
	size_t current_count = byte_list_size (self);
	struct list *lst = &self->underlying_list;
	list_ensure_capacity (lst, current_count + data.size);
	memmove((u1 *)lst->data + data.size, lst->data, current_count);
	memcpy((u1 *)lst->data, data.data, data.size);
	lst->count += data.size;
}

void
byte_list_append_byte_list (struct byte_list *self, struct byte_list *lst)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(lst);
	byte_list_append_bytes (self, (struct bytes_view){
		.size = lst->underlying_list.count,
		.data = lst->underlying_list.data
	});
}

void
byte_list_append_front_byte_list (struct byte_list *self, struct byte_list *lst)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(lst);
	byte_list_append_front_bytes (self, (struct bytes_view){
		.size = lst->underlying_list.count,
		.data = lst->underlying_list.data
	});
}

void
byte_list_append_str (struct byte_list *self, struct dep_val_str *str)
{
	CHECK_NOT_NULL(self);
	struct bytes_view bytes = (struct bytes_view){
		.size = str->count,
		.data = (u1 *)str->data
	};
	byte_list_append_bytes (self, bytes);
}

HAMAL_API void
byte_list_append_str_view (struct byte_list *self, struct dep_val_str_view str)
{
	byte_list_append_bytes (self, (struct bytes_view){
		.size = str.count,
		.data = (u1 *)str.data
	});
}

bool
byte_list_at_u1 (bv *self, size_t idx, u1 *out)
{
	CHECK_NOT_NULL(self);
	void **ptr = direct_underlying_data (self, idx);
	if (!ptr)
		{
			return false;
		}
	*out = *(u1 *)ptr;
	return true;
}

bool
byte_list_at_u2 (bv *self, size_t idx, u2 *out)
{
	CHECK_NOT_NULL(self);
	void **ptr = direct_underlying_data (self, idx);
	if (!ptr)
		{
			return false;
		}
	*out = *(u2 *)ptr;
	return true;
}

bool
byte_list_at_u4 (bv *self, size_t idx, u4 *out)
{
	CHECK_NOT_NULL(self);
	void **ptr = direct_underlying_data (self, idx);
	if (!ptr)
		{
			return false;
		}
	*out = *(u4 *)ptr;
	return true;
}

bool
byte_list_at_str_view (struct byte_list *self, size_t idx, size_t count, struct dep_val_str_view *out)
{
	CHECK_NOT_NULL(self);
	void **ptr = direct_underlying_data (self, idx);
	if (!ptr)
		{
			return false;
		}

	*out = dep_val_str_view_from_bytes (
		(struct bytes_view){
			.size = count,
			.data = (u1 *)ptr
		}
	);
	return true;
}

HAMAL_API bool
byte_list_at (struct byte_list *self, size_t idx, u1 **out)
{
	CHECK_NOT_NULL(self);
	void **ptr = direct_underlying_data (self, idx);
	if (!ptr)
		{
			return false;
		}
	*out = (u1 *)((u1 **)ptr);
	return true;
}

char *
byte_list_raw_c_str (struct byte_list *self)
{
	CHECK_NOT_NULL(self);
	return (char *)direct_underlying_data (self, 0);
}

struct bytes_view
byte_list_raw_bytes (struct byte_list *self)
{
	CHECK_NOT_NULL(self);
	return (struct bytes_view){
		.data = (u1 *)direct_underlying_data (self, 0),
		.size = self->underlying_list.count
	};
}

void *
byte_list_data_ptr (struct byte_list *self)
{
	CHECK_NOT_NULL(self);
	return self->underlying_list.data;
}

size_t
byte_list_size (bv *self)
{
	CHECK_NOT_NULL(self);
	return list_count (&self->underlying_list);
}

size_t
byte_list_capacity (bv *self)
{
	CHECK_NOT_NULL(self);
	return list_capacity (&self->underlying_list);
}

void
byte_list_reset (struct byte_list *self)
{
	CHECK_NOT_NULL(self);
	list_reset (&self->underlying_list);
}

void
byte_list_free (struct byte_list *self)
{
	CHECK_NOT_NULL(self);
	struct mem *mem = self->underlying_list.mem;
	byte_list_reset (self);
	mem_deallocate (mem, self);
}

void
byte_list_free_safe (bv **self)
{
	CHECK_NOT_NULL(self);
	byte_list_free (*self);
	*self = NULL;
}
