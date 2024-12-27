#include "../unit-test.h"
#include "core/fsm/fsm.h"

struct active_state_payload {
  u4 barrier;
};

static bool
active_to_active_transition_test (struct fsm_state *origin, struct fsm_state *destination, struct fsm_event evt)
{
	auto payload = (struct active_state_payload *)fsm_state_payload (origin);
	return payload->barrier > 0;
}

static bool
active_to_halt_transition_test (struct fsm_state *origin, struct fsm_state *destination, struct fsm_event evt)
{
	auto payload = (struct active_state_payload *)fsm_state_payload (origin);
	return payload->barrier == 0;
}

const struct fsm_event_type countdown_event_type = {.value = 1};
struct countdown_event_payload {
  u1 decrement_amount;
};

const struct fsm_state_id active_state_type = {.value = 1};
const struct fsm_state_id halt_state_type = {.value = 2};

static void
active_state_apply_event_fn (struct fsm_state *self, struct fsm_event event)
{
	if (fsm_event_has_type (event, countdown_event_type))
		{
			auto event_payload = (struct countdown_event_payload *)fsm_event_payload (event);
			auto state_payload = (struct active_state_payload *)fsm_state_payload (self);
			state_payload->barrier -= event_payload->decrement_amount;
		}
}

static void
active_state_init (struct fsm_state *state)
{
	state->apply_fn = active_state_apply_event_fn;
	((struct active_state_payload *)fsm_state_payload (state))->barrier = 2;
}

static void
halt_state_init (struct fsm_state *state)
{
}

TEST(fsm, countdown)
{
	auto tm = mem_test_new_default (1024);
	struct fsm test_instance{};
	fsm_init (&test_instance, MEM(tm));

	struct fsm_state_id active_state_id = fsm_register_state (&test_instance, 4, active_state_init);
	struct fsm_state_id halt_state_id = fsm_register_state (&test_instance, 0, fsm_state_no_initialization);

	ASSERT_TRUE(fsm_add_transition (&test_instance, active_state_id, active_state_id, active_to_active_transition_test));
	ASSERT_TRUE(fsm_add_transition (&test_instance, active_state_id, halt_state_id, active_to_halt_transition_test));

	struct fsm_event evt{};
	fsm_event_init (&evt, MEM(tm), countdown_event_type, sizeof (struct countdown_event_payload));
	auto event_payload = (struct countdown_event_payload *)fsm_event_payload (evt);
	event_payload->decrement_amount = 1;

	ASSERT_TRUE(fsm_state_id_equal (active_state_id, fsm_current_state_id (&test_instance)));
	fsm_accept (&test_instance, evt);
	ASSERT_TRUE(fsm_state_id_equal (active_state_id, fsm_current_state_id (&test_instance)));
	fsm_accept (&test_instance, evt);
	ASSERT_TRUE(fsm_state_id_equal (halt_state_id, fsm_current_state_id (&test_instance)));

	fsm_reset (&test_instance);
	fsm_event_reset (&evt, MEM(tm));
	mem_test_verify (tm);
	mem_test_free (tm);
}