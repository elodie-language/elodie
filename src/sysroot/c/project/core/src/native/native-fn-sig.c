#include "core/check.h"
#include "core/native/native-fn.h"

typedef struct mem m;
typedef struct hash8 h;
typedef struct type tok;
typedef struct native_fn_signature sig;
typedef struct native_fn_signature_ident sig_ident;
typedef struct native_fn_signature_param sig_param;
typedef struct native_fn_signature_result sig_result;
typedef struct string_view sv;

// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-[native_fn_signature_ident]+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-

const u1 MARKER_COUNT = 2;

sig_ident *
native_fn_signature_ident_allocate (m *mem, sv ident)
{
	CHECK_NOT_NULL(mem);
	sig_ident *result = mem_allocate (mem, sizeof (sig_ident));
	native_fn_signature_ident_init (result, mem, ident);
	return result;
}

void
native_fn_signature_ident_init (sig_ident *self, m *mem, sv ident)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(mem);
	string_init_from_view (&self->ident, mem, ident);
	u4 position;
	CHECK_TRUE(string_view_last_occurrence_of (ident, STRING_VIEW ("::"), &position));
	CHECK_LESS_THAN(position, U2_MAX);
	self->marker = position;
	CHECK_LESS_THAN(self->marker + MARKER_COUNT, ident.count);
}

sv
native_fn_signature_ident_package_ident (sig_ident self)
{
	return (sv){
		.data = self.ident.data,
		.count = self.marker
	};
}

sv
native_fn_signature_ident_fn_ident (sig_ident self)
{
	return (sv){
		.count = self.ident.count - self.marker - MARKER_COUNT,
		.data = self.ident.data + self.marker + MARKER_COUNT
	};
}

sv
native_fn_signature_ident (sig_ident self)
{
	return string_view_from_str (self.ident);
}

void
native_fn_signature_ident_reset (sig_ident *self, m *mem)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(mem);
	string_reset (&self->ident, mem);
	self->marker = 0;
}

void
native_fn_signature_ident_deallocate (sig_ident *self, m *mem)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(mem);
	native_fn_signature_ident_reset (self, mem);
	mem_deallocate (mem, self);
}

ELODIE_API void
native_fn_signature_ident_deallocate_safe (sig_ident **self, m *mem)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(mem);
	native_fn_signature_ident_deallocate (*self, mem);
	*self = NULL;
}


// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-[native_fn_signature_param]+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-

sig_param *
native_fn_signature_param_allocate (m *mem, sv ident, tok type)
{
	CHECK_NOT_NULL(mem);
	sig_param *result = mem_allocate (mem, sizeof (sig_param));
	native_fn_signature_param_init (result, mem, ident, type);
	return result;
}

void
native_fn_signature_param_init (sig_param *self, m *mem, sv ident, tok type)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(mem);
	string_init_from_view (&self->ident, mem, ident);
	self->type = type;
	self->next = NULL;
}

void
native_fn_signature_param_append (sig_param *self, sig_param *other)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(other);
	for (sig_param *cur = self; cur != NULL; cur = cur->next)
		{
			if (cur->next == NULL)
				{
					cur->next = other;
					return;
				}
		}
}

u1
native_fn_signature_param_count (sig_param const *self)
{
	u1 result = 0;
	for (sig_param const *cur = self; cur != NULL; cur = cur->next)
		{
			result++;
		}
	return result;
}

void
native_fn_signature_param_reset (sig_param *self, m *mem)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(mem);
	string_reset (&self->ident, mem);
	if (self->next != NULL)
		{
			native_fn_signature_param_reset (self->next, mem);
		}
}

void
native_fn_signature_param_deallocate (sig_param *self, m *mem)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(mem);
	if (self->next != NULL)
		{
			native_fn_signature_param_deallocate_safe (&self->next, mem);
		}
	native_fn_signature_param_reset (self, mem);
	mem_deallocate (mem, self);
}

void
native_fn_signature_param_deallocate_safe (sig_param **self, m *mem)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(mem);
	native_fn_signature_param_deallocate (*self, mem);
	*self = NULL;
}


// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-[native_fn_signature_result]+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-

sig_result *
native_fn_signature_result_allocate (m *mem, tok type)
{
	CHECK_NOT_NULL(mem);
	sig_result *result = mem_allocate (mem, sizeof (sig_result));
	native_fn_signature_result_init (result, mem, type);
	return result;
}

void
native_fn_signature_result_init (sig_result *self, m *mem, tok type)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(mem);
	self->type = type;
}

void
native_fn_signature_result_reset (sig_result *self, m *mem)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(mem);
}

void
native_fn_signature_result_deallocate (sig_result *self, m *mem)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(mem);
	native_fn_signature_result_reset (self, mem);
	mem_deallocate (mem, self);
}

void
native_fn_signature_result_deallocate_safe (sig_result **self, m *mem)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(mem);
	native_fn_signature_result_deallocate (*self, mem);
	*self = NULL;
}

// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-[native_fn_sig]+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-

static h
calculate_signature_param_hash (sig_param *params, struct hash8_fn hash_fn)
{
	u1 param_count = native_fn_signature_param_count (params);
	struct hash8 hashes[param_count];

	u1 idx = 0;
	for (sig_param const *cur = params; cur != NULL; cur = cur->next)
		{
			hashes[idx++] = hash8_of (hash_fn, bytes_view_of_u2 (&cur->type.id));
			idx++;
		}
	return hash8_of_hashes (hash_fn, hashes, param_count);
}

static h
calculate_hash (sig_ident ident, sig_param *params, sig_result result)
{
	struct hash8_fn hash_fn = hash_fn_murmur_3 (127);
	struct hash8 hashes[3];
	hashes[0] = hash8_of (hash_fn, string_view_as_byte_view (native_fn_signature_ident_fn_ident (ident)));
	hashes[1] = calculate_signature_param_hash (params, hash_fn);
	hashes[2] = hash8_of (hash_fn, bytes_view_of_u2 (&result.type.id));
	return hash8_of_hashes (hash_fn, hashes, 3);
}

sig *
native_fn_signature_allocate (m *mem, sig_ident ident, sig_param *param, sig_result sig_result)
{
	CHECK_NOT_NULL(mem);
	CHECK_NOT_NULL(param);
	sig *result = mem_allocate (mem, sizeof (sig));
	native_fn_signature_init (result, mem, ident, param, sig_result);
	return result;
}

void
native_fn_signature_init (sig *self, m *mem, sig_ident ident, sig_param *params, sig_result result)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(mem);
	CHECK_NOT_NULL(params);
	h hash = calculate_hash (ident, params, result);
	CHECK_NOT_EQUAL(0, hash.value);
	self->ident = ident;
	self->params = params;
	self->result = result;
	self->hash = hash;
}

void
native_fn_signature_reset (sig *self, m *mem)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(mem);
	native_fn_signature_ident_reset (&self->ident, mem);
	native_fn_signature_param_deallocate_safe (&self->params, mem);
	native_fn_signature_result_reset (&self->result, mem);
}

void
native_fn_signature_deallocate (sig *self, m *mem)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(mem);
	native_fn_signature_reset (self, mem);
	mem_deallocate (mem, self);
}

void
native_fn_signature_deallocate_safe (sig **self, m *mem)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(mem);
	native_fn_signature_deallocate (*self, mem);
	*self = NULL;
}
