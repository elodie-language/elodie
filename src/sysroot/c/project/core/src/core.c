#include <stdio.h>
#include <string.h>

#include "core/check.h"
#include "core/core.h"
#include "core/val/val-str.h"

struct dep_val_str *
u2_to_str (struct mem *mem, u2 val)
{
	CHECK_NOT_NULL(mem);
	char buffer[5];
	sprintf(buffer, "%d", val);
	return dep_val_str_allocate_from_c_str (mem, (const char *)&buffer);
}

