#ifndef CORE_JSON_H
#define CORE_JSON_H

#include "core/algo/algo-list-byte.h"
#include "macro.h"

struct json_key_view {
  char const *data;
  size_t count;
};

struct val_bool;
struct val_num;
struct dep_val_str;

typedef struct dep_val_str dep_string_t;
typedef struct dep_val_str_view dep_str_view_t;

HAMAL_API struct json_key_view
json_key_view_from_c_str (char const *str);

HAMAL_API struct json_key_view
json_key_view_from_str (struct dep_val_str *str);

HAMAL_API struct json_key_view
json_key_view_from_str_view (struct dep_val_str_view str);

#define JSON_KEY(T) _Generic((T),                 \
    char  *: json_key_view_from_c_str,       \
    char const *: json_key_view_from_c_str,       \
    dep_string_t* : json_key_view_from_str,         \
    dep_str_view_t: json_key_view_from_str_view    \
)(T)

enum json_node_type {
  JSON_NODE_TYPE_OBJ,
  JSON_NODE_TYPE_ARRAY,
  JSON_NODE_TYPE_NULL,
  JSON_NODE_TYPE_NUM,
  JSON_NODE_TYPE_str,
  JSON_NODE_TYPE_BOOL
};

enum json_writer_status {
  JSON_WRITER_STATUS_OK,
  JSON_WRITER_STATUS_STACK_FULL,
  JSON_WRITER_STATUS_STACK_EMPTY,
  //not every obj/ array was closed properly
  JSON_WRITER_STATUS_NESTING_ERROR
};

struct json_writer_node_stack {
  enum json_node_type type;
  size_t element_count;
};

struct json_writer {
  struct mem *mem;
  struct byte_list buffer;
  enum json_writer_status status;
  size_t stack_idx;
  struct json_writer_node_stack stack[256];
  /**
   * vape mem used to convert vals to str without having to worry about mem management
   */
  struct mem_vape *vape_mem;
};

HAMAL_API void
json_writer_init (struct json_writer *self, struct mem *mem);

HAMAL_API enum json_writer_status
json_writer_obj_start (struct json_writer *self);

HAMAL_API enum json_writer_status
json_writer_obj_start_obj (struct json_writer *self, struct json_key_view key);

HAMAL_API enum json_writer_status
json_writer_obj_start_array (struct json_writer *self, struct json_key_view key);

HAMAL_API enum json_writer_status
json_writer_obj_null (struct json_writer *self, struct json_key_view key);

HAMAL_API enum json_writer_status
json_writer_obj_c_str (struct json_writer *self, struct json_key_view key, char const *str);

HAMAL_API enum json_writer_status
json_writer_obj_str_view (struct json_writer *self, struct json_key_view key, struct dep_val_str_view view);

HAMAL_API enum json_writer_status
json_writer_obj_str (struct json_writer *self, struct json_key_view key, struct dep_val_str *val);

HAMAL_API enum json_writer_status
json_writer_obj_num (struct json_writer *self, struct json_key_view key, struct val_num *val);

HAMAL_API enum json_writer_status
json_writer_obj_bool (struct json_writer *self, struct json_key_view key, struct val_bool *val);

HAMAL_API enum json_writer_status
json_writer_obj_val (struct json_writer *self, struct json_key_view key, struct dep_val *val);

HAMAL_API enum json_writer_status
json_writer_obj_end (struct json_writer *self);

HAMAL_API enum json_writer_status
json_writer_array_start (struct json_writer *self);

HAMAL_API enum json_writer_status
json_writer_array_end (struct json_writer *self);

HAMAL_API enum json_writer_status
json_writer_array_null (struct json_writer *self);

HAMAL_API enum json_writer_status
json_writer_array_c_str (struct json_writer *self, char const *str);

HAMAL_API enum json_writer_status
json_writer_array_str_view (struct json_writer *self, struct dep_val_str_view view);

HAMAL_API enum json_writer_status
json_writer_array_str (struct json_writer *self, struct dep_val_str *val);

HAMAL_API enum json_writer_status
json_writer_array_num (struct json_writer *self, struct val_num *val);

HAMAL_API enum json_writer_status
json_writer_array_bool (struct json_writer *self, struct val_bool *val);

HAMAL_API enum json_writer_status
json_writer_array_val (struct json_writer *self, struct dep_val *val);

HAMAL_API enum json_writer_status
json_writer_to_str_view (struct json_writer *self, struct dep_val_str_view *out);

HAMAL_API void
json_writer_reset (struct json_writer *self);

#endif //CORE_JSON_H
