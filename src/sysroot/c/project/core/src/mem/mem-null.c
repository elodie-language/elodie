#include "core/check.h"
#include "core/mem/mem-null.h"
#include "core/mem/mem.h"

static void *
mem_null_allocate (void *, size_t)
{
	NOT_IMPLEMENTED_YET()
}

static void
mem_null_deallocate (void *, void *)
{
	NOT_IMPLEMENTED_YET()
}

struct mem_ref
next_reference (void *, u1)
{
	NOT_IMPLEMENTED_YET()
}

static struct mem_null null_mem = (struct mem_null){
	.base = (struct mem){
		.kind = MEM_KIND_NULL,
		.allocator = (struct mem_allocator){
			.self = NULL,
			.fn = mem_null_allocate
		},
		.deallocator = (struct mem_deallocator){
			.fn = mem_null_deallocate
		},
		.ref_generator = (struct mem_ref_generator){
			.fn = next_reference
		}
	}
};

struct mem_null *
mem_null_new (void)
{
	return &null_mem;
}
