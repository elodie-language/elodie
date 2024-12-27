#ifndef CORE_NATIVE_REGISTRY_H
#define CORE_NATIVE_REGISTRY_H

#include "core/native/native-api.h"

struct native_registry {
  struct native_fn fbs[10];
  u1 idx;
};

HAMAL_API void
native_registry_init (struct native_registry *self, struct mem *mem);

HAMAL_API bool
native_registry_register_fn (struct native_registry *self, struct native_fn native_fn);

HAMAL_API bool
native_registry_resolve_fn (struct native_registry *self, struct native_fn_signature signature, struct native_fn *out);

HAMAL_API void
native_registry_reset (struct native_registry *self, struct mem *mem);

#endif //CORE_NATIVE_REGISTRY_H
