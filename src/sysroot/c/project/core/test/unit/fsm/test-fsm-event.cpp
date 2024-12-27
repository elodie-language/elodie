#include "../unit-test.h"
#include "core/fsm/fsm-event.h"

struct payload {
  u8 value;
};

TEST(fsm_event_has_type, true)
{
	struct fsm_event_type some_event_type = {.value = 23};
	struct fsm_event_type another_event_type = {.value = 45};

	struct fsm_event some_event{};
	fsm_event_init_without_payload (&some_event, some_event_type);

	ASSERT_TRUE(fsm_event_has_type (some_event, some_event_type));
	ASSERT_FALSE(fsm_event_has_type (some_event, another_event_type));
}

TEST(fsm_event, ok)
{
	auto tm = mem_test_new_default (128);
	struct fsm_event test_instance{};
	fsm_event_init (&test_instance, MEM(tm), {.value = 2}, sizeof (struct payload));

	ASSERT_EQ(2, test_instance.type.value);
	ASSERT_EQ(8, test_instance.payload.size);
	ASSERT_EQ(0, *(u8 *)(void *)test_instance.payload.data);

	auto payload = (struct payload *)fsm_event_payload (test_instance);
	payload->value = 2810;

	auto result = (struct payload *)fsm_event_payload (test_instance);
	ASSERT_EQ(2810, result->value);

	fsm_event_reset (&test_instance, MEM(tm));
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(fsm_event_init_without_payload, ok)
{
	auto tm = mem_test_new_default (128);
	struct fsm_event test_instance{};
	fsm_event_init_without_payload (&test_instance, {.value = 4});

	ASSERT_EQ(4, test_instance.type.value);
	ASSERT_EQ(0, test_instance.payload.size);
	ASSERT_TRUE(test_instance.payload.data == nullptr);

	fsm_event_reset (&test_instance, MEM(tm));
	mem_test_verify (tm);
	mem_test_free (tm);
}


