#include "core/check.h"
#include "core/fsm/fsm-event.h"

typedef struct fsm_event e;
typedef struct fsm_event_type si;
typedef struct mem m;

void
fsm_event_init (e *self, m *mem, si event_type, u4 payload_size)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(mem);
	CHECK_GREATER_THAN(event_type.value, 0);
	self->type = event_type;
	bytes_init (&self->payload, mem, payload_size);
}

void
fsm_event_init_without_payload (e *self, si event_type)
{
	CHECK_NOT_NULL(self);
	self->type = event_type;
	self->payload = NO_BYTES;
}

void *
fsm_event_payload (e self)
{
	CHECK_GREATER_THAN(self.payload.size, 0);
	return (void *)self.payload.data;
}

bool
fsm_event_has_type (e self, si type)
{
	return self.type.value == type.value;
}

void
fsm_event_reset (e *self, m *mem)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(mem);
	bytes_reset (&self->payload, mem);
	self->type.value = 0;
}

