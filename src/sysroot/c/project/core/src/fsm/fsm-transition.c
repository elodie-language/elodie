#include "core/check.h"
#include "core/fsm/fsm-api.h"

typedef struct fsm_transition si;
typedef struct fsm f;
typedef struct fsm_event e;
typedef struct fsm_state s;

bool
fsm_transition_test (si transition, f *fsm, e event)
{
	CHECK_NOT_NULL(fsm);
	s *origin = fsm_get_state (fsm, transition.origin);
	s *destination = fsm_get_state (fsm, transition.destination);
	return transition.transition_test_fn (origin, destination, event);
}

