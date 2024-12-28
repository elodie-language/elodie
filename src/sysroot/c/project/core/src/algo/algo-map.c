#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdint.h>
#include <stddef.h>

#include "core/core-api.h"
#include "core/algo/algo-map.h"

typedef struct map map;
typedef struct map_config c;
typedef struct map_entry e;
typedef struct map_entry_view ev;
typedef struct map_key k;
typedef struct bytes b;
typedef struct bytes_view bv;
typedef struct mem m;

void
map_key_init (struct map_key *self, struct hash8 hash)
{
	CHECK_NOT_NULL(self);
	CHECK_GREATER_THAN(hash.value, 0);
	self->hash = hash;
}

k
map_key_from_bytes (map *self, struct bytes_view bytes)
{
	CHECK_NOT_NULL(self);
	k result = {0};
	map_key_init (&result, hash8_of (self->key_hash_fn, bytes));
	return result;
}

k
map_key_from_str (struct map *self, struct val_str *str)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(str);
	return map_key_from_bytes (
		self,
		(struct bytes_view){
			.data = (u1 *)str->data,
			.size = str->count
		}
	);
}

k
map_key_from_str_view (map *self, struct val_str_view view)
{
	CHECK_NOT_NULL(self);
	return map_key_from_bytes (
		self,
		(struct bytes_view){
			.data = (u1 *)view.data,
			.size = view.count
		}
	);
}

k
map_key_from_string_view (struct map *self, struct string_view view)
{
	CHECK_NOT_NULL(self);
	return map_key_from_bytes (
		self,
		(struct bytes_view){
			.data = (u1 *)view.data,
			.size = view.count
		}
	);
}

k
map_key_from_size_t (map *self, size_t value)
{
	CHECK_NOT_NULL(self);
	CHECK_LESS_THAN(value, U8_MAX - 1);
	size_t data = value + 1;
	return map_key_from_bytes (
		self,
		(struct bytes_view){
			.data = (u1 *)&data,
			.size = sizeof (size_t)
		}
	);
}

k
map_key_from_c_str (map *self, char const *str)
{
	CHECK_NOT_NULL(self);
	size_t count = strlen (str);
	struct bytes_view bytes = (struct bytes_view){.size = count, .data = (u1 *)str};
	return map_key_from_bytes (self, bytes);
}

static e *
find_entry (e *entries, size_t capacity, k *key)
{
	CHECK_NOT_NULL(entries);
	CHECK_NOT_NULL(key);
	size_t index = key->hash.value % capacity;
	for (;;)
		{
			e *entry = &entries[index];
			if (entry->key.hash.value == key->hash.value)
				{
					return entry;
				}
			else
				{
					if (entry->key.hash.value == 0)
						{
							return entry;
						}
				}
			index = (index + 1) % capacity;
		}
}

static void
map_destroy_entry (map *self, e *entry)
{
	if (entry != NULL && entry->value.size != 0)
		{
			bytes_reset (&entry->value, self->mem);
		}
}

static void
map_destroy_entries (map *self)
{
	CHECK_NOT_NULL(self);
	for (size_t idx = 0; idx < self->capacity; idx++)
		{
			e *entry = &self->entries[idx];
			map_destroy_entry (self, entry);
		}
}

static bool
map_set_internal (map *self, ev entry_view)
{
	CHECK_NOT_NULL(self);

	e *result = find_entry (self->entries, self->capacity, &entry_view.key);
	bool is_new_key = result->key.hash.value == 0;
	if (is_new_key)
		{
			self->count++;
		}
	result->key.hash = entry_view.key.hash;
	if (!is_new_key)
		{
			map_destroy_entry (self, result);
		}
	if (entry_view.value.size > 0)
		{
			bytes_init (&result->value, self->mem, entry_view.value.size);
			memcpy(result->value.data, entry_view.value.data, entry_view.value.size);
		}
	else
		{
			result->value = NO_BYTES;
		}
	return is_new_key;
}

static void
resize (map *self, size_t new_capacity)
{
	CHECK_NOT_NULL(self);
	map new_map;
	map_init (&new_map, (struct map_config){
		.mem = self->mem,
		.initial_capacity = new_capacity,
		.key_hash_fn = self->key_hash_fn
	});

	/**
	 * copying data would be faster - but then all data sits in the first half and none in the second half of the bucket
	 * --> we make sure that data is evenly distributed
	 */
	for (size_t idx = 0; idx < self->capacity; idx++)
		{
			e *entry = &self->entries[idx];
			if (entry->key.hash.value == 0)
				{
					continue;
				}
			map_set_internal (&new_map, (ev){
				.key = entry->key,
				.value = {
					.data =entry->value.data,
					.size = entry->value.size
				}
			});
		}
	map_destroy_entries (self);
	mem_deallocate (self->mem, self->entries);

	self->count = new_map.count;
	self->capacity = new_map.capacity;
	self->entries = new_map.entries;
}

map *
map_new (c config)
{
	map *result = mem_allocator_allocate (config.mem->allocator, sizeof (map));
	map_init (result, config);
	return result;
}

void
map_init (map *self, c config)
{
	CHECK_NOT_NULL(self);
	self->capacity = (config.initial_capacity > 8) ? config.initial_capacity : 8;
	self->mem = config.mem;
	self->key_hash_fn = config.key_hash_fn;
	self->count = 0;
	self->entries = mem_allocator_allocate (config.mem->allocator, sizeof (e) * self->capacity);
	memset(self->entries, '\0', sizeof (e) * self->capacity);
}

map *
map_copy (map *self, m *mem)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(mem);
	c map_config = {
		.mem = mem,
		.initial_capacity = self->capacity,
		.key_hash_fn = self->key_hash_fn
	};

	map *result = map_new (map_config);

	for (size_t idx = 0; idx < self->capacity; idx++)
		{
			e *entry = &self->entries[idx];
			if (entry->key.hash.value == 0) continue;
			map_set_internal (result, (ev){
				.key = entry->key,
				.value = {
					.data =entry->value.data,
					.size = entry->value.size
				}
			});
		}

	return result;
}

void
map_copy_into (map *self, map *target)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(target);
	for (size_t idx = 0; idx < self->capacity; idx++)
		{
			e *entry = &self->entries[idx];
			if (entry->key.hash.value == 0) continue;
			map_set_internal (target, (ev){
				.key = entry->key,
				.value = {
					.data =entry->value.data,
					.size = entry->value.size
				}
			});
		}
}

bool
map_set_bytes_view (map *self, k key, struct bytes_view bytes)
{
	CHECK_NOT_NULL(self);

	if (self->count >= (size_t)((double)self->capacity * 0.75))
		{
			resize (self, self->capacity * 2);
		}

	return map_set_internal (self, (ev){
		.key = key,
		.value = bytes
	});
}

bool
map_get_as_entry_view (map *self, k key, ev *out)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(out);

	if (self->count == 0)
		{
			return false;
		}

	e *entry = find_entry (self->entries, self->capacity, &key);
	if (entry->value.size == 0)
		{
			return false;
		}
	*out = (ev){
		.value = bytes_view_of_bytes (entry->value),
		.key = entry->key
	};
	return true;
}

bool
map_get_as_bytes_view (struct map *self, struct map_key key, struct bytes_view *out)
{
	struct map_entry_view entry_view;
	bool result = map_get_as_entry_view (self, key, &entry_view);
	if (!result)
		{
			return false;
		}
	*out = entry_view.value;
	return result;
}

bool
map_has_key (map *self, k key)
{
	if (self->count == 0)
		{
			return false;
		}

	e *entry = find_entry (self->entries, self->capacity, &key);
	return entry->value.size != 0;
}

static bool
map_iterator_cb_has_next (struct iterator *it)
{
	CHECK_NOT_NULL(it);
	map *instance = it->target;
	for (size_t idx = it->current.index; idx < instance->capacity; idx++)
		{
			if (instance->entries[idx].key.hash.value != 0)
				{
					return true;
				}
		}
	return false;
}

static void *
map_iterator_cb_next (struct iterator *it)
{
	CHECK_NOT_NULL(it);
	map *instance = it->target;
	while (it->current.index < instance->capacity)
		{
			if (instance->entries[it->current.index].key.hash.value != 0)
				{

					void *result = &instance->entries[it->current.index].key;
					it->current.index++;
					return result;
				}
			else
				{
					it->current.index++;
				}
		}
	ILLEGAL_STATE();
}

struct iterator
map_keys_iterator (map *self)
{
	CHECK_NOT_NULL(self);
	return iterator_index (self, map_iterator_cb_has_next, map_iterator_cb_next);
}

bool
map_remove (map *self, k key)
{
	CHECK_NOT_NULL(self);
	if (self->count == 0)
		{
			return false;
		}

	e *entry = find_entry (self->entries, self->capacity, &key);
	if (entry == NULL)
		{
			return false;
		}

	entry->key.hash = (struct hash8){.value=0};
	bytes_reset (&entry->value, self->mem);

	self->count--;
	return true;
}

size_t
map_count (map *self)
{
	CHECK_NOT_NULL(self);
	return self->count;
}

size_t
map_capacity (map *self)
{
	CHECK_NOT_NULL(self);
	return self->capacity;
}

void
map_reset (map *self)
{
	CHECK_NOT_NULL(self);
	map_destroy_entries (self);
	mem_deallocate (self->mem, self->entries);
	self->capacity = 0;
	self->mem = (struct mem *)mem_null_new ();
	self->count = 0;
	self->entries = NULL;
}

void
map_free (map *self)
{
	CHECK_NOT_NULL(self);
	map_destroy_entries (self);
	mem_deallocate (self->mem, self->entries);
	mem_deallocate (self->mem, self);
}

void
map_free_safe (map **self)
{
	CHECK_NOT_NULL(self);
	map_free (*self);
	*self = NULL;
}
