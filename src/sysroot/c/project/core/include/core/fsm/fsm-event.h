#ifndef CORE_FSM_EVENT_H
#define CORE_FSM_EVENT_H

#include "core/bytes/bytes-buffer.h"

struct fsm_event_type { u1 value; };

struct fsm_event {
  struct fsm_event_type type;
  struct bytes payload;
};

HAMAL_API void
fsm_event_init (struct fsm_event *self, struct mem *mem, struct fsm_event_type event_type, u4 payload_size);

HAMAL_API void
fsm_event_init_without_payload (struct fsm_event *self, struct fsm_event_type event_type);

HAMAL_API void *
fsm_event_payload (struct fsm_event self);

 HAMAL_API bool
fsm_event_has_type (struct fsm_event self, struct fsm_event_type type);

HAMAL_API void
fsm_event_reset (struct fsm_event *self, struct mem *mem);

#endif //CORE_FSM_EVENT_H
