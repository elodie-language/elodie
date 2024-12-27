#include <string.h>

#include "core/check.h"
#include "core/algo/algo-list-byte.h"
#include "core/val/val-str.h"
#include "core/mem/mem-stack.h"

typedef struct mem m;
typedef struct byte_list bl;
typedef struct bytes_view bv;
typedef struct mem_ref mr;
typedef struct val_ref vr;
typedef struct dep_val_str_view sv;
typedef struct mem m;
typedef struct byte_list bl;
typedef struct bytes_view bv;
typedef struct val_str s;
typedef struct dep_val_str_view sv;

vr
val_str_new_from_c_str (m *mem, char const *src)
{
	CHECK_NOT_NULL(mem);
	return val_str_new_from_bytes (
		mem,
		(bv){
			.size = strlen (src),
			.data = (u1 *)src
		});
}

vr
val_str_new_from_view (m *mem, sv view)
{
	CHECK_NOT_NULL(mem);
	return val_str_new_from_bytes (
		mem,
		(bv){
			.size = view.count,
			.data = (u1 *)view.data
		});
}

vr
val_str_new_from_bytes (m *mem, bv bytes)
{
	CHECK_NOT_NULL(mem);
	mr result = mem_next_ref (mem, VAL_KIND_STR);
	internal_val_str_allocate_from_bytes (mem, bytes);
	return (vr){
		.realm = result.realm,
		.value = result.value
	};
}

vr
val_str_new_from_byte_list (m *mem, bl *lst)
{
	CHECK_NOT_NULL(mem);
	CHECK_NOT_NULL(lst);
	bv bytes = byte_list_raw_bytes (lst);
	return val_str_new_from_bytes (mem, bytes);
}

s *
internal_val_str_allocate_from_bytes (m *mem, bv bytes)
{
	CHECK_NOT_NULL(mem);
	size_t size = bytes.size;
	s *result = mem_allocate (mem, sizeof (s));
	val_init (&result->base, VAL_KIND_STR, mem);

	result->count = size;
	result->data = mem_allocate (mem, size + 1);
	memcpy(result->data, bytes.data, size);
	result->data[size] = '\0';
	return result;
}

void
internal_val_str_deallocate (s *self, m *mem)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(self);
	CHECK_EQUAL(self->base.mem_realm, mem->realm);
	mem_deallocate (mem, self->data);
	mem_deallocate (mem, self);
}

// FIXME remove deprecated code
struct dep_val_str *
dep_val_str_allocate_from_c_str (struct mem *mem, char const *src)
{
	CHECK_NOT_NULL(mem);
	CHECK_NOT_NULL(src);

	size_t count = strlen (src);
	struct dep_val_str *result = mem_allocate (mem, sizeof (struct dep_val_str));
	dep_val_init (&result->base, VAL_KIND_STR, mem);

	result->count = count;
	result->data = mem_allocate (mem, count + 1);
	memcpy(result->data, src, count);
	result->data[count] = '\0';
	return result;
}

struct dep_val_str *
dep_val_str_allocate_from_bytes (struct mem *mem, struct bytes_view bytes)
{
	CHECK_NOT_NULL(mem);

	size_t count = bytes.size;
	struct dep_val_str *result = mem_allocate (mem, sizeof (struct dep_val_str));
	dep_val_init (&result->base, VAL_KIND_STR, mem);

	result->count = count;
	result->data = mem_allocate (mem, count + 1);
	memcpy(result->data, bytes.data, count);
	result->data[count] = '\0';
	return result;
}

struct dep_val_str *
dep_val_str_allocate_from_byte_list (struct mem *mem, struct byte_list *src)
{
	CHECK_NOT_NULL(mem);
	CHECK_NOT_NULL(src);
	size_t count = byte_list_size (src);
	struct dep_val_str *result = mem_allocate (mem, sizeof (struct dep_val_str));
	dep_val_init (&result->base, VAL_KIND_STR, mem);
	result->count = count;
	result->data = mem_allocate (mem, count + 1);
	memcpy(result->data, (u1 *)src->underlying_list.data, result->count);
	result->data[count] = '\0';
	return result;
}

struct dep_val_str *
dep_val_str_allocate_from_view (struct mem *mem, struct dep_val_str_view view)
{
	CHECK_NOT_NULL(mem);

	size_t count = view.count;
	struct dep_val_str *result = mem_allocate (mem, sizeof (struct dep_val_str));
	dep_val_init (&result->base, VAL_KIND_STR, mem);
	result->count = count;
	result->data = mem_allocate (mem, count + 1);
	memcpy(result->data, view.data, count);
	result->data[count] = '\0';
	return result;
}

struct dep_val_str *
dep_val_str_copy (struct dep_val_str *src, struct mem *mem)
{
	CHECK_NOT_NULL(mem);
	CHECK_NOT_NULL(src);

	struct dep_val_str *result = mem_allocate (mem, sizeof (struct dep_val_str));
	dep_val_init (&result->base, VAL_KIND_STR, mem);
	result->count = src->count;
	result->data = mem_allocate (mem, result->count);
	memcpy(result->data, src->data, result->count);
	return result;
}

size_t
dep_val_str_count (struct dep_val_str *self)
{
	CHECK_NOT_NULL(self);
	return self->count;
}

bool
dep_val_str_equal (struct dep_val_str *lhs, struct dep_val_str *rhs)
{
	CHECK_NOT_NULL(lhs);
	CHECK_NOT_NULL(rhs);
	if (lhs == rhs) return true;
	if (lhs->count != rhs->count) return false;
	return strncmp (lhs->data, rhs->data, lhs->count) == 0;
}

struct dep_val_str *
dep_val_str_concat (struct dep_val_str *self, struct dep_val_str *other, struct mem *mem)
{
	CHECK_NOT_NULL(mem);
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(other);
	size_t count = self->count + other->count;

	struct dep_val_str *result = mem_allocate (mem, sizeof (struct dep_val_str));
	dep_val_init (&result->base, VAL_KIND_STR, mem);

	result->count = count;
	result->data = mem_allocate (mem, count + 1);
	memcpy(result->data, self->data, self->count);
	memcpy(result->data + self->count, other->data, other->count);
	result->data[count] = '\0';

	return result;
}

void
dep_val_str_deallocate (struct dep_val_str *self)
{
	CHECK_NOT_NULL(self);
	mem_deallocate (self->base.mem, self->data);
	mem_deallocate (self->base.mem, self);
}

void
dep_val_str_deallocate_safe (struct dep_val_str **self)
{
	CHECK_NOT_NULL(self);
	dep_val_str_deallocate (*self);
	*self = NULL;
}
