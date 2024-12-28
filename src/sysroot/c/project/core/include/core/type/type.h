#ifndef CORE_TYPE_H
#define CORE_TYPE_H

#include "core/macro.h"

#include "core/val/val-str-view.h"

struct type {
  u2 id;
};

enum type_id {
  TYPE_ID_ANY = 0,
  TYPE_ID_NIL = 1,
  TYPE_ID_OBJECT = 2,
  TYPE_ID_NUMBER = 3,
  TYPE_ID_STRING = 4,
  TYPE_ID_UNIT = 5
};

ELODIE_API  bool
type_equal (struct type lhs, struct type rhs);

extern const struct type type_any;
extern const struct type type_nil;
extern const struct type type_object;
extern const struct type type_number;
extern const struct type type_string;
extern const struct type type_unit;

struct type_info {
  u2 id;
  u2 base_id;
  struct val_str_view ident;
};

#endif //CORE_TYPE_H
