#ifndef CORE_VAL_SERIALIZER_H
#define CORE_VAL_SERIALIZER_H

#include "core/macro.h"
#include "core/json.h"

struct val_obj;
struct dep_val_str;

struct val_writer {
  struct json_writer writer;
  struct mem_vape *vape_mem;
};

HAMAL_API void
val_writer_write (struct val_writer *self, struct dep_val *val);

HAMAL_API void
val_writer_write_obj (struct val_writer *self, struct val_obj *obj);

HAMAL_API void
val_writer_print (struct val_writer *self);

#endif //CORE_VAL_SERIALIZER_H
