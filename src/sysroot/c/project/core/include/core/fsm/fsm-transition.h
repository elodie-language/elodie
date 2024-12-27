#ifndef CORE_FSM_TRANSITION_H
#define CORE_FSM_TRANSITION_H

#include "fsm-state.h"

struct fsm;

typedef bool (*fsm_transition_test_fn) (struct fsm_state *origin, struct fsm_state *destination, struct fsm_event event);

struct fsm_transition {
  struct fsm_state_id origin;
  struct fsm_state_id destination;
  fsm_transition_test_fn transition_test_fn;
};

HAMAL_API bool
fsm_transition_test (struct fsm_transition transition, struct fsm *fsm, struct fsm_event event);

#endif //CORE_FSM_TRANSITION_H
