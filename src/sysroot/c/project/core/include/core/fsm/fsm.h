#ifndef CORE_FSM_H
#define CORE_FSM_H

#include "core/core.h"
#include "core/bytes/bytes.h"
#include "core/algo/algo-list-ptr.h"
#include "core/algo/algo-map-ptr.h"

#include "fsm-state.h"
#include "fsm-transition.h"

struct fsm {
  struct mem *mem;
  struct fsm_state *current_state;
  struct ptr_list states;
  struct ptr_map transitions;
};

HAMAL_API void
fsm_init (struct fsm *self, struct mem *mem);

HAMAL_API struct fsm_state_id
fsm_current_state_id (struct fsm *self);

HAMAL_API struct fsm_state *
fsm_get_state (struct fsm *self, struct fsm_state_id state_id);

HAMAL_API void
fsm_accept (struct fsm *self, struct fsm_event event);

HAMAL_API struct fsm_state_id
fsm_register_state (struct fsm *self, u4 payload_size, void (*init_state_fn) (struct fsm_state *self));

HAMAL_API bool
fsm_add_transition (struct fsm *self, struct fsm_state_id origin, struct fsm_state_id destination, fsm_transition_test_fn transition_test_fn);

HAMAL_API void
fsm_reset (struct fsm *self);

#endif //CORE_FSM_H
