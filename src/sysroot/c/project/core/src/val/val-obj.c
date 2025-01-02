#include "core/check.h"
#include "core/val/val-api.h"

typedef struct val_obj o;
typedef struct val_fld f;
typedef struct val v;
typedef struct val_prop p;

static struct ptr_list_config
default_config(struct mem *mem) {
    CHECK_NOT_NULL(mem);
    return (struct ptr_list_config) {
            .initial_capacity = 1,
            .mem = mem,
    };
}

o *
val_obj_new(struct mem *mem, struct val_str_view ident) {
    struct val_obj *result = mem_allocate(mem, sizeof(struct val_obj));
    val_init(&result->base, VAL_KIND_OBJ, mem);
    result->ident = val_str_new_from_view(mem, ident);

    ptr_list_init(&result->props, default_config(mem));
    ptr_list_init(&result->values, default_config(mem));

    return result;
}

void
val_obj_append(o *self, f *field, v *value) {
    CHECK_NOT_NULL(self);
    CHECK_NOT_NULL(field);
    CHECK_NOT_NULL(value);

    // FIXME make sure that field is unique
    // FIXME object needs to have a type as well - eventually implicit by following object-fields


    u2 next_prop_id = val_obj_next_prop_id(self);
    p *new_prop = val_prop_new(self->base.mem, next_prop_id, field, self);
    ptr_list_append(&self->props, new_prop);
    ptr_list_append(&self->values, value);

    if (value->kind == VAL_KIND_OBJ) {
        o *obj = AS_OBJ(value);

        for (size_t idx = 0; idx < ptr_list_count(&obj->props); idx++) {

            v *nested_val = ptr_list_at(&obj->values, idx);
            next_prop_id = val_obj_next_prop_id(self);

            switch (nested_val->kind) {
                case VAL_KIND_NUM: {
                    p *new_nested_prop = val_prop_new(self->base.mem, next_prop_id, field, obj);
                    ptr_list_append(&self->props, new_nested_prop);
                    ptr_list_append(&self->values, nested_val);
                    break;
                }
                case VAL_KIND_STR: {
                    p *new_nested_prop = val_prop_new(self->base.mem, next_prop_id, field, obj);
                    ptr_list_append(&self->props, new_nested_prop);
                    ptr_list_append(&self->values, nested_val);
                    break;
                }
                default:
                    NOT_IMPLEMENTED_YET();
            }
        }
    }

    CHECK_EQUAL(ptr_list_count(&self->props), ptr_list_count(&self->values));
}

v *
val_obj_val_at(o *self, size_t idx) {
    CHECK_NOT_NULL(self);
    CHECK_LESS_THAN(idx, val_obj_count(self));
    return ptr_list_at(&self->values, idx);
}

v *
val_obj_val_of_prop(o *self, p *prop) {
    CHECK_NOT_NULL(self);
    CHECK_NOT_NULL(prop);
    return val_obj_val_at(self, prop->id);
}

p *
val_obj_prop_at(o *self, size_t idx) {
    CHECK_NOT_NULL(self);
    CHECK_LESS_THAN(idx, val_obj_count(self));
    return (p *) ptr_list_at(&self->props, idx);
}

struct iterator
val_obj_prop_iter(o *self) {
    CHECK_NOT_NULL(self);
    return ptr_list_iterator(&self->props);
}

u2
val_obj_next_prop_id(o *self) {
    CHECK_NOT_NULL(self);
    return ptr_list_count(&self->values);
}

size_t
val_obj_count(o *self) {
    CHECK_NOT_NULL(self);
    return ptr_list_count(&self->props);
}

void
val_obj_clear(o *self) {
    CHECK_NOT_NULL(self);
    for (size_t idx = 0; idx < ptr_list_count(&self->props); idx++) {
        p *prop = ptr_list_at(&self->props, idx);
        val_prop_free_safe(&prop);
    }
    ptr_list_reset(&self->props);

    for (size_t idx = 0; idx < ptr_list_count(&self->values); idx++) {
        v *val = ptr_list_at(&self->values, idx);
        val_free_safe(&val);
    }
    ptr_list_reset(&self->values);
}

void
val_obj_free(o *self) {
    CHECK_NOT_NULL(self);
    val_str_free_safe(&self->ident);

    for (size_t idx = 0; idx < ptr_list_count(&self->props); idx++) {
        p *prop = ptr_list_at(&self->props, idx);
        val_prop_free_safe(&prop);
    }
    ptr_list_reset(&self->props);
    ptr_list_reset(&self->values);
    mem_deallocate(self->base.mem, self);
}

void
val_obj_free_safe(o **self) {
    CHECK_NOT_NULL(self);
    val_obj_free(*self);
    *self = NULL;
}