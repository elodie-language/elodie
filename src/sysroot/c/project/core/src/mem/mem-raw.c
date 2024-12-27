#include <stdlib.h>

#include "core/check.h"
#include "core/mem/mem-raw.h"

static void *
mem_raw_allocate (void *ctx, size_t size)
{
	CHECK_NULL(ctx);
	void *result = malloc (size);
	if (result == NULL)
		{
			ABORT("unable to mem_allocator_allocate %lu bytes\n", size);
		}
	LOG_TRACE("allocated %lu bytes - %p", size, result);
	return result;
}

static void
mem_raw_deallocate (void *ctx, void *ptr)
{
	CHECK_NULL(ctx);
	if (ptr != NULL)
		{
			LOG_TRACE("deallocated %p", ptr);
			free (ptr);
		}
}

static struct mem_ref
next_reference (void *, u1)
{
	NOT_IMPLEMENTED_YET()
}

static struct mem_raw raw_mem = (struct mem_raw){
	.base = (struct mem){
		.kind = MEM_KIND_RAW,
		.allocator = (struct mem_allocator){
			.self = NULL,
			.fn = mem_raw_allocate
		},
		.deallocator = (struct mem_deallocator){
			.self = NULL,
			.fn = mem_raw_deallocate
		},
		.ref_generator = (struct mem_ref_generator){
			.self = NULL,
			.fn = next_reference
		}
	}
};

struct mem_raw *
mem_raw_new (void)
{
	return &raw_mem;
}
