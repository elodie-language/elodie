#ifndef CORE_VAL_REF_H
#define CORE_VAL_REF_H

#include "core/core.h"

struct val_ref {
  u4 kind: 8;
  u4 realm: 8;
  u4 flags: 8;
  u4 value;
};

#endif //CORE_VAL_REF_H
