#ifndef CORE_NATIVE_REGISTRY_H
#define CORE_NATIVE_REGISTRY_H

#include "core/native/native-api.h"

struct native_registry {
  struct native_fn fbs[10];
  u1 idx;
};

ELODIE_API void
native_registry_init (struct native_registry *self, struct mem *mem);

ELODIE_API bool
native_registry_register_fn (struct native_registry *self, struct native_fn native_fn);

ELODIE_API bool
native_registry_resolve_fn (struct native_registry *self, struct native_fn_signature signature, struct native_fn *out);

ELODIE_API void
native_registry_reset (struct native_registry *self, struct mem *mem);

#endif //CORE_NATIVE_REGISTRY_H
