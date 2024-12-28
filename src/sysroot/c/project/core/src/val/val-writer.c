#include "core/check.h"
#include "core/val/val-api.h"
#include "core/val/val-writer.h"
#include "core/type/type-api.h"

void
val_writer_write (struct val_writer *self, struct val *val)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(val);
	switch (val->kind)
		{
			case VAL_KIND_OBJ:
				{
					json_writer_obj_start (&self->writer);
					val_writer_write_obj (self, AS_OBJ(val));
					json_writer_obj_end (&self->writer);
					break;
				}
			default: NOT_IMPLEMENTED_YET();
		}
}

void
val_writer_write_obj (struct val_writer *self, struct val_obj *obj)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(obj);

	struct iterator it = val_obj_prop_iter (obj);
	while (iterator_has_next (&it))
		{
			struct val_prop *prop = iterator_next (&it);
			if (prop->of != obj)
				{
					continue;
				}
			struct val *val = val_obj_val_of_prop (obj, prop);
			switch (val->kind)
				{
					case VAL_KIND_STR:
						{
							CHECK_EQUAL(JSON_WRITER_STATUS_OK, json_writer_obj_str (&self->writer, JSON_KEY (prop->field->ident), AS_STR (val)));
							break;
						}
					case VAL_KIND_OBJ:
						{
							CHECK_EQUAL(JSON_WRITER_STATUS_OK, json_writer_obj_start_obj (&self->writer, JSON_KEY (prop->field->ident)));
							val_writer_write_obj (self, AS_OBJ(val));
							CHECK_EQUAL(JSON_WRITER_STATUS_OK, json_writer_obj_end (&self->writer));
							break;
						}
					default: NOT_IMPLEMENTED_YET();
				}
		}
}

void
val_writer_print (struct val_writer *self)
{
	CHECK_NOT_NULL(self);
	struct val_str_view out = {0};
	json_writer_to_str_view (&self->writer, &out);
	printf ("%.*s", (int)out.count, out.data);
}
