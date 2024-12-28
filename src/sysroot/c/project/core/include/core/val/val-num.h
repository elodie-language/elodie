#ifndef CORE_VAL_NUM_H
#define CORE_VAL_NUM_H

#include "val.h"

struct val_num {
  struct val base;
  double data;
};

ELODIE_API struct val_num *
val_num_new_from_double (struct mem *mem, double val);

ELODIE_API struct val_num *
val_num_copy (struct val_num *self, struct mem *mem);

ELODIE_API bool
val_numb_equal (struct val_num *lhs, struct val_num *rhs);

ELODIE_API struct val_str *
val_num_to_str (struct val_num *self, struct mem *mem);

ELODIE_API void
val_num_free (struct val_num *self);

ELODIE_API void
val_num_free_safe (struct val_num **self);

#endif //CORE_VAL_NUM_H
