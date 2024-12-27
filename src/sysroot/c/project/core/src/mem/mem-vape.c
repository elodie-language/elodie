#include "core/check.h"
#include "core/mem/mem-vape.h"

static void *
allocate (void *ctx, size_t size)
{
	CHECK_NOT_NULL(ctx);
	struct mem_vape *impl = (struct mem_vape *)ctx;
	void *result = mem_vape_allocate (impl, size);
	LOG_TRACE("allocated %lu bytes - %p", size, result);
	return result;
}

static void
deallocate (void *, void *)
{
}

static struct mem_ref
next_reference (void *ctx, u1 kind)
{
	CHECK_NOT_NULL(ctx);
	struct mem_vape *impl = (struct mem_vape *)ctx;
	mem_vape_reset (impl);
	return (struct mem_ref){
		.kind = kind,
		.realm = impl->base.realm,
		.value = 0
	};
}

static void *
resolve (void *self, struct mem_ref ref)
{
	CHECK_NOT_NULL(self);
	struct mem_vape *impl = (struct mem_vape *)self;
	return mem_vape_resolve (impl, ref);
}

struct mem_vape *
mem_vape_new (struct mem_vape_config config)
{
	struct mem_allocator allocator = config.root->allocator;
	struct mem_vape *result = mem_allocator_allocate (allocator, sizeof (struct mem_vape));
	mem_vape_init (result, config);
	return result;
}

void
mem_vape_init (struct mem_vape *self, struct mem_vape_config config)
{
	CHECK_NOT_NULL(self);
	CHECK_NOT_NULL(config.root->allocator.fn);
	CHECK_NOT_NULL(config.root->deallocator.fn);
	CHECK_GREATER_THAN(config.size, 0);

	self->base = (struct mem){
		.kind = MEM_KIND_VAPE,
		.allocator = (struct mem_allocator){
			.self = self,
			.fn = allocate
		},
		.deallocator = (struct mem_deallocator){
			.self = self,
			.fn = deallocate
		},
		.ref_generator = (struct mem_ref_generator){
			.self = self,
			.fn = next_reference
		},
		.resolver = (struct mem_resolver){
			.self = self,
			.fn = resolve
		},
		.root = (struct mem *)config.root
	};
	self->start = mem_allocator_allocate (self->base.root->allocator, config.size);
	CHECK_NOT_NULL(self->start);
	self->current = self->start;
	self->end = (u1 *)self->start + config.size;
	self->capacity = config.size;
}

void *
mem_vape_allocate (struct mem_vape *self, size_t size)
{
	CHECK_NOT_NULL(self);
	CHECK_LESS_THAN_EQUAL(size, self->capacity);

	if ((u1 *)self->current + size > (u1 *)self->end)
		{
			self->current = self->start;
		}

	void *result = (u1 *)self->current;
	self->current = (u1 *)result + size;
	return result;
}

void *
mem_vape_resolve (struct mem_vape *self, struct mem_ref ref)
{
	CHECK_NOT_NULL(self);
	CHECK_LESS_THAN((u1 *)self->start + ref.value, (u1 *)self->end);
	return (void *)((u1 *)self->start + ref.value);
}

size_t
mem_vape_size (struct mem_vape *self)
{
	CHECK_NOT_NULL(self);
	return self->capacity - ((u1 *)self->end - (u1 *)self->current);
}

void
mem_vape_reset (struct mem_vape *self)
{
	CHECK_NOT_NULL(self);
	self->current = self->start;
}

void
mem_vape_free (struct mem_vape *self)
{
	CHECK_NOT_NULL(self);
	struct mem_deallocator root_deallocator = self->base.root->deallocator;
	CHECK_NOT_NULL(root_deallocator.fn);

	mem_deallocator_deallocate (root_deallocator, self->start);
	mem_deallocator_deallocate (root_deallocator, self);
}

void
mem_vape_free_safe (struct mem_vape **self)
{
	CHECK_NOT_NULL(self);
	mem_vape_free (*self);
	*self = NULL;
}
