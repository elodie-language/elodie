#ifndef CORE_FSM_STATE_H
#define CORE_FSM_STATE_H

#include "core/bytes/bytes.h"
#include "fsm-event.h"

struct fsm_state_id { u1 value; };

 HAMAL_API bool
fsm_state_id_equal (struct fsm_state_id lhs, struct fsm_state_id rhs);

struct fsm_state {
  struct fsm_state_id id;
  struct bytes payload;
  void (*apply_fn) (struct fsm_state *self, struct fsm_event event);
};

HAMAL_API struct fsm_state *
fsm_state_allocate (struct mem *mem, struct fsm_state_id id, u4 payload_size, void (*init_state_fn) (struct fsm_state *self));

HAMAL_API void
fsm_state_apply (struct fsm_state *self, struct fsm_event event);

HAMAL_API void *
fsm_state_payload (struct fsm_state *self);

HAMAL_API void
fsm_state_deallocate (struct fsm_state *self, struct mem *mem);

HAMAL_API void
fsm_state_deallocate_safe (struct fsm_state **self, struct mem *mem);

HAMAL_API void
fsm_state_no_initialization (struct fsm_state *self);

#endif //CORE_FSM_STATE_H
