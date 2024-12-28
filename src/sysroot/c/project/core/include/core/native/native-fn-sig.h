#ifndef CORE_NATIVE_fn_signature_H
#define CORE_NATIVE_fn_signature_H

#include "core/algo/algo-hash.h"
#include "core/string/string-api.h"
#include "core/type/type-api.h"

// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-[native_fn_signature_ident]+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-
struct native_fn_signature_ident {
  /**
   * ident contains both package and fn identifier in one string
   */
  struct string ident;
  /**
   * marks the position of where the function ident part starts
   * e.g. some_package::fn - position = 13 (::)
   */
  u2 marker;
};

ELODIE_API struct native_fn_signature_ident *
native_fn_signature_ident_allocate (struct mem *mem, struct string_view ident);

ELODIE_API void
native_fn_signature_ident_init (struct native_fn_signature_ident *self, struct mem *mem, struct string_view ident);

ELODIE_API struct string_view
native_fn_signature_ident_package_ident (struct native_fn_signature_ident self);

ELODIE_API struct string_view
native_fn_signature_ident_fn_ident (struct native_fn_signature_ident self);

ELODIE_API struct string_view
native_fn_signature_ident (struct native_fn_signature_ident self);

ELODIE_API void
native_fn_signature_ident_reset (struct native_fn_signature_ident *self, struct mem *mem);

ELODIE_API void
native_fn_signature_ident_deallocate (struct native_fn_signature_ident *self, struct mem *mem);

ELODIE_API void
native_fn_signature_ident_deallocate_safe (struct native_fn_signature_ident **self, struct mem *mem);

// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-[native_fn_signature_param]+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-

struct native_fn_signature_param {
  struct string ident;
  struct type type;
  struct native_fn_signature_param *next;
};

ELODIE_API struct native_fn_signature_param *
native_fn_signature_param_allocate (struct mem *mem, struct string_view ident, struct type type);

ELODIE_API void
native_fn_signature_param_init (struct native_fn_signature_param *self, struct mem *mem, struct string_view ident, struct type type);

ELODIE_API void
native_fn_signature_param_append (struct native_fn_signature_param *self, struct native_fn_signature_param *other);

ELODIE_API u1
native_fn_signature_param_count (struct native_fn_signature_param const *self);

ELODIE_API void
native_fn_signature_param_reset (struct native_fn_signature_param *self, struct mem *mem);

ELODIE_API void
native_fn_signature_param_deallocate (struct native_fn_signature_param *self, struct mem *mem);

ELODIE_API void
native_fn_signature_param_deallocate_safe (struct native_fn_signature_param **self, struct mem *mem);

// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-[native_fn_signature_result]+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-

struct native_fn_signature_result {
  struct type type;
};

ELODIE_API struct native_fn_signature_result *
native_fn_signature_result_allocate (struct mem *mem, struct type type);

ELODIE_API void
native_fn_signature_result_init (struct native_fn_signature_result *self, struct mem *mem, struct type type);

ELODIE_API void
native_fn_signature_result_reset (struct native_fn_signature_result *self, struct mem *mem);

ELODIE_API void
native_fn_signature_result_deallocate (struct native_fn_signature_result *self, struct mem *mem);

ELODIE_API void
native_fn_signature_result_deallocate_safe (struct native_fn_signature_result **self, struct mem *mem);

// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-[native_fn_sig]+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-

struct native_fn_signature {
  struct native_fn_signature_ident ident;
  struct native_fn_signature_param *params;
  struct native_fn_signature_result result;
  struct hash8 hash;
};

ELODIE_API struct native_fn_signature *
native_fn_signature_allocate (struct mem *mem, struct native_fn_signature_ident ident, struct native_fn_signature_param *param,
                              struct native_fn_signature_result result);

ELODIE_API void
native_fn_signature_init (struct native_fn_signature *self, struct mem *mem, struct native_fn_signature_ident ident,
                          struct native_fn_signature_param *params, struct native_fn_signature_result result);

ELODIE_API void
native_fn_signature_reset (struct native_fn_signature *self, struct mem *mem);

ELODIE_API void
native_fn_signature_deallocate (struct native_fn_signature *self, struct mem *mem);

ELODIE_API void
native_fn_signature_deallocate_safe (struct native_fn_signature **self, struct mem *mem);

#endif //CORE_NATIVE_fn_signature_H
