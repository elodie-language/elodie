#ifndef CORE_VAL_OBJ_H
#define CORE_VAL_OBJ_H

#include "val.h"
#include "val-str.h"
#include "val-prop.h"

#include "core/algo/algo-list-ptr.h"
#include "core/type/type.h"

struct val_lst;
struct val_prop;

struct val_obj {
  struct dep_val base;
  struct dep_val_str *ident;

  struct ptr_list props;    // FIXME objects of same type must have same props as well --> put this into a global place
  struct ptr_list values;
};

HAMAL_API struct val_obj *
val_obj_new (struct mem *mem, struct dep_val_str_view ident);

HAMAL_API void
val_obj_append (struct val_obj *self, struct val_fld *field, struct dep_val *value);

HAMAL_API struct dep_val *
val_obj_val_at (struct val_obj *self, size_t idx);

HAMAL_API struct dep_val *
val_obj_val_of_prop (struct val_obj *self, struct val_prop *prop);

HAMAL_API struct val_prop *
val_obj_prop_at (struct val_obj *self, size_t idx);

HAMAL_API struct iterator
val_obj_prop_iter (struct val_obj *self);


//HAMAL_API struct val_num *
//val_obj_get_num_at (struct val_obj *self, size_t idx);

//HAMAL_API struct val *
//val_obj_get(struct  val_obj *self, prop)

//HAMAL_API struct val_obj*
//val_obj_copy_compact(struct val_obj* self, struct mem* mem);

HAMAL_API u2
val_obj_next_prop_id (struct val_obj *self);

HAMAL_API size_t
val_obj_count (struct val_obj *self);

HAMAL_API void
val_obj_clear (struct val_obj *self);

HAMAL_API void
val_obj_free (struct val_obj *self);

HAMAL_API void
val_obj_free_safe (struct val_obj **self);

#endif //CORE_VAL_OBJ_H
