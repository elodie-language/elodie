#include "string.h"
#include "core/check.h"
#include "core/bytes/bytes-view.h"

typedef struct bytes b;
typedef struct bytes_view bv;

bv
bytes_view_of_u2 (u2 const *data)
{
	return (bv){.data = (u1 *)data, .size = 2};
}

u2
bytes_view_as_u2 (bv self)
{
	CHECK_GREATER_THAN_EQUAL(4, self.size);
	return *(u2 *)(void *)self.data;
}

bv
bytes_view_of_u4 (u4 const *data)
{
	return (bv){.data = (u1 *)data, .size = 4};
}

u4
bytes_view_as_u4 (bv self)
{
	CHECK_GREATER_THAN_EQUAL(4, self.size);
	return *(u4 *)(void *)self.data;
}

bv
bytes_view_of_u8 (u8 const *data)
{
	return (bv){.data = (u1 *)data, .size = 8};
}

u8
bytes_view_as_u8 (bv self)
{
	CHECK_GREATER_THAN_EQUAL(8, self.size);
	return *(u8 *)(void *)self.data;
}

bv
bytes_view_of_c_str (char const *c_str)
{
	CHECK_NOT_NULL(c_str);
	return (bv){.data = (u1 *)c_str, .size = strlen (c_str)};
}

bv
bytes_view_of_bytes (b bytes)
{
	return (bv){.data = bytes.data, .size = bytes.size};
}

bv
bytes_view_of_ptr (void *ptr, u4 size)
{
	return (bv){.data = ptr, .size = size};
}

void *
bytes_view_as_ptr (bv self, u4 size)
{
	CHECK_EQUAL(self.size, size);
	CHECK_NOT_NULL(self.data);
	return (void *)self.data;
}
