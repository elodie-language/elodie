#include "core/check.h"
#include "core/algo/algo-set.h"

typedef struct bytes_view bv;
typedef struct map_key k;
typedef struct set s;
typedef struct set_config c;

static bool
has_key (s *self, k key)
{
	return map_has_key (&self->store, key);
}

s *
set_new (c config)
{
	s *result = mem_allocate (config.mem, sizeof (s));
	set_init (result, config);
	return result;
}

void
set_init (s *self, c config)
{
	CHECK_NOT_NULL(self);
	self->mem = config.mem;

	struct map_config map_config = {
		.mem = config.mem,
		.initial_capacity = config.initial_capacity,
		.key_hash_fn = config.hash_fn
	};
	map_init (&self->store, map_config);
}

size_t
set_count (s *self)
{
	CHECK_NOT_NULL(self);
	return map_count (&self->store);
}

size_t
set_capacity (s *self)
{
	CHECK_NOT_NULL(self);
	return map_capacity (&self->store);
}

bool
set_set (s *self, bv bytes)
{
	CHECK_NOT_NULL(self);
	k key = map_key_from_bytes (&self->store, bytes);
	if (has_key (self, key))
		{
			return false;
		}
	map_set_bytes_view (&self->store, key, bytes);
	return true;
}

bool
set_has (s *self, bv bytes)
{
	CHECK_NOT_NULL(self);
	k key = map_key_from_bytes (&self->store, bytes);
	return has_key (self, key);
}

void
set_reset (s *self)
{
	CHECK_NOT_NULL(self);
	map_reset (&self->store);
}

void
set_free (s *self)
{
	CHECK_NOT_NULL(self);
	map_reset (&self->store);
	mem_deallocate (self->mem, self);
}

void
set_free_safe (s **self)
{
	CHECK_NOT_NULL(self);
	set_free (*self);
	*self = NULL;
}

