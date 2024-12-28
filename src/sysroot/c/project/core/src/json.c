#include <string.h>
#include "core/check.h"
#include "core/json.h"
#include "core/mem/mem-vape.h"
#include "core/val/val-api.h"

struct json_key_view
json_key_view_from_c_str (char const *str)
{
	CHECK_NOT_NULL(str);
	return (struct json_key_view){
		.data = str,
		.count = strlen (str)
	};
}

struct json_key_view
json_key_view_from_str (struct val_str *str)
{
	return (struct json_key_view){
		.data = str->data,
		.count = str->count
	};
}

struct json_key_view
json_key_view_from_str_view (struct val_str_view str)
{
	return (struct json_key_view){
		.data = str.data,
		.count = str.count
	};
}

static bool
json_writer_first_element_on_stack (struct json_writer *self)
{
	CHECK_NOT_NULL(self);
	return self->stack[self->stack_idx].element_count == 0;
}

static void
json_writer_increment_element_count (struct json_writer *self)
{
	CHECK_NOT_NULL(self);
	self->stack[self->stack_idx].element_count++;
}

static void
json_writer_prepare_new_element (struct json_writer *self)
{
	CHECK_NOT_NULL(self);

	if (!json_writer_first_element_on_stack (self))
		{
			byte_list_append_c_str (&self->buffer, ",");
		}

	json_writer_increment_element_count (self);
}

static void
json_writer_write_key (struct json_writer *self, struct json_key_view key)
{
	CHECK_NOT_NULL(self);
	byte_list_append_c_str (&self->buffer, "\"");
	byte_list_append_c_str (&self->buffer, key.data);
	byte_list_append_c_str (&self->buffer, "\"");
}

static void
json_writer_write_c_str (struct json_writer *self, char const *str)
{
	CHECK_NOT_NULL(self);
	byte_list_append_c_str (&self->buffer, "\"");
	byte_list_append_c_str (&self->buffer, str);
	byte_list_append_c_str (&self->buffer, "\"");
}

static void
json_writer_write_str (struct json_writer *self, struct val_str *str)
{
	CHECK_NOT_NULL(self);
	byte_list_append_c_str (&self->buffer, "\"");
	byte_list_append_str (&self->buffer, str);
	byte_list_append_c_str (&self->buffer, "\"");
}

static void
json_writer_write_colon (struct json_writer *self)
{
	CHECK_NOT_NULL(self);
	byte_list_append_c_str (&self->buffer, ":");
}

static void
json_writer_ensure_node_type (struct json_writer *self, enum json_node_type node_type)
{
	CHECK_NOT_NULL(self);
	if (self->stack[self->stack_idx].type != node_type)
		{
			self->status = JSON_WRITER_STATUS_NESTING_ERROR;
		}
}

static bool
json_writer_status_ok (struct json_writer *self)
{
	CHECK_NOT_NULL(self);
	return self->status == JSON_WRITER_STATUS_OK;
}

static void
json_writer_obj_key_colon (struct json_writer *self, struct json_key_view key)
{
	CHECK_NOT_NULL(self);

	json_writer_prepare_new_element (self);
	json_writer_ensure_node_type (self, JSON_NODE_TYPE_OBJ);
	if (json_writer_status_ok (self))
		{
			json_writer_write_key (self, key);
			json_writer_write_colon (self);
		}
}

static void
json_writer_push_stack (struct json_writer *self, enum json_node_type node_type)
{
	CHECK_NOT_NULL(self);
	CHECK_TRUE(node_type == JSON_NODE_TYPE_ARRAY || node_type == JSON_NODE_TYPE_OBJ);
	if (self->stack_idx + 1 >= 256)
		{
			self->status = JSON_WRITER_STATUS_STACK_FULL;
		}
	else
		{
			self->stack[++self->stack_idx] = (struct json_writer_node_stack){
				.type = node_type,
				.element_count = 0
			};
		}
}

static void
json_writer_pop_stack (struct json_writer *self, enum json_node_type node_type)
{
	CHECK_NOT_NULL(self);
	CHECK_TRUE(node_type == JSON_NODE_TYPE_ARRAY || node_type == JSON_NODE_TYPE_OBJ);
	if (self->stack_idx == 0)
		{
			self->status = JSON_WRITER_STATUS_STACK_EMPTY;
		}
	else
		{
			if (self->stack[self->stack_idx].type != node_type)
				{
					self->status = JSON_WRITER_STATUS_NESTING_ERROR;
				}
			else
				{
					self->stack_idx--;
				}
		}
}

void
json_writer_init (struct json_writer *self, struct mem *mem)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(mem);
	self->mem = mem;

	struct byte_list_config config = (struct byte_list_config){
		.mem = mem,
		.initial_capacity = 64
	};
	byte_list_init (&self->buffer, config);

	self->vape_mem = mem_vape_new ((struct mem_vape_config){
		.size = 64,
		.root = mem
	});
}

enum json_writer_status
json_writer_obj_start (struct json_writer *self)
{
	CHECK_NOT_NULL(self);
	json_writer_prepare_new_element (self);
	json_writer_push_stack (self, JSON_NODE_TYPE_OBJ);
	byte_list_append_c_str (&self->buffer, "{");
	return self->status;
}

enum json_writer_status
json_writer_obj_start_obj (struct json_writer *self, struct json_key_view key)
{
	CHECK_NOT_NULL(self);
	json_writer_prepare_new_element (self);
	json_writer_push_stack (self, JSON_NODE_TYPE_OBJ);
	json_writer_write_key (self, key);
	json_writer_write_colon (self);
	byte_list_append_c_str (&self->buffer, "{");
	return self->status;
}

enum json_writer_status
json_writer_obj_start_array (struct json_writer *self, struct json_key_view key)
{
	CHECK_NOT_NULL(self);
	json_writer_prepare_new_element (self);
	json_writer_push_stack (self, JSON_NODE_TYPE_ARRAY);
	json_writer_write_key (self, key);
	json_writer_write_colon (self);
	byte_list_append_c_str (&self->buffer, "[");
	return self->status;
}

enum json_writer_status
json_writer_obj_c_str (struct json_writer *self, struct json_key_view key, char const *str)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(str);
	json_writer_obj_key_colon (self, key);
	json_writer_write_c_str (self, str);
	return self->status;
}

enum json_writer_status
json_writer_obj_str_view (struct json_writer *self, struct json_key_view key, struct val_str_view view)
{
	CHECK_NOT_NULL(self);
	json_writer_obj_key_colon (self, key);
	json_writer_write_c_str (self, view.data);
	return self->status;
}

enum json_writer_status
json_writer_obj_null (struct json_writer *self, struct json_key_view key)
{
	CHECK_NOT_NULL(self);
	json_writer_obj_key_colon (self, key);
	byte_list_append_c_str (&self->buffer, "null");
	return self->status;
}

enum json_writer_status
json_writer_obj_str (struct json_writer *self, struct json_key_view key, struct val_str *val)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(val);
	json_writer_obj_key_colon (self, key);
	json_writer_write_str (self, val);
	return self->status;
}

enum json_writer_status
json_writer_obj_num (struct json_writer *self, struct json_key_view key, struct val_num *val)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(val);
	json_writer_obj_key_colon (self, key);

	byte_list_append_str (&self->buffer, val_num_to_str (val, (struct mem *)self->vape_mem));
	return self->status;
}

enum json_writer_status
json_writer_obj_bool (struct json_writer *self, struct json_key_view key, struct val_bool *val)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(val);
	json_writer_obj_key_colon (self, key);
	byte_list_append_str (&self->buffer, val_bool_to_str (val, (struct mem *)self->vape_mem));
	return self->status;
}

enum json_writer_status
json_writer_obj_val (struct json_writer *self, struct json_key_view key, struct val *val)
{
	CHECK_NOT_NULL(self);
	json_writer_obj_key_colon (self, key);
	if (val->kind == VAL_KIND_STR)
		{
			json_writer_write_str (self, (struct val_str *)val);
		}
	else if (val->kind == VAL_KIND_UNIT)
		{
			json_writer_write_str (self, val_to_str (val, (struct mem *)self->vape_mem));
		}
	else
		{
			byte_list_append_str (&self->buffer, val_to_str (val, (struct mem *)self->vape_mem));
		}
	return self->status;
}

enum json_writer_status
json_writer_obj_end (struct json_writer *self)
{
	CHECK_NOT_NULL(self);
	json_writer_pop_stack (self, JSON_NODE_TYPE_OBJ);
	byte_list_append_c_str (&self->buffer, "}");
	return self->status;
}

enum json_writer_status
json_writer_array_start (struct json_writer *self)
{
	CHECK_NOT_NULL(self);
	json_writer_prepare_new_element (self);
	json_writer_push_stack (self, JSON_NODE_TYPE_ARRAY);
	byte_list_append_c_str (&self->buffer, "[");
	return self->status;
}

enum json_writer_status
json_writer_array_c_str (struct json_writer *self, char const *str)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(str);
	json_writer_prepare_new_element (self);
	json_writer_ensure_node_type (self, JSON_NODE_TYPE_ARRAY);
	json_writer_write_c_str (self, str);
	return self->status;
}

enum json_writer_status
json_writer_array_null (struct json_writer *self)
{
	CHECK_NOT_NULL(self);
	json_writer_prepare_new_element (self);
	json_writer_ensure_node_type (self, JSON_NODE_TYPE_ARRAY);
	byte_list_append_c_str (&self->buffer, "null");
	return self->status;
}

enum json_writer_status
json_writer_array_str (struct json_writer *self, struct val_str *val)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(val);
	json_writer_prepare_new_element (self);
	json_writer_ensure_node_type (self, JSON_NODE_TYPE_ARRAY);
	json_writer_write_str (self, val);
	return self->status;
}

enum json_writer_status
json_writer_array_str_view (struct json_writer *self, struct val_str_view view)
{
	CHECK_NOT_NULL(self);
	json_writer_prepare_new_element (self);
	json_writer_ensure_node_type (self, JSON_NODE_TYPE_ARRAY);
	json_writer_write_c_str (self, view.data);
	return self->status;
}

enum json_writer_status
json_writer_array_num (struct json_writer *self, struct val_num *val)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(val);
	json_writer_prepare_new_element (self);
	json_writer_ensure_node_type (self, JSON_NODE_TYPE_ARRAY);

	struct val_str *str = val_num_to_str (val, (struct mem *)self->vape_mem);
	byte_list_append_str (&self->buffer, str);
	val_str_deallocate_safe (&str);

	return self->status;
}

enum json_writer_status
json_writer_array_bool (struct json_writer *self, struct val_bool *val)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(val);
	json_writer_prepare_new_element (self);
	json_writer_ensure_node_type (self, JSON_NODE_TYPE_ARRAY);
	byte_list_append_str (&self->buffer, val_bool_to_str (val, (struct mem *)self->vape_mem));
	return self->status;
}

enum json_writer_status
json_writer_array_val (struct json_writer *self, struct val *val)
{
	CHECK_NOT_NULL(self);
	json_writer_prepare_new_element (self);
	json_writer_ensure_node_type (self, JSON_NODE_TYPE_ARRAY);
	if (val->kind == VAL_KIND_STR)
		{
			json_writer_write_str (self, val_to_str (val, (struct mem *)self->vape_mem));
		}
	else
		{
			byte_list_append_str (&self->buffer, val_to_str (val, (struct mem *)self->vape_mem));
		}
	return self->status;
}

enum json_writer_status
json_writer_array_end (struct json_writer *self)
{
	CHECK_NOT_NULL(self);
	json_writer_pop_stack (self, JSON_NODE_TYPE_ARRAY);
	byte_list_append_c_str (&self->buffer, "]");
	return self->status;
}

enum json_writer_status
json_writer_to_str_view (struct json_writer *self, struct val_str_view *out)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(out);
	if (!json_writer_status_ok (self))
		{
			return self->status;
		}

	if (self->stack_idx != 0)
		{
			return JSON_WRITER_STATUS_NESTING_ERROR;
		}

	byte_list_at_str_view (&self->buffer, 0, byte_list_size (&self->buffer), out);
	return JSON_WRITER_STATUS_OK;
}

void
json_writer_reset (struct json_writer *self)
{
	CHECK_NOT_NULL(self);
	byte_list_reset (&self->buffer);
	mem_vape_free_safe (&self->vape_mem);
}
