#include "core/io.h"
#include "core/check.h"
#include "core/mem/mem-api.h"
#include "core/algo/algo-api.h"

struct io *
io_new (struct mem *mem)
{
	struct io *result = mem_allocate (mem, sizeof (struct io));
	io_init (result, mem);
	return result;
}

void
io_init (struct io *self, struct mem *mem)
{
	CHECK_NOT_NULL(self);
	LOG_DEBUG("init io interface");
	self->mem = mem;
	LOG_DEBUG("io interface inited");
}

struct buffer *
io_read_file (struct io *self, struct val_str_view path)
{
	CHECK_NOT_NULL(self);

	FILE *file = fopen (path.data, "rb");
	if (file == NULL)
		{
			ABORT("Could not open file \"%.*s\"\n", (int)path.count, path.data);
		}

	fseek (file, 0L, SEEK_END);
	size_t file_size = ftell (file);
	rewind (file);

	struct buffer *result = buffer_new (self->mem, file_size);
	if (result == NULL)
		{
			ABORT("Not enough mem to read \"%.*s\"\n", (int)path.count, path.data);
		}

	size_t bytes_read = fread (result->data, sizeof (u1), file_size, file);
	if (bytes_read < file_size)
		{
			ABORT ("Could not read file \"%.*s\"\n", (int)path.count, path.data);
		}
	result->position = bytes_read;
	fclose (file);
	buffer_flip (result);
	return result;

}

void
io_reset (struct io *self)
{
	CHECK_NOT_NULL(self);
	self->mem = (struct mem *)mem_null_new ();
}

void
io_free (struct io *self)
{
	CHECK_NOT_NULL(self);
	mem_deallocate (self->mem, self);
}
