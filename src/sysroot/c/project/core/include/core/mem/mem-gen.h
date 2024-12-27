#ifndef CORE_MEM_GEN_H
#define CORE_MEM_GEN_H

#include "mem.h"
#include "core/val/val-api.h"
#include "core/val/val-ref.h"
#include "core/algo/algo-list-ptr.h"

struct mem_gen_config {
  size_t size;
  struct mem *root;
};

struct mem_gen {
  struct mem base;
//  struct list *colors;

  struct ptr_list vals;
};

HAMAL_API struct mem_gen *
mem_gen_new (struct mem_gen_config config);

HAMAL_API void
mem_gen_init (struct mem_gen *self, struct mem_gen_config config);

// allocates a new val and returns the reference
// e.g. to copy over a stack value, a value from mem_stack / mem_volatile / mem_static
HAMAL_API struct val_ref
mem_gen_allocate (struct mem_gen *self, struct dep_val *val);

//HAMAL_API struct val_ref
//mem_gen_allocate_from_mem (struct mem_val *self, struct mem_val *other, struct val_ref ref);

HAMAL_API struct val_ref
val_str_ng_new (struct mem_gen *self, char const *c_str);

// returns a pointer to the underlying data
HAMAL_API struct dep_val *
mem_gen_resolve (struct mem_gen *self, struct val_ref ref);

//HAMAL_API struct val_ref
//mem_gen_allocate_str_from_c_str (struct mem_val *self, struct val *v);

//HAMAL_API void
//mem_gen_mark_val (struct mem_val *self, struct val *val, enum mem_gen_color color);
//
//HAMAL_API void
//mem_gen_mark (struct mem_val *self, struct val *val);
//
//HAMAL_API void
//mem_gen_sweep (struct mem_val *self);
//
HAMAL_API size_t
mem_gen_count (struct mem_gen *self);

HAMAL_API void
mem_gen_free (struct mem_gen *self);

HAMAL_API void
mem_gen_free_safe (struct mem_gen **self);

#endif //CORE_MEM_GEN_H
