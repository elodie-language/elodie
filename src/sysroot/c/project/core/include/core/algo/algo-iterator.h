#ifndef CORE_ALGO_ITERATOR_H
#define CORE_ALGO_ITERATOR_H

#include "core/core.h"
#include "core/macro.h"

struct iterator {
  bool (*has_next) (struct iterator *);
  void *(*next) (struct iterator *);
  void *target;
  union {
	void *ptr;
	size_t index;
  } current;
};

HAMAL_API struct iterator
iterator_index (void *target, bool (*has_next) (struct iterator *), void *(*next) (struct iterator *));

HAMAL_API struct iterator
iterator (void *target, bool (*has_next) (struct iterator *), void *(*next) (struct iterator *), void *start);

HAMAL_API bool
iterator_has_next (struct iterator *self);

HAMAL_API void *
iterator_next (struct iterator *self);

#endif //CORE_ALGO_ITERATOR_H
