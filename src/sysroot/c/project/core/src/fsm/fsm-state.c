#include "core/check.h"
#include "core/fsm/fsm-state.h"
#include "core/mem/mem-api.h"

typedef struct fsm_event e;
typedef struct fsm_state s;
typedef struct fsm_state_id si;
typedef struct mem m;

void
fsm_state_no_initialization (s *self)
{}

bool
fsm_state_id_equal (si lhs, si rhs)
{
	return lhs.value == rhs.value;

}

s *
fsm_state_allocate (m *mem, si id, u4 payload_size, void (*init_state_fn) (struct fsm_state *self))
{
	CHECK_NOT_NULL(mem);
	CHECK_NOT_NULL(init_state_fn);
	s *result = mem_allocate (mem, sizeof (s));
	result->id = id;
	result->apply_fn = NULL;
	if (payload_size > 0)
		{
			bytes_init (&result->payload, mem, payload_size);
		}
	else
		{
			result->payload = NO_BYTES;
		}
	init_state_fn (result);
	return result;
}

void
fsm_state_init (struct fsm_state *self, void (*init_state_fn) (struct fsm_state *self))
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(init_state_fn);
	init_state_fn (self);
}

void
fsm_state_apply (s *self, e event)
{
	CHECK_NOT_NULL(self);
	if (self->apply_fn != NULL)
		{
			self->apply_fn (self, event);
		}
}

void *
fsm_state_payload (s *self)
{
	return (void *)self->payload.data;
}

void
fsm_state_deallocate (s *self, m *mem)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(mem);
	bytes_reset (&self->payload, mem);
	mem_deallocate (mem, self);
}

void
fsm_state_deallocate_safe (s **self, m *mem)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(mem);
	fsm_state_deallocate (*self, mem);
	*self = NULL;
}
