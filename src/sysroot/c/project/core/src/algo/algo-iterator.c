#include "core/check.h"
#include "core/algo/algo-iterator.h"

typedef struct iterator it;

struct iterator
iterator_index (void *target, bool (*has_next) (struct iterator *), void *(*next) (struct iterator *))
{
	CHECK_NOT_NULL(target);
	CHECK_NOT_NULL(has_next);
	CHECK_NOT_NULL(next);

	return (struct iterator){
		.target = target,
		.current.index = 0,
		.has_next = has_next,
		.next = next
	};
}

struct iterator
iterator (void *target, bool (*has_next) (struct iterator *), void *(*next) (struct iterator *), void *start)
{
	CHECK_NOT_NULL(target);
	CHECK_NOT_NULL(has_next);
	CHECK_NOT_NULL(next);

	return (struct iterator){
		.target = target,
		.current.ptr = start,
		.has_next = has_next,
		.next = next
	};
}

bool
iterator_has_next (struct iterator *self)
{
	CHECK_NOT_NULL(self);
	return self->has_next (self);
}

void *
iterator_next (struct iterator *self)
{
	CHECK_NOT_NULL(self);
	return self->next (self);
}
