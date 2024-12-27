#include "../unit-test.h"
#include "core/fsm/fsm-state.h"

struct test_counting_state_payload {
  u8 counter;
};

struct test_decrement_event_payload {
  u8 value;
};

const struct fsm_state_id some_state_id = {.value = 23};
const size_t payload_size = sizeof (test_counting_state_payload);

static void
test_counting_state_apply_fn (struct fsm_state *self, struct fsm_event event)
{
	ASSERT_EQ(event.type.value, 45);

	auto state_payload = (struct test_counting_state_payload *)fsm_state_payload (self);
	auto event_payload = (struct test_decrement_event_payload *)fsm_event_payload (event);

	state_payload->counter -= event_payload->value;
}

static void
test_state_init (struct fsm_state *state)
{
	state->apply_fn = test_counting_state_apply_fn;
	((struct test_counting_state_payload *)fsm_state_payload (state))->counter = 100;
}

TEST(fsm_state_allocate, ok)
{
	auto tm = mem_test_new_default (128);

	auto result = fsm_state_allocate (MEM(tm), some_state_id, payload_size, test_state_init);
	ASSERT_TRUE(fsm_state_id_equal (some_state_id, result->id));
	ASSERT_EQ(8, result->payload.size);
	ASSERT_TRUE(result->apply_fn != nullptr);

	fsm_state_deallocate (result, MEM(tm));
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(fsm_state_allocate, fsm_state_no_initialization)
{
	auto tm = mem_test_new_default (128);

	auto result = fsm_state_allocate (MEM(tm), some_state_id, payload_size, fsm_state_no_initialization);
	ASSERT_TRUE(fsm_state_id_equal (some_state_id, result->id));
	ASSERT_EQ(8, result->payload.size);
	ASSERT_TRUE(result->apply_fn == nullptr);

	fsm_state_deallocate (result, MEM(tm));
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(fsm_state_apply, ok)
{
	auto tm = mem_test_new_default (128);
	auto test_instance = fsm_state_allocate (MEM(tm), some_state_id, payload_size, test_state_init);

	struct fsm_event decrement_event{};
	fsm_event_init (&decrement_event, MEM(tm), {.value = 45}, sizeof (test_decrement_event_payload));
	((struct test_decrement_event_payload *)fsm_event_payload (decrement_event))->value = 42;

	fsm_state_apply (test_instance, decrement_event);
	ASSERT_EQ(58, ((struct test_counting_state_payload *)fsm_state_payload (test_instance))->counter);

	fsm_state_apply (test_instance, decrement_event);
	ASSERT_EQ(16, ((struct test_counting_state_payload *)fsm_state_payload (test_instance))->counter);

	fsm_state_deallocate (test_instance, MEM(tm));
	fsm_event_reset (&decrement_event, MEM(tm));
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(fsm_state_apply, no_payload)
{
	auto tm = mem_test_new_default (128);
	auto test_instance = fsm_state_allocate (MEM(tm), some_state_id, 0, fsm_state_no_initialization);

	struct fsm_event decrement_event{};
	fsm_event_init_without_payload (&decrement_event, {.value = 45});

	fsm_state_apply (test_instance, decrement_event);
	fsm_state_apply (test_instance, decrement_event);

	fsm_state_deallocate (test_instance, MEM(tm));
	fsm_event_reset (&decrement_event, MEM(tm));
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(fsm_state_deallocate_safe, ok)
{
	auto tm = mem_test_new_default (128);
	auto test_instance = fsm_state_allocate (MEM(tm), some_state_id, payload_size, test_state_init);
	fsm_state_deallocate_safe (&test_instance, MEM(tm));

	ASSERT_TRUE(test_instance == nullptr);
	mem_test_verify (tm);
	mem_test_free (tm);
}