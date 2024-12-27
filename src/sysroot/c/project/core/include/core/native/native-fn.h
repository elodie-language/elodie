#ifndef CORE_NATIVE_H
#define CORE_NATIVE_H

#include "native-context.h"
#include "native-fn-sig.h"

enum native_fn_result_kind {
  NATIVE_FN_RESULT_KIND_OK = 0x01
};

struct native_fn_result {
  enum native_fn_result_kind kind;
};

typedef struct native_fn_result (*native_fn_ptr) (native_context_t *ctx);

struct native_fn {
  struct native_fn_signature sig;
  native_fn_ptr fn_ptr;
};

#endif //CORE_NATIVE_H
