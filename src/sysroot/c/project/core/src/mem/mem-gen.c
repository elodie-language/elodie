#include "core/check.h"
#include "core/mem/mem-gen.h"
#include "string.h"

static void *
allocate(void *ctx, size_t size) {
    NOT_IMPLEMENTED_YET();
}

static void
deallocate(void *ctx, void *) {
    NOT_IMPLEMENTED_YET();
}

struct mem_gen *
mem_gen_new(struct mem_gen_config config) {
    struct mem_allocator allocator = config.root->allocator;
    struct mem_gen *result = mem_allocator_allocate(allocator, sizeof(struct mem_gen));
    mem_gen_init(result, config);
    return result;
}

void
mem_gen_init(struct mem_gen *self, struct mem_gen_config config) {
    CHECK_NOT_NULL(self);
    CHECK_NOT_NULL(config.root);
    CHECK_GREATER_THAN(config.size, 0);
    self->base = (struct mem) {
            .kind = MEM_KIND_VAL,
            .allocator = (struct mem_allocator) {
                    .self = self,
                    .fn = allocate
            },
            .deallocator = (struct mem_deallocator) {
                    .self = self,
                    .fn = deallocate
            },
            .root = config.root
    };

    struct list_config list_config = list_default_config(config.root);
    list_config.initial_capacity = config.size;

    ptr_list_init(&self->vals, ptr_list_default_config(config.root));
}

struct val_ref
mem_gen_allocate(struct mem_gen *self, struct val *val) {

    struct val *result = val_copy(val, self->base.root);
    ptr_list_append(&self->vals, result);

    return (struct val_ref) {
            .kind = 0x01,
            .value = ptr_list_count(&self->vals) - 1
    };
}

//struct val_ref
//val_str_ng_new (struct mem_gen *self, char const *c_str)
//{
//	struct mem *mem = self->base.root;
//	struct val_str_ng *result = mem_allocate (mem, sizeof (struct val_str));
////	val_init (&result->base, VAL_KIND_STR, mem);
//
//	size_t count = strlen (c_str);
//	result->count = strlen (c_str);
//	result->data = mem_allocate (mem, count + 1);
//
//	memcpy(result->data, c_str, count);
//	result->data[count] = '\0';
////	return result;
//
//	ptr_list_append (&self->vals, result);
//
//	return (struct val_ref){
//		.kind = 0x01,
//		.value = ptr_list_count (&self->vals) - 1
//	};
////}
//}

struct val *
mem_gen_resolve(struct mem_gen *self, struct val_ref ref) {
    return ptr_list_at(&self->vals, ref.value);
}

size_t
mem_gen_count(struct mem_gen *self) {
    return ptr_list_count(&self->vals);
}

void
mem_gen_free(struct mem_gen *self) {
    NOT_IMPLEMENTED_YET()
}

void
mem_gen_free_safe(struct mem_gen **self) {
    NOT_IMPLEMENTED_YET()
}