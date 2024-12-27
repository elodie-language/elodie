#include "core/check.h"
#include "core/fsm/fsm.h"

typedef struct fsm f;
typedef struct mem m;
typedef struct fsm_state s;
typedef struct fsm_state_id si;
typedef struct fsm_event e;
typedef struct fsm_event_type et;
typedef struct fsm_transition t;
typedef struct ptr_map map;
typedef struct ptr_map_key pmk;

static struct list *
get_transitions (f *self, pmk key)
{
	struct list *result;
	CHECK_TRUE(ptr_map_get (&self->transitions, key, (void **)&result));
	return result;
}

static void
reset_states (f *self)
{
	struct iterator state_iterator = ptr_list_iterator (&self->states);
	while (iterator_has_next (&state_iterator))
		{
			struct fsm_state *state = iterator_next (&state_iterator);
			fsm_state_deallocate_safe (&state, self->mem);
		}
	ptr_list_reset (&self->states);
}

static pmk
transitions_key (f *self, si origin)
{
	return ptr_map_key_from_size_t (&self->transitions, origin.value);
}

static struct iterator
get_transition_iterator (f *self, pmk key)
{
	return list_iterator (get_transitions (self, key));
}

static void
reset_transitions (f *self)
{
	struct iterator key_iterator = ptr_map_keys_iterator (&self->transitions);
	while (iterator_has_next (&key_iterator))
		{
			pmk key = *(pmk *)iterator_next (&key_iterator);
			struct list *transitions = get_transitions (self, key);
			list_free_safe (&transitions);
		}
	ptr_map_reset (&self->transitions);
}

static void
apply_event_to_current_state (f *self, e event)
{
	if (self->current_state->apply_fn != NULL)
		{
			self->current_state->apply_fn (self->current_state, event);
		}
}

static struct list *
create_new_transition_list (f *self)
{
	struct list_config config = {
		.mem = self->mem,
		.initial_capacity = 2,
		.resize_factor = 2
	};
	return list_new (config, struct fsm_transition);
}

static bool
transition_exists (f *self, si origin, si destination)
{
	pmk key = transitions_key (self, origin);
	struct list *transitions = get_transitions (self, key);
	struct iterator iterator = list_iterator (transitions);
	while (iterator_has_next (&iterator))
		{
			t *current = iterator_next (&iterator);
			if (fsm_state_id_equal (destination, current->destination))
				{
					return true;
				}
		}

	return false;
}

void
fsm_init (f *self, m *mem)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(mem);
	self->mem = mem;
	self->current_state = NULL;
	ptr_list_init (&self->states, (struct ptr_list_config){
		.mem = mem,
		.initial_capacity = 2
	});
	ptr_map_init (&self->transitions, (struct ptr_map_config){
		.mem = mem,
		.initial_capacity = 8,
		.key_hash_fn = hash_fn_murmur_3 (0)
	});
}

s *
fsm_current_state (f *self)
{
	CHECK_NOT_NULL(self);
	return self->current_state;
}

si
fsm_current_state_id (f *self)
{
	CHECK_NOT_NULL(self);
	return fsm_current_state (self)->id;
}

s *
fsm_get_state (f *self, si state_id)
{
	CHECK_NOT_NULL(self);
	CHECK_GREATER_THAN(state_id.value, 0);
	CHECK_LESS_THAN_EQUAL(state_id.value, ptr_list_count (&self->states));
	return ptr_list_at (&self->states, state_id.value - 1);
}

void
fsm_accept (f *self, e event)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(self->current_state);
	apply_event_to_current_state (self, event);

	pmk key = transitions_key (self, self->current_state->id);
	struct iterator it = get_transition_iterator (self, key);
	while (it.has_next (&it))
		{
			t *transition = iterator_next (&it);
			if (fsm_transition_test (*transition, self, event))
				{
					LOG_DEBUG("State[%d]--[transition]->State[%d]", transition->origin, transition->destination);
					self->current_state = fsm_get_state (self, transition->destination);
					break;
				}
		}
}

si
fsm_register_state (f *self, u4 payload_size, void (*init_state_fn) (s *self))
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(init_state_fn);
	si new_state_id = {.value = ptr_list_count (&self->states) + 1};

	s *new_state = fsm_state_allocate (self->mem, new_state_id, payload_size, init_state_fn);
	if (self->current_state == NULL)
		{
			self->current_state = new_state;
		}

	ptr_list_append (&self->states, new_state);
	return new_state_id;
}

bool
fsm_add_transition (f *self, si origin, si destination, fsm_transition_test_fn transition_test_fn)
{
	CHECK_NOT_NULL(self);
	pmk key = ptr_map_key_from_size_t (&self->transitions, origin.value);

	if (!ptr_map_has_key (&self->transitions, key))
		{
			ptr_map_set (&self->transitions, key, create_new_transition_list (self));
		}

	/**
	 * Makes sure that there is only one transition possible from an origin state to a destination state.
	 */
	CHECK_FALSE(transition_exists (self, origin, destination));

	struct list *transitions_from_origin = get_transitions (self, key);
	t new_transition = (t){
		.origin = origin,
		.destination = destination,
		.transition_test_fn = transition_test_fn
	};
	list_append_rval(transitions_from_origin, &new_transition);

	return true;
}

void
fsm_reset (f *self)
{
	CHECK_NOT_NULL(self);
	reset_states (self);
	reset_transitions (self);
}
